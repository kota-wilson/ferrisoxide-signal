# MVP Implementation Plan

## MVP Goal

Deliver a CLI-driven Rust MVP that can parse a simple CSV waveform fixture into typed waveform data, apply planned filter interfaces, evaluate basic min/max voltage criteria, and produce a report model.

Current status: M1-M3 dependency-reviewed slice is implemented and locally validated. M4 is approved for publication execution after release readiness review.

## Milestone M1: Foundation

Owner: Core Software Engineer

Files:

- `crates/wra-core/src/model.rs`
- `crates/wra-core/src/csv.rs`
- `crates/wra-core/src/error.rs`
- `crates/wra-cli/src/main.rs`
- `tests/fixtures/basic_waveform.csv`

Acceptance criteria:

- `Waveform` validates sample lengths.
- CSV parser interface exists with a simple MVP parser.
- CLI accepts explicit local analysis arguments.
- Unit tests exist for model and parser basics.

## Milestone M2: Filters

Owner: Systems Engineer

Files:

- `crates/wra-core/src/filter.rs`
- `crates/wra-core/src/filter.rs` unit tests

Acceptance criteria:

- Filter trait exists.
- Moving average filter has basic implementation.
- Low-pass filter has a simple first-order implementation and must not claim validated frequency response yet.
- Tests use synthetic fixtures with tolerances.

## Milestone M3: Criteria And Reports

Owner: Core Software Engineer / Documentation Engineer

Files:

- `crates/wra-core/src/criteria.rs`
- `crates/wra-core/src/analysis.rs`
- `crates/wra-core/src/report.rs`
- `examples/basic-config.toml`

Acceptance criteria:

- Min/max voltage criteria can be represented.
- Analysis result reports pass/fail, measured value, threshold, and reason.
- Report model can render text output.
- CLI accepts explicit min/max criteria flags and TOML config files.
- Report model can render JSON output.

## Milestone M4: Open-Source Readiness

Owner: GitHub Maintainer Specialist / Release Engineer

Acceptance criteria:

- CI passes.
- README has usage example.
- Contribution guide and security docs are current.
- License confirmed.
- Changelog has MVP entry.

## Validation Commands

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Approval Stops

- Stop before adding dependencies.
- Stop before full GUI planning.
- Stop before public repository publication.
