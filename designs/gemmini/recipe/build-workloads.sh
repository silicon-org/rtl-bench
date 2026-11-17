#!/usr/bin/env bash
# Build the workload images of gemmini.rocket16x16: the bare-metal tests of gemmini-rocc-tests (bareMetalC),
# with upstream's own build.sh, and rv64ui-p-simple of riscv-tests, for the HTIF environment that Chipyard's
# simulator (fesvr) serves.
# usage: nix develop -c recipe/build-workloads.sh
# Writes sim/template, sim/tiled_matmul_ws, sim/conv and sim/rv64ui-p-simple.
# The sources are fetched at the commits below (the pins of gemmini-rocc-tests at the Gemmini commit of the
# design). The parameter header of the tests (include/gemmini_params.h) must be the one that the generator
# wrote with the RTL (src/software/gemmini_params.h); the script checks that they are identical.
# The compiler is the riscv64-unknown-elf GCC of the dev shell, not the Chipyard toolchain build; the flags
# are those of the Makefrag of bareMetalC (rv64gc, medany).
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
w=/tmp/rtl-bench-gemmini-workloads   # a fixed path, because the images may embed it
rm -rf "$w"; mkdir -p "$w" "$here/sim"
cd "$w"
# gemmini-rocc-tests, submodule generators/gemmini/software/gemmini-rocc-tests of Gemmini 8c3f992
git clone -q https://github.com/ucb-bar/gemmini-rocc-tests rt
git -C rt checkout -q 7c540b3adf1b86ad93d07f893abe3a73489b568e
# riscv-tests, its submodule riscv-tests (gitlink c84daca) with its own submodule env
git clone -q https://github.com/riscv/riscv-tests rt/riscv-tests
git -C rt/riscv-tests checkout -q c84daca8824635b7d896003c78f9c6245997cf7a
git -C rt/riscv-tests submodule update -q --init --recursive
# rocc-software, its submodule rocc-software: gitlink fddb795, whose repository (ibm/rocc-software) is
# gone; ucb-bar/rocc-software has restored its files (xcustom.h, riscv_test_rocc.h)
git clone -q https://github.com/ucb-bar/rocc-software rt/rocc-software
git -C rt/rocc-software checkout -q 9492c0abd0ca04abb9cddf3737ba68232326fdae
cmp "$here/src/software/gemmini_params.h" rt/include/gemmini_params.h
cd rt
./build.sh bareMetalC >build.log 2>&1
for t in template tiled_matmul_ws conv; do
  riscv64-unknown-elf-objcopy --strip-debug "build/bareMetalC/$t-baremetal" "$here/sim/$t"
done
make -C riscv-tests/isa XLEN=64 RISCV_PREFIX=riscv64-unknown-elf- rv64ui-p-simple >/dev/null
# --strip-debug also drops the FILE symbol, which names a random temporary object of the compiler
riscv64-unknown-elf-objcopy --strip-debug riscv-tests/isa/rv64ui-p-simple "$here/sim/rv64ui-p-simple"
rm -rf "$w"
