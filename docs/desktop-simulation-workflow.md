# Desktop Simulation Workflow

Status: implemented fixture-driven workflow for M9-006 / issue #82.

Command: `ferrisoxide-signal simulate`

## Purpose

The desktop simulation workflow connects the controller-in-the-loop building blocks that were previously separate:

```text
production control config
+ test verification config
+ channel map
+ fixture CSV input
-> fixture DAQ frames
-> virtual controller simulation trace
-> waveform verification evidence
-> text or JSON workflow report
```

This is a software-only desktop workflow. It is useful for controller design review, configuration tuning, and repeatable fixture-based evidence before final controller hardware or live DAQ integration exists.

## Command

```bash
cargo run --quiet --bin ferrisoxide-signal -- simulate \
  --input tests/e2e/heated_actuator/input/passing_run.csv \
  --control-config examples/control-config/production-control-config.toml \
  --verification-config examples/test-verification-config/test-verification-config.toml \
  --channel-map examples/simulation/heated-actuator-channel-map.toml \
  --format json
```

Optional:

```bash
--mode normal
--format text
--output-json /tmp/ferrisoxide-simulation-report.json
```

`--output-json` writes a new file and refuses to overwrite an existing artifact.

## Required Inputs

| Input | Role |
|---|---|
| Fixture CSV | DAQ-like sample input with time and signal columns. |
| Production control config | Defines controller inputs, outputs, thresholds, modes, state machines, actions, and fault responses. |
| Test verification config | Defines channels, expected transitions, limits, timing windows, evidence settings, and report settings. |
| Channel map | Connects fixture CSV columns to logical verification channels and maps production-control input IDs to those logical channels. |

## Channel Map

Example: `examples/simulation/heated-actuator-channel-map.toml`

```toml
[simulation]
mode = "normal"
time_column = "time_s"
time_unit = "s"

[[channels]]
id = "command"
column = "command_v"
unit = "V"

[[channels]]
id = "feedback"
column = "actuator_feedback_v"
unit = "V"

[[control_inputs]]
input = "command"
channel = "command"

[[control_inputs]]
input = "feedback"
channel = "feedback"
```

Validation rules:

- `simulation.mode` must be non-empty.
- `simulation.time_column` must be non-empty.
- `simulation.time_unit` must be `s`.
- Channel IDs and source columns must be unique.
- Every production-control input must be mapped.
- Every verification channel must appear in the channel map.
- Verification channel column and unit values must match the channel map.

## Output

`--format json` returns:

```text
workflow
simulation_trace
verification_evidence
```

The `workflow` section records the loaded input paths, selected mode, loaded channel map, and non-certification scope note.

The `simulation_trace` section is produced by `ferrisoxide-simulator` and includes:

- sample index
- timestamp
- mode
- state-machine state
- transitions
- actions
- outputs
- faults

The `verification_evidence` section is an ordinary FerrisOxide analysis report and includes:

- overall pass/fail
- measurements
- criterion results
- measured value
- required value
- sample index
- timestamp
- channel

## Text Example

```text
Desktop Simulation Workflow
Input: tests/e2e/heated_actuator/input/passing_run.csv
Mode: normal
Simulation Frames: 9
Verification Overall: Pass
Simulation Transitions:
- sample_index=3 timestamp=1.000000 machine=actuator_control transition=command_to_heating idle -> heating
- sample_index=4 timestamp=1.020000 machine=actuator_control transition=feedback_reached heating -> idle
Verification Criteria:
- REQ-001: Pass channel=feedback measured=0.020000 required=0.050000 sample_index=4 timestamp=1.020000
```

## Current Verification Mapping

The workflow converts supported test verification schema entries into existing FerrisOxide criteria:

| Verification schema item | Runtime criterion |
|---|---|
| `expected_transitions` with reference channel and max latency | response latency |
| `voltage_limits` | min/max voltage |
| `pulse_widths` | pulse width |
| `transient_limits` | transient event duration |
| `dropout_limits` | dropout duration |
| `stable_state_requirements` | stable-state duration |

Timing windows are applied to transient and dropout duration checks. Response-latency, pulse-width, stable-state, and voltage-limit checks currently evaluate against the loaded waveform as a whole.

When a criterion needs a state threshold, the workflow uses the verification channel low/high thresholds when present. If those are absent, it can use a compatible voltage limit such as `min_v` for an expected high state.

## Scope Limits

This workflow does not add:

- GUI behavior,
- live DAQ SDK integration,
- production RTOS binding,
- hardware HAL access,
- hardware timing guarantees,
- safety or flight certification evidence.

It is desktop software validation evidence only.

## Hand-Off Note

Role: Software Architect / Core Software Engineer / Verification and Validation Engineer
Goal: Document the M9-006 desktop simulation workflow.
Files changed: `docs/desktop-simulation-workflow.md`.
Checks run: See `docs/validation-log.md`.
Status: Implemented and validated locally; PR, protected CI, merge, and issue #82 closure pending.
Known gaps: No live DAQ SDK, GUI, deployment package, RTOS runtime binding, target hardware execution, or certification evidence.
Next recommended step: Open PR with `Fixes #82`, wait for required CI, and merge only after checks pass.
