#!/usr/bin/env bash
# Generate the RTL of Occamy for one variant, with upstream's own flow (`make -C target/sim rtl`).
# usage: nix develop -c recipe/generate.sh <workdir> <variant: full|q2c2>
# Needs network access (git clone, bender checkout). Writes <workdir>/<variant>/occamy: a clone of the
# pinned commit with its dependencies checked out by bender (.bender/git/checkouts) and the generated
# files in target/sim/src and target/sim/test/testharness.sv. recipe/resolve.sh reads that tree.
#
# Pins (recipe/README.md): occamy d5d0d8320bb633c36319e5477006d4c17cf0e448. Every dependency is checked
# out at the revision of Bender.lock, never at the one that Bender.yml names: Bender.yml asks for the
# branch `occamy` of snitch_cluster, which no longer exists upstream; Bender.lock has
# da57b043dfe0ba563a55a9d83bf362873c713648, which is reachable by hash only.
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
w=$(realpath -m "$1")
variant=$2
occamy_commit=d5d0d8320bb633c36319e5477006d4c17cf0e448
snitch_commit=da57b043dfe0ba563a55a9d83bf362873c713648
export LC_ALL=C

# The Python of the generators (hjson, mako, jsonref, ...) first on PATH: the scripts start with
# `#!/usr/bin/env python3`. The packages of the Python of the shell (PYTHONPATH) stay out.
unset PYTHONPATH
export PATH="$OCCAMY_PY/bin:$PATH"

case "$variant" in
  full) cfg_override="" ;;                 # target/sim/cfg/full.hjson, the default of the Makefile
  q2c2) cfg_override=cfg/q2c2.hjson ;;     # recipe/q2c2.hjson, copied next to full.hjson
  *) echo "unknown variant $variant" >&2; exit 1 ;;
esac

rm -rf "$w/$variant"; mkdir -p "$w/$variant"
cd "$w/$variant"
git clone -q https://github.com/pulp-platform/occamy.git occamy
git -C occamy checkout -q "$occamy_commit"
test "$(git -C occamy rev-parse HEAD)" = "$occamy_commit"
cd occamy

# Dependencies at the revisions of Bender.lock (bender checkout never updates the lock file).
bender checkout
test "$(git -C "$(bender path snitch_cluster)" rev-parse HEAD)" = "$snitch_commit"
git diff --quiet Bender.lock   # the lock file is unchanged

if [ -n "$cfg_override" ]; then
  # our configuration: full.hjson with two values changed; recipe/q2c2.diff is the difference
  cp "$here/q2c2.hjson" "target/sim/$cfg_override"
  { diff -u target/sim/cfg/full.hjson "target/sim/$cfg_override" || true; } | tail -n +3 > "$w/$variant/q2c2.diff.new"
  tail -n +3 "$here/q2c2.diff" | diff - "$w/$variant/q2c2.diff.new" >/dev/null \
    || { echo "recipe/q2c2.hjson does not match recipe/q2c2.diff" >&2; exit 1; }
fi

# Upstream's rule: clint, plic, soc_ctrl, hbm_xbar_ctrl, quadrant_s1_ctrl, the SoC and the testharness,
# formatted with verible-verilog-format. One job: the grouped targets of the Makefile are not safe in parallel.
make -j1 -C target/sim rtl CFG_OVERRIDE="$cfg_override"

# The C headers of the platform (register layouts, configuration, base addresses) that the host software
# includes: the PLATFORM_HEADERS of the same Makefile, from the same configuration.
hdr=sw/shared/platform/generated
make -j1 -C target/sim CFG_OVERRIDE="$cfg_override" \
  $hdr/occamy_cfg.h $hdr/occamy_base_addr.h $hdr/clint.h $hdr/occamy_soc_ctrl.h \
  $hdr/snitch_cluster_peripheral.h $hdr/snitch_quad_peripheral.h $hdr/snitch_hbm_xbar_peripheral.h $hdr/idma.h

# Tool versions that the bytes depend on
{
  echo "$(bender --version)"
  echo "verible $(verible-verilog-format --version | head -1 | cut -f2)"
  echo "clang-format $(clang-format --version | sed 's/.*version //')"
  echo "python $(python3 --version)"
  python3 - <<'PY'
import importlib.metadata as m
for p in ["hjson", "Mako", "jsonref", "jsonschema", "PyYAML", "tabulate", "setuptools"]:
    print(p, m.version(p))
PY
  echo "$(make --version | head -1)"
} > "$w/$variant/tools.txt"
