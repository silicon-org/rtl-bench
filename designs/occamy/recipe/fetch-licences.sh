#!/usr/bin/env bash
# Fetch the licence texts that the Occamy sources need but do not ship, into licences/.
# usage: nix develop -c recipe/fetch-licences.sh
#   SHL-0.51.txt   https://solderpad.org/licenses/SHL-0.51/ (the text of the page, as in the opentitan recipe)
#   SHL-2.0.txt    https://solderpad.org/licenses/SHL-2.0/ (same extraction; the licence of the files
#                  marked `Apache-2.0 WITH SHL-2.0`, the Apache licence is src/occamy/LICENSE)
#   LGPL-2.1.txt   https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt (apb_uart, which has no licence file;
#                  its files say LGPL-2.1-or-later in their headers)
#   cva6/LICENSE   pulp-platform/cva6 at 6a6b8c656b8bb789d50b9c84cce1b99c74966884, the commit that
#                  hw/vendor/openhwgroup_cva6.lock.hjson names; the vendored tree has no licence file
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
mkdir -p "$here/licences/cva6"

solderpad() { # <id> <out>
  curl -fsSL "https://solderpad.org/licenses/$1/" | python3 -I -c '
import html, re, sys
page = sys.stdin.read()
title = re.search(r"<h1[^>]*>(.*?)</h1>", page, re.S).group(1)
body = re.search(r"<section class=\"page__content\"[^>]*>(.*?)</section>", page, re.S).group(1)
body = re.sub(r"<blockquote.*?</blockquote>", "", body, flags=re.S)
out = [title.strip()]
for m in re.finditer(r"<(h\d|p|li)[^>]*>(.*?)</\1>", body, re.S):
    t = re.sub(r"<br\s*/?>", "\n", m.group(2))
    t = html.unescape(re.sub(r"<[^>]+>", "", t)).strip()
    out.append(("- " if m.group(1) == "li" else "") + t)
print("\n\n".join(out))
' > "$2"
}
solderpad SHL-0.51 "$here/licences/SHL-0.51.txt"
solderpad SHL-2.0 "$here/licences/SHL-2.0.txt"
curl -fsSL https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt > "$here/licences/LGPL-2.1.txt"
curl -fsSL https://raw.githubusercontent.com/pulp-platform/cva6/6a6b8c656b8bb789d50b9c84cce1b99c74966884/LICENSE > "$here/licences/cva6/LICENSE"
