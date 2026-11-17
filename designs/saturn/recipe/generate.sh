#!/usr/bin/env bash
# Generate the Verilog of chipyard.REFV256D128RocketConfig with Chipyard's own flow.
# usage: nix develop -c recipe/generate.sh <work dir, outside the repository>
#
# What it runs, and with which tools (the dev shell provides JDK 17, dtc, make, gcc, perl and python):
#   Chipyard 1.15.0 (tag, commit 157dd69e49b9866be2bf8db390ee78bc38cd4438), submodules from
#     scripts/init-submodules-no-riscv-tools-nolog.sh (the minimal set: no RISC-V tools), then generators/saturn alone (not recursive: the
#     script option --saturn would also fetch the submodules of saturn-vectors, which are software; the Shuttle
#     generators/shuttle is in the default set and saturn compiles against it, but the config below has a Rocket core),
#   espresso (from the submodule of constellation, built with cmake as scripts/build-toolchain-extra.sh does),
#   sbt (the sbt-launch.jar in scripts/), Chisel 6.7.0 and Scala 2.13.16 (build.sbt, USE_CHISEL6=1),
#   CIRCT firtool-1.75.0 (conda-reqs/circt.json), the binary release below,
#   command: make -C sims/verilator CONFIG=REFV256D128RocketConfig verilog
# which elaborates chipyard.harness.TestHarness with chipyard.REFV256D128RocketConfig
# (WithRocketVectorUnit(256, 128, VectorParams.refParams) ++ WithSystemBusWidth(128) ++ WithNHugeCores(1) ++ AbstractConfig), runs firtool and the MacroCompiler (--mode synflops), and
# writes sims/verilator/generated-src/chipyard.harness.TestHarness.REFV256D128RocketConfig/.
# The conda environment of Chipyard is not used: its packages (JDK, sbt, dtc, verilator, gcc) come from nix.
set -euo pipefail
w=$(realpath -m "$1")
mkdir -p "$w"
cd "$w"
[ -d cy ] || git clone -q --branch 1.15.0 --depth 1 https://github.com/ucb-bar/chipyard cy
test "$(git -C cy rev-parse HEAD)" = 157dd69e49b9866be2bf8db390ee78bc38cd4438
if [ ! -x tools/firtool-1.75.0/bin/firtool ]; then
  mkdir -p tools
  curl -fsSL -o tools/firrtl-bin-linux-x64.tar.gz \
    https://github.com/llvm/circt/releases/download/firtool-1.75.0/firrtl-bin-linux-x64.tar.gz
  echo "5c822c07268d20cb85f4488a416cd62b983d33c13ffa9a5b5656ce5078d6475c  tools/firrtl-bin-linux-x64.tar.gz" | sha256sum -c -
  tar xzf tools/firrtl-bin-linux-x64.tar.gz -C tools
fi
export PATH="$w/tools/firtool-1.75.0/bin:$PATH"
(cd cy && ./scripts/init-submodules-no-riscv-tools-nolog.sh && git submodule update --init generators/saturn)
# The espresso logic minimizer (generators/constellation/espresso, v2.4-12-g2cc4778), built as Chipyard does
# (generators/constellation/scripts/install-espresso.sh, called by scripts/build-toolchain-extra.sh): the Chisel
# decoder of Saturn (21 input bits) needs it, its fallback (Quine-McCluskey) runs out of memory.
[ -x tools/espresso/bin/espresso ] || (cd cy/generators/constellation && bash scripts/install-espresso.sh "$w/tools/espresso") >espresso.log 2>&1
export PATH="$w/tools/espresso/bin:$PATH"
{ firtool --version | grep -i "firtool"; java -version 2>&1 | head -1; } | tee versions.txt
git -C cy submodule status | sed 's/^[ +-]//' | tee submodules.txt >/dev/null
make -C cy/sims/verilator CONFIG=REFV256D128RocketConfig verilog
