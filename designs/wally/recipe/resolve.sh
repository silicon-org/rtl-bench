#!/usr/bin/env bash
# Resolve the file lists of wally.rv64gc and copy the files from an upstream checkout.
# usage: nix develop -c recipe/resolve.sh <cvw checkout at the pinned commit>
# Writes designs/wally/src/ (copies, upstream paths), variants/rv64gc/{synth,sim}.f.
set -euo pipefail
cvw=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
work=$(mktemp -d)
out=$here/src
export WALLY=$cvw

# 1. Synthesis top: upstream's own wrapper generator (synthDC/scripts/wrapperGen.py), as synth.tcl runs it.
mkdir -p "$out/generated"
python3 "$cvw/synthDC/scripts/wrapperGen.py" wallypipelinedcore "$out/generated"

# 2. Files that each top needs: the module closure over every instantiation in the sources,
#    in any generate branch (recipe/closure.py).
export LC_ALL=C
python3 -I "$here/recipe/closure.py" "$cvw" wallypipelinedcore src/cvw.sv | grep -v '^src/cvw.sv$' > "$work/synth.list"
python3 -I "$here/recipe/closure.py" "$cvw" testbench src/cvw.sv | grep -v '^src/cvw.sv$' > "$work/sim.list"
cd "$cvw"

# 3. Variant file lists (paths relative to the design directory; the package first).
mkdir -p "$here/variants/rv64gc"
{ echo src/src/cvw.sv
  echo src/generated/wallypipelinedcorewrapper.sv
  sed 's#^#src/#' "$work/synth.list"; } > "$here/variants/rv64gc/synth.f"
{ comm -13 "$work/synth.list" "$work/sim.list" | grep -v '^testbench/' | sed 's#^#src/#'
  comm -13 "$work/synth.list" "$work/sim.list" | grep '^testbench/' | grep -v '^testbench/testbench.sv$' | sed 's#^#src/#'
  echo src/testbench/testbench.sv
  echo src/sim/verilator/wrapper.c; } > "$here/variants/rv64gc/sim.f"

# 4. Copy every listed file, plus configuration, support files, workload sources and licence.
extra="LICENSE
config/shared/config-shared.vh
config/shared/parameter-defs.vh
config/rv64gc/config.vh
testbench/Makefile
testbench/mempipeprobe.svh
bin/elf2hex
bin/extractFunctionRadix.sh
tests/coverage64gc/ieu.S
tests/coverage64gc/fpu.S
tests/coverage64gc/WALLY-init-lib.h
tests/coverage64gc/WALLY-selfcheck.h
examples/link/link.ld
benchmarks/coremark/build.sh
benchmarks/coremark/crt.S
benchmarks/coremark/core_portme.c
benchmarks/coremark/core_portme.h
benchmarks/coremark/ee_printf.c
config/rv64gc/act/link.ld
config/rv64gc/act/rvmodel_macros.h
addins/coremark/core_list_join.c
addins/coremark/core_main.c
addins/coremark/core_matrix.c
addins/coremark/core_state.c
addins/coremark/core_util.c
addins/coremark/coremark.h
addins/coremark/LICENSE.md"
{ cat "$here/variants/rv64gc/synth.f" "$here/variants/rv64gc/sim.f" | grep -v '^src/generated/' | sed 's#^src/##'; echo "$extra"; } | sort -u | while read -r f; do
  mkdir -p "$out/$(dirname "$f")"
  cp -p "$cvw/$f" "$out/$f"
done
rm -rf "$work"
