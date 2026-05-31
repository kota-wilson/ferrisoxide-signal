# DSL Parity Golden Report Evidence

Date: 2026-05-31

Issue: #58, `M7-004 Add DSL legacy-parity golden JSON tests`

## Scope

These fixtures prove that equivalent legacy and DSL criteria produce exact JSON report evidence for representative software-validation waveforms.

This is not hardware qualification, DAQ validation, performance evidence, or certification evidence.

## Exact Parity Cases

| Case | CSV fixture | Legacy config | DSL config | Expected JSON |
|---|---|---|---|---|
| Clean square wave | `tests/fixtures/clean_square_wave.csv` | `tests/configs/criteria-engine-pass.toml` | `tests/configs/criteria-engine-pass-dsl.toml` | `tests/golden/criteria_engine_pass.json` |
| Dropout transient event | `tests/fixtures/dropout_event.csv` | `tests/configs/transient-event-dropout-fail.toml` | `tests/configs/transient-event-dropout-fail-dsl.toml` | `tests/golden/transient_event_dropout_fail.json` |
| Slow rise/fall signal | `tests/fixtures/slow_rise_fall_signal.csv` | `tests/configs/slow-rise-fail.toml` | `tests/configs/slow-rise-fail-dsl.toml` | `tests/golden/slow_rise_fail.json` |
| Measurement-engine known answer | `validation/measurement_engine/known_answer_measurements.csv` | `validation/measurement_engine/known_answer_measurements.toml` | `validation/measurement_engine/known_answer_measurements_dsl.toml` | `validation/reports/measurement_engine_known_answer.json` |

## Evidence Fields Locked By Exact JSON

| Criterion | Channel | Measurement ID | Measured value | Required value | Unit | Sample index | Timestamp | Expected outcome |
|---|---|---|---:|---:|---|---:|---:|---|
| `control_transition_count` | `control_v` | `control_transition_count_measurement` | 4.0 | 4.0 | transitions | 2 | 0.002 | Pass |
| `control_high_pulse_width` | `control_v` | `control_high_pulse_width_measurement` | 0.002 | 0.002 | s | 7 | 0.007 | Pass |
| `control_stable_low` | `control_v` | `control_stable_low_measurement` | 0.002 | 0.002 | s | 5 | 0.005 | Pass |
| `supply_dropout_max_1ms` | `supply_v` | `supply_dropout_max_1ms_measurement` | 0.002 | 0.001 | s | 3 | 0.003 | Fail |
| `signal_rise_time_max_3ms` | `signal_v` | `signal_rise_time_max_3ms_measurement` | 0.005 | 0.003 | s | 6 | 0.006 | Fail |
| `signal_fall_time_max_3ms` | `signal_v` | `signal_fall_time_max_3ms_measurement` | 0.004 | 0.003 | s | 12 | 0.012 | Fail |
| `supply_dropout_tolerance` | `supply_v` | `supply_dropout_tolerance_measurement` | 0.002 | 0.0015 | s | 3 | 0.003 | Pass |
| `rise_time_tolerance` | `rise_v` | `rise_time_tolerance_measurement` | 0.002 | 0.0015 | s | 3 | 0.003 | Pass |
| `fall_time_tolerance` | `fall_v` | `fall_time_tolerance_measurement` | 0.002 | 0.0015 | s | 7 | 0.007 | Pass |

## Compatibility Notes

- DSL criteria reuse the same criterion IDs as their legacy counterparts so `measurement_id` links remain byte-for-byte identical.
- DSL `state_transition_count` requirements use `unit = "count"` in TOML while the existing report evidence unit remains `transitions` for legacy compatibility.
- DSL pulse-width parity uses the same selected evidence as the legacy report. For the clean square-wave config, the legacy pass evidence reports the minimum-width side of the configured min/max limits.

## Hand-Off Note

Role: Verification and Validation Engineer
Goal: Document expected evidence for DSL/legacy parity golden tests.
Files changed: `tests/golden/dsl-parity.md`
Checks run: Exact JSON parity tests in `crates/ferrisoxide-core/tests/criteria_engine.rs`.
Status: Ready for validation.
Known gaps: Broader DSL examples and schema reference remain in issues #60 and #61.
Next recommended step: Keep these cases stable unless criteria evidence semantics intentionally change.
