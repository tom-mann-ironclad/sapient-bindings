# Contributing

## Regenerating Bindings

This repository ships checked-in generated bindings and does not compile
`.proto` files during downstream package installs.

The schema source-of-truth in this repository lives under `proto/`.

When the upstream SAPIENT schema changes, regenerate all checked-in bindings
from the repository copies of the schema source with:

```bash
./scripts/regenerate-bindings.sh
```

That script:

- stages the repository `.proto` files into temporary workspaces
- regenerates Rust bindings into `rust/sapient-rs/src/generated/`
- regenerates Python bindings into `python/sapient-py/src/sapient_msg/`

Generated outputs, especially the Python `*_pb2.py` files, are sensitive to the
exact `protoc` version. CI is pinned to `protoc 28.0`, so regeneration should
use the same version when updating checked-in generated files.

The Rust regeneration script resolves `prost-build` dynamically when needed.

## Verification

After regeneration, run:

```bash
cargo test
cargo test -p sapient-rs --features v1_0,v2_0
python3 -m compileall python/sapient-py/src
./scripts/build-python-package.sh
./scripts/check-python-package.sh
```

The Python packaging helpers expect `python -m build` and `python -m twine` to
be installed in the active environment.

## Release Automation

GitHub Actions workflows are defined in `.github/workflows`:

- `ci.yml` validates language-specific regeneration, Rust, and Python packaging on pushes and pull requests
- `release.yml` publishes tagged releases

Manual releases via `workflow_dispatch` take:

- `package`: `rust` or `python`
- `version`: the package version to publish

Release tag conventions:

- Rust: `rust-sapient-rs-vX.Y.Z`
- Python: `python-sapient-py-vX.Y.Z`

Release protections:

- Rust publishing is gated by the `crates-io` environment
- Python publishing is gated by the `pypi` environment
- the tag version, or manual `version` input, must match the package manifest version exactly

Repository secrets:

- `CARGO_REGISTRY_TOKEN` for crates.io publishing

The Python publish job uses PyPI trusted publishing via GitHub OIDC.

If you prefer package-local commands, the Rust crate lives in:

```bash
cd rust/sapient-rs
cargo test
cargo package --allow-dirty --offline --no-verify
```

and the Python package lives in:

```bash
cd python/sapient-py
python3 -m build
python3 -m twine check dist/*
```
