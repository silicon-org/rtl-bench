#!/usr/bin/env bash
# Copy the generated sources of naxriscv.rv64 from the work directory of recipe/generate.sh, the C++ testbench of
# NaxRiscv and the licence files, and write the file lists.
# usage: nix develop -c recipe/resolve.sh <work dir>
set -euo pipefail
w=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
nax=$w/nax
out=$here/src
rm -rf "$out"; mkdir -p "$out/generated" "$out/testbench" "$here/variants/rv64"
cp -p "$nax/NaxRiscv.v" "$nax/nax.h" "$out/generated/"
# the testbench of NaxRiscv (src/test/cpp/naxriscv/src), as the Makefile there builds it; it includes nax.h
cp -p "$nax"/src/test/cpp/naxriscv/src/*.cpp "$nax"/src/test/cpp/naxriscv/src/*.h "$out/testbench/"
echo src/generated/NaxRiscv.v > "$here/variants/rv64/synth.f"
{ echo src/testbench/main.cpp; } > "$here/variants/rv64/sim.f"
# the licence files of the repositories whose code is in the output or in the simulator (read from the checkouts, the
# last three from their repositories at the commits named in design.toml)
lic=$here/licences
rm -rf "$lic"; mkdir -p "$lic"/{naxriscv,spinalhdl,riscv-isa-sim,elfio,riscv-tests,naxsoftware}
cp -p "$nax/LICENSES/MIT.txt" "$lic/naxriscv/MIT.txt"
cp -p "$nax/ext/SpinalHDL/LICENSE_lib" "$nax/ext/SpinalHDL/LICENSE_core" "$lic/spinalhdl/"
cp -p "$nax/ext/riscv-isa-sim/LICENSE" "$lic/riscv-isa-sim/LICENSE"
curl -fsSL https://raw.githubusercontent.com/serge1/ELFIO/d251da09a07dff40af0b63b8f6c8ae71d2d1938d/LICENSE.txt -o "$lic/elfio/LICENSE.txt"
curl -fsSL https://raw.githubusercontent.com/riscv-software-src/riscv-tests/c84daca8824635b7d896003c78f9c6245997cf7a/LICENSE -o "$lic/riscv-tests/LICENSE"
# CoreMark in NaxSoftware carries the Apache-2.0 notice in its headers; its repository has the licence file
curl -fsSL https://raw.githubusercontent.com/eembc/coremark/1f483d5b8316753a742cbf5590caf5bd0a4e4777/LICENSE.md -o "$lic/naxsoftware/COREMARK-LICENSE.md"
