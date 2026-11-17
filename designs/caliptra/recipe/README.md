# Recipe of caliptra

How `src/`, `variants/core/` and `sim/` were made. Nothing under `src/` is edited; every file is a copy from the pinned commit at the same relative path (`src/src/aes/rtl/aes.sv` is upstream `src/aes/rtl/aes.sv`).

Upstream: https://github.com/chipsalliance/caliptra-rtl, commit `49370266d12cb0c4a8f71b3a0ff7e54ba7d4866e`, the release tag `v2.1.2` (2026-03-19; the newer `v2.2-rc1` is a release candidate). The Adams Bridge ML-DSA and ML-KEM accelerator is the submodule `submodules/adams-bridge` (https://github.com/chipsalliance/adams-bridge) at `b77e3d899e828d626cfc2a0d26a6b5704cc121e0`. The VeeR EL2 core is not a submodule: upstream keeps the generated RTL in `src/riscv_core/veer_el2`.

## Licences

`LICENSE` is the Apache License 2.0, and `LICENSE` of Adams Bridge is the same text. Read from the files:

- About 660 sources carry `SPDX-License-Identifier: Apache-2.0` (Microsoft, lowRISC and Western Digital notices). The register blocks that PeakRDL generates have no header and fall under `LICENSE`.
- 17 files (the SHA-2 and DOE cores: `src/sha256`, `src/sha512`, `src/sha512_masked`, `src/doe`, among them `sha256_core.v`) carry the two-clause BSD notice of Secworks Sweden AB without an SPDX line: BSD-2-Clause. `sha256_core.v` stands in as the licence file.
- `skidbuffer.v` and `axi_addr.v` (WB2AXIP, Gisselquist Technology) state the Apache License 2.0.
- Two firmware headers, `libs/caliptra_isr/riscv-csr.h` and `riscv-interrupts.h` (five-embeddev), carry `SPDX-License-Identifier: Unlicense` next to the Apache-2.0 line of Caliptra. They are not RTL. `riscv-csr.h` stands in as the licence file.

## Reproduce

```
git clone https://github.com/chipsalliance/caliptra-rtl /path/to/caliptra-rtl
git -C /path/to/caliptra-rtl checkout 49370266d12cb0c4a8f71b3a0ff7e54ba7d4866e
git -C /path/to/caliptra-rtl submodule update --init
python3 -I designs/caliptra/recipe/resolve.py /path/to/caliptra-rtl designs/caliptra \
  hello_world_iccm smoke_test_sha512 smoke_test_mldsa smoke_test_ecc_sign
nix develop -c designs/caliptra/recipe/build-workloads.sh \
  hello_world_iccm smoke_test_sha512 smoke_test_mldsa smoke_test_ecc_sign
```

## File lists

Upstream keeps flat file lists for its flows: `src/integration/config/caliptra_top.vf` (the RTL of `caliptra_top`, the `rtl` target of `compile.yml`) and `caliptra_top_tb.vf` (the Verilator and VCS testbench, which is the first list plus the testbench). They are copied under `src/src/integration/config/`. `resolve.py` resolves their variables the way `tools/scripts/Makefile` does (`CALIPTRA_PRIM_ROOT` is `src/caliptra_prim_generic` with the prefix `caliptra_prim_generic`; `CALIPTRA_AXI4PC_DIR` is `src/integration/tb`, whose `Axi4PC.sv` is empty under Verilator and a deliberate syntax error elsewhere, because the ARM protocol checker has to be acquired from ARM) and writes:

- `variants/core/synth.f`: 579 files of `caliptra_top.vf`, in upstream's order (packages and macro headers first);
- `variants/core/sim.f`: the 45 files that `caliptra_top_tb.vf` adds (SRAM model, assertions, coverage interfaces, testbench, JTAG DPI), then the three C and C++ sources of the model (`test_caliptra_top_tb.cpp`, `jtagdpi.c`, `tcp_server.c`, as `TB_VERILATOR_SRCS` of the Makefile lists them);
- `*-incdirs.txt` (the `+incdir+` lines) and `sim-vlt.txt` (the lint waivers of `caliptra_top_tb.vlt`, once each), which `design.toml` repeats.

The headers that the listed files include are copied too, and the sources of the workloads (see Workloads).

## Synthesis top

`caliptra_top` (`src/integration/rtl/caliptra_top.sv`). Upstream ships no synthesis script; its tool flow is `src/integration/config/compile.yml`, where the `caliptra_top` unit has the `rtl` target with `tops: [caliptra_top]` and the `rtl_lint` and `cdc` targets for it. Everything the SoC integrates sits below it: VeeR EL2, the SHA, HMAC, ECC, AES, DOE, key and PCR vaults, the Adams Bridge ML-DSA and ML-KEM accelerator, the SoC interface with the mailbox, the entropy source and CSRNG, the AXI DMA.

The top has interface ports (AXI subordinate and manager, memory export), which slang accepts with `--allow-toplevel-iface-ports`. The SRAMs of the core (ICCM, DCCM, instruction cache), of the mailbox and of the instruction memory are outside the top (`el2_mem_export`, `abr_memory_export`, `mbox_sram_*`, `imem_*`); the testbench has them (`caliptra_veer_sram_export.sv`, `caliptra_sram.sv`). The `macros` of the variant are the three memory modules that stay inside the top.

Defines: `CALIPTRA_INTERNAL_TRNG` (the Makefile default and a define of the `rtl_lint` target). The `rtl_lint` target also sets `CALIPTRA_AXI_DMA_ADDR_WIDTH=32`; the variant keeps the 48 bits of `config_defines.svh`, as the Verilator flow builds it. Subsystem mode (`CALIPTRA_MODE_SUBSYSTEM`) is another variant of upstream and not used here.

## Constraints

Upstream has no SDC and publishes no frequency, so the clock has no period and the I/O delays no fraction. The resets are the ports `cptra_rst_b` and `cptra_pwrgood`.

## Simulation

Upstream's Verilator flow: the `verilator-build` target of `tools/scripts/Makefile` around `caliptra_top_tb` and its C++ main. `rtl-bench sim` builds the model with the same options (`--cc --exe --timing --timescale 1ns/100ps --autoflush`, the warning waivers of the Makefile variable `suppress`, `-std=c++17`, `+define+RV_OPENSOURCE`), plus `-Wno-fatal`: upstream documents Verilator 5.012, and 5.052 draws warnings that stop a build otherwise. Build on the NAS with `-j 16` at a load of 20 to 50: 425 s wall, 1767 s CPU, 2.5 GB resident at most.

The testbench reads `program.hex`, `iccm.hex`, `dccm.hex` and `mailbox.hex` from the run directory. The Makefile makes them from the ELF file of the test with four `objcopy` commands; `sim.prepare` runs the same commands on the image. The test ends by writing to the mailbox, on which the testbench prints `* TESTCASE PASSED` and `Finished : minstret = N, mcycle = N`; the pass check reads the first, the cycle count the second.

The testbench output has `SVA ERROR: KV[0][0] debug flush failed` lines from the assertions of `src/integration/asserts/caliptra_top_sva.sv` (5441 lines in the 5328 cycles of `hello_world_iccm`). They come from upstream's assertion `KV_debug_comprehensive` under Verilator 5.052 and do not change the result of any test.

## Workloads

Firmware of the test suite of upstream (`src/integration/test_suites/<test>`), built by `recipe/build-workloads.sh` with upstream's own Makefile (`make TESTNAME=<test> program.hex`) and the 32 bit tools of the nix shell (`riscv32-none-elf-gcc` 16.2.0, binutils 2.46, reachable as `riscv32-unknown-elf-*`). The images are the ELF files; `sim.prepare` turns them into the hex files. Differences from a run of the Makefile with upstream's documented toolchain (gcc 12.2):

- `GCC_PREFIX=riscv32-unknown-elf`: the Makefile defaults to `riscv64-unknown-elf`, whose rv32 libraries are not in the nix shell.
- `ABI="-mabi=ilp32 -march=rv32imc_zicsr_zifencei"`: the value that the Makefile chooses for gcc 12.2 and later. Its version test does not read the version line of the nix tools and falls back to an architecture without the CSR extension.
- `BUILD_CFLAGS="-O2 -std=gnu17 -ffile-prefix-map=..."`: the Makefile builds with `-O3`, which gives a larger image with gcc 16 than the 96 KiB that `link.ld` allows for the ROM image (`ERROR: ROM size exceeds 96KiB`), so the later `-O2` wins. `-std=gnu17` is the default of gcc 12; gcc 15 and later default to C23, where `void f();` means `void f(void)` and the declarations in `libs/soc_ifc` no longer match their definitions. The prefix maps keep the build paths out of the debug information. `PLAYBOOK_RANDOM_SEED=1` replaces the date in `-DMY_RANDOM_SEED` (the seed of the test yml).
- `TEST_LIBS=-L$RISCV_NEWLIB_RV32IMC`: the firmware uses `memcpy`, `strlen`, `strstr` and `rand`. The nixpkgs newlib for 32 bit RISC-V is built for ilp32d; the nix shell builds one for rv32imc and ilp32 (`flake.nix`).

Two builds give identical files.

| test | what it runs | cycles | wall time | resident |
|---|---|---|---|---|
| `hello_world_iccm` | ROM flow, a print from the ICCM | 5328 | 4 s | 20 MB |
| `smoke_test_sha512` | SHA-512 engine | 65744 | 42 s | 20 MB |
| `smoke_test_mldsa` | ML-DSA-87 (Adams Bridge): key generation, signing, verification | 363965 | 218 s | 67 MB |
| `smoke_test_ecc_sign` | ECC secp384r1 signing | 800313 | 496 s | 141 MB |

Times and resident sizes of the run of the model (the build is above), on the NAS under other load.

| image | sha256 |
|---|---|
| `sim/hello_world_iccm.exe` | `5761598fbcb00ada664dcf4b5b2af8d8341db4d6150fda2bf99d2f040460410c` |
| `sim/smoke_test_sha512.exe` | `ce739ecebc4b5552debcf90af504398137719948c0642d275bbcf1dfa37bdfb5` |
| `sim/smoke_test_mldsa.exe` | `ff03beb6486d0c73f5884ab8f1852b38a18b4ba1f777e730f46d489d215399b2` |
| `sim/smoke_test_ecc_sign.exe` | `f4483aae50a8aa82252a0ade0057cfd9eaa109c2281313cdbc006d1433c0866d` |

The firmware sources (`libs/*`, `includes/`, the test directories) are copied by `resolve.py`: the directories of the tests are listed as `sources` of their workloads, the libraries that every image links are in `recipe/unused.txt`.

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1, gcc and binutils for `riscv32-none-elf` 16.2.0 and 2.46, and a newlib for rv32imc and ilp32 built from the nixpkgs recipe.
