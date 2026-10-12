# AGENTS.md

This repo holds two minimal packages that identify the Ouroboric project on PyPI and crates.io.

- `python/`: the PyPI package (`pyproject.toml`, `src/ouroboric/`). Test with `uv run --project python pytest python/tests`.
- `rust/`: the crates.io crate (`Cargo.toml`, `src/main.rs`). Test with `cargo test --manifest-path rust/Cargo.toml`.
- Keep both commands' output identical: name, one-line description, website, source.
- Do not add dependencies. Do not add network access.
- Only a human publishes releases. Never handle registry tokens.
- Architecture: [docs/architecture.md](docs/architecture.md).
