# Recipe of wally

How `src/`, `variants/rv64gc/` and `sim/` were made. Nothing under `src/` is edited; every file is a copy of the pinned upstream commit at the same relative path (`src/src/cvw.sv` is upstream `src/cvw.sv`), except `src/generated/`.

Upstream: https://github.com/openhwfoundation/cvw, commit `90259bdca99c187eb1d482837d54f5c618884ef2` (main, 2026-11-10).

Licence: upstream `LICENSE` states `SPDX-License-Identifier: Apache-2.0 WITH SHL-2.1` (Solderpad Hardware License v2.1 or the Apache License 2.0, at the user's option). Most vendored source files carry the same line. Five files (`src/generic/binencoder.sv`, `src/generic/decoder.sv`, `src/generic/priorityonehot.sv`, `src/mmu/hptw.sv`, `src/privileged/privileged.sv`) carry an MIT-style permission notice with the same copyright holders instead, and no SPDX line. The file `LICENSE` holds the notice and a link, not the licence text.

## Reproduce

```
git clone https://github.com/openhwfoundation/cvw /path/to/cvw
git -C /path/to/cvw checkout 90259bdca99c187eb1d482837d54f5c618884ef2
nix develop -c designs/wally/recipe/resolve.sh /path/to/cvw
nix develop -c designs/wally/recipe/build-workloads.sh
```

## Synthesis top

Upstream synthesises `wallypipelinedcore` (the core without the peripherals): `synthDC/wallySynth.py` runs `make synth DESIGN=wallypipelinedcore CONFIG=<config>`, and `synthDC/Makefile` copies `config/shared/*.vh` and `config/<config>/config.vh` for the run. The README of upstream, section Configurations, says the same: "Synthesis is of `wallypipelinedcore`". The core takes a parameter struct `P`, so `synthDC/scripts/synth.tcl` generates a wrapper that binds `P` to the configuration:

```
WALLY=/path/to/cvw python3 synthDC/scripts/wrapperGen.py wallypipelinedcore designs/wally/src/generated
```

The result is `src/generated/wallypipelinedcorewrapper.sv`, the synthesis top `wallypipelinedcorewrapper`. It includes `config.vh` and `parameter-defs.vh` (the include directories of the variant). The simulation leaves it out, because the testbench includes `config.vh` itself in the same compilation unit.

## File lists

Upstream's flows read every `.sv` under `src/`. `closure.py` finds the files that a top needs instead: the top's file plus the file of every module instantiated by name, followed transitively, in any generate branch (Verilator and slang need the definition of a module that sits in a branch that the configuration does not take) and with no macro defined. The package `src/cvw.sv` comes first.

- `variants/rv64gc/synth.f`: closure of `wallypipelinedcore` (195 upstream files besides the package), the package and the generated wrapper.
- `variants/rv64gc/sim.f`: the files that the closure of `testbench` adds (`wallypipelinedsoc`, the uncore, the testbench and its helpers) and the DPI wrapper `sim/verilator/wrapper.c`.

Besides the lists, the script copies `LICENSE`, `config/shared/*.vh`, `config/rv64gc/config.vh`, and the run-time files of the simulation (`testbench/Makefile`, `testbench/mempipeprobe.svh`, `bin/elf2hex`, `bin/extractFunctionRadix.sh`) and the sources of the workloads. Other files of `config/rv64gc/` (coverage and ACT configuration) are not needed.

## Simulation

Upstream's Verilator flow is `sim/verilator/Makefile` driven by `bin/wsim`. `rtl-bench sim` builds the same model with the same options (`--binary --assert --no-trace-top --relative-includes`, `OPT_FAST=-O2 OPT_SLOW=-O1 OPT_GLOBAL=-O2`) and runs it as `wsim` does: `+TEST=<name> +ElfFile=<elf>`. Differences:

- `-Wno-fatal`: the testbench parameter `PrintHPMCounters=1`, which prints the cycle count (`Cnt[ 0] = N Mcycle`) at the end of a test, makes `testbench/common/loggers.sv:39` draw WIDTHEXPAND and WIDTHTRUNC warnings.
- At run time the testbench calls `make -f $WALLY/testbench/Makefile`, which calls `elf2hex` (uses `uv run --script` and `objcopy`) and `extractFunctionRadix.sh` (shebang `/bin/bash`). `WALLY` is set to `src/`, `src/bin` is appended to `PATH`, and the nix shell supplies a small wrapper for `extractFunctionRadix.sh` because NixOS has no `/bin/bash`.
- A test reports its result by storing to `tohost`; the testbench ends with `SUCCESS! All tests ran without failures.` and Verilator exits with 0 either way, so the pass check reads the output.

## Workloads

`ieu` and `fpu` are directed self-checking tests of upstream's regression: `tests/coverage64gc/ieu.S` and `fpu.S` (the `make coverage` target of the top-level Makefile). Each checks its results and stores 1 to `tohost` on success.

`build-workloads.sh` runs the commands of `tests/coverage64gc/Makefile` (`gcc -E`, `as`, `gcc` with `examples/link/link.ld`) with the toolchain of the nix shell: `riscv64-none-elf-gcc` 14.3.0 and binutils 2.44 from nixpkgs (revision in `flake.lock`, `pkgsCross.riscv64-embedded`), reachable under the names `riscv64-unknown-elf-*` that upstream uses. One addition: `as --debug-prefix-map "$PWD=."`, so the images do not carry the build path.

| image | sha256 |
|---|---|
| `sim/ieu.elf` | `473de26fcd19b4a9692dedcccb52cd4bc11b05bdf0496421695b89ddeefc7836` |
| `sim/fpu.elf` | `2cd7b382937338d337b0c772f3bf6bc9833ac49182d5fbb8f8956d31c7f7cc25` |

## Not used

Upstream's riscv-arch-test (ACT) programs are built by `make act` with `uv`, the ACT framework and the Sail reference model; they are not part of this variant yet.
