#!/usr/bin/env python3
"""Resolve the file lists of occamy.full and occamy.q2c2 and copy the files from the generator trees.

usage: python3 -I resolve.py <workdir of generate.sh> <design directory>

For each variant, `bender script flist-plus` of the generated tree gives the files, include directories and
defines of the synthesis view and of the simulation view. The files are copied to src/ without change:
  dependency checkouts       .bender/git/checkouts/<name>-<hash>/...  ->  src/deps/<name>/...
  tracked files of occamy    ...                                      ->  src/occamy/...
  generated files (per variant, not tracked)                          ->  src/generated/<variant>/...
"""
import os
import re
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

VARIANTS = ["full", "q2c2"]
# The bender targets of the file lists (recipe/README.md). Synthesis: no simulation or test target, so the
# generic tech_cells_generic (behavioural tc_sram, tc_clk) and the latch based scm are listed. Simulation: the
# targets of upstream's VSIM_BENDER (occamy Makefile) with `verilator` and `simulation`, without `test`.
SYNTH_TARGETS = ["rtl", "synthesis", "cv64a6_imafdc_sv39", "occamy_sim", "snitch_cluster"]
# No `simulation` target: it adds the class based verification libraries (axi_test, apb_test, spm_test, ...), which
# Verilator cannot take and the testbench does not use; the files of the testbench (testharness, tb_memory_*,
# uartdpi) are listed for `verilator`.
# `tech_cells_generic_exclude_deprecated` is the switch of tech_cells_generic for its src/deprecated cells (pads
# with Verilog 1995 `pmos` gates, which Verilator does not support); nothing of Occamy uses them.
SIM_TARGETS = ["rtl", "verilator", "cv64a6_imafdc_sv39", "occamy_sim", "snitch_cluster",
               "tech_cells_generic_exclude_deprecated"]
# the define of upstream's Verilator flow (snitch_cluster target/common/common.mk, VLT_BENDER)
SIM_DEFINES = ["COMMON_CELLS_ASSERTS_OFF"]
HEADER_SUFFIXES = {".svh", ".vh", ".h", ".hh", ".inc"}
LICENCE_NAMES = re.compile(r"^(LICENSE|LICENCE|NOTICE|COPYING|CREDITS)")
# tracked files of the checkout that the simulation and the host software need, besides the file lists
OCCAMY_TREES = ["target/sim/test", "target/sim/sw/host", "target/sim/sw/shared", "target/sim/cfg"]
SNITCH_TREES = ["target/common/test"]
# The C++ of the simulation: sim/main.cc (ours, after tb_bin.cc and verilator_lib.cc of upstream's Verilator flow
# in snitch_cluster, target/common/common.mk) with the fesvr frontend and the boot ROM image of upstream and the DPI files of Occamy's
# target/sim (TB_CC_SOURCES of its Makefile, uartdpi).
SIM_CPP = [
    "sim/main.cc",
    "src/deps/snitch_cluster/target/common/test/common_lib.cc",
    "src/deps/snitch_cluster/target/common/test/bootrom.cc",
    "src/deps/snitch_cluster/target/common/test/ipc.cc",
    "src/occamy/target/sim/test/bootdata.cc",
    "sim/uartdpi.cc",
]


def run(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, check=True, capture_output=True, text=True).stdout


def flist(root, targets, extra_defines):
    cmd = ["bender", "script", "flist-plus"]
    for t in targets:
        cmd += ["-t", t]
    for d in extra_defines:
        cmd += ["-D", d]
    files, incdirs, defines = [], [], []
    for line in run(cmd, root).splitlines():
        if line.startswith("+incdir+"):
            incdirs.append(line[len("+incdir+"):])
        elif line.startswith("+define+"):
            d = line[len("+define+"):]
            if " " in d:
                # bender 0.28.2 writes the target defines of several `-t` as one broken define
                # ("TARGET_ RTL -T SYNTHESIS ..."); the intended ones are TARGET_<TARGET>
                defines += [f"TARGET_{t.upper()}" for t in targets]
            elif d != "TARGET_FLIST":  # the format of the script, not a target of the design
                defines.append(d)
        elif line.strip():
            files.append(line.strip())
    def uniq(xs):
        seen = set()
        return [x for x in xs if not (x in seen or seen.add(x))]
    return uniq(files), uniq(incdirs), uniq(defines)


DEF_RE = re.compile(r"^\s*(?:module|package|interface|program)\s+(?:automatic\s+)?(\w+)", re.M)


def definitions(path):
    text = Path(path).read_text(errors="replace")
    text = re.sub(r"/\*.*?\*/", "", re.sub(r"//[^\n]*", "", text), flags=re.S)
    return set(DEF_RE.findall(text))


def drop_shadowed(files):
    """The simulator of upstream keeps the last definition of a module that two listed files define.
    A file whose definitions are all redefined by later files is dropped from the list (the generated clint
    over the one of the clint repository, hw/future/reg_to_apb.sv over the one of register_interface, ...).
    A file that is shadowed in part stays; slang is told to accept that (design.toml)."""
    later, keep, dropped = set(), [], []
    for f in reversed(files):
        names = definitions(f) if f.endswith((".sv", ".v")) else set()
        if names and names <= later:
            dropped.append(f)
        else:
            keep.append(f)
        later |= names
    return keep[::-1], dropped[::-1]


def write_unused(design):
    """recipe/unused.txt: every file of src/ that no view of design.toml lists, with the reason. The views are
    read from design.toml, so this runs when the file exists (a second run of this script after writing it)."""
    toml = design / "design.toml"
    if not toml.exists():
        return
    t = tomllib.loads(toml.read_text())
    listed = set(t["source"].get("licence_files", []))
    for dep in t["source"].get("dependency", []):
        listed |= set(dep.get("licence_files", []))
    for v in t["variant"]:
        for key in ("synth", "sim"):
            sec = v.get(key, {})
            if "files" in sec:
                listed |= {l for l in (design / sec["files"]).read_text().split() if l}
            for d in sec.get("include_dirs", []) + (v["synth"].get("include_dirs", []) if key == "sim" else []):
                if (design / d).is_dir():
                    listed |= {f"{d}/{p.name}" for p in (design / d).iterdir() if p.is_file()}
            for m in sec.get("macros", []):
                listed.add(m["model"])
            listed |= set(sec.get("support", []))
            listed |= set(sec.get("sdc", []))
        for w in v.get("sim", {}).get("workload", []):
            listed |= set(w.get("sources", []))
            if "image" in w:
                listed.add(w["image"])
    rules = [
        ("src/generated/", "/sw/shared/platform/generated/", "platform header written by the Makefile of Occamy; hello_world is built with it (recipe/build-workloads.sh)"),
        ("src/occamy/target/sim/sw/", "", "host software of Occamy that hello_world does not use"),
        ("src/occamy/target/sim/cfg/", "", "configuration of upstream; recipe/README.md says which file each variant uses"),
        ("src/occamy/target/sim/test/", "", "file of the testbench of upstream's own simulator flow, the boot ROM sources or the device tree; the Verilator testbench does not compile it"),
        ("src/deps/snitch_cluster/target/common/test/", "", "testbench file of upstream that this simulation does not compile (sim/main.cc replaces tb_bin.cc and verilator_lib.cc)"),
        ("src/generated/", "", "generated by the Makefile of Occamy, in no file list of the variant (FPGA top, register and memory descriptions)"),
    ]
    lines = []
    for p in sorted(x for x in (design / "src").rglob("*") if x.is_file()):
        rel = p.relative_to(design).as_posix()
        if rel in listed:
            continue
        for prefix, infix, why in rules:
            if rel.startswith(prefix) and infix in rel:
                break
        else:
            why = "header of an include directory, included from a listed file" if p.suffix in HEADER_SUFFIXES else "in no view"
        lines.append(f"{rel} {why}\n")
    (design / "recipe" / "unused.txt").write_text("".join(lines))
    print(f"{len(lines)} files in recipe/unused.txt")


def main():
    work = Path(sys.argv[1]).resolve()
    design = Path(sys.argv[2]).resolve()
    src = design / "src"
    copies = {}  # destination relative to src -> source path

    def add(dst, srcp):
        srcp = Path(srcp)
        if dst in copies and copies[dst] != srcp:
            # two variants may reach the same dependency file through different checkouts: the bytes must agree
            if copies[dst].read_bytes() != srcp.read_bytes():
                sys.exit(f"{dst}: two different sources")
            return
        copies[dst] = srcp

    shutil.rmtree(src, ignore_errors=True)
    for v in VARIANTS:
        root = (work / v / "occamy").resolve()
        tracked = set(run(["git", "ls-files"], root).splitlines())
        tracked_dirs = {str(d) for t in tracked for d in Path(t).parents}
        checkouts = root / ".bender/git/checkouts"

        def dst_of(p):
            rel = Path(p).relative_to(root).as_posix()
            m = re.match(r"\.bender/git/checkouts/(.+)-[0-9a-f]{16}/(.*)", rel)
            if m:
                return f"deps/{m[1]}/{m[2]}"
            if rel in tracked or rel in tracked_dirs:
                return f"occamy/{rel}"
            return f"generated/{v}/{rel}"

        vdir = design / "variants" / v
        vdir.mkdir(parents=True, exist_ok=True)
        info = {}
        for kind, targets, extra in (("synth", SYNTH_TARGETS, []), ("sim", SIM_TARGETS, SIM_DEFINES)):
            files, incdirs, defines = flist(root, targets, extra)
            files, dropped = drop_shadowed(files)
            (vdir / f"{kind}-shadowed.txt").write_text("".join(f"src/{dst_of(f)}\n" for f in dropped))
            for f in files:
                add(dst_of(f), f)
            for d in incdirs:
                for dirpath, _, names in os.walk(d):
                    for n in sorted(names):
                        if Path(n).suffix in HEADER_SUFFIXES:
                            p = Path(dirpath) / n
                            add(dst_of(p), p)
            info[kind] = ([f"src/{dst_of(f)}" for f in files], [f"src/{dst_of(d)}" for d in incdirs], defines)
        # the generated files of this variant, listed or not
        for base in ("target/sim/src", "target/sim/test/testharness.sv", "target/sim/sw/shared/platform/generated"):
            b = root / base
            paths = [b] if b.is_file() else sorted(p for p in b.rglob("*") if p.is_file())
            for p in paths:
                rel = p.relative_to(root).as_posix()
                if rel not in tracked:
                    add(f"generated/{v}/{rel}", p)
        # tracked trees of the testbench and the host software
        for t in OCCAMY_TREES:
            for rel in sorted(tracked):
                if rel.startswith(t + "/"):
                    add(f"occamy/{rel}", root / rel)
        snitch = next(checkouts.glob("snitch_cluster-*"))
        for t in SNITCH_TREES:
            for p in sorted((snitch / t).rglob("*")):
                if p.is_file():
                    add(f"deps/snitch_cluster/{p.relative_to(snitch).as_posix()}", p)
        # licence files: the top of every checkout and of the vendored trees
        tops = list(checkouts.glob("*-*")) + [root, root / "hw"] + sorted((root / "hw/vendor").glob("*")) + [snitch / "hw"]
        for top in tops:
            if top.is_dir():
                for p in sorted(top.iterdir()):
                    if p.is_file() and LICENCE_NAMES.match(p.name):
                        add(dst_of(p), p)
        (vdir / "synth.f").write_text("".join(f + "\n" for f in info["synth"][0]))
        (vdir / "synth-incdirs.txt").write_text("".join(f + "\n" for f in info["synth"][1]))
        (vdir / "synth-defines.txt").write_text("".join(f + "\n" for f in info["synth"][2]))
        (vdir / "sim-incdirs.txt").write_text("".join(f + "\n" for f in info["sim"][1]))
        (vdir / "sim-defines.txt").write_text("".join(f + "\n" for f in info["sim"][2]))
        (vdir / "sim.f").write_text("".join(f + "\n" for f in info["sim"][0] + SIM_CPP))
        shutil.copy(work / v / "tools.txt", vdir / "tools.txt")

    for dst, p in sorted(copies.items()):
        t = src / dst
        t.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(p, t)
    print(f"{len(copies)} files in src/")
    write_unused(design)


if __name__ == "__main__":
    main()
