#!/usr/bin/env bash
# Build the workload images of boom.smallv3: riscv-tests, unmodified, for the HTIF environment that
# Chipyard's simulator (fesvr) serves.
# usage: nix develop -c recipe/build-workloads.sh <riscv-tests checkout at 1eb47d9, submodules initialised>
# Writes sim/rv64ui-p-simple, sim/rv64ui-p-add and sim/dhrystone.riscv.
# The compiler is the riscv64-unknown-elf GCC of the dev shell (rv64gc, lp64d, the flags of the
# riscv-tests makefiles), not the Chipyard toolchain build.
set -euo pipefail
src=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
w=/tmp/rtl-bench-boom-workloads   # a fixed path, because the images may embed it
rm -rf "$w"; mkdir -p "$w" "$here/sim"
rsync -a --exclude .git "$src/" "$w/riscv-tests/"
cd "$w/riscv-tests"
make -C isa XLEN=64 RISCV_PREFIX=riscv64-unknown-elf- rv64ui-p-simple rv64ui-p-add >/dev/null
make -C benchmarks XLEN=64 RISCV_PREFIX=riscv64-unknown-elf- dhrystone.riscv >/dev/null
# --strip-debug also drops the FILE symbol, which names a random temporary object of the compiler
for t in rv64ui-p-simple rv64ui-p-add; do
  riscv64-unknown-elf-objcopy --strip-debug "isa/$t" "$here/sim/$t"
done
riscv64-unknown-elf-objcopy --strip-debug benchmarks/dhrystone.riscv "$here/sim/dhrystone.riscv"
rm -rf "$w"
