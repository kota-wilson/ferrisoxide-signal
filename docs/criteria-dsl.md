# Criteria DSL Direction

Date: 2026-05-31

Issue: #46, `M6-004 Document criteria DSL direction for engineering measurements`

## Scope

This document defines the intended direction for a future measurement-backed criteria DSL. It is documentation only. It does not implement a new parser, plugin runtime, GUI, DAQ integration, ML, RTOS expansion, hardware qualification, or certification claim.

## Current Compatibility Baseline

Existing `[[criteria]]` TOML entries remain supported and should continue to work:

```toml
[[criteria]]
id = "rise_time_tolerance"
type = "rise_fall_time"
channel = "rise_v"
direction = "rise"
low_threshold_v = 0.5
high_threshold_v = 4.5
max_duration_s = 0.0015
```

Future DSL work should be additive. Existing explicit fields should not be silently reinterpreted.

## Measurement-Backed Criteria Concept

The future criteria model should separate three concepts:

| Concept | Meaning | Example |
|---|---|---|
| Measurement | What is measured from the waveform. | `rise_time`, `pulse_width`, `state_transition_count` |
| Comparator | How the measured value is compared. | `less_than_or_equal` |
| Requirement value | The required engineering value with explicit unit. | `{ value = 0.005, unit = "s" }` |

Example direction:

```toml
[[criteria]]
id = "rise_time_max_5ms"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
```

## Initial Operator Vocabulary

The first operator vocabulary should be small and auditable:

| Operator | Meaning |
|---|---|
| `less_than` | Measured value must be strictly below the required value. |
| `less_than_or_equal` | Measured value may equal or be below the required value. |
| `greater_than` | Measured value must be strictly above the required value. |
| `greater_than_or_equal` | Measured value may equal or be above the required value. |
| `equal_to` | Measured value must equal the required value after the configured tolerance model is applied. |

Tolerance handling should remain explicit and reportable. The report must show the measured value, required value, unit, tolerance used, sample index, timestamp, channel, and measurement ID.

## Explicit Units Before Shorthand Strings

Numeric values with explicit `unit` fields are preferred before accepting shorthand strings such as `10ms`.

Preferred:

```toml
value = 0.010
unit = "s"
```

Deferred shorthand:

```toml
value = "10ms"
```

Reasons:

- Explicit units are easier to validate with TOML deserialization.
- Errors can identify the exact missing or unsupported field.
- Reports can preserve units without parsing ambiguity.
- Engineering review is clearer when unit conversion rules are explicit.
- Shorthand strings require a unit parser, rounding policy, and compatibility tests.

## Candidate Measurement Types

Initial DSL measurement types should map to existing measurement primitives:

| Measurement type | Existing backing logic |
|---|---|
| `minimum_sample` | `minimum_sample` |
| `maximum_sample` | `maximum_sample` |
| `state_transition_count` | `count_state_transitions` |
| `pulse_width` | `state_run_extremum` with `shortest` or `longest` selection |
| `stable_state_duration` | `state_run_extremum` |
| `transient_event_duration` | `state_run_extremum` over the opposite state |
| `rise_time` | `measure_rise_time` |
| `fall_time` | `measure_fall_time` |

Future measurement types such as duty cycle, frequency, period, overshoot, undershoot, RMS, peak-to-peak, and noise floor should be added only after known-answer validation fixtures exist.

## Non-Goals

- No plugin runtime.
- No GUI or interactive form builder.
- No DAQ integration.
- No machine learning.
- No RTOS expansion.
- No unit-shorthand parser in this documentation slice.
- No certification, hardware qualification, or flight-readiness claim.

## Gate Decision

- Gate: Documentation Gate for M6-004.
- Decision: Pass.
- Reason: The future DSL direction defines concepts, operator vocabulary, unit policy, compatibility expectations, and non-goals without changing runtime behavior.
- Residual risk: The future implementation still needs parser design, migration tests, and user feedback before becoming public API.
- Next owner: Software Architect / Core Software Engineer.

## Hand-Off Note

Role: Software Architect / Documentation Engineer
Goal: Document the criteria DSL direction before expanding syntax.
Files changed: `docs/criteria-dsl.md`
Checks run: Documentation review plus workspace validation in the M6 completion branch.
Status: Ready for review.
Known gaps: No DSL parser is implemented; this is direction only.
Next recommended step: Keep existing `[[criteria]]` compatibility until a separate implementation issue is approved.
