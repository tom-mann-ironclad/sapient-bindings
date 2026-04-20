#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
crate_root="$repo_root/rust/sapient-rs"

mkdir -p "$tmp_dir/src"

cat >"$tmp_dir/Cargo.toml" <<'EOF'
[package]
name = "sapient-rs-binding-generator"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
prost-build = "0.14.3"
EOF

cat >"$tmp_dir/src/main.rs" <<'EOF'
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const SOURCE_PROTO_OPTIONS: &str = "proto/proto_options.proto";
const PACKAGE_ROOT: &str = "sapient_msg";
const PROTO_SETS: &[ProtoSet] = &[
    ProtoSet {
        source_dir: "proto/bsi_flex_335_v1_0",
        package_dir: "sapient_msg/bsi_flex_335_v1_0",
        root_proto: "sapient_msg/bsi_flex_335_v1_0/sapient_message.proto",
    },
    ProtoSet {
        source_dir: "proto/bsi_flex_335_v2_0",
        package_dir: "sapient_msg/bsi_flex_335_v2_0",
        root_proto: "sapient_msg/bsi_flex_335_v2_0/sapient_message.proto",
    },
];

struct ProtoSet {
    source_dir: &'static str,
    package_dir: &'static str,
    root_proto: &'static str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let repo_root = PathBuf::from(std::env::args().nth(1).ok_or("missing repo root")?);
    let crate_root = repo_root.join("rust/sapient-rs");
    let generated_dir = crate_root.join("src/generated");
    let work_dir = repo_root.join("target/proto-regeneration");
    let include_root = work_dir.join("proto_include");

    if work_dir.exists() {
        fs::remove_dir_all(&work_dir)?;
    }

    fs::create_dir_all(include_root.join(PACKAGE_ROOT))?;
    fs::create_dir_all(&generated_dir)?;

    for proto_set in PROTO_SETS {
        let staged_proto_dir = include_root.join(proto_set.package_dir);
        fs::create_dir_all(&staged_proto_dir)?;
        stage_local_protos(&repo_root.join(proto_set.source_dir), &staged_proto_dir)?;
    }

    fs::copy(
        repo_root.join(SOURCE_PROTO_OPTIONS),
        include_root.join(PACKAGE_ROOT).join("proto_options.proto"),
    )?;

    let root_protos: Vec<PathBuf> = PROTO_SETS
        .iter()
        .map(|proto_set| include_root.join(proto_set.root_proto))
        .collect();

    let mut config = prost_build::Config::new();
    config.out_dir(&generated_dir);
    config.compile_protos(&root_protos, &[include_root])?;

    Ok(())
}

fn stage_local_protos(source_dir: &Path, staged_dir: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) != Some("proto") {
            continue;
        }

        let destination = staged_dir.join(entry.file_name());
        fs::copy(path, destination)?;
    }

    Ok(())
}
EOF

echo "Regenerating checked-in Rust protobuf bindings..."
cargo run --quiet --manifest-path "$tmp_dir/Cargo.toml" -- "$repo_root"
cargo fmt --manifest-path "$crate_root/Cargo.toml" -- "$crate_root/src/generated/"*.rs
echo "Rust bindings regenerated in $crate_root/src/generated"
