# rtl-bench

Open RTL benchmark designs for synthesis, place-and-route and simulation tools.

Each design is real-world RTL from its upstream repository, pinned to a commit and kept as upstream wrote it: no patches, no conversions, no stripped constructs. Each variant comes with:

- a **synthesis view**: top, files, include directories, defines and memory macros;
- **constraints**: clocks, I/O delays, resets and false paths in TOML, with the SDC generated from them, plus any SDC that upstream ships;
- a **simulation view**: a testbench, prebuilt workloads and a pass check, so that every variant runs, for example to record switching activity for power analysis.

The release is a tarball. Tools pin it by hash.

## Designs

<!-- designs:begin -->
| Design | Variant | Upstream | Licence | Source |
|---|---|---|---|---|
| BlackParrot | unicore | [black-parrot/black-parrot](https://github.com/black-parrot/black-parrot) at `909619a68f35` | BSD-3-Clause AND SHL-0.51 | vendored |
| SonicBOOM | smallv3 | [ucb-bar/chipyard](https://github.com/ucb-bar/chipyard) at `157dd69e49b9` | BSD-3-Clause AND Apache-2.0 AND BSD-2-Clause | vendored |
| OpenTitan Earl Grey | earlgrey | [lowRISC/opentitan](https://github.com/lowRISC/opentitan) at `96c62c527b18` | Apache-2.0 AND SHL-0.51 | vendored |
| Vortex | default | [vortexgpgpu/vortex](https://github.com/vortexgpgpu/vortex) at `ca18837bd087` | Apache-2.0 | vendored |
| CORE-V Wally | rv64gc | [openhwfoundation/cvw](https://github.com/openhwfoundation/cvw) at `90259bdca99c` | (Apache-2.0 WITH SHL-2.1) AND MIT AND Apache-2.0 | vendored |
<!-- designs:end -->

Generated from `designs/*/design.toml` by `rtl-bench table`.

## Layout

```
designs/<design>/
  design.toml     source, licence and variants (schema: docs/schema.md)
  src/            upstream files as they are (empty for designs included by reference)
  recipe/         how src/ was produced: file-list resolution, generator runs and their options
  sim/            testbench files, workload images, expected results
  constraints/    SDC shipped by upstream, unmodified
```

## Rules

- **Upstream as is.** Sources are copied from the pinned commit without change. Generated RTL (Chisel, register tops, crossbars) is the output of a recorded generator run, with its version and options in `recipe/`. A design that a tool cannot read is a gap in that tool, not in the package.
- **Licences from the files.** A design is vendored when its licence file allows redistribution; the file is kept next to the sources and its SPDX id is recorded. A design without a licence that allows redistribution is included by reference: `design.toml` names the repository, commit and files, and `rtl-bench fetch` gets them.
- **Every variant runs.** CI runs every workload of every variant in Verilator and checks its pass condition, and elaborates every synthesis view with slang.
- **Every number has a source.** A clock period, an I/O delay or a cycle count names where it comes from: the upstream SDC or a published paper; a clock period is given only for open PDKs (`docs/constraints.md`).
- **Tool-neutral.** No results of any particular implementation tool live here.

## Use

```
nix develop
rtl-bench check                 # schema, licences, file lists, the Designs table
rtl-bench sim <variant> <workload>
rtl-bench elaborate <variant>   # slang on the synthesis view
rtl-bench sdc <variant> --technology <name>
rtl-bench table                 # regenerate the Designs table
rtl-bench package               # release tarball
```

## Licence

The tooling and metadata of this repository are under the Apache License 2.0 (`LICENSE`). Each design stays under its own licence, named in its `design.toml` and kept in its directory.
