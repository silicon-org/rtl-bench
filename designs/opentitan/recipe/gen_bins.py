import re,sys
# usage: python3 gen_bins.py <sw/device/tests/BUILD> <test name> ...
# Prints Bazel targets rtlbench_<name>: the sources and dependencies of the opentitan_test of that
# name as an opentitan_binary for the sim_verilator_base environment (no test harness, no model).
src=open(sys.argv[1]).read()
names=sys.argv[2:]
out=["\n# rtl-bench: plain binaries of tests of this package for the Verilator environment (no harness, no model)\n"]
for n in names:
    m=re.search(r'^opentitan_test\(\n    name = "%s",\n'%re.escape(n),src,re.M)
    if not m: print("missing",n,file=sys.stderr); continue
    i=m.start(); depth=0; j=i
    while True:
        c=src[j]
        if c=='(': depth+=1
        elif c==')':
            depth-=1
            if depth==0: break
        j+=1
    block=src[i:j+1]
    # split top-level keys (4-space indented "key = ")
    lines=block.split("\n")[1:-1]
    keys=[];cur=None
    for l in lines:
        mm=re.match(r'^    (\w+) = ',l)
        if mm: cur=[mm.group(1),[l]]; keys.append(cur)
        elif cur: cur[1].append(l)
    keep=("name","srcs","deps","copts","defines","kind","linker_script","linkopts","data","includes","local_defines")
    body="".join("\n".join(k[1])+"\n" for k in keys if k[0] in keep and k[0]!="name")
    out.append('opentitan_binary(\n    name = "rtlbench_%s",\n    testonly = True,\n    exec_env = ["//hw/top_earlgrey:sim_verilator_base"],\n%s)\n'%(n,body))
print("".join(out))
