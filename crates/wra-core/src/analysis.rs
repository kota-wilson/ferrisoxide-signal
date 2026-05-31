use crate::criteria::{Criterion, CriterionCheck};
use crate::error::{Result, WaveformError};
use crate::model::Waveform;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Pass,
    Fail,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AnalysisResult {
    pub criterion_id: String,
    pub outcome: Outcome,
    pub measured_value: f64,
    pub threshold: f64,
    pub unit: String,
    pub reason: String,
}

pub fn evaluate_criteria(
    waveform: &Waveform,
    criteria: &[Criterion],
) -> Result<Vec<AnalysisResult>> {
    criteria
        .iter()
        .map(|criterion| evaluate_criterion(waveform, criterion))
        .collect()
}

fn evaluate_criterion(waveform: &Waveform, criterion: &Criterion) -> Result<AnalysisResult> {
    let channel =
        waveform
            .channel(criterion.channel())
            .ok_or_else(|| WaveformError::MissingColumn {
                column: criterion.channel().to_string(),
            })?;

    match &criterion.check {
        CriterionCheck::MinimumVoltage { threshold_v, .. } => {
            let measured = channel
                .samples
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min);
            let outcome = if measured >= *threshold_v {
                Outcome::Pass
            } else {
                Outcome::Fail
            };
            Ok(AnalysisResult {
                criterion_id: criterion.id.clone(),
                outcome,
                measured_value: measured,
                threshold: *threshold_v,
                unit: "V".to_string(),
                reason: format!("minimum observed voltage was {measured:.6} V"),
            })
        }
        CriterionCheck::MaximumVoltage { threshold_v, .. } => {
            let measured = channel
                .samples
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);
            let outcome = if measured <= *threshold_v {
                Outcome::Pass
            } else {
                Outcome::Fail
            };
            Ok(AnalysisResult {
                criterion_id: criterion.id.clone(),
                outcome,
                measured_value: measured,
                threshold: *threshold_v,
                unit: "V".to_string(),
                reason: format!("maximum observed voltage was {measured:.6} V"),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Channel, Unit};

    fn waveform() -> Waveform {
        Waveform::new(
            vec![0.0, 0.1, 0.2],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 3.3, 5.0])],
        )
        .expect("test waveform should be valid")
    }

    #[test]
    fn evaluates_minimum_and_maximum_voltage_criteria() {
        let results = evaluate_criteria(
            &waveform(),
            &[
                Criterion::minimum_voltage("min", "input_v", 0.0),
                Criterion::maximum_voltage("max", "input_v", 5.5),
            ],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Pass);
        assert_eq!(results[1].outcome, Outcome::Pass);
    }

    #[test]
    fn fails_when_voltage_exceeds_threshold() {
        let results = evaluate_criteria(
            &waveform(),
            &[Criterion::maximum_voltage("max", "input_v", 4.5)],
        )
        .expect("criteria should evaluate");

        assert_eq!(results[0].outcome, Outcome::Fail);
        assert_eq!(results[0].measured_value, 5.0);
    }
}
