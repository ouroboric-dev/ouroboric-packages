---
title: Architecture
status: accepted
date: 2026-10-12
related: [../README.md]
style: reference
---

# Architecture

Two independent packages with no dependencies and no network access:

| Package | Language | Entry point | Prints |
|---|---|---|---|
| `python/` | Python 3.9+ | `ouroboric` console script, `python -m ouroboric` | project name, description, website, source |
| `rust/` | Rust 2021 | `ouroboric` binary | the same text |

Releases are published by a human from a local checkout. Registry tokens are created for one
upload, then revoked.
