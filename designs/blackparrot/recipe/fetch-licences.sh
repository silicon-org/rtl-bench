#!/usr/bin/env bash
# Fetch the licence text of BaseJump STL that is not in its repository into licences/.
# usage: nix develop -c recipe/fetch-licences.sh
#   SHL-0.51.txt  https://solderpad.org/licenses/SHL-0.51/ (the text of the page: title, headings,
#                 paragraphs and list items, without the notice about the newer version)
# The other licences are BSD-3-Clause texts in the repositories: src/LICENSE (black-parrot),
# src/external/HardFloat/COPYING.txt; BaseJump STL's own LICENSE names SHL-0.51.
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
mkdir -p "$here/licences"
curl -fsSL https://solderpad.org/licenses/SHL-0.51/ | python3 -I -c '
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
' > "$here/licences/SHL-0.51.txt"
