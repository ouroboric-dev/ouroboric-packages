---
title: Architecture
status: accepted
date: 2026-10-12
related: [../README.md]
style: reference
---

# Architecture

One package with no dependencies and no network access:

| Package | Language | Entry point | Prints |
|---|---|---|---|
| `python/` | Python 3.9+ | `ouroboric` console script, `python -m ouroboric` | project name, description, website, source |

There is no crates.io package: Ouroboric has no Rust code planned, so the name is not claimed there.

Releases are published by a human from a local checkout. Registry tokens are created for one
upload, then revoked.
