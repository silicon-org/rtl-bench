#!/usr/bin/env bash
# Build the workloads of bsg-manycore with upstream's own makefiles (software/spmd/<program>/Makefile, software/mk) for
# each machine, and keep the network boot files (main.nbf, the file that the testbench loads) in sim/<variant>/.
# usage: nix develop .#bsg -c recipe/build-workloads.sh
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
# a fixed path: the build directory is in the debug information of the ELF files, and the nbf files are made from them
w=/tmp/rtl-bench-bsg-manycore-workloads
rm -rf "$w"
mkdir -p "$w"
# upstream expects basejump_stl next to bsg_manycore (software/riscv-tools/Makefile: BSG_IP_CORES_DIR)
cp -r "$here/src/bsg_manycore" "$here/src/basejump_stl" "$w/"
# BSG_MANYCORE_DIR: the makefile finds it with /usr/bin/perl otherwise, which NixOS does not have.
# RISCV_BIN_DIR: riscv32-unknown-elf-dramfs-* of flake.nix (GCC 16 for rv32imaf and ilp32f).
# IGNORE_CADENV: no VCS (software/mk/Makefile.verilog).
export BSG_MANYCORE_DIR=$w/bsg_manycore RISCV_BIN_DIR=$BSG_RISCV_BIN BSG_PLATFORM=verilator IGNORE_CADENV=1

# build <machine> <program> [all]: "all" puts the tile group on every tile of the pod (bsg_tiles_X and bsg_tiles_Y of the
# machine, as software/mk/Makefile.dimensions defaults them); without it the Makefile of the program sets the group.
# ARCH_OP: the same rv32imaf as upstream, with zicsr and zifencei because binutils 2.46 needs them for the CSR
# instructions of crt.S.
build() {
  local machine=$1 prog=$2 group=${3:-} vid dir args=()
  vid=$(echo "$machine" | tr 'A-Z' 'a-z')
  dir=$BSG_MANYCORE_DIR/software/spmd/$prog
  if [ "$group" = all ]; then
    args=(bsg_tiles_X="$(sed -n 's/^BSG_MACHINE_GLOBAL_X *= *//p' "$BSG_MANYCORE_DIR/machines/$machine/Makefile.machine.include")"
          bsg_tiles_Y="$(sed -n 's/^BSG_MACHINE_GLOBAL_Y *= *//p' "$BSG_MANYCORE_DIR/machines/$machine/Makefile.machine.include")")
  fi
  make -C "$dir" -s main.nbf \
    BSG_MACHINE_PATH="$BSG_MANYCORE_DIR/machines/$machine" \
    ARCH_OP=rv32imaf_zicsr_zifencei "${args[@]}" >"$w/$prog.$machine.log" 2>&1 \
    || { tail -30 "$w/$prog.$machine.log"; exit 1; }
  mkdir -p "$here/sim/$vid"
  cp "$dir/main.nbf" "$here/sim/$vid/$prog.nbf"
  make -C "$dir" -s clean >/dev/null 2>&1
}

for machine in pod_1x1_4X2Y pod_1x1; do
  build "$machine" hello
  build "$machine" bsg_mutex all
  build "$machine" bsg_barrier_amoadd_test all
  build "$machine" vcache_atomic_inc all
done
rm -rf "$w"
