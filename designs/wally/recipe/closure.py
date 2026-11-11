"""Files that a top module needs, found by reading the sources.

usage: python3 -I closure.py <upstream root> <top module> [<top module> ...] <file> ...

The files after the tops are the extra roots (the package, the generated wrapper).
Prints the files (paths relative to the upstream root, one per line, sorted) that
define the tops and every module instantiated from them, in any generate branch
and with no macro defined.  Modules are found by name in src/ and testbench/.
Verilator and slang both need the definition of every instantiated module, also
of one that sits in a generate branch that the configuration does not take.
"""
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
tops = []
roots = []
for a in sys.argv[2:]:
    (roots if "/" in a or a.endswith(".sv") else tops).append(a)

COMMENT = re.compile(r"//[^\n]*|/\*.*?\*/", re.S)


def strip(text):
    """Remove comments and the branches of `ifdef/`ifndef (no macro is defined)."""
    text = COMMENT.sub("", text)
    out = []
    stack = []  # one bool per open conditional: is the current branch live
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("`ifdef"):
            stack.append(False)
        elif s.startswith("`ifndef"):
            stack.append(True)
        elif s.startswith("`elsif"):
            stack[-1] = False
        elif s.startswith("`else"):
            stack[-1] = not stack[-1]
        elif s.startswith("`endif"):
            stack.pop()
        elif all(stack):
            out.append(line)
    return "\n".join(out)


defs = {}   # module name -> file
text = {}   # file -> stripped text
for d in ("src", "testbench"):
    for f in sorted((root / d).rglob("*.sv")):
        rel = f.relative_to(root).as_posix()
        t = strip(f.read_text(errors="replace"))
        text[rel] = t
        for m in re.finditer(r"(?m)^\s*module\s+(?:automatic\s+)?(\w+)", t):
            defs[m.group(1)] = rel

INST = re.compile(r"\b(\w+)\s*(?:#\s*\(|\w+\s*[\(\[])")
seen = set(roots)
todo = [defs[t] for t in tops] + roots
while todo:
    f = todo.pop()
    seen.add(f)
    if f not in text:
        continue
    for m in INST.finditer(text[f]):
        g = defs.get(m.group(1))
        if g and g not in seen:
            seen.add(g)
            todo.append(g)
print("\n".join(sorted(seen)))
