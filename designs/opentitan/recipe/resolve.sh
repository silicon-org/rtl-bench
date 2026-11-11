#!/usr/bin/env bash
# Resolve the file lists of opentitan.earlgrey and copy the files from an upstream checkout.
# usage: nix develop -c recipe/resolve.sh <opentitan checkout at the pinned commit>
# Writes designs/opentitan/src/ (copies, upstream paths; generated RTL in src/generated/),
# variants/earlgrey/{synth,sim}.f and the define and include lists in variants/earlgrey/*.txt.
#
# The lists are what upstream's tools produce: FuseSoC (the lowRISC fork, ot-0.5.dev0, as
# python-requirements.txt pins it) resolves
#   lowrisc:systems:chip_earlgrey_asic:0.1   target syn, flag fileset_top  (hw/syn/tools/dvsim/common_syn_cfg.hjson)
#   lowrisc:dv:chip_verilator_sim:0.1        target sim, flag fileset_top, tool verilator
#                                            (hw/dv/tools/dvsim/verilator.hjson, hw/top_earlgrey/dv/verilator)
# The cores root is the checkout without the FPGA technology libraries (hw/ip/prim_xilinx,
# hw/ip/prim_xilinx_ultrascale) and without the other chip, hw/top_englishbreakfast, whose cores
# carry the same names as the Earl Grey ones. primgen then generates the abstract primitives
# (prim_flop, ...) with the generic implementation only.
set -euo pipefail
ot=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
out=$here/src
export LC_ALL=C
w=/tmp/rtl-bench-opentitan-resolve
rm -rf "$w"; mkdir -p "$w"

# the tools: the lowRISC forks of FuseSoC and edalize, the pins of python-requirements.txt
uv venv --python "$(command -v python3)" "$w/venv" >/dev/null
uv pip install --python "$w/venv/bin/python" \
  "edalize @ https://github.com/lowRISC/edalize/archive/refs/tags/v0.4.0.zip" \
  "fusesoc @ https://github.com/lowRISC/fusesoc/archive/refs/tags/ot-0.5.dev0.zip" \
  hjson==3.1.0 mako==1.1.6 pyyaml==6.0.2 simplesat==0.9.2 jsonschema==4.17.3 >/dev/null

rsync -a --exclude .git --exclude hw/ip/prim_xilinx --exclude hw/ip/prim_xilinx_ultrascale \
  --exclude hw/top_englishbreakfast "$ot/" "$w/ot/"
cd "$w/ot"
fs() { PATH="$w/venv/bin:$PATH" fusesoc --cores-root . run --no-export --flag=fileset_top "$@" 2>&1 | grep -v "unknown file type" | tail -2; }
fs --target=syn --tool icarus --build-root="$w/syn" --setup lowrisc:systems:chip_earlgrey_asic:0.1
fs --target=sim --tool verilator --build-root="$w/sim" --setup lowrisc:dv:chip_verilator_sim:0.1
synscr=$(ls "$w"/syn/syn-icarus/*.scr)
simvc=$(ls "$w"/sim/sim-verilator/*.vc)
simmk="$w/sim/sim-verilator/config.mk"

mkdir -p "$here/variants/earlgrey"
python3 -I "$here/recipe/flist.py" "$synscr" "$w/syn/syn-icarus" "$w/ot" > "$w/syn.txt"
python3 -I "$here/recipe/flist.py" "$simvc" "$w/sim/sim-verilator" "$w/ot" > "$w/sim.txt"
# the lists, in the order of the tools (waivers and packages first)
sed -n 's/^F //p' "$w/syn.txt" | sed 's#^#src/#' > "$here/variants/earlgrey/synth.f"
sed -n 's/^F //p' "$w/sim.txt" | sed 's#^#src/#' > "$here/variants/earlgrey/sim.f"
sed -n 's/^D //p' "$w/syn.txt" > "$here/variants/earlgrey/synth-defines.txt"
sed -n 's/^D //p' "$w/sim.txt" > "$here/variants/earlgrey/sim-defines.txt"
sed -n 's/^I //p' "$w/syn.txt" | sed 's#^#src/#' > "$here/variants/earlgrey/synth-incdirs.txt"
sed -n 's/^I //p' "$w/sim.txt" | sed 's#^#src/#' > "$here/variants/earlgrey/sim-incdirs.txt"
cp "$simmk" "$here/variants/earlgrey/sim-edalize-config.mk.txt"

# copy: the listed files, the headers of the include directories and what the listed files include
mkdir -p "$out/generated"
cp -a "$w/syn/syn-icarus/generated/." "$out/generated/"
cp -a "$w/sim/sim-verilator/generated/." "$out/generated/"
listed=$( { sed -n 's/^F //p' "$w/syn.txt"; sed -n 's/^F //p' "$w/sim.txt"; } | grep -v '^generated/' | sort -u)
dirs=$( { sed -n 's/^I //p' "$w/syn.txt"; sed -n 's/^I //p' "$w/sim.txt"; } | grep -v '^generated/' | sort -u)
{
  echo "$listed"
  for d in $dirs; do (cd "$w/ot" && find "$d" -maxdepth 1 -type f \( -name '*.h' -o -name '*.svh' -o -name '*.vh' \)); done
  python3 -I "$here/recipe/includes.py" "$w/ot" $dirs -- $listed
  echo LICENSE; echo NOTICE
  echo hw/vendor/lowrisc_ibex/LICENSE; echo hw/vendor/lowrisc_ibex/NOTICE; echo hw/vendor/lowrisc_ibex/CREDITS.md
  echo hw/vendor/pulp_riscv_dbg/LICENSE; echo hw/vendor/pulp_riscv_dbg/LICENSE.SiFive
  echo hw/vendor/lowrisc_ibex.lock.hjson; echo hw/vendor/pulp_riscv_dbg.lock.hjson
} | sort -u | while read -r f; do
  mkdir -p "$out/$(dirname "$f")"
  cp -p "$w/ot/$f" "$out/$f"
done
# the SDC that upstream ships for the chip, unmodified
mkdir -p "$here/constraints"
cp -p "$w/ot/hw/top_earlgrey/syn/chip_earlgrey_asic.sdc" "$w/ot/hw/top_earlgrey/syn/ot.sdc_setup.tcl" "$here/constraints/"
rm -rf "$w"
