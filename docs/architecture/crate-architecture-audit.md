# FerrisOxide Crate Architecture Audit

Date: 2026-06-06

## Scope

This audit backfills the WRA-RQ-140 crate-local architecture documentation rule for every current workspace crate. It reviewed `Cargo.toml`, `cargo metadata --no-deps`, each crate `src` entrypoint, public structs/enums/functions, crate README files where present, and the current top-level system overview in `docs/architecture/ferrisoxide-overview.md`.

The diagrams are crate-focused. They do not duplicate the full FerrisOxide system flowchart.

## Workspace Crates Reviewed

| Crate | Current role | Runtime scope | Internal upstream crates | Architecture doc |
|---|---|---|---|---|
| `ferrisoxide-core` | Desktop waveform model, config, parsing, transform, analysis, report, and catalog core. | `std` desktop library. | `ferrisoxide-measurements`, `ferrisoxide-rule-engine` | `crates/ferrisoxide-core/architecture.md` |
| `ferrisoxide-workflow` | Shared command/workflow orchestration API used by CLI and GUI. | `std` desktop workflow library. | `ferrisoxide-control-schema`, `ferrisoxide-core`, `ferrisoxide-daq`, `ferrisoxide-plot`, `ferrisoxide-rule-schema`, `ferrisoxide-simulator`, `ferrisoxide-verification-schema` | `crates/ferrisoxide-workflow/architecture.md` |
| `ferrisoxide-cli` | CLI binaries and benchmark helper. | `std` desktop binaries. | `ferrisoxide-core`, `ferrisoxide-workflow` | `crates/ferrisoxide-cli/architecture.md` |
| `ferrisoxide-gui` | Optional native egui workflow shell and GUI session state. | `std`; native UI behind `native` feature. | `ferrisoxide-workflow` | `crates/ferrisoxide-gui/architecture.md` |
| `ferrisoxide-measurements` | Reusable measurement primitives over time/sample slices. | `no_std`. | None | `crates/ferrisoxide-measurements/architecture.md` |
| `ferrisoxide-rule-engine` | Shared criteria semantics and borrowed runtime-compatible rule path. | `no_std` with `alloc`. | `ferrisoxide-measurements` | `crates/ferrisoxide-rule-engine/architecture.md` |
| `ferrisoxide-rule-schema` | Versioned portable rule package and manifest schema. | `std` schema crate. | None | `crates/ferrisoxide-rule-schema/architecture.md` |
| `ferrisoxide-control-schema` | Versioned production control config schema. | `std` schema crate. | None | `crates/ferrisoxide-control-schema/architecture.md` |
| `ferrisoxide-verification-schema` | Versioned test verification config schema. | `std` schema crate. | None | `crates/ferrisoxide-verification-schema/architecture.md` |
| `ferrisoxide-daq` | Fixture/test-double DAQ abstraction. | `std`; no vendor SDKs. | None | `crates/ferrisoxide-daq/architecture.md` |
| `ferrisoxide-controller-io` | Host-checkable controller I/O abstraction. | `std`; no HAL/RTOS binding. | None | `crates/ferrisoxide-controller-io/architecture.md` |
| `ferrisoxide-simulator` | Deterministic virtual controller simulation over control configs. | `std` simulation library. | `ferrisoxide-control-schema` | `crates/ferrisoxide-simulator/architecture.md` |
| `ferrisoxide-deployment` | Deployment manifest and qualification evidence report schemas. | `std` schema/validation crate. | None | `crates/ferrisoxide-deployment/architecture.md` |
| `ferrisoxide-plot` | Desktop SVG waveform plotting and evidence overlays. | `std` desktop rendering library. | `ferrisoxide-core` | `crates/ferrisoxide-plot/architecture.md` |
| `ferrisoxide-signal` | Minimal embedded-compatible signal primitives. | `no_std`. | None | `crates/ferrisoxide-signal/architecture.md` |
| `ferrisoxide-embedded` | Embedded adapter traits over signal primitives. | `no_std`; host tests only. | `ferrisoxide-signal` | `crates/ferrisoxide-embedded/architecture.md` |

## Audit Findings

- All 16 workspace crates qualify as major crates under `docs/architecture.md`.
- Current crate boundaries are documentation-clean: schema crates do not run workflows, no-std crates do not perform file I/O, and workflow/GUI/CLI crates own user-facing orchestration.
- Current live DAQ, hardware HAL, RTOS runtime loader, package signing, installer, release publication, and certification evidence remain outside implemented scope.
- The new crate-local files document current source-level responsibilities, public APIs, validation commands, and important error paths.

## Verification Boundary

This audit validates documentation accuracy against current repository source. It does not add runtime behavior, dependencies, hardware support, certification claims, release artifacts, or installer/package support.
