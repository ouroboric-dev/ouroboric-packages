# ouroboric-packages

The official `ouroboric` packages on PyPI and crates.io. They identify the
[Ouroboric](https://ouroboric.dev) project and hold its name on those registries, so that nobody else
can publish a package that looks official. Each one installs a small `ouroboric` command that prints
what the project is and where to find it.

| Registry | Package | Source |
|---|---|---|
| PyPI | [`ouroboric`](https://pypi.org/project/ouroboric/) | [`python/`](python/) |
| crates.io | [`ouroboric`](https://crates.io/crates/ouroboric) | [`rust/`](rust/) |

Official Ouroboric software is published only from the
[`ouroboric-dev`](https://github.com/ouroboric-dev) GitHub organisation, the registries above, the npm
scope `@ouroboric`, and container images at `ghcr.io/ouroboric-dev`. Anything else using the name is
not ours.

## Licence

The code is MIT. The name "Ouroboric" and the logos are trademarks and are not covered by the code
licence; see [TRADEMARKS.md](https://github.com/ouroboric-dev/ouroboric/blob/main/TRADEMARKS.md).
