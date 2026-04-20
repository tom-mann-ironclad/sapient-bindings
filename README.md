# SAPIENT

This repository is a small monorepo for SAPIENT / BSI Flex 335 language
bindings generated from the protocol schema source.

It is maintained by Tom Mann, one of the Principal Authors of the SAPIENT
standard. It is not affiliated with or sponsored by DSTL.

The generated bindings are distinct from the protocol specification itself.
They are derived from schema definitions published in the
[DSTL SAPIENT-Proto-Files repository](https://github.com/dstl/SAPIENT-Proto-Files).
Those source schema files are attributed to DSTL and remain available under
Apache License 2.0.
The repository copy of the schema source lives under `proto/`.

## Packages

- Rust crate: `rust/sapient-rs`
- Python package: `python/sapient-py`

## Rust Crate

The Rust crate lives in `rust/sapient-rs` and is published as `sapient-rs`.
It ships pre-generated Rust bindings plus Rust framing helpers.

## Python Package

The Python package lives in `python/sapient-py` and is intended for PyPI
publication as `sapient-py`. It ships generated Python protobuf modules plus
Python framing helpers.

## Maintenance

Maintainers can regenerate all checked-in bindings with:

```bash
./scripts/regenerate-bindings.sh
```

For language-specific regeneration, use:

```bash
./scripts/regenerate-rust-bindings.sh
./scripts/regenerate-python-bindings.sh
```

The Rust regeneration helper uses Cargo offline mode and expects `prost-build`
to be available in the local Cargo cache.

## Package Checks

Rust:

```bash
cargo test
cargo test -p sapient-rs --features v1_0,v2_0
cargo package -p sapient-rs --allow-dirty --offline --no-verify
```

Python:

```bash
python3 -m compileall python/sapient-py/src
./scripts/build-python-package.sh
./scripts/check-python-package.sh
```

The Python packaging scripts expect `python -m build` and `python -m twine` to
be installed in the active environment.

## CI/CD

GitHub Actions workflows live under `.github/workflows`:

- `ci.yml` runs regeneration checks, Rust tests, Rust packaging, Python source compilation, Python build, and `twine check`
- `release.yml` publishes:
  - `sapient-rs` on tags matching `rust-sapient-rs-v*`
  - `sapient-py` on tags matching `python-sapient-py-v*`
  - one package at a time manually via `workflow_dispatch`

Release safeguards:

- publish jobs require approved GitHub environments:
  - `crates-io` for Rust
  - `pypi` for Python
- tag or manual-input versions must match the package manifest version exactly

## Licensing

The Rust and Python packages are licensed under either of:

- MIT
- Apache-2.0

at your option.

The SAPIENT `.proto` schema source from which these bindings were generated is
derived from the DSTL SAPIENT-Proto-Files repository and is separately
attributed to DSTL under Apache License 2.0.
