# AGENTS.md

This repo holds a minimal package that identifies the Ouroboric project on PyPI.

- `python/`: the PyPI package (`pyproject.toml`, `src/ouroboric/`). Test with `uv run --project python pytest python/tests`.
- The command prints the name, a one-line description, the website and the source.
- Do not add dependencies. Do not add network access.
- Only a human publishes releases. Never handle registry tokens.
- Architecture: [docs/architecture.md](docs/architecture.md).
