#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work_dir="$repo_root/target/proto-py"
include_root="$work_dir/include"
output_root="$repo_root/python/sapient-py/src"

rm -rf "$work_dir" "$output_root/sapient_msg"
mkdir -p \
    "$include_root/sapient_msg/bsi_flex_335_v1_0" \
    "$include_root/sapient_msg/bsi_flex_335_v2_0" \
    "$output_root"

cp "$repo_root"/proto/bsi_flex_335_v1_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v1_0/"
cp "$repo_root"/proto/bsi_flex_335_v2_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v2_0/"
cp "$repo_root/proto/proto_options.proto" "$include_root/sapient_msg/proto_options.proto"

echo "Regenerating checked-in Python protobuf bindings..."
protoc \
    --proto_path="$include_root" \
    --python_out="$output_root" \
    "$include_root/sapient_msg/proto_options.proto" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

mkdir -p \
    "$output_root/sapient_msg/bsi_flex_335_v1_0" \
    "$output_root/sapient_msg/bsi_flex_335_v2_0"
touch \
    "$output_root/sapient_msg/__init__.py" \
    "$output_root/sapient_msg/bsi_flex_335_v1_0/__init__.py" \
    "$output_root/sapient_msg/bsi_flex_335_v2_0/__init__.py"

echo "Python bindings regenerated in $output_root/sapient_msg"
