# M8-001 Rule Schema Crate Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide-signal`

Branch: `feature/m8-001-rule-schema-crate`

Issue: #67, `M8-001 Create ferrisoxide-rule-schema crate`

Requirement: WRA-RQ-043

Owner Roles: Software Architect / Core Software Engineer

## Objective

Create a local `ferrisoxide-rule-schema` crate for the versioned portable FerrisOxide Rule Package model.

The crate must cover package metadata, target profile, channel definitions, units, sample-rate assumptions, filters, criteria, thresholds, and timing limits without depending on CLI, CSV, plotting, reports, controller I/O, hardware HALs, RTOS SDKs, export commands, checksums, binary packages, or a rule execution engine.

## Scope Boundaries

In scope:

- Workspace member `crates/ferrisoxide-rule-schema`.
- Data-only Rust schema types.
- Serde serialization/deserialization derives using approved workspace dependencies.
- Unit tests for construction and serialization round-trip behavior.
- Crate README and traceability updates.

Out of scope:

- Export command.
- Rule package validator.
- Shared rule execution engine.
- Binary package format.
- Manifest or checksum algorithm.
- no_std compatibility claim.
- RTOS SDK, HAL, controller I/O, DAQ, GUI, hardware qualification, or certification claim.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | User requested open issues be worked through the correct pipelines; issue #67 already exists under milestone #8. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing repository and milestone artifacts are already present. | No new project package was needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | Issue #67 selected first because #68-#74 depend on a stable schema boundary. | Later M8 issues still need their own pipeline artifacts. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed `decisions/ADR-004-portable-rule-package-architecture.md`, `docs/v0.6.0-portable-rule-package-milestone-proposal.md`, `requirements.md`, and `traceability-matrix.md`. | Future package format details remain in #71. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-043 updated in `requirements.md`. | Requirement remains partially open for #71 package format documentation. | Software Architect |
| Architecture | Architecture Gate | Pass | `docs/architecture.md` updated with the new schema crate and boundaries. | no_std boundary remains #72. | Abstraction Review Engineer |
| Abstraction Review | Granularity Gate | Pass | Schema owns typed package structures only; validator, export, checksum, engine, and parity behavior are separate issues. | Future issues must not duplicate schema definitions. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User previously approved continuing through the issue pipeline and PR creation. | None for this scoped local crate. | Project Coordinator |
| Dependency | Dependency Gate | Pass | `docs/dependency-review.md`; crate uses approved `serde` and `serde_json` dev-dependency only. | Future checksum, binary format, or signing dependencies need fresh review. | Security Engineer |
| Implementation | Implementation Gate | Pass | `Cargo.toml`, `Cargo.lock`, `crates/ferrisoxide-rule-schema/Cargo.toml`, `crates/ferrisoxide-rule-schema/src/lib.rs`, `crates/ferrisoxide-rule-schema/README.md`. | Validation and runtime semantics are not implemented yet. | Core Software Engineer |
| Testing | Testing Gate | Pass | `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`. | Protected GitHub CI remains pending until PR creation. | Verification and Validation Engineer |
| V&V | V&V Gate | Pass | Schema tests verify construction, explicit unit serialization, and round-trip deserialization. Dependency tree confirms no CLI, CSV, plotting, report, HAL, SDK, or controller dependency. | This is verification of schema shape, not validation of rule execution accuracy. | V&V Engineer |
| QA | QA Gate | Pass | Public API is limited to typed structs/enums and constructors; no behavior path changes to CLI/core analysis. | Future docs in #71 must define the human-authored file format. | QA Engineer |
| Security | Security Gate | Pass | No new third-party dependencies; no file I/O, network I/O, parsing engine, checksum, signing, unsafe code, HAL, or SDK added. | Future package integrity work may create security overclaims if not gated. | Security Engineer |
| Performance | Performance Gate | Not Applicable | Data-only schema crate adds no hot-path analysis or parsing behavior. | Large package parsing/export performance remains future work. | Performance Engineer |
| Documentation | Documentation Gate | Pass | Crate README, architecture, dependency review, requirements, traceability, risk register, project state, validation log, and this report. | Full package format examples remain #71. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review checked scope boundaries, dependency surface, and serialization shape. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass | Acceptance criteria are mapped to files and commands below. | #71, #68, #69, #70, #73, #72, and #74 remain open. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Local branch passes required checks; release requires PR, required `rust` CI, and protected merge. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | Issue #67 will close via PR body `Fixes #67`. | Milestone #8 remains open after this issue. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lessons recorded below. | Update if PR review requires changes. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| Crate is added to workspace. | Root `Cargo.toml` includes `crates/ferrisoxide-rule-schema`; crate has its own `Cargo.toml`. |
| Package metadata covered. | `PackageMetadata` with name, version, schema version, and description. |
| Channels covered. | `ChannelDefinition` with logical name, optional source name, unit, sample rate, and thresholds. |
| Units covered. | `EngineeringUnit` serializes canonical `V`, `s`, `count`, `sample`, and `Hz` symbols. |
| Sample-rate assumptions covered. | `SampleTimingAssumption` records timestamp unit, nominal sample rate, sample-rate tolerance, sample interval, and timestamp tolerance. |
| Filters covered. | `FilterDefinition` includes moving average, low pass, and ADC quantization definitions. |
| Criteria covered. | `CriterionDefinition`, `MeasurementDefinition`, and `RequirementDefinition` model measurement-backed criteria. |
| Thresholds covered. | `ThresholdDefinition`, `ThresholdRole`, and criterion measurement thresholds. |
| Timing limits covered. | `RequirementDefinition` values carry explicit units; pulse width, stable-state duration, transient event duration, rise time, and fall time measurements can require `s` values. |
| Target profile covered. | `TargetProfile` and `TargetProfileKind`. |
| No desktop/IO/runtime dependencies. | `cargo tree -p ferrisoxide-rule-schema` shows only `serde` plus `serde_json` as dev-dependency. |
| Schema docs and tests included. | Crate README and unit tests in `crates/ferrisoxide-rule-schema/src/lib.rs`. |

## Validation Commands

| Command | Result | Notes |
|---|---|---|
| `cargo tree -p ferrisoxide-rule-schema` | Passed | Runtime dependency is approved `serde`; dev-dependency is approved `serde_json`. |
| `cargo test -p ferrisoxide-rule-schema` | Passed | 2 unit tests passed plus doctests. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | 108 tests passed across workspace plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- The crate intentionally does not validate semantic correctness. Invalid package detection belongs to M8-003 / issue #68.
- The crate intentionally does not define package artifact layout. Human-authored TOML/JSON examples and package artifact roles belong to M8-002 / issue #71.
- The crate intentionally does not execute criteria. Shared execution belongs to M8-006 / issue #73.
- The crate intentionally does not claim no_std compatibility yet. That boundary belongs to M8-007 / issue #72.
- The stable `WRA-*` audit identifiers remain unchanged after the FerrisOxide rename.

## Retrospective

What worked:

- Starting with a data-only crate creates a concrete boundary for later validator, export, and engine work.
- Reusing approved Serde dependencies avoids a new dependency gate while keeping the schema serializable.

What to watch:

- Package format docs in #71 should reuse these types rather than inventing a parallel TOML/JSON vocabulary.
- no_std work in #72 may require feature-gating or a smaller embedded subset if Serde defaults remain too desktop-oriented.

## Hand-Off Note

Role: Software Architect / Core Software Engineer
Goal: Create the initial portable rule package schema crate for issue #67.
Files changed: `Cargo.toml`, `Cargo.lock`, `crates/ferrisoxide-rule-schema/`, `docs/architecture.md`, `docs/dependency-review.md`, `docs/platform-targets.md`, `docs/validation-log.md`, `docs/m8-001-rule-schema-crate-pipeline-report.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`.
Checks run: `cargo tree -p ferrisoxide-rule-schema`; `cargo test -p ferrisoxide-rule-schema`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR/CI/merge pending.
Known gaps: No package format docs, validator, export command, manifest/checksum, shared rule engine, no_std compatibility claim, or parity tests yet.
Next recommended step: Open a protected-branch PR with `Fixes #67`, wait for required `rust` CI, merge, then continue with M8-002 / issue #71 or M8-003 / issue #68.
