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
files = "variants/unicore/synth.f"       # one path per line (blank lines and # comments allowed), in compilation order
include_dirs = ["src/bp_common/src/include"]
defines = { BP_CFG = "e_bp_unicore_cfg" }
slang_flags = []                         # further slang options of `rtl-bench elaborate`, with the reason in a comment
single_unit = false                      # true: one compilation unit for slang, so listed include files apply to the files after them
parameters = { }                         # top-level parameter overrides
# Memories kept as macros: the module name pattern and the behavioural model that simulation uses.
macros = [{ module = "bsg_mem_1rw_sync_mask_write_bit*", model = "src/basejump_stl/bsg_mem/bsg_mem_1rw_sync_mask_write_bit.sv" }]

[[variant.clock]]
name = "core_clk"
port = "clk_i"
period_ps = { asap7 = 1000, sky130 = 10000 }   # optional; keys: asap7, sky130, gf180mcu, ihp-sg13g2, freepdk45 (docs/constraints.md)
source = "docs/constraints.md, rule 2: the 1 GHz figure of the upstream paper for these PDKs, page 3"

[[variant.clock.reference]]               # optional: a frequency published for any process
frequency_mhz = 920
process = "as the source writes it"
source = "where it is published"

[variant.constraints]
input_delay_fraction = 0.2               # optional, like period_ps; of the period of the first clock, which launches the inputs
output_delay_fraction = 0.2              # and captures the outputs
source = "docs/constraints.md, rule 1"   # where the two fractions come from (mandatory)
resets = ["reset_i"]                     # no input delay on resets
false_paths = [{ from = "reset_i" }]
sdc = []                                 # upstream SDC files, unmodified, if any

[variant.sim]
simulator = "verilator"
top = "testbench"
files = "variants/unicore/sim.f"         # testbench files; the synthesis files are added first
verilator_flags = []                     # Verilator options of this variant (a .vlt file goes here, before the sources), with the reason in a comment;
                                         # ${NAME} is replaced by an environment variable and ${DESIGN_DIR} by the absolute design directory;
                                         # --exe: the design has its own C++ main (built with --cc --build instead of --binary)
standalone = false                       # true: the simulation lists all its files in `files` and takes none from the synthesis view
include_dirs = []                        # include directories of the simulation, after those of the synthesis view
undefine = []                            # names of synthesis defines that the simulation does not inherit
exclude = []                             # synthesis files that the simulation leaves out (a generated synthesis wrapper)
defines = { }                            # value "" defines a macro without a value
parameters = { }                         # parameter overrides of the simulation top
dut = "testbench.wrapper.dut"            # instance path of the synthesis top
support = []                             # files needed at run time: scripts, makefiles, headers
path = []                                # directories appended to PATH when a workload runs
env_dirs = { }                           # variable = directory of the design, set to its absolute path
prepare = []                             # commands run in the run directory first, as word lists; for image conversion
args = ["+TEST={workload}", "+ElfFile={image}"]   # arguments of the run; in args and prepare, {workload}, {image} (the copy of the image in the run directory) and {dir} (the run directory) are replaced
cycles_pattern = "Cycles: ([0-9]+)"      # optional regular expression with one group: the cycles in the output
uart_file = "uart0.log"                  # file of the run directory with the UART text, for `uart` pass checks
cycles_file = "perf.trace"               # optional: a file of the run directory that cycles_pattern reads instead of the output

[[variant.sim.workload]]
id = "coremark"
image = "sim/coremark.riscv"             # prebuilt, with the recipe in recipe/
sources = ["src/coremark/core_main.c"]   # upstream files that the image is built from (optional)
pass = { kind = "exit_code", value = 0 } # see below
max_cycles = 20000000                    # optional: a limit for the cycle count; without it the count is not read
args = []                                # further arguments of this run, after those of the variant
timeout_s = 600                          # wall-clock limit of a run; needed when there is no max_cycles
activity = { start = "roi_begin", stop = "roi_end" }   # markers for switching activity, optional
```

## Pass checks

| `kind` | keys | passes when |
|---|---|---|
| `exit_code` | `value` | the simulator exits with `value` |
| `stdout` | `contains` | the simulator exits with 0 and its output contains `contains` |
| `uart` | `contains` | the simulator exits with 0 and the file `sim.uart_file` of the run directory contains `contains` |
| `signature` | `reference` | the signature equals the reference file (parsed; `rtl-bench sim` does not run it yet) |

With `cycles_pattern`, `rtl-bench sim` also reads the cycle count from the output (or from `cycles_file`) and fails above `max_cycles`.

`period_ps` and the two delay fractions are optional. `rtl-bench sdc` fails with a message that names the clock or the missing key when one is not set; a variant without periods needs no `source` for them.

## Package

`rtl-bench package --version <x.y.z> --out <dir>` writes `rtl-bench-v<x.y.z>.tar.gz`. It runs `check` first and refuses a git tree that is not clean. The archive holds the tracked files of `designs/`, `docs/`, the tool sources (`rtl-bench/`, `Cargo.toml`, `Cargo.lock`, `flake.nix`, `flake.lock`) and the top-level documents, under the folder `rtl-bench-v<x.y.z>/`, plus:

- `VERSION` and `SOURCE_COMMIT` (the git commit),
- `MANIFEST.toml`: one `[[variant]]` per variant with `id`, `design`, `top`, `licence`, the paths of the synthesis file list, include directories, macros, constraints and workloads (relative to the package root), so a consumer needs no tool.

The bytes depend only on the commit and the version: entries are sorted by path, owner and group are 0, the time of every entry is the commit time, and gzip has no name and no time. A push of a tag `v*` builds the package in CI and attaches it to a GitHub release.

## Rules checked by `rtl-bench check`

- Every listed file exists; every path is relative to the design directory and stays inside it.
- Every file under `src/` is listed by some view or by `recipe/unused.txt` (`<path> <reason>` per line). A view lists a file by its file lists, `macros`, `support`, `sdc`, workload `image` and `sources`, licence files, or by being directly in one of its `include_dirs`.
- `licence` matches the licence files: an `SPDX-License-Identifier:` line in a licence file decides; without one, the text of Apache-2.0, MIT, BSD-2-Clause, BSD-3-Clause and ISC is recognised. A vendored design without a licence file is an error. For an expression with `AND`, `OR` or `WITH`, every part must be stated by some licence file (`A WITH B`: by a line with exactly that expression, or by the texts of A and B); a source file that carries the notice can serve as the licence file of `MIT`. The `licence` of `[source]` covers everything vendored: every part of every dependency licence must be a part of it, and the licence files of the dependencies count as its licence files.
- `period_ps` has only the keys of open PDKs; a `reference` record has a frequency above 0, a process and a source; a workload has `max_cycles` or `timeout_s`.
- Every clock that has a period has a `source`, and a variant that sets a delay fraction has a `constraints.source`.
- A `reference` design has no files under `src/`.
