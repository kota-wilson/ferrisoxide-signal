# Measurement Engine Known-Answer Fixture

Date: 2026-05-31

Fixture: `known_answer_measurements.csv`

Config: `known_answer_measurements.toml`

Expected report: `../reports/measurement_engine_known_answer.json`

## Scope

This fixture validates measurement-engine behavior with deterministic software-generated samples. It is not hardware qualification, DAQ validation, production performance evidence, or certification evidence.

## Time-Axis Assumptions

- Time unit: seconds.
- Sample interval: 0.001 s.
- Time samples are strictly increasing.
- Duration measurements use adjacent timestamp differences and do not interpolate between samples.

## Tolerance Policy

- Voltage tolerance: 0.0 V.
- Time tolerance: 0.0005 s.
- Time tolerance allows the 0.002 s measured dropout/rise/fall durations to satisfy 0.0015 s requirements.

## Expected Measurements

| Criterion | Channel | Method | Expected measured value | Unit | Evidence sample index | Evidence timestamp | Requirement | Expected outcome |
|---|---|---|---:|---|---:|---:|---|---|
| `control_transition_count` | `control_v` | `state_transition_count` | 4.0 | transitions | 2 | 0.002 s | equal to 4 | Pass |
| `control_high_pulse_width` | `control_v` | `state_run_duration` shortest high run | 0.002 | s | 2 | 0.002 s | >= 0.0015 s with 0.0005 s tolerance | Pass |
| `control_stable_low` | `control_v` | `state_run_duration` longest low run | 0.002 | s | 4 | 0.004 s | >= 0.002 s with 0.0005 s tolerance | Pass |
| `supply_dropout_tolerance` | `supply_v` | `state_run_duration` longest unintended low dropout | 0.002 | s | 3 | 0.003 s | <= 0.0015 s with 0.0005 s tolerance | Pass |
| `rise_time_tolerance` | `rise_v` | `edge_time` rise from 0.5 V to 4.5 V | 0.002 | s | 3 | 0.003 s | <= 0.0015 s with 0.0005 s tolerance | Pass |
| `fall_time_tolerance` | `fall_v` | `edge_time` fall from 4.5 V to 0.5 V | 0.002 | s | 7 | 0.007 s | <= 0.0015 s with 0.0005 s tolerance | Pass |

## Hand-Off Note

Role: Verification and Validation Engineer
Goal: Provide independently documented expected values for measurement-engine validation.
Files changed: `validation/measurement_engine/`
Checks run: Expected values are verified by exact JSON report comparison in the workspace test suite.
Status: Ready for validation.
Known gaps: This is synthetic software evidence only; external capture corpora remain future work.
Next recommended step: Keep this fixture stable unless measurement semantics intentionally change.
