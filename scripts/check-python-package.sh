#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
package_root="$repo_root/python/sapient-py"

if ! python3 -c "import importlib.util, sys; sys.exit(0 if importlib.util.find_spec('twine') else 1)"; then
    echo "python -m twine is not available. Install the 'twine' package first." >&2
    exit 1
fi

python3 -m twine check "$package_root"/dist/*
