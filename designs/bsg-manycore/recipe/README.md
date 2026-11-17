# Recipe of bsg-manycore

How `src/`, `variants/`, `sim/` and `licences` were made. Nothing under `src/` is edited: every file is a copy from the pinned commits at the same relative path, with `bsg_manycore` and `basejump_stl` side by side as upstream expects them (`BASEJUMP_STL_DIR` of `machines/` is `../basejump_stl`).

| repository | commit | licence (file) |
|---|---|---|
| https://github.com/bespoke-silicon-group/bsg_manycore | `aa62185eca7623bf270c361e5f663676b2c89cd2` (master, 2026-11-17) | Solderpad Hardware License 0.51 (`src/bsg_manycore/LICENSE`, which holds the full text) |
| https://github.com/bespoke-silicon-group/basejump_stl | `3904f06a907108107b2fd2f54364a28a2ca27e10` (master, 2026-11-16) | Solderpad Hardware License 0.51 (`src/basejump_stl/LICENSE` names it; the text is in `src/bsg_manycore/LICENSE`) |
| https://github.com/bsg-external/HardFloat (`imports/HardFloat` of bsg_manycore) | `5b7d5fe2df7e297b5ba095b3eb8a9517dc2e9d88` | BSD-3-Clause (`src/bsg_manycore/imports/HardFloat/COPYING.txt`) |
| https://github.com/bsg-external/DRAMsim3 (`imports/DRAMSim3` of basejump_stl) | `0bd141a8d6b606e11c2aea6cf605a62853c60e74` | MIT (`src/basejump_stl/imports/DRAMSim3/LICENSE`) |

bsg_manycore does not pin basejump_stl: the README says to clone it next to bsg_manycore, and `.gitlab-ci.yml` clones branch master. The pin here is master at the last commit before the pinned commit of bsg_manycore. The files of the two repositories carry no licence header; the licence is that of the repository. The other submodules of bsg_manycore (`imports/riscv-tests`, `imports/coremark`, `imports/beebs`, `imports/greenlight`) hold no file of the synthesis or simulation view or of the workloads below, so they are not vendored. The `licence` of `[source]` lists every licence that a vendored file is under.

## Reproduce

```
git clone https://github.com/bespoke-silicon-group/bsg_manycore /path/to/bsg_manycore
git -C /path/to/bsg_manycore checkout aa62185eca7623bf270c361e5f663676b2c89cd2
git -C /path/to/bsg_manycore submodule update --init imports/HardFloat
git clone https://github.com/bespoke-silicon-group/basejump_stl /path/to/basejump_stl
git -C /path/to/basejump_stl checkout 3904f06a907108107b2fd2f54364a28a2ca27e10
git -C /path/to/basejump_stl submodule update --init imports/DRAMSim3
nix develop -c python3 -I designs/bsg-manycore/recipe/resolve.py /path/to/bsg_manycore /path/to/basejump_stl
nix develop .#bsg -c designs/bsg-manycore/recipe/build-workloads.sh
```

## File lists

`recipe/resolve.py` reads the two lists of upstream as `machines/Makefile` does with `BSG_PLATFORM=verilator` (make includes `machines/arch_filelist.mk` and `machines/sim_filelist.mk`):

- `variants/<id>/synth.f`: the headers and sources of `arch_filelist.mk` (177 files, the synthesizable architecture: vanilla core, HardFloat, tile, pod, mesh and ruche networks, vcache, the BaseJump STL modules they use), in upstream's order, headers first.
- `variants/<id>/sim.f`: the files that `sim_filelist.mk` adds (the testbench, the non-synthesizable memories and DRAM models, the profilers, the C++ sources of the testbench and of DRAMSim3), then the tag boot ROM of the machine. The synthesis files come first.
- The files that the listed ones `include` (Verilog and C++) are copied too, and the include directories are those of `VINCLUDES`.

The tag boot ROM `sim/<id>/bsg_tag_boot_rom.v` is generated the way `machines/Makefile.verilator` does: `testbenches/py/pod_trace_gen.py <pods x> <pods y>` makes the bsg_tag trace, `bsg_mem/bsg_ascii_to_rom.py` makes the module `bsg_tag_boot_rom` from it. Both machines have one pod, so both ROMs are the same. The generator writes the path of its input into the first line, so the script runs it in a fixed directory.

## Synthesis top and parameters

The top is `bsg_manycore_pod_ruche_array`, the DUT that `bsg_nonsynth_manycore_testbench.sv` instantiates for `BSG_MACHINE_NETWORK_CFG = e_network_half_ruche_x`, which is the network of both machines. The synthesis flow of upstream (`syn/`, only `bsg_sv2v`) takes the tile modules (`bsg_manycore_tile_compute_ruche`, `bsg_manycore_tile_vcache`) as targets, and names `bsg_manycore_pod_ruche` as too slow for it. The array is the top that has the whole mesh: pods of compute tiles with the vcache rows north and south.

The parameters of the array are those that `spmd_testbench.sv` and `bsg_nonsynth_manycore_testbench.sv` compute from the `BSG_MACHINE_*` values of `machines/<machine>/Makefile.machine.include`. `design.toml` lists the result; `pod_x_cord_width_p` and `pod_y_cord_width_p` are the coordinate widths minus `clog2` of the tile counts; `vcache_size_p` is sets times ways times block words; `wh_cid_width_p` and `wh_len_width_p` follow `wh_ruche_factor_p` and the block size as in the testbench. The tile parameters that the testbench takes as constants are used as constants (`dmem_size_p` 1024, `icache_entries_p` 1024, `icache_tag_width_p` 12, `icache_block_size_in_words_p` 4, 32 bit data).

Memories: the array instantiates `bsg_mem_1rw_sync` (instruction cache, data memory and vcache data), `bsg_mem_1rw_sync_mask_write_byte` and `bsg_mem_1rw_sync_mask_write_bit` (vcache tag and statistics) and `bsg_mem_1r1w` (FIFO storage). Each is a wrapper of BaseJump STL that instantiates its behavioural `_synth` model; `design.toml` lists the four as macros. Upstream's `syn/README.md` says the instruction cache (1024x46), the data memory (1024x32) and the vcache memories are replaced by hardened memories of the process; that replacement is not done here. The register file is `regfile_synth` (flip-flops).

slang needs `-Wno-index-oob`: `bsg_mesh_router_decoder_dor.sv` reads `from_p[RN]` and `from_p[RS]` of a 7 bit vector in the ruche router, which slang reports as an error; Verilog gives x for such a read and the branch is not taken.

There is no clock period: upstream has no SDC for the array and publishes no frequency for an open PDK (the Celerity and HammerBlade papers report commercial nodes). The testbench clocks the core at 1 GHz, a simulation setting.

## Machines

All machines are those of `machines/`; no parameter is changed.

| variant | machine | tiles (x by y) | network | vcache banks |
|---|---|---|---|---|
| `pod_1x1_4x2y` | `pod_1x1_4X2Y` | 4 by 2 = 8 | half ruche in x, ruche factor 3 | 4 north, 4 south |
| `pod_1x1` | `pod_1x1` | 16 by 8 = 128 | half ruche in x, ruche factor 3, IPOLY hashing | 16 north, 16 south |

`machines/` also has `pod_1x1_2X2Y` (a 2 by 2 mesh), `pod_1x1_hbm2`, `pod_4x4` (16 pods) and `pod_4x4_hbm2`. The first uses `bsg_manycore_pod_mesh_array`; the others use the HBM2 model of DRAMSim3 or 16 pods, which the Verilator flow of upstream does not support (`README.md`: "This platform only currently supports the machine pod_1x1_4X2Y due to excessive compilation times for larger machines"). `pod_1x1` is the machine that upstream's VCS flow uses by default; it builds in Verilator, slowly.

## Simulation

Upstream's Verilator flow is `machines/Makefile.verilator` with the testbench `spmd_testbench` (`testbenches/common/v`) and the C++ main `test.cpp`. `rtl-bench sim` builds the same: the options, the `BSG_MACHINE_*` defines and the C++ flags are in `design.toml`. Differences, each with its reason:

- `--no-assert`: `test.cpp` turns assertions off until `reset_o` reads 1, but the model is two-state and every register starts at 0, so `reset_o` is 0 at time 0 and the assertions of the not yet reset array fire at the first clock (`Errant fwd packet detected` in `bsg_manycore_link_sif_tieoff.sv`). A four-state simulator reads x there. blackparrot takes the same option from its upstream flow.
- `VERILATOR_WORKAROUND_DISABLE_*` defines: the switches that `bsg_nonsynth_manycore_testbench.sv` has for Verilator. Without them the profilers of the testbench are bound into every tile. They cost build time and run time (see below) and write CSV traces that nothing here reads. The cycle counts are the same with and without.
- `-std=c++17` instead of `-std=c++14` (Verilator 5.052 generates code that needs C++17, and the fmt of DRAMSim3 does not compile as C++20, the default of this Verilator); no `-g`.
- `-j` and the output directory are those of `rtl-bench`.

The testbench loads the program through the network (`bsg_nonsynth_manycore_spmd_loader`, from the network boot file `+nbf_file`), runs until the tile group sends the finish packet to the host, and prints `RECEIVED BSG_FINISH PACKET from all pods, time=<ps>` and calls `$finish`. The exit code is 0 in both cases, so the pass check reads the line; with the cycle limit `+max_cycle` it prints `BSG_TIMEOUT` instead. A failing program sends `BSG_FAIL`. The core clock is 1000 ps, so the cycle count is the time without its three last zeros.

## Workloads

Programs of `software/spmd` that `ci/spmd.sh` runs (it recurses into every directory of `software/spmd` that is not in `NO-RECURSE`), built with upstream's makefiles (`software/mk`). The program and the machine fix the ELF (`bsg_link.ld` from `software/py/bsg_manycore_link_gen.py`, the `-D` options of `Makefile.builddefs`) and the network boot file (`software/py/nbf.py`, as the rule `%.nbf` of `Makefile.builddefs`), so the images are per variant.

| program | tile group | what it does |
|---|---|---|
| `hello` | 1 by 1 (its Makefile) | prints through the host, reads DRAM, finishes |
| `bsg_mutex` | every tile of the pod | every tile takes a mutex of tile (0,0) and increments a counter; tile (0,0) waits for the count of all tiles, then finishes |
| `bsg_barrier_amoadd_test` | every tile | seven rounds of an AMO add, the AMO barrier and a check of a neighbour tile; a wrong value is a fail packet |
| `vcache_atomic_inc` | every tile | every tile takes a lock in DRAM (the vcache banks) with amoswap, increments a counter and prints it; the last tile finishes |

`bsg_barrier` and `bsg_tile_group_barrier` of the same directory print one line per tile twice; on 128 tiles `bsg_barrier` had printed 72 of the 128 lines of its first print after 20 minutes of wall time and was stopped, so they are not used.

The tile groups of `bsg_mutex` (2 by 4) and `bsg_barrier_amoadd_test` (16 by 8) are set in their Makefiles; the build passes `bsg_tiles_X` and `bsg_tiles_Y` of the machine on the make command line (`software/mk/Makefile.dimensions` takes them from the machine when the Makefile does not set them, as for `vcache_atomic_inc`), so that the program runs on every tile of either machine. This is what makes the same program a scaling workload.

Toolchain: upstream builds a patched GCC 9.2 or LLVM (`software/riscv-tools`, a build of several hours). Stock GCC works with upstream's linker script generator and makefiles, so the images are built with GCC 16.2.0 and newlib built from nixpkgs for `rv32imaf` and `ilp32f` (`nix develop .#bsg`: `BSG_RISCV_BIN`, with the names `riscv32-unknown-elf-dramfs-*`; the shell builds GCC and newlib from source, which the default shell does not). Differences from upstream's compiler:

- `-mtune=bsg_vanilla_2020` (a scheduling model of the patched GCC) is dropped by the wrapper; only the instruction order differs.
- `ARCH_OP=rv32imaf_zicsr_zifencei` instead of `rv32imaf`: binutils 2.46 needs the extensions for the CSR instructions of `crt.S`.
- `IGNORE_CADENV=1` and `BSG_MANYCORE_DIR`: no VCS, no `/usr/bin/perl`.

`recipe/build-workloads.sh` builds all images in a fixed directory (`/tmp/rtl-bench-bsg-manycore-workloads`) and copies `main.nbf` to `sim/<id>/<program>.nbf`. Two builds gave identical files.

| image | sha256 |
|---|---|
| `sim/pod_1x1_4x2y/bsg_barrier_amoadd_test.nbf` | `a475170458a190ce74b18bf9dbe5deaffc2770147969f2b1f59b35dd43c3dd9b` |
| `sim/pod_1x1_4x2y/bsg_mutex.nbf` | `fa6812c6af15ea93fff16bdf20422e19fcddb09a9cc3dc0bc11f7f9d45dffa92` |
| `sim/pod_1x1_4x2y/hello.nbf` | `78c127b99f56d73f5d5bf9dd2bafb59b14054d5d2736450f7cb4404f616358b7` |
| `sim/pod_1x1_4x2y/vcache_atomic_inc.nbf` | `ad757125f07b4a25dd7b6f882ef1663677ee4fea29f33e208f640450c3a5205e` |
| `sim/pod_1x1/bsg_barrier_amoadd_test.nbf` | `aac57bcd36947f0006f37e762c94f3d8029d1467d4ed3dda2f05b42744de798d` |
| `sim/pod_1x1/bsg_mutex.nbf` | `d1004ffb6d60e677a96b4ff481a7245b19219d97e7c6f2912871a4e6eb432e41` |
| `sim/pod_1x1/hello.nbf` | `01eaabf339ccc4e4caf8d977bb2e8c45362a663dae272916d1a39fe04ffbe79d` |
| `sim/pod_1x1/vcache_atomic_inc.nbf` | `31cedb627d1b636ea9e3866e1a46a666332de4fb59a5754a66946f7740f96ff8` |

## Measurements

Measured on the NAS (28 threads, 62 GB; a shared machine, so the load of other jobs is part of the numbers). Verilator 5.052, GCC 16.2.0, slang 12.1.

Elaboration (`rtl-bench elaborate`, slang 12.1):

| variant | tiles | wall time (two runs) | peak memory (two runs) |
|---|---|---|---|
| `pod_1x1_4x2y` | 8 | 0.8 s, 1.3 s | 0.29 GB, 0.24 GB |
| `pod_1x1` | 128 | 0.9 s, 0.5 s | 0.36 GB, 0.32 GB |

slang elaborates instances with identical parameters once, so the time does not grow with the tile count (the 128 tile pod is 16 times the tiles of the 8 tile pod and has the same tile, router and cache modules). The AST in JSON grows from 380 MB to 462 MB.

Verilator build (`rtl-bench sim`, which verilates and compiles; peak memory is the sum of the resident sets of all processes, sampled every 0.5 s):

| variant | jobs | wall time | CPU time | peak memory |
|---|---|---|---|---|
| `pod_1x1_4x2y` | 16 | 34 s | 186 s | 4.1 GB (the run of the first workload follows in the same measurement, 2.2 GB) |
| `pod_1x1_4x2y` | 2 | 92 s | 169 s | 2.2 GB, the run of the first workload; the build stays below |
| `pod_1x1` | 16 | 546 s | 2212 s | 15.2 GB (verilator 4.1 GB) |

Without the `VERILATOR_WORKAROUND_DISABLE_*` defines the same builds took 47 s, 280 s of CPU and 5.4 GB for `pod_1x1_4x2y`, and 906 s, 3396 s of CPU and 24.1 GB (verilator 7.8 GB) for `pod_1x1`. A runner with 2 cores and 7 GB builds the 8 tile model. The 128 tile model peaks at 15 GB with 16 jobs, and its verilator process keeps 4.1 GB while the C++ compiles (two compile jobs of up to 1.4 GB come on top), so it is left out of CI; it was not measured with 2 jobs.

Runs (the cycle count of the line `RECEIVED BSG_FINISH PACKET`; wall time with the other jobs of the NAS running, load average 40 to 80, so the times of the long runs vary by a factor of 2 between repeats; peak memory is that of the process):

| variant | workload | cycles | wall time | peak memory |
|---|---|---|---|---|
| `pod_1x1_4x2y` | `hello` | 10558 | 2.5 s | 2.2 GB |
| `pod_1x1_4x2y` | `bsg_mutex` | 2958 | 2.6 s | 2.2 GB |
| `pod_1x1_4x2y` | `bsg_barrier_amoadd_test` | 4695 | 2.2 s | 2.1 GB |
| `pod_1x1_4x2y` | `vcache_atomic_inc` | 2813 | 2.3 s | 2.0 GB |
| `pod_1x1` | `hello` | 13780 | 41 s | 2.2 GB |
| `pod_1x1` | `bsg_mutex` | 57761 | 63 s | 2.2 GB |
| `pod_1x1` | `bsg_barrier_amoadd_test` | 53751 | 134 s | 2.2 GB |
| `pod_1x1` | `vcache_atomic_inc` | 69047 | 158 s | 2.2 GB |

The 2 GB of every run is the memory of the testbench (the host and DRAM models), not of the pod. The simulator runs at about 4000 cycles per second for 8 tiles and 300 to 500 cycles per second for 128 tiles.

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, GCC 16.2.0, Python 3.14. The compiler of the workloads is built by `flake.nix` (`rv32f`, shell `bsg`); `rtl-bench` itself needs only the default shell.
