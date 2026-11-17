"""Resolve the file lists of bsg-manycore and copy the files from the upstream checkouts.

usage: python3 -I recipe/resolve.py <bsg_manycore checkout, imports/HardFloat initialised> <basejump_stl checkout, imports/DRAMSim3 initialised>
Writes designs/bsg-manycore/src/{bsg_manycore,basejump_stl}/ (copies at the upstream relative paths, the two
repositories side by side as upstream expects them), variants/<id>/{synth,sim}.f, sim/<id>/bsg_tag_boot_rom.v and
recipe/unused.txt.

The lists are those of machines/arch_filelist.mk (the synthesizable architecture) and machines/sim_filelist.mk (the
testbench), read by make as upstream's machines/Makefile does with BSG_PLATFORM=verilator.
"""
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
MC = Path(sys.argv[1]).resolve()
BJ = Path(sys.argv[2]).resolve()
SRC = HERE / "src"
ROOTS = {"bsg_manycore": MC, "basejump_stl": BJ}

# The machines of the variants (directories of machines/), and the programs that the workloads are built from.
MACHINES = ["pod_1x1_4X2Y", "pod_1x1"]
PROGRAMS = ["hello", "bsg_mutex", "bsg_barrier_amoadd_test", "vcache_atomic_inc"]

MK = f"""BSG_MANYCORE_DIR := {MC}
BASEJUMP_STL_DIR := {BJ}
BSG_PLATFORM := verilator
include $(BSG_MANYCORE_DIR)/machines/arch_filelist.mk
ARCH_VINC := $(VINCLUDES)
ARCH_VH := $(VHEADERS)
ARCH_VS := $(VSOURCES)
include $(BSG_MANYCORE_DIR)/machines/sim_filelist.mk
print:
\t@echo "ARCH_VINC $(ARCH_VINC)"
\t@echo "ARCH_VH $(ARCH_VH)"
\t@echo "ARCH_VS $(ARCH_VS)"
\t@echo "ALL_VINC $(VINCLUDES)"
\t@echo "ALL_VH $(VHEADERS)"
\t@echo "ALL_VS $(VSOURCES)"
\t@echo "ALL_CS $(CSOURCES)"
"""


def make_lists():
    with tempfile.TemporaryDirectory() as d:
        mk = Path(d) / "lists.mk"
        mk.write_text(MK)
        out = subprocess.run(["make", "-s", "-f", str(mk), "print"], check=True, capture_output=True, text=True).stdout
    lists = {}
    for line in out.splitlines():
        key, _, rest = line.partition(" ")
        lists[key] = rest.split()
    return lists


def rel(p):
    """(repository, path relative to it) of an absolute path of a checkout."""
    p = Path(p)
    for name, root in ROOTS.items():
        try:
            return name, p.relative_to(root).as_posix()
        except ValueError:
            pass
    raise SystemExit(f"{p} is in neither checkout")


def dest(p):
    name, r = rel(p)
    return f"src/{name}/{r}"


lists = make_lists()
arch_inc = lists["ARCH_VINC"]
all_inc = lists["ALL_VINC"]
arch_files = lists["ARCH_VH"] + lists["ARCH_VS"]
sim_files = [f for f in lists["ALL_VH"] + lists["ALL_VS"] if f not in set(arch_files)]
csources = lists["ALL_CS"]

# files to copy (absolute paths of the checkouts) and the reason for those that no view lists
copy = {}


def want(p, why=None):
    copy.setdefault(str(p), why)


for f in arch_files + sim_files + csources:
    want(f)

# the files that the listed ones include: Verilog `include next to the includer and in the include directories;
# C++ #include "x" and <x> next to the includer and in the -I directories of Makefile.verilator
INC_V = re.compile(r'^\s*`include\s+"([^"]+)"', re.M)
INC_C = re.compile(r'^\s*#\s*include\s*[<"]([^>"]+)[>"]', re.M)
cinc = [str(MC / "testbenches/common/v"), str(BJ / "bsg_test")] + [
    str(BJ / "imports/DRAMSim3" / x) for x in ("src", "ext/headers", "ext/fmt/include")
]
todo = list(copy)
seen = set(todo)
while todo:
    f = todo.pop()
    text = Path(f).read_text(errors="replace")
    is_c = f.endswith((".cc", ".cpp", ".h", ".hpp", ".hh"))
    for name in (INC_C if is_c else INC_V).findall(text):
        for d in [str(Path(f).parent)] + (cinc if is_c else all_inc):
            c = Path(d) / name
            if c.is_file():
                c = str(c.resolve())
                if c not in seen:
                    seen.add(c)
                    want(c, "included by a listed file")
                    todo.append(c)
                break

# licences
for p in [MC / "LICENSE", BJ / "LICENSE", MC / "imports/HardFloat/COPYING.txt", BJ / "imports/DRAMSim3/LICENSE"]:
    want(p)

# the generators of the boot ROM of the testbench (Makefile.verilator: pod_trace_gen.py, then bsg_ascii_to_rom.py)
GEN = "generator of the tag boot ROM of the testbench (machines/Makefile.verilator), run by recipe/resolve.py"
for p in [MC / "testbenches/py/pod_trace_gen.py", MC / "testbenches/py/bsg_tag_trace_gen.py", BJ / "bsg_mem/bsg_ascii_to_rom.py"]:
    want(p, GEN)

# the machines and the software that the workloads are built from (recipe/build-workloads.sh)
MACH = "machine definition of a variant"
want(MC / "machines/platform.mk", "machine selection of the software makefiles")
for m in MACHINES:
    want(MC / "machines" / m / "Makefile.machine.include", MACH)
BUILD = "workload build (recipe/build-workloads.sh): makefiles, runtime library, linker script generator"
sw = MC / "software"
for sub in ("mk", "bsg_manycore_lib", "spmd/common"):
    for p in sorted((sw / sub).rglob("*")):
        if p.is_file():
            want(p, BUILD)
want(sw / "spmd/Makefile.include", BUILD)
for f in ("bsg_manycore_link_gen.py", "nbf.py"):
    want(sw / "py" / f, BUILD)
PROG_SRC = {}
for prog in PROGRAMS:
    PROG_SRC[prog] = []
    for p in sorted((sw / "spmd" / prog).rglob("*")):
        if p.is_file():
            want(p)  # listed by the `sources` of the workloads
            PROG_SRC[prog].append(dest(p))

# copy
if SRC.exists():
    shutil.rmtree(SRC)
unused = []
for f, why in sorted(copy.items()):
    d = HERE / dest(f)
    d.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(f, d)
    # a file directly in an include directory is listed by that directory
    if why and not any(Path(f).parent == Path(i) for i in all_inc):
        unused.append(f"{dest(f)} {why}")

# lists
def paths(fs):
    return "\n".join(dest(f) for f in fs) + "\n"


# the boot ROM, as machines/Makefile.verilator makes it
for m in MACHINES:
    vid = m.lower()
    vd = HERE / "variants" / vid
    vd.mkdir(parents=True, exist_ok=True)
    mf = (MC / "machines" / m / "Makefile.machine.include").read_text()
    px = int(re.search(r"^BSG_MACHINE_PODS_X\s*=\s*(\d+)", mf, re.M).group(1))
    py = int(re.search(r"^BSG_MACHINE_PODS_Y\s*=\s*(\d+)", mf, re.M).group(1))
    tr = subprocess.run(
        [sys.executable, str(MC / "testbenches/py/pod_trace_gen.py"), str(px), str(py)],
        check=True, capture_output=True, text=True, cwd=MC / "testbenches/py",
    ).stdout
    # a fixed directory: the generator writes the absolute path of its input in the first line of the output
    d = "/tmp/rtl-bench-bsg-manycore-rom"
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    try:
        (Path(d) / "pod_trace.tr").write_text(tr)
        rom = subprocess.run(
            [sys.executable, str(BJ / "bsg_mem/bsg_ascii_to_rom.py"), "pod_trace.tr", "bsg_tag_boot_rom"],
            check=True, capture_output=True, text=True, cwd=d,
        ).stdout
    finally:
        shutil.rmtree(d, ignore_errors=True)
    (HERE / "sim" / vid).mkdir(parents=True, exist_ok=True)
    (HERE / "sim" / vid / "bsg_tag_boot_rom.v").write_text(rom)
    (vd / "synth.f").write_text(paths(arch_files))
    (vd / "sim.f").write_text(paths(sim_files + csources) + f"sim/{vid}/bsg_tag_boot_rom.v\n")

(HERE / "recipe" / "unused.txt").write_text("\n".join(sorted(unused)) + "\n")
print("include dirs of the synthesis view:")
for d in arch_inc:
    print("  ", dest(d))
print("further include dirs of the simulation:")
for d in all_inc:
    if d not in arch_inc:
        print("  ", dest(d))
for prog, fs in PROG_SRC.items():
    print(prog, fs)
