"""Turn a fusesoc file list (.scr of the syn target, .vc of the Verilator sim) into design lists.

usage: python3 -I flist.py <list file> <build dir> <checkout copy used as cores root>
Prints one line per entry: `D <define>`, `I <dir>`, `F <file>`; paths are relative to the
checkout (`U:` prefix dropped) or to the build directory's generated/ (`generated/<path>`).
Only the .sv, .svh, .vh, .v, .vlt, .c, .cc and .cpp file entries are files; edalize's own
options (--cc, -CFLAGS ...) are skipped.
"""
import os
import sys

lst, build, root = sys.argv[1:4]
build = os.path.realpath(build)
root = os.path.realpath(root)
gen = os.path.join(build, "generated")
FILES = (".sv", ".svh", ".vh", ".v", ".vlt", ".c", ".cc", ".cpp", ".h")


def rel(p):
    a = os.path.normpath(os.path.join(build, p))
    if a.startswith(root + "/"):
        return os.path.relpath(a, root)
    if a.startswith(gen + "/"):
        return "generated/" + os.path.relpath(a, gen)
    sys.exit(f"outside the checkout and the generated files: {a}")


for line in open(lst):
    line = line.strip()
    if not line or line.startswith("#"):
        continue
    if line.startswith("+define+"):
        print("D", line[len("+define+"):])
    elif line.startswith("-D") and "=" in line or line.startswith("-D"):
        print("D", line[2:])
    elif line.startswith("+incdir+"):
        print("I", rel(line[len("+incdir+"):]))
    elif line.startswith("-"):
        continue
    elif line.endswith(FILES):
        print("F", rel(line))
