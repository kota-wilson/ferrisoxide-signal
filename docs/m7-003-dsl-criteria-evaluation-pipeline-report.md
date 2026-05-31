# M7-003 DSL Criteria Evaluation Pipeline Report

Date: 2026-05-31

Branch: `feature/m7-003-dsl-criteria-evaluation`

Milestone: #7, `v0.5.0: Measurement-Backed Criteria DSL`

Issue: #57, `M7-003 Evaluate DSL criteria through existing measurement evidence`

Pull request: Pending.

Status: Implemented and locally validated; PR/CI pending.

## Scope

This slice maps approved DSL criteria into runtime criteria evaluation without changing the legacy criteria schema or report schema.

In scope:

- Convert DSL `[criteria.measurement]` and `[criteria.requirement]` entries into runtime measurement-backed criteria.
- Evaluate `minimum_sample`, `maximum_sample`, `state_transition_count`, `pulse_width`, `stable_state_duration`, `transient_event_duration`, `rise_time`, and `fall_time`.
- Apply the approved operators: `less_than`, `less_than_or_equal`, `greater_than`, `greater_than_or_equal`, and `equal_to`.
- Reuse existing measurement primitives and M6 report evidence records.
- Preserve `measurement_id`, measured value, required value, sample index, timestamp, channel, tolerance, and reason fields.
- Keep legacy configs and existing golden JSON reports unchanged.

Out of scope:

- DSL/legacy parity golden JSON fixture expansion for issue #58.
- Full invalid DSL config matrix for issue #59.
- User-facing DSL migration docs and schema reference for issues #60 and #61.
- Unit conversion or shorthand unit parsing.
- New measurements such as duty cycle, RMS, frequency, overshoot, undershoot, or noise floor.
- GUI, DAQ, plugin runtime, batch analysis, RTOS expansion, hardware qualification, or certification claims.

## Research

- Owner role: Software Architect / Core Software Engineer
- Artifact: Issue #57, `docs/criteria-dsl.md`, M7-001/M7-002 pipeline reports, and existing `ferrisoxide-core` analysis code.
- Evidence: Existing measurement primitives already back legacy criteria evidence, while M7-001/M7-002 added DSL config shape and validation.
- Gate: Intake Gate.
- Decision: Pass.
- Residual risk: The first runtime DSL slice can be mistaken for a complete public DSL unless parity and docs issues remain visible.
- Next owner: Software Architect.

## Requirements

- Owner role: Software Architect / V&V Engineer
- Artifact: WRA-RQ-039 in `requirements.md`; WRA-RQ-039 row in `traceability-matrix.md`.
- Requirement: DSL criteria shall evaluate through existing measurement evidence paths while preserving legacy compatibility.
- Gate: Requirements Traceability Gate.
- Decision: Pass.
- Residual risk: WRA-RQ-040 through WRA-RQ-042 remain planned for parity golden tests, invalid-config coverage, and user-facing docs.
- Next owner: Software Architect.

## Architecture

- Owner role: Software Architect
- Artifact: `crates/ferrisoxide-core/src/criteria.rs`, `crates/ferrisoxide-core/src/config.rs`, and `crates/ferrisoxide-core/src/analysis.rs`.
- Design: Add a runtime `CriterionCheck::Measurement` path so DSL criteria can carry a measurement spec plus requirement operator while legacy criteria keep their existing constructors and evaluator branches.
- Gate: Architecture Gate.
- Decision: Pass.
- Residual risk: Future rule-package work should avoid duplicating these DSL semantics in a second engine.
- Next owner: Abstraction Review Engineer.

## Abstraction Review

- Owner role: Abstraction Review Engineer
- Artifact: This report and focused code diff.
- Review: The implementation is limited to runtime evaluation for the already-approved DSL model and does not absorb #58 parity golden fixtures, #59 invalid matrices, #60/#61 docs, or v0.6.0 rule package work.
- Gate: Granularity Gate.
- Decision: Pass.
- Residual risk: Pulse-width selection defaults should be documented in #61.
- Next owner: Core Software Engineer.

## Implementation

- Owner role: Core Software Engineer
- Artifact: `crates/ferrisoxide-core/src/criteria.rs`, `crates/ferrisoxide-core/src/config.rs`, and `crates/ferrisoxide-core/src/analysis.rs`.
- Behavior:
  - DSL configs now convert to `CriterionCheck::Measurement`.
  - Runtime measurement specs represent voltage extrema, transition counts, state-run duration measurements, transient-event duration, and edge timing.
  - Requirement operators are evaluated with the configured voltage or time tolerance; count comparisons use zero tolerance.
  - Pulse-width DSL criteria default to `shortest` for minimum-style operators and `longest` for maximum-style operators.
  - Legacy criteria branches remain unchanged.
- Gate: Implementation Gate.
- Decision: Pass locally.
- Residual risk: Exact DSL/legacy JSON parity belongs to #58.
- Next owner: Test Automation Engineer.

## Testing

- Owner role: Test Automation Engineer
- Artifact: Unit and integration tests.
- Evidence:
  - Config conversion test confirms DSL criteria convert to runtime measurement criteria.
  - Analysis unit test confirms strict and inclusive operators behave differently.
  - Integration test evaluates all supported DSL measurement types through `evaluate_criteria_with_measurements`.
  - Existing golden JSON tests continue to pass for legacy configs.
- Gate: Testing Gate.
- Decision: Pass locally.
- Validation:
  - `cargo test -p ferrisoxide-core`: Pass.
  - `cargo fmt`: Pass.
  - `cargo fmt --check`: Pass.
  - `cargo test --workspace`: Pass.
  - `cargo clippy --workspace --all-targets -- -D warnings`: Pass.
  - `git diff --check`: Pass.
- Residual risk: Protected CI remains pending until PR creation.
- Next owner: Verification and Validation Engineer.

## Verification And Validation

- Owner role: Verification and Validation Engineer
- Artifact: WRA-RQ-039 traceability and test evidence.
- Verification: #57 acceptance criteria map to config conversion, runtime evaluator behavior, measurement records, result links, and unchanged legacy report tests.
- Validation: This is software validation against synthetic fixtures only; it is not hardware validation, DAQ validation, RTOS validation, production readiness, or certification evidence.
- Gate: V&V Gate.
- Decision: Pass locally.
- Residual risk: External engineering confidence still depends on #58 parity fixtures and #60/#61 documentation.
- Next owner: QA Engineer.

## QA

- Owner role: QA Engineer
- Artifact: Scope and behavior review.
- Evidence: The implementation is additive, keeps the existing report schema, and continues to route all evidence through `ferrisoxide-core` analysis records.
- Gate: QA Gate.
- Decision: Pass locally.
- Residual risk: User-facing examples still need to show the exact DSL form and defaults.
- Next owner: Security Engineer.

## Security

- Owner role: Security Engineer
- Artifact: Dependency and parser-surface review.
- Evidence: No new dependencies, unsafe Rust, expression language, unit parser, plugin runtime, network surface, DAQ SDK, HAL, RTOS SDK, FFI, or shell surface were added.
- Gate: Security Gate.
- Decision: Pass locally.
- Residual risk: Future unit shorthand or plugin work requires separate parser and supply-chain review.
- Next owner: Performance Engineer.

## Performance

- Owner role: Performance Engineer
- Artifact: Code inspection.
- Evidence: DSL criteria use the same per-criterion linear measurement scans as existing criteria and add only small enum matching and operator comparison overhead.
- Gate: Performance Gate.
- Decision: Pass locally.
- Residual risk: Batch analysis and large-file DSL benchmark claims remain out of scope.
- Next owner: Documentation Engineer.

## Documentation

- Owner role: Documentation Engineer
- Artifact: This report, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, and `project-state.md`.
- Evidence: Durable project files record the local implementation, evidence path, and remaining M7 documentation/parity work.
- Gate: Documentation Gate.
- Decision: Pass locally.
- Residual risk: Public DSL examples and schema reference remain in #60 and #61.
- Next owner: Code Reviewer.

## Code Review

- Owner role: Code Review Engineer
- Artifact: Local code review.
- Findings: No blocking findings. The DSL runtime path reuses existing measurement functions and leaves legacy branches intact.
- Gate: Code Review Gate.
- Decision: Pass locally.
- Residual risk: PR review and protected CI remain pending.
- Next owner: Evaluation Engineer.

## Evaluation

- Owner role: Evaluation Engineer
- Artifact: This report.
- Result: #57 is scoped, traceable, and implemented without absorbing remaining M7 issues or v0.6.0 rule package work.
- Gate: Evaluation Gate.
- Decision: Pass locally.
- Residual risk: Milestone #7 remains open until #58 through #61 close.
- Next owner: Release Engineer.

## Release

- Owner role: Release Engineer
- Artifact: Local validation evidence.
- Evidence: Full local validation passed; PR creation and required `rust` CI remain pending.
- Gate: Release Gate.
- Decision: Pass locally; blocked on protected CI before merge.
- Residual risk: PR/CI may uncover host-level issues.
- Next owner: GitHub Maintainer Specialist.

## Community

- Owner role: Community Engineering Lead
- Artifact: Pending PR body.
- Evidence: PR must include `Fixes #57` after full validation.
- Gate: Community Gate.
- Decision: Blocked until PR creation.
- Residual risk: Issue #57 remains open until PR merge.
- Next owner: Project Coordinator.

## Retrospective

- Owner role: Project Coordinator
- Artifact: This report.
- Lesson: Keeping DSL as a runtime measurement criterion avoids changing legacy criteria constructors while preserving future room for rule-package semantics.
- Gate: Retrospective Gate.
- Decision: Pass locally.
- Residual risk: #58 should lock parity with exact JSON before expanding docs.
- Next owner: Project Orchestrator.

## Hand-Off Note

Role: Core Software Engineer
Goal: Complete M7-003 / issue #57.
Files changed: `crates/ferrisoxide-core/src/criteria.rs`, `crates/ferrisoxide-core/src/config.rs`, `crates/ferrisoxide-core/src/analysis.rs`, `crates/ferrisoxide-core/tests/criteria_engine.rs`, requirements, traceability, risk register, project state, validation log, and this report.
Checks run: `cargo test -p ferrisoxide-core`; `cargo fmt`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Implemented and locally validated; PR pending.
Known gaps: #58 parity golden tests, #59 invalid-config matrix, and #60/#61 user-facing DSL docs remain open.
Next recommended step: Run full validation, open PR for #57, wait for required CI, merge, then continue issue #58.
