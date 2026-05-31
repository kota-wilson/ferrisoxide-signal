# Security Policy

## Supported Versions

The project is pre-release. Security fixes apply to the current `main` branch until versioned releases begin.

## Reporting A Vulnerability

Do not open public issues for vulnerabilities. Contact the maintainers through a private channel once one is published for the repository.

## Security Scope

Current security-relevant surfaces:

- CSV parsing and malformed input handling.
- File path handling in the CLI.
- Future report exporters.
- Future plugin or binding layers.

The project must not execute input files, run shell commands from configs, or load untrusted dynamic code.
