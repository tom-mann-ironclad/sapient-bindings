#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
work_dir="$repo_root/target/proto-go"
include_root="$work_dir/include"
output_root="$repo_root/go/sapient/generated"
module_path="github.com/tom-mann-ironclad/sapient-bindings/go/sapient"

if ! command -v protoc-gen-go >/dev/null 2>&1; then
    echo "protoc-gen-go is not available. Install it with:" >&2
    echo "  go install google.golang.org/protobuf/cmd/protoc-gen-go@v1.36.0" >&2
    exit 1
fi

rm -rf "$work_dir" "$output_root"
mkdir -p \
    "$include_root/sapient_msg/bsi_flex_335_v1_0" \
    "$include_root/sapient_msg/bsi_flex_335_v2_0" \
    "$output_root"

cp "$repo_root"/proto/bsi_flex_335_v1_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v1_0/"
cp "$repo_root"/proto/bsi_flex_335_v2_0/*.proto "$include_root/sapient_msg/bsi_flex_335_v2_0/"
cp "$repo_root/proto/proto_options.proto" "$include_root/sapient_msg/proto_options.proto"

set_go_package() {
    local package_path="$1"
    local package_name="$2"
    shift 2

    for proto_file in "$@"; do
        awk -v package_path="$package_path" -v package_name="$package_name" '
            /^option go_package = / { next }
            /^package / {
                print
                print "option go_package = \"" package_path ";" package_name "\";"
                next
            }
            { print }
        ' "$proto_file" > "$proto_file.tmp"
        mv "$proto_file.tmp" "$proto_file"
    done
}

set_go_package \
    "$module_path/generated/proto_options" \
    "proto_options" \
    "$include_root/sapient_msg/proto_options.proto"
set_go_package \
    "$module_path/generated/bsi_flex_335_v1_0" \
    "bsi_flex_335_v1_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto
set_go_package \
    "$module_path/generated/bsi_flex_335_v2_0" \
    "bsi_flex_335_v2_0" \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

echo "Regenerating checked-in Go protobuf bindings..."
protoc \
    --proto_path="$include_root" \
    --go_out="$repo_root/go/sapient" \
    --go_opt=module="$module_path" \
    "$include_root/sapient_msg/proto_options.proto" \
    "$include_root"/sapient_msg/bsi_flex_335_v1_0/*.proto \
    "$include_root"/sapient_msg/bsi_flex_335_v2_0/*.proto

echo "Go bindings regenerated in $output_root"
