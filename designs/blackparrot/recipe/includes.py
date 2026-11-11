"""Files that the listed sources include.

usage: python3 -I includes.py <checkout> <incdir> ... -- <file> ...
Paths are relative to the checkout. Prints the files found by `include "name"` directives, followed
transitively, looked up next to the including file and then in the include directories, one per line.
"""
import re
import sys
from pathlib import Path

args = sys.argv[1:]
root = Path(args[0])
sep = args.index("--")
incdirs = args[1:sep]
todo = list(args[sep + 1:])
INC = re.compile(r'^\s*`include\s+"([^"]+)"', re.M)
seen = set(todo)
found = set()
while todo:
    f = todo.pop()
    text = (root / f).read_text(errors="replace")
    for name in INC.findall(text):
        for d in [str(Path(f).parent)] + incdirs:
            cand = Path(d) / name
            if (root / cand).is_file():
                c = cand.as_posix()
                if c not in seen:
                    seen.add(c)
                    found.add(c)
                    todo.append(c)
                break
        else:
            print(f"warning: {f}: include \"{name}\" not found", file=sys.stderr)
print("\n".join(sorted(found)))
