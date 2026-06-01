use crate::analysis::Outcome;
use crate::criteria::SignalState;
use crate::error::{Result, WaveformError};
use crate::model::{
    TransformCategory, TransformExecutionMetadata, TransformOutputChannels,
    TransformParameterMetadata, TransformPhaseEffect, TransformStepMetadata, Waveform,
};
use ferrisoxide_rule_engine::{evaluate_schmitt_states, SchmittTriggerSpec};
use serde::Serialize;

const FLOAT_TOLERANCE: f64 = 1.0e-12;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    StateTransition,
    Edge,
    RejectedPulse,
    Bounce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventValidationKind {
    MissingPulse,
    ExtraPulse,
    DwellTime,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EventRecord {
    pub id: String,
    pub transform: String,
    pub kind: EventKind,
    pub channel: String,
    pub sample_index: usize,
    pub timestamp: f64,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_threshold_v: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_threshold_v: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_s: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<usize>,
    pub source_event_ids: Vec<String>,
    pub transform_metadata: TransformStepMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EventValidationResult {
    pub requirement_id: String,
    pub validation: EventValidationKind,
    pub outcome: Outcome,
    pub channel: String,
    pub measured_value: f64,
    pub required_value: f64,
    pub unit: String,
    pub linked_event_ids: Vec<String>,
    pub reason: String,
    pub transform_metadata: TransformStepMetadata,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventEvaluation {
    pub records: Vec<EventRecord>,
    pub validations: Vec<EventValidationResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventTransformStep {
    SchmittTrigger(SchmittTriggerTransform),
    Debounce(DebounceTransform),
    GlitchRemoval(GlitchRemovalTransform),
    EdgeExtraction(EdgeExtractionTransform),
    BounceDetection(BounceDetectionTransform),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventValidationStep {
    MissingPulse(MissingPulseValidation),
    ExtraPulse(ExtraPulseValidation),
    DwellTime(DwellTimeValidation),
    Timeout(TimeoutValidation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SchmittTriggerTransform {
    pub id: String,
    pub channel: String,
    pub on_threshold_v: f64,
    pub off_threshold_v: f64,
    pub initial_state: SignalState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebounceTransform {
    pub id: String,
    pub channel: String,
    pub min_duration_s: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlitchRemovalTransform {
    pub id: String,
    pub channel: String,
    pub max_duration_s: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeExtractionTransform {
    pub id: String,
    pub channel: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BounceDetectionTransform {
    pub id: String,
    pub channel: String,
    pub window_s: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MissingPulseValidation {
    pub id: String,
    pub channel: String,
    pub direction: EdgeDirectionFilter,
    pub expected_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtraPulseValidation {
    pub id: String,
    pub channel: String,
    pub direction: EdgeDirectionFilter,
    pub max_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DwellTimeValidation {
    pub id: String,
    pub channel: String,
    pub state: SignalState,
    pub min_duration_s: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeoutValidation {
    pub id: String,
    pub channel: String,
    pub direction: EdgeDirectionFilter,
    pub start_time_s: f64,
    pub max_time_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeDirectionFilter {
    Rising,
    Falling,
    Any,
}

impl EdgeDirectionFilter {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "rising" => Some(Self::Rising),
            "falling" => Some(Self::Falling),
            "any" => Some(Self::Any),
            _ => None,
        }
    }

    fn matches(self, direction: &str) -> bool {
        matches!(self, Self::Any)
            || matches!(
                (self, direction),
                (Self::Rising, "rising") | (Self::Falling, "falling")
            )
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Rising => "rising",
            Self::Falling => "falling",
            Self::Any => "any",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EventStateTrace {
    transform_id: String,
    channel: String,
    on_threshold_v: f64,
    off_threshold_v: f64,
    states: Vec<SignalState>,
}

pub fn evaluate_event_pipeline(
    waveform: &Waveform,
    transforms: &[EventTransformStep],
    validations: &[EventValidationStep],
) -> Result<EventEvaluation> {
    if transforms.is_empty() && validations.is_empty() {
        return Ok(EventEvaluation {
            records: Vec::new(),
            validations: Vec::new(),
        });
    }

    let mut trace = None;
    let mut records = Vec::new();

    for (step_index, step) in transforms.iter().enumerate() {
        match step {
            EventTransformStep::SchmittTrigger(transform) => {
                let mut output = transform.apply(waveform)?;
                set_event_metadata_sequence(&mut output.records, step_index);
                records.extend(output.records);
                trace = Some(output.trace);
            }
            EventTransformStep::Debounce(transform) => {
                let current = trace.as_mut().ok_or_else(missing_trace_error)?;
                let first_new_record = records.len();
                transform.apply(waveform, current, &mut records)?;
                set_event_metadata_sequence(&mut records[first_new_record..], step_index);
            }
            EventTransformStep::GlitchRemoval(transform) => {
                let current = trace.as_mut().ok_or_else(missing_trace_error)?;
                let first_new_record = records.len();
                transform.apply(waveform, current, &mut records)?;
                set_event_metadata_sequence(&mut records[first_new_record..], step_index);
            }
            EventTransformStep::EdgeExtraction(transform) => {
                let current = trace.as_ref().ok_or_else(missing_trace_error)?;
                let mut output = transform.apply(waveform, current)?;
                set_event_metadata_sequence(&mut output, step_index);
                records.extend(output);
            }
            EventTransformStep::BounceDetection(transform) => {
                let current = trace.as_ref().ok_or_else(missing_trace_error)?;
                if let Some(mut record) = transform.apply(waveform, current)? {
                    record.transform_metadata.sequence_index = step_index;
                    records.push(record);
                }
            }
        }
    }

    let validation_results = validations
        .iter()
        .enumerate()
        .map(|(validation_index, validation)| {
            let mut result = validation.evaluate(waveform, trace.as_ref(), &records)?;
            result.transform_metadata.sequence_index = transforms.len() + validation_index;
            Ok(result)
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(EventEvaluation {
        records,
        validations: validation_results,
    })
}

fn set_event_metadata_sequence(records: &mut [EventRecord], sequence_index: usize) {
    for record in records {
        record.transform_metadata.sequence_index = sequence_index;
    }
}

struct SchmittOutput {
    trace: EventStateTrace,
    records: Vec<EventRecord>,
}

impl SchmittTriggerTransform {
    fn apply(&self, waveform: &Waveform) -> Result<SchmittOutput> {
        let channel =
            waveform
                .channel(&self.channel)
                .ok_or_else(|| WaveformError::InvalidParameter {
                    name: "event_transforms.channel".to_string(),
                    reason: format!("missing channel `{}`", self.channel),
                })?;
        let spec = SchmittTriggerSpec {
            on_threshold_v: self.on_threshold_v,
            off_threshold_v: self.off_threshold_v,
            initial_state: self.initial_state,
        };
        let states = evaluate_schmitt_states(&waveform.time, &channel.samples, spec)
            .map_err(rule_engine_event_error)?;
        let trace = EventStateTrace {
            transform_id: self.id.clone(),
            channel: self.channel.clone(),
            on_threshold_v: self.on_threshold_v,
            off_threshold_v: self.off_threshold_v,
            states,
        };
        let metadata = event_transform_metadata(
            format!(
                "schmitt_trigger(channel={},on_threshold_v={},off_threshold_v={},initial_state={})",
                self.channel,
                self.on_threshold_v,
                self.off_threshold_v,
                self.initial_state.as_str()
            ),
            "schmitt_trigger",
            vec![
                TransformParameterMetadata::float("on_threshold_v", self.on_threshold_v, "V"),
                TransformParameterMetadata::float("off_threshold_v", self.off_threshold_v, "V"),
            ],
            true,
        );
        let records = transition_records(
            waveform,
            &trace,
            &self.id,
            "schmitt_trigger",
            EventKind::StateTransition,
            metadata,
            Vec::new(),
        );
        Ok(SchmittOutput { trace, records })
    }
}

impl DebounceTransform {
    fn apply(
        &self,
        waveform: &Waveform,
        trace: &mut EventStateTrace,
        records: &mut Vec<EventRecord>,
    ) -> Result<()> {
        ensure_trace_channel(&self.channel, trace)?;
        validate_duration("min_duration_s", self.min_duration_s)?;
        let metadata = event_transform_metadata(
            format!(
                "debounce(channel={},min_duration_s={})",
                self.channel, self.min_duration_s
            ),
            "debounce",
            vec![TransformParameterMetadata::float(
                "min_duration_s",
                self.min_duration_s,
                "s",
            )],
            true,
        );
        records.extend(remove_short_pulses(
            waveform,
            trace,
            &self.id,
            "debounce",
            self.min_duration_s,
            metadata,
        )?);
        Ok(())
    }
}

impl GlitchRemovalTransform {
    fn apply(
        &self,
        waveform: &Waveform,
        trace: &mut EventStateTrace,
        records: &mut Vec<EventRecord>,
    ) -> Result<()> {
        ensure_trace_channel(&self.channel, trace)?;
        validate_duration("max_duration_s", self.max_duration_s)?;
        let metadata = event_transform_metadata(
            format!(
                "glitch_removal(channel={},max_duration_s={})",
                self.channel, self.max_duration_s
            ),
            "glitch_removal",
            vec![TransformParameterMetadata::float(
                "max_duration_s",
                self.max_duration_s,
                "s",
            )],
            true,
        );
        records.extend(remove_short_pulses(
            waveform,
            trace,
            &self.id,
            "glitch_removal",
            self.max_duration_s,
            metadata,
        )?);
        Ok(())
    }
}

impl EdgeExtractionTransform {
    fn apply(&self, waveform: &Waveform, trace: &EventStateTrace) -> Result<Vec<EventRecord>> {
        ensure_trace_channel(&self.channel, trace)?;
        let metadata = event_transform_metadata(
            format!("edge_extraction(channel={})", self.channel),
            "edge_extraction",
            Vec::new(),
            false,
        );
        Ok(transition_records(
            waveform,
            trace,
            &self.id,
            "edge_extraction",
            EventKind::Edge,
            metadata,
            Vec::new(),
        ))
    }
}

impl BounceDetectionTransform {
    fn apply(&self, waveform: &Waveform, trace: &EventStateTrace) -> Result<Option<EventRecord>> {
        ensure_trace_channel(&self.channel, trace)?;
        validate_duration("window_s", self.window_s)?;
        let transitions = transition_points(trace);
        if transitions.len() <= 1 {
            return Ok(None);
        }

        let first = transitions[0];
        let window_end = waveform.time[first.index] + self.window_s + FLOAT_TOLERANCE;
        let bounce_points = transitions
            .iter()
            .skip(1)
            .copied()
            .filter(|transition| waveform.time[transition.index] <= window_end)
            .collect::<Vec<_>>();
        if bounce_points.is_empty() {
            return Ok(None);
        }

        let last = bounce_points[bounce_points.len() - 1];
        let metadata = event_transform_metadata(
            format!(
                "bounce_detection(channel={},window_s={})",
                self.channel, self.window_s
            ),
            "bounce_detection",
            vec![TransformParameterMetadata::float(
                "window_s",
                self.window_s,
                "s",
            )],
            true,
        );

        Ok(Some(EventRecord {
            id: format!("{}_bounce_0", self.id),
            transform: "bounce_detection".to_string(),
            kind: EventKind::Bounce,
            channel: self.channel.clone(),
            sample_index: bounce_points[0].index,
            timestamp: waveform.time[bounce_points[0].index],
            state: bounce_points[0].state.as_str().to_string(),
            previous_state: Some(bounce_points[0].previous.as_str().to_string()),
            direction: Some(
                direction(bounce_points[0].previous, bounce_points[0].state).to_string(),
            ),
            on_threshold_v: Some(trace.on_threshold_v),
            off_threshold_v: Some(trace.off_threshold_v),
            duration_s: Some(waveform.time[last.index] - waveform.time[first.index]),
            count: Some(bounce_points.len()),
            source_event_ids: transitions
                .iter()
                .take(bounce_points.len() + 1)
                .enumerate()
                .map(|(index, _)| format!("{}_transition_{index}", trace.transform_id))
                .collect(),
            transform_metadata: metadata,
        }))
    }
}

impl EventValidationStep {
    fn evaluate(
        &self,
        waveform: &Waveform,
        trace: Option<&EventStateTrace>,
        records: &[EventRecord],
    ) -> Result<EventValidationResult> {
        match self {
            Self::MissingPulse(validation) => validation.evaluate(records),
            Self::ExtraPulse(validation) => validation.evaluate(records),
            Self::DwellTime(validation) => validation.evaluate(waveform, trace),
            Self::Timeout(validation) => validation.evaluate(records),
        }
    }
}

impl MissingPulseValidation {
    fn evaluate(&self, records: &[EventRecord]) -> Result<EventValidationResult> {
        let edges = matching_edges(records, &self.channel, self.direction);
        let outcome = if edges.len() >= self.expected_count {
            Outcome::Pass
        } else {
            Outcome::Fail
        };
        Ok(EventValidationResult {
            requirement_id: self.id.clone(),
            validation: EventValidationKind::MissingPulse,
            outcome,
            channel: self.channel.clone(),
            measured_value: edges.len() as f64,
            required_value: self.expected_count as f64,
            unit: "events".to_string(),
            linked_event_ids: edges.iter().map(|event| event.id.clone()).collect(),
            reason: if outcome == Outcome::Pass {
                format!(
                    "observed {} {} pulse event(s)",
                    edges.len(),
                    self.direction.as_str()
                )
            } else {
                format!(
                    "expected at least {} {} pulse event(s), observed {}",
                    self.expected_count,
                    self.direction.as_str(),
                    edges.len()
                )
            },
            transform_metadata: event_validation_metadata(
                format!(
                    "missing_pulse(channel={},direction={},expected_count={})",
                    self.channel,
                    self.direction.as_str(),
                    self.expected_count
                ),
                "missing_pulse",
                vec![TransformParameterMetadata::integer(
                    "expected_count",
                    self.expected_count as u64,
                    "events",
                )],
            ),
        })
    }
}

impl ExtraPulseValidation {
    fn evaluate(&self, records: &[EventRecord]) -> Result<EventValidationResult> {
        let edges = matching_edges(records, &self.channel, self.direction);
        let outcome = if edges.len() <= self.max_count {
            Outcome::Pass
        } else {
            Outcome::Fail
        };
        Ok(EventValidationResult {
            requirement_id: self.id.clone(),
            validation: EventValidationKind::ExtraPulse,
            outcome,
            channel: self.channel.clone(),
            measured_value: edges.len() as f64,
            required_value: self.max_count as f64,
            unit: "events".to_string(),
            linked_event_ids: edges.iter().map(|event| event.id.clone()).collect(),
            reason: if outcome == Outcome::Pass {
                format!(
                    "observed {} {} pulse event(s), within maximum {}",
                    edges.len(),
                    self.direction.as_str(),
                    self.max_count
                )
            } else {
                format!(
                    "expected no more than {} {} pulse event(s), observed {}",
                    self.max_count,
                    self.direction.as_str(),
                    edges.len()
                )
            },
            transform_metadata: event_validation_metadata(
                format!(
                    "extra_pulse(channel={},direction={},max_count={})",
                    self.channel,
                    self.direction.as_str(),
                    self.max_count
                ),
                "extra_pulse",
                vec![TransformParameterMetadata::integer(
                    "max_count",
                    self.max_count as u64,
                    "events",
                )],
            ),
        })
    }
}

impl DwellTimeValidation {
    fn evaluate(
        &self,
        waveform: &Waveform,
        trace: Option<&EventStateTrace>,
    ) -> Result<EventValidationResult> {
        validate_duration("min_duration_s", self.min_duration_s)?;
        let trace = trace.ok_or_else(missing_trace_error)?;
        ensure_trace_channel(&self.channel, trace)?;
        let dwell = longest_state_run(waveform, trace, self.state);
        let outcome = if dwell.duration_s + FLOAT_TOLERANCE >= self.min_duration_s {
            Outcome::Pass
        } else {
            Outcome::Fail
        };
        Ok(EventValidationResult {
            requirement_id: self.id.clone(),
            validation: EventValidationKind::DwellTime,
            outcome,
            channel: self.channel.clone(),
            measured_value: dwell.duration_s,
            required_value: self.min_duration_s,
            unit: "s".to_string(),
            linked_event_ids: Vec::new(),
            reason: if outcome == Outcome::Pass {
                format!(
                    "{} state persisted for {:.9} s",
                    self.state.as_str(),
                    dwell.duration_s
                )
            } else {
                format!(
                    "{} state persisted for {:.9} s, below required {:.9} s",
                    self.state.as_str(),
                    dwell.duration_s,
                    self.min_duration_s
                )
            },
            transform_metadata: event_validation_metadata(
                format!(
                    "dwell_time(channel={},state={},min_duration_s={})",
                    self.channel,
                    self.state.as_str(),
                    self.min_duration_s
                ),
                "dwell_time",
                vec![TransformParameterMetadata::float(
                    "min_duration_s",
                    self.min_duration_s,
                    "s",
                )],
            ),
        })
    }
}

impl TimeoutValidation {
    fn evaluate(&self, records: &[EventRecord]) -> Result<EventValidationResult> {
        validate_duration("start_time_s", self.start_time_s)?;
        validate_duration("max_time_s", self.max_time_s)?;
        let deadline = self.start_time_s + self.max_time_s + FLOAT_TOLERANCE;
        let event = matching_edges(records, &self.channel, self.direction)
            .into_iter()
            .find(|event| event.timestamp >= self.start_time_s && event.timestamp <= deadline);
        let outcome = if event.is_some() {
            Outcome::Pass
        } else {
            Outcome::Fail
        };
        let measured = event
            .map(|event| event.timestamp - self.start_time_s)
            .unwrap_or(self.max_time_s);
        Ok(EventValidationResult {
            requirement_id: self.id.clone(),
            validation: EventValidationKind::Timeout,
            outcome,
            channel: self.channel.clone(),
            measured_value: measured,
            required_value: self.max_time_s,
            unit: "s".to_string(),
            linked_event_ids: event
                .map(|event| vec![event.id.clone()])
                .unwrap_or_else(Vec::new),
            reason: if outcome == Outcome::Pass {
                format!(
                    "{} event occurred within {:.9} s",
                    self.direction.as_str(),
                    self.max_time_s
                )
            } else {
                format!(
                    "no {} event occurred within {:.9} s",
                    self.direction.as_str(),
                    self.max_time_s
                )
            },
            transform_metadata: event_validation_metadata(
                format!(
                    "timeout(channel={},direction={},start_time_s={},max_time_s={})",
                    self.channel,
                    self.direction.as_str(),
                    self.start_time_s,
                    self.max_time_s
                ),
                "timeout",
                vec![
                    TransformParameterMetadata::float("start_time_s", self.start_time_s, "s"),
                    TransformParameterMetadata::float("max_time_s", self.max_time_s, "s"),
                ],
            ),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct TransitionPoint {
    index: usize,
    previous: SignalState,
    state: SignalState,
}

#[derive(Debug, Clone, Copy)]
struct StateRun {
    start: usize,
    end: usize,
    state: SignalState,
    duration_s: f64,
}

fn transition_records(
    waveform: &Waveform,
    trace: &EventStateTrace,
    transform_id: &str,
    transform_name: &str,
    kind: EventKind,
    metadata: TransformStepMetadata,
    source_event_ids: Vec<String>,
) -> Vec<EventRecord> {
    transition_points(trace)
        .into_iter()
        .enumerate()
        .map(|(index, transition)| EventRecord {
            id: format!("{transform_id}_transition_{index}"),
            transform: transform_name.to_string(),
            kind: kind.clone(),
            channel: trace.channel.clone(),
            sample_index: transition.index,
            timestamp: waveform.time[transition.index],
            state: transition.state.as_str().to_string(),
            previous_state: Some(transition.previous.as_str().to_string()),
            direction: Some(direction(transition.previous, transition.state).to_string()),
            on_threshold_v: Some(trace.on_threshold_v),
            off_threshold_v: Some(trace.off_threshold_v),
            duration_s: None,
            count: None,
            source_event_ids: source_event_ids.clone(),
            transform_metadata: metadata.clone(),
        })
        .collect()
}

fn transition_points(trace: &EventStateTrace) -> Vec<TransitionPoint> {
    trace
        .states
        .windows(2)
        .enumerate()
        .filter_map(|(index, pair)| {
            (pair[0] != pair[1]).then_some(TransitionPoint {
                index: index + 1,
                previous: pair[0],
                state: pair[1],
            })
        })
        .collect()
}

fn remove_short_pulses(
    waveform: &Waveform,
    trace: &mut EventStateTrace,
    transform_id: &str,
    transform_name: &str,
    duration_limit_s: f64,
    metadata: TransformStepMetadata,
) -> Result<Vec<EventRecord>> {
    let runs = state_runs(waveform, trace)?;
    let mut rejected = Vec::new();

    for (run_index, run) in runs.iter().enumerate() {
        if run_index == 0 || run_index + 1 == runs.len() {
            continue;
        }
        let previous = runs[run_index - 1].state;
        let next = runs[run_index + 1].state;
        if previous == next && run.duration_s + FLOAT_TOLERANCE < duration_limit_s {
            for state in &mut trace.states[run.start..=run.end] {
                *state = previous;
            }
            rejected.push(EventRecord {
                id: format!("{transform_id}_rejected_pulse_{}", rejected.len()),
                transform: transform_name.to_string(),
                kind: EventKind::RejectedPulse,
                channel: trace.channel.clone(),
                sample_index: run.start,
                timestamp: waveform.time[run.start],
                state: run.state.as_str().to_string(),
                previous_state: Some(previous.as_str().to_string()),
                direction: None,
                on_threshold_v: Some(trace.on_threshold_v),
                off_threshold_v: Some(trace.off_threshold_v),
                duration_s: Some(run.duration_s),
                count: None,
                source_event_ids: Vec::new(),
                transform_metadata: metadata.clone(),
            });
        }
    }

    Ok(rejected)
}

fn state_runs(waveform: &Waveform, trace: &EventStateTrace) -> Result<Vec<StateRun>> {
    if trace.states.is_empty() {
        return Err(WaveformError::EmptyInput);
    }
    let mut runs = Vec::new();
    let mut start = 0;
    for index in 1..trace.states.len() {
        if trace.states[index] != trace.states[start] {
            runs.push(run(waveform, trace, start, index - 1));
            start = index;
        }
    }
    runs.push(run(waveform, trace, start, trace.states.len() - 1));
    Ok(runs)
}

fn run(waveform: &Waveform, trace: &EventStateTrace, start: usize, end: usize) -> StateRun {
    StateRun {
        start,
        end,
        state: trace.states[start],
        duration_s: waveform.time[end] - waveform.time[start],
    }
}

fn longest_state_run(waveform: &Waveform, trace: &EventStateTrace, state: SignalState) -> StateRun {
    state_runs(waveform, trace)
        .unwrap_or_default()
        .into_iter()
        .filter(|run| run.state == state)
        .max_by(|left, right| left.duration_s.total_cmp(&right.duration_s))
        .unwrap_or(StateRun {
            start: 0,
            end: 0,
            state,
            duration_s: 0.0,
        })
}

fn matching_edges<'a>(
    records: &'a [EventRecord],
    channel: &str,
    direction_filter: EdgeDirectionFilter,
) -> Vec<&'a EventRecord> {
    let explicit_edges = records
        .iter()
        .filter(|record| {
            record.channel == channel
                && record.kind == EventKind::Edge
                && record
                    .direction
                    .as_deref()
                    .is_some_and(|direction| direction_filter.matches(direction))
        })
        .collect::<Vec<_>>();

    if !explicit_edges.is_empty() {
        return explicit_edges;
    }

    records
        .iter()
        .filter(|record| {
            record.channel == channel
                && record.kind == EventKind::StateTransition
                && record
                    .direction
                    .as_deref()
                    .is_some_and(|direction| direction_filter.matches(direction))
        })
        .collect()
}

fn direction(previous: SignalState, state: SignalState) -> &'static str {
    match (previous, state) {
        (SignalState::Low, SignalState::High) => "rising",
        (SignalState::High, SignalState::Low) => "falling",
        _ => "stable",
    }
}

fn event_transform_metadata(
    history_label: impl Into<String>,
    name: impl Into<String>,
    parameters: Vec<TransformParameterMetadata>,
    stateful: bool,
) -> TransformStepMetadata {
    let mut metadata = TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        name,
        if stateful {
            TransformCategory::Stateful
        } else {
            TransformCategory::Event
        },
        parameters,
        TransformExecutionMetadata {
            sample_rate_required: true,
            stateful,
            causal: true,
            phase_effect: TransformPhaseEffect::Nonlinear,
            streaming_supported: false,
            offline_only: true,
        },
    );
    metadata.output_channels = TransformOutputChannels::event_records();
    metadata
}

fn event_validation_metadata(
    history_label: impl Into<String>,
    name: impl Into<String>,
    parameters: Vec<TransformParameterMetadata>,
) -> TransformStepMetadata {
    let mut metadata = TransformStepMetadata::implemented_desktop_with_execution(
        history_label,
        name,
        TransformCategory::Validation,
        parameters,
        TransformExecutionMetadata {
            sample_rate_required: true,
            stateful: false,
            causal: true,
            phase_effect: TransformPhaseEffect::Nonlinear,
            streaming_supported: false,
            offline_only: true,
        },
    );
    metadata.output_channels = TransformOutputChannels::validation_records();
    metadata
}

fn ensure_trace_channel(channel: &str, trace: &EventStateTrace) -> Result<()> {
    if channel == trace.channel {
        Ok(())
    } else {
        Err(WaveformError::InvalidParameter {
            name: "event_transforms.channel".to_string(),
            reason: format!(
                "event transform channel `{channel}` does not match active state trace channel `{}`",
                trace.channel
            ),
        })
    }
}

fn validate_duration(name: &str, value: f64) -> Result<()> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: "must be a finite non-negative value".to_string(),
        })
    }
}

fn missing_trace_error() -> WaveformError {
    WaveformError::InvalidParameter {
        name: "event_transforms".to_string(),
        reason: "event transforms require a preceding schmitt_trigger state trace".to_string(),
    }
}

fn rule_engine_event_error(error: ferrisoxide_rule_engine::RuleEngineError) -> WaveformError {
    match error {
        ferrisoxide_rule_engine::RuleEngineError::EmptyInput => WaveformError::EmptyInput,
        ferrisoxide_rule_engine::RuleEngineError::MissingChannel { channel } => {
            WaveformError::InvalidParameter {
                name: "event_transforms.channel".to_string(),
                reason: format!("missing channel `{channel}`"),
            }
        }
        ferrisoxide_rule_engine::RuleEngineError::InvalidWaveform { reason } => {
            WaveformError::InvalidWaveform { reason }
        }
        ferrisoxide_rule_engine::RuleEngineError::InvalidParameter { name, reason } => {
            WaveformError::InvalidParameter { name, reason }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Channel, Unit};

    fn switch_waveform(samples: Vec<f64>) -> Waveform {
        Waveform::new(
            vec![0.0, 0.001, 0.002, 0.003, 0.004, 0.005, 0.006],
            vec![Channel::new("switch_v", Unit::volts(), samples)],
        )
        .expect("test waveform should be valid")
    }

    fn schmitt() -> EventTransformStep {
        EventTransformStep::SchmittTrigger(SchmittTriggerTransform {
            id: "switch_state".to_string(),
            channel: "switch_v".to_string(),
            on_threshold_v: 3.0,
            off_threshold_v: 2.0,
            initial_state: SignalState::Low,
        })
    }

    #[test]
    fn schmitt_trigger_records_edges_and_prevents_chatter() {
        let waveform = switch_waveform(vec![0.0, 2.5, 3.2, 2.6, 1.9, 3.4, 3.5]);
        let evaluation = evaluate_event_pipeline(
            &waveform,
            &[
                schmitt(),
                EventTransformStep::EdgeExtraction(EdgeExtractionTransform {
                    id: "switch_edges".to_string(),
                    channel: "switch_v".to_string(),
                }),
            ],
            &[],
        )
        .expect("event pipeline should evaluate");

        let edge_directions = evaluation
            .records
            .iter()
            .filter(|record| record.kind == EventKind::Edge)
            .filter_map(|record| record.direction.as_deref())
            .collect::<Vec<_>>();

        assert_eq!(edge_directions, vec!["rising", "falling", "rising"]);
        assert!(evaluation.records.iter().all(|record| {
            record.on_threshold_v == Some(3.0) && record.off_threshold_v == Some(2.0)
        }));
        assert_eq!(
            evaluation.records[0].transform_metadata.category,
            TransformCategory::Stateful
        );
        assert_eq!(evaluation.records[0].transform_metadata.sequence_index, 0);
        assert!(evaluation
            .records
            .iter()
            .filter(|record| record.kind == EventKind::Edge)
            .all(|record| record.transform_metadata.sequence_index == 1));
    }

    #[test]
    fn debounce_and_glitch_removal_reject_short_pulses() {
        let waveform = switch_waveform(vec![0.0, 3.5, 0.0, 0.0, 3.5, 3.5, 3.5]);
        let evaluation = evaluate_event_pipeline(
            &waveform,
            &[
                schmitt(),
                EventTransformStep::Debounce(DebounceTransform {
                    id: "debounce".to_string(),
                    channel: "switch_v".to_string(),
                    min_duration_s: 0.002,
                }),
                EventTransformStep::GlitchRemoval(GlitchRemovalTransform {
                    id: "glitch".to_string(),
                    channel: "switch_v".to_string(),
                    max_duration_s: 0.002,
                }),
                EventTransformStep::EdgeExtraction(EdgeExtractionTransform {
                    id: "clean_edges".to_string(),
                    channel: "switch_v".to_string(),
                }),
            ],
            &[],
        )
        .expect("event pipeline should evaluate");

        assert!(evaluation.records.iter().any(|record| {
            record.kind == EventKind::RejectedPulse
                && record.transform == "debounce"
                && record.duration_s == Some(0.001)
        }));
        let clean_edges = evaluation
            .records
            .iter()
            .filter(|record| record.transform == "edge_extraction")
            .count();
        assert_eq!(clean_edges, 1);
    }

    #[test]
    fn bounce_detection_and_event_validations_link_evidence() {
        let waveform = switch_waveform(vec![0.0, 3.5, 0.0, 3.5, 0.0, 3.5, 3.5]);
        let evaluation = evaluate_event_pipeline(
            &waveform,
            &[
                schmitt(),
                EventTransformStep::EdgeExtraction(EdgeExtractionTransform {
                    id: "edges".to_string(),
                    channel: "switch_v".to_string(),
                }),
                EventTransformStep::BounceDetection(BounceDetectionTransform {
                    id: "bounce".to_string(),
                    channel: "switch_v".to_string(),
                    window_s: 0.004,
                }),
            ],
            &[
                EventValidationStep::MissingPulse(MissingPulseValidation {
                    id: "must_rise".to_string(),
                    channel: "switch_v".to_string(),
                    direction: EdgeDirectionFilter::Rising,
                    expected_count: 1,
                }),
                EventValidationStep::ExtraPulse(ExtraPulseValidation {
                    id: "no_extra_rise".to_string(),
                    channel: "switch_v".to_string(),
                    direction: EdgeDirectionFilter::Rising,
                    max_count: 1,
                }),
                EventValidationStep::DwellTime(DwellTimeValidation {
                    id: "high_dwell".to_string(),
                    channel: "switch_v".to_string(),
                    state: SignalState::High,
                    min_duration_s: 0.001,
                }),
                EventValidationStep::Timeout(TimeoutValidation {
                    id: "rise_timeout".to_string(),
                    channel: "switch_v".to_string(),
                    direction: EdgeDirectionFilter::Rising,
                    start_time_s: 0.0,
                    max_time_s: 0.002,
                }),
            ],
        )
        .expect("event pipeline should evaluate");

        let bounce = evaluation
            .records
            .iter()
            .find(|record| record.kind == EventKind::Bounce)
            .expect("bounce should be detected");
        assert_eq!(bounce.count, Some(4));
        assert_eq!(evaluation.validations[0].outcome, Outcome::Pass);
        assert_eq!(evaluation.validations[1].outcome, Outcome::Fail);
        assert_eq!(evaluation.validations[2].outcome, Outcome::Pass);
        assert_eq!(evaluation.validations[3].outcome, Outcome::Pass);
        assert_eq!(
            evaluation.validations[0].transform_metadata.sequence_index,
            3
        );
        assert_eq!(
            evaluation.validations[3].transform_metadata.sequence_index,
            6
        );
        assert!(!evaluation.validations[0].linked_event_ids.is_empty());
    }

    #[test]
    fn event_pipeline_rejects_invalid_parameters() {
        let waveform = switch_waveform(vec![0.0, 3.5, 0.0, 3.5, 0.0, 3.5, 3.5]);
        let result = evaluate_event_pipeline(
            &waveform,
            &[EventTransformStep::SchmittTrigger(
                SchmittTriggerTransform {
                    id: "bad".to_string(),
                    channel: "switch_v".to_string(),
                    on_threshold_v: 2.0,
                    off_threshold_v: 2.0,
                    initial_state: SignalState::Low,
                },
            )],
            &[],
        );
        assert!(matches!(
            result,
            Err(WaveformError::InvalidParameter { .. })
        ));

        let result = evaluate_event_pipeline(
            &waveform,
            &[EventTransformStep::Debounce(DebounceTransform {
                id: "debounce".to_string(),
                channel: "switch_v".to_string(),
                min_duration_s: 0.001,
            })],
            &[],
        );
        assert!(matches!(
            result,
            Err(WaveformError::InvalidParameter { .. })
        ));
    }
}
