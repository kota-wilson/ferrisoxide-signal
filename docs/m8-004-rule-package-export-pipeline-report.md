# M8-004 Rule Package Export Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide-signal`

Branch: `feature/m8-004-rule-package-export`

Issue: #69, `M8-004 Add rule package export command`

Requirement: WRA-RQ-046

Owner Roles: Core Software Engineer / Verification and Validation Engineer

## Objective

Add a desktop CLI command that exports FerrisOxide Rule Package artifacts from validated analysis config and analysis evidence.

## Scope Boundaries

In scope:

- `export-rule-package` CLI subcommand.
- Required inputs: CSV path, TOML config path, output directory, package name, and package version.
- Optional target profile and target identifier.
- Config loading, waveform analysis, package construction, package validation, and artifact writing.
- Reviewable desktop artifacts: `rules.toml`, `rules.json`, and `validation-report.json`.
- Exact expected-artifact tests and overwrite-refusal tests.
- Documentation of command usage and scope limits.

Out of scope:

- GUI, DAQ, controller SDK, HAL, RTOS production integration, or hardware I/O.
- Manifest generation, checksum algorithms, signing, or binary package serialization.
- Shared rule execution engine.
- no_std rule-engine boundary.
- Desktop-vs-embedded parity tests.
- Hardware qualification, flight certification, production-readiness, or safety claims.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Issue #69 exists in milestone #8 and follows #67 schema, #71 format, and #68 validator work. | None for issue selection. | Project Orchestrator |
| Project Creation | Project Creation Gate | Not Applicable | Existing repository and milestone package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | #69 selected after schema, format, and validator work so export can reuse those artifacts. | Remaining M8 issues still need manifest/checksum, engine, no_std, and parity work. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed issue #69, `ferrisoxide-cli`, `ferrisoxide-rule-schema`, and `docs/rule-package-format.md`. | Future deployment package behavior may revise artifact set. | Software Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-046 updated in `requirements.md` and `traceability-matrix.md`. | Requirement remains local until PR/CI/merge. | Software Architect |
| Architecture | Architecture Gate | Pass | Export remains isolated to `ferrisoxide-cli` and consumes `ferrisoxide-rule-schema`; existing analyze and plot commands are unchanged. | Shared execution engine remains #73. | Abstraction Review Engineer |
| Abstraction Review | Granularity Gate | Pass | Command inputs, artifact names, helper functions, tests, and exclusions are concrete and issue-sized. | Future package-export expansion needs separate issues. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User asked to continue open issues through the pipeline and previously approved PR creation. | None for this scoped export slice. | Project Coordinator |
| Dependency | Dependency Gate | Pass | Adds local `ferrisoxide-rule-schema` dependency and approved workspace `serde_json`; no new third-party crates. | Future checksum/binary/signing dependencies require fresh review. | Security Engineer |
| Implementation | Implementation Gate | Pass | `crates/ferrisoxide-cli/src/main.rs` adds `export-rule-package`, package construction helpers, validation before write, and overwrite-safe artifact writes. | Exported artifacts are not deployment manifests yet. | Core Software Engineer |
| Testing | Testing Gate | Pass | `cargo tree -p ferrisoxide-cli`; `cargo test -p ferrisoxide-cli`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`. | Protected GitHub CI pending until PR creation. | Verification and Validation Engineer |
| V&V | V&V Gate | Pass | Tests compare `rules.toml`, `rules.json`, and `validation-report.json` exactly and validate the exported package for the controller runtime target. | This validates export shape, not embedded runtime execution. | V&V Engineer |
| QA | QA Gate | Pass | Command errors on missing required flags, invalid package validation, and pre-existing output artifacts. | Additional CLI UX refinements can follow future user feedback. | QA Engineer |
| Security | Security Gate | Pass | Uses create-new file writes to avoid overwriting artifacts; adds no network, SDK, HAL, unsafe code, signing, binary serialization, or runtime loader. | Manifest/checksum integrity work remains #70. | Security Engineer |
| Performance | Performance Gate | Not Applicable | Export runs one desktop analysis and writes small package/report artifacts; no new waveform hot path. | Large batch export remains out of scope. | Performance Engineer |
| Documentation | Documentation Gate | Pass | README, rule-package format docs, dependency review, risk register, traceability, validation log, and this report updated. | Manifest/checksum docs will need revision in #70. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review checked command scope, artifact content, overwrite behavior, and helper decomposition. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass | Issue #69 acceptance criteria are mapped below. | Remaining M8 work still required for portable runtime claims. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Local branch passes required checks; release requires PR, required `rust` CI, and protected merge. | GitHub CI may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | Issue #69 will close via PR body `Fixes #69`. | Milestone #8 remains open after this issue. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lessons recorded below. | Update if PR review requires changes. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| CLI exports package artifacts from validated config and analysis evidence. | `export-rule-package` loads TOML config, analyzes the CSV, builds a `RulePackage`, validates it for the selected target, then writes `rules.toml`, `rules.json`, and `validation-report.json`. |
| Existing analyze and plot behavior remains unchanged. | The new command is a separate `run()` subcommand branch; existing tests for analyze and plot still pass. |
| Export tests compare expected artifact files. | `exports_rule_package_artifacts_from_config_and_evidence` compares all three exported artifacts exactly against `tests/expected/rule-package-basic/`. |
| Docs show command usage and scope limits. | README and `docs/rule-package-format.md` document the command and exclude manifest/checksum, binary, SDK, RTOS, hardware qualification, and certification behavior. |
| Workspace fmt, tests, clippy, and diff check pass. | Validation commands below. |

## Validation Commands

| Command | Result | Notes |
|---|---|---|
| `cargo tree -p ferrisoxide-cli` | Passed | Shows local `ferrisoxide-rule-schema` and approved existing dependencies only. |
| `cargo test -p ferrisoxide-cli` | Passed | 13 CLI tests passed. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | 120 tests passed across workspace plus doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings after package-builder input refactor. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- Export refuses to overwrite existing artifact files by using create-new file writes.
- Export validates the package before writing artifacts.
- The command writes software validation evidence only and does not claim hardware qualification or certification.
- M8-005 owns manifest/checksum behavior; M8-006 owns shared rule execution; M8-007 owns no_std compatibility boundaries; M8-008 owns parity tests.

## Retrospective

What worked:

- Reusing `ferrisoxide-rule-schema` kept CLI export shape aligned with the documented package format.
- Exact expected-artifact comparisons make package output changes visible during review.

What to watch:

- Future manifest/checksum work must avoid implying signing, certification, or production deployment readiness before those gates exist.
- Exported rule semantics still need shared execution and parity tests before desktop-vs-embedded runtime claims are safe.

## Hand-Off Note

Role: Core Software Engineer / Verification and Validation Engineer
Goal: Add the M8-004 desktop rule package export command for issue #69.
Files changed: `README.md`, `Cargo.lock`, `crates/ferrisoxide-cli/Cargo.toml`, `crates/ferrisoxide-cli/src/main.rs`, `docs/dependency-review.md`, `docs/rule-package-format.md`, `docs/validation-log.md`, `docs/m8-004-rule-package-export-pipeline-report.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`, `tests/expected/rule-package-basic/rules.toml`, `tests/expected/rule-package-basic/rules.json`, `tests/expected/rule-package-basic/validation-report.json`.
Checks run: `cargo tree -p ferrisoxide-cli`; `cargo test -p ferrisoxide-cli`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR/CI/merge pending.
Known gaps: Manifest/checksum generation, shared rule execution, no_std compatibility, and desktop-vs-embedded parity tests remain separate M8 issues.
Next recommended step: Open a protected-branch PR with `Fixes #69`, wait for required `rust` CI, merge, then implement M8-005 / issue #70.
