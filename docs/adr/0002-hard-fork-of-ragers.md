# Hard fork of Ragers

This repository is a hard fork of [Sutaai/Ragers](https://codeberg.org/Sutaai/Ragers), created to develop the codebase independently while applying AI-assisted engineering practices — agent skills, a domain glossary, ADRs, and a real automated test suite.

## Considered options

- **Contribute upstream**: rejected. Upstream is the author's first Rust project and evolves as a learning exercise; this fork is a sandbox for agent-driven engineering, not a contribution channel.
- **Fresh rewrite**: rejected. The value is in enhancing the existing codebase, not replacing it.
- **Hard fork**: chosen. Preserves the full 56-commit history and provenance while allowing the codebase to diverge freely.

## Consequences

- Upstream `Sutaai/Ragers` is tracked as the `upstream` remote; `origin` is this GitHub repository.
- Upstream metadata still references Codeberg and Forgejo (docs site URL, CI workflow, package repository URL). Hardening the fork to run on GitHub is tracked separately from this decision.
