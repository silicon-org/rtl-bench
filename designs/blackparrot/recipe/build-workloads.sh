#!/usr/bin/env bash
# Build the workload images of blackparrot.unicore the way black-parrot-sdk builds them, with the
# patches of the SDK applied to copies of its submodules.
# usage: nix develop -c recipe/build-workloads.sh <black-parrot-sdk checkout>
# The checkout needs these submodules (see recipe/README.md for the commits): libperch,
# libgloss-dramfs (with its littlefs), coremark, riscv-tests (with its env).
# Writes sim/rv64ui-p-simple.riscv, sim/rv64ui-p-add.riscv and sim/coremark.riscv.
set -euo pipefail
sdk=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
# a fixed path, because the images embed the build path of the riscv-tests files
w=/tmp/rtl-bench-blackparrot-workloads
rm -rf "$w"
mkdir -p "$w"
I=$w/install/riscv64-unknown-elf
mkdir -p "$I/include" "$I/lib" "$I/bin" "$here/sim"
for d in libperch libgloss-dramfs coremark riscv-tests; do
  rsync -a --exclude .git "$sdk/$d/" "$w/$d/"
done
# riscv-tests/env is a submodule of riscv-tests, kept as a directory by rsync

# 1. riscv-tests with the SDK patches: patches/riscv-tests/*.patch on the top, env/*.patch on env
for p in "$sdk"/patches/riscv-tests/000*.patch; do patch -s -d "$w/riscv-tests" -p1 < "$p"; done
for p in "$sdk"/patches/riscv-tests/env/*.patch; do patch -s -d "$w/riscv-tests/env" -p1 < "$p"; done
mkdir -p "$w/rt-build"
(cd "$w/rt-build" && RISCV_PREFIX=riscv64-unknown-elf- "$w/riscv-tests/configure" --target=riscv64-unknown-elf >/dev/null \
  && RISCV_PREFIX=riscv64-unknown-elf- make -j8 isa >/dev/null 2>&1)
# --strip-debug also drops the FILE symbol, which names a random temporary object of the compiler
for t in rv64ui-p-simple rv64ui-p-add; do
  riscv64-unknown-elf-objcopy --strip-debug "$w/rt-build/isa/$t.riscv" "$here/sim/$t.riscv"
done

# 2. libgloss-dramfs (crt0, specs, libc glue), configured for bare metal rv64gc; the tool names
#    CC, AR, ... that the dev shell sets point at the host compiler, so they are unset
(
  unset CC CXX AR LD NM RANLIB OBJCOPY STRIP SIZE
  mkdir "$w/gl-build" && cd "$w/gl-build"
  "$w/libgloss-dramfs/configure" --prefix="$I" --host=riscv64-unknown-elf >/dev/null
  # the last install step links a tool into the compiler directory, which is read-only here
  make all install >/dev/null 2>&1 || true
)
test -f "$I/lib/dramfs.specs" -a -f "$I/lib/libgloss_dramfs.a"

# 3. libperch, with the headers installed above on the include path (encoding.h)
(
  export WITH_PREFIX=$w/install WITH_TRIPLE=riscv64-unknown-elf WITH_MARCH=rv64gc WITH_MABI=lp64d \
    WITH_DRAM_BASE=0x80000000 WITH_STACK_SIZE=0x01000000 WITH_HEAP_SIZE=0x10000000 WITH_BOOT_HART=0x0
  export CPATH=$I/include
  make -C "$w/libperch" all install >/dev/null
)

# 4. CoreMark with the SDK patch, as mk/Makefile.prog builds it (core_portme.mak), linked with
#    the newlib built for the medany code model and with empty crtbegin.o and crtend.o (flake.nix).
#    FLAGS_STR is set, so the image does not carry the build paths.
patch -s -d "$w/coremark" -p1 < "$sdk/patches/coremark/0001-portme.patch"
(
  cd "$w/coremark/barebones"
  PORT_CC=riscv64-unknown-elf-gcc \
  PORT_CFLAGS="-B$RISCV_EMPTY_CRT -B$I/lib -I$I/include -ffile-prefix-map=$w=." \
  PORT_LD_FLAGS="-L$RISCV_NEWLIB_MEDANY -L$I/lib" \
    make -f core_portme.mak clean coremark.riscv FLAGS_STR=riscv64-unknown-elf-gcc >/dev/null 2>&1
  cp coremark.riscv "$here/sim/coremark.riscv"
)
rm -rf "$w"
