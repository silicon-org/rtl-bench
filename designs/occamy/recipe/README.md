# Recipe of occamy

How `src/`, `variants/`, `sim/` and `licences/` were made. Nothing under `src/` is edited: every file is a copy from the upstream commits below, at a path that names its repository, or an output of upstream's own generator.

Run everything inside `nix develop`:

```
recipe/fetch-licences.sh
recipe/generate.sh <workdir> full
recipe/generate.sh <workdir> q2c2
recipe/resolve.sh <workdir>
recipe/build-workloads.sh
```

`generate.sh` needs network access (git clone, `bender checkout`). `resolve.sh` writes `src/`, `variants/` and `recipe/unused.txt` (the last one needs `design.toml`, so it is a second run of the script after the first).

## Upstream

| repository | commit | note |
|---|---|---|
| https://github.com/pulp-platform/occamy | `d5d0d8320bb633c36319e5477006d4c17cf0e448` | `main`, 2024-09-02. Its generator, templates and configurations. |
| https://github.com/pulp-platform/snitch_cluster | `da57b043dfe0ba563a55a9d83bf362873c713648` | The revision of `Bender.lock`. `Bender.yml` names the branch `occamy`, which no longer exists upstream; this commit is reachable by hash only. The lock file, not `Bender.yml`, was followed. |

Every other dependency is at the revision of `Bender.lock` (`bender checkout` does not change the lock file; `generate.sh` checks it). Three trees are in `hw/vendor` of Occamy and come with it: `cva6` (pulp-platform/cva6 `6a6b8c656b8bb789d50b9c84cce1b99c74966884`), `axi_tlb` (`82730842f5037fef9754dfd1212c7711289e5439`) and `opentitan_peripherals` (`e38ebcc6dbfe30e03b7548f5acb87a61d0fc17bd`), the revisions of their `*.lock.hjson`. The commits of the 19 repositories of the lock are in `design.toml`.

## Licences

`hw/` of Occamy is Solderpad Hardware License 0.51, `util/` and `sw/` are Apache-2.0. The dependencies are Solderpad 0.51 as well, with Apache-2.0 for parts of `cvfpu`, `riscv-dbg` (SiFive), `snitch_cluster` (outside `hw/`), `axi_tlb` and `opentitan_peripherals`. Nine files of CVA6 carry `SPDX-License-Identifier: Apache-2.0 WITH SHL-2.0`. `apb_uart` has no licence file; its headers name the GNU Lesser General Public License (LGPL-2.1-or-later). The vendored CVA6 tree has no licence file either.

`recipe/fetch-licences.sh` fetches the texts into `licences/`:

- `SHL-0.51.txt`, `SHL-2.0.txt`: https://solderpad.org/licenses/ (the text of the page);
- `LGPL-2.1.txt`: https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt;
- `cva6/LICENSE`: the `LICENSE` of pulp-platform/cva6 at `6a6b8c656b8bb789d50b9c84cce1b99c74966884` (Solderpad 0.51).

The sha256 of each file is in `recipe/licences.sha256`, and the script fails when one differs. `rtl-bench check` recognises the SHL-2.0 and LGPL-2.1 texts.

## Tools

The generator output depends on these versions. They come from the nix shell (`flake.nix`). verible changes the bytes of the generated files.

| tool | version | note |
|---|---|---|
| bender | 0.28.2 | Occamy pins none; its lock file is of the 0.28 series. The bender of nixpkgs (0.32.1) crashes at start. |
| verible | v0.0-4148-g1ea007ec | `verible-verilog-format --inplace` on every generated `.sv` (the Makefile of Occamy). |
| clang-format | 21.1.8 | formats the generated C headers. |
| Python | 3.12.16 | |
| hjson, Mako, jsonref, jsonschema, PyYAML, tabulate | 3.0.2, 1.4.1, 1.1.0, 4.26.0, 6.0.3, 0.10.0 | `python-requirements.txt` of snitch_cluster lists them without versions. |
| setuptools | 80.9.0 | `regtool.py` imports `pkg_resources`, which newer releases do not have. |
| GNU make | 4.4.1 | |
| Verilator | 5.052 | |
| slang | 12.1.0 | |

`generate.sh` writes the versions it used to `<workdir>/<variant>/tools.txt`; `resolve.sh` copies them to `variants/<variant>/tools.txt`.

## Generation

`generate.sh <workdir> <variant>` clones Occamy at the commit, runs `bender checkout`, and runs `make -C target/sim rtl`, the rule of upstream that writes the register tops of CLINT, PLIC, SoC control, HBM crossbar control and quadrant control, the SoC (`occamy_top`, `occamy_soc`, `occamy_pkg`, `occamy_quadrant_s1`, `occamy_quadrant_s1_ctrl`, `occamy_cva6`, `occamy_cluster_wrapper`, the FPGA top and `occamy_memories.json`) and `testharness.sv`. It also runs the rule for the C headers of the platform (`PLATFORM_HEADERS`), which the host program includes. The generated files are not tracked by upstream.

Variants:

- `full`: `target/sim/cfg/full.hjson`, unmodified (6 quadrants of 4 clusters of 9 cores).
- `q2c2`: our configuration, `recipe/q2c2.hjson`. It is `full.hjson` with `nr_s1_quadrant = 2` and `s1_quadrant.nr_clusters = 2` (`recipe/q2c2.diff`, made with fixed labels; `generate.sh` writes the diff the same way and compares). It is the smallest variant with more than one quadrant and more than one cluster per quadrant, made for hierarchical flow work. `narrow_xbar_user_width` stays 5 as in `full.hjson` (it is wide enough for fewer clusters).

Generated lines of `occamy_soc.sv`: 2946 (full), 2162 (q2c2). The quadrant is a module of its own (`occamy_quadrant_s1`, instantiated once per quadrant).

### Determinism

Two generations from scratch (`generate.sh` into two work directories, both variants) and `resolve.sh` of each into an empty design directory: `diff -r` of the two results is empty, and so is `diff -r` of the generated files themselves (`target/sim/src`, `testharness.sv`, the platform headers).

## File lists

`resolve.sh` runs `bender script flist-plus` in the generated tree and copies what it lists. Paths: `src/deps/<repository>/...` for the dependencies, `src/occamy/...` for the tracked files of Occamy (with its `hw/vendor` trees), `src/generated/<variant>/...` for the generated files (at the path they have in the checkout). The headers of the include directories, the licence files at the top of each repository, the host software (`target/sim/sw`), the configurations and the testbench directory (`target/sim/test`) are copied too.

| view | targets | file |
|---|---|---|
| synthesis | `rtl synthesis cv64a6_imafdc_sv39 occamy_sim snitch_cluster` | `variants/<v>/synth.f` |
| simulation | `rtl verilator cv64a6_imafdc_sv39 occamy_sim snitch_cluster tech_cells_generic_exclude_deprecated` | `variants/<v>/sim.f` (plus the C++ files) |

`cv64a6_imafdc_sv39`, `occamy_sim` and `snitch_cluster` are the bender targets of upstream's simulation Makefile (`target/sim/Makefile`). `synthesis` and no `simulation` target gives the generic `tech_cells_generic` (behavioural `tc_sram`, `tc_sram_impl`, `tc_clk`) and the latch based `scm`. The simulation list has no `simulation` target, because that adds the class based verification libraries (`axi_test`, `apb_test`, `spm_test`, ...), which Verilator cannot read and the testbench does not use; the testbench files (`testharness`, `tb_memory_*`, `uartdpi`) are listed for `verilator`. `tech_cells_generic_exclude_deprecated` is the switch of `tech_cells_generic` for its `src/deprecated` cells; one of them has Verilog 1995 `pmos` gates, which Verilator does not support, and nothing of Occamy uses them.

`*-defines.txt` and `*-incdirs.txt` are the defines and include directories of the lists. `bender` 0.28.2 writes the target defines of several `-t` as one broken define (`TARGET_ RTL -T SYNTHESIS ...`); the script writes `TARGET_<target>` for each instead. The define `TARGET_FLIST` is the script format of bender and is left out. `COMMON_CELLS_ASSERTS_OFF` is the define of upstream's Verilator flow (`VLT_BENDER` of `snitch_cluster`).

Upstream's simulator keeps the last definition when two listed files define the same module. A file whose definitions are all redefined by later files is left out of the list; `variants/<v>/*-shadowed.txt` has one line `<dropped path> shadowed by <kept path>` for each. The dependency copies are dropped and the generated ones are kept: the generated `clint.sv`, `clint_reg_pkg.sv` and `clint_reg_top.sv` over those of the clint repository, and `hw/future/reg_to_apb.sv` over the one of register_interface. Those files are not copied. `axi_intf.sv` of axi and `core/include/axi_intf.sv` of CVA6 both define `AXI_BUS`, `AXI_BUS_ASYNC` and `AXI_LITE` and each defines more, so both stay listed (see the slang flags).

## Synthesis view

Top: `occamy_top`. It has no closed IP inside. What upstream keeps outside the chip is a port of `occamy_top`:

- 8 HBM2E channels (`hbm_{0..7}_req_o`, `hbm_{0..7}_rsp_i`, AXI), the controller and PHY are not in the repository;
- the die-to-die links (`hbi_wide_*`, `hbi_narrow_*`) and their configuration ports;
- PCIe (`pcie_axi_*`, `pcie_cfg_*`);
- three FLLs and the boot ROM (`fll_*`, `bootrom_*`, AXI lite);
- chip control, JTAG, GPIO, I2C and SPI pins.

The testbench serves them with behavioural memories (`tb_memory_axi`, `tb_memory_regbus`).

The memories are the behavioural `tc_sram` and `tc_sram_impl` of `tech_cells_generic` (listed as `macros`; the `sram_cfgs_i` inputs carry the configuration of the SRAM macro of the taped-out chip). Clock cells are the generic `tc_clk_*`.

slang flags (`design.toml` has the reasons):

- `--translate-off-format pragma,translate_off,translate_on`: the instruction tracer of CVA6 is between those comments, as synthesis tools read them;
- `--allow-use-before-declare`: `riscv-dbg` `src/dmi_jtag.sv` line 62 uses `dtmcs_q` before its declaration at line 68;
- `-Wno-duplicate-definition`: the two `axi_intf.sv` above.

No other construct was rejected.

### Elaboration

`rtl-bench elaborate occamy.<variant>` (slang 12.1.0, NAS, `-j` as the tool sets it, load average from `/proc/loadavg` before and after):

| | full | q2c2 |
|---|---|---|
| files in `synth.f` | 516 | 516 |
| unique modules elaborated | 315 | 315 |
| instances (all levels, `slang-hier`) | 167986 | 35914 |
| lines of `occamy_soc.sv` | 2946 | 2162 |
| wall time | 4.7 s | 1.0 s |
| peak resident set | 663 MB | 739 MB |
| load average (before, after) | 22.5, 23.1 | 31.1, 30.1 |
| result | 0 errors, 13 warnings | 0 errors, 13 warnings |

The peak of q2c2 is above the one of full: the measurement is that of the `time -v` of the tool and slang caches the parse of the files in both; the time is short and noisy at that load.

## Clocks

`clk_i` (system: CVA6, clusters, groups, serial links) 1 GHz and `clk_periph_i` (peripheral) 0.3 GHz, GF12LP+, from `docs/rm/4_clocking.md` (clock table) and `docs/rm/1_overview.md` ("designed to operate at 1GHz", GF12LP+ technology) of Occamy at the pinned commit. GF12LP+ is not an open PDK: the clocks have no `period_ps`.

## Simulation

Verilator, with a testbench built from upstream's generated `testharness.sv`, `tb_memory_axi.sv`, `tb_memory_regbus.sv` and `uartdpi.sv`, and the C++ of upstream's Verilator flow in snitch_cluster (`common_lib.cc` with the fesvr frontend that loads the ELF and the boot ROM, `bootrom.cc`, `ipc.cc`), `bootdata.cc` and `uartdpi.c` of Occamy. Two files are ours (`sim/`):

- `main.cc`: the main of upstream's Verilator flow (`tb_bin.cc` and `verilator_lib.cc` of snitch_cluster) with the model built with `--timing` (the testharness has delays for the 32.768 kHz RTC, so the main advances the time of the Verilator context) and a last line `Cycles: <number of rising edges of clk_i>`. This is the source of the cycle count.
- `uartdpi.cc`: includes upstream's `uartdpi.c` inside `extern "C"`. Verilator compiles every source file as C++, and upstream's header declares no C linkage.

The libraries of fesvr come from the Spike of the nix shell (`CY_SPIKE`), as for other designs. `bootrom.bin` (the boot ROM of upstream, in `target/sim/test`) is included into the model by `bootrom.cc` with `.incbin`; `-Wa,-I` tells the assembler where it is. The Verilator options are those of `VLT_FLAGS` of snitch_cluster, plus `--output-groups 128`: with the default one group per job, the 28 files of one quadrant were joined into one group of about 100 MB of C++, which one compiler process worked on for more than 20 minutes.

The boot ROM of upstream is the model of a boot ROM behind `bootrom_req_o`: fesvr writes `bootrom.bin`, the entry point of the ELF and the boot data into the memory of the testbench, and CVA6 starts from it.

### Workload

`hello_world` (`target/sim/sw/host/apps/hello_world`): CVA6 runs it from the boot ROM, sets up the UART and prints `Hello world!`; the program ends by writing the exit code to `tohost`, which fesvr reads. `verify.py` of upstream expects exactly `Hello world!\r\n` in `uart0.log`; the pass check is `contains = "Hello world!"` on `uart0.log`. The clusters stay idle but clocked.

`recipe/build-workloads.sh` builds it with upstream's Makefiles (`apps/common.mk`, `toolchain.mk`) and the RISC-V GCC of the nix shell (`riscv64-unknown-elf-gcc`, GCC 16.2.0; upstream names a PULP LLVM only for the Snitch device programs, which are not built). The platform headers that upstream's Makefile generates into `sw/shared/platform/generated` come from the variant, so the two images differ. The link leaves a random temporary object name in the symbol table as a FILE symbol; `objcopy --strip-debug` removes it and keeps `tohost` and `fromhost`, which makes the image the same in every build (two builds were compared).

| image | sha256 |
|---|---|
| `sim/hello_world.full.elf` | `b13ec5c9073fcd18b2b8c684c41ff9ca29930439a5d442c2e4c267f5dbeb81bc` |
| `sim/hello_world.q2c2.elf` | `ac76ef0627798b6b95ea27ea383247f331d15787615662e4cd6190f4a0638bf7` |

### Measurements (q2c2)

`rtl-bench sim occamy.q2c2 hello_world` on the NAS (28 cores, 62 GB; `-j 16`; `/usr/bin/time -v` of the nix `time`; other jobs ran on the machine, so the numbers are not those of an idle box):

| | q2c2 |
|---|---|
| Verilator 5.052: wall time of the build | 2313.6 s (elaboration 6.7 s, conversion 317.1 s, C++ build 1979.6 s) |
| CPU of the whole command (build and run) | 5580 s user, 119 s system |
| peak resident set (the verilator process) | 12.16 GB (Verilator allocated 11.9 GB) |
| input of Verilator | 1888 MB of sources in 706 modules |
| generated C++ | 1382 MB in 708 C++ files (723 files with the headers), 718 objects |
| model | `Vtestharness`, 45.7 MB |
| build directory | 1.9 GB |
| `hello_world` | 280.1 s wall, 1225800 cycles (4.4 kcycles/s), `PASS` |
| load average before and after the command | 28.5 23.7 16.7 and 13.9 29.1 49.6 |

The cycle count is the `Cycles:` line of `sim/main.cc`. One file sets the build time: `Vtestharness__Syms__Slow.cpp` (545 KB, the table of all scopes) took about 25 minutes of one compiler process at `-O1`; the other 700 objects were done in about 8 minutes. The CPU time of the C++ build is 5100 s, 16 % of 16 jobs over its wall time. The run writes `trace_hart_00.dasm` (5.6 MB for this run; the mock tracer of CVA6 for Verilator) and `logs/` into its run directory; both grow with the length of a run.

### Estimate for full (not built)

The model of `full` was not built. From the q2c2 numbers: slang counts 4.68 times as many instances in `full` (167986 against 35914), and the clusters, which hold most of the model, are 6 times as many (24 against 4); Verilator makes one module per parameter set (`snitch_cc__pi177` and the like) so the number of generated modules grows with the cores. A factor of 5 to 6 on size gives:

| | q2c2 (measured) | full (estimate) |
|---|---|---|
| peak resident set of Verilator | 12 GB | 55 to 70 GB (the NAS has 62 GB) |
| generated C++ | 1.4 GB | 7 to 8 GB |
| conversion | 5 min | 25 to 30 min |
| C++ CPU time | 1.4 h | 7 to 9 h |
| wall time of the build | 39 min | 3 h or more (the scope table alone is one file and grows too) |
| `hello_world` run | 4.7 min | 30 to 60 min |

The build of `full` does not fit the NAS and needs a machine with at least 96 GB.


## Costs

The times and memory sizes in this file are costs of elaboration and simulation, not results of an implementation tool.

The simulation model of `full` has not been built; its numbers above are an estimate.

## Other files

- `variants/<v>/tools.txt`: versions of the tools of the run.
- `recipe/unused.txt`: the files of `src/` that no view lists, with the reason.
- `recipe/q2c2.hjson`, `recipe/q2c2.diff`: the configuration of `q2c2` and its difference to `full.hjson`.
