#!/usr/bin/env python3
"""Generate the Project field-access bench: record.field lowering strategies, at
a MONOMORPHIC site and a POLYMORPHIC site, so the inline-cache hazard is visible.
Audit-fixed replacement for `project-field-access/project.zig`, which ran outside
the harness with a hand-timed loop and a header-only CSV.

The fix, structurally:
  - The record sets (1024 records) are built ONCE per process via OnceLock
    (runtime state, so the access cannot be const-folded), never inside the timed
    region.
  - The record index is folded with the FFI input byte, so the access cannot be
    hoisted.
  - Within a bench every strategy returns the byte-identical field value, so the
    harness cross-validation catches a wrong strategy. Direct-offset is therefore
    STRUCTURALLY absent from the polymorphic bench: with a rotating shape there is
    no single fixed offset, so a fixed-offset read returns the wrong value and
    would fail cross-validation. That is the invalidity the finding names, now
    enforced by the harness rather than a footnote.

Two benches:
  project_field_mono  the access site sees one shape repeatedly (a loop over a
                      homogeneous collection). Strategies: direct-offset
                      (baseline), linear-scan, hash-lookup, inline-cache. The
                      inline cache should reach near-direct speed here (guard
                      hits every access).
  project_field_poly  the access site sees four rotating shapes. Strategies:
                      linear-scan, hash-lookup (baseline), inline-cache. The
                      load-bearing hazard: the inline cache is the WORST here
                      (the shape guard misses almost every access and pays a
                      failed guard plus a refill on top of the fallback scan), so
                      it must self-disable to a hash lookup at a megamorphic site.

Run: python3 gen_project_field_access.py && for v in project_mono_direct \
  project_mono_linear project_mono_hash project_mono_ic project_poly_linear \
  project_poly_hash project_poly_ic; do (cd variants/$v && cargo build --release); done
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SIZES = [64, 256, 1024, 4096, 16384]
SEED_MONO = "0x5eed_f1e1_0503_0003"
SEED_POLY = "0x5eed_f1e1_0503_0004"

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

# site build bodies (identical across the strategies within a site).
BUILD_MONO = """\
fn build_recs() -> Vec<Rec> {
    let mut v = Vec::with_capacity(NRECS);
    for ri in 0..NRECS {
        let mut names = [0u32; FIELDS];
        let mut vals = [0i64; FIELDS];
        for i in 0..FIELDS {
            names[i] = (100 + i) as u32;
            vals[i] = ((ri * FIELDS + i) * 11) as i64;
        }
        v.push(Rec { shape_id: 1, names, vals });
    }
    v
}
"""
BUILD_POLY = """\
fn build_recs() -> Vec<Rec> {
    let mut v = Vec::with_capacity(NRECS);
    for ri in 0..NRECS {
        let mut names = [0u32; FIELDS];
        let mut vals = [0i64; FIELDS];
        let rot = ri & 3; // four rotating shapes defeat a monomorphic inline cache
        for i in 0..FIELDS {
            let perm = (i + rot) & (FIELDS - 1);
            names[i] = (100 + perm) as u32;
            vals[i] = ((ri * FIELDS + perm) * 11) as i64;
        }
        v.push(Rec { shape_id: (1 + rot) as u32, names, vals });
    }
    v
}
"""

# strategy access expressions (yield i64). {PRE} declares any pre-timed state.
DIRECT = {"pre": "", "expr": "rec.vals[OFF]"}
LINEAR = {"pre": "", "expr": """{
                let mut r = -1i64;
                let mut i = 0;
                while i < FIELDS {
                    if rec.names[i] == NAME {
                        r = rec.vals[i];
                        break;
                    }
                    i += 1;
                }
                r
            }"""}
HASH = {"pre": "", "expr": """{
                let mut h = (NAME.wrapping_mul(2654435761) as usize) & (FIELDS - 1);
                let mut r = -1i64;
                let mut p = 0;
                while p < FIELDS {
                    if rec.names[h] == NAME {
                        r = rec.vals[h];
                        break;
                    }
                    h = (h + 1) & (FIELDS - 1);
                    p += 1;
                }
                r
            }"""}
IC = {
    "pre": "    let mut ic_shape: u32 = u32::MAX;\n    let mut ic_off: usize = 0;",
    "expr": """if rec.shape_id == ic_shape {
                rec.vals[ic_off] // guard hit: direct load
            } else {
                let mut r = -1i64;
                let mut i = 0;
                while i < FIELDS {
                    if rec.names[i] == NAME {
                        ic_shape = rec.shape_id;
                        ic_off = i;
                        r = rec.vals[i];
                        break;
                    }
                    i += 1;
                }
                r // miss: scan + refill
            }""",
}

LIB_TMPL = """\
// AUTO-GENERATED by gen_project_field_access.py. Project field-access axis:
// {site} site, strategy = {strat}. Within a bench every strategy returns the
// byte-identical field value; the harness cross-validates output. The record set
// is built once per process via OnceLock; the record index is folded with the
// FFI input byte so the access cannot be hoisted.
#![allow(dead_code)] // NAME unused by direct-offset; OFF unused by the lookup strategies
use mockspace_bench_core::{{timed, FfiBenchCall}};
use mockspace_bench_macro::bench_variant;
use std::sync::OnceLock;

const FIELDS: usize = 8;
const NRECS: usize = 1024;
const NAME: u32 = 105; // the interned field-name id under access
const OFF: usize = 5; // its compile-resolved slot in the static (monomorphic) shape

#[repr(C)]
#[derive(Clone, Copy)]
struct Rec {{
    shape_id: u32,
    names: [u32; FIELDS], // interned field-name ids
    vals: [i64; FIELDS],
}}
// the layout the finding names: 8-field record, fixed size.
const _: () = assert!(FIELDS == 8 && NRECS.is_power_of_two() && core::mem::size_of::<Rec>() == 104);

static RECS: OnceLock<Vec<Rec>> = OnceLock::new();
{build}
#[bench_variant("{name}", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {{
    let recs = RECS.get_or_init(build_recs);
{pre}
    timed! {{ run {{
        let mut acc: u64 = 0;
        for (k, &b) in input.iter().enumerate() {{
            let rec = &recs[(k ^ b as usize) & (NRECS - 1)];
            let v: i64 = {expr};
            acc ^= v as u64;
        }}
        output.copy_from_slice(&acc.to_le_bytes());
    }} }}
}}
"""

MONO = [
    ("project_mono_direct", "direct-offset", DIRECT),
    ("project_mono_linear", "linear-scan", LINEAR),
    ("project_mono_hash", "hash-lookup", HASH),
    ("project_mono_ic", "inline-cache", IC),
]
POLY = [
    ("project_poly_linear", "linear-scan", LINEAR),
    ("project_poly_hash", "hash-lookup", HASH),
    ("project_poly_ic", "inline-cache", IC),
]


def write_variant(name, strat_tag, strat, site, build):
    d = os.path.join(HERE, "variants", name, "src")
    os.makedirs(d, exist_ok=True)
    with open(os.path.join(HERE, "variants", name, "Cargo.toml"), "w") as f:
        f.write(CARGO.format(name=name))
    lib = LIB_TMPL.format(
        site=site, strat=strat_tag, build=build, name=name,
        pre=strat["pre"], expr=strat["expr"],
    )
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
    mono_names = [write_variant(n, t, s, "monomorphic", BUILD_MONO) for n, t, s in MONO]
    poly_names = [write_variant(n, t, s, "polymorphic", BUILD_POLY) for n, t, s in POLY]
    sec = (
        bench_section(
            "project_field_mono",
            "Project field access, monomorphic site: direct offset vs inline cache vs hash vs linear",
            "project_mono_direct", SEED_MONO, mono_names)
        + "\n"
        + bench_section(
            "project_field_poly",
            "Project field access, polymorphic site: inline-cache hazard vs hash vs linear",
            "project_poly_hash", SEED_POLY, poly_names)
    )
    frag_dir = os.path.join(HERE, "carrier_fragments")
    os.makedirs(frag_dir, exist_ok=True)
    frag = os.path.join(frag_dir, "resolve_interner_project.toml")
    marker = "# >>> project_field_access (generated)"
    prior = ""
    if os.path.exists(frag):
        prior = open(frag).read()
        if marker in prior:
            prior = prior.split(marker)[0].rstrip() + "\n"
    with open(frag, "w") as f:
        f.write(prior.rstrip() + ("\n\n" if prior.strip() else "") + marker + "\n" + sec)
    print("generated variants:", ", ".join(mono_names + poly_names))
    print("wrote [bench.project_field_mono] + [bench.project_field_poly] to carrier_fragments/resolve_interner_project.toml")


if __name__ == "__main__":
    main()
