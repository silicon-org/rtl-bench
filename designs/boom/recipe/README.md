# Recipe of boom

How `src/`, `variants/smallv3/`, `licences/` and `sim/` were made. Unlike the other designs, the RTL is not a copy of an upstream file: it is the output of one run of Chipyard's own generation flow, vendored as it came out.

## Generation

`recipe/generate.sh <work dir>` runs, in the nix dev shell (JDK 17, dtc, make, gcc, perl, python from `flake.nix`; Chipyard's conda environment is not used):

| step | tool and version |
|---|---|
| Chipyard | https://github.com/ucb-bar/chipyard, tag `1.15.0`, commit `157dd69e49b9866be2bf8db390ee78bc38cd4438` |
| submodules | `scripts/init-submodules-no-riscv-tools-nolog.sh` (the minimal set); commits below and in the design file |
| elaboration | sbt (`scripts/sbt-launch.jar`), Chisel 6.7.0, Scala 2.13.16 (`build.sbt`, `USE_CHISEL6=1`), JDK 17.0.20 |
| Verilog | CIRCT `firtool-1.75.0` (`conda-reqs/circt.json`), the release binary https://github.com/llvm/circt/releases/download/firtool-1.75.0/firrtl-bin-linux-x64.tar.gz, sha256 `5c822c07268d20cb85f4488a416cd62b983d33c13ffa9a5b5656ce5078d6475c` |
| memories | `tapeout.macros.MacroCompiler --mode synflops` (Chipyard `tools/barstools`) |
| command | `make -C sims/verilator CONFIG=SmallBoomV3Config verilog` |

The configuration is `chipyard.SmallBoomV3Config` = `boom.v3.common.WithNSmallBooms(1) ++ chipyard.config.AbstractConfig`, elaborated on `chipyard.harness.TestHarness`. (The tag also has `SmallBoomV4Config`; V3 is the configuration that was asked for.) JDK 21 does not work: the Scala 2.12 of sbt cannot read its class files.

Generation, measured on the NAS (load below 50): 4 min 5 s wall for the whole script (clone, submodules, sbt and Scala compilation, elaboration, firtool), 754 s CPU, 6.4 GB resident at most (largest process: the JVM). A first run with cold caches took 4 min 43 s, 1020 s CPU and 8.2 GB. Two runs produced identical `gen-collateral` directories.

`recipe/resolve.sh <work dir>` copies from `sims/verilator/generated-src/chipyard.harness.TestHarness.SmallBoomV3Config/gen-collateral/` the `.sv`, `.v`, `.cc` and `.h` files (615 in all) to `src/generated/`, and `tools/DRAMSim2` (without its own main) to `src/DRAMSim2/`, and writes the file lists:

- `variants/smallv3/synth.f`: the 533 modules of `ChipTop` (`top.f`), the black boxes of the DUT (`EICG_wrapper.v`, `GenericDigitalInIOCell.v`, `GenericDigitalOutIOCell.v`, `plusarg_reader.v`) and `top.mems.v`.
- `variants/smallv3/sim.f`: `sim_files.common.f` of the run (the DUT, the TestHarness, TestDriver, the DPI Verilog and the host C++ of testchipip), then the DRAMSim2 sources.

## Synthesis top and memories

`ChipTop`: the chip that Chipyard's VLSI flow (`vlsi/`) synthesises; `TestHarness` is the testbench around it (it holds the simulated memory, the TSI and UART adapters, and the clock and reset). `ChipTop` has the clock inputs `clock_uncore` (the SoC and the core), `jtag_TCK` and `serial_tl_0_clock_in`, and the reset `reset_io`.

The 20 memories of the chip appear as `*_ext` modules (`cc_dir_ext`, `tag_array_ext`, `mem_ext`, ... listed in `.top.mems.conf`). Chipyard's VLSI flow replaces them by SRAM macros of a technology; for simulation its MacroCompiler writes them in `synflops` mode as flop arrays with the ports of a macro. These models (`chipyard...top.mems.v`) are the behavioural models of the `macros` in `design.toml`.

## Licences

The generated Verilog carries source locators (`@[generators/boom/...]`) that name the files it comes from: BOOM (57 k locators), rocket-chip (57 k), the Chisel library (7 k), diplomacy (4 k), rocket-chip-inclusive-cache (4 k), testchipip (2 k), hardfloat (1 k), Chipyard itself and rocket-chip-blocks. The hand-written Verilog and C++ that Chipyard copies next to it (TestDriver, SimDRAM, ...) say "See LICENSE" or "See LICENSE.SiFive". The licence files of these repositories are in `licences/`, read from the checkouts: BSD-3-Clause (UC Berkeley: Chipyard, BOOM, rocket-chip `LICENSE.Berkeley` and `LICENSE.jtag`, hardfloat, testchipip), Apache-2.0 (SiFive files in Chipyard, BOOM and rocket-chip; rocket-chip-blocks, rocket-chip-inclusive-cache, diplomacy, Chisel). DRAMSim2 (compiled into the simulator, not in the RTL) is BSD-2-Clause (the header of its files). The Chisel licence is the `LICENSE` of the Chisel commit `8db8c1e063c22440c6fcbc61ea7e8923a969ad15` (tag `v6.7.0`).

| repository | commit |
|---|---|
| riscv-boom | `ba03175e0a3cf5c0d5a7f14d385ac1c8998c4428` |
| rocket-chip | `ece7b9ad544b07df39cd13b6fd7236562a26355d` |
| berkeley-hardfloat | `0ecaef097ce2accbd16a61613699450ed5533f29` |
| testchipip | `555ff3fbc12f81a8feff7a4801930fb384ddd2ae` |
| rocket-chip-blocks | `f8c7fddbd7639b15fefc968af59fcc4a8f7df73b` |
| rocket-chip-inclusive-cache | `27df9284e60136a6f7f1b19224dfbe99d112ea77` |
| diplomacy | `fe5e131d4fc8adec14a3ce4a4935bb5c0a269871` |
| DRAMSim2 | `44322e2f935d7dac83b7adf8dd270b41a54c6acb` |

## Simulation

Chipyard's Verilator build (`sims/verilator/Makefile`): the command that `make -n` shows is in `design.toml` (the options, the `RANDOMIZE_*` defines and `--vpi --main --timing`), with `--threads 1`. The host libraries `libfesvr` and `libriscv` are those of Spike from nixpkgs (`CY_SPIKE`), not the ones that Chipyard builds from `riscv-isa-sim`. The model takes the program as an argument and loads it over the simulated serial link (about 3 M cycles). The TestDriver prints `*** PASSED *** Completed after N simulation cycles` with `+verbose` (Chipyard's `run-binary` passes it); the simulator ends with `$finish` at success. The RTL uses random initial values (`RANDOMIZE_REG_INIT`); the run has the default seed.

Build, measured: 51 s wall (Verilator 51 s: 15 s conversion, 31 s C++), 266 s CPU, 660 MB resident at most (the C++ compiler processes are smaller); 72 MB of generated C++ in 158 files.

## Workloads

riscv-tests (https://github.com/riscv-software-src/riscv-tests, commit `1eb47d946c55f55cab8653c224c2993acc0276bd`), unmodified: the ISA tests `rv64ui-p-simple` and `rv64ui-p-add`, and the benchmark `dhrystone` (the longest of the standard benchmark set that fits a CI-length run; the run takes 128 s). They are built by `recipe/build-workloads.sh` with the `riscv64-unknown-elf` GCC 16.2.0 of the dev shell (rv64gc, lp64d, the flags of the riscv-tests makefiles) and `objcopy --strip-debug` (drops the random temporary-file symbol); the program uses the HTIF `tohost`. No activity window: Dhrystone has no marker symbol, only the calls to `setStats` that switch its counters.

| image | sha256 |
|---|---|
| `sim/rv64ui-p-simple` | `878432f5e0cf58b5aabbca973768d793185d2fb6122927aab9873a7103d11550` |
| `sim/rv64ui-p-add` | `c157976bccfedc882b7604a3b98667bf1ad883b5eb2a39fd8b0bd8c787776f46` |
| `sim/dhrystone.riscv` | `43fd6e587250ed7d20c46da2fae282264ac89876aab8ac9b2d43a8b1b3f0f33e` |

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, GCC 16.2.0, Spike (libfesvr, libriscv).
