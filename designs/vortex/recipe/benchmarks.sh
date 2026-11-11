#!/usr/bin/env bash
# Build the riscv-tests benchmarks as upstream's tests/riscv/common.mk does, to reproduce the failure
# described in recipe/README.md. The images are not part of the design: they stop on an RTL assertion.
# usage: nix develop -c recipe/benchmarks.sh <riscv-tests checkout at 1eb47d9, submodules initialised> <output dir>
set -euo pipefail
src=$(realpath "$1"); out=$(realpath "$2")
here=$(cd "$(dirname "$0")/.." && pwd)
w=/tmp/rtl-bench-vortex-benchmarks
rm -rf "$w"; mkdir -p "$w" "$out"
rsync -a --exclude .git "$src/" "$w/riscv-tests/"
cd "$w/riscv-tests"
patch -s -p1 < "$here/src/miscs/patches/riscv-benchmarks.patch"
# the benchmarks link the generated VX_types.h symbols (VX_MEM_IO_EXIT_CODE, VX_MEM_IO_COUT_*); the
# include path goes in through RISCV_GCC, as upstream does. XLEN 32: ABI ilp32f, rv32imf_zicsr.
make -C benchmarks XLEN=32 RISCV_PREFIX=riscv32-unknown-elf- src_dir="$w/riscv-tests/benchmarks" \
  ABI=ilp32f RISCV_MARCH=rv32imf_zicsr RISCV_GCC="riscv32-unknown-elf-gcc -mstrict-align -Wl,--no-warn-mismatch -I$here/src/generated/sw" \
  median.riscv qsort.riscv rsort.riscv towers.riscv memcpy.riscv multiply.riscv dhrystone.riscv >/dev/null
for b in median qsort rsort towers memcpy multiply dhrystone; do
  riscv32-unknown-elf-objcopy --strip-debug "benchmarks/$b.riscv" "$out/$b.riscv"
done
rm -rf "$w"
