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

# CoreMark: upstream benchmarks/coremark/Makefile, target $(elf), with CONFIG=rv64gc XLEN=64
# ARCH=rv64gc_zba_zbb_zbc_zbs ABI=lp64 ITERATIONS=10 (the example of benchmarks/coremark/README.md) and
# -DWALLY_HPM. build.sh is run through bash, because its shebang is /bin/bash. The extra
# -ffile-prefix-map keeps the build path out of the image.
out=$(mktemp -d)
WALLY="$here/src" OUT="$out" bash "$here/src/benchmarks/coremark/build.sh" rv64gc "$here/src/config/rv64gc/act" \
  rv64gc_zba_zbb_zbc_zbs lp64 10 -DWALLY_HPM "-ffile-prefix-map=$here/src=src"
mv "$out/coremark.elf" "$here/sim/coremark.elf"
rm -rf "$out"
