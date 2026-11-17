#!/usr/bin/env bash
# Build the workload images of saturn.rocketv256d128: the vector benchmarks of saturn-vectors (benchmarks/),
# with upstream's own Makefile, and rv64ui-p-simple of riscv-tests, for the HTIF environment that Chipyard's
# simulator (fesvr) serves.
# usage: nix develop -c recipe/build-workloads.sh
# Writes sim/rv64ui-p-simple, sim/vec-strlen, sim/vec-mixed_width_mask and sim/vec-igemm.
# The compiler is the riscv64-unknown-elf GCC of the dev shell, not the Chipyard toolchain build; the flags
# are those of benchmarks/Makefile (rv64gcv_zfh_zvfh, lp64d, medany, -O2) and `objcopy --strip-debug`.
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
w=/tmp/rtl-bench-saturn-workloads   # a fixed path, because the images may embed it
rm -rf "$w"; mkdir -p "$w" "$here/sim"
cd "$w"
# saturn-vectors at the commit of the submodule generators/saturn of Chipyard 1.15.0; only its benchmarks
# are needed, with their submodule benchmarks/env (riscv-test-env, the pin of saturn-vectors)
git clone -q https://github.com/ucb-bar/saturn-vectors sv
git -C sv checkout -q 8c5115c1900432433c6f68f1039fc482b24413d6
git -C sv submodule update -q --init benchmarks/env
test "$(git -C sv/benchmarks/env rev-parse HEAD)" = 1c577dc7c7d6aee27b8d5cb0e2e87c8473e3ad12
make -C sv/benchmarks XLEN=64 RISCV_PREFIX=riscv64-unknown-elf- vec-strlen.riscv vec-mixed_width_mask.riscv vec-igemm.riscv >build.log 2>&1
for t in vec-strlen vec-mixed_width_mask vec-igemm; do
  riscv64-unknown-elf-objcopy --strip-debug "sv/benchmarks/$t.riscv" "$here/sim/$t"
done
# riscv-tests at the pin of Chipyard 1.15.0 (toolchains/riscv-tools/riscv-tests), with its submodule env
git clone -q https://github.com/riscv-software-src/riscv-tests rt
git -C rt checkout -q 0494f954a3d8d2ca9e4972da7a01e94b6a909bce
git -C rt submodule update -q --init --recursive
make -C rt/isa XLEN=64 RISCV_PREFIX=riscv64-unknown-elf- rv64ui-p-simple >/dev/null
# --strip-debug also drops the FILE symbol, which names a random temporary object of the compiler
riscv64-unknown-elf-objcopy --strip-debug rt/isa/rv64ui-p-simple "$here/sim/rv64ui-p-simple"
rm -rf "$w"
