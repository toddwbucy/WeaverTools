# Repository Guidelines

## Project Structure & Module Organization

This Rust 2024 workspace contains twelve packages under `crates/weaver-*`. Each crate keeps implementation in `src/` and integration tests in `tests/`, with fixtures where needed. `weaver-traits` and `weaver-types` provide shared foundations; the harness connects components through contracted seams. Web assets live in `crates/weaver-web/assets/`, templates under its `src/`, and database migrations in `migrations/`.

`docs/crates/` holds crate PRDs, Specs, and contracts. `process/` holds working rules and validation scripts; `deploy/` contains deployment scripts. Read `process/WeaverTools-Working-Process.md` before changing behavior: implementation must follow ratified Specs.

## Build, Test, and Development Commands

Use the toolchain pinned in `rust-toolchain.toml` (`nightly-2026-02-13`). Run from the repository root:

- `cargo build --workspace --locked`: build all packages.
- `cargo test --workspace --locked`: run workspace tests and doctests.
- `cargo test -p weaver-harness --locked`: test one package.
- `cargo clippy -p <crate> --all-targets --locked -- -D warnings`: lint each changed crate.
- `cargo fmt --all -- --check`: check formatting.
- `python3 process/gates/census.py`: check document/code conformance against the baseline; introduce no new defects.

Keep `--locked` on Cargo commands that resolve dependencies. The SPU defaults to GGUF and builds llama.cpp; CUDA is optional. For local web startup, use `cargo run -p weaver-web --bin weaver-web --locked -- --config <config.toml>`. Follow `crates/weaver-web/README.md` for PostgreSQL and connector setup.

## Coding Style & Naming Conventions

Use rustfmt, four-space Rust indentation, `snake_case` functions/modules, and `PascalCase` types. Preserve source `conforms:` headers citing Spec assertions. Reuse contract-defined types across seams. Write documentation in ASCII with absolute dates and canonical terminology (`trace`, `reflection`, `substrate-state`).

## Testing Guidelines

Use Rust tests and doctests, with descriptive `snake_case` test names and integration files such as `tests/identity.rs`. Exercise refusal paths alongside success paths. Behavioral invariant tests must fail when the property is deliberately removed; use compile-time checks for type properties. Coverage is evidence, not a percentage target.

## Commit & Pull Request Guidelines

Follow history's descriptive prefixes: `code:`, `docs:`, or `process:`. Open PRs as drafts. Describe behavior changes, relevant Spec clauses/issues, and validation results. Complete required gates and review before leaving draft; answer every finding and repeat review after substantive rework. See `CLAUDE.md` for the detailed review sequence.
