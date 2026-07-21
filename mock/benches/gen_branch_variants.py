#!/usr/bin/env python3
"""Generate branch-strategy variants for the vehje branch-lowering bench.
Axes: STRATEGY x ARM-COST x SITUATION. Arms are #[inline(never)] so the `branch`
strategy stays a REAL branch (LLVM won't if-convert a call), modelling the
interpreter dispatching into a script subtree of `WORK` cost. Every strategy in
a (situation,cost) bench computes the identical xor-reduction => may_differ=false,
cross-validated. Extensible: add a STRATEGY, COST, or SITUATION and re-run.

The core tradeoff being mapped: `branch` does ONE arm but risks a mispredict;
branchless (`predicate`/`mask`/`lut`) does BOTH arms but never mispredicts;
`profiled_hot` does the hot arm always and only the cold arm on the rare case.
So the winner is governed by predictability (mispredict rate) x arm-cost (the
price of the extra arm). This bench measures that map."""
import os

SIZES = "[64, 256, 1024, 4096, 16384]"
# arms: inline(never), do WORK hash-iterations over acc (models interpreting a subtree of WORK cost)
ARMS = '''#[inline(never)]
fn arm_taken(b: u8, work: u32) -> u64 {
    let mut a: u64 = 0xcbf29ce484222325 ^ (b as u64);
    let mut k = 0; while k < work { a = a.wrapping_mul(0x100000001b3) ^ (a >> 29); k += 1; }
    a
}
#[inline(never)]
fn arm_nottaken(b: u8, work: u32) -> u64 {
    let mut a: u64 = 0x9e3779b97f4a7c15u64.wrapping_add(b as u64);
    let mut k = 0; while k < work { a = a.rotate_left(13).wrapping_add(0x100000001b3); k += 1; }
    a
}'''

# strategy -> v-expression given `b`, `cond`, `W` (work const), and (for profiled) `hot_taken: bool`
def strat_body(strat, hot_taken):
    if strat == "branch":
        return "let v = if cond { arm_taken(b, W) } else { arm_nottaken(b, W) };"
    if strat == "predicate":
        return "let t = arm_taken(b, W); let nt = arm_nottaken(b, W); let v = if cond { t } else { nt };"
    if strat == "mask":
        return "let m = (cond as u64).wrapping_neg(); let v = (arm_taken(b, W) & m) | (arm_nottaken(b, W) & !m);"
    if strat == "lut":
        return "let v = [arm_nottaken(b, W), arm_taken(b, W)][cond as usize];"
    if strat == "profiled_hot":
        # do the HOT arm always; only compute+use the cold arm on the rare (cold) case.
        # a real branch, but predicted-not-taken (rare) => cheap; and no wasted cold-arm work in the common case.
        if hot_taken:
            return "let mut v = arm_taken(b, W); if !cond { v = arm_nottaken(b, W); }"
        else:
            return "let mut v = arm_nottaken(b, W); if cond { v = arm_taken(b, W); }"
    raise ValueError(strat)

# situation -> (desc, iter_header, cond_line, hot_taken_or_None)
SITS = {
    "rand50":   ("~50% taken, UNPREDICTABLE (b&1)",      "for &b in input.iter() {", "let cond = (b & 1) == 0;", None),
    "pred95":   ("~95% taken, predictable (b<243)",       "for &b in input.iter() {", "let cond = b < 243;", True),
    "pred05":   ("~5% taken, predictable (b<13)",         "for &b in input.iter() {", "let cond = b < 13;", False),
    "biased25": ("~25% taken (b<64)",                     "for &b in input.iter() {", "let cond = b < 64;", False),
    "runs":     ("correlated long runs (flip when b<24)", "let mut prev = false;\n            for &b in input.iter() {", "if b < 24 { prev = !prev; } let cond = prev;", None),
    "alt":      ("strict alternation (i&1)",              "for (i, &b) in input.iter().enumerate() {", "let cond = (i & 1) == 0;", None),
}
COSTS = {"c": 1, "h": 24}  # cheap arm (1 hash iter) vs heavy arm (24)

# applicability: mask/lut only meaningful for cheap arms (heavy => identical to predicate); profiled only where a hot arm exists
def applicable(strat, cost, sit):
    hot = SITS[sit][3]
    if strat == "profiled_hot" and hot is None: return False
    if strat in ("mask", "lut") and cost == "h": return False
    return True

STRATS = ["branch", "predicate", "mask", "lut", "profiled_hot"]

def crate(strat, cost, sit):
    name = f"br_{strat}_{cost}_{sit}"
    desc, iterhdr, condline, hot = SITS[sit]
    W = COSTS[cost]
    vexpr = strat_body(strat, hot if hot is not None else True)
    lib = f'''use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;

const W: u32 = {W};

{ARMS}

// strategy `{strat}`, arm-cost `{cost}` (W={W}), situation `{sit}`: {desc}
#[bench_variant("{name}", sizes = {SIZES})]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    timed! {{
        run {{
            let mut acc: u64 = 0;
            {iterhdr}
                {condline}
                {vexpr}
                acc ^= v;
            }}
            output.copy_from_slice(&acc.to_le_bytes());
        }}
    }}
}}
'''
    cargo = f'''[workspace]
[package]
name = "{name}"
version = "0.0.0"
edition = "2021"
publish = false
[lib]
name = "{name}"
path = "src/lib.rs"
crate-type = ["cdylib"]
[dependencies]
mockspace-bench-core = {{ git = "https://github.com/hiisi-digital/mockspace", branch = "dev", features = ["std"] }}
mockspace-bench-macro = {{ git = "https://github.com/hiisi-digital/mockspace", branch = "dev" }}
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
'''
    d = f"variants/{name}"
    os.makedirs(f"{d}/src", exist_ok=True)
    open(f"{d}/Cargo.toml","w").write(cargo)
    open(f"{d}/src/lib.rs","w").write(lib)
    return name

def bench_toml():
    out = ["[docgen]\nenabled = true\n",
           "[timing]\npasses = 6\nruns_per_pass = 100\nbatch_size = 100\nharness_runs = 1\ncooldowns_ms = [0]\n"]
    for cost in COSTS:
        for sit in SITS:
            variants = [f'"variants/br_{s}_{cost}_{sit}/target/release/br_{s}_{cost}_{sit}"'
                        for s in STRATS if applicable(s, cost, sit)]
            if not variants: continue
            bn = f"branch_{sit}_{cost}"
            desc = SITS[sit][0]
            costname = "cheap-arm" if cost == "c" else "heavy-arm"
            out.append(f'[bench.{bn}]')
            out.append(f'title = "Branch strategies, {costname}, {sit}: {desc}"')
            out.append('workload = "realistic"')
            out.append('master_seed = 0x7654_3210_fedc_ba98')
            for n in [64,256,1024,4096,16384]:
                out.append(f'[[bench.{bn}.sizes]]')
                out.append(f'n = {n}')
                out.append(f'variants = [{", ".join(variants)}]')
            out.append('')
    return "\n".join(out)

if __name__ == "__main__":
    names = []
    for cost in COSTS:
        for sit in SITS:
            for s in STRATS:
                if applicable(s, cost, sit): names.append(crate(s, cost, sit))
    open("bench.toml","w").write(bench_toml())
    print(f"generated {len(names)} variants")
