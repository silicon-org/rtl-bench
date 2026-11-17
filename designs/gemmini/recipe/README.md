# Recipe of gemmini

How `src/`, `variants/rocket16x16/`, `licences/` and `sim/` were made. As for boom, the RTL is not a copy of an upstream file: it is the output of one run of Chipyard's own generation flow, vendored as it came out.

## Generation

`recipe/generate.sh <work dir>` runs, in the nix dev shell (JDK 17, dtc, make, gcc, perl, python from `flake.nix`; Chipyard's conda environment is not used):

| step | tool and version |
|---|---|
| Chipyard | https://github.com/ucb-bar/chipyard, tag `1.15.0`, commit `157dd69e49b9866be2bf8db390ee78bc38cd4438` |
| submodules | `scripts/init-submodules-no-riscv-tools-nolog.sh` (the minimal set), then `git submodule update --init generators/gemmini`; commits below and in the design file |
| elaboration | sbt (`scripts/sbt-launch.jar`), Chisel 6.7.0, Scala 2.13.16 (`build.sbt`, `USE_CHISEL6=1`), JDK 17.0.20 |
| Verilog | CIRCT `firtool-1.75.0` (`conda-reqs/circt.json`), the release binary https://github.com/llvm/circt/releases/download/firtool-1.75.0/firrtl-bin-linux-x64.tar.gz, sha256 `5c822c07268d20cb85f4488a416cd62b983d33c13ffa9a5b5656ce5078d6475c` |
| memories | `tapeout.macros.MacroCompiler --mode synflops` (Chipyard `tools/barstools`) |
| command | `make -C sims/verilator CONFIG=GemminiRocketConfig verilog` |

The configuration is `chipyard.GemminiRocketConfig` (`generators/gemmini/chipyard/GemminiConfigs.scala`, compiled into Chipyard when the submodule is present) = `gemmini.DefaultGemminiConfig` ++ `WithNHugeCores(1)` ++ `WithSystemBusWidth(128)` ++ `chipyard.config.AbstractConfig`, elaborated on `chipyard.harness.TestHarness`. `DefaultGemminiConfig` takes `GemminiConfigs.defaultConfig`: a 16x16 mesh (`meshRows` = `meshColumns` = 16, tile 1x1) of int8 processing elements with 32 bit accumulation, both dataflows, a 256 KiB scratchpad in 4 banks and a 64 KiB accumulator in 2 banks. The config is used as Gemmini ships it. JDK 21 does not work: the Scala 2.12 of sbt cannot read its class files.

The `--gemmini` option of the init script is not used: it also fetches the submodules of `gemmini-rocc-tests`, one of which (`ibm/rocc-software`) no longer exists on GitHub. They are software, not part of the RTL, so the script initialises `generators/gemmini` alone.

Generation, measured on the NAS (load 30 to 50), from an empty directory (clone, submodules, sbt and Scala compilation, elaboration, firtool): 4 min 17 s wall, 832 s CPU, 7.5 GB resident at most (largest process: the JVM). Two runs, the second from a fresh clone in another directory, produced identical `gen-collateral` directories and an identical `gemmini_params.h`.

`recipe/resolve.sh <work dir>` copies from `sims/verilator/generated-src/chipyard.harness.TestHarness.GemminiRocketConfig/gen-collateral/` the `.sv`, `.v`, `.cc` and `.h` files (649 in all) to `src/generated/`, the parameter header to `src/software/gemmini_params.h`, and `tools/DRAMSim2` (without its own main) to `src/DRAMSim2/`, and writes the file lists:

- `variants/rocket16x16/synth.f`: the modules of `ChipTop` (`top.f`), the black boxes of the DUT (`EICG_wrapper.v`, `GenericDigitalInIOCell.v`, `GenericDigitalOutIOCell.v`, `plusarg_reader.v`) and `top.mems.v`.
- `variants/rocket16x16/sim.f`: `sim_files.common.f` of the run (the DUT, the TestHarness, TestDriver, the DPI Verilog and the host C++ of testchipip), then the DRAMSim2 sources.

`gemmini_params.h` is written by the Gemmini generator (`Controller.scala`, `GemminiArrayConfig.generateHeader`) next to the RTL, into `generators/gemmini/software/gemmini-rocc-tests/include/` when that directory exists, otherwise into the Chipyard root, as here. It holds the parameters of the array for the C tests. It is byte-identical to the `include/gemmini_params.h` of gemmini-rocc-tests.

## Synthesis top and memories

`ChipTop`: the chip that Chipyard's VLSI flow (`vlsi/`) synthesises; `TestHarness` is the testbench around it. `ChipTop` has the clock inputs `clock_uncore` (the SoC, the core and Gemmini), `jtag_TCK` and `serial_tl_0_clock_in`, and the reset `reset_io`.

The 9 memories of the chip appear as `*_ext` modules (`.top.mems.conf`): the scratchpad of Gemmini (`mem_ext`, 4096 x 128, one instance per bank) and its accumulator (`mem_0_ext`, 512 x 512 with write mask, one per bank), `mem_1_ext` (8192 x 64, the `TLRAM_ScratchpadBank` of the SoC), the data and tag arrays of the L1 caches of the core, and the directory and bank of the inclusive cache. Chipyard's VLSI flow replaces them by SRAM macros of a technology; for simulation its MacroCompiler writes them in `synflops` mode as flop arrays with the ports of a macro. These models (`chipyard...top.mems.v`) are the behavioural models of the `macros` in `design.toml`.

## Licences

The generated Verilog carries source locators (`@[generators/gemmini/...]`) that name the files it comes from: rocket-chip (73 k locators), Gemmini (47 k), the Chisel library (13 k), diplomacy (5 k), rocket-chip-inclusive-cache (5 k), testchipip (2 k), hardfloat (2 k), Chipyard itself and rocket-chip-blocks. BOOM is not in the output. The hand-written Verilog and C++ that Chipyard copies next to it (TestDriver, SimDRAM, ...) say "See LICENSE" or "See LICENSE.SiFive". The licence files of these repositories are in `licences/`, read from the checkouts: BSD-3-Clause (UC Berkeley: Chipyard, Gemmini, rocket-chip `LICENSE.Berkeley` and `LICENSE.jtag`, hardfloat, testchipip), Apache-2.0 (SiFive files in Chipyard and rocket-chip; rocket-chip-blocks, rocket-chip-inclusive-cache, diplomacy, Chisel). The Gemmini `LICENSE` is the Berkeley three-clause text (copyright 2018-2019, the Regents of the University of California). DRAMSim2 (compiled into the simulator, not in the RTL) is BSD-2-Clause (the header of its files). The Chisel licence is the `LICENSE` of the Chisel commit `8db8c1e063c22440c6fcbc61ea7e8923a969ad15` (tag `v6.7.0`).

| repository | commit |
|---|---|
| gemmini | `8c3f9923a44a2fe2c7930587be297d6d4f8c09ca` |
| rocket-chip | `ece7b9ad544b07df39cd13b6fd7236562a26355d` |
| berkeley-hardfloat | `0ecaef097ce2accbd16a61613699450ed5533f29` |
| testchipip | `555ff3fbc12f81a8feff7a4801930fb384ddd2ae` |
| rocket-chip-blocks | `f8c7fddbd7639b15fefc968af59fcc4a8f7df73b` |
| rocket-chip-inclusive-cache | `27df9284e60136a6f7f1b19224dfbe99d112ea77` |
| diplomacy | `fe5e131d4fc8adec14a3ce4a4935bb5c0a269871` |
| DRAMSim2 | `44322e2f935d7dac83b7adf8dd270b41a54c6acb` |

## Simulation

Chipyard's Verilator build (`sims/verilator/Makefile`), as for boom: the options, the `RANDOMIZE_*` defines and `--vpi --main --timing` are in `design.toml`, with `--threads 1`. The host libraries `libfesvr` and `libriscv` are those of Spike from nixpkgs (`CY_SPIKE`). The model takes the program as an argument and loads it over the simulated serial link. The TestDriver prints `*** PASSED *** Completed after N simulation cycles` with `+verbose`; the simulator ends with `$finish` at success. `+verbose` also prints the commit log of the core, one line per instruction, which limits the simulator to about 17 k cycles/s and makes the output large (780 MB for conv). The RTL uses random initial values (`RANDOMIZE_REG_INIT`); the run has the default seed.

Build, measured on the NAS with 16 jobs: 39 s wall, 209 s CPU, 0.83 GB resident at most (a compiler or Verilator process).

## Workloads

`recipe/build-workloads.sh` builds the bare-metal tests of gemmini-rocc-tests (https://github.com/ucb-bar/gemmini-rocc-tests, commit `7c540b3adf1b86ad93d07f893abe3a73489b568e`, the pin of the submodule at the Gemmini commit above) with its own `build.sh` (`make BAREMETAL_ONLY=1 bareMetalC`): `template`, `tiled_matmul_ws` and `conv`. The tests use `include/gemmini_params.h`, which the script checks against the header of the generator run. The submodules of the test repository are riscv-tests (commit `c84daca8824635b7d896003c78f9c6245997cf7a`, its `env` and `benchmarks/common`) and rocc-software. The pinned rocc-software commit (`fddb795a0b52e82f8f4ce9ead9b1428440a62ab0`, https://github.com/ibm/rocc-software) is no longer on GitHub; https://github.com/ucb-bar/rocc-software, commit `9492c0abd0ca04abb9cddf3737ba68232326fdae` ("restore rocc-software sources from https://github.com/ibm/rocc-software"), has its two headers (`xcustom.h`, `riscv_test_rocc.h`) and is used instead. `rv64ui-p-simple` is built from the same riscv-tests checkout. The compiler is the `riscv64-unknown-elf` GCC 16.2.0 of the dev shell (rv64gc, medany, the flags of the makefile of the tests) and `objcopy --strip-debug`; the programs use the HTIF `tohost`. Two builds gave identical images.

| image | sha256 | what it does |
|---|---|---|
| `sim/rv64ui-p-simple` | `2e97ca445d6d911657b32fd19d2b757966fd74ebf4008b892eac59fd381bc652` | riscv-tests sanity check of the core |
| `sim/template` | `6a991165348cffe9f29c3525a2d1b2f5bc7f975927c2c6ccf6f4cc15cb5984e9` | one 16x16 matrix times the identity on Gemmini |
| `sim/tiled_matmul_ws` | `aa958a1d1a5f1fb3b38a491e96d11590e4ddf607c8b8c3692e86994f607fdb2a` | 64x64x64 matmul, weight stationary, checked on the core |
| `sim/conv` | `676a0346d35c9e0a7fed9b88a1b9a166e63645dffa7f8c0fa19ed1d778701499` | a convolution, checked on the core |

Measured on the NAS (Verilator 5.052, one thread; peak memory is that of `rtl-bench sim`, which keeps the output; the simulator itself allocates 19 MB):

| workload | cycles | run time | peak memory |
|---|---|---|---|
| rv64ui-p-simple | 30696 | 1.8 s | 0.83 GB (with the build) |
| template | 652306 | 39 s | 45 MB |
| tiled_matmul_ws | 10723836 | 673 s | 0.68 GB |
| conv | 18776526 | 1170 s | 1.5 GB |

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, GCC 16.2.0, Spike (libfesvr, libriscv), autoconf (for the build of the tests).
