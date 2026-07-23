# mockspace-bench-matrix: authoring surface + the function-cell mechanism

**Date:** 2026-07-23
**Phase:** design (extraction of Layer 1 from the carrier matrix into an upstream crate)
**Scope:** the `bench_matrix!` authoring surface, the module boundary, the load-bearing function-cell mechanism, and the own-vs-expose split for the panel's disciplines
**Source topics:** `extraction_proposal.md`, the four panel findings, `bench-harness/src/matrix.rs`, `bench-core/src/lib.rs`, `mock/benches/src/bin/gen_matrix.rs`

Trace one cell end to end before designing anything, because the cell is where the whole argument lives. Today a dispatch cell is three strings: a `prep` (`static PREP: OnceLock<Vec<u8>> = ...; let d = c::ir::Decoded::parse(...);`), a `body` (`c::interpret(&d, seed, &mut r); acc ^= c::checksum(&r);`), and a feature list. Those strings are concatenated into a generated crate's `src/lib.rs`, wrapped in the fixed template's `timed_calibrated!` skeleton, and each generated crate is built as its own cdylib under fat LTO with one codegen unit. The isolation the whole design exists for comes from that per-cdylib boundary: cell A and cell B never share a compiled object, never share a codegen unit, and run in separate subprocesses, so measuring one cannot contaminate the other.

Here is the thing the proposal noticed and the thing this design turns into a load-bearing fact: the string `body` is *already a call back into the consumer crate*. `c::interpret` is `vehje_bench_carrier::interpret`. The variant crate already declares `vehje-bench-carrier = { path = "../../carrier" }` as a dependency, and fat LTO already inlines `interpret` into the timed region. The stringly-typed cell is not stringly-typed because it *has* to splice a body; it is stringly-typed because nobody separated "what code runs" from "where the code physically lives." The body is Rust that happens to live in a string. It could live in a `pub fn` in the carrier crate, and the variant crate could name it by path, and fat LTO would inline it into the isolated cdylib *identically*, because the string form already relies on exactly that inlining across exactly that crate boundary.

So the isolation claim is not a new bet. It is the bet the current design already makes and already trusts. The function-cell form changes the cell from a spliced string into a named, type-checked, borrow-checked, IDE-visible `pub fn`, and changes the variant crate from "a crate containing the cell body" into "a crate containing a one-line call to the cell by path." The measured machine code is the same after monomorphization plus fat LTO, provided one detail holds that the design must make structural: the scaffold is generic over the cell type and takes it as an `FnMut` type parameter (a zero-sized fn-item or closure), never as a `fn` pointer and never as `&dyn Fn`. A `fn` pointer reintroduces an indirect call the optimizer may decline to devirtualize; a generic parameter guarantees monomorphization inlines the cell with no indirection. That single constraint is the entire soundness argument, and it is enforceable by the scaffold's signature.

The rest of this document designs the authoring surface that this unlocks, draws the module boundary, works the mechanism (including the "how does a macro get closures into on-disk sibling crates" question, whose honest answer is "it does not, and it must not try"), and assigns each of the panel's four distortions to either the scaffold (made impossible to get wrong) or the consumer (exposed, with the scaffold providing the corrective).

## 1. The load-bearing mechanism (settle this first; the surface depends on it)

### 1.1 Why the function-cell form preserves isolation

The measured region in a variant cdylib is `scaffold(...)` with the cell inlined. Under `lto = "fat"` plus `codegen-units = 1` (the variant `Cargo.toml` already sets both, `matrix.rs:256`), the carrier rlib ships as LLVM bitcode and the final cdylib link re-optimizes from bitcode, monomorphizing `cell::<N>` and inlining it into the timed loop. Two consequences, both matching the string form exactly:

- **No cross-cell contamination.** Each variant is its own cdylib and its own subprocess. The carrier rlib is a *source* dependency shared across siblings, not a shared *compiled object*: fat LTO pulls the needed monomorphizations into each cdylib and optimizes them there, so cell A's cdylib contains cell A optimized in cell A's context and nothing of cell B. This is identical to how `c::interpret` and `c::interpret_fntable` are isolated today.
- **Full within-variant optimization.** The cell is inlined into `scaffold`'s timed loop as it would be inlined into a real hot call site. Nothing about the callback defeats an optimization the isolation measures, because the callback is resolved at monomorphization, before any optimization runs.

The one way to break this, and the one thing the scaffold signature forbids:

```rust
// CORRECT: generic over the cell type. Monomorphized, inlined, no indirection.
pub fn warm<const N: usize, St, Setup, Cell>(
    input: &[u8; N], output: &mut [u8; 8], setup: Setup, mut cell: Cell,
) -> FfiBenchCall
where Setup: FnOnce(usize) -> St, Cell: FnMut(&mut St, u64) -> u64 { /* ... */ }

// WRONG: fn pointer. Introduces an indirect call the optimizer may not
// devirtualize under LTO; the measured region now includes a call the real
// deployment would not have. Never expose this shape.
pub fn warm_bad<const N: usize, St>(
    input: &[u8; N], output: &mut [u8; 8],
    setup: fn(usize) -> St, cell: fn(&mut St, u64) -> u64,
) -> FfiBenchCall { /* ... */ }
```

Because the cells are passed as generic fn-items, the entire "does the callback defeat the measurement" worry resolves to "no, as long as the signature is generic, which it is." The design mandates the generic form and never ships the pointer form.

### 1.2 The real question: how does the macro get cells into sibling crates?

The proposal frames the risk as "proc-macro capturing closures for emission into on-disk sibling crates." That framing is the trap, and naming why is the most useful thing this section does.

A proc-macro runs *during* the compile of the consumer crate. The sibling crates must exist on disk *before* cargo builds them. You cannot have cargo build siblings that are produced by a macro that runs inside a build that is downstream of building the siblings. Worse, if a proc-macro captured the cell closure *tokens* and re-emitted them as text into sibling files, the cell bodies would live only inside a closure that the consumer crate never calls (it only `stringify!`s it), so they would not be type-checked in the consumer crate. That is the current string approach wearing a nicer coat: you would have thrown away the entire type-checking win to buy back the exact problem you set out to remove.

The unlock is to stop trying to move the cell *bodies* at all. **The sibling crate never contains a cell body. It contains a call to the cell by path.** The generator that writes the sibling crates needs to know the cell's *path* (a short string like `vehje_bench_carrier::bench::carrier_dispatch::cell_switch`), not its body. The body stays in the consumer crate, compiled once, type-checked once, inlined into each cdylib by LTO. Capturing a path is trivial; capturing a body is the trap. Decouple "declare the matrix (data, includes paths)" from "emit the crates (writes files)."

Three options for the machinery, with costs. All three share: cells are `pub fn`s in the consumer crate; siblings reference them by path; a generator binary (a normal, cargo-tracked `src/bin/gen_matrix.rs`, exactly as today) writes the sibling crates. They differ only in how the matrix *declaration* travels from the authoring site to the generator.

**Option A (recommended): declarative macro emits typed cells + a `MatrixDecl` manifest; a generator binary reads it.**

`bench_matrix! { ... }` is a `macro_rules!` macro invoked in a `pub mod bench` of the consumer *library* crate. It expands, in that crate, to:

1. one `pub fn` per `cell` and one `pub fn` per `setup` (real Rust, type-checked with the rest of the crate on every `cargo check`), and
2. a `pub fn matrix_decls() -> Vec<mockspace_bench_matrix::MatrixDecl>` that returns the matrix as data: name, sizes, sweep values, baseline, floor, regime, and the *paths* of the setup and cell functions, computed with `concat!(module_path!(), "::cell_switch")` so the path can never drift from the function it names.

The generator binary does:

```rust
// mock/benches/src/bin/gen_matrix.rs, the whole of it
fn main() -> std::io::Result<()> {
    let decls = vehje_bench_carrier::bench::matrix_decls();
    mockspace_bench_matrix::generate_all(&decls, std::path::Path::new("."))?;
    Ok(())
}
```

`generate_all` maps each `MatrixDecl` onto a `harness::MatrixSpec` whose `lib_template` is the *single canonical template owned by mockspace-bench-matrix* (the anti-hoist, seed table, digest, scaffold call: never authored by the consumer), then calls `harness::generate`. Cost: the generator step is unchanged from today (`cargo run --bin gen_matrix`). The macro does *zero* filesystem IO and never touches closure bodies. The only strings are function paths, and the macro generates both the function and the path expression, so they are one source and cannot disagree. Because the macro references cells by path and never introspects a body, it is a `macro_rules!` macro; there is no proc-macro and no closure capture anywhere. This is the de-risking: the scary design risk was an artifact of the body-capture framing, which we dropped.

**Option B: `build.rs` in the consumer runs the generator at build time.** The consumer's `build.rs` writes the sibling crates during the consumer's own build. This fails on phase ordering: `build.rs` runs *before* the consumer crate is compiled, so it cannot read a `const`/`fn` the macro produced (the crate is not built yet). To make it work you would move the matrix declaration into a data file (TOML/RON) that both the macro-less authoring and `build.rs` read, which loses inline cell authoring and lands you back at "cells are named functions referenced by path in a TOML," which is Option A with a worse front-end. `build.rs` also does heavy filesystem writes on every build, which is slow and surprising. Rejected: the siblings are a *separate set of cargo build targets*, not an artifact of the consumer's own compilation, so the consumer's `build.rs` is the wrong place to emit them.

**Option C: attribute macro + inventory-style self-registration.** `#[bench_matrix(...)]` on the cell module leaves the cells in place, generates the scaffold wrappers, and registers the `MatrixDecl` into a distributed slice (`linkme`) so the generator enumerates all matrices without a hand-written `matrix_decls()` aggregator. Cost: a `linkme` dependency and distributed-slice machinery. Benefit: each `bench_matrix!` self-registers, so adding a matrix needs no edit to a central list. This is orthogonal to the core mechanism: it changes only how declarations are *aggregated*, not how cells reach siblings. Adopt it as a later ergonomic refinement if the central `matrix_decls()` list becomes a maintenance point; it is not needed for the first cut.

Recommendation: **Option A now, Option C's self-registration later if the aggregator list grows annoying.** The load-bearing move in all three is identical and is the actual answer to the proposal's open question 3: cells are referenced by path, never captured as tokens; the macro emits typed functions plus a data manifest; a normal generator binary writes the siblings. No proc-macro captures any closure, because no closure body ever leaves the consumer crate.

### 1.3 What a generated sibling actually looks like

`generate_all` fills the canonical template (owned by the matrix crate) per (sweep value, cell). For the dispatch matrix, profile `real`, cell `switch`:

```rust
// variants/carrier_disp_real_switch/src/lib.rs, generated, never hand-edited
use mockspace_bench_core::FfiBenchCall;
use mockspace_bench_macro::bench_variant;

#[bench_variant("carrier_disp_real_switch", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    mockspace_bench_matrix::scaffold::warm::<N, _, _>(
        input,
        output,
        move |n| vehje_bench_carrier::bench::carrier_dispatch::setup("real", n),
        vehje_bench_carrier::bench::carrier_dispatch::cell_switch,
    )
}
```

`Cargo.toml` is what the transport already emits (`matrix.rs:246-256`): cdylib, fat LTO, cgu 1, plus `vehje-bench-carrier = { path = "../../carrier", features = [...cell's features...] }`. The setup and cell are the carrier crate's own `pub fn`s. The profile literal `"real"` is baked in by the generator (it is the sweep value for this sibling). The canonical scaffold call, the anti-hoist, the seed table, and the digest are all inside `scaffold::warm`, so this file has no measurement logic to get wrong. That is the point.

## 2. The authoring surface

### 2.1 The `bench_matrix!` shape

One invocation declares one *family* of benches: the `sweep` axis produces one bench per value (separate baselines, separate normalise blocks, because different sweep values are different programs and are not directly comparable, which is exactly how the current `PROFILES.iter().map(|p| spec(..))` families work), and the `cell` list is the set of variants compared *within* each bench. `sizes` is the harness's per-size monomorphization. `setup` is the S term, shared by default. `cell` is the I term.

```
bench_matrix! {
    name: "carrier_dispatch",              // bench-family prefix
    crate_path: vehje_bench_carrier,       // how siblings name the consumer crate
    sweep profile in ["real","madd","tight","scatter","wideselect","leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "switch",                    // the ratio denominator, per bench
    floor:    "nullfloor",                 // null-floor differencing target (2.4)
    regime:   warm,                        // warm | cold_cycle(M) | first_touch

    setup |profile: &str, n: usize| -> DispatchState { /* S term, real Rust */ }

    cell switch    |s, seed| { /* I term, returns one keep-alive u64 */ }
    cell fntable   |s, seed| { /* ... */ }
    cell threaded  #[feature = "threaded"] |s, seed| { /* ... */ }
    cell nullfloor |s, seed| { /* ... */ }
}
```

Grammar notes:

- `setup |args| -> St { .. }` becomes `pub fn setup(profile: &str, n: usize) -> St { .. }`. `St` is the shared state type; every cell in the family reads it. One setup, one state type, is the structural guarantee behind the vertical and residual fixes (see 3): all cells demonstrably run over the same decoded form because they share one state.
- `cell tag |s, seed| { .. }` becomes `pub fn cell_tag(s: &mut St, seed: u64) -> u64 { .. }`. The body returns the keep-alive the scaffold folds once per iteration. Feature gates ride as `#[feature = "..."]` between the tag and the closure; they flow into the sibling's carrier dependency features exactly as `AxisValue::features` does today.
- `sweep <axis> in [..]` is the outer family loop. Its value is passed to `setup` as the first argument (a `&str` or an integer literal). Multiple `sweep` lines take the cartesian product of family benches (rare; the entropy grid uses it).
- `baseline`/`floor` name cells by tag. `baseline` is the ratio denominator; `floor` is the null-floor subtrahend the reporter differences against (2.4). Both are checked at macro-expansion against the declared cell tags, so a typo is a compile error, not a silent wrong-baseline (which is precisely the bug the panel found: subtract mode was wired but pointed at `switch`, not `nullfloor`).
- `regime` selects the scaffold entry point. `warm` (setup once, calibrated reps over one state), `cold_cycle(M)` (setup returns M states, scaffold cycles them so no single program's dispatch sequence is memorized), `first_touch` (the first-rep column is the headline). Selecting the regime is the consumer's; implementing it is the scaffold's.

### 2.2 Shared setup vs per-cell setup

The shared `setup` is the default S term for every cell. A cell that needs a different construction declares `setup { .. }` before its closure; the scaffold times that per-cell setup instead of the shared one. This covers the current `predecode_family`'s `direct` cell (which needs resolved handlers built in prep) without forcing a separate family:

```rust
    // shared setup builds the predecoded form all simple cells read
    setup |profile: &str, n: usize| -> PredState { /* pd + scratch + sinks */ }

    cell switch |s, seed| { c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r);
                            c::access::checksum_at(&s.r, &s.sinks) }

    // per-cell setup: direct threading needs resolved handlers. The scaffold
    // times THIS setup for this cell, and the cell reads its own state type.
    cell direct #[feature = "threaded"]
        setup |profile: &str, n: usize| -> DirectState {
            let base = PredState::build(profile, n);
            let mut hs = Vec::new();
            c::predecode::resolve_handlers(&base.pd, &mut hs);
            DirectState { base, hs }
        }
        |s, seed| { c::predecode::interpret_predecoded_direct(&s.base.pd, &s.hs, seed, &mut s.base.r);
                    c::access::checksum_at(&s.base.r, &s.base.sinks) }
```

The scaffold always times *a* setup, shared or per-cell, and always emits `setup_ticks`. There is no path on which S is untimed. That is the panel's number-one finding turned into a structural invariant: setup is a required argument, and the scaffold brackets it with counter reads.

### 2.3 Worked family 1: dispatch (the wire interpreters)

```rust
pub struct DispatchState { pub d: c::ir::Decoded, pub r: Vec<u64>, pub sinks: Vec<u32> }

bench_matrix! {
    name: "carrier_dispatch",
    crate_path: vehje_bench_carrier,
    sweep profile in ["real","madd","tight","scatter","wideselect","leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "switch",
    floor:    "nullfloor",
    regime:   warm,

    setup |profile: &str, n: usize| -> DispatchState {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let prog  = c::generate(&gp);
        let sinks = c::optimize::sinks(&prog);
        let bytes = c::ir::encode(&prog, &c::ir::REC24);
        let d     = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
        let r     = vec![0u64; d.node_count];
        DispatchState { d, r, sinks }
    }

    // every cell folds checksum_at over the SAME s.sinks, so the fidelity fold is
    // symmetric across cells by construction (finding 2 cannot recur here).
    cell switch    |s, seed| { c::interpret(&s.d, seed, &mut s.r);              c::access::checksum_at(&s.r, &s.sinks) }
    cell fntable   |s, seed| { c::interpret_fntable(&s.d, seed, &mut s.r);      c::access::checksum_at(&s.r, &s.sinks) }
    cell bittree   |s, seed| { c::interpret_bittree(&s.d, seed, &mut s.r);      c::access::checksum_at(&s.r, &s.sinks) }
    cell threaded  #[feature = "threaded"]
                   |s, seed| { c::interpret_threaded(&s.d, seed, &mut s.r);     c::access::checksum_at(&s.r, &s.sinks) }
    cell nullfloor |s, seed| { c::interpret_nulldispatch(&s.d, seed, &mut s.r); c::access::checksum_at(&s.r, &s.sinks) }
}
```

Contrast with today: the same six profiles times eight shapes, but the `prep`/`body` strings are gone. Every cell is a `pub fn` the compiler checks. `s.sinks` is built once in the shared setup and folded identically by every cell, so the register-vs-something checksum-scope asymmetry the panel found in the residual family is unreachable here: there is one sink set, physically shared.

### 2.4 Worked family 2: residual (the finding that most needed the shared state)

The panel found this family carried a *triple* asymmetry, all loaded onto the register cell: register ran over the wire `Decoded` while stack ran predecoded, register folded a full-array checksum while stack folded live-outs, and the two were meant to isolate one axis. The fix is to build both forms and one sink set in a shared setup, so the only measured difference is the encoding:

```rust
pub struct ResidualState {
    pub pd:    c::predecode::Predecoded,
    pub sp:    c::stackbc::StackProgram,
    pub sinks: Vec<u32>,          // live-out node ids (register side)
    pub r:     Vec<u64>,          // register scratch
    pub st:    Vec<u64>,          // stack operand stack
    pub lo:    Vec<u64>,          // stack locals
}

bench_matrix! {
    name: "carrier_residual",
    crate_path: vehje_bench_carrier,
    sweep profile in ["real","madd","tight","scatter","wideselect","leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "register",
    regime:   warm,

    setup |profile: &str, n: usize| -> ResidualState {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let prog  = c::generate(&gp);
        let sinks = c::optimize::sinks(&prog);           // ONE sink set
        let bytes = c::ir::encode(&prog, &c::ir::REC24);
        let d     = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
        let pd    = c::predecode::predecode(&d);          // register form: predecoded
        let sp    = c::stackbc::compile(&prog);           // stack form: from the SAME prog
        let r     = vec![0u64; pd.nodes.len()];
        let st    = vec![0u64; 64];
        let lo    = vec![0u64; sp.num_locals];
        ResidualState { pd, sp, sinks, r, st, lo }
    }

    // both cells are PREDECODED-form and both fold live-outs of equal cardinality;
    // sharing one setup makes "same program, same decode form, same fold scope"
    // impossible to violate.
    cell register |s, seed| {
        c::predecode::interpret_predecoded(&s.pd, seed, &mut s.r);
        c::access::checksum_at(&s.r, &s.sinks)
    }
    cell stack |s, seed| {
        c::stackbc::interpret_stack(&s.sp, seed, &mut s.st, &mut s.lo);
        c::access::checksum_at(&s.lo, &s.sp.out_locals)
    }
}
```

The register cell now runs `interpret_predecoded` (not the wire `interpret`), matching that the stack cell runs a predecoded `Vec<Bc>`; both fold live-outs. This is the panel's fix, but the shared-setup surface makes it the *only* thing you can write: there is one `prog`, one `sinks`, and the two forms descend from them. You cannot accidentally decode the register side from wire while the stack side is predecoded, because there is no second program to decode differently.

### 2.5 Worked family 3: the S-cost / setup family (the central measurement, and why it is not special)

The S-cost family measures the one-time cost to get from wire bytes to a dispatch-ready form: the S in `total(k) = S + k*I`, hidden in untimed prep everywhere else. The realization that makes it cheap: **the S-cost family is just a matrix whose measured op is a construction.** There is no separate S-mode in the scaffold. The shared setup holds the raw material (the owned program, the wire bytes) and each cell *times a build*:

```rust
pub struct SetupInput { pub prog: c::ir::Program, pub bytes: Vec<u8> }

bench_matrix! {
    name: "carrier_setup",
    crate_path: vehje_bench_carrier,
    sweep profile in ["real","madd","tight","scatter","wideselect","leaf"],
    sizes: [64, 256, 1024, 4096, 16384],
    baseline: "parse",
    regime:   warm,

    // the shared "setup" here is the raw material, cheap to hold, NOT the measured
    // thing. Each cell measures a construction over it. `seed` is unused.
    setup |profile: &str, n: usize| -> SetupInput {
        let mut gp = c::GenParams::profile(profile).unwrap();
        gp.node_count = n;
        let prog  = c::generate(&gp);
        let bytes = c::ir::encode(&prog, &c::ir::REC24);
        SetupInput { prog, bytes }
    }

    // each cell TIMES a build; the keep-alive is a size of the built form so the
    // build cannot be DCE'd. The scaffold's calibrated loop re-runs the build, so
    // S is measured above the counter floor. This is S, measured directly.
    cell parse         |s, _| { c::ir::Decoded::parse(&s.bytes, c::ir::REC24).unwrap().node_count as u64 }
    cell predecode     |s, _| { let d = c::ir::Decoded::parse(&s.bytes, c::ir::REC24).unwrap();
                                c::predecode::predecode(&d).nodes.len() as u64 }
    cell stackcompile  |s, _| { c::stackbc::compile(&s.prog).num_locals as u64 }
    cell optall        |s, _| { c::optimize::optimize(&s.prog, true, true, true, false, true).prog.nodes.len() as u64 }
    cell emitdirect    #[feature = "jit"] |s, _| { c::copypatch::emit(&s.prog).unwrap().len() as u64 }
    cell emitcopypatch #[feature = "jit"] |s, _| { c::stencil::emit_stencil(&s.prog).unwrap().len() as u64 }
}
```

This is the same authoring surface as the execution families; only the cell body differs (a build instead of an interpret). Combined with (a) the `setup_ticks` column the scaffold emits on *every* execution family (a soft S alongside its I) and (b) this family's precise S, the consumer computes the tier breakeven `k* = (S_b - S_a) / (I_a - I_b)` from data the matrix now always carries. The panel's number-one finding ("S hidden in untimed prep, breakeven unknowable") is answered two ways at once: S is always reported for every family, and it is measured precisely by this family, and neither requires a new scaffold mode.

### 2.6 Regime example: cold_cycle

```rust
bench_matrix! {
    name: "carrier_coldcycle",
    crate_path: vehje_bench_carrier,
    sweep profile in ["real","madd","tight","scatter","wideselect","leaf"],
    sizes: [64, 256, 1024],
    baseline: "switch",
    regime:   cold_cycle(16),          // scaffold cycles 16 distinct programs

    // setup returns M distinct same-profile programs; the scaffold provides the
    // iteration index k, and the cell picks state.pds[k % M] so no single
    // program's dispatch sequence fits the predictor.
    setup |profile: &str, n: usize| -> ColdState {
        let pds: Vec<_> = (0..16u64).map(|i| {
            let mut gp = c::GenParams::profile(profile).unwrap();
            gp.node_count = n;
            gp.seed ^= i.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let bytes = c::ir::encode(&c::generate(&gp), &c::ir::REC24);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            c::predecode::predecode(&d)
        }).collect();
        let r = vec![0u64; pds.iter().map(|q| q.nodes.len()).max().unwrap()];
        ColdState { pds, r }
    }

    // cold cells take a third parameter k (the iteration index); warm cells omit it.
    cell switch  |s, k, seed| { let pd = &s.pds[k % 16]; c::predecode::interpret_predecoded(pd, seed, &mut s.r); c::checksum(&s.r) }
    cell fntable |s, k, seed| { let pd = &s.pds[k % 16]; c::predecode::interpret_predecoded_fntable(pd, seed, &mut s.r); c::checksum(&s.r) }
    cell null    |s, k, seed| { let pd = &s.pds[k % 16]; c::predecode::interpret_predecoded_nulldispatch(pd, seed, &mut s.r); c::checksum(&s.r) }
}
```

The scaffold owns the "distinct-state-per-iteration so the predictor cannot memorize" guarantee (it requires M states and provides k); the cell owns picking `pds[k % M]`. The macro generates the three-parameter closure signature when `regime: cold_cycle(_)` is set, so the arity is not a footgun.

## 3. Own vs expose: the four distortions and the three disciplines

The question is which disciplines the scaffold makes *impossible to get wrong* (OWN) versus which it *lets the consumer control while providing the corrective* (EXPOSE). The dividing line: a discipline the scaffold can enforce without knowing carrier semantics is owned; a discipline that depends on what the cell computes is exposed with a structural helper and a reporter-side safety net.

| Discipline | Own / Expose | Mechanism |
|---|---|---|
| Anti-hoist chain | OWN | `scaffold` seeds `acc` from `output[0]`, folds per iteration, writes back inside the timed loop. Consumers never see it. |
| Shared seed table | OWN | `scaffold` holds `const SEEDS: [u64;16]`; uses `SEEDS[k] ^ (k as u64)`. Kills the `input[k % N]` per-size-seed bug by construction. |
| S-vs-I split | OWN (that S is timed) + EXPOSE (what setup does) | `setup` is a required argument; `scaffold` brackets it with counter reads and always emits `setup_ticks`. S can never be untimed. What the setup builds is the consumer's. |
| Fold one keep-alive per iteration | OWN | `scaffold` folds the cell's single returned `u64` once per outer iteration, never per node. Kills "O(N) checksum inside the inner loop." |
| Which value the cell folds (scope symmetry) | EXPOSE + structural helper | The cell returns its own keep-alive. Shared setup makes the fold scope (`s.sinks`) physically shared across cells; `checksum_at` is provided. Null-floor differencing neutralizes residual symmetric cost. |
| Fidelity anchor | OWN (the digest) + EXPOSE (the real anchor) | `scaffold` computes a reps-invariant digest and returns it in `FfiBenchCall.digest`; the harness cross-validates on that, not on the reps-variant `output`. The docs generator surfaces, once, that the authoritative anchor is the consumer's byte-exact cross-validation `#[test]`s. |
| Null-floor differencing | OWN (the wiring) + EXPOSE (the floor cell) | The matrix declares `floor:` a cell tag; `generate_all` points the `normalise` block at it; the reporter differences against it. The consumer supplies the carrier-specific nullfloor cell. |
| SIMD per-input equalization | EXPOSE + structural guard | Lane geometry is deeply cell-specific, so the cell owns processing a batch to the common input count. The shared setup returning one state type makes the vertical baseline asymmetry (wire vs predecoded) impossible; the matrix may declare `inputs_per_call: 8` for the reporter to assert equal counts. |

### 3.1 The anti-hoist / digest split (finding 6, the subtle one)

Finding 6: the FFI `output` is `acc` after R calibration reps, and R is timing-dependent, so `output` is reps-variant and the `MAY_DIFFER = false` cross-validation is inert. But the anti-hoist *needs* `output` to be reps-variant: the whole trick is that rep N+1 reads what rep N wrote, forming a dependency the optimizer cannot collapse. The two requirements conflict on the same 8 bytes.

Resolution: split them onto two channels. `output` stays the anti-hoist channel (reps-variant, written each rep, for liveness). Add a `digest: u64` field to `FfiBenchCall`; the scaffold computes it on a single pass over the fixed seeds from a fixed init (not from `output[0]`), so it is reps-invariant and cell-semantics-dependent, and returns it. The harness cross-validates on `digest`. Two masters, two channels, no compromise:

```rust
pub const SEEDS: [u64; 16] = [ /* the fixed 16-entry table, shared across sizes and cells */ ];

pub fn warm<const N: usize, St, Setup, Cell>(
    input: &[u8; N], output: &mut [u8; 8], setup: Setup, mut cell: Cell,
) -> FfiBenchCall
where Setup: FnOnce(usize) -> St, Cell: FnMut(&mut St, u64) -> u64 {
    let _ = input;

    // S term: time setup once, always reported.
    let s0 = mockspace_bench_core::counter::read_counter();
    let mut state = setup(N);
    let s1 = mockspace_bench_core::counter::read_counter();

    // reps-INVARIANT fidelity digest: one pass, fixed init, fixed seeds. Not in the
    // timed loop, so its cost is not charged; reps-invariant, so cross-validation
    // on it is meaningful under calibration (finding 6).
    let mut digest: u64 = 0xF1DE_1178_ABCD_0001;
    { let mut k = 0usize; while k < 16 { digest = digest.rotate_left(7) ^ cell(&mut state, SEEDS[k]); k += 1; } }

    // first-touch column: one explicitly timed cold pass, always emitted.
    let f0 = mockspace_bench_core::counter::read_counter();
    { let mut acc = 0u64; let mut k = 0usize; while k < 16 { acc ^= cell(&mut state, SEEDS[k]); k += 1; } core::hint::black_box(acc); }
    let f1 = mockspace_bench_core::counter::read_counter();

    // I term: anti-hoist acc seeded from output[0], written back each rep. The
    // scaffold folds ONE keep-alive per iteration (cell's return), never per node.
    let call = mockspace_bench_core::timed_calibrated! { run {
        let mut acc: u64 = output[0] as u64;
        let mut k = 0usize;
        while k < 16 { acc ^= cell(&mut state, SEEDS[k] ^ (k as u64)); k += 1; }
        output.copy_from_slice(&acc.to_le_bytes());
    }};

    FfiBenchCall { run_ticks: call.run_ticks, setup_ticks: s1 - s0, first_ticks: f1 - f0, digest }
}
```

The cell must be idempotent under repeated calls (overwrite scratch, not accumulate), which the calibration contract already requires (`bench-core/lib.rs:514`, "must be safe to execute several times"). The digest pre-pass, the first-touch pass, and the calibrated loop all rely on that same property, so the scaffold adds no new constraint on cell authors.

## 4. The module boundary

The split follows the proposal's instinct (harness is the transport, matrix is the opinionated discipline) and keeps them apart so a consumer can use the raw harness without the opinions.

**`mockspace-bench-core` (exists; small additions).** Keep `timed_calibrated!`, `calibrate_reps`, `counter`, the `Routine`/`RoutineBridge`/`ByteRoutine` surface. Change `FfiBenchCall` from `{ run_ticks }` to `{ run_ticks, setup_ticks, first_ticks, digest }` and update `abi_hash()` to cover the new layout (the ABI hash exists precisely so this drift is caught on load, `lib.rs:420-430`). No new macro is needed: the scaffold times setup with two `read_counter()` calls, so the two-timer logic lives in the matrix crate, not here. This is the only core change, and it is additive plus one struct.

**`mockspace-bench-harness` (exists; reporter additions).** Keep `matrix::{MatrixSpec, AxisSpec, AxisValue, Composition, expand, render, render_variant, render_bench_section, generate}` unchanged: the cartesian product, the crate scaffold, the `bench.toml` section wiring. It stays the dumb transport and knows nothing about honest measurement. Additions, all in the analysis/reporting layer: read the new `FfiBenchCall` fields and emit `setup_ns` and `algo_ns_first` columns; make the `normalise` block floor-aware so the reporter can difference against a named floor cell (null-floor differencing); read `digest` for cross-validation instead of the reps-variant `output`. The `render`/`expand`/`generate` core does not change; the matrix crate feeds it a `MatrixSpec` exactly as `gen_matrix.rs` does today.

**`mockspace-bench-matrix` (new; the opinionated layer).** Depends on `-harness`, `-core`, `-macro`. Contents:
- `scaffold::{warm, cold_cycle, first_touch}`: the canonical measurement wrappers. Own the seed table, the anti-hoist chain, the reps-invariant digest, the S timing, the fold-one-keep-alive rule, the regime cycling. This is where the template's measurement logic (currently duplicated in `gen_matrix.rs`'s `TEMPLATE` string) becomes one compiled, tested function instead of a string every consumer re-derives.
- The `bench_matrix!` `macro_rules!` macro: expands to typed `setup`/`cell` functions plus `matrix_decls()`.
- `MatrixDecl` / `CellDecl` / `Regime`: the data the macro emits and the generator consumes.
- The single canonical `lib_template` string (the sibling shape from 1.3): consumers never author it.
- `generate_all(&[MatrixDecl], out_dir)`: maps each `MatrixDecl` to a `harness::MatrixSpec` (filling the canonical template with cell paths, sweep values, sizes, features), calls `harness::generate`, and appends the `bench.toml` sections with the floor wired in.

**Consumer (`vehje-bench-carrier` lib + `mock/benches/`).** Owns Layer 2, unchanged in spirit: the program generator (`gen.rs`), the op semantics (`ops.rs`), the JIT/predecode/stackbc/vertical code. What changes: the old string `prep` fragments become the typed `setup` bodies, the string `body` fragments become the typed `cell` bodies, both inside `pub mod bench { bench_matrix!{ .. } }` in the carrier *library* crate (so they are `pub` and path-referenceable). The `gen_matrix.rs` bin shrinks to the four-line generator in 1.2. The byte-exact cross-validation `#[test]`s stay in the carrier crate; they are the authoritative fidelity anchor the scaffold's digest merely smoke-checks.

The dogfood: after extraction, `gen_matrix.rs` is the four-line generator, `carrier::bench` is the `bench_matrix!` invocations plus the typed cells, and every discipline the panel corrected is inherited from `scaffold` rather than restated in a template string. The carrier matrix is `mockspace-bench-matrix`'s first consumer and its regression test.

## 5. What this design deliberately does not do

- It does not build the full `total(k) = S + k*I` geometric k-ladder with least-squares fit. The panel ranked that high-effort and deferred it (its in-process rungs are not cold; it needs process-per-rung). This design delivers the pieces the panel recommended instead (the S-cost family, `setup_ticks` on every family, `first_ticks`, and `cold_cycle`), which together give S, I, warm, and aliased-cold without the k-ladder machinery. The k-ladder can be added later as a fourth regime (`regime: kladder([1,2,4,..,128])`) that emits a per-rung column; the scaffold shape already accommodates it (it is a loop count sweep around the same cell), and nothing here forecloses it.
- It does not capture closure tokens in a proc-macro. That path is named in 1.2 and rejected: it either does untracked filesystem IO from a macro or throws away type-checking by re-emitting bodies as text. Cells are referenced by path; bodies never leave the consumer crate.
- It does not change `harness::{expand, render, generate}`. The transport is correct and the extraction sits on top of it, feeding it `MatrixSpec`s exactly as `gen_matrix.rs` does today.

## 6. The one line to carry forward

The stringly-typed cell existed only because nobody separated "what code runs" from "where it physically lives." Separate them: the cell body is a typed `pub fn` in the consumer crate (compiled once, inlined into each cdylib by the same fat LTO the string form already trusted), and the sibling crate names it by path. Everything else, the ergonomic macro, the disappearance of the proc-macro risk, the four distortions becoming structural invariants, follows from that separation.
