# Recipe of opentitan

How `src/`, `variants/earlgrey/`, `constraints/` and `sim/` were made. Nothing under `src/` is edited; every file is a copy from the pinned commit at the same relative path, except `src/generated/`.

Upstream: https://github.com/lowRISC/opentitan, commit `96c62c527b18ff7b2088063866f4f48b63b66fb0` (master, 2026-10-21). It is the commit of the release `devbundle-2026-10-21-1`, which ships the boot ROM and the OTP images used here (see Simulation).

## Licences

`LICENSE` is the full Apache License 2.0; `NOTICE` is kept. 797 of the vendored files carry `SPDX-License-Identifier: Apache-2.0`. Third-party code, as OpenTitan imports it under `hw/vendor` (with its local patches; the revision is in the lock file):

| code | upstream revision | licence |
|---|---|---|
| `hw/vendor/lowrisc_ibex` | lowRISC/ibex `38c070939183cc10940b66c8e9e04eeca6b65470` | Apache-2.0 (`LICENSE`, `NOTICE`, `CREDITS.md`) |
| `hw/vendor/pulp_riscv_dbg` | pulp-platform/riscv-dbg `358f90110220adf7a083f8b65d157e836d706236` | Solderpad Hardware License 0.51 (ETH Zurich and University of Bologna; the header of its files, `LICENSE`; the full text is `licences/SHL-0.51.txt`, fetched by `recipe/fetch-licences.sh` from solderpad.org); `debug_rom` from SiFive: Apache-2.0 (`LICENSE.SiFive`) |

Files of OpenTitan that carry other copyright lines: six files of `hw/ip/usbdev/rtl` (`usb_fs_*.sv`, "Copyright Luke Valenty (TinyFPGA project)") and `hw/ip/prim/dv/prim_prince/crypto_dpi_prince/prince_ref.h` ("Copyright 2016 Sebastien Riou"); both have the OpenTitan Apache-2.0 SPDX line. `hw/dv/dpi/usbdpi/usb_crc.c` (a C routine of the simulation, "Ported by Ron Hemphill" from a USB white paper) has no licence statement at all; it is in the simulation list only. It is kept: the Apache-2.0 `LICENSE` of the repository covers it.

## File lists

OpenTitan's tools resolve the file lists with FuseSoC (the lowRISC fork `ot-0.5.dev0` and edalize `v0.4.0`, as `python-requirements.txt` pins them). `recipe/resolve.sh` runs the same commands in a copy of the checkout:

- synthesis: `lowrisc:systems:chip_earlgrey_asic:0.1`, target `syn`, flag `fileset_top` (the command of `hw/syn/tools/dvsim/common_syn_cfg.hjson`);
- simulation: `lowrisc:dv:chip_verilator_sim:0.1`, target `sim`, tool verilator (the build of `hw/dv/tools/dvsim/verilator.hjson`, `hw/top_earlgrey/dv/verilator`).

The cores root of the copy lacks three directories: `hw/ip/prim_xilinx` and `hw/ip/prim_xilinx_ultrascale` (FPGA technology libraries), and `hw/top_englishbreakfast` (the other chip, whose cores have the same names as the Earl Grey cores). Without the Xilinx cores, the generator `primgen` (a FuseSoC generator, `hw/ip/prim/util/primgen.py`) writes the abstract primitives (`prim_flop`, `prim_rom`, `prim_flash`, ...: 22 files in `src/generated/`, one core each) with the generic implementation only, so `prim_xilinx*` is neither in the list nor named in the generated RTL. `PRIM_DEFAULT_IMPL` defaults to `prim_pkg::ImplGeneric`. Everything else that the list names is checked in: the register tops, the crossbar, `top_earlgrey` and `chip_earlgrey_asic` are under `hw/top_earlgrey/rtl/autogen` and `hw/top_earlgrey/ip_autogen`.

- `variants/earlgrey/synth.f`: 652 files; `*-defines.txt`, `*-incdirs.txt` and `sim-edalize-config.mk.txt` are the defines, include directories and edalize options that FuseSoC wrote.
- `variants/earlgrey/sim.f`: 772 files (waivers `.vlt` first, then RTL, then the DPI C files and the C++ main).
- The headers of the include directories and the files that the listed ones include are copied too (`recipe/includes.py`); `uvm_macros.svh` and `formal_tb_frag.svh` are not (guarded by `UVM` and `FORMAL`).

## Synthesis top

`chip_earlgrey_asic` (`hw/top_earlgrey/rtl/autogen/chip_earlgrey_asic.sv`): upstream's ASIC synthesis flow (`hw/top_earlgrey/syn/chip_earlgrey_asic_syn_cfg.hjson`: `name: chip_earlgrey_asic`, `fusesoc_core: lowrisc:systems:chip_earlgrey_asic:0.1`, with `top_earlgrey` expanded inside it) synthesises the chip with the pad ring and the AST. The GTECH flow (`top_earlgrey_gtech_syn_cfg.hjson`) synthesises `top_earlgrey` alone, without pads and AST, as a technology-independent check; it is not used here. `chip_earlgrey_verilator` is the simulation chip (a different pad ring, `padring_verilator.sv`, and clocks from a divider).

## Memories and analog blocks

The abstract primitives resolve to `prim_generic`: `prim_rom`, `prim_ram_1p`, `prim_ram_2p`, `prim_ram_1r1w`, `prim_otp`, `prim_flash` (with `prim_generic_flash_bank`) and `prim_pad_wrapper`. The analog top is `ast` in `hw/top_earlgrey/ip/ast`, upstream's behavioural model (`lowrisc:systems:ast`; a partner core can replace it with `fileset_partner`). They are the `macros` of the variant.

## Constraints

Upstream ships an SDC for the chip: `hw/top_earlgrey/syn/chip_earlgrey_asic.sdc`, with its setup script `ot.sdc_setup.tcl`; both are in `constraints/`, unmodified. It defines the clocks `MAIN_CLK`, `USB_CLK`, `IO_CLK`, `AON_CLK` (pins `u_ast/clk_src_*_o`), `JTAG_TCK` (port `IOR3`), `AST_EXT_CLK` (`IOC6`) and `SPI_DEV_CLK`, the SPI and TPM clocks derived from the last, and the I/O constraints. Its periods are targets for a generic process (main 100 MHz, over-constrained by a factor), which is not an open PDK, so the clocks have no `period_ps`.

## Simulation

Upstream's Verilator chip target: `chip_sim_tb` around `chip_earlgrey_verilator`, with the DPI models of UART, GPIO, SPI, USB, JTAG and DMI and the memory loader (`hw/dv/verilator`). The options are those of the sim target (`sim-edalize-config.mk.txt`), with differences: no `--trace` (upstream's choice to trace costs build time, the run does not need it), and `--no-timing` (the AST models have delays, which Verilator 5.052 refuses without `--timing` or `--no-timing`). The model runs with 4 threads, as upstream sets it. Build, measured on the NAS with `-j 16` at a load of about 20: 78 s wall (Verilator 77 s, of which 42 s conversion and 24 s C++ build), 389 s CPU, 2.3 GB resident at most; 189 MB of generated C++ in 167 files.

The sim target of upstream defines `RVFI` (`RVFI=true` in `chip_sim.core`), which makes the Ibex tracer write `trace_core_00000000.log`, a few MB per 200k cycles and without bound (about 190 MB per 10 M cycles). The RTL has the tracer behind `ifdef RVFI` (`rv_core_ibex.sv`), so the variant leaves the define out and no trace file is written. The UART text goes to `uart0.log`, which the pass checks read; the simulation prints its cycle count at the end.

### Boot ROM and OTP image

`sim/test_rom_sim_verilator.39.scr.vmem` (scrambled for this RTL), its `.elf` and `.map`, and `sim/img_rma.24.vmem` (and `img_dev.24.vmem`, unused) come from the release `devbundle-2026-10-21-1` of OpenTitan (https://github.com/lowRISC/opentitan/releases/download/devbundle-2026-10-21-1/devbundle.tar.xz, sha256 `789b40789b3dc9c76087a00c89f1fc71acd22939c51ae39429e2379b7c41c702`; files `earlgrey/test_rom/test_rom_sim_verilator.*` and `earlgrey/otp/img_*.24.vmem`). They are the ROM and OTP image of the `sim_verilator` environment (`hw/top_earlgrey/BUILD`: rom `//sw/device/lib/testing/test_rom:test_rom`, otp `//hw/ip/otp_ctrl/data:img_rma`). The bundle has no test programs, only the images for FPGA, the DV ROM and this ROM, so the tests below are built (not downloaded).

| file | sha256 |
|---|---|
| `sim/test_rom_sim_verilator.39.scr.vmem` | `c52fd13a6d545459374ecc32ddbc1f153e80517e7b5eedd72876090b64115670` |
| `sim/test_rom_sim_verilator.elf` | `f7bb6d787b9c1daee17723822333331766f7abf81565d6fb87d4840162d3ff2f` |
| `sim/img_rma.24.vmem` | `d296d08495125b61f7d3778bad7c20e22992a16277e7a46a23684a1738b62502` |
| `sim/img_dev.24.vmem` | `7fa6a5af67c9fb90c4dc1c55e8463bceb68894a5d3b981afaff7305d0352255b` |

### Tests

`uart_smoketest`, `crt_test` and `otbn_smoketest` are OpenTitan's own tests (`sw/device/tests`, OpenTitan Test Framework), built by `recipe/build-workloads.sh` with OpenTitan's Bazel build: Bazel 8.7.0 (`.bazelversion`) through `bazelisk.sh`, the hermetic lowRISC rv32imcb clang toolchain and the dependencies that `MODULE.bazel` pins, downloaded at build time. The script appends plain `opentitan_binary` targets (`recipe/gen_bins.py`) with the sources and dependencies of the `opentitan_test` targets for the environment `//hw/top_earlgrey:sim_verilator_base`, because the test rule would also build the simulation model; the images are the flash images for that environment. Bazel runs in a FHS environment (`otfhs` of the nix shell), as its toolchains are dynamically linked and its actions use `/bin/bash` and `srec_cat`, with `LD_LIBRARY_PATH=/lib:/usr/lib`. Two builds give identical files.

| image | sha256 |
|---|---|
| `sim/uart_smoketest_sim_verilator.64.vmem` | `ca9fa3b26216ca0ff91ce7735b421bc418a3648e1320b4c90cc6cb1cc0308f82` |
| `sim/crt_test_sim_verilator.64.vmem` | `c88cdba95a990e2a444e33741d7b796f2ec86227501f3e6d91c46718101051f7` |
| `sim/otbn_smoketest_sim_verilator.64.vmem` | `7c5b2bd7b8a61345c27d71f9e9e50c67f07e42fa857a5b9b52ded61c84179465` |

The sources of the tests (`sw/device/tests/*.c` and the device libraries) are not vendored. Of the crypto smoke tests built (aes 0.43 M, kmac 0.48 M, hmac 0.69 M, otbn 1.0 M cycles), `otbn_smoketest` is the longest.

## Tools

Pinned by `flake.lock`: Verilator 5.052, slang 12.1; FuseSoC and edalize as above, installed by `resolve.sh` into a temporary virtual environment.
