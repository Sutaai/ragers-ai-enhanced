# AGENTS.md

Ragers is a Rust CLI for encrypting files in a git repository using the `age` format. This repository is a hard fork of [Sutaai/Ragers](https://codeberg.org/Sutaai/Ragers); see `docs/adr/0002-hard-fork-of-ragers.md`.

## Build, test, and lint

- Toolchain via `mise` (`mise install` provisions Rust, `age`, `prek`, and docs tooling).
- Build: `cargo build`
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Test: `cargo test`
- Pre-commit: `prek` (see `prek.toml`)

## Domain language

Read `GLOSSARY.md` before naming anything in code, docs, or issues, and use its vocabulary.

## Agent skills

### Issue tracker

Issues live in GitHub Issues on this repository. See `docs/agents/issue-tracker.md`.

### Triage labels

The five canonical triage roles map to the default label strings. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: one `GLOSSARY.md` plus `docs/adr/` at the repo root. See `docs/agents/domain.md`.
