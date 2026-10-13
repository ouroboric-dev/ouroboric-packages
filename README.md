# ouroboric-packages

The official `ouroboric` package on PyPI. It identifies the [Ouroboric](https://ouroboric.dev)
project and holds its name on the registry, so that nobody else can publish a package that looks
official. It installs a small `ouroboric` command that prints what the project is and where to find
it.

| Registry | Package | Source |
|---|---|---|
| PyPI | [`ouroboric`](https://pypi.org/project/ouroboric/) | [`python/`](python/) |

Official Ouroboric software is published only from the
[`ouroboric-dev`](https://github.com/ouroboric-dev) GitHub organisation, the PyPI package above, the
npm scope `@ouroboric`, and container images at `ghcr.io/ouroboric-dev`. Anything else using the name,
including any `ouroboric` crate on crates.io, is not ours.

## Contributing

Run `uvx pre-commit install` once after cloning, so `gitleaks` checks each commit for secrets.

## Licence

The code is MIT. The name "Ouroboric" and the logos are trademarks and are not covered by the code
licence; see [TRADEMARKS.md](https://github.com/ouroboric-dev/ouroboric/blob/main/TRADEMARKS.md).
