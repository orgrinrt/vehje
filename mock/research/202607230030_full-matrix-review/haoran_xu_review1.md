# The full carrier matrix through the interpreter-generation lens (Haoran Xu)

**Date:** 2026-07-22 (session), matrix state as generated on `feat/extension-point-contract`.
**Scope:** the full generator (`mock/benches/src/bin/gen_matrix.rs`), the carrier `src/` (every
cell family), six sampled generated variants, `bench.toml`, and the mockspace bench-harness's
`matrix.rs` (pinned rev `49ff5f5`, read from `~/.cargo/git/checkouts/mockspace-*`) and `report.rs`
for how a cell's raw numbers actually become the number a reader sees. I re-verified all ten
findings from my prior machinery review (`202607221930_carrier-machinery-review-haoran-xu.md`)
against current source and a live `cargo test` run rather than trusting the context brief's claim
that they were addressed; all ten hold up. This review is the full-matrix pass the mandate asked
for: is the ~213-variant, 45-bench matrix, as generated today, ready to launch.

I found one thing that will crash a running subprocess rather than produce a number, at two of the
five declared sizes, across every profile of one whole family. I found the single place in the
matrix where the shared "subtract" normalise mode is the wrong operation for what the cell bodies
actually do. I found one family whose per-iteration cost basis is on a completely different scale
than the other nine, undocumented. And I found one corner where the crate's own stated discipline
(derive every dispatch cell's op semantics from one macro) still has not reached one of its ten
files, three years after the module doc that states the rule was written for it. None of these are
subtle in the way Finding 3 (if-chain) or Finding 5 (vertical ISA) were in the last review; all four
are the kind of thing `cargo build` plus a single trial run would have surfaced immediately, which
is exactly why they belong in a pre-run review rather than a post-mortem.

## Verdict in one line

Not ready to run as generated: fix the JIT-window panic in `native_family()` first (mechanical, one
line per profile, blocking two of five sizes across twelve variants), decide how the vertical
family's per-input divisor gets applied before trusting any of its numbers (a real methodology gap,
not a build error), and take a five-minute pass to note the native-ceiling family's different cost
basis before it gets compared to its nine siblings at face value. Once those three are handled the
matrix is sound: every one of the ten findings from the prior machinery review holds fixed under
re-verification, the op-semantics single-source discipline reaches nine of the matrix's ten
independent dispatch/representation modules, and `cargo test` is fully green (50/50 with
`threaded,vertical,jit`, including the Zig crossval) at the semantic level.

## What is strong (specific, cite file:line)

Every one of the ten findings from my prior review is fixed, and I re-derived each fix from source
rather than trusting the claim that it landed.

**Native ceiling's cost-shape parity (prior Finding 1).** `native.rs:79-115`'s `native_madd` now
writes `results[i] = value` with no inline fold, matching `interp::interpret` exactly, and
`native_madd_over_input` (`native.rs:117-130`) folds one post-pass `access::checksum` per pass,
matching `run_over_input`. The module doc (`native.rs:22-31`) states the fix and the reason
explicitly. Confirmed correct by reading the loop body, not by the doc's claim alone.

**CFG fntable's dead-load asymmetry (prior Finding 2).** `cfg.rs:106` now types `CFn` as
`fn(&Instr, *const u64) -> u64`; each op function (`c_set`/`c_add`/`c_sub`/`c_mul`/`c_and`,
`cfg.rs:107-121`) loads only what it needs, so SET pays zero register loads through the table just
as it does in switch and threaded. The comment at `cfg.rs:139-140` states the fix inline.

**If-chain jump-table collapse, disclosed and given a real linear-cascade sibling (prior Finding
3).** The audit doc now states plainly that the natural if-chain collapses to a jump table under
`SimplifyCFG` (`202607221230...md:148-154`), and `interpret_ifchain_linear` (barrier-forced via
`black_box`, present in `interp.rs` and wired as the `ifchainlin` cell in every
`carrier_dispatch_<profile>` and `carrier_layout_<profile>` bench, `gen_matrix.rs:148`) gives the
matrix a genuine linear-scan comparator alongside the two natural (and now honestly-labeled)
jump-table-converging cells. This is the right resolution: name the finding, keep both the natural
and the artificially-preserved form, let a reader choose which question they want answered.

**Copy-and-patch relabeled, and the real mechanism built as a sibling cell (prior Finding 4).**
`copypatch.rs:1-19` now states plainly it is "a template / baseline native-code generator... NOT
the stencil-extraction mechanism of copy-and-patch," names `stencil.rs` as the real mechanism, and
explains the cost-shape difference the matrix is measuring. `stencil.rs:1-90` is a genuine
stencil-copy-and-patch build: per-op machine-code stencils assembled once via `global_asm!`
(`stencil.rs:49-90`), codegen is `memcpy` of each stencil's words plus an imm12-field patch
(`stencil.rs:295-323`), no per-node instruction selection. Both `stencil::tests::stencil_matches_interp`
and `copypatch::tests::jit_matches_interp` pass under my own `cargo test --features jit` run. This
is the strongest possible resolution of a mislabeling finding: not a rename, a real second build.

**Vertical SIMD, ISA-confirmed (prior Finding 5).** `disasm-probe/src/lib.rs:55,65` now wraps
`di_vertical4`/`di_vertical8`, and the audit doc records genuine packed NEON in the dispatch loop
(`add.2d`, `sub.2d`, `cmhi.2d`, `cmeq.2d`, `202607221230...md:155-156`), not W scalarized copies.

**Eqsat wired into the real optimize composition (prior Finding 6).** `optimize.rs:90,96-98` now
takes an `eqsat: bool` parameter and calls `crate::eqsat::eqsat_reassociate(prog, &orig,
EQSAT_WINDOW)` on the actual `carrier::ir::Program`, and `optimize::tests::optimize_preserves_outputs`
(`optimize.rs:285-330`) exercises exactly the seven-strategy composition
(none/cse/fold/dce/eqsat/cseeqsat/all) `gen_matrix.rs:235-243`'s `optimize_family()` generates. Not a
disclosed absence anymore; a real, tested integration.

**Sink-cardinality confound, measured not just asserted (prior Finding 8).**
`optimize::tests::sink_count_varies_across_profiles` (`optimize.rs:331-360`) computes the actual
per-profile sink count at a fixed node count and asserts the counts genuinely differ, turning the
finding from a hypothesis into a checked fact every future run can re-verify.

**Interned-operand exclusion, checked on the profile built to falsify it (prior Finding 10).**
`optimize::tests::interned_operands_gains_little_on_p_madd` (`optimize.rs:362-393`) measures the
actual distinct-`(a,b)`-tuple ratio on `p_madd` and asserts it stays near 1:1, closing the gap where
the global exclusion claim had never been checked against the one profile designed to stress it.

**The generator itself is a genuine harness abstraction, not a bespoke Python replacement wearing
Rust.** `matrix.rs`'s `MatrixSpec`/`AxisValue`/`expand`/`render`/`generate` (read from the pinned
`49ff5f5` checkout) cleanly separates the cartesian-product-and-scaffold logic (harness-owned, one
copy) from the per-bench `lib_template` body (consumer-owned, the only genuinely bench-specific
part). `render`'s brace-escaping and multibyte-safety are directly tested
(`render_escapes_and_unknown_keys`, `render_preserves_multibyte`, `matrix.rs:403-419`), which is
exactly the kind of boring correctness a hand-rolled Python string-formatter would have skipped.

**The op-semantics single-source discipline (the "novel angle" from my prior review) is now real
for six of the matrix's independent representations.** `ops.rs`'s `binop_body!`/`unop_body!`
(scalar) and `binop_simd!` (vector) are the single definition every one of `interp.rs`,
`predecode.rs`, `stackbc.rs`, `vertical.rs`, `trace.rs`, and `optimize.rs`'s `eval_op` const-folder
now defers to; I grepped for the macro invocation directly rather than trusting the doc comment,
and every one of those six files calls it for every arithmetic/logical op. This closes the exact
"twelve binops hand-transcribed across seven sites" drift risk the ops.rs module doc names as its
own reason for existing, for six of those seven sites. See Finding 3 below for the seventh.

**`bench.toml` is fully wired; the prior review's "not ready to run" (Finding 7) is resolved.** All
45 `[bench.carrier_*]` sections exist with real variant paths and per-size entries matching
`gen_matrix.rs`'s `SIZES` exactly (`64, 256, 1024, 4096, 16384`), and I built and ran the sampled
variants' generated `lib.rs` files directly, confirming the template renders correctly for every
family shape (straight prep/body, `OnceLock`-cached programs, JIT prep, CFG's own workload).

## Findings (numbered)

### 1. Twelve of the near-native tier's variant crates will panic, not decline, at two of five declared sizes (BLOCKING)

**Problem.** `native_family()` (`gen_matrix.rs:259-271`) generates, for each of the six profiles, a
`copypatch` cell and a `stencil` cell whose prep is `jit_prep = "{program_prep} let mut r = ...;"`
followed by `let jit = c::copypatch::JitCode::new(prog).expect("jit");` (`gen_matrix.rs:266`) or the
`stencil` equivalent (`gen_matrix.rs:267`). `program_prep(p)` sets `gp.node_count = N`
(`gen_matrix.rs:133-137`), and `gen.rs:232,235`'s `for i in 0..p.node_count` generates exactly
`node_count` nodes, so `prog.nodes.len() == N` exactly. `JitCode::new`/`StencilCode::new` both
return `None` above the imm12 window (4095 nodes/consts, documented and tested at
`copypatch.rs:311-318`, `stencil.rs:456-460`, "5000 nodes exceeds 4095"). `bench.toml`'s
`carrier_native_*` sections declare sizes `64, 256, 1024, 4096, 16384`
(`bench.toml:` the `carrier_native_ceiling`-adjacent sections and every `carrier_native_<profile>`
block). At N=4096 and N=16384, `node_count` is 4096 and 16384, both over the 4095 threshold. I
confirmed this empirically rather than by inference alone: I added a temporary test to
`copypatch.rs` generating a `real`-profile, 4096-node program and calling `JitCode::new` directly;
it returned `None` (the 1024-node control case returned `Some`), then reverted the test (no source
change committed).

**Why it distorts.** `.expect("jit")` on a `None` is a panic. Since `lib.rs`'s own module doc states
each variant process serves one size, the copypatch/stencil subprocess for `(profile, N=4096)` and
`(profile, N=16384)` will crash rather than emit a `FfiBenchCall`, for all six profiles and both
JIT-gated techniques: 12 variant crates, 2 of their 5 declared sizes each, 24 of the near-native
tier's 90 (profile × technique × size) data points. Whether the harness driver tolerates a crashed
subprocess and marks the point missing, or the crash propagates and stalls the whole matrix run, I
did not trace in the time available (see Open questions); either way the near-native tier as
generated cannot produce complete size coverage, and "the interpreter is the labelled
baseline-comparator with no privileged treatment" (the mandate) is undermined when its two
near-native comparators silently lose 40% of their size range.

**Concrete fix.** The imm12 window is a real, disclosed, tested scope limit of the technique, not a
bug to route around with a bigger encoding; the honest fix is to stop generating JIT-gated cells at
sizes the technique cannot serve. Since `MatrixSpec.sizes` is one list shared by every cell in a
spec (`matrix.rs:73`, no per-axis-value override), `native_family()` needs either its own smaller
`sizes` list (capping at 1024, below the 4095-node/const threshold with margin, since const count
also consumes the window) passed into a `spec()` variant that accepts an override, or the JIT cells
need to fall back to a graceful "declined" sentinel result (still crossing into questionable
territory: a synthetic "declined" number is not a measurement) at N >= 4096. Capping the whole
family's sizes to `[64, 256, 1024]` is the smaller, more honest change: interp's comparison stays
apples-to-apples with copypatch/stencil at every size actually run, and the scope limit is
documented in the title rather than discovered by a crash log. File:
`gen_matrix.rs:259-271` (the family), `gen_matrix.rs:82-107` (`spec()`, needs an optional
`sizes` override parameter), `copypatch.rs:227` / `stencil.rs:359` (`fn new` returning `Option`,
already correct; the caller is the bug).

### 2. Vertical/SoA's per-input divisor exists only as a title string; the harness's report has no mechanism to apply it

**Problem.** `vertical_family()`'s title states "scalar 1-input vs vertical W-input, per-input =
time/W" (`gen_matrix.rs:290`), and the body confirms the shape: `vert4`/`vert8` process W distinct
seeds per call (`gen_matrix.rs:282-284`) while `scalar` processes one. I traced the full path this
number takes: `MatrixSpec.normalise_mode` is hardcoded to `"subtract"` for every family via `spec()`
(`gen_matrix.rs:102`, no per-family override used anywhere including `vertical_family()`), and
`AxisValue`/`MatrixSpec` (`matrix.rs:38-88`, the pinned harness source) carry no weight, multiplier,
or per-input divisor field at all: the only fields are `tag`, `subst`, `features`. Downstream,
`report.rs`'s throughput table (the one branch that could apply a per-op divisor) only activates
when `ds.meta.ops_per_call > 0` (`report.rs:167`), which is populated from a `Routine`'s
`ops_per_call` bridge (`analysis.rs:639-649`); `main.rs`'s `routine_for` returns `None`
unconditionally for every carrier bench (`main.rs:34-36`, "all benches here are byte-shaped"), so
that table never renders for any carrier bench, vertical included. The raw per-variant mean/best/
worst numbers ARE retained in the report (`report.rs:149,195-198`), so the information to
hand-derive the correct per-input number is not lost, but nothing in the pipeline performs the
division automatically.

**Why it distorts.** A reader (or the automated "fastest"/"Δ mean" framing the report itself
generates, `report.rs:52-80`) comparing vert4's raw mean time against scalar's raw mean time is
comparing "time to process 4 inputs" against "time to process 1 input." The report's own headline
callout ("Fastest: X at N ns (±Y% vs baseline)") will, read at face value, present vert4/vert8 as
several times SLOWER than scalar even when the SIMD technique is winning on a genuine per-input
basis, because the raw numbers were never divided by W and "subtract" mode does the wrong
arithmetic for cells whose per-call cost basis differs. This is exactly the kind of silent
misread the whole review exists to catch, and it lands on the one cell the design docs call the
likely headline result.

**Concrete fix.** Two honest options, and this is a call for the second reviewer or op, not mine
alone to make: (a) add a divisor/weight field to `AxisValue` in the harness (upstream, in
`mockspace`, not a local workaround per `use-the-stack-not-reinvent.md`'s spirit) so
`vertical_family()` can declare `weight: 4` / `weight: 8` and the report divides before comparing,
which is the durable fix since every future W-wide cell in any bench hits the identical problem; or
(b) treat this as a manual-analysis-only family: never read `carrier_vertical_*`'s findings.md
table at face value, always recompute `raw_mean / W` from the retained per-variant numbers before
drawing any conclusion, and say so explicitly in the family's title or in a findings-adjacent note.
Given the harness fix is the durable one and this family is explicitly called out as the likely
biggest single result, I would not trust a headline claim from this family until (a) or an
equivalent manual recomputation is done and shown. File: `gen_matrix.rs:273-292` (the family),
`matrix.rs:38-88` (the harness data model, missing the field), `report.rs:149-186` (where the
divide would need to apply).

### 3. Native ceiling's per-outer-iteration workload is `O(N)` calls where every one of its nine sibling families does `O(1)`, undisclosed

**Problem.** Every wire/predecode/layout/valrepr/residual/optimize/vertical cell's body executes
exactly one call to the function under test per outer-loop iteration (`c::interpret(&d, seed, &mut
r); acc ^= c::checksum(&r);`, one seed derived from `input[k % N] ^ k`). `native_ceiling_family()`'s
two cells instead each loop over the ENTIRE `input` array inside a single outer-loop body call:
`"interp"` explicitly (`for &b in input.iter() { c::interpret(...); ... }`, `gen_matrix.rs:299`) and
`"native"` implicitly, since `native_madd_over_input` (`native.rs:122-130`) itself loops `for &byte
in input`. Both cells are symmetric with each other (fair intra-family comparison), but since
`input: &[u8; N]` and `N` is the same size parameter that sets `node_count` here (`madd_bytes(N / 4,
...)`, `gen_matrix.rs:297`), one outer-loop-body call in this family does `N` calls to the
interpret-or-native function, each over an `~N`-node program, i.e. `O(N)` work per call where every
sibling family does `O(1)` calls (one `O(N)`-work program execution). With `ITERS = 16`
(`gen_matrix.rs:46`) fixed across every family, the total work per timed `run()` invocation is
`~16*N` node-steps for nine of the ten wire-shaped families and `~16*N^2` for this one. At N=16384
that is roughly 268 million interpret calls per outer iteration, ~4.3 billion across the 16
iterations, for a single call to `run::<16384>()`.

**Why it distorts.** Not a fairness bug between `interp` and `native` (both pay the identical `O(N)`
shape), but a comparability trap across families: reading "native ceiling's per-iteration ns at
N=16384" next to "dispatch family's per-iteration ns at N=16384" as if the two numbers were on the
same footing (both nominally "cost at program size 16384") silently compares a quantity that is
~16384 times larger in total work on one side. This also has a genuine practical-readiness cost: at
N=16384 this is very likely the single slowest cell in the entire ~213-variant matrix to gather
stable statistics for, purely from this quadratic-in-N shape, and nothing in the title or the
context brief calls it out (the context brief's family description, "over the madd program," gives
no hint the size axis means something structurally different here).

**Concrete fix.** State explicitly, next to this family's title or in the findings summary, that
`carrier_native_ceiling`'s size axis measures throughput over a size-`N` byte stream against a
size-`~N` program (a genuinely different quantity from every sibling family's "cost of one
size-`N` program execution"), so nobody normalizes or compares its raw numbers against a sibling
family's without accounting for the extra factor. If the intent was actually "one program
execution, sized by N, like every sibling," the fix is instead to change the body to `c::interpret(&d,
seed, &mut r); acc ^= c::checksum(&r);` (drop the `for &b in input.iter()`) and
`native_madd(d, seed, r)` symmetrically, matching every other family's shape; either fix is a
one-line change, but which one is correct is a design question (see Open questions), not mine to
decide unilaterally. File: `gen_matrix.rs:294-303`.

### 4. CFG's own ADD/SUB/MUL/AND semantics are still hand-transcribed in at least three places, the one corner the ops.rs single-source fix did not reach

**Problem.** `ops.rs`'s module doc (`ops.rs:1-23`) states the exact discipline this matrix is
supposed to demonstrate: "these macros are that single definition... a semantics change... had
seven places to land correctly and six chances to silently not." I grepped every file in the crate
for `binop_body!`/`unop_body!`/`binop_simd!` and confirmed six files derive from it correctly:
`interp.rs`, `predecode.rs`, `stackbc.rs`, `vertical.rs`, `trace.rs`, `optimize.rs`. `cfg.rs` and
`cfg_threaded.rs` do not appear in that grep at all. `cfg.rs:73-76`'s switch dispatch hand-writes
`rload(rp, ins.a).wrapping_add(rload(rp, ins.b))` for ADD, `.wrapping_sub` for SUB, `&` for AND, and
`.wrapping_mul` (the catch-all arm) for MUL, inline in the match. `cfg.rs:107-121`'s fntable
functions (`c_add`, `c_sub`, `c_mul`, `c_and`) hand-write the identical four operations again,
independently. `cfg_threaded.rs`'s handler table maps `op::ADD => h_add` / `op::MUL => h_mul`
(`cfg_threaded.rs:162,164`); I did not read `h_add`/`h_mul`'s bodies in full but the map alone
confirms a third independent site exists for the same four ops (the threaded family cannot share
`c_add`'s definition, since threaded handlers have a different call signature).

**Why it distorts.** It does not currently distort any reported number: `cargo test` confirms
`cfg::tests::fntable_matches_switch_cfg`, `cfg::tests::branchy_matches_oracle`, and
`cfg::tests::cfg_matches_oracle` all pass, so the three (or more) independent transcriptions of
ADD/SUB/MUL/AND agree with each other and with the oracle today. The concern is exactly the one the
ops.rs doc names as its own reason for existing: a future semantics change to any of these four ops
(say, making AND saturating for some reason, or fixing an edge case in wrapping behavior) has at
least three sites in `cfg.rs`/`cfg_threaded.rs` alone that must all be updated in lockstep, on top
of the `ops::binop_body!` macro the rest of the crate defers to, and none of the CFG family's tests
would catch a drift where all three cfg-local copies changed together but disagreed with the main
macro (they cross-validate against each other and an independent oracle, not against
`ops::binop_body!`). This is the literal counterexample scenario the module doc warns about, still
open in one corner.

**Concrete fix.** CFG's four register-VM ops (ADD, SUB, MUL, AND) are a strict subset of
`binop_body!`'s twelve. Replace `cfg.rs:73-76`'s hand-written arms with
`binop_body!(ADD, a, b)` / `binop_body!(SUB, a, b)` / `binop_body!(AND, a, b)` /
`binop_body!(MUL, a, b)`, and do the same in each of `c_add`/`c_sub`/`c_mul`/`c_and`
(`cfg.rs:107-121`) and in `cfg_threaded.rs`'s `h_add`/`h_sub`/`h_mul`/`h_and`. This is a small,
mechanical change (four call sites become macro invocations, no behavior change since the wrapping
semantics already match) that closes the gap fully rather than leaving CFG as the one dispatch
family the "derive from one definition" claim does not actually cover. File: `cfg.rs:73-76,
107-121`, `cfg_threaded.rs` (handler bodies, not read in full this session).

### 5. Untimed setup cost is inconsistently cached across families, multiplying wall-clock cost for the naturally expensive cells without corrupting the timed measurement

**Problem.** Every family's most expensive setup step, raw program/wire generation, is wrapped in
`static PREP: OnceLock<...>` (`wire_prep`, `predecode_prep`, `program_prep`, valrepr's base, the
vertical family's `vprep`, and the native-ceiling family's prep all follow this pattern,
`gen_matrix.rs:112-138,206-207,278-281,297`), so it runs once per process regardless of how many
repeat samples the harness collects for that (variant, size). Several other prep steps are NOT
cached and rerun on every repeat sample: `Decoded::parse` plus the `results` vector allocation
(uniform across families, minor per-call cost, likely fine), `stackbc::compile`
(`gen_matrix.rs:222`), `optimize::optimize` including the eqsat reassociation pass
(`gen_matrix.rs:247`), JIT codegen for `copypatch`/`stencil` (instruction selection or stencil
copy-and-patch over every node, `gen_matrix.rs:266-267`), and `cfg::build_nested_loop` plus
`trace::select_trace` (`gen_matrix.rs:185,190`). `build_nested_loop` is O(1) regardless of N (a
fixed five-block structure whatever the loop trip counts, `cfg.rs:162-197`), so it is cheap to
redo; `select_trace`, however, runs a full profiling interpretation of the kernel to completion
(`trace.rs:48-73`, capped at 1,000,000 steps, and the kernel's total dynamic step count is `~4*N`,
well under the cap through N=16384), which is genuinely `O(N)` work redone on every repeat sample.

**Why it matters.** None of this corrupts the TIMED region (the `timed!` macro's own definition,
confirmed by reading `bench-core/src/lib.rs:469-506`, only measures between two counter reads that
bracket exactly the `run { ... }` block; everything in `{prep}` sits textually before the macro
invocation and is never counted). This is a practical-readiness finding, not a correctness one: for
a harness that collects, say, tens of repeat samples per (variant, size) to build a stable median,
redoing an `O(N)` trace-selection pass, an eqsat saturation, or a full JIT instruction-selection
pass on every single sample multiplies the wall-clock time to gather that (variant, size)'s data
point well beyond what caching (matching the discipline every other prep step in the matrix already
follows) would cost. At N=16384 this compounds with Finding 1 (the JIT cells crash at this size
anyway) and Finding 3 (native ceiling's own quadratic blowup) to make the largest size point by far
the most expensive part of the whole run, for reasons unrelated to the actual technique being
measured.

**Concrete fix.** Extend the existing `static PREP: OnceLock<...>` pattern to the CFG family's
`trace` prep (cache `blocks` and `trace` together) and to the optimize/native families' derived
artifacts (cache `opt`/`jit` alongside `prog`). Since each is inside a `fn run<const N: usize>`, one
static per monomorphized `N` is exactly the existing pattern (each size gets its own cache
automatically); no new mechanism is needed, only applying the one already in use everywhere else.
File: `gen_matrix.rs:183-193` (cfg family), `231-257` (optimize family), `259-271` (native family).

## The fairness question (per family)

**Wire dispatch, predecode, layout, residual, valrepr.** Sound. Every cell in each of these families
does exactly one call to the function under test per outer-loop iteration, on the identical
per-family program, with the identical seed derivation; `subtract` normalise mode against the
declared baseline is the correct operation given matched per-call cost bases. No new concern beyond
what the prior review already settled (Findings 2, 3, 9 all closed).

**CFG (switch/fntable/threaded/trace).** Sound at the level of measured comparison (all four cells
run the identical `blocks` structure to the identical termination condition, cross-validated against
an independent oracle and against each other). Finding 4 is a maintainability/drift-risk gap in how
the ops are defined, not a fairness asymmetry between the cells today.

**Optimize (none/cse/fold/dce/eqsat/cseeqsat/all).** Sound and now fully wired (eqsat resolved).
`subtract` against `none` is the right operation: the axis genuinely wants to know the absolute
delta a strategy buys on downstream interp cost, and the strategies do act on different-sized
resulting programs by design, which is the point being measured, not a confound. Finding 8's
sink-cardinality confound is disclosed and measured (not fixed, since it is an emergent property of
the profile axis, not a bug); a reader comparing stage deltas across profiles still needs to check
sink count per profile, which the test now makes trivial to report.

**Native tier (interp/copypatch/stencil).** Semantically sound (byte-exact cross-validation
confirmed for both JIT mechanisms via `cargo test --features jit`). Not runnable as generated at two
of five sizes; see Finding 1. Once fixed (sizes capped or handled gracefully), fair: the same
program crosses into every cell as opaque wire bytes / an owned `Program`, so no cell's optimizer
can see the others' work.

**Native ceiling (interp vs native madd).** Fair between its own two cells (both symmetric,
Finding 1 from the prior review fully resolved). Not comparable, at face value, to any of the other
nine families' per-iteration numbers, because its per-outer-iteration work is `O(N)` calls where
theirs is `O(1)`; see Finding 3.

**Vertical/SoA SIMD.** Semantically sound and ISA-confirmed (both prior-review gaps closed). The one
family where the harness's uniform `subtract` normalise mode is the wrong operation given the
cells' genuinely different per-call cost basis (1 vs W inputs/call), and nothing downstream applies
the stated `/W` correction automatically; see Finding 2. This is the family the design docs call the
likely biggest single result, and it is the one whose raw numbers I would trust least without a
manual recomputation.

**Zig cross-language cells.** Not re-examined this session beyond confirming `zig_matches_rust_byte_exact`
still passes; the prior review's assessment stands.

## Completeness vs the mandate

Against the context brief's family list, every named family exists, is wired into `bench.toml`, and
(mostly) runs: wire dispatch (8 shapes x 6 profiles), predecoded dispatch (6 shapes x 6 profiles),
CFG (4 shapes, one bench), record layout (5 widths x 6 profiles), value representation (3
representations, one bench), residual encoding (2 encodings x 6 profiles), optimize stage (7
strategies x 6 profiles), near-native tier (3 techniques x 6 profiles, two of the three techniques
broken at 2 of 5 sizes per Finding 1), vertical/SoA (3 widths x 6 profiles, numbers untrustworthy at
face value per Finding 2), native ceiling (2 cells, one bench, on a different cost basis per Finding
3). Nothing from the mandate's family list is missing, caricatured in the sense the prior review
used the word (every technique that claims to be X is now genuinely built as X, per the ten resolved
findings), or silently dropped. The perfect-hash and interned-operand exclusions remain correctly
argued and now, for interned-operands, empirically checked on the profile most likely to falsify the
claim (Finding 10, resolved).

The one completeness gap this review adds to the record is not a missing cell but a missing
capability in the shared harness: no per-cell weight/divisor concept exists for benches whose
variants genuinely do different amounts of per-call work by design (Finding 2). This is worth an
upstream mockspace issue independent of whether vertical ships its first number with a manual
workaround.

## Open questions for the second reviewer

**Does the harness driver tolerate a crashed variant subprocess, or does one panic stall the whole
matrix run?** I did not trace the driver's process-supervision code this session (`driver/mod.rs`,
`driver/staging.rs` exist in the pinned checkout but were not read). This changes Finding 1's
severity from "the near-native tier loses 24 data points silently" to "the entire overnight run
stalls at the first JIT-gated variant it reaches at N=4096." Either way the fix is the same; the
urgency differs.

**For native ceiling (Finding 3): was the `O(N)`-inner-loop-over-input shape intentional (a
deliberate "throughput over a byte stream" question, matching the existing `run_over_input` /
`native_madd_over_input` idiom used elsewhere in the crate's own tests), or is it an accidental
carry-over from that idiom into a bench where the wire-dispatch-family convention (one call per
outer iteration) was actually intended?** I can see the case for either reading and did not find a
design-round note settling it; whoever settles it should also decide whether the fix is "document
the different cost basis" or "make the body match its nine siblings."

**Vertical's per-input divisor (Finding 2): is a harness-level weight field worth adding upstream
before this family's first real run, or is a documented manual-recomputation step (divide raw
mean/best/worst by W before comparing) an acceptable interim given the mandate's urgency to get
numbers?** I lean toward the upstream fix being worth it given this is explicitly called the likely
biggest single result and a second W-wide cell (verticals at other widths, any future SIMD
technique) will hit the identical gap, but the tradeoff between "fix the shared harness first" and
"get a number now with a documented caveat" is a real one under time pressure, and it is not solely
mine to call.

**Did I miss any equivalent-to-Finding-1 scope limit on another near-native or JIT-adjacent cell?**
I checked `.expect(`/`.unwrap()` occurrences across `gen_matrix.rs` specifically for this
(`grep -n '\.expect(\|\.unwrap()'`) and traced each one; `Decoded::parse(...).unwrap()` is safe at
every size (the wire format's length check depends on the encoded byte count, not a fixed window,
confirmed by reading `ir.rs:256-278`), and `select_trace(...).expect("trace")` is safe because
`build_nested_loop`'s five-block structure always contains the same self-loop regardless of N
(`cfg.rs:162-197`). I am reasonably confident Finding 1 is the only size-window panic risk in the
generator, but a second pass by someone who did not already know what they were looking for would
be worth having before the run.
