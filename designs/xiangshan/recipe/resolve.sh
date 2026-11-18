#!/usr/bin/env bash
# Copy the generated sources of xiangshan.minimal from the work directory of recipe/generate.sh, the C++ and Verilog
# harness of the difftest, the licence files and the workload images, and write the file lists.
# usage: nix develop -c recipe/resolve.sh <work dir>
set -euo pipefail
w=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
xs=$w/xs
out=$here/src
rm -rf "$out" "$here/licences" "$here/sim"
mkdir -p "$out/generated/synth/rtl" "$out/generated/sim/build" "$out/difftest/src/test" "$out/difftest/config" "$here/variants/minimal" "$here/sim"

# Synthesis view: build/rtl of `make verilog` (XSTop). The .fir (the FIRRTL of the run, 0.8 GB) and the metadata
# (dts, json, graphml, plusArgs) are not copied.
cp -p "$w/out/synth/rtl/"*.sv "$w/out/synth/rtl/"*.v "$w/out/synth/rtl/filelist.f" \
  "$w/out/synth/rtl/firrtl_black_box_resource_files.f" "$w/out/synth/rtl/XSTop.sv.conf" "$out/generated/synth/rtl/"

# Simulation view: build of `make sim-verilog` (SimTop), laid out as upstream's build directory, because the emu reads
# $NOOP_HOME/build/constantin.txt. Not copied: SimTop.fir and the metadata.
s=$w/out/sim
mkdir -p "$out/generated/sim/build/rtl" "$out/generated/sim/build/generated-src"
cp -p "$s/rtl/"*.sv "$s/rtl/"*.v "$s/rtl/filelist.f" "$s/rtl/firrtl_black_box_resource_files.f" "$s/rtl/SimTop.sv.conf" \
  "$out/generated/sim/build/rtl/"
cp -p "$s/generated-src/"* "$out/generated/sim/build/generated-src/"
cp -p "$s/chisel_db.cpp" "$s/chisel_db.h" "$s/constantin.cpp" "$s/constantin.hpp" "$s/constantin.txt" "$out/generated/sim/build/"

# The difftest harness: what its Makefile compiles into the emu (common, difftest, plugin/spikedasm, verilator; the Verilog
# of common) and the config header. The VCS, Palladium and runahead parts are not used.
d=$xs/difftest
cp -rp "$d/src/test/csrc" "$out/difftest/src/test/csrc"
cp -rp "$d/src/test/vsrc" "$out/difftest/src/test/vsrc"
rm -rf "$out/difftest/src/test/csrc/vcs" "$out/difftest/src/test/csrc/plugin/runahead" "$out/difftest/src/test/vsrc/vcs"
cp -p "$d/config/config.h" "$out/difftest/config/"

# file lists: the filelist.f of the run (in its order), then the memory modules (array_*_ext.v, written by
# scripts/gen_sep_mem.sh and not in filelist.f)
export LC_ALL=C
list() { # <rtl dir relative to src> <filelist.f>
  sed -E "s#^(\./)?#$1/#" "$2"
  (cd "$out/$1" && ls array*_ext.v | sort | sed "s#^#$1/#")
}
list generated/synth/rtl "$out/generated/synth/rtl/filelist.f" > "$here/variants/minimal/synth.f"
sed -i 's#^#src/#' "$here/variants/minimal/synth.f"
{
  list generated/sim/build/rtl "$out/generated/sim/build/rtl/filelist.f" | sed 's#^#src/#'
  ls "$out"/difftest/src/test/vsrc/common/*.v | sed "s#^$here/##"
  for dir in difftest/src/test/csrc/common difftest/src/test/csrc/difftest difftest/src/test/csrc/plugin/spikedasm difftest/src/test/csrc/verilator; do
    ls "$out/$dir/"*.cpp | sed "s#^$here/##"
  done
  echo src/generated/sim/build/generated-src/difftest-dpic.cpp
  echo src/generated/sim/build/chisel_db.cpp
  echo src/generated/sim/build/constantin.cpp
} > "$here/variants/minimal/sim.f"

# licence files, read from the checkouts
lic=$here/licences
mkdir -p "$lic"/{xiangshan,difftest,rocket-chip,cde,hardfloat,huancun,coupledL2,openLLC,yunsuan}
cp -p "$xs/LICENSE" "$lic/xiangshan/LICENSE"
cp -p "$xs/difftest/LICENSE" "$lic/difftest/LICENSE"
cp -p "$xs/rocket-chip/LICENSE.Berkeley" "$xs/rocket-chip/LICENSE.jtag" "$xs/rocket-chip/LICENSE.SiFive" "$lic/rocket-chip/"
cp -p "$xs/rocket-chip/cde/LICENSE" "$lic/cde/LICENSE"
cp -p "$xs/rocket-chip/hardfloat/LICENSE" "$lic/hardfloat/LICENSE"
cp -p "$xs/huancun/LICENSE" "$xs/huancun/LICENSE.SiFive" "$lic/huancun/"
cp -p "$xs/coupledL2/LICENSE" "$xs/coupledL2/LICENSE.SiFive" "$lic/coupledL2/"
cp -p "$xs/openLLC/LICENSE" "$xs/openLLC/LICENSE.SiFive" "$lic/openLLC/"
cp -p "$xs/yunsuan/LICENSE" "$lic/yunsuan/LICENSE"

# workload images of ready-to-run
cp -p "$xs/ready-to-run/coremark-2-iteration.bin" "$xs/ready-to-run/microbench.bin" "$here/sim/"
