#!/usr/bin/env python3
"""Normalise interp-tier archetype results: within each bench, subtract the minimal-dispatch baseline
(the `table` strategy = arm[k], one index) so the shared interpreter/other-branch overhead cancels and
each strategy's ATTRIBUTABLE cost remains (pure dispatch delta for one-arm strategies; arm-walk cost for
predicate). Reads the per-variant median from the findings tables. Usage: python3 normalise.py [n]"""
import sys, re, os, glob
N = sys.argv[1] if len(sys.argv)>1 else "4096"
def parse_medians(findings):
    # 'Function-under-test only' table: | variant | mean | best20 | worst20 | Δ | throughput |
    # we want a stable per-variant time; use the mean column (col 2)
    out={}
    if not os.path.exists(findings): return out
    t=open(findings).read()
    m=re.search(r'Function-under-test only.*?(?=Performance model|\Z)', t, re.S)
    if not m: return out
    for line in m.group(0).splitlines():
        if line.startswith('| ab_') or line.startswith('| ir_') or line.startswith('| sd_'):
            cols=[c.strip() for c in line.strip('|').split('|')]
            name=cols[0]
            # mean like '695766ns' or '1.86 ms'
            v=cols[1]
            mult=1.0
            if 'ms' in v: mult=1e6
            elif 'us' in v: mult=1e3
            num=float(re.sub(r'[^0-9.]','',v))
            out[name]=num*mult  # ns
    return out
benches=sorted(set(os.path.basename(os.path.dirname(p)) for p in glob.glob('results/arch_*/arch_*_n'+N+'_findings.md')))
print(f"NORMALISED archetype strategy cost (ns above the minimal-dispatch `table` baseline), n={N}")
print(f"(interp overhead + other-branch cost cancels; what remains is the strategy's own cost)\n")
for bn in benches:
    f=f"results/{bn}/{bn}_n{N}_findings.md"
    med=parse_medians(f)
    if not med: continue
    # find the table-strategy variant (name contains _table)
    base=None
    for k,v in med.items():
        if k.endswith('_table') or '_table_' in k: base=v; break
    if base is None: continue
    print(f"## {bn}")
    for k in sorted(med, key=lambda k:med[k]):
        strat=re.sub(r'^ab_b\d+_|^ab_(match|ifchain|whole)_','',k)
        delta=med[k]-base
        print(f"   {strat:10s} {delta:+10.0f} ns   (raw {med[k]:.0f})")
    print()
