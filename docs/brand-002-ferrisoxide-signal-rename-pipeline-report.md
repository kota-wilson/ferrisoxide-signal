# BRAND-002 FerrisOxide Signal Rename Pipeline Report

Date: 2026-05-31

Issue: #98, `BRAND-002 Adopt FerrisOxide Signal in-repository identity`

Status: Implemented and merged through PR #99. Repository host renamed to `kota-wilson/ferrisoxide-signal`.

## Summary

BRAND-002 moves FerrisOxide from proposal-stage brand architecture to the adopted in-repository identity for the current signal-analysis product. The rename covers source package names, the CLI binary, documentation, examples, scripts, and future crate naming references. It intentionally preserves historical `WRA-*` audit IDs and does not claim external namespace, domain, trademark, crates.io, or logo readiness.

## Stage Artifacts

| Stage | Gate | Artifact | Decision |
|---|---|---|---|
| Intake | Maintainer approval | User requested a thorough rename after BRAND-001. | Pass |
| Requirements | WRA-RQ-064 | `requirements.md` | Pass |
| Architecture | Identity adoption boundary | `docs/brand-architecture.md` | Pass |
| Decision | ADR-006 | `decisions/ADR-006-ferrisoxide-signal-identity-adoption.md` | Pass |
| Risk | Rename and namespace risk | `risk-register.md` WRA-RISK-027 | Pass |
| Traceability | Requirement mapping | `traceability-matrix.md` | Pass |
| Implementation | Source/docs rename | `Cargo.toml`, `Cargo.lock`, `crates/ferrisoxide-*`, README, docs, scripts, fixtures | Pass |
| Testing | Local validation | `docs/validation-log.md` BRAND-002 section | Pass |
| Release | PR, CI, and repository-host rename | PR #99 merged after required `rust` CI passed; `gh repo view kota-wilson/ferrisoxide-signal` verified the new URL; issue #98 closed. | Pass |

## Rename Boundary

In scope:

- Workspace package names.
- Workspace member paths.
- Rust crate imports.
- CLI binary name.
- Benchmark helper binary name.
- README, examples, docs, scripts, reports, and future crate references.
- Repository target URL references.

Out of scope:

- GitHub organization creation.
- Domain registration.
- crates.io package publishing or reservation.
- Trademark or legal clearance.
- Logo, mascot, or visual identity.
- Aerospace certification, hardware qualification, or production-controller claims.

## Stable Audit IDs

The `WRA-RQ-*`, `WRA-RISK-*`, and historical issue/PR IDs remain unchanged. They are traceability keys, not product branding.

## Verification Commands

Completed before PR:

```sh
cargo metadata --format-version 1 --no-deps
cargo clean
cargo fmt --check
cargo test --workspace
cargo test --manifest-path embedded/arm64/qemu/Cargo.toml
cargo clippy --workspace --all-targets -- -D warnings
cargo run --quiet --bin ferrisoxide-signal -- analyze --input examples/basic-waveform.csv --config examples/basic-config.toml --format text
cargo run --quiet --bin ferrisoxide-signal -- plot --input tests/fixtures/dropout_event.csv --config tests/configs/transient-event-dropout-fail.toml --output /private/tmp/ferrisoxide-signal-dropout-evidence.svg --title "Dropout Evidence"
rg -n "Evidence status|FAIL supply_dropout|threshold 2.500000" /private/tmp/ferrisoxide-signal-dropout-evidence.svg
sh scripts/benchmark-large-csv.sh 1000 1
git diff --check
```

Identifier scan: remaining findings are intentional historical references in ADR-005/BRAND-001 and the ADR-006 no-alias note. Stable `WRA-*` traceability IDs are intentionally preserved.

Release verification:

```sh
gh pr checks 99 --watch
gh pr merge 99 --rebase --delete-branch
gh repo rename ferrisoxide-signal -R kota-wilson/waveform-reconstructor-analyzer --yes
git remote set-url origin https://github.com/kota-wilson/ferrisoxide-signal.git
gh repo view kota-wilson/ferrisoxide-signal --json nameWithOwner,url
gh issue view 98 --json state,url
```

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist / Core Software Engineer
Goal: Adopt FerrisOxide Signal as the in-repository product identity.
Files changed: `Cargo.toml`, `Cargo.lock`, `crates/ferrisoxide-*`, README, docs, scripts, examples, validation reports, ADR-006, and project state files.
Checks run: `cargo metadata --format-version 1 --no-deps`; `cargo clean`; `cargo fmt --check`; `cargo test --workspace`; `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml`; `cargo clippy --workspace --all-targets -- -D warnings`; CLI analyze smoke; CLI plot smoke; SVG overlay scan; benchmark smoke; `git diff --check`; identifier scan.
Status: Complete for in-repository identity and repository-host rename; PR #99 merged and issue #98 closed.
Known gaps: External namespace, trademark, crates.io, domain, logo, and legal-suitability checks remain future gates.
Next recommended step: Return to M7-003 / issue #57 unless a new brand namespace/legal gate is explicitly opened.
