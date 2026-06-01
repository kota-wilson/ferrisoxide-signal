# M13 Runtime Profile Validation Issue Planning Report

Date: 2026-06-01

Status: Complete. GitHub milestone #13 and issues #158 through #163 are closed through PR #164.

## Scope

This report converts the M13 runtime-profile validation proposal into issue-ready work items.

It intentionally stops before new dependencies, new signal algorithms, live DAQ, HAL/RTOS SDKs, target hardware execution, binary package signing, hardware validation, certification evidence, or incompatible schema changes.

## Planned Milestone

| Milestone | Version | Proposal | Status |
|---|---|---|---|
| M13 | v0.11.0 | `docs/v0.11.0-transform-runtime-profile-validation-milestone-proposal.md` | GitHub milestone #13 closed with issues #158 through #163 |

## Planned GitHub Issues

| Placeholder | Title | Requirement Links | Acceptance Summary |
|---|---|---|---|
| #158 / M13-001 | Add transform runtime-profile validator API and structured error model | WRA-RQ-087 | Core validator returns stable structured errors for unsupported transform/profile requests. |
| #159 / M13-002 | Validate sample-timing evidence for runtime-profile exposure | WRA-RQ-089 | Sample-rate-required metadata rejects missing or invalid timing evidence before exposure. |
| #160 / M13-003 | Add runtime-profile tests for waveform transform metadata | WRA-RQ-088 | Existing waveform transforms pass desktop and reject embedded/Pico/future-gated exposure. |
| #161 / M13-004 | Add runtime-profile tests for event and validation transform metadata | WRA-RQ-090 | M12 event/validation metadata remains desktop-only until bounded runtime work is separately approved. |
| #162 / M13-005 | Document rule-package, deployment, and legacy export guardrails | WRA-RQ-091 | Docs separate current legacy rule-package filter export from transform runtime support claims. |
| #163 / M13-006 | Update traceability, risk, and pipeline evidence for M13 closure | WRA-RQ-092 | Requirements, traceability, risk, state, validation, and closure artifacts are updated. |

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Intake Gate | Pass | M10/M12 known gaps identify runtime-profile validator code as future work; user approved continuing after M12 closure. | Project Coordinator |
| Requirements Gate | Pass for proposal | WRA-RQ-087 through WRA-RQ-092 are added as M13 requirements. | Software Architect |
| Architecture Gate | Pass locally | Uses existing M10 transform metadata model and runtime-profile vocabulary. | Abstraction Review Engineer |
| Scope Gate | Pass locally | Excludes dependencies, algorithms, live DAQ, HAL/RTOS, target hardware, GUI, signing, and certification claims. | Project Orchestrator |
| Human Approval Gate | Pass for M13 issue creation | User approved continuing after M12 closure on 2026-06-01. | Project Coordinator |
| Issue Planning Gate | Pass | GitHub milestone #13 and issues #158 through #163 created. | GitHub Maintainer Specialist |
| Implementation Gate | Pass | Runtime-profile validator, tests, docs, and traceability merged in PR #164. | Core Software Engineer |
| Release Gate | Pass | PR #164 merged after required `rust` CI passed; squash commit `ae0366dcd20a81a71262f38d2409dc2b85774051`. | GitHub Maintainer Specialist |
| Community Gate | Pass | Issues #158 through #163 closed and milestone #13 closed with 6 closed items and 0 open items. | Project Coordinator |

## Stop Conditions

Stop and ask for approval before:

- adding third-party dependencies
- changing public report/config schema incompatibly
- changing legacy rule-package schema behavior incompatibly
- adding live DAQ, vendor SDK, HAL, RTOS SDK, unsafe FFI, target hardware execution, or global setup
- claiming hardware validation, real-time readiness, certification, signing, authentication, or production controller readiness

## Hand-Off Note

Role: Project Orchestrator / GitHub Maintainer Specialist
Goal: Convert M13 proposal into issue-ready work.
Files changed: This report plus proposal, requirements, traceability, risk, roadmap, orchestration, and project state.
Checks run: Documentation and code-path inspection.
Status: Complete through PR #164, closed issues #158 through #163, and closed milestone #13.
Known gaps: No GitHub release tag was published for M13; runtime loaders, hardware evidence, certification evidence, and M14+ work remain separately gated.
Next recommended step: Hold before M14 or new scope until explicit approval.
