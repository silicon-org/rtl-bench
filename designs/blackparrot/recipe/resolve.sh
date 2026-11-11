#!/usr/bin/env bash
# Resolve the file lists of blackparrot.unicore and copy the files from upstream checkouts.
# usage: nix develop -c recipe/resolve.sh <black-parrot checkout, submodules initialised>
# Writes designs/blackparrot/src/ (copies, upstream paths; submodules below external/) and
# variants/unicore/{synth,sim}.f.
#
# The lists are upstream's own: bp_top/flist.vcs is what upstream's dc and verilator flows read for
# the design (mk/Makefile.dc, mk/Makefile.verilator), plus the testbench wrapper for the synthesis top;
# bp_top/test/tb/bp_tethered/flist.vcs adds the testbench. $BP_DIR is the checkout.
set -euo pipefail
bp=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
out=$here/src
export LC_ALL=C
tb=bp_top/test/tb/bp_tethered

files() { # paths of a flist, relative to the checkout
  grep '^\$BP_DIR/' "$bp/$1" | sed 's#^\$BP_DIR/##'
}
incdirs() {
  grep '^+incdir+' "$bp/$1" | sed 's#^+incdir+\$BP_DIR/##'
}

mkdir -p "$here/variants/unicore"
{ files bp_top/flist.vcs; echo $tb/wrapper.sv; } | sed 's#^#src/#' > "$here/variants/unicore/synth.f"
# the testbench list; wrapper.sv is in the synthesis list already
files $tb/flist.vcs | grep -v "^$tb/wrapper.sv$" | sed 's#^#src/#' > "$here/variants/unicore/sim.f"
{ incdirs bp_top/flist.vcs; incdirs $tb/flist.vcs; } | sort -u | sed 's#^#src/#' > "$here/variants/unicore/incdirs.txt"

# copy: listed files, the files they include (recipe/includes.py), licences, waiver file, demo program
listed=$( { files bp_top/flist.vcs; files $tb/flist.vcs; echo $tb/wrapper.sv; } | sort -u)
dirs=$( { incdirs bp_top/flist.vcs; incdirs $tb/flist.vcs; } | sort -u)
{
  echo "$listed"
  python3 -I "$here/recipe/includes.py" "$bp" $dirs -- $listed
  echo LICENSE; echo external/basejump_stl/LICENSE; echo external/HardFloat/COPYING.txt
  echo bp_top/verilator/waiver.vlt
  echo $tb/demo.riscv
} | sort -u | while read -r f; do
  mkdir -p "$out/$(dirname "$f")"
  cp -p "$bp/$f" "$out/$f"
done
