#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$repo_root/scripts/regenerate-rust-bindings.sh"
"$repo_root/scripts/regenerate-python-bindings.sh"
"$repo_root/scripts/regenerate-csharp-bindings.sh"
