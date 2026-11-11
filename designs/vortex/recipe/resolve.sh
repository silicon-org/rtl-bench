#!/usr/bin/env bash
# Resolve the file lists of vortex.default and copy the files from an upstream checkout.
# usage: nix develop -c recipe/resolve.sh <vortex checkout>
# Writes designs/vortex/src/ (copies, upstream paths, and the generated headers in src/generated/),
# variants/default/{synth,sim}.f and recipe/unused.txt.
set -euo pipefail
vx=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
out=$here/src
export LC_ALL=C

# 1. The generated headers: what ./configure does for the default configuration (VX_config.toml,
#    XLEN 32): hw/*.vh for the RTL and sw/*.h for the C++ (configure, "generate header files").
mkdir -p "$out/generated/hw" "$out/generated/sw"
for n in VX_config VX_types; do
  extra=; [ $n = VX_types ] && extra=--resolved
  XLEN=32 python3 "$vx/ci/gen_config.py" --config "$vx/$n.toml" --output "$out/generated/hw/$n.vh" --format verilog $extra
  XLEN=32 python3 "$vx/ci/gen_config.py" --config "$vx/$n.toml" --output "$out/generated/sw/$n.h" --format cpp $extra
done

# 2. The synthesis list: upstream's ASIC flow (hw/syn/yosys/Makefile, hw/scripts/gen_sources.sh) takes every
#    .v and .sv file that is directly in one of its include directories. With the default configuration these
#    are RTL_DIR and its libs, interfaces, core, mem, vm, cache and fpu directories, plus hw/syn/libs/no_mem
#    (the RAM stubs of the ASIC flow). The extensions are off, so their directories are not included.
dirs="hw/rtl hw/rtl/libs hw/rtl/interfaces hw/rtl/core hw/rtl/mem hw/rtl/vm hw/rtl/cache hw/rtl/fpu hw/syn/libs/no_mem"
all=$( for d in $dirs; do (cd "$vx" && find "$d" -maxdepth 1 -type f \( -name '*.v' -o -name '*.sv' \)); done | sort)
is_pkg() { grep -qE '^[[:space:]]*package[[:space:]]+[A-Za-z_]' "$vx/$1"; }
pkgs=$(for f in $all; do if is_pkg "$f"; then echo "$f"; fi; done)
rest=$(for f in $all; do if ! is_pkg "$f"; then echo "$f"; fi; done)
mkdir -p "$here/variants/default"
{ echo "$pkgs"; echo "$rest"; } | sed 's#^#src/#' > "$here/variants/default/synth.f"

# 3. The simulation list: the host program of sim/rtlsim (what its Makefile compiles) and the shim top.
cat > "$here/variants/default/sim.f" <<LIST
src/sim/rtlsim/rtlsim_shim.sv
src/sw/common/util.cpp
src/sim/common/mem.cpp
src/sw/common/softfloat_ext.cpp
src/sw/common/rvfloats.cpp
src/sim/common/dram_sim.cpp
src/sim/common/elf_loader.cpp
src/sim/common/host_monitor.cpp
src/sim/common/cout_drainer.cpp
src/hw/dpi/dpi_util.cpp
src/hw/dpi/dpi_float.cpp
src/sim/rtlsim/processor.cpp
src/sim/rtlsim/main.cpp
LIST

# 4. Copy: the listed files, the headers of the include directories, the headers of the host program (the
#    ones that its compilation reads), the generator and its inputs, the licence and the benchmark patch.
{
  echo "$all"
  for d in $dirs; do (cd "$vx" && find "$d" -maxdepth 1 -type f \( -name '*.vh' -o -name '*.svh' \)); done
  sed 's#^src/##' "$here/variants/default/sim.f"
  echo hw/dpi/dpi_util.vh; echo hw/dpi/dpi_float.vh
  echo sim/common/cout_drainer.h; echo sim/common/dram_bank_model.h; echo sim/common/dram_sim.h
  echo sim/common/elf_loader.h; echo sim/common/host_monitor.h; echo sim/common/mem.h
  echo sim/rtlsim/processor.h
  echo sw/common/bitmanip.h; echo sw/common/dram_timing.h; echo sw/common/nan_box.h
  echo sw/common/rvfloats.h; echo sw/common/softfloat_ext.h; echo sw/common/util.h; echo sw/common/vm_types.h
  echo ci/gen_config.py; echo VX_config.toml; echo VX_types.toml
  echo LICENSE; echo miscs/patches/riscv-benchmarks.patch
} | sort -u | while read -r f; do
  mkdir -p "$out/$(dirname "$f")"
  cp -p "$vx/$f" "$out/$f"
done
