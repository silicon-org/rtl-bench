#!/usr/bin/env bash
# Build the firmware of the workloads with upstream's own Makefile (tools/scripts/Makefile, target
# program.hex) and keep the ELF files in sim/.
# usage: nix develop -c designs/caliptra/recipe/build-workloads.sh <test> ...
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
export CALIPTRA_ROOT=$here/src
mkdir -p "$here/sim"
for t in "$@"; do
  work=$(mktemp -d)
  trap 'rm -r "$work"' EXIT
  # GCC_PREFIX: the 32 bit tools of the nix shell (the Makefile defaults to riscv64-unknown-elf).
  # ABI: the value that the Makefile picks for gcc 12.2 and later; its version test does not read
  #      the version line of the nix tools.
  # TEST_LIBS: the newlib of the nix shell for rv32imc and ilp32 (the nixpkgs one is for ilp32d), as the
  #   multilib newlib of upstream's toolchain; the firmware uses memcpy, strlen, strstr and rand.
  # PLAYBOOK_RANDOM_SEED: the seed of the test yml (1), instead of the date.
  # BUILD_CFLAGS: no build paths in the debug information; -std=gnu17, the default of the gcc 12
  #   that upstream documents (gcc 15 and later default to C23, where `void f();` means `void f(void)`
  #   and the declarations of the libraries no longer match their definitions).
  make -C "$work" -f "$CALIPTRA_ROOT/tools/scripts/Makefile" TESTNAME="$t" \
    GCC_PREFIX=riscv32-unknown-elf \
    ABI="-mabi=ilp32 -march=rv32imc_zicsr_zifencei" \
    PLAYBOOK_RANDOM_SEED=1 \
    BUILD_CFLAGS="-O2 -std=gnu17 -ffile-prefix-map=$CALIPTRA_ROOT=src -ffile-prefix-map=$work=." \
    TEST_LIBS="-L$RISCV_NEWLIB_RV32IMC" \
    program.hex
  cp "$work/$t.exe" "$here/sim/$t.exe"
  rm -r "$work"
  trap - EXIT
done
