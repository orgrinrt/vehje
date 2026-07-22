#!/usr/bin/env python3
"""Automated ISA fairness audit for the carrier dispatch cells.

The old reproduce script (an awk one-liner) classified a whole symbol by counting
`stp` / `br` / `blr`. That is coarse enough to miss a per-opcode load-count
asymmetry buried in an otherwise-fine symbol (the CFG-SET finding) and a cascade
that collapsed into a jump table (the if-chain finding). This script is the
automated generalization: it disassembles every `di_*` probe symbol in the built
disasm-probe dylib, classifies each cell's dispatch lowering from its instruction
mix (callee-saved spills, computed `br`, indirect `blr`), and asserts each cell
lowers to the shape its technique is supposed to.

The assertions are the regression guard for the two findings' failure modes: the
barrier-forced `di_ifchain_linear` must stay a real compare cascade (no computed
jump, or the `black_box` barrier stopped preventing switch formation), the natural
if-chains must show the jump-table collapse, and the function-pointer-table cells
must keep their surviving indirect call (not get devirtualized). The script exits
non-zero if any cell drifts, so it can gate a build.

Whole-symbol arithmetic-histogram diffs across cells are deliberately NOT used:
dispatch shape relocates the arithmetic (a function-pointer table's compute lives
in the called-out `f_*`, not in the table-walk symbol; a switch inlines it), so
the histograms are not comparable across shapes. A true per-opcode diff (isolating
each opcode's arm and comparing its instruction schedule across cells, which every
cell now sharing one `ops::binop_body` definition should make identical) needs
per-op symbols the fat-LTO'd probe inlines away; exposing them is the documented
refinement. The classification assertions here already catch the findings' failure
modes and run on every build.

Usage:
    cd mock/benches/disasm-probe && cargo build --release --quiet
    python3 isa_audit.py [--dylib target/release/libdisasm_probe.dylib]
"""

import argparse
import re
import subprocess
import sys
from collections import Counter

# Expected dispatch lowering per cell, as (predicate, description). The audit
# fails if a cell's classification drifts from what its technique is supposed to
# lower to. Whole-symbol arithmetic-histogram diffs across cells are NOT used:
# dispatch shape relocates the arithmetic (a function-pointer table's compute is
# in the called-out `f_*`, not in the table-walk symbol; a switch inlines it), so
# the histograms are not comparable across shapes. A true per-opcode diff needs
# per-op symbols the fat-LTO'd probe inlines away; that is the documented
# refinement. These classification assertions are the regression guard that runs
# now: they catch the two findings' failure modes (the barrier-forced cascade
# regressing to a jump table, an indirect-call cell getting devirtualized).
def _is_jump_table(c):
    return c["br"] > 0

def _is_compare_tree(c):
    return c["br"] == 0 and c["blr"] == 0

def _has_indirect_call(c):
    return c["blr"] > 0

EXPECTED = {
    # natural if-chains collapse to a jump table under SimplifyCFG (the finding).
    "di_ifchain": (_is_jump_table, "jump table (natural if-chain collapse)"),
    "di_ifchain_ascending": (_is_jump_table, "jump table (natural if-chain collapse)"),
    # the barrier-forced cell must stay a real compare cascade, no computed jump:
    # if this ever fails, the black_box barrier stopped preventing switch formation.
    "di_ifchain_linear": (_is_compare_tree, "compare cascade (barrier held, no jump table)"),
    # bit-test tree is a genuine compare tree, no computed/indirect branch.
    "di_bittree": (_is_compare_tree, "compare tree"),
    # function-pointer tables must keep their surviving indirect call.
    "di_fntable": (_has_indirect_call, "surviving indirect call"),
    "di_cfg_fntable": (_has_indirect_call, "surviving indirect call"),
    "di_pre_fntable": (_has_indirect_call, "surviving indirect call"),
}


def objdump(dylib, symbol):
    out = subprocess.run(
        ["objdump", "-d", f"--disassemble-symbols=_{symbol}", dylib],
        capture_output=True, text=True,
    )
    return out.stdout


def mnemonic_histogram(disasm):
    hist = Counter()
    for line in disasm.splitlines():
        # objdump line: "    19a0: f00001ac  \tadrp\tx12, ..."
        m = re.search(r"\t([a-z][a-z0-9._]*)\b", line)
        if m:
            mn = m.group(1).split(".")[0]  # strip vector suffix (.2d etc)
            hist[mn] += 1
    return hist


def classify(hist):
    return {
        "spills": hist.get("stp", 0),
        "br": hist.get("br", 0),
        "blr": hist.get("blr", 0),
        "bl": hist.get("bl", 0),
    }


def symbols_present(dylib):
    out = subprocess.run(["objdump", "-t", dylib], capture_output=True, text=True).stdout
    found = []
    for line in out.splitlines():
        m = re.search(r"\b_(di_[a-z0-9_]+)\b", line)
        if m:
            found.append(m.group(1))
    return sorted(set(found))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dylib", default="target/release/libdisasm_probe.dylib")
    args = ap.parse_args()

    syms = symbols_present(args.dylib)
    if not syms:
        print(f"no di_* symbols in {args.dylib}; build with `cargo build --release` first", file=sys.stderr)
        return 2

    hists = {}
    print(f"{'cell':<26} {'spills':>6} {'br':>3} {'blr':>3} {'bl':>3}  dispatch")
    print("-" * 70)
    for s in syms:
        d = objdump(args.dylib, s)
        h = mnemonic_histogram(d)
        hists[s] = h
        c = classify(h)
        # dispatch label inferred from the classification
        if c["blr"] > 0:
            label = "indirect call (fn-ptr table)"
        elif c["br"] > 0 and c["spills"] == 0:
            label = "computed branch (jump table / tail thread)"
        elif c["br"] == 0 and c["blr"] == 0:
            label = "no computed/indirect branch (compare tree)"
        else:
            label = "mixed"
        print(f"{s:<26} {c['spills']:>6} {c['br']:>3} {c['blr']:>3} {c['bl']:>3}  {label}")

    # expected-classification regression checks.
    print("\nexpected-lowering checks")
    print("-" * 70)
    findings = []
    for cell, (pred, desc) in EXPECTED.items():
        if cell not in hists:
            print(f"  {cell:<24} SKIP (symbol absent)")
            continue
        c = classify(hists[cell])
        ok = pred(c)
        print(f"  {cell:<24} {'ok  ' if ok else 'FAIL'} expected {desc}")
        if not ok:
            findings.append((cell, desc, c))

    if findings:
        print("\nFINDINGS (dispatch lowering drifted from the expected shape):")
        for cell, desc, c in findings:
            print(f"  {cell}: expected {desc}, got spills={c['spills']} br={c['br']} blr={c['blr']} bl={c['bl']}")
        return 1
    print("\nall cells lower to their expected dispatch shape.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
