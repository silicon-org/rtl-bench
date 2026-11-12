#!/usr/bin/env bash
# Copy the generated sources of boom.smallv3 from the work directory of recipe/generate.sh
# and write the file lists.
# usage: nix develop -c recipe/resolve.sh <work dir>
set -euo pipefail
w=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
export LC_ALL=C
cy=$w/cy
g=$cy/sims/verilator/generated-src/chipyard.harness.TestHarness.SmallBoomV3Config
out=$here/src
rm -rf "$out"; mkdir -p "$out/generated" "$out/DRAMSim2" "$here/variants/smallv3"

# the generated RTL and the sources of the Verilator host (DPI) that firtool and Chipyard write with it
(cd "$g/gen-collateral" && find . -maxdepth 1 -type f \( -name '*.sv' -o -name '*.v' -o -name '*.cc' -o -name '*.h' \) | sed 's#^\./##' | sort) \
  | while read -r f; do cp -p "$g/gen-collateral/$f" "$out/generated/$f"; done

# DRAMSim2 (submodule tools/DRAMSim2), compiled into the simulator; TraceBasedSim.cpp is its own main
(cd "$cy/tools/DRAMSim2" && ls *.cpp *.h | grep -v '^TraceBasedSim.cpp$') | while read -r f; do cp -p "$cy/tools/DRAMSim2/$f" "$out/DRAMSim2/$f"; done

rel() { sed "s#^$g/gen-collateral/#src/generated/#; s#^\./#src/generated/#"; }
# synthesis: the modules of ChipTop (top.f), the black boxes of the DUT (clock gate, IO cells) and the memories
{
  rel < "$g"/chipyard.harness.TestHarness.SmallBoomV3Config.top.f
  grep -E '/(EICG_wrapper|GenericDigitalInIOCell|GenericDigitalOutIOCell|plusarg_reader)\.v$' "$g"/chipyard.harness.TestHarness.SmallBoomV3Config.bb.f | rel
  echo src/generated/chipyard.harness.TestHarness.SmallBoomV3Config.top.mems.v
} | awk '!seen[$0]++' > "$here/variants/smallv3/synth.f"   # top.f already names the black boxes of bb.f
# simulation: what Chipyard's Verilator build reads (sim_files.common.f), then DRAMSim2
{
  rel < "$g/sim_files.common.f"
  ls "$out/DRAMSim2"/*.cpp | sed "s#^$here/##"
} > "$here/variants/smallv3/sim.f"

# the licence files of the repositories whose code is in the output (read from the checkouts), and
# the licence of Chisel, whose library modules (Queue, Arbiter, ...) are in it
lic=$here/licences
rm -rf "$lic"; mkdir -p "$lic"
for pair in \
  "chipyard:LICENSE LICENSE.SiFive" \
  "boom:generators/boom/LICENSE generators/boom/LICENSE.SiFive" \
  "rocket-chip:generators/rocket-chip/LICENSE.Berkeley generators/rocket-chip/LICENSE.jtag generators/rocket-chip/LICENSE.SiFive" \
  "hardfloat:generators/hardfloat/LICENSE" \
  "testchipip:generators/testchipip/LICENSE" \
  "rocket-chip-blocks:generators/rocket-chip-blocks/LICENSE" \
  "rocket-chip-inclusive-cache:generators/rocket-chip-inclusive-cache/LICENSE" \
  "diplomacy:generators/diplomacy/LICENSE.SiFive"; do
  name=${pair%%:*}; mkdir -p "$lic/$name"
  for f in ${pair#*:}; do cp -p "$cy/$f" "$lic/$name/$(basename "$f")"; done
done
mkdir -p "$lic/chisel"
curl -fsSL https://raw.githubusercontent.com/chipsalliance/chisel/8db8c1e063c22440c6fcbc61ea7e8923a969ad15/LICENSE -o "$lic/chisel/LICENSE"
