# Measurement Engine Validation

This directory contains synthetic known-answer fixtures for the reusable measurement layer.

| Case | CSV | Config | Expected Report | Expected Measurements |
|---|---|---|---|---|
| Multi-measurement known-answer fixture | `known_answer_measurements.csv` | `known_answer_measurements.toml` | `../reports/measurement_engine_known_answer.json` | `expected-measurements.md` |

The fixture covers:

- State transition count.
- Pulse width.
- Transient/dropout duration.
- Stable-state duration.
- Rise time.
- Fall time.
- Time tolerance behavior.
- Strictly increasing time-axis assumptions.

These fixtures are software validation artifacts only. They are not hardware qualification, DAQ validation, production performance evidence, or certification evidence.
