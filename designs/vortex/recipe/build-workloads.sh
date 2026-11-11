#!/usr/bin/env bash
# Build the workload images of vortex.default as upstream's tests/riscv/common.mk builds them:
# the riscv-tests ISA tests unmodified except for the place of the HTIF `tohost` word, and the
# riscv-tests benchmarks with miscs/patches/riscv-benchmarks.patch.
# usage: nix develop -c recipe/build-workloads.sh <riscv-tests checkout at 1eb47d9, submodules initialised>
# Writes sim/rv32ui-p-simple and sim/rv32ui-p-add.
set -euo pipefail
src=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
# a fixed path, because the images may embed the build path
w=/tmp/rtl-bench-vortex-workloads
rm -rf "$w"; mkdir -p "$w" "$here/sim"
rsync -a --exclude .git "$src/" "$w/riscv-tests/"
cd "$w/riscv-tests"
# route the console output and the exit of the benchmarks through Vortex MMIO instead of HTIF
git_apply() { patch -s -p1 < "$here/src/miscs/patches/riscv-benchmarks.patch"; }
git_apply
# the HTIF `tohost` word goes to VX_MEM_IO_EXIT_CODE (uncached IO), so that its store reaches the host monitor
TOHOST=$(sed -n 's/.*define VX_MEM_IO_EXIT_CODE \([0-9]*\).*/\1/p' "$here/src/generated/sw/VX_types.h")
sed -i -e '/^[[:space:]]*\.tohost : { \*(\.tohost) }/d' \
       -e "/^[[:space:]]*_end = \.;/a\\  .tohost $TOHOST : { *(.tohost) }" env/p/link.ld
make -C isa XLEN=32 RISCV_PREFIX=riscv32-unknown-elf- rv32ui-p-simple rv32ui-p-add >/dev/null
# --strip-debug also drops the FILE symbol, which names a random temporary object of the compiler
for t in rv32ui-p-simple rv32ui-p-add; do
  riscv32-unknown-elf-objcopy --strip-debug "isa/$t" "$here/sim/$t"
done
rm -rf "$w"
