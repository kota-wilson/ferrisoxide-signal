# MVP Usage Sketch

This document describes the current MVP usage. The command shape can still change before a public release.

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format text
```

Current output:

```text
Waveform Analysis Report
Input: examples/basic-waveform.csv
Overall: Pass
Criteria:
- min_voltage_input_v: Pass measured=0.000000 V threshold=0.000000 V
- max_voltage_input_v: Pass measured=5.000000 V threshold=5.500000 V
```

Supported MVP filters:

- `--moving-average <samples>` applies a trailing moving average to each channel.
- `--low-pass <hz>` applies a simple first-order low-pass filter to each channel.

JSON output:

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --config examples/basic-config.toml \
  --format json
```

Explicit CLI criteria remain available for one-off checks:

```bash
cargo run --bin wra -- analyze \
  --input examples/basic-waveform.csv \
  --time-column time \
  --channels input_v \
  --moving-average 2 \
  --min input_v:0.0 \
  --max input_v:5.5
```

Richer CSV dialect support and stable numerical filter guarantees remain planned.
