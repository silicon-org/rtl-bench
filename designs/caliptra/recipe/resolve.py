"""Resolve the file lists of caliptra.core and copy the files from an upstream checkout.

usage: python3 -I resolve.py <caliptra-rtl checkout with submodules> <design dir> <test> ...

Reads src/integration/config/caliptra_top.vf (synthesis) and caliptra_top_tb.vf (Verilator
testbench), the lists that upstream keeps for its own flows, and resolves their variables the way
tools/scripts/Makefile does (CALIPTRA_PRIM_ROOT = src/caliptra_prim_generic, prefix
caliptra_prim_generic, CALIPTRA_AXI4PC_DIR = src/integration/tb).  Writes
variants/core/{synth,sim}.f and the include directories (variants/core/*-incdirs.txt), and copies
every listed file, the headers they include, the DPI and C++ sources of the testbench, and the
firmware sources of the tests, to <design dir>/src at the same relative path.
"""
import re
import shutil
import sys
from pathlib import Path

up = Path(sys.argv[1])
dd = Path(sys.argv[2])
tests = sys.argv[3:]
cfg = up / "src/integration/config"

SUB = {
    "CALIPTRA_ROOT": ".",
    "CALIPTRA_PRIM_ROOT": "./src/caliptra_prim_generic",
    "CALIPTRA_PRIM_MODULE_PREFIX": "caliptra_prim_generic",
    "CALIPTRA_AXI4PC_DIR": "./src/integration/tb",
}


def expand(s):
    for k, v in SUB.items():
        s = s.replace("${%s}" % k, v)
    return str(Path(s))  # drops "./"


def read_vf(name):
    files, incs = [], []
    for line in (cfg / name).read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("//"):
            continue
        if line.startswith("+incdir+"):
            incs.append(expand(line[8:]))
        else:
            files.append(expand(line))
    return files, incs


synth, synth_inc = read_vf("caliptra_top.vf")
tb, tb_inc = read_vf("caliptra_top_tb.vf")
assert tb[: len(synth)] == synth, "the testbench list does not extend the synthesis list"
assert tb_inc[: len(synth_inc)] == synth_inc
sim = tb[len(synth):]
sim_inc = tb_inc[len(synth_inc):]
# The C and C++ sources of the model, as tools/scripts/Makefile lists them (TB_VERILATOR_SRCS).
cpp = [
    "src/integration/tb/test_caliptra_top_tb.cpp",
    "src/integration/test_suites/libs/jtagdpi/jtagdpi.c",
    "src/integration/test_suites/libs/tcp_server/tcp_server.c",
]
# The lint waivers of the testbench (caliptra_top_tb.vlt names them, one per line, passed with -f).
vlt = [expand(l.strip()) for l in (cfg / "caliptra_top_tb.vlt").read_text().splitlines()
       if l.strip() and not l.startswith("//")]

# Headers that listed files include, found next to the includer and in the include directories.
INC = re.compile(r'^\s*`include\s+"([^"]+)"', re.M)
copy = set(synth) | set(sim) | set(cpp) | set(vlt)
copy |= {"LICENSE", "submodules/adams-bridge/LICENSE",
         "src/integration/test_suites/libs/jtagdpi/jtagdpi.h",
         "src/integration/test_suites/libs/tcp_server/tcp_server.h"}
todo = [f for f in copy if f.endswith((".sv", ".svh", ".v", ".vh"))]
seen = set(todo)
while todo:
    f = todo.pop()
    for name in INC.findall((up / f).read_text(errors="replace")):
        for d in [str(Path(f).parent)] + synth_inc + sim_inc:
            c = (Path(d) / name).as_posix()
            if (up / c).is_file():
                if c not in seen:
                    seen.add(c)
                    copy.add(c)
                    todo.append(c)
                break
        else:
            print(f'warning: {f}: include "{name}" not found', file=sys.stderr)

# Firmware: the Makefile, the test directories and the libraries it compiles.
ts = "src/integration/test_suites"
copy.add("tools/scripts/Makefile")
for n in ("caliptra_top.vf", "caliptra_top_tb.vf", "caliptra_top_tb.vlt"):
    copy.add(f"src/integration/config/{n}")
for lib in ["aes", "doe", "ecc", "mldsa", "mlkem", "hmac", "keyvault", "datavault", "sha256",
            "sha512", "sha3", "soc_ifc", "caliptra_rtl_lib", "clk_gate", "wdt",
            "riscv_hw_if", "caliptra_isr", "printf"]:
    for f in sorted((up / ts / "libs" / lib).iterdir()):
        if f.is_file():
            copy.add(f"{ts}/libs/{lib}/{f.name}")
for f in sorted((up / ts / "includes").iterdir()):
    copy.add(f"{ts}/includes/{f.name}")
copy.add("src/integration/rtl/caliptra_reg/caliptra_reg.h")
for t in tests:
    for f in sorted((up / ts / t).iterdir()):
        if f.suffix in (".c", ".h", ".ld", ".s", ".S"):
            copy.add(f"{ts}/{t}/{f.name}")

for f in sorted(copy):
    dst = dd / "src" / f
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(up / f, dst)

out = dd / "variants/core"
out.mkdir(parents=True, exist_ok=True)
(out / "synth.f").write_text("".join(f"src/{f}\n" for f in synth))
(out / "sim.f").write_text("".join(f"src/{f}\n" for f in sim + cpp))
(out / "synth-incdirs.txt").write_text("".join(f"src/{d}\n" for d in synth_inc))
(out / "sim-incdirs.txt").write_text("".join(f"src/{d}\n" for d in sim_inc))
(out / "sim-vlt.txt").write_text("".join(f"src/{f}\n" for f in vlt))
print(f"{len(copy)} files copied; synth {len(synth)}, sim {len(sim) + len(cpp)}")
