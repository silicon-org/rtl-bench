#!/usr/bin/env bash
# Build the software images of opentitan.earlgrey with OpenTitan's own Bazel build.
# usage: nix develop -c recipe/build-workloads.sh <opentitan checkout at the pinned commit, a git repository>
# Writes the flash images sim/<test>_sim_verilator.64.vmem and the ELF files next to them.
#
# The tests are the opentitan_test targets of sw/device/tests with the sim_verilator environment. The
# opentitan_test rule would also build the simulation model that runs it (//hw:verilator), so
# recipe/gen_bins.py writes plain opentitan_binary targets with the same sources, dependencies and the
# environment //hw/top_earlgrey:sim_verilator_base, appended to the BUILD file of a copy of the
# checkout. Bazel runs inside a FHS environment (flake.nix: otfhs) with the library path of the
# distribution: its toolchains are dynamically linked programs, its actions use /bin/bash and srec_cat.
# Bazel is the version that .bazelversion names, started by bazelisk.sh; it downloads the toolchains
# and dependencies that MODULE.bazel pins (hermetic, the lowRISC rv32imcb clang among them).
set -euo pipefail
ot=$(realpath "$1")
here=$(cd "$(dirname "$0")/.." && pwd)
tests="uart_smoketest crt_test otbn_smoketest"
# a fixed path, because the build output may embed it
w=/tmp/rtl-bench-opentitan-sw
rm -rf "$w"
cp -a "$ot" "$w"
cd "$w"
rm -f bazel-bin bazel-out bazel-testlogs bazel-ot
git checkout -q -- .
python3 "$here/recipe/gen_bins.py" sw/device/tests/BUILD $tests >> sw/device/tests/BUILD
targets=$(for t in $tests; do printf '//sw/device/tests:rtlbench_%s ' "$t"; done)
otfhs -c "export LD_LIBRARY_PATH=/lib:/usr/lib; ./bazelisk.sh shutdown >/dev/null 2>&1 || true; \
  ./bazelisk.sh build --jobs=8 --spawn_strategy=local --action_env=LD_LIBRARY_PATH=/lib:/usr/lib \
    --noshow_progress $targets"
mkdir -p "$here/sim"
for t in $tests; do
  for ext in 64.vmem elf; do
    cp -L "bazel-bin/sw/device/tests/rtlbench_${t}_sim_verilator.$ext" "$here/sim/${t}_sim_verilator.$ext"
  done
done
otfhs -c "cd $w && ./bazelisk.sh shutdown >/dev/null 2>&1 || true"
rm -rf "$w"
