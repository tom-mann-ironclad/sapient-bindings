#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work_dir="$repo_root/target/proto-csharp"
include_root="$work_dir/include"
output_root="$repo_root/csharp/sapient-csharp/Generated"

rm -rf "$work_dir" "$output_root"
mkdir -p \
    "$include_root/sapient_msg/bsi_flex_335_v1_0" \
    "$include_root/sapient_msg/bsi_flex_335_v2_0" \
    "$output_root/Common" \
    "$output_root/BsiFlex335V1_0" \
    "$output_root/BsiFlex335V2_0"

cp "$repo_root"/proto/bsi_flex_335_v1_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v1_0/"
cp "$repo_root"/proto/bsi_flex_335_v2_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v2_0/"
cp "$repo_root/proto/proto_options.proto" "$include_root/sapient_msg/proto_options.proto"

set_csharp_namespace() {
    local namespace="$1"
    shift

    for proto_file in "$@"; do
        awk -v csharp_namespace="$namespace" '
            /^option csharp_namespace = / { next }
            /^package / {
                print
                print "option csharp_namespace = \"" csharp_namespace "\";"
                next
            }
            { print }
        ' "$proto_file" > "$proto_file.tmp"
        mv "$proto_file.tmp" "$proto_file"
    done
}

set_csharp_namespace \
    "Sapient.Bindings.Proto" \
    "$include_root/sapient_msg/proto_options.proto"
set_csharp_namespace \
    "Sapient.Bindings.BsiFlex335.V1_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto
set_csharp_namespace \
    "Sapient.Bindings.BsiFlex335.V2_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

echo "Regenerating checked-in C# protobuf bindings..."
protoc \
    --proto_path="$include_root" \
    --csharp_out="$output_root/Common" \
    "$include_root/sapient_msg/proto_options.proto"

protoc \
    --proto_path="$include_root" \
    --csharp_out="$output_root/BsiFlex335V1_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto

protoc \
    --proto_path="$include_root" \
    --csharp_out="$output_root/BsiFlex335V2_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

echo "C# bindings regenerated in $output_root"
