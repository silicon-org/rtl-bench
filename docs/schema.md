# design.toml

One file per design. Paths are relative to the design directory. Unknown keys are an error.

```toml
[design]
id = "blackparrot"                       # directory name, lower case
name = "BlackParrot"
description = "Linux-capable RISC-V multicore."

[source]
repository = "https://github.com/black-parrot/black-parrot"
commit = "<40 hex digits>"
mode = "vendored"                        # vendored | reference
licence = "BSD-3-Clause"                 # SPDX expression, read from the licence file
licence_files = ["src/LICENSE"]          # kept next to the sources (vendored only)

# Further upstream repositories that the sources come from (submodules, libraries).
[[source.dependency]]
repository = "https://github.com/bespoke-silicon-group/basejump_stl"
commit = "<40 hex digits>"
licence = "BSD-3-Clause"
licence_files = ["src/basejump_stl/LICENSE"]

[[variant]]
id = "unicore"                           # full id: <design>.<variant>
description = "One core, L2 slice, default configuration."

[variant.synth]
top = "bp_unicore"
language = "sv-2017"                     # sv-2017 | verilog-2005
files = "variants/unicore/synth.f"       # one path per line, in compilation order
include_dirs = ["src/bp_common/src/include"]
defines = { BP_CFG = "e_bp_unicore_cfg" }
parameters = { }                         # top-level parameter overrides
# Memories kept as macros: the module name pattern and the behavioural model that simulation uses.
macros = [{ module = "bsg_mem_1rw_sync_mask_write_bit*", model = "src/basejump_stl/bsg_mem/bsg_mem_1rw_sync_mask_write_bit.sv" }]

[[variant.clock]]
name = "core_clk"
port = "clk_i"
period_ps = { asap7 = 1000, sky130 = 10000 }
source = "docs/constraints.md, scaling rule 1, from the 1 GHz figure of the upstream paper"

[variant.constraints]
input_delay_fraction = 0.2               # of the period of the clock that launches the input
output_delay_fraction = 0.2
resets = ["reset_i"]
false_paths = [{ from = "reset_i" }]
sdc = []                                 # upstream SDC files, unmodified, if any

[variant.sim]
simulator = "verilator"
top = "testbench"
files = "variants/unicore/sim.f"         # testbench files; the synthesis files are added first
defines = { }
dut = "testbench.wrapper.dut"            # instance path of the synthesis top

[[variant.sim.workload]]
id = "coremark"
image = "sim/coremark.riscv"             # prebuilt, with the recipe in recipe/
pass = { kind = "exit_code", value = 0 } # exit_code | uart | signature
max_cycles = 20000000
activity = { start = "roi_begin", stop = "roi_end" }   # markers for switching activity, optional
```

## Rules checked by `rtl-bench check`

- Every listed file exists; every file under `src/` is listed by some view or by `recipe/unused.txt` with a reason.
- `licence` matches the licence files; a vendored design without a licence file is an error.
- Every clock period has a `source`.
- A `reference` design has no files under `src/`.
