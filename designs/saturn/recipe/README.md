# Recipe of saturn

How `src/`, `variants/rocketv256d128/`, `licences/` and `sim/` were made. As for gemmini and boom, the RTL is not a copy of an upstream file: it is the output of one run of Chipyard's own generation flow, vendored as it came out.

## Generation

`recipe/generate.sh <work dir>` runs, in the nix dev shell (JDK 17, dtc, make, gcc, cmake, perl, python from `flake.nix`; Chipyard's conda environment is not used):

| step | tool and version |
|---|---|
| Chipyard | https://github.com/ucb-bar/chipyard, tag `1.15.0`, commit `157dd69e49b9866be2bf8db390ee78bc38cd4438` |
| submodules | `scripts/init-submodules-no-riscv-tools-nolog.sh` (the minimal set), then `git submodule update --init generators/saturn`; commits below and in the design file |
| espresso | `generators/constellation/scripts/install-espresso.sh`, the logic minimizer at `v2.4-12-g2cc4778` (commit `2cc477880bd99a40f257155a676fc61fd53e0d9e`), built with cmake as Chipyard's `build-toolchain-extra.sh` does |
| elaboration | sbt (`scripts/sbt-launch.jar`), Chisel 6.7.0, Scala 2.13.16 (`build.sbt`, `USE_CHISEL6=1`), JDK 17.0.20 |
| Verilog | CIRCT `firtool-1.75.0` (`conda-reqs/circt.json`), the release binary https://github.com/llvm/circt/releases/download/firtool-1.75.0/firrtl-bin-linux-x64.tar.gz, sha256 `5c822c07268d20cb85f4488a416cd62b983d33c13ffa9a5b5656ce5078d6475c` |
| memories | `tapeout.macros.MacroCompiler --mode synflops` (Chipyard `tools/barstools`) |
| command | `make -C sims/verilator CONFIG=REFV256D128RocketConfig verilog` |

The configuration is `chipyard.REFV256D128RocketConfig` (`chipyard/SaturnConfigs.scala` of saturn-vectors, compiled into Chipyard when the submodule is present) = `saturn.rocket.WithRocketVectorUnit(256, 128, VectorParams.refParams)` ++ `WithSystemBusWidth(128)` ++ `WithNHugeCores(1)` ++ `chipyard.config.AbstractConfig`, elaborated on `chipyard.harness.TestHarness`. It is one Rocket core with a Saturn vector unit of VLEN 256 and a 128-bit datapath (DLEN) with the reference parameters of Saturn (`refParams`), as the file defines it. Saturn supports the V extension with Zvfh and Zvbb. The config is used as Saturn ships it. JDK 21 does not work: the Scala 2.12 of sbt cannot read its class files.

The decoder of Saturn (`insns/Decode.scala`) uses the decoder of the Chisel library, which calls `espresso` and falls back to Quine-McCluskey when it is not on the `PATH`; the fallback runs out of the Java heap (8 GB, then 16 GB) on this config. The script builds espresso from the submodule of constellation as Chipyard does and puts it on the `PATH`.

Saturn is compiled against `generators/shuttle` (ucb-bar/shuttle, `build.sbt`: `saturn dependsOn(rocketchip, shuttle)`), which the init script fetches. That repository has no licence file. The config here has a Rocket core, and the generated files have no Shuttle code: no source locator names `generators/shuttle` in the Verilog (the locators name rocket-chip, saturn, hardfloat, inclusive-cache, diplomacy, testchipip, chipyard, rocket-chip-blocks and the Chisel library) and no file, module or text of the output contains `shuttle`.

Generation, measured on the NAS (load average 113 at the start), from an empty directory (clone, submodules, espresso, sbt and Scala compilation, elaboration, firtool): 300 s wall, 971 s CPU, 5.8 GB resident at most (largest process: the JVM). Two runs, the second from a fresh clone in another directory, produced identical `gen-collateral` directories (652 files).

`recipe/resolve.sh <work dir>` copies from `sims/verilator/generated-src/chipyard.harness.TestHarness.REFV256D128RocketConfig/gen-collateral/` the `.sv`, `.v`, `.cc` and `.h` files (649 in all) to `src/generated/`, and `tools/DRAMSim2` (without its own main) to `src/DRAMSim2/`, and writes the file lists:

- `variants/rocketv256d128/synth.f`: the modules of `ChipTop` (`top.f`), the black boxes of the DUT (`EICG_wrapper.v`, `GenericDigitalInIOCell.v`, `GenericDigitalOutIOCell.v`, `plusarg_reader.v`) and `top.mems.v`.
- `variants/rocketv256d128/sim.f`: `sim_files.common.f` of the run (the DUT, the TestHarness, TestDriver, the DPI Verilog and the host C++ of testchipip), then the DRAMSim2 sources.

## Synthesis top and memories

`ChipTop`: the chip that Chipyard's VLSI flow (`vlsi/`) synthesises; `TestHarness` is the testbench around it. `ChipTop` has the clock inputs `clock_uncore` (the SoC, the core and the vector unit), `jtag_TCK` and `serial_tl_0_clock_in`, and the reset `reset_io`.

The 7 memories of the chip appear as `*_ext` modules (`.top.mems.conf`): the data and tag arrays of the L1 caches of the core (`rockettile_dcache_data_arrays_0_ext`, `rockettile_dcache_tag_array_ext`, `rockettile_icache_data_arrays_0_ext`, `rockettile_icache_tag_array_ext`), the directory and bank of the inclusive cache (`cc_dir_ext`, `cc_banks_0_ext`) and `mem_ext` (8192 x 64, the `TLRAM_ScratchpadBank` of the SoC). The vector register file of Saturn is built from flops, not from a memory. Chipyard's VLSI flow replaces the memories by SRAM macros of a technology; for simulation its MacroCompiler writes them in `synflops` mode as flop arrays with the ports of a macro. These models (`chipyard...top.mems.v`) are the behavioural models of the `macros` in `design.toml`.

## Licences

The generated Verilog carries source locators (`@[generators/saturn/...]`) that name the files it comes from: rocket-chip, Saturn, hardfloat, rocket-chip-inclusive-cache, diplomacy, testchipip, Chipyard itself, rocket-chip-blocks and the Chisel library. The hand-written Verilog and C++ that Chipyard copies next to it (TestDriver, SimDRAM, ...) say "See LICENSE" or "See LICENSE.SiFive". The licence files of these repositories are in `licences/`, read from the checkouts: BSD-3-Clause (UC Berkeley: Chipyard, Saturn, rocket-chip `LICENSE.Berkeley` and `LICENSE.jtag`, hardfloat, testchipip), Apache-2.0 (SiFive files in Chipyard and rocket-chip; rocket-chip-blocks, rocket-chip-inclusive-cache, diplomacy, Chisel). The Saturn `LICENSE` is the BSD 3-Clause text (copyright 2024, the Regents of the University of California). DRAMSim2 (compiled into the simulator, not in the RTL) is BSD-2-Clause (the header of its files). The Chisel licence is the `LICENSE` of the Chisel commit `8db8c1e063c22440c6fcbc61ea7e8923a969ad15` (tag `v6.7.0`). Shared with gemmini: the commits of rocket-chip, hardfloat, testchipip, rocket-chip-blocks, rocket-chip-inclusive-cache, diplomacy and DRAMSim2 are the same.

| repository | commit |
|---|---|
| saturn-vectors | `8c5115c1900432433c6f68f1039fc482b24413d6` |
| rocket-chip | `ece7b9ad544b07df39cd13b6fd7236562a26355d` |
| berkeley-hardfloat | `0ecaef097ce2accbd16a61613699450ed5533f29` |
| testchipip | `555ff3fbc12f81a8feff7a4801930fb384ddd2ae` |
| rocket-chip-blocks | `f8c7fddbd7639b15fefc968af59fcc4a8f7df73b` |
| rocket-chip-inclusive-cache | `27df9284e60136a6f7f1b19224dfbe99d112ea77` |
| diplomacy | `fe5e131d4fc8adec14a3ce4a4935bb5c0a269871` |
| DRAMSim2 | `44322e2f935d7dac83b7adf8dd270b41a54c6acb` |

## Simulation

Chipyard's Verilator build (`sims/verilator/Makefile`), as for boom and gemmini: the options, the `RANDOMIZE_*` defines and `--vpi --main --timing` are in `design.toml`, with `--threads 1`. The host libraries `libfesvr` and `libriscv` are those of Spike from nixpkgs (`CY_SPIKE`). The model takes the program as an argument and loads it over the simulated serial link. The TestDriver prints `*** PASSED *** Completed after N simulation cycles` with `+verbose`; the simulator ends with `$finish` at success. `+verbose` also prints the commit log of the core, one line per instruction, which limits the simulator to about 13 to 17 k cycles/s. The RTL uses random initial values (`RANDOMIZE_REG_INIT`); the run has the default seed.

Build, measured on the NAS with 16 jobs: 40 s wall, 233 s CPU, 0.70 GB resident at most (a compiler or Verilator process; measured by the first `rtl-bench sim`, together with the 1.8 s run of rv64ui-p-simple).

## Workloads

`recipe/build-workloads.sh` builds, with the `riscv64-unknown-elf` GCC 16.2.0 of the dev shell, the vector benchmarks of saturn-vectors (https://github.com/ucb-bar/saturn-vectors, `benchmarks/`, commit `8c5115c1900432433c6f68f1039fc482b24413d6`, the pin of Chipyard 1.15.0) with upstream's own `benchmarks/Makefile` (`rv64gcv_zfh_zvfh`, `lp64d`, `-mcmodel=medany`, `-O2`, `-ffast-math`; its `common/` runtime and `env` submodule `riscv-test-env` at `1c577dc7c7d6aee27b8d5cb0e2e87c8473e3ad12`): `vec-strlen`, `vec-mixed_width_mask` and `vec-igemm`. The benchmarks come from the Ara suite (ETH Zurich, Apache-2.0) and the riscv-tests benchmarks. `rv64ui-p-simple` is built from riscv-tests at the pin of Chipyard 1.15.0 (https://github.com/riscv-software-src/riscv-tests, commit `0494f954a3d8d2ca9e4972da7a01e94b6a909bce`, with its submodule `env`). `objcopy --strip-debug` follows, as for the other designs. The programs use the HTIF `tohost`. Two builds gave identical images.

Each benchmark checks its own result and returns a nonzero code on a difference, so `*** PASSED ***` means the check held: `vec-strlen` compares the vector `strlen` with the length of the string at several offsets, `vec-mixed_width_mask` compares a masked 8 bit predicate and 32 bit load example with gold data, `vec-igemm` runs int64 matrix products of size 4 to 64 and compares the last with a gold matrix. All three give the same result in Spike (`--isa=rv64gcv_zfh_zvfh_zvl256b`).

| image | sha256 | what it does |
|---|---|---|
| `sim/rv64ui-p-simple` | `938882be980c86cca6ab800d9bf34610688a296c3af225a3bbe0b3d53006d947` | riscv-tests sanity check of the core |
| `sim/vec-strlen` | `793dd31fdc8e2244a654b006f7f666951e0177634c48ad3d0941e1d60be79dab` | vector `strlen` |
| `sim/vec-igemm` | `e2899ea672ae3620603bc743e163d6a627e3bb8032f3ea4941a711cced993005` | int64 matmul up to 64x64 |
| `sim/vec-mixed_width_mask` | `6dbe2f79540b9649c6b0cac3e557938ac7f392cd996607047bcb7faed088caf1` | int 7x7 convolution of a 112x112 image |

Measured on the NAS (Verilator 5.052, one thread; peak memory is that of `rtl-bench sim`, which keeps the output):

| workload | cycles | run time | peak memory |
|---|---|---|---|
| rv64ui-p-simple | 30696 | 1.8 s | 0.70 GB (with the build) |
| vec-strlen | 666786 | 41 s | 40 MB |
| vec-mixed_width_mask | 1041576 | 61 s | 40 MB |
| vec-igemm | 8123636 | 619 s | 80 MB |

Left out: `vec-sgemm` (float matmul of upstream's benchmarks) ended with `*** FAILED ***` in the RTL simulation (the exact comparison of the 71x71 float result with the gold data failed after 4.2 M cycles) and passes in Spike; it is not in the variant. `vec-iconv2d` and `vec-pathfinder` ran for more than 20 minutes or have no check of the result, and are not in it either.

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, GCC 16.2.0, Spike (libfesvr, libriscv), cmake (for espresso).
