#!/usr/bin/env bash
# Build the workload images of wally.rv64gc from the vendored sources.
# usage: nix develop -c recipe/build-workloads.sh
# The three commands per program are those of upstream tests/coverage64gc/Makefile.
# The only addition is --debug-prefix-map, so that the image does not carry the build path.
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
cd "$here/src/tests/coverage64gc"
mkdir -p "$here/sim"
for t in ieu fpu; do
  riscv64-unknown-elf-gcc -E -g -o "$t.s" "$t.S"
  riscv64-unknown-elf-as -g --debug-prefix-map "$PWD=." -o "$t.o" \
    -march=rv64gqc_zcb_zfa_zba_zbb_zbc_zbs_zfh_zicboz_zicbop_zicbom_zbkb_zbkx_zknd_zkne_zknh_svinval -mabi=lp64 "$t.s"
  riscv64-unknown-elf-gcc -g -o "$t.elf" -mcmodel=medany -nostartfiles -T../../examples/link/link.ld "$t.o"
  mv "$t.elf" "$here/sim/$t.elf"
  rm -f "$t.s" "$t.o"
done
