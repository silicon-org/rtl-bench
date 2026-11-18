#!/usr/bin/env bash
# Generate the Verilog of XiangShan (MinimalConfig) with its own Makefile flow.
# usage: nix develop -c otfhs -c recipe/generate.sh <work dir, outside the repository>
# (otfhs, the FHS shell of the dev shell, gives the /bin/bash that scripts/gen_sep_mem.sh names)
#
# What it runs, and with which tools (JAVA_HOME of the dev shell: JDK 17.0.20; make, gcc and python):
#   XiangShan v3.2.2-alpha (commit 0303f76a84fd705d32f6f0434c63c55e2ff02186, the newest tag) and the submodules that
#     the commit pins (listed below, checked by commit),
#   mill 0.11.8 (.mill-version of the tag), the assembly from the release page, checked by sha256,
#   Chisel 6.5.0 and Scala 2.13.14 (build.sc of the tag), fetched by mill,
#   CIRCT firtool-1.62.0, the version that Chisel 6.5.0 asks for (firtool-resolver downloads it when it is not on the
#     PATH; here the release binary is checked by sha256 and put on the PATH, and the tool prints the same version),
#   commands: make verilog CONFIG=MinimalConfig        (the synthesis view, XSTop, with the release arguments of the Makefile)
#             make sim-verilog CONFIG=MinimalConfig    (the simulation view, SimTop, with the difftest arguments)
# Both write build/rtl, so they run one after the other, and the results are kept in $w/out/synth and $w/out/sim.
set -euo pipefail
w=$(realpath -m "$1")
mkdir -p "$w/tools/bin"
cd "$w"
commit=0303f76a84fd705d32f6f0434c63c55e2ff02186
[ -d xs ] || git clone -q https://github.com/OpenXiangShan/XiangShan xs
git -C xs checkout -q "$commit"
test "$(git -C xs rev-parse HEAD)" = "$commit"
(cd xs && git submodule update --init -q rocket-chip difftest huancun fudian utility yunsuan coupledL2 openLLC src/main/resources/aia ready-to-run \
  && git -C rocket-chip submodule update --init -q cde hardfloat)
# the commits of the submodules that the build and the simulator use (the nested ones that the build does not use are not fetched)
while read -r path rev; do
  test "$(git -C "xs/$path" rev-parse HEAD)" = "$rev" || { echo "$path is not at $rev" >&2; exit 1; }
done <<'EOF'
rocket-chip 2c117a41bb187add61f92a018be5f2e5ab044a41
rocket-chip/cde 52768c97a27b254c0cc0ac9401feb55b29e18c28
rocket-chip/hardfloat d93aa570806013dea479a92ba9bb33d1f2d4f69f
difftest e3cd34c3492ce0c2dde0078ecce966d98d79c2e4
huancun 3fc7e7e0c2127c601b2a7d180f49845421a86d8d
coupledL2 024b7261b9ddcde8782ad6fd3f222745f29f7432
openLLC 0bf044830921ab3aa9c55152e247f0ea14f827ab
utility 051d07961ce1679891b2ea7f1ea2f19c0a00a3fd
yunsuan f568aa01568e48dced707721af761288ca51b5de
fudian e1bd4695ca7beb36a5ce7357e9527ad9e95b9ec1
src/main/resources/aia 9464aee3c77b021a037f28aa1b3fe53c71b516f6
ready-to-run c9c1451937e61eefe5adfcf6e197d0ee7ca188f0
EOF
if [ ! -x tools/bin/mill ]; then
  curl -fsSL -o tools/bin/mill https://github.com/com-lihaoyi/mill/releases/download/0.11.8/0.11.8-assembly
  echo "1ce4537b1233af16d68dc1ab4b9f49a996e8c460b393cc0a7f778a328606aab2  tools/bin/mill" | sha256sum -c -
  chmod +x tools/bin/mill
fi
if [ ! -x tools/firtool-1.62.0/bin/firtool ]; then
  curl -fsSL -o tools/firtool.tgz https://github.com/llvm/circt/releases/download/firtool-1.62.0/firrtl-bin-linux-x64.tar.gz
  echo "1aee0d13fea52b504b0f689f4d890cfb7c63f7dc832308ceff9ae3f099e8e800  tools/firtool.tgz" | sha256sum -c -
  tar xzf tools/firtool.tgz -C tools
fi
export PATH="$w/tools/bin:$w/tools/firtool-1.62.0/bin:$PATH"
export COURSIER_CACHE="$w/coursier"
export NOOP_HOME="$w/xs"
{ firtool --version | grep -i "CIRCT firtool"; "$JAVA_HOME/bin/java" -version 2>&1 | head -1; mill --version | head -1; } | tee versions.txt
cd xs
rm -rf build "$w/out"; mkdir -p "$w/out"
make verilog CONFIG=MinimalConfig
cp -r build "$w/out/synth"
rm -rf build
make sim-verilog CONFIG=MinimalConfig
cp -r build "$w/out/sim"
