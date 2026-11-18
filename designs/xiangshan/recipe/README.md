# Recipe of xiangshan

How `src/`, `variants/minimal/`, `licences/` and `sim/` were made. The RTL is not a copy of an upstream file: it is the output of two runs of the Makefile of XiangShan (the Verilog of the SoC and the Verilog of the simulation top), vendored as it came out, plus the harness of the difftest.

## Generation

`recipe/generate.sh <work dir>` runs, in the nix dev shell and its FHS shell (`nix develop -c otfhs -c recipe/generate.sh <dir>`; `scripts/gen_sep_mem.sh` has the shebang `/bin/bash`, which NixOS does not have):

| step | tool and version |
|---|---|
| XiangShan | https://github.com/OpenXiangShan/XiangShan, tag `v3.2.2-alpha`, commit `0303f76a84fd705d32f6f0434c63c55e2ff02186` (2024-10-24) |
| submodules | rocket-chip, difftest, huancun, fudian, utility, yunsuan, coupledL2, openLLC, `src/main/resources/aia` (OpenAIA) and ready-to-run, at the commits of the tag (`design.toml`; the script checks each); the nested submodules of rocket-chip are cde and hardfloat, the others (torture, softfloat and testfloat, the copies of rocket-chip and utility inside the cache projects) are not used by the build and not fetched |
| build | mill 0.11.8 (`.mill-version` of the tag; the assembly of the release page, sha256 `1ce4537b1233af16d68dc1ab4b9f49a996e8c460b393cc0a7f778a328606aab2`), JDK 17.0.20 (the dev shell), Chisel 6.5.0 and Scala 2.13.14 (`build.sc`) |
| Verilog | CIRCT `firtool-1.62.0`, the version that Chisel 6.5.0 asks for (its firtool-resolver would download the same release); the release binary, sha256 `1aee0d13fea52b504b0f689f4d890cfb7c63f7dc832308ceff9ae3f099e8e800`, is put on the `PATH` |
| commands | `make verilog CONFIG=MinimalConfig` and `make sim-verilog CONFIG=MinimalConfig` |

Why this commit: `v3.2.2-alpha` is the newest tag of the repository (the others are v0.2020, v1.0, v2.0, v2.1). Master has moved on by more than 2400 commits and its Kunminghu development has no tag; a tag has a defined state, its submodules are pinned, and upstream's own CI builds `MinimalConfig` from it (`.github/workflows/emu.yml`: "build MinimalConfig Release emu").

The two commands write to the same directory (`build/rtl`), so the script runs them one after the other and keeps both results (`out/synth`, `out/sim`):

- `make verilog` is the synthesis view: `TOP = XSTop`, with the release arguments of the Makefile (`--fpga-platform --disable-all --remove-assert --reset-gen --firtool-opt --ignore-read-enable-mem`): no difftest, no assertions, no performance counters. firtool writes one file per module (`--split-verilog`, 1531 files of `build/rtl`), `filelist.f`, and the memory configuration `XSTop.sv.conf`, from which `scripts/gen_sep_mem.sh` and `scripts/vlsi_mem_gen` write the 27 memory modules (`array_*_ext.v`). The Makefile puts the commit and `git diff` of the repository (empty here) as comments at the head of `XSTop.sv`.
- `make sim-verilog` is the simulation view: `SimTop` (`XSTop` plus the simulation devices) with `--enable-difftest`, so the Verilog has the difftest event modules, the performance counters and the assertions, and a `generated-src` directory (`difftest-dpic.cpp`, `diffstate.h`, ...) next to `chisel_db.cpp` and `constantin.cpp`. The Makefile then runs `sed` on the Verilog (`$fatal` to `xs_assert_v2`, `$error(` to `$fwrite(32'h80000002, `); that is upstream's own step and part of the output.

Not kept: the FIRRTL of each run (`XSTop.fir`, `SimTop.fir`, 0.8 GB each) and the metadata (`.dts`, `.json`, `.graphml`, `.plusArgs`).

Two runs of the whole script, each from an empty directory (clone, submodules, mill and Chisel compilation, elaboration, firtool) with different work directories, gave identical output: `diff -r` of the two `out/` trees shows only `time.log`, which holds the times of the run. No timestamp or path is in the Verilog or the C++. Measured on the NAS (load average between 8 and 19):

| run | wall | CPU | largest process (the JVM) |
|---|---|---|---|
| first, `make verilog` | 6 min 34 s | 1376 s | 13.5 GB |
| first, `make sim-verilog` | 4 min 07 s | 1030 s | 13.3 GB |
| second, `make verilog` | 4 min 37 s | 1064 s | 17.7 GB |
| second, `make sim-verilog` | 3 min 56 s | 973 s | 14.9 GB |

The whole script took 11 min 27 s (first) and 9 min 18 s (second). The JVM takes a quarter of the memory of the machine as its heap limit by default and grows to it; no limit is set.

`recipe/resolve.sh <work dir>` copies the results to `src/`, the licence files to `licences/` and the images to `sim/`, and writes the file lists:

- `src/generated/synth/rtl/`: the `.sv` and `.v` files of `build/rtl` of `make verilog`, 1531 files and 1.46 M lines (the largest, `StoreQueue.sv`, has 87923 lines), plus `filelist.f`, `firrtl_black_box_resource_files.f` and `XSTop.sv.conf`;
- `src/generated/sim/build/`: the same for `make sim-verilog` (`rtl/`, 1614 files and 2.87 M lines), `generated-src/`, `chisel_db.*`, `constantin.*`; laid out like the `build` directory of upstream, because the emu reads `$NOOP_HOME/build/constantin.txt`;
- `src/difftest/`: the files that the emu is built from (`src/test/csrc/{common,difftest,plugin,verilator}`, `src/test/vsrc/common`, `config/config.h` of the difftest submodule);
- `variants/minimal/synth.f`: the `filelist.f` of `make verilog`, then the memory modules;
- `variants/minimal/sim.f`: the `filelist.f` of `make sim-verilog`, the memory modules, the Verilog of the difftest (`SimJTAG.v`, `assert.v`, `ref.v`) and the C++ files that `difftest/verilator.mk` compiles.

## Synthesis top and memories

`XSTop`: the SoC of one core with its L2 and its devices (what `make verilog` names `TOP`). It has the clock inputs `io_clock` (the SoC and the core), `io_rtc_clock` and `io_systemjtag_jtag_TCK`, the resets `io_reset` and `io_systemjtag_reset`, and the AXI4 interfaces `memory`, `peripheral` and `dma`. No constraint file and no published frequency for an open PDK exist, so the variant has no period.

The 27 memories are the `array_*_ext` modules of `XSTop.sv.conf` (depths 4 to 8192, widths 2 to 317 bits, with and without write masks). `scripts/vlsi_mem_gen` writes them as flop arrays with the ports of a macro; these are the behavioural models of the `macros` in `design.toml`. The same 27 modules are in the simulation view.

## Licences

XiangShan is Mulan PSL v2 (`LICENSE`, the Chinese and the English text; the sources carry `XiangShan is licensed under Mulan PSL v2`). The generator compiles in the submodules, and their Verilog is in the output: rocket-chip (UC Berkeley BSD-3-Clause in `LICENSE.Berkeley` and `LICENSE.jtag`, Apache-2.0 for SiFive files in `LICENSE.SiFive`), cde (Apache-2.0), hardfloat (BSD-3-Clause), huancun, coupledL2 and openLLC (each `LICENSE` is Mulan PSL v2, `LICENSE.SiFive` Apache-2.0), YunSuan (Mulan PSL v2). utility and fudian have no licence file; their sources carry the XiangShan notice. The difftest is Mulan PSL v2. The output has no source locators (`locationInfoStyle=none`), so which module of which submodule is in it is not recorded; all submodules that the build uses are listed.

Open: the `imsic_*.sv` files in the output are copied unchanged from OpenAIA (`src/main/resources/aia`, commit `9464aee3c77b021a037f28aa1b3fe53c71b516f6`). That repository has no licence file, and the files say `Copyright bosc` and nothing else. `design.toml` gives it the licence of XiangShan, which is the licence of the same institute, not a licence that OpenAIA states. The workload images are in the same position (below).

## Simulation

The emu of the difftest, built as `make emu CONFIG=MinimalConfig` builds it (`difftest/verilator.mk`, `difftest/Makefile`), with the Verilator of the dev shell (5.052; the makefile enables its Verilator 5 mode with `--no-timing +define+VERILATOR_5`). `design.toml` carries the flags. The emu links `libz`, `libzstd` and `libsqlite3` (image compression and ChiselDB), which `flake.nix` provides. Left out: profile guided optimisation (upstream's CI uses it), waves and snapshots.

Upstream's `make emu` run with `-j16` on the NAS (default flags): 20 min 40 s of wall time, 5138 s of CPU, 8.2 GB for the largest process (`verilator_bin`). `rtl-bench sim` (`--cc --build -j 16`, the options of the variant, Verilator 5.052): 1734 s of wall time, 6730 s of CPU, 8.4 GB for the largest process (`verilator_bin`, which stays alive while its compile jobs run, 1.4 GB each at most); all build processes together peaked at 34 GB (sum of the resident sizes, sampled every 5 s). The model has the 1614 Verilog files, 27 C++ files and `--output-split 30000`.

Reference model: the emu compares every commit with a reference model (NEMU or Spike, as a shared library, `--diff <library>`). With `--no-diff` there is none, and the emu still stops at the trap instruction of the program (`HIT GOOD TRAP` or `HIT BAD TRAP`). `rtl-bench sim` runs with `--no-diff`, so no reference binary is in the repository. With difftest on (`--diff ready-to-run/riscv64-nemu-interpreter-so`, the library of the ready-to-run submodule), the same emu runs coremark-2-iteration to the same trap in 1151 s (checked with the emu of `make emu`).

## Workloads

`rtl-bench sim` on the NAS (one thread, no reference model, 0.05 GB resident): `coremark-2-iteration` 542257 cycles in 1293 s; `microbench` 366729 cycles in 513 s. Both end with `HIT GOOD TRAP`.

The images `coremark-2-iteration.bin` and `microbench.bin` are copied from the ready-to-run submodule (commit `c9c1451937e61eefe5adfcf6e197d0ee7ca188f0`, 2024-10-19) unmodified. Its README says they are built by the CI of OpenXiangShan/nexus-am (`apps/coremark` with two iterations, `apps/microbench`); it names no nexus-am commit. The programs are bare metal images loaded at `0x80000000`, and end with the trap instruction.

Open: neither ready-to-run nor nexus-am has a licence file. CoreMark carries the EEMBC notice that refers to the CoreMark licence of its official release; microbench has files of several authors (QuickLZ, among others) with their own notices. The images are vendored because the emu needs an AM image to end a run (riscv-tests end by `tohost`, which the emu does not serve); if redistribution is not allowed, the variant has to build them from the nexus-am sources instead.
