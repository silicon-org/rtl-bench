#!/usr/bin/env bash
# Generate the Verilog of NaxRiscv (RV64IMAFDCSU) with its own generator, naxriscv.Gen64.
# usage: nix develop -c recipe/generate.sh <work dir, outside the repository>
#
# What it runs, and with which tools (the dev shell provides sbt, which runs on its own JDK 21.0.13):
#   NaxRiscv, commit 9f452d50560d02fb391bc8039f5453c54e0911af (the head of main when the design was added; the
#     repository has no release tags), submodule ext/SpinalHDL at c362657003fa178350d6955055867e9ab363c44a
#     (SpinalHDL "dev", v1.7.0-2132 on the way to 1.8; the library of NaxRiscv is the submodule, not a release),
#   sbt with sbt.version 1.9.9 on the command line instead of the 1.6.0 of project/build.properties (the Scala 2.12
#     of sbt 1.6.0 cannot read the class files of this JDK), Scala 2.12.18 (build.sbt),
#   command: sbt "runMain naxriscv.Gen64 --withRvc --withFloat --withDouble"
# which is the rv64imafdcsu target of the Makefile of NaxRiscv (TARGET_NAX default, `make NaxRiscv.v`): a dual
# issue core with RVC, F and D, supervisor and user mode and the Sv39 MMU, which runs Linux. The generator writes
# NaxRiscv.v (the core for simulation: the top module NaxRiscv with the memories as behavioural arrays), nax.h
# (the parameters the testbench is built with) and NaxRiscvSynt.v (the same modules with the name NaxRiscvSynt, an
# FPGA benchmark wrapper with registered I/O; it duplicates the module names of NaxRiscv.v and is not used).
set -euo pipefail
w=$(realpath -m "$1")
mkdir -p "$w"
cd "$w"
[ -d nax ] || git clone -q https://github.com/SpinalHDL/NaxRiscv nax
git -C nax checkout -q 9f452d50560d02fb391bc8039f5453c54e0911af
# the submodules that the generator and the testbench need: SpinalHDL (the library), rvls (build.sbt compiles its JNI
# bindings into NaxRiscv, though Gen64 does not use them), riscv-isa-sim (the Spike of the testbench, built by flake.nix),
# NaxSoftware (the baremetal programs of the workloads)
git -C nax submodule update --init -q ext/SpinalHDL ext/riscv-isa-sim ext/NaxSoftware ext/rvls
test "$(git -C nax/ext/SpinalHDL rev-parse HEAD)" = c362657003fa178350d6955055867e9ab363c44a
test "$(git -C nax/ext/riscv-isa-sim rev-parse HEAD)" = 5b37ab699f6176ea7f2d2b3187f75625eca7e511
test "$(git -C nax/ext/NaxSoftware rev-parse HEAD)" = 865b8c1465cb24c78423d16fd6dd74d12f643bda
test "$(git -C nax/ext/rvls rev-parse HEAD)" = db0e5acbc941e4bef416c151dc19c47574a80f3b
sbtopts=(-Dsbt.version=1.9.9 "-Dsbt.global.base=$w/sbt-global" "-Dsbt.boot.directory=$w/sbt-boot" "-Dsbt.ivy.home=$w/ivy" "-Dsbt.coursier.home=$w/coursier")
# build.sbt loads the compiler plugin of SpinalHDL from the jar of its idslplugin project
(cd nax/ext/SpinalHDL && sbt "${sbtopts[@]}" idslplugin/package)
rm -f nax/NaxRiscv.v nax/NaxRiscvSynt.v nax/nax.h
(cd nax && sbt "${sbtopts[@]}" "runMain naxriscv.Gen64 --withRvc --withFloat --withDouble")
