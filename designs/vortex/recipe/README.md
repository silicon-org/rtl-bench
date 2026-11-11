# Recipe of vortex

How `src/`, `variants/default/` and `sim/` were made. Nothing under `src/` is edited; every file is a copy from the pinned commit at the same relative path, except `src/generated/`.

Upstream: https://github.com/vortexgpgpu/vortex, commit `ca18837bd0871911af691e704d8ebdd14de5b4ec` (master, 2026-11-10). Licence: Apache-2.0 (`src/LICENSE`, the full text; every source file carries an Apache header). Vortex has five submodules (`third_party/`: softfloat, ramulator, cvfpu, hardfloat, cocogfx). None is vendored: cvfpu and hardfloat serve the FPNEW floating point and the tensor-unit options, which this variant does not use; softfloat and ramulator are tools of the host program and come from the nix shell (see Simulation); cocogfx serves the graphics extensions.

## Reproduce

```
git clone https://github.com/vortexgpgpu/vortex /path/to/vortex
git -C /path/to/vortex checkout ca18837bd0871911af691e704d8ebdd14de5b4ec
git clone https://github.com/riscv-software-src/riscv-tests /path/to/riscv-tests
git -C /path/to/riscv-tests checkout 1eb47d946c55f55cab8653c224c2993acc0276bd
git -C /path/to/riscv-tests submodule update --init --recursive
nix develop -c designs/vortex/recipe/resolve.sh /path/to/vortex
nix develop -c designs/vortex/recipe/build-workloads.sh /path/to/riscv-tests
```

## Variant

`default`: one cluster, one core, 4 warps, 4 threads, XLEN 32 (RV32IMF with Zicond), no L2, no extensions. These are the defaults of `VX_config.toml` (`VX_CFG_NUM_CLUSTERS = 1`, `VX_CFG_NUM_CORES = 1`, `VX_CFG_NUM_WARPS = 4`, `VX_CFG_NUM_THREADS = 4`); upstream's own flows (`rtlsim`, `hw/syn/yosys`) build this configuration unless told otherwise, and `./configure` defaults to `--xlen=32`.

## Generated headers

`./configure` generates `hw/VX_config.vh`, `hw/VX_types.vh`, `sw/VX_config.h` and `sw/VX_types.h` from `VX_config.toml` and `VX_types.toml` with `ci/gen_config.py` (the "generate header files" step of `configure`). `resolve.sh` runs the same commands with `XLEN=32` (`--resolved` for `VX_types`) into `src/generated/`. The generator and its two inputs are vendored. The macros that depend on the flow (`VX_CFG_FPU_TYPE_*`) are given as defines, as the flows pass them (`ci/gen_config.py --cflags=...`): `STD` for `-DSYNTHESIS -DASIC -DYOSYS`, `DPI` for `-DSIMULATION -DSV_DPI`.

## Synthesis top and file list

Upstream's ASIC flow is `hw/syn/yosys` (`make asap7 / synthesis / techmap / timing`, PDK asap7 or nangate45): `TOP_LEVEL_ENTITY ?= Vortex`, with `-DSYNTHESIS -DASIC -DYOSYS` and `-DNDEBUG`. The top is therefore `Vortex` (`hw/rtl/Vortex.sv`), the generic top with the memory interface of the cache hierarchy, not `Vortex_axi` (the AXI4 adapter that the FPGA platform shells instantiate) and none of the shells. The flow reads (`hw/scripts/gen_sources.sh`) every `.v` and `.sv` file that is directly in one of its include directories: `hw/rtl` and its `libs`, `interfaces`, `core`, `mem`, `vm`, `cache` and `fpu` directories, and `hw/syn/libs/no_mem`; the extension directories are added only when their extension is enabled (`hw/syn/extensions.mk`). `resolve.sh` writes that list, packages first, to `variants/default/synth.f` (247 files), and copies the headers of those directories.

Left out of the vendored tree, and why:

- `hw/rtl/afu/` (OPAE, XRT, AVED, FireSim and FoldSim shells): FPGA platform wrappers, not in the ASIC flow.
- `hw/rtl/cp`, `dxa`, `tcu`, `tex`, `raster`, `om`, `rtu`, `mufu`: extensions that the default configuration leaves off (`VX_CFG_EXT_*_ENABLE = false`); the flow includes them only when enabled.
- `hw/syn/xilinx`, `altera`, `synopsys`, `yosys`, `firesim`, `foldsim` and the process libraries `hw/syn/libs/*` except `no_mem`: flow scripts and vendor libraries (the `cln28*` memory compilers are for a commercial process).
- `sim/` except the rtlsim host program, `sw/` except `sw/common`, `tests/`, `docs/`, the other `hw/` directories.

The files `VX_fpu_dsp.sv`, `VX_fdiv_unit.sv`, `VX_fma_unit.sv`, `VX_fsqrt_unit.sv`, `VX_fetch.sv` and a few others contain `ifdef QUARTUS` and `ifdef VIVADO` branches (FPGA vendor primitives); they are in the list as upstream has them, and the branches are inactive.

## Memories

With `-DASIC`, `VX_sp_ram`, `VX_dp_ram` and `VX_1rw1r_ram` instantiate `VX_sp_ram_asic` and `VX_dp_ram_asic`. Upstream's `hw/syn/libs/no_mem` has them as black boxes with a behavioural body (a register array); a process replaces them by memory macros (the flow has `BB_MODULES=VX_dp_ram_asic,VX_sp_ram_asic`). They are the `macros` of the variant. The simulation does not define `ASIC`; its RAMs are arrays in the RTL, and the two stubs are left out of its list.

## Simulation

Upstream's `rtlsim` (`sim/rtlsim/Makefile`): Verilator on `rtlsim_shim` (the memory model around `Vortex`) linked with the host program `main.cpp`, `processor.cpp` and the models of `sim/common` (memory, ELF loader, host monitor, console drain, DRAM). `design.toml` has its Verilator options and C++ flags; `rtl-bench sim` builds it with `--exe --cc --build`. The DRAM model is Ramulator 2 and the floating point goes through SoftFloat (the `dpi_float` functions). Both are built by `flake.nix` from the commits of the Vortex submodules (SoftFloat `b51ef8f`, Ramulator `e62c84a` with its GDDR6 model as `third_party/Makefile` builds it); Ramulator fetches yaml-cpp 0.7.0 (`0579ae3`), spdlog 1.11.0 (`ad0e89c`) and argparse 2.9 (`997da92`), pinned there. The flake sets `VX_SOFTFLOAT` and `VX_RAMULATOR`, which `design.toml` uses as `${VX_SOFTFLOAT}` in the flags. Deviation: GCC 16 warns about a dangling reference in the fmt of spdlog, so `-Wno-error=dangling-reference` follows the upstream `-Werror`.

`rtlsim` takes the program as its argument. An ELF carries its entry point and its HTIF `tohost` word; the host monitor ends the run with the exit code of the program. rtlsim prints no cycle count, so the workloads have no `max_cycles`.

## Workloads

`rv32ui-p-simple` and `rv32ui-p-add` are riscv-tests (https://github.com/riscv-software-src/riscv-tests, commit `1eb47d946c55f55cab8653c224c2993acc0276bd`, the commit of `tests/riscv/common.mk`) ISA tests, built as that makefile builds them: unmodified, except that `env/p/link.ld` places `.tohost` at `VX_MEM_IO_EXIT_CODE` (uncached IO, so that the store reaches the host monitor), the sed command of `common.mk`. The compiler is the nixpkgs GCC 16.2.0 for `riscv32-none-elf` (named `riscv32-unknown-elf-*` in the shell), not Vortex's own riscv32-gnu-toolchain; the ISA tests need only the assembler and the linker. `objcopy --strip-debug` drops the random temporary-file symbol, so two builds give the same bytes.

| image | sha256 |
|---|---|
| `sim/rv32ui-p-simple` | `dc1f905e25bcaef24789ebda33fa92374b3d7ab428642afccce2a3855caa29ad` |
| `sim/rv32ui-p-add` | `749945a5c0a0701555df3b491b084efc4f94bffd8e80aa2219d8acd22efa6f99` |

There is no long workload. Upstream's regression kernels (`tests/regression/*`) except `basic` build with Vortex's LLVM (`+xvortex`) and need the host runtime; `basic` needs the host runtime too. `tests/kernel/*` (hello, fibonacci with n = 9, vecadd) are a few hundred cycles. The longest programs that rtlsim runs standalone are the riscv-tests benchmarks (`tests/riscv/benchmarks`: median, qsort, rsort, towers, memcpy, multiply, dhrystone; spmv needs soft double precision that the nixpkgs libgcc does not have). `recipe/benchmarks.sh` builds them as `tests/riscv/common.mk` does (with `miscs/patches/riscv-benchmarks.patch`, `-Wl,--no-warn-mismatch` and `-mstrict-align`, because the nixpkgs libgcc is built for the double-float ABI and its GCC may emit misaligned accesses). All of them stop in an RTL assertion shortly before the program ends (at the cycle in brackets):

```
[270923] %Error: VX_allocator.sv:66: Assertion failed in TOP.rtlsim_shim.vortex.g_clusters[0].cluster.g_sockets[0].socket.g_cores[0].core.g_compute_slices.g_slices[0].compute_slice.backend_slice.execute_slice.lsu_block.mem_path.g_coalescing.mem_coalescer.req_ibuf.allocator: releasing invalid addr 1
```

median 270923, towers 154757, memcpy 705693, multiply 1147329, dhrystone 6088877, qsort 5760099, rsort 6868351. With `--no-assert` median does not finish (three minutes of wall time with no exit). The cause is not known: either the RTL at this commit with Verilator 5.052, or the compiler difference (GCC 16 against Vortex's GCC 9.2 toolchain). The benchmarks are not part of the design.

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, GCC 16.2.0 (riscv32 and riscv64 cross compilers).
