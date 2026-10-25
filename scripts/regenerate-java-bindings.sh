#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work_dir="$repo_root/target/proto-java"
include_root="$work_dir/include"
output_root="$repo_root/java/sapient-java/src/generated/java"

rm -rf "$work_dir" "$output_root"
mkdir -p \
    "$include_root/sapient_msg/bsi_flex_335_v1_0" \
    "$include_root/sapient_msg/bsi_flex_335_v2_0" \
    "$output_root"

cp "$repo_root"/proto/bsi_flex_335_v1_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v1_0/"
cp "$repo_root"/proto/bsi_flex_335_v2_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v2_0/"
cp "$repo_root/proto/proto_options.proto" "$include_root/sapient_msg/proto_options.proto"

set_java_package() {
    local package="$1"
    shift

    for proto_file in "$@"; do
        awk -v java_package="$package" '
            /^option java_package = / {
                print "option java_package = \"" java_package "\";"
                next
            }
            { print }
        ' "$proto_file" > "$proto_file.tmp"
        mv "$proto_file.tmp" "$proto_file"
    done
}

set_java_package \
    "io.github.tommannironclad.sapient.proto" \
    "$include_root/sapient_msg/proto_options.proto"
set_java_package \
    "io.github.tommannironclad.sapient.bsiflex335.v1_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto
set_java_package \
    "io.github.tommannironclad.sapient.bsiflex335.v2_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

echo "Regenerating checked-in Java protobuf bindings..."
protoc \
    --proto_path="$include_root" \
    --java_out="$output_root" \
    "$include_root/sapient_msg/proto_options.proto" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

echo "Java bindings regenerated in $output_root"
