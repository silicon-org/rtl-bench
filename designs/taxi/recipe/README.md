# Recipe of taxi

How `src/`, `variants/` and the tests were made. Nothing under `src/` is edited: every file is a copy from the pinned commit at the same relative path under `src/taxi/`, and the symbolic link of upstream (`baser.py` of the test) is kept as a link.

| repository | commit | licence (file) |
|---|---|---|
| https://github.com/fpganinja/taxi | `cc70b270b910d369ab1ad7b3855e76399fd461f1` (master, 2026-08-28) | CERN-OHL-S-2.0 (`src/taxi/LICENSE`); MIT for the interfaces, two stream helpers and the example design |

CERN-OHL-S-2.0 is strongly reciprocal: the source must be given on request with the licence kept, which a vendored copy with `LICENSE` does. The files under MIT carry only the line `SPDX-License-Identifier: MIT`; the repository has no MIT text. They are `taxi_axis_if.sv`, `taxi_axil_if.sv`, `taxi_axis_null_src.sv`, `taxi_axis_tie.sv`, and `fpga_core.sv`, `Makefile` and `test_fpga_core.py` of the KC705 example. `taxi_axis_if.sv` stands in as the licence file of MIT in `design.toml`.

## Reproduce

```
git clone https://github.com/fpganinja/taxi /path/to/taxi
git -C /path/to/taxi checkout cc70b270b910d369ab1ad7b3855e76399fd461f1
nix develop -c python3 -I designs/taxi/recipe/vendor.py /path/to/taxi designs/taxi
```

`recipe/vendor.py` copies the files, writes `variants/<id>/synth.f` and `sim.f`, and writes `recipe/unused.txt`.

## File lists

Upstream lists the files of a module in `.f` files (`src/<area>/rtl/<module>.f`), which name other `.f` files and files of other areas through `../lib/taxi/src/...` (`lib/taxi` is a link to the repository root). The Makefile of every test expands them with `process_f_files` and keeps the last file of each name in order (`uniq_base`). `vendor.py` does the same, with the links resolved, so `variants/<id>/synth.f` is the `VERILOG_SOURCES` of the test Makefile of that variant. The `.f` files are copied too and listed in `unused.txt`; the `lib/taxi` links are not copied (a link to the root would be a loop).

## Variants

Both variants use the same view for synthesis and simulation: the top, the file list and the parameters are those of the Makefile of upstream's test.

- `eth_phy_10g`: `taxi_eth_phy_10g`, the DUT of `src/eth/tb/taxi_eth_phy_10g` (parameters of its Makefile: 64-bit datapath, no gearbox interface, PRBS31 enabled). Plain ports, no interfaces inside.
- `kc705_gmii`: `fpga_core` of `src/eth/example/KC705/fpga`, the DUT of its test with the parameters of its Makefile (`SIM = 1`, `VENDOR = "XILINX"`, `BASET_PHY_TYPE = "GMII"`). Three 1G MACs, XFCP over UART, statistics and I2C master, joined by SystemVerilog interfaces (`taxi_axis_if` instances and arrays of them).

Not variants:

- The MAC tops (`taxi_eth_mac_phy_10g`, `taxi_eth_mac_10g`, the AXI crossbars, the cndm NIC cores) have interface ports. A top with an interface port needs an enclosing module that instantiates the interface; upstream does that only in the portless test wrappers (`test_*.sv`), whose design has no input or output. slang says `top-level module ... has unconnected interface port`, and with `--allow-toplevel-iface-ports` the default interface parameters fail the `$fatal` checks of the modules.
- The board cores of the cndm NIC and of the 10G/25G/100G examples include the transceiver wrappers of UltraScale (`taxi_eth_mac_25g_us`, `taxi_eth_phy_*_us_gt`) and the PCIe hard block interface of UltraScale.
- The board shells (`fpga.sv`): pads and clocking primitives of the board.

## Vendor primitives

With `SIM = 1`, which the tests of upstream set to avoid them, the primitives of the I/O library (`taxi_iddr`, `taxi_oddr`, `taxi_ssio_*`, `taxi_gmii_phy_if`: `IDDR`, `ODDR`, `BUFIO`, `BUFR`, `BUFGMUX`) sit in generate branches that do not elaborate; a generic implementation is used.

## Tests

Upstream's tests are cocotb Python (`test_*.py` in `tb/`), run with the cocotb makefiles under Verilator. `sim.cocotb` of `design.toml` runs the unmodified test module; every workload names the test functions of the module that it runs. Versions of cocotb and the extensions are those of `tox.ini` of upstream (cocotb 2.0.1, cocotb-bus 0.3.0, cocotb-test 0.2.6, cocotbext-axi and cocotbext-eth 0.1.28, cocotbext-i2c 0.1.2, cocotbext-uart 0.1.4, scapy 2.6.1); the flake builds them as `COCOTB_ENV`.

`test_taxi_eth_phy_10g.py` also runs with a 32-bit datapath, with the gearbox interface and with other options under `pytest`; the variant is the single configuration of the Makefile.

## Known tool gap

slang 12.1 does not take `$size` of an array of interface instances as a constant (`'x' cannot be used in an expression`), which `fpga_core.sv`, `taxi_eth_mac_stats.sv` and `taxi_xfcp_mod_stats.sv` do (`.PORTS($size(xfcp_sw_us))`). `rtl-bench elaborate taxi.kc705_gmii` fails with it; Verilator elaborates the files. slang also needs `--timescale` for the files of the interfaces, which have none while the others do.
