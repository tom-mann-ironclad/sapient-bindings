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
- regenerates C# bindings into `csharp/sapient-csharp/Generated/`
- regenerates Go bindings into `go/sapient/generated/`
- regenerates Java bindings into `java/sapient-java/src/generated/java/`

Generated outputs, especially the Python `*_pb2.py` files, are sensitive to the
exact `protoc` version. CI is pinned to `protoc 28.0`, so regeneration should
use the same version when updating checked-in generated files.

The Rust regeneration script resolves `prost-build` dynamically when needed.
The Go regeneration script requires `protoc-gen-go`:

```bash
go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.36.0
```

## Verification

After regeneration, run:

```bash
cargo test
cargo test -p sapient-rs --features v1_0,v2_0
python3 -m compileall python/sapient-py/src
./scripts/build-python-package.sh
./scripts/check-python-package.sh
dotnet build csharp/sapient-csharp/Sapient.Bindings.csproj
dotnet test csharp/sapient-csharp.Tests/Sapient.Bindings.Tests.csproj
dotnet pack csharp/sapient-csharp/Sapient.Bindings.csproj --configuration Release
(cd go/sapient && go test ./...)
(cd java/sapient-java && ./mvnw verify)
```

The Python packaging helpers expect `python -m build` and `python -m twine` to
be installed in the active environment.

## Release Automation

GitHub Actions workflows are defined in `.github/workflows`:

- `ci.yml` validates language-specific regeneration, Rust, Python packaging, C# packaging, Go tests, and Java packaging on pushes and pull requests
- `release.yml` publishes tagged releases

Manual releases via `workflow_dispatch` take:

- `package`: `rust`, `python`, `csharp`, or `java`
- `version`: the package version to publish

Release tag conventions:

- Rust: `rust-sapient-rs-vX.Y.Z`
- Python: `python-sapient-py-vX.Y.Z`
- C#: `csharp-sapient-bindings-vX.Y.Z`
- Java: `java-sapient-bindings-vX.Y.Z`

Release protections:

- Rust publishing is gated by the `crates-io` environment
- Python publishing is gated by the `pypi` environment
- C# publishing is gated by the `Nuget` environment
- Java publishing is gated by the `maven-central` environment
- the tag version, or manual `version` input, must match the package manifest version exactly

Repository secrets:

- `CARGO_REGISTRY_TOKEN` for crates.io publishing
- `MAVEN_CENTRAL_USERNAME` / `MAVEN_CENTRAL_PASSWORD`: a Maven Central Portal
  user token (generated at https://central.sonatype.com/account, not your login)
- `MAVEN_GPG_PRIVATE_KEY`: ASCII-armored private key used to sign Java artifacts
  (`gpg --armor --export-secret-keys <KEY_ID>`); the public key must be
  published to a keyserver such as `keys.openpgp.org`
- `MAVEN_GPG_PASSPHRASE`: passphrase for that key

Repository or environment variables:

- `NUGET_USER` for NuGet trusted publishing. This should be the NuGet account username, not an email address.

The Python publish job uses PyPI trusted publishing via GitHub OIDC.
The C# publish job uses NuGet trusted publishing via GitHub OIDC.
The Java publish job uploads to the Maven Central Portal with
`central-publishing-maven-plugin`. The `io.github.tom-mann-ironclad` namespace
must be verified in the Central Portal before the first release.

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

and the C# package lives in:

```bash
cd csharp/sapient-csharp
dotnet build --configuration Release
dotnet test ../sapient-csharp.Tests/Sapient.Bindings.Tests.csproj --configuration Release
dotnet pack --configuration Release --no-build
```

and the Java package lives in:

```bash
cd java/sapient-java
./mvnw verify
```
