#!/usr/bin/env bash
# Build the workload images of naxriscv.rv64: four tests of riscv-tests (add, sd, mul, amoadd_d, from the lists of its regression), and the dhrystone and
# coremark programs of NaxSoftware (the baremetal programs that the regression of NaxRiscv runs, with its own makefiles).
# usage: nix develop -c recipe/build-workloads.sh
# Writes sim/rv64ui-p-add, sim/rv64ui-p-sd, sim/rv64um-p-mul, sim/rv64ua-p-amoadd_d, sim/dhrystone and sim/coremark.
# rv64ui-p-simple is not built: it passes by writing tohost (HTIF), which the testbench does not serve; the other tests end at
# the symbols pass and fail, which the regression of NaxRiscv uses.
# The sources are fetched at the commits below. The compiler is the riscv64-unknown-elf GCC of the dev shell, not the
# toolchain that the Makefile of NaxRiscv builds (RISCV_VERSION 409b951). The march is rv64imafdc with the Zicsr and
# Zifencei that GCC 16 names separately (the only multilib of the nix toolchain is its default, rv64gc with lp64d),
# the arch of the rv64imafdc build of NaxSoftware. GCC 16 compiles C as C23 and the sources of dhrystone
# and coremark are K&R C: they build with -std=gnu17. The newlib is the medany one of the dev shell
# (RISCV_NEWLIB_MEDANY), put first on the link line with -L.
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
w=/tmp/rtl-bench-naxriscv-workloads   # a fixed path, because the objects of NaxSoftware are built under their absolute path
rm -rf "$w"; mkdir -p "$w" "$here/sim"
cd "$w"
# NaxSoftware, the submodule ext/NaxSoftware of NaxRiscv 9f452d5
git clone -q https://github.com/SpinalHDL/NaxSoftware.git ns
git -C ns checkout -q 865b8c1465cb24c78423d16fd6dd74d12f643bda
# riscv-tests and its submodule env, the commit that the Gemmini and BOOM designs use
git clone -q https://github.com/riscv/riscv-tests rt
git -C rt checkout -q c84daca8824635b7d896003c78f9c6245997cf7a
git -C rt submodule update -q --init --recursive

arch=rv64imafdc_zicsr_zifencei
cc="riscv64-unknown-elf-gcc -std=gnu17 -L$RISCV_NEWLIB_MEDANY"

tests="rv64ui-p-add rv64ui-p-sd rv64um-p-mul rv64ua-p-amoadd_d"
make -C rt/isa XLEN=64 RISCV_PREFIX=riscv64-unknown-elf- $tests >/dev/null
for t in $tests; do
  # --strip-debug also drops the FILE symbol, which names a random temporary object of the compiler
  riscv64-unknown-elf-objcopy --strip-debug "rt/isa/$t" "$here/sim/$t"
done

# dhrystone: baremetal/dhrystone/makefile, target compile of ../common/asm.mk (what all_arch runs per arch)
make -C ns/baremetal/dhrystone compile OBJDIR="$w/dhrystone" MARCH=$arch MABI=lp64d \
  RISCV_CC="$cc" RISCV_OBJCOPY=riscv64-unknown-elf-objcopy RISCV_OBJDUMP=riscv64-unknown-elf-objdump >/dev/null
riscv64-unknown-elf-objcopy --strip-debug dhrystone/dhrystone.elf "$here/sim/dhrystone"

# coremark: the command of LIST_ARCH in baremetal/coremark/naxriscv_sim/core_portme.mak, with ITERATIONS=10
make -C ns/baremetal/coremark PORT_DIR=naxriscv_sim RVM=yes ITERATIONS=10 REBUILD=1 XCFLAGS="-DCORE_DEBUG=0" \
  MARCH=$arch MABI=lp64d CC="$cc" clean compile link >/dev/null
riscv64-unknown-elf-objcopy --strip-debug ns/baremetal/coremark/coremark.elf "$here/sim/coremark"
rm -rf "$w"
