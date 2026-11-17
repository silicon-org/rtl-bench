# Recipe of naxriscv

How `src/`, `variants/rv64/`, `licences/` and `sim/` were made. The RTL is not a copy of an upstream file: it is the output of one run of the generator of NaxRiscv, vendored as it came out.

## Generation

`recipe/generate.sh <work dir>` runs, in the nix dev shell:

| step | tool and version |
|---|---|
| NaxRiscv | https://github.com/SpinalHDL/NaxRiscv, commit `9f452d50560d02fb391bc8039f5453c54e0911af` (head of main, 2026-01-23; the repository has no release tags) |
| SpinalHDL | the submodule `ext/SpinalHDL`, commit `c362657003fa178350d6955055867e9ab363c44a` (`dev`, 2132 commits after v1.7.0); the library of NaxRiscv is this checkout, not a release |
| other submodules | `ext/rvls` (`db0e5acbc941e4bef416c151dc19c47574a80f3b`; `build.sbt` compiles its JNI bindings into NaxRiscv), `ext/riscv-isa-sim` (`5b37ab699f6176ea7f2d2b3187f75625eca7e511`), `ext/NaxSoftware` (`865b8c1465cb24c78423d16fd6dd74d12f643bda`) |
| build | sbt 1.9.9 (`-Dsbt.version=1.9.9`; `project/build.properties` says 1.6.0, whose Scala 2.12 cannot read the class files of the JDK), Scala 2.12.18 (`build.sbt`); the nixpkgs sbt runs on its own JDK 21.0.13 |
| command | `sbt "runMain naxriscv.Gen64 --withRvc --withFloat --withDouble"` |

`naxriscv.Gen64` with these options is the `rv64imafdcsu` target of the `Makefile` of NaxRiscv (`TARGET_NAX`, default): RV64IMAFDC with machine, supervisor and user mode, Sv39, two decoders and two ALUs, 16 KiB 4-way instruction and data caches, a BTB and a gshare predictor, 64 ROB entries. Linux and Buildroot run on it upstream. The generator writes three files into the repository root:

- `NaxRiscv.v` (66 k lines, 37 modules, one flat Verilog-2005 file): the core for simulation. Its memories are arrays inside the modules, so there are no memory files and no separate memory modules.
- `nax.h`: the parameters (XLEN, queue sizes, `RVF`, `RVD`, ...) that the testbench is compiled with.
- `NaxRiscvSynt.v`: the same modules with the top `NaxRiscvSynt`, a wrapper with registered inputs and outputs for FPGA benchmarks. It has the module names of `NaxRiscv.v`, so a design cannot read both; it is not vendored.

Three runs (the first in the dev checkout, two more from fresh clones in other directories, with fresh sbt caches) gave identical `NaxRiscv.v`, `nax.h` and `NaxRiscvSynt.v`. A run from an empty directory (clone, submodules, sbt and Scala compilation, elaboration) took 1 min 33 s of wall time and 291 s of CPU on the NAS.

`recipe/resolve.sh <work dir>` copies `NaxRiscv.v` and `nax.h` to `src/generated/`, the C++ testbench of NaxRiscv (`src/test/cpp/naxriscv/src`: `main.cpp` and its headers) to `src/testbench/`, the licence files to `licences/`, and writes the file lists.

## Synthesis top

`NaxRiscv`, the module `Gen64` writes: the clock `clk`, the synchronous active-high `reset`, and the memory interfaces of the instruction cache, the data cache (a read and a write channel) and the peripheral bus, plus the interrupt inputs. No constraint file and no published frequency for an open PDK exist, so the variant has no period (upstream reports a frequency on a Xilinx Artix 7).

## Licences

NaxRiscv is MIT (`LICENSES/MIT.txt`; the files carry `SPDX-License-Identifier: MIT`). The output Verilog has the SpinalHDL library in it (`StreamFifo`, `StreamArbiter`, ...), which is MIT (`LICENSE_lib`); the SpinalHDL core, the generator, is LGPL-3.0 (`LICENSE_core`) and none of its code is in the output. The simulator links the SpinalHDL fork of Spike (BSD-3-Clause) and includes ELFIO (MIT), and the workloads come from riscv-tests (BSD-3-Clause) and NaxSoftware (dhrystone from riscv-tests, coremark by EEMBC under Apache-2.0).

## Simulation

The testbench is `main.cpp` of NaxRiscv, built as its `Makefile` builds it (`make compile` in `src/test/cpp/naxriscv`): Verilator with `--exe`, the C++ main, the Spike libraries (`libspike_main`, `libriscv`, `libdisasm`, `libsoftfloat`, `libfesvr`, `libfdt`, `package.so`), SDL2 and zlib. `flake.nix` provides them:

- Verilator 4.228, the version the Makefile pins (`VERILATOR_VERSION_NAX`); `main.cpp` reaches into the model as Verilator 4 lays it out (`top->NaxRiscv`), so Verilator 5 does not compile it. The variant selects it with `verilator = "${NAX_VERILATOR4}/bin/verilator"`.
- The Spike of NaxRiscv, the SpinalHDL fork at the commit of the submodule, configured and built as `ci/install-libsdl-elfio-spikespinalhdl.sh` does (`--without-boost`, then `package.so`), with the GCC of the dev shell.
- ELFIO at the commit of `ELFIO_VERSION`, and SDL2 and zlib from nixpkgs.

Left out of the Makefile's flags: `TRACE=yes` (FST waves), `-g` and `--gdbbt`.

The testbench runs a Spike model in lock step and compares every commit. A run prints `SUCCESS` when the program reaches the pass symbol (and `FAILURE` otherwise); the exit code is 0 in both cases, so the pass check reads the output. `--stats-print` prints `cycles N`.

Build, measured on the NAS with 16 jobs: 13 s of wall time, 78 s of CPU, 0.45 GB resident at most (the largest process).

## Workloads

`recipe/build-workloads.sh` builds, with the riscv64-unknown-elf GCC 16.2.0 of the dev shell: `rv64ui-p-add`, `rv64ui-p-sd`, `rv64um-p-mul` and `rv64ua-p-amoadd_d` from riscv-tests (commit `c84daca8824635b7d896003c78f9c6245997cf7a`, the commit that the BOOM and Gemmini designs use), and `dhrystone` and `coremark` from NaxSoftware (the makefiles of `baremetal/`, arch `rv64imafdc`, `lp64d`, `-mcmodel=medany`; coremark with `ITERATIONS=10`). GCC 16 compiles C as C23, which the K&R code of both programs does not accept: they build with `-std=gnu17`. The images are stripped of debug information only (`objcopy --strip-debug`). Two builds gave identical images. `rv64ui-p-simple` is not used: it passes by writing `tohost` (HTIF), which this testbench does not serve.
