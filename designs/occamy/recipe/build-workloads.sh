#!/usr/bin/env bash
# Build the host program of the simulation: hello_world of target/sim/sw/host/apps, with upstream's Makefiles
# and the RISC-V GCC of the nix shell (riscv64-unknown-elf-gcc, the aliases of flake.nix).
# usage: nix develop -c recipe/build-workloads.sh
# The sources are the copies under src/: the tracked software tree of Occamy plus the platform headers that the
# generator wrote for the variant (src/generated/<variant>/target/sim/sw/shared/platform/generated, which
# upstream's Makefile writes into the same directory as its tracked headers). The headers hold the
# configuration of the variant, and the ELF differs between the variants (start.S and host.c use constants of
# the headers), so each variant has its own image: sim/hello_world.<variant>.elf.
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
export LC_ALL=C
w=$(mktemp -d)
trap 'rm -rf "$w"' EXIT
bin=$(dirname "$(command -v riscv64-unknown-elf-gcc)")
for v in full q2c2; do
  mkdir -p "$w/$v/sw"
  cp -a "$here/src/occamy/target/sim/sw/host" "$here/src/occamy/target/sim/sw/shared" "$w/$v/sw/"
  cp -a "$here/src/generated/$v/target/sim/sw/shared/platform/generated/." "$w/$v/sw/shared/platform/generated/"
  make -s -C "$w/$v/sw/host/apps/hello_world" RISCV_GCC_BINROOT="$bin" finalize-build >/dev/null 2>"$w/$v.err" \
    || { cat "$w/$v.err" >&2; exit 1; }
done
mkdir -p "$here/sim"
# The link of upstream's Makefile (one gcc call for the C and the assembler file) leaves the random name of a
# temporary object file in the symbol table as a FILE symbol. strip-debug drops the FILE symbols and keeps the
# symbols that fesvr needs (tohost, fromhost), so the image is the same in every build.
for v in full q2c2; do
  riscv64-unknown-elf-objcopy --strip-debug "$w/$v/sw/host/apps/hello_world/build/hello_world.elf" "$here/sim/hello_world.$v.elf"
done
cd "$here" && sha256sum sim/hello_world.*.elf
