# Contributing

Thank you for considering a contribution.

## Development Rules

- Keep changes small and reviewable.
- Use Cargo workspace commands from the project root.
- Do not add third-party dependencies without an issue, rationale, license review, and explicit approval.
- Include tests for new behavior.
- Document units, sample rate assumptions, tolerances, and analysis criteria.
- Do not overwrite raw example data.

## Validation

Run before opening a PR:

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets
```

## Pull Requests

PRs should include:

- Summary of the change.
- Requirement or issue link.
- Validation commands and results.
- Risk or limitation notes.
- Documentation updates when behavior changes.

## AI Assistance

AI-assisted contributions are allowed when the contributor reviews, understands, and validates the result. The contributor remains responsible for the submitted work.
