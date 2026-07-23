
# Entry-point form and codegen: what `execute<const W>` actually lowers to at the C boundary

**Verdict on the candidate decomposition:** the four-axis reading (boundary form, W, entry-point form, payload, sink,
floors) is the right skeleton, but its "entry-point form" axis is binary (runtime-W vs per-W-monomorphised) when a
third form is already built, running, and load-bearing elsewhere in this tree: a single exported symbol with a
runtime `w` argument that internally dispatches, by a match on `w`, to N statically monomorphised const-generic
bodies. That is not a hypothetical; it is `mockspace-bench-macro`'s own `#[bench_variant]` expansion
(`bench-macro/src/lib.rs:320-335`), generated into every carrier variant crate in this repo
(`mock/benches/variants/carrier_dispatch_real_fntable/src/lib.rs:1-13`). The brief's framing ("C has no generics, so
the concrete lowering is the per-W entry set or a runtime-W-with-max") elides this middle form, and the middle form
is the one that actually resembles what a real runtime ABI would ship (bounded, known W set; no unbounded symbol
proliferation; no un-vectorisable generic loop). The candidate's individual bench (3) needs a third arm, and the
composition matrix needs "boundary form" demoted from an independent axis to a derived label, argued below. Where I
agree with the brief (the two-object host/runtime split is the right realisation of the boundary) I say so, and I
sharpen why for a reason the brief does not state.

## The entry-form axis, in full

Four forms exist at the C boundary once a `execute<const W>(program, columns, out) -> Digest`-shaped operation
needs a concrete ABI. Three are genuinely different lowerings; the fourth (scalar) is the W=1 degenerate anchor.

**Scalar per-record entry.** `extern "C" fn execute1(program: *const u8, seed: u64, out: *mut u64) -> u64`. One
call per record. This is `interp.rs:20` (`interpret`) wrapped at a symbol boundary, unchanged. It never touches
`Simd<u64, W>`; it is the W=1 anchor every other form's amortisation curve is measured against, and it is the only
form that can serve as that anchor honestly (see trap 7 below on why W=1 vertical is not automatically a fair
substitute).

**Runtime-W entry, internal loop, no monomorphisation.** `extern "C" fn execute(program: *const u8, seeds: *const
u64, w: usize, out: *mut u64) -> u64`, body `for r in 0..w { interpret(program, seeds[r], &mut scratch); out[r] =
checksum(&scratch) }`. `w` is a genuine runtime value; the loop trip count is not known at compile time. This form
can only ever wrap the SCALAR interpreter. It cannot wrap `interpret_vertical::<W>` at all, because `Simd<u64, W>`
is a distinct concrete Rust type per `W` (`vertical.rs:35`, gated by `LaneCount<W>: SupportedLaneCount`); there is
no Rust type "`Simd<u64, w>` for runtime `w`". A caller cannot select which monomorphisation of `Simd` to use with a
value that does not exist until the call executes. So "runtime-W entry" and "column-SoA payload" are not a free
combination in the composition matrix; they are structurally incompatible except through the third form below. This
is the single most important correction to the brief's axis list: the brief lists entry-form and payload as
independent, but they collapse into each other at exactly the point (SIMD) that motivates the whole arc.

**Dispatch-table entry.** One exported symbol, runtime `w`, an internal `match w { 4 => execute4(...), 8 =>
execute8(...), ... }` where each arm calls a statically monomorphised `execute::<W>` body (the same shape as
`interpret_vertical::<W>`). This is exactly what `#[bench_variant]` already generates
(`bench-macro/src/lib.rs:320-335`): `bench_entry` takes `n: usize`, matches it, and calls
`vehje_bench_carrier::bench::dispatch::cell_fntable::<N>` per arm with `N` shadowed as a compile-time const inside
that arm (`bench-macro/src/lib.rs:287-298`, the "typed form" dispatch). Every arm's body is fully monomorphised
(unrollable, vectorisable); the caller crosses the boundary once per call regardless of which arm fires; the callee
resolves W-to-body with one branch. This is a real, already-proven-out third category, not a permutation of the
other two.

**Per-W monomorphised entry set.** N distinct exported symbols (`execute_w4`, `execute_w8`, ...), no `w` argument at
all (W is baked into the symbol identity). The caller resolves the symbol ONCE (at bind time: header-declared name,
or a `dlsym` cached into a function pointer) and calls through that pointer repeatedly. Bodies are identical to the
dispatch-table form's arms; the only difference is where and when the W-to-body binding happens: callee-side,
every call, one branch (dispatch-table) versus caller-side, once, zero branches per call thereafter (per-W set).

The dispatch-table and per-W-set forms are NOT "the same thing with cosmetic symbol-count differences." They make
a different, measurable claim about where dispatch cost lives, and that claim is exactly what bench (3) below has
to isolate.

## Fairness traps, the check that defeats each, file:line

**Trap 1, payload leaking into entry-form's number.** The runtime-W entry can only wrap the scalar interpreter (see
above); the per-W-set and dispatch-table entries CAN wrap the vertical interpreter. If bench (3) compares "runtime-W
entry" against "per-W-set entry" at the same W using each form's NATURAL payload (scalar for one, vertical for the
other), the resulting delta is not an entry-form measurement at all: it is `interpret` vs `interpret_vertical`
wearing an entry-form costume. **Check:** bench (3) holds payload fixed at scalar for ALL three non-degenerate
entry forms (runtime-W, dispatch-table, per-W-set all wrapping `interp.rs:20`), and only bench (2) below, where
payload is deliberately the varied axis, ever pairs vertical with dispatch-table/per-W-set. Entry-form is measured
once, cleanly, on the payload every form can express.

**Trap 2, ABI argument count masquerading as monomorphisation cost.** The dispatch-table form's signature carries
one more argument (`w: usize`) than the per-W-set form's. On AArch64 (AAPCS64), the first eight integer/pointer
arguments pass in `x0`-`x7`; a fourth pointer-or-integer argument costs nothing extra at the call site, register
pressure is unaffected, no stack spill. **Check:** disassemble both call sites (extending the existing
`disasm-probe` / `isa_audit.py` discipline, `mock/benches/disasm-probe/isa_audit.py`) and confirm the argument
setup is identical modulo the one extra `mov`/register load for `w`. If the timing delta between dispatch-table and
per-W-set exceeds what one extra register move can plausibly cost (low tens of picoseconds), the delta is real
dispatch cost, not calling-convention noise; if it is within that budget, report "no measurable ABI-shape cost,"
not a fabricated one.

**Trap 3, resolving the symbol inside the timed region.** The per-W-set form's entire advantage over the
dispatch-table form is that the caller resolves the symbol ONCE, outside the hot loop. If a bench's `cell` closure
does the `dlsym` (or the moral equivalent, a `match` on a string picking which function pointer to hold) on every
timed call, the "zero dispatch branches per call" claim is fabricated: the bench would be measuring symbol-table
lookup cost, not steady-state per-call cost. **Check:** the existing `warm` scaffold's S/I split is exactly the
mechanism that prevents this, and it must be used correctly: `setup: FnOnce(usize) -> St` runs once and is timed as
`S` (`scaffold.rs:94-110`); the resolved `fn` pointer belongs in `St`, built inside `setup`, never inside `cell:
FnMut(&mut St, u64) -> u64` which runs per-iteration under calibration. Any per-W-set variant whose `cell` re-derives
which symbol to call, rather than reading a pointer already resolved in `St`, is measuring the wrong thing and the
per-W-set number is void.

**Trap 4, a fixed `w` across the whole run flatters the dispatch table.** A branch predicted the same way for
thousands of consecutive calls (dispatch-table's `match w` with `w` constant across a warm run) is nearly free after
the first few calls; that is not the real workload's shape, where a residual may stream columns of varying trailing
width (a batch of W=64 records followed by a W=23 remainder). **Check:** run the dispatch-table cell under BOTH the
warm regime (fixed `w` for the whole run, the optimistic case) and a `cold_cycle`/`stream`-style regime that varies
`w` across the harness's declared W set between calls, so the predictor cannot lock onto one target
(`mockspace-bench-matrix`'s cold/stream scaffolds, reused per the "do not rebuild" instruction in the brief). Report
both rows. The gap between them IS the dispatch table's real, workload-dependent cost; reporting only the fixed-`w`
row overstates the form.

**Trap 5, the floor stack needs an entry-form-shaped middle rung.** The composition arc's floor discipline names
"an in-process no-FFI call" as the boundary-free floor and stops there. For this arc that single floor conflates two
distinct costs: indirect-call overhead (paying for a `fn`-pointer call instead of a direct, staticaly-resolved call,
which exists in-process too) and cross-object ABI overhead (the actual dlopen'd boundary crossing). **Check:** three
floors, stacked, not one: (a) direct in-process call to the monomorphised body, no `fn` pointer, fully inlinable,
the true zero; (b) in-process call through an already-resolved `fn` pointer, no dylib boundary, isolating indirect-
call cost alone; (c) the cross-cdylib dlopen'd call, isolating genuine ABI-crossing cost on top of (b). Skipping (b)
means a bench that finds "form 4 costs X over the direct-call floor" cannot say how much of X is "indirect call" (a
cost every real deployment pays too, process boundary or not) versus "actually crossing an object boundary." The
composition arc's own backbone already decomposes a native ceiling and a null-dispatch floor this way
(`00_context.md:106`); this arc needs the same decomposition applied to the entry-form axis specifically.

**Trap 6, the auto-vectoriser might grant (or the isolation might suppress) SIMD nobody asked for.** Per-variant
cdylib isolation (`lto=fat`, `codegen-units=1`, e.g. `mock/benches/variants/carrier_dispatch_real_fntable/Cargo.toml:16-18`)
is there so the optimizer cannot partial-evaluate ACROSS the harness/variant boundary; it says nothing about what
LLVM does INSIDE one variant's own compiled body. The runtime-W entry's internal loop (`for r in 0..w {
interpret(program, seeds[r], &mut scratch) }`) calls `interpret` W times with independent inputs; LLVM's loop
vectoriser handles unknown trip counts via a vector-body-plus-scalar-remainder pattern and, in principle, could try
to vectorise across `r`. In practice `interpret`'s inner loop reads `rload(rp, d.operand(i, k, 2))`
(`interp.rs:27-29`), a program-data-dependent gather, which is the textbook autovec blocker (SLP and the loop
vectoriser both need provably-constant-stride, non-aliasing access; an operand index read from the decoded program
is neither). So auto-vectorisation across the outer `r` loop is unlikely, not impossible, and "unlikely" is not a
number. **Check:** disassemble the runtime-W scalar cell and confirm zero packed/NEON instructions (no `v`-register
traffic, no `.2d`/`.4s` suffix); only the `Simd`-typed dispatch-table/per-W-set cells should show NEON. This extends
the exact discipline already in `interp.rs:129-144` (the `di_ifchain` / `di_ifchain_linear` probes that confirm
jump-table-vs-linear-scan lowering by disassembly, not by source-level assumption); the same probe style applies
here to confirm scalar-vs-vector lowering.

**Trap 7, W=1 "vertical" is not automatically the scalar baseline.** `Simd<u64, 1>` is a legal instantiation
(`LaneCount<1>: SupportedLaneCount` holds) but there is no guarantee LLVM lowers a one-lane SIMD op to the same
scalar GPR instruction the plain `u64` arithmetic in `interp.rs` produces; a one-lane vector ADD could legitimately
round-trip through a NEON register for no benefit. If it does, comparing the vertical sweep's W=1 point against the
scalar-entry baseline silently compares two different instruction sequences computing the same numbers, and any
"vertical wins starting at W=2" claim is contaminated by an unfair W=1 anchor on the vertical side. **Check:**
disassemble the vertical cell at W=1 and confirm it lowers to plain scalar GPR ops identical to `interpret`'s; if it
does not, exclude vertical W=1 from the fair-comparison claim and use only the true scalar-entry cell as the W=1
anchor for every composition-matrix row (which the brief's axis list already implies by keeping "scalar per-record"
as its own boundary form, but the composition matrix must enforce it, not just imply it).

## Boundary realisation: two objects, but not the object pair the brief sketches

**I agree with the brief that a genuine two-object split is required, for a reason the brief does not state.** The
existing harness ALREADY gives every carrier cell a real cross-object C-ABI call: `bench-harness/src/harness.rs:97-115`
`dlopen`s the variant `.dylib`, resolves `bench_entry` via `libloading::Symbol`, and the worker process calls it
repeatedly inside the calibrated loop. `lto=fat` + `codegen-units=1` is scoped per-crate
(`mock/benches/variants/carrier_dispatch_real_fntable/Cargo.toml:12-18`), so nothing inlines across that dlopen
boundary. So "does crossing a dlopen'd ABI cost something over an in-process call" is already answered structurally
by every existing cell; this arc does not need a special setup to ask that question again.

What the brief's two-object shape is actually for is different, and more load-bearing: making the PER-CALL-COUNT
itself a real, un-elidable cost. The whole ABI question this arc exists to answer is "does crossing the boundary
N times (W=1) cost more than crossing it N/W times (W=64)," and that comparison is worthless if "crossing the
boundary" is modelled as an ordinary in-crate Rust function call, because the compiler can (and, under
`lto=fat`/`codegen-units=1` within ONE variant crate, almost certainly will) inline a same-crate loop that calls the
payload function W times into the loop body directly, at which point "W calls" and "one call processing W records"
compile to indistinguishable code and the entire axis under test vanishes into the optimizer. The two-object split
is not there to prove FFI has a cost (known); it is there to make each SIMULATED per-batch call a genuine,
non-inlinable, ISA-visible `bl`/`blr` that the harness's outer dlopen'd call cannot already provide, because the
harness's dlopen boundary sits ONCE around the whole `bench_entry` call, not once per simulated record-batch inside
it.

**Where I would change the brief's realisation:** not a bespoke host/runtime CRATE PAIR authored per cell. That is
N-choose-2 duplication (one host crate and one runtime crate per entry-form x W combination) for a payload that
never changes. Reuse the precedent already built in this tree: `mock/benches/carrier-zig/` is exactly a shared
runtime object (`interp.zig`, `export fn carrier_zig_switch(...) callconv(.c)`, built once via `build.sh` into
`libcarrier_zig.dylib`) that a consumer dlopens and drives. The right shape is one shared runtime cdylib (Rust,
`crate-type = ["cdylib"]`, exporting `execute1` / `execute_w{W}` per per-W-set, plus one `execute` dispatch-table
symbol, all wrapping the SAME unmodified `interp.rs` / `vertical.rs` bodies, zero new payload logic) built once,
and N thin `#[bench_variant]` host crates (one per entry-form x W cell), each whose `setup` `dlopen`s the shared
runtime object and resolves its one symbol of interest into `St` (satisfying trap 3's discipline for free, since
`setup` is exactly where dlsym belongs), and whose `cell` drives it under `warm`/`cold_cycle`. This is cheaper to
build, matches the "do not rebuild machinery" instruction, and avoids inventing a duplication pattern the workspace
already treats as a smell.

**A further sharpening the brief does not raise at all: the shipped runtime is Zig, not Rust, and that changes what
"the runtime object" should be.** `callconv(.c)` in Zig and `extern "C"` in Rust both target the platform C ABI
(AAPCS64 here), so the ISA-level call SEQUENCE at the boundary should be identical either way; what differs is the
CALLEE-side codegen, because LLVM-via-Zig and LLVM-via-rustc apply different inlining and vectorisation heuristics
inside the callee body even for semantically identical source. A Rust-cdylib runtime stand-in answers "what does
crossing a C ABI cost" (language-agnostic, ISA-level) honestly; it does not automatically answer "what does calling
into the REAL vehje runtime cost" (callee-side, language-specific), and treating the Rust stand-in as a full proxy
for the Zig runtime without confirmation is an unstated assumption, not a measured fact. `carrier-zig`'s own header
comment already names Zig as "the load-bearing cross-language cell" for the dispatch question
(`interp.zig:1-8`); the same reasoning applies here. **Recommendation:** build the entry-form matrix primarily on
the Rust runtime stand-in (fast, reuses `interp.rs`/`vertical.rs` unchanged), and extend `libcarrier_zig.dylib` with
the equivalent `execute1` / `execute_w{W}` / dispatch-table exports (mechanical: the file already has the byte
layout, opcode table, and checksum reduction; only the batch-loop and SIMD-lane wrapper are new) as a language-
boundary confirmation cell, cross-validated byte-exact the same way `carrier-zig`'s existing cells already are.
This is a whole category the candidate decomposition misses, not a variant of one already listed: it is asking
"does the language on the other side of the boundary change the entry-form conclusion," which none of the brief's
five axes (boundary form, W, entry form, payload, sink) can express.

## Individual benches (revised from the candidate)

**(0) ISA-shape gate, not a timing bench.** Before any of the below produce a trusted number: disassemble the call
site and callee body for every entry form (scalar, runtime-W, dispatch-table, per-W-set) and confirm each is
actually the code shape it claims (per-W-set's caller holds a resolved pointer and calls it directly with no branch;
dispatch-table's callee contains a jump table or predictable single-target branch on `w`; runtime-W's callee
contains a scalar loop with no packed instructions). This is not optional infrastructure; it is a gating bench,
because it is entirely possible for two of these forms to compile to indistinguishable code once one arm dominates
a warm run (trap 4), and reporting a spurious timing delta between two identical instruction sequences would be
worse than reporting nothing. Extends `disasm-probe` / `isa_audit.py` and the `di_ifchain` precedent
(`interp.rs:129-144`) directly.

**(1) Entry-form x W, scalar payload only.** The merged form of the brief's (1) and (3): sweep W (1, 2, 4, 8, 16,
32, 64, ...) across all four entry forms (scalar-anchor at W=1 only, then runtime-W / dispatch-table / per-W-set at
each W), payload pinned to `interp.rs:20` for every cell (trap 1's fix). This isolates the boundary-amortisation
curve AND the entry-form choice in one clean sweep, without SIMD in the mix, so the S+kI cost-model fit
(`00_context.md:63-64`) directly yields the FFI-call-amortisation knee per entry form, and the three floors from
trap 5 bracket it.

**(2) Payload x entry-form, SIMD-capable forms only.** Cross vertical/SoA payload (`vertical.rs:35`) against
dispatch-table and per-W-set entry forms ONLY (runtime-W structurally excluded, see the entry-form section), at
each vertical-supported W. This isolates the column-SoA win specifically, cleanly separated from bench (1)'s
entry-form-alone question. The scalar-payload row of bench (1) at matching W is the shared baseline both bench (1)
and (2) report against, so the two benches compose without re-measuring the scalar floor twice.

**(3) Output-sink shape x W.** Per-record reserve/commit versus batched-columnar reserve/commit versus the
null-sink floor, crossed with W, using the same S+kI treatment. The existing `output_building.rs` full/compact/
inline three-way (`output_building.rs:1-24`) is the in-process precedent for "output materialisation is its own
composable stage separate from dispatch"; this bench is that same stage measured across the ABI boundary instead of
in-process, with the null-sink floor mandatory (not one bullet among several, since op's evidence-honesty framing
elsewhere in this arc treats an un-isolated axis as an incomplete floor stack).

**(4) Language-boundary confirmation.** Rust-stand-in-runtime versus the extended `libcarrier_zig.dylib`, at the
entry forms and W points bench (1)/(2) found most decisive, cross-validated byte-exact per `carrier-zig`'s existing
discipline. Answers whether the Rust stand-in's entry-form conclusion transfers to the actual shipped-language
boundary, or whether it is a Rust-specific artefact of rustc's inlining/vectorisation heuristics.

## Composition matrix: drop "boundary form" as an independent axis

The brief's candidate composition matrix lists "boundary form x W x entry form x payload x sink" as five
independent axes to cross. Boundary form is not independent; it is a name for a REGION of the (entry-form, payload,
W) cube, and treating it as a sixth free dimension manufactures cells that are either impossible (scalar-per-record
boundary form paired with a per-W-monomorphised entry form) or degenerate (W=1 paired with vertical payload, per
trap 7). Concretely: "scalar boundary" = {any entry form} x {scalar payload} x {W=1}; "batched-scalar boundary" =
{dispatch-table, per-W-set} x {scalar payload} x {W>1}; "column-SoA boundary" = {dispatch-table, per-W-set} x
{vertical payload} x {W>1, W in the vertical-supported set}. Build the matrix over the three real independent axes
(entry form, payload, W), plus sink shape and regime (warm/cold-many) and language (Rust-stand-in/Zig), and derive
"boundary form" as a report-time label over the coordinate, not a swept dimension. This prunes the matrix to
structurally coherent cells only and matches the discipline of not permuting axes that do not actually vary
independently.

The resulting matrix, in the composition-arc's own shape (cost-model lines, floor decomposition, oracle envelope,
selector): rows are (entry form, W, payload-where-legal, sink shape, regime, language), reported per the six
designed program profiles (P_real/P_madd/P_tight/P_scatter/P_wideselect/P_leaf, `gen.rs:99-187`) the same way the
interpreter composition matrix already does, since program shape (arity mix, decode-vs-arithmetic weight) plausibly
interacts with how much a batched entry amortises (P_leaf's decode-bound shape should amortise the boundary crossing
less usefully than P_real's balanced shape, because the per-node cost the boundary crossing sits on top of differs).

## Open questions for the synthesiser

Whether the per-W-set form's symbol count is itself a real-runtime-ABI constraint worth benching (an unbounded W
set means an unbounded export table; the dispatch-table form bounds the exported surface to one symbol regardless
of how many W values are supported, which may matter for the real ABI decision independent of raw speed).

Whether the "cold-many" regime (many distinct programs, per `00_context.md:63`) interacts with entry-form choice
differently than with dispatch-form choice in the prior arc: a cold-many regime re-resolves `St` (and hence the
per-W-set's cached pointer) every program, which may erase per-W-set's steady-state advantage entirely in that
regime, making dispatch-table the actual winner for a cold-many-shaped real workload even if per-W-set wins warm.

Whether extending `libcarrier_zig.dylib` for the vertical/SIMD entry forms is in scope for this arc's first pass, or
whether the Rust-stand-in-only matrix ships first and the Zig confirmation cell is a fast-follow; Zig's own SIMD
vector types (`@Vector`) would need their own small port, which is mechanical but not zero-cost, and the arc's
schedule may prefer shipping the scalar-payload Zig confirmation (bench 1's forms only) before the vertical one.
