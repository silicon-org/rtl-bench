#!/usr/bin/env bash
# Resolve the file lists of occamy.full and occamy.q2c2 and copy the files into src/.
# usage: nix develop -c recipe/resolve.sh <workdir of generate.sh> [design directory]
# The workdir holds <variant>/occamy for full and q2c2 (recipe/generate.sh). The design directory defaults
# to the one of this recipe; the determinism check (recipe/README.md) passes another one.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
export LC_ALL=C
exec python3 -I "$here/resolve.py" "$1" "${2:-$(cd "$here/.." && pwd)}"
