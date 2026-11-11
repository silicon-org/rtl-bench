# Recipe of blackparrot

How `src/`, `variants/unicore/` and `sim/` were made. Nothing under `src/` is edited; every file is a copy from the pinned commits at the same relative path (the submodules sit where upstream has them, under `src/external/`).

| repository | commit | licence (file) |
|---|---|---|
| https://github.com/black-parrot/black-parrot | `909619a68f3541a4cd0973767daf2a561d1f8c95` (main, 2026-11-10) | BSD-3-Clause (`src/LICENSE`) |
| https://github.com/bespoke-silicon-group/basejump_stl (`external/basejump_stl`) | `c9710312482ddd725e14b26eafdcbf3c629dc14c` | Solderpad Hardware License 0.51 (`src/external/basejump_stl/LICENSE`; full text in `licences/SHL-0.51.txt`) |
| https://github.com/bsg-external/HardFloat (`external/HardFloat`) | `5b7d5fe2df7e297b5ba095b3eb8a9517dc2e9d88` | BSD-3-Clause (`src/external/HardFloat/COPYING.txt`, and the header of every HardFloat file) |

The commits of the submodules are those that the pinned commit records. The third submodule, `external/bedrock` (`ed68e749079df57afa4b2e153184bf9301137bf2`), holds the assembler of the CCE microcode; no file list of this variant uses it, so it is not vendored. The `LICENSE` of BaseJump STL is a notice that names the Solderpad Hardware License 0.51 and gives its URL, so the text is vendored by `recipe/fetch-licences.sh` from https://solderpad.org/licenses/SHL-0.51/ (title, headings, paragraphs and list items of the page). The files of BlackParrot and BaseJump STL carry no licence header of their own.

## Reproduce

```
git clone --recurse-submodules https://github.com/black-parrot/black-parrot /path/to/bp
git -C /path/to/bp checkout 909619a68f3541a4cd0973767daf2a561d1f8c95
git -C /path/to/bp submodule update --init external/basejump_stl external/HardFloat
nix develop -c designs/blackparrot/recipe/resolve.sh /path/to/bp
nix develop -c designs/blackparrot/recipe/fetch-licences.sh
nix develop -c designs/blackparrot/recipe/build-workloads.sh /path/to/black-parrot-sdk
```

## Synthesis top and file lists

Upstream's synthesis checks (`bp_top/dc/Makefile`, target `check_design.dc`, and `sv2v.dc`, both in `mk/Makefile.dc`) take `wrapper` as the top: `DESIGN_NAME ?= wrapper`, `SV2V_DESIGN_NAME ?= wrapper`. It is `bp_top/test/tb/bp_tethered/wrapper.sv`, which instantiates `bp_processor` (the SoC top: a unicore for `e_bp_unicore_cfg`) and the clock divider for its real-time clock. The flow builds the file list from `bp_top/flist.vcs` plus `wrapper.sv`, and passes `+define+BP_CFG_FLOWVAR=<cfg>`; the configuration of the variant is `e_bp_unicore_cfg` (`bp_common/src/include/bp_common_aviary_pkgdef.svh`), the unicore the upstream documentation uses (`docs/eval_guide.md`).

`resolve.sh` reads the lists as they are:

- `variants/unicore/synth.f`: the files of `bp_top/flist.vcs` (286) and the wrapper.
- `variants/unicore/sim.f`: the files of `bp_top/test/tb/bp_tethered/flist.vcs` (the testbench, tracers, host, DRAM model, a null co-simulation stub), without the wrapper.
- `variants/unicore/incdirs.txt`: the `+incdir+` lines of both lists. Two of them, `bsg_clk_gen` and `bsg_dmc`, contain no file that any listed file includes, so they are not in `design.toml`.
- The files that the listed ones `include` (found by `recipe/includes.py`, looked up next to the including file and in the include directories) are copied too, for example `bsg_defines.sv`.

## Memories

BaseJump STL's `bsg_mem_*` modules are wrappers that instantiate the behavioural `bsg_mem_*_synth` module; a process replaces the wrapper by a hard macro. Upstream's synthesis checks run without a process, so the behavioural models stay in. The unicore instantiates seven of the wrappers; `design.toml` lists them as `macros` with their models (found in the slang AST of the elaborated `wrapper`).

## Simulation

Upstream's Verilator flow is `mk/Makefile.verilator` (`build.verilator`, `sim.verilator`) with the testbench `bp_top/test/tb/bp_tethered` (`Makefile.params`, `Makefile.collateral`). `rtl-bench sim` builds and runs the same: the options and the `-pvalue+` parameters (with their default values) are in `design.toml`; `waiver.vlt` comes first on the command line, as in the Makefile. Differences: `-j` and the output directory are ours, the OPT flags of the C++ build are the ones of `rtl-bench` for every design. The run converts the program as `Makefile.collateral` does (`objcopy` to `prog.bin`, then to a Verilog memory image `prog.mem`) and passes `+elf_file` and `+init_mem`; the unicore needs no CCE microcode (`+ucode_mem` is only passed when `CFG_UCODE=1`). The testbench ends with `[BSG-PASS]` or `[BSG-FAIL]` and exits with 0, so the pass check reads the output. The cycle count is the `clk` counter that the testbench writes to `perf_0.trace` at the end.

## Workloads

`ci/sim-riscv-tests.sh` and `ci/sim-coremark.sh` name the programs: suite `riscv-tests` (`rv64ui-p-simple`, `rv64ui-p-add`) and suite `coremark` (program `coremark`). The programs come from black-parrot-sdk (https://github.com/black-parrot-sdk/black-parrot-sdk, commit `1b400234fd4c7f1f91d3e204d9469681f0e5e7de`), not from the BlackParrot repository; `demo.riscv` is the one prebuilt program in BlackParrot (`bp_top/test/tb/bp_tethered`), it prints "Hello World!".

`build-workloads.sh` follows `mk/Makefile.prog` and `mk/Makefile.tools` of the SDK, with the patches of `patches/` applied to copies of its submodules:

| submodule | commit | |
|---|---|---|
| riscv-tests | `3df703cd52d5f0d8224e3c50bf03c6cb70efff98` (master), env `6de71edb142be36319e380ce782c3d1830c65d68` | the commit that the SDK records (`bf18c909ba7b1c4f17163e674846bd876c1f026b`) cannot be fetched from github.com/riscv/riscv-tests; its patches (`0001`, `0002`, env `0001` to `0005`) apply to master |
| libgloss-dramfs | `8852d252fcfc584b81b2a2a4301ad7e171251de2`, littlefs `9c163b875636f63e752dccd9ab51ecb366f4d79c` | crt0, linker script, specs, libc glue |
| libperch | `d685054e125f16221b6f7ae9c2f787302b959caa` | BlackParrot runtime (`perch.specs`, `bp_utils.h`) |
| coremark | `2e9d6165f362f7071d29b7f88c6d32655e5d4a3c`, patch `0001-portme.patch` | 10 iterations (`ITERATIONS ?= 10` of the patched `core_portme.mak`) |

Differences from the SDK, all because the SDK builds its own cross compiler (riscv-gnu-toolchain, newlib for `-mcmodel=medany`) and we use the one from nixpkgs (GCC 16.2.0, binutils 2.46):

- The nixpkgs newlib is built for the medium-low code model, which cannot address the DRAM at 0x80000000 ("relocation truncated to fit: R_RISCV_HI20"). `flake.nix` builds newlib again with `-mcmodel=medany` (`RISCV_NEWLIB_MEDANY`) and CoreMark links against it (`-L`).
- The `crtbegin.o` and `crtend.o` of the nixpkgs compiler have the same problem. CoreMark links empty ones (`RISCV_EMPTY_CRT`), so no C++ or transactional-memory constructors run; CoreMark has none.
- `FLAGS_STR` is set on the make command line, so that the compiler flags that CoreMark prints, and the image, carry no build paths. The build directory is a fixed path (`/tmp/rtl-bench-blackparrot-workloads`) because the riscv-tests images embed it.
- `objcopy --strip-debug` on the riscv-tests images drops the symbol that names a random temporary file of the compiler; two builds give the same bytes.

| image | sha256 |
|---|---|
| `sim/rv64ui-p-simple.riscv` | `8280365122550960cc5ff56123d5b32cf6be6a107f864094608ad9c694a7da32` |
| `sim/rv64ui-p-add.riscv` | `2f5cdfeeab744d178c9d84efbd4154a297ada21907052655d83c097dbef1dee5` |
| `sim/coremark.riscv` | `4866f07d62743436b2a7069362c3c5c7423d169920bf39c57a86fdefcf0b580b` |

The `activity` window of CoreMark is `start_time` to `stop_time`, the functions of `core_portme.c` that bound its timed region.

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, GCC 16.2.0.
