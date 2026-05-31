# M7-004 DSL Parity Golden Tests Pipeline Report

Date: 2026-05-31

Branch: `feature/m7-004-dsl-parity-golden-tests`

Milestone: #7, `v0.5.0: Measurement-Backed Criteria DSL`

Issue: #58, `M7-004 Add DSL legacy-parity golden JSON tests`

Pull request: Pending.

Status: Implemented and locally validated; PR/CI pending.

## Scope

This slice proves representative DSL configs produce byte-for-byte identical JSON report evidence to equivalent legacy configs.

In scope:

- Add DSL sibling configs for clean square wave, dropout transient event, slow rise/fall, and measurement-engine known-answer cases.
- Compare legacy-rendered JSON, DSL-rendered JSON, and existing golden JSON exactly.
- Keep existing golden JSON files unchanged.
- Document expected measured values, required values, sample index, timestamp, channel, and measurement IDs.

Out of scope:

- Full invalid DSL config matrix for issue #59.
- User-facing DSL examples, migration docs, or schema reference for issues #60 and #61.
- Broad validation-corpus expansion.
- Hardware qualification, DAQ validation, performance claims, or certification evidence.

## Research

- Owner role: Software Architect / V&V Engineer
- Artifact: Issue #58, existing golden JSON reports, and M7-003 runtime DSL evaluator.
- Evidence: Issue #57 closed in PR #101, making DSL criteria executable through existing measurement evidence.
- Gate: Intake Gate.
- Decision: Pass.
- Residual risk: Golden parity can prove representative equivalence but not every possible DSL combination.
- Next owner: Software Architect.

## Requirements

- Owner role: Software Architect / Test Automation Engineer
- Artifact: WRA-RQ-039 and WRA-RQ-040 in `requirements.md`; traceability rows in `traceability-matrix.md`.
- Requirements: Equivalent DSL and legacy configs shall preserve pass/fail decisions, measurement records, result fields, JSON report evidence, and legacy config compatibility.
- Gate: Requirements Traceability Gate.
- Decision: Pass.
- Residual risk: Invalid-config coverage remains WRA-RQ-041 / issue #59.
- Next owner: Software Architect.

## Architecture

- Owner role: Software Architect
- Artifact: DSL sibling TOML configs and exact report tests.
- Design: Keep report schema and legacy configs unchanged; add DSL configs with matching criterion IDs so `measurement_id` values and result evidence remain identical.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: Future schema changes must update both legacy and DSL golden paths together.
- Next owner: Abstraction Review Engineer.

## Abstraction Review

- Owner role: Abstraction Review Engineer
- Artifact: This report and test fixture diff.
- Review: The slice is test-evidence only and does not add new DSL semantics, new measurements, new parser behavior, or docs beyond parity evidence notes.
- Gate: Granularity Gate.
- Decision: Pass.
- Residual risk: User-facing migration guidance remains deferred to #60/#61.
- Next owner: Test Automation Engineer.

## Implementation

- Owner role: Test Automation Engineer
- Artifact: `tests/configs/*-dsl.toml`, `validation/measurement_engine/known_answer_measurements_dsl.toml`, `tests/golden/dsl-parity.md`, and `crates/ferrisoxide-core/tests/criteria_engine.rs`.
- Behavior:
  - `assert_legacy_and_dsl_reports_match` renders both configs and compares them to the same expected JSON.
  - Clean square wave parity covers transition count, pulse width, and stable-state duration.
  - Dropout parity covers transient-event duration failure evidence.
  - Slow rise/fall parity covers edge-time failure evidence.
  - Measurement-engine parity covers the broader known-answer fixture with tolerances.
- Gate: Implementation Gate.
- Decision: Pass locally.
- Residual risk: Protected CI remains pending.
- Next owner: Test Automation Engineer.

## Testing

- Owner role: Test Automation Engineer
- Artifact: Exact JSON parity tests.
- Evidence:
  - `cargo test -p ferrisoxide-core --test criteria_engine`: Pass; 15 tests passed.
  - `cargo fmt`: Pass.
  - `cargo fmt --check`: Pass.
  - `cargo test --workspace`: Pass; 101 tests passed.
  - `cargo clippy --workspace --all-targets -- -D warnings`: Pass.
  - `git diff --check`: Pass.
  - Existing golden tests still pass.
  - DSL parity tests compare legacy output, DSL output, and expected golden JSON exactly.
- Gate: Testing Gate.
- Decision: Pass locally.
- Residual risk: Protected CI remains pending until PR creation.
- Next owner: Verification and Validation Engineer.

## Verification And Validation

- Owner role: Verification and Validation Engineer
- Artifact: `tests/golden/dsl-parity.md`.
- Verification: Test artifacts list expected measurement IDs, channels, measured values, required values, sample indices, timestamps, units, and outcomes locked by exact JSON.
- Validation: This is synthetic software-validation evidence only; it is not hardware validation, DAQ validation, RTOS validation, production readiness, or certification evidence.
- Gate: V&V Gate.
- Decision: Pass locally.
- Residual risk: Broader external waveform validation remains future work.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Fixture and test review.
- Evidence: DSL parity configs reuse existing criterion IDs and golden reports, reducing report-consumer migration risk.
- Gate: QA Gate.
- Decision: Pass locally.
- Residual risk: Pulse-width min/max semantics need user-facing documentation before broad DSL promotion.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Dependency and fixture review.
- Evidence: No new dependencies, unsafe code, parser language, network surface, DAQ SDK, HAL, RTOS SDK, or FFI were added.
- Gate: Security Gate.
- Decision: Pass locally.
- Residual risk: Future config export/import packages require separate supply-chain and checksum review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Test inspection.
- Evidence: Added tests render small fixtures only; no benchmark or runtime performance claim is introduced.
- Gate: Performance Gate.
- Decision: Pass locally.
- Residual risk: Large-file DSL parity remains out of scope.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: `tests/golden/dsl-parity.md`, requirements, traceability, risk register, project state, and this report.
- Evidence: Test artifact documents the expected evidence fields for auditability.
- Gate: Documentation Gate.
- Decision: Pass locally.
- Residual risk: User-facing docs remain #60/#61.
- Next owner: Code Reviewer.

## Code Review

- Owner role: Code Review Engineer
- Artifact: Local code review.
- Findings: No blocking findings. The tests use existing report rendering and avoid introducing new comparison formats or schema changes.
- Gate: Code Review Gate.
- Decision: Pass locally.
- Residual risk: Protected CI and PR review remain pending.
- Next owner: Evaluation Engineer.

## Evaluation

- Owner role: Evaluation Engineer
- Artifact: This report.
- Result: #58 provides exact evidence for DSL runtime compatibility without expanding product scope.
- Gate: Evaluation Gate.
- Decision: Pass locally.
- Residual risk: Milestone #7 remains open for #59 through #61.
- Next owner: Release Engineer.

## Release

- Owner role: Release Engineer
- Artifact: Local focused validation.
- Evidence: Full local validation passed; PR creation and required `rust` CI remain pending.
- Gate: Release Gate.
- Decision: Pass locally; blocked on protected CI before merge.
- Residual risk: PR/CI may uncover host-level issues.
- Next owner: GitHub Maintainer Specialist.

## Community

- Owner role: Community Engineering Lead
- Artifact: Pending PR body.
- Evidence: PR must include `Fixes #58` after full validation.
- Gate: Community Gate.
- Decision: Blocked until PR creation.
- Residual risk: Issue #58 remains open until PR merge.
- Next owner: Project Coordinator.

## Retrospective

- Owner role: Project Coordinator
- Artifact: This report.
- Lesson: Exact parity tests are a useful gate before documenting a new config form as user-facing behavior.
- Gate: Retrospective Gate.
- Decision: Pass locally.
- Residual risk: Invalid DSL cases should be tested before final docs.
- Next owner: Project Orchestrator.

## Hand-Off Note

Role: Test Automation Engineer
Goal: Complete M7-004 / issue #58.
Files changed: DSL parity configs, criteria-engine tests, parity evidence docs, requirements, traceability, risk register, project state, and this report.
Checks run: `cargo test -p ferrisoxide-core --test criteria_engine`; `cargo fmt`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Implemented and locally validated; PR pending.
Known gaps: #59 invalid-config matrix and #60/#61 docs remain open.
Next recommended step: Run full validation, open PR for #58, wait for required CI, merge, then continue issue #59.
