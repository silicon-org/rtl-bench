#!/usr/bin/env bash
# Fetch the official licence texts of wally into licences/.
# usage: nix develop -c recipe/fetch-licences.sh
#   Apache-2.0.txt  https://www.apache.org/licenses/LICENSE-2.0.txt (as published)
#   SHL-2.1.txt     https://solderpad.org/licenses/SHL-2.1/ (the text of the page: title and paragraphs)
set -euo pipefail
here=$(cd "$(dirname "$0")/.." && pwd)
mkdir -p "$here/licences"
curl -fsSL https://www.apache.org/licenses/LICENSE-2.0.txt -o "$here/licences/Apache-2.0.txt"
curl -fsSL https://solderpad.org/licenses/SHL-2.1/ | python3 -I -c '
import html, re, sys
page = sys.stdin.read()
title = re.search(r"<h1[^>]*>(.*?)</h1>", page, re.S).group(1)
body = re.search(r"<section class=\"page__content\"[^>]*>(.*?)</section>", page, re.S).group(1)
out = [title.strip()]
for p in re.findall(r"<p>(.*?)</p>", body, re.S):
    p = re.sub(r"<br\s*/?>", "\n", p)
    out.append(html.unescape(re.sub(r"<[^>]+>", "", p)).strip())
print("\n\n".join(out))
' > "$here/licences/SHL-2.1.txt"
