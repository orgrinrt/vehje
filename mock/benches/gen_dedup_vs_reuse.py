#!/usr/bin/env python3
"""Generate the dedup-vs-reuse composition bench: does hash-consing produced
values compose with the exact-meet reuse analysis, or do they undercut each other?

They are not independent. `record-update-reuse` measured in-place-when-unique at
9.7x over copy-on-write at 0% sharing but only 1.5x at 60%, and hash-consing
RAISES the sharing rate by construction, so dedup buys cheap equality and memory
by spending exactly the property reuse runs on. Benching them separately and
switching both on would recommend two mechanisms that fight.

A first standalone Zig draft of this cell found the sharp edge the hard way: it
mutated in place a record that was still in the intern table, which corrupts the
table (the probe for that key finds a record whose contents no longer hash there,
so the chain never terminates on a match and never on an empty slot). That is not
a coding slip to route around, it is the design constraint stated operationally:
THE DEDUP TABLE IS ITSELF A REFERRER. A value the table holds is shared no matter
what the local referrer count says, so intern and reuse are an EXCLUSIVE choice
per construction site, and a value cannot flip between them. The dedup_shared_reuse
strategy below encodes that rule; harness cross-validation now enforces it, since a
model that violates it produces a wrong answer rather than a plausible number.

Four strategies, byte-identical output required (an optimisation that changes the
answer is not an optimisation, which is precisely what cross-validation checks):
  plain               copy-on-write update, structural equality. The naive runtime.
  reuse               in-place-when-unique update, structural equality.
  dedup_all           hash-cons every constructed value; equality is an id compare;
                      update always copies, since every value is table-held.
  dedup_shared_reuse  the emitter's verdict picks ONE per site: a may-be-shared
                      value is interned and never mutates in place; a proven-unique
                      value stays out of the table and mutates freely. Equality is
                      an id compare when both sides are interned, structural otherwise.

Two benches, differing only in how often the emitter's verdict says "may be shared":
  dedup_reuse_unique  20% shared. The templating and config norm: records are built
                      locally, filled, and emitted, so they are overwhelmingly unique.
  dedup_reuse_shared  60% shared. The adversarial regime for reuse, where dedup has
                      the most to offer and reuse the least.

Run: python3 gen_dedup_vs_reuse.py && for v in dru_u_plain dru_u_reuse \\
  dru_u_dedup dru_u_both dru_s_plain dru_s_reuse dru_s_dedup dru_s_both; do \\
  (cd variants/$v && cargo build --release); done
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SIZES = [64, 256, 1024, 4096, 16384]
SEED_U = "0x5eed_d3d0_0725_0001"
SEED_S = "0x5eed_d3d0_0725_0002"

CARGO = """\
[workspace]
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
"""

# Each strategy body runs one record cycle: seed a record, apply UPDATES field
# writes, then COMPARES equality checks against the previous cycle's record.
# `sc` is the scratch (arena + table), `k`/`b` the loop key and input byte.

PLAIN = """\
                let mut cur = sc.alloc(seed);
                for u in 0..UPDATES {
                    let nn = sc.copy_of(cur);
                    sc.arena[nn][u & (FIELDS - 1)] = u as i64;
                    cur = nn;
                }
                for _ in 0..COMPARES {
                    if sc.struct_eq(cur, prev) { acc ^= 1; }
                }
                prev = cur;"""

REUSE = """\
                let mut cur = sc.alloc(seed);
                for u in 0..UPDATES {
                    if shared_at(k, u) {
                        let nn = sc.copy_of(cur);
                        sc.arena[nn][u & (FIELDS - 1)] = u as i64;
                        cur = nn;
                    } else {
                        sc.arena[cur][u & (FIELDS - 1)] = u as i64;
                    }
                }
                for _ in 0..COMPARES {
                    if sc.struct_eq(cur, prev) { acc ^= 1; }
                }
                prev = cur;"""

DEDUP = """\
                let fresh = sc.alloc(seed);
                let mut cur = sc.intern(fresh);
                for u in 0..UPDATES {
                    // every value is table-held, so an update can never be in place
                    let nn = sc.copy_of(cur);
                    sc.arena[nn][u & (FIELDS - 1)] = u as i64;
                    cur = sc.intern(nn);
                }
                for _ in 0..COMPARES {
                    if cur == prev { acc ^= 1; }
                }
                prev = cur;"""

BOTH = """\
                let mut cur = sc.alloc(seed);
                let mut interned = false;
                for u in 0..UPDATES {
                    if shared_at(k, u) {
                        // may be shared: copy, then hand the copy to the table.
                        // From here it is table-held and can never mutate in place.
                        let nn = sc.copy_of(cur);
                        sc.arena[nn][u & (FIELDS - 1)] = u as i64;
                        cur = sc.intern(nn);
                        interned = true;
                    } else if interned {
                        // already table-held: the table is a referrer, so even a
                        // locally-unique verdict cannot license an in-place write.
                        let nn = sc.copy_of(cur);
                        sc.arena[nn][u & (FIELDS - 1)] = u as i64;
                        cur = nn;
                        interned = false;
                    } else {
                        sc.arena[cur][u & (FIELDS - 1)] = u as i64;
                    }
                }
                for _ in 0..COMPARES {
                    let eq = if interned && prev_interned { cur == prev } else { sc.struct_eq(cur, prev) };
                    if eq { acc ^= 1; }
                }
                prev = cur;
                prev_interned = interned;"""

LIB_TMPL = """\
// AUTO-GENERATED by gen_dedup_vs_reuse.py. Dedup-vs-reuse composition axis:
// {frac}% may-be-shared, strategy = {strat}. Within a bench every strategy returns
// the byte-identical accumulator; the harness cross-validates output, which is what
// enforces the rule that an optimisation must not change the answer. The scratch
// (record arena + intern table) is allocated once per process and cleared outside
// the timed region; the seed index is folded with the FFI input byte so no cycle
// can be hoisted or const-folded.
#![allow(dead_code, unused_mut)]
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::cell::RefCell;

const FIELDS: usize = 16;
const NSEED: usize = 1024;
const NDISTINCT: usize = 64; // duplicate-rich: dedup must have something to find
const UPDATES: usize = 8;
const COMPARES: usize = 2;
const SHARED_PCT: u64 = {frac};
const CAP: usize = 16384 * (UPDATES + 1) + 8;
const TBITS: usize = 18;
const TSIZE: usize = 1 << TBITS;
const EMPTY: u32 = u32::MAX;

type Rec = [i64; FIELDS];

/// Is the emitter's verdict at cycle `k`, update `u`, "may be shared"? A fixed
/// hash so every strategy sees the identical verdict sequence.
#[inline(always)]
fn shared_at(k: usize, u: usize) -> bool {{
    let h = ((k * UPDATES + u) as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    (h >> 33) % 100 < SHARED_PCT
}}

struct Scratch {{
    arena: Vec<Rec>,
    slots: Vec<u32>,
    top: usize,
}}

impl Scratch {{
    fn new() -> Self {{
        Self {{ arena: vec![[0i64; FIELDS]; CAP], slots: vec![EMPTY; TSIZE], top: 0 }}
    }}
    fn reset(&mut self) {{
        self.top = 0;
        self.slots.iter_mut().for_each(|s| *s = EMPTY);
    }}
    #[inline(always)]
    fn alloc(&mut self, seed: &Rec) -> usize {{
        let i = self.top;
        self.arena[i] = *seed;
        self.top += 1;
        i
    }}
    #[inline(always)]
    fn copy_of(&mut self, src: usize) -> usize {{
        let i = self.top;
        self.arena[i] = self.arena[src];
        self.top += 1;
        i
    }}
    #[inline(always)]
    fn struct_eq(&self, a: usize, b: usize) -> bool {{
        self.arena[a] == self.arena[b]
    }}
    /// Insert `idx`, or return the index already holding a structurally equal
    /// record and give `idx`'s slot back. Open addressing, linear probe.
    #[inline(always)]
    fn intern(&mut self, idx: usize) -> usize {{
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for f in self.arena[idx].iter() {{
            h ^= *f as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }}
        let mut i = (h as usize) & (TSIZE - 1);
        loop {{
            let s = self.slots[i];
            if s == EMPTY {{
                self.slots[i] = idx as u32;
                return idx;
            }}
            if self.arena[s as usize] == self.arena[idx] {{
                if idx + 1 == self.top {{ self.top -= 1; }}
                return s as usize;
            }}
            i = (i + 1) & (TSIZE - 1);
        }}
    }}
}}

thread_local! {{
    static SCRATCH: RefCell<Scratch> = RefCell::new(Scratch::new());
    static SEEDS: Vec<Rec> = {{
        let mut v = vec![[0i64; FIELDS]; NSEED];
        let mut s: u64 = 1;
        for k in 0..NDISTINCT {{
            for f in v[k].iter_mut() {{
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
                *f = (s >> 40) as i64;
            }}
        }}
        for k in NDISTINCT..NSEED {{
            v[k] = v[k & (NDISTINCT - 1)];
        }}
        v
    }};
}}

#[bench_variant("{name}", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    SEEDS.with(|seeds| {{
        SCRATCH.with(|cell| {{
            let mut sc = cell.borrow_mut();
            let sc = &mut *sc;
            sc.reset(); // untimed: the arena and table start clean every call
            timed! {{ run {{
                let mut acc: u64 = 0;
                let mut prev: usize = 0;
                let mut prev_interned = false;
                let _ = &prev_interned;
                for (k, &b) in input.iter().enumerate() {{
                    let seed = &seeds[(k ^ b as usize) & (NSEED - 1)];
{body}
                    acc ^= sc.arena[cur_out(&sc.arena, prev)] as u64;
                }}
                output.copy_from_slice(&acc.to_le_bytes());
            }} }}
        }})
    }})
}}
"""

VARIANTS = [
    ("plain", "copy-on-write, structural equality", PLAIN),
    ("reuse", "in-place-when-unique, structural equality", REUSE),
    ("dedup", "hash-cons every value, id equality", DEDUP),
    ("both", "intern-or-reuse chosen per site, mixed equality", BOTH),
]


def write_variant(name, strat_tag, body, frac):
    d = os.path.join(HERE, "variants", name, "src")
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(HERE, "variants", name, "Cargo.toml"), "w") as f:
        f.write(CARGO.format(name=name))
    lib = LIB_TMPL.format(name=name, strat=strat_tag, body=body, frac=frac)
    # the accumulator folds the surviving record's first field; written plainly
    # rather than through a helper so the generated code has no indirection.
    lib = lib.replace("acc ^= sc.arena[cur_out(&sc.arena, prev)] as u64;",
                      "acc ^= sc.arena[prev][0] as u64;")
    with open(os.path.join(d, "lib.rs"), "w") as f:
        f.write(lib)
    return name


def bench_section(bench, title, baseline, seed, names):
    paths = ", ".join(f'"variants/{n}/target/release/{n}"' for n in names)
    out = [
        f"[bench.{bench}]",
        f'title = "{title}"',
        'workload = "realistic"',
        f"master_seed = {seed}",
        f"[bench.{bench}.normalise]",
        f'baseline = "{baseline}"',
        'mode = "subtract"',
    ]
    for n in SIZES:
        out.append(f"[[bench.{bench}.sizes]]")
        out.append(f"n = {n}")
        out.append(f"variants = [{paths}]")
    return "\n".join(out) + "\n"


def main():
    u_names = [write_variant(f"dru_u_{n}", t, b, 20) for n, t, b in VARIANTS]
    s_names = [write_variant(f"dru_s_{n}", t, b, 60) for n, t, b in VARIANTS]
    sec = (
        bench_section(
            "dedup_reuse_unique",
            "Dedup vs reuse composition, 20% shared: the templating norm",
            "dru_u_plain", SEED_U, u_names)
        + "\n"
        + bench_section(
            "dedup_reuse_shared",
            "Dedup vs reuse composition, 60% shared: reuse's adversarial regime",
            "dru_s_plain", SEED_S, s_names)
    )
    frag_dir = os.path.join(HERE, "carrier_fragments")
    os.makedirs(frag_dir, exist_ok=True)
    frag = os.path.join(frag_dir, "dedup_vs_reuse.toml")
    with open(frag, "w") as f:
        f.write("# >>> dedup_vs_reuse (generated)\n" + sec)
    print("generated variants:", ", ".join(u_names + s_names))
    print("wrote [bench.dedup_reuse_unique] + [bench.dedup_reuse_shared] to carrier_fragments/dedup_vs_reuse.toml")


if __name__ == "__main__":
    main()
