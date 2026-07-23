# Extracting the semantic-matrix scaffolding upstream

The carrier matrix is two things bolted together: a genuinely reusable way to author a
composited benchmark (one bench, many isolated cells, each timed individually, swept across
parameter axes) and a pile of carrier-specific program-generation. The reusable half is worth
extracting into mockspace so any project gets it. The panel is the argument: the measurement
scaffolding is subtle enough that the first hand-rolled attempt shipped four distinct
distortions (checksum co-timed in the hot loop, a wire-vs-predecoded baseline asymmetry, a
reps-variant fidelity witness, and the entire `S` setup term hidden in untimed prep), and it
took four experts to find them. If that scaffolding is extracted once, gotten right once, and
handed to every consumer through an ergonomic macro, nobody re-derives it and nobody re-ships
those bugs.

## The three layers

**Layer 0, already upstream (`mockspace-bench-harness` + `-core` + `-macro`).** The cdylib-per-
variant isolation, the subprocess driver with the ABI-hash check, `timed_calibrated!` and the
CNTVCT calibration, the CSV/meta/findings emission and the analysis (CV, autocorrelation,
throttle detection), the kperf PMU path, and the mechanical multi-axis codegen
(`matrix::{MatrixSpec, AxisSpec, AxisValue, expand, render, render_variant, generate}`). This
takes a raw `lib_template` string plus per-cell substitution maps and emits the variant crates
and the `bench.toml` section. It knows nothing about how to measure honestly; it is the
transport.

**Layer 1, hand-rolled in the consumer today, the extraction target.** Everything in
`vehje/mock/benches/src/bin/gen_matrix.rs` that is not carrier-specific: the canonical
measurement TEMPLATE (the anti-hoist acc-chain seeded from `output`, the shared seed table, the
`ITERS` loop, the `timed_calibrated!` wrap, the checksum fold kept out of the per-node loop, the
`output` write, the reps-variant fidelity note), the `Cell { tag, prep, body, features }` model,
the `spec` / `spec_sized` builders that wrap a `MatrixSpec` with that template plus a baseline
and a normalise mode, the family/sweep pattern (`PROFILES.iter().map(|p| spec(..))`, one bench
per parameter point), and the measurement disciplines the panel validated (fixed seeds, S in
prep vs I in body, per-8-input equalisation for the SIMD family, null-floor differencing,
live-out checksum symmetry).

**Layer 2, genuinely consumer-specific, stays put.** The program generator and its profiles
(`gen.rs`, real/madd/tight/...), the prep fragments that build carrier programs (`wire_prep`,
`predecode_prep`, `program_prep`, `bytes_prep`), the op semantics (`ops.rs`), and the cells
themselves (dispatch shapes, valrepr, the JIT paths). None of this generalises; it is what
makes the matrix a *carrier* matrix.

The proposal is: lift Layer 1 into a new `mockspace-bench-matrix` crate on top of the existing
harness, with the panel's corrections baked into the scaffolding, and an ergonomic macro as the
authoring surface.

## The design win that changes everything: function cells, not stringly-typed fragments

The current authoring is Rust-as-strings. A cell is `(tag, prep: String, body: String,
features)`, where `prep`/`body` are Rust source concatenated into a generated crate. The
consequences are exactly what you would expect from stringly-typed code: no type checking until
a variant is built, no IDE support, no borrow-checker feedback while authoring, and errors that
surface as a failed `cargo build` of a generated crate three steps removed from the source. It
exists only because each cell becomes a separate cdylib (for isolation), so the body cannot be
an inline closure in the consumer crate.

But it does not have to be a *string*. The cells can be real functions in the consumer crate,
and each generated variant crate depends on the consumer crate and calls back into it:

```rust
// generated variant crate, the whole of it:
#[bench_variant("carrier_disp_real_switch", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    mockspace_bench_matrix::scaffold::<N, _, _>(
        input, output,
        carrier::cells::disp::setup::<N>,     // the S term
        carrier::cells::disp::switch::<N>,    // the I term
    )
}
```

`carrier::cells::disp::switch` is a real, type-checked, IDE-supported, borrow-checked function.
Per-variant fat LTO still inlines it into the isolated cdylib, so the isolation the whole design
exists for is preserved (LTO monomorphises and inlines the callee into each variant; no
cross-variant sharing). The stringly-typed prep/body disappears, and with it a whole class of
authoring pain. This is the single highest-value change.

## The scaffolding provides the discipline (so consumers cannot omit it)

`scaffold` is the canonical, correct measurement wrapper the panel converged on. It owns:

- **The anti-hoist chain.** The `acc` seeded from `output[0]`, folded, written back, so the
  calibrated reps form a real loop-carried dependency the optimiser cannot collapse. Consumers
  never see it and cannot get it wrong.
- **The shared seed table.** Fixed seeds identical across sizes and cells, so a cross-size or
  cross-cell comparison varies only the thing under test. (Panel finding: `input[k % N]` drew
  different seeds per size.)
- **The S-vs-I split, measured automatically.** This is the panel's number-one finding turned
  into a structural guarantee. The scaffold times `setup` (the S term, once) and the per-
  iteration `op` (the I term) *separately*, and emits both as columns. Every matrix then carries
  the `total(k) = S + k*I` breakeven data for free, and it becomes *impossible* to hide a JIT's
  codegen cost or an optimiser's pass cost in untimed prep, because prep IS the timed setup. The
  hand-authored `carrier_setup_*` family stops being necessary; it is what every cell already
  reports.
- **The fidelity anchor, stated honestly.** The scaffold documents (and the docs generator
  surfaces) that the reps-variant `output` is not a cross-cell fidelity witness under
  calibration, and points at the consumer's own byte-exact cross-validation tests as the anchor.
  One place, said once, correctly.
- **The checksum/keep-alive helper** kept out of the hot loop by construction (the op returns a
  fold contribution; the scaffold folds it once per iteration, not per inner element).

## Regime modes, not hand-authored families

Two of the new carrier families are really scaffold *options*, not domain benches:

- **Cold / aliased-predictor** (the `carrier_coldcycle_*` family): cycling M distinct states
  round-robin so no single program's dispatch sequence is memorised. This is a `regime:
  ColdCycle(M)` option on the scaffold: the consumer provides a `setup` that returns M states,
  the scaffold cycles them. Every matrix can ask for the cold regime without authoring a family.
- **First-touch latency**: report the probe pass alone alongside the calibrated median. A
  `first_rep: true` scaffold flag (the probe is already timed; plumb it to a column). Needs a
  small harness change but generalises to every bench.

The warm/cold/first-touch axis becomes a property a consumer selects, not scaffolding it
re-implements.

## The ergonomic macro

The authoring surface is a `bench_matrix!` macro that takes real Rust, enumerates cells and
sweeps, and emits the variant crates plus the `bench.toml` section:

```rust
bench_matrix! {
    name: "carrier_dispatch",
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "switch",
    regime: warm,                       // or cold_cycle(16), or first_touch

    // the S term: runs once, timed as setup. Real Rust, type-checked.
    setup |profile, N| {
        let bytes = carrier::encode(carrier::generate(profile, N), REC24);
        let d = carrier::Decoded::parse(&bytes, REC24);
        State { r: vec![0u64; d.node_count], d }
    }

    // each cell: the I term, one measured op given the shared state + a seed.
    cell switch     |s, seed| { carrier::interpret(&s.d, seed, &mut s.r); carrier::checksum(&s.r) }
    cell fntable    |s, seed| { carrier::interpret_fntable(&s.d, seed, &mut s.r); carrier::checksum(&s.r) }
    cell threaded   #[feature = "threaded"] |s, seed| { carrier::interpret_threaded(&s.d, seed, &mut s.r); carrier::checksum(&s.r) }
    cell nullfloor  |s, seed| { carrier::interpret_nulldispatch(&s.d, seed, &mut s.r); carrier::checksum(&s.r) }
}
```

Every cell above is real Rust that the compiler checks at authoring time. The macro emits, per
(profile, cell), an isolated variant crate wired to `scaffold`, with the anti-hoist, the seed
table, the S/I split, the calibration, the fidelity discipline, and the PMU all provided. A cell
that needs different setup gets a per-cell `setup { .. }` override; the shared `setup` is the
default. Null-floor differencing is a `baseline: switch, floor: nullfloor` pair the reporter
consumes.

The macro likely wants a companion build step (a generator binary or a `build.rs`) to emit the
variant crates, since the crates must exist on disk before `cargo` builds them; the macro
captures the token trees and the generator writes them. That is the one piece of real
machinery to design carefully (proc-macro capturing closures for emission into sibling crates),
and it is the reason this is a mockspace design round rather than an afternoon.

## What ships where

- `mockspace-bench-matrix` (new crate): `scaffold`, the `Cell`/family model, the canonical
  template, the S/I split, the regime modes, the null-floor differencing, the `bench_matrix!`
  macro. Depends on `-harness` / `-core` / `-macro`.
- `mockspace-bench-harness`: the small additions the scaffold needs (a first-rep column; an S/I
  two-timer entry point next to `timed_calibrated!`; optionally a reps-invariant fidelity slot
  if we decide the harness cross-check is worth saving).
- The carrier matrix becomes the first consumer and the dogfood: `gen_matrix.rs` shrinks to the
  `bench_matrix!` invocations plus Layer 2 (the program generator and cells), and every
  discipline the panel corrected is inherited rather than restated.

## Open questions for op

1. **Scope of the first cut.** Minimal (extract the template + Cell + family builders as-is,
   string-mode, so the discipline is shared immediately) versus full (function-cell macro + the
   automatic S/I split + regime modes). The minimal cut is a few hours and removes the
   copy-paste; the full cut is a mockspace design round and is where the real ergonomic and
   correctness wins live. I lean full, staged: land the scaffold + S/I split first (that is the
   panel's #1 finding made structural), then the macro.
2. **Where it lives.** A new `mockspace-bench-matrix` crate (clean separation) versus folding it
   into `mockspace-bench-harness` (fewer crates). I lean separate: the harness is the transport,
   the matrix layer is the opinionated discipline, and keeping them apart lets a consumer use the
   raw harness without the opinions.
3. **The macro's emission mechanism.** Proc-macro capturing closures for a generator to emit,
   versus keeping an explicit generator binary that the macro feeds. This is the one genuine
   design risk and wants its own sketch.
4. **Whether to save the harness cross-validation** (reps-invariant output) or formally bless the
   consumer-tests-are-the-anchor position. The panel showed the harness cross-check is inert
   under calibration; making it meaningful is a harness change, and it may not be worth it.
