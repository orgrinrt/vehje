# Breadth and honesty: the runtime C ABI batched-execute bench arc

**Fork:** Haoran Xu's lens (interpreter/JIT boundary engineering, benchmark methodology).
**Reacting to:** `00_context.md` in this directory.

I read the brief, the settled sink decision at `mock/design_rounds/202607201315_topic.runtime-output-value-transfer.md`,
the current `vehje-runtime-abi` source, the full carrier corpus (`mock/benches/carrier/src/`), and the
`mockspace-bench-matrix` / `mockspace-bench-harness` sources that the arc is required to reuse rather than
reinvent. The verdict up front, because it should not take a whole document to say: the candidate's five axes
are the right shape of skeleton, but they conflate two pairs of things that must be pulled apart (layout versus
vectorization, batch width versus program size), they propose an output-sink shape that is not the one the
design round actually settled, and they are silent on three whole categories a batched host-call boundary
cannot honestly be measured without: who owns the buffers across a call, what the same-toolchain cdylib floor
is a floor *for* rather than a ceiling estimate *of*, and what happens to the cost model when a batch faults
partway through. None of that is a permutation of the axes already on the page. It is the category-breadth
failure the brief's own origin story warns about, and I would rather name it now than build five careful
benches around a hole.

The methodology stakes are the same ones I have made a career of belaboring. LuaJIT's own JIT loses to a
carefully engineered interpreter on real workloads not because anyone was clever, but because the interpreter
tier had never been held to the JIT's own standard of measurement: real disassembly, a shared unchecked
operand primitive so an axis varies only its own dimension, and floors that decompose the number rather than
asserting it. This arc is exactly that discipline applied one layer out, at the boundary between an embedding
host and a compiled runtime, and the discipline does not get to relax just because the crossing is a `dlopen`
symbol instead of a dispatch loop. A batched-execute ABI bench that reports "batching wins by 4x" without first
proving the 4x is not an artifact of an under-amortized call floor, a hidden transpose, or a reps-starved
calibration at the high end of the width sweep is not evidence. It is a number that happens to point the
direction someone wanted.

## The category enumeration

| Category | In / out of scope | Candidate coverage |
|---|---|---|
| Call-crossing cost and its amortization over W | In (primary question) | Candidate names it as boundary form + W sweep; broadly right, needs the wrapper-layer sub-question folded in |
| Marshalling layout, AoS vs SoA, at the boundary | In, but must be split from vectorization | Candidate conflates it into "boundary form"; must be its own axis |
| Entry-point form (runtime-W vs per-W monomorphised) | In | Candidate has this; needs the LLVM-cannot-see-through-an-erased-loop-bound mechanism made explicit, and disassembly confirmation |
| Payload vectorization win surviving the boundary | In (the brief's stated headline question) | Candidate has this correctly, contingent on the layout split above |
| Output/value-transfer, sink shape under batching | In, but against the real sink, not a placeholder | Candidate's sink axis models a generic reserve/commit, not the settled `#[repr(C)]` two-fn-ptr struct |
| Residency and ownership of the batch buffers | In, missing from the candidate | Absent |
| Warm vs cold **instance lifecycle** (not program identity) | In, missing as a distinct axis | Silently reused the interpreter arc's warm/cold label for a different concept |
| Cross-language host (a real Zig-compiled boundary) | In, missing from the candidate | Absent; candidate's cdylib realisation is Rust-host-calls-Rust-cdylib only |
| Error-path / partial-batch-fault contract | Named, not benched as timing | Absent; and it cannot be benched honestly until designed |

Nine categories, not five. Two are refinements of what the candidate already named (splitting one axis into
two, or renaming a mislabeled one); three are whole categories the candidate does not touch at all. None of the
nine is a new permutation of W, boundary form, entry form, payload, or sink; each is a genuinely different
question a batched host-call boundary raises that those five axes do not answer.

### Call-crossing cost and its amortization (in scope, primary)

The brief is right that this is the load-bearing question: does a batched entry pay a per-crossing fixed cost
(`S`) that amortizes as W grows, and at what W does it stop mattering. The `mockspace-bench-harness` cdylib
isolation already gives a genuine, non-inlinable cross: `harness.rs:97-124` resolves one symbol via
`libloading::Library::new` plus `lib.get::<BenchEntryFn>`, once per worker process, and calls it as a raw
function pointer with no vtable and no trampoline. That is a real C ABI call the optimizer cannot see through,
which is exactly what a "does the batching survive the boundary" bench needs.

One sub-question the candidate does not separate out: if vehje ships a safe Rust wrapper over the raw
`extern "C"` entry (the framework's own `vehje-runtime-driver` sits exactly there, per the repo's own
description, "compiler-side dispatch over the ABI"), the wrapper either inlines away, leaving one real
crossing, or it does not, leaving two. This is the same question PyO3's generated shim or wasm-bindgen's JS
glue answer for their respective boundaries: a thin wrapper that inlines is free; a thick one is a second
tax on top of the one being measured. I read the brief's boundary as host-raw-C-ABI only (the driver is
compile-side, a third party the brief never names), so I am not proposing a wrapper-inlining bench as a
first-class category here, but flag it as a scope question for the synthesiser (see Open Questions).

### Marshalling layout, AoS vs SoA (in scope, must be split from vectorization)

The candidate's "boundary form" axis bundles three things that are not one dimension: how many records per
call (W), what shape the records take on the wire (array-of-structs vs structure-of-arrays), and whether the
runtime vectorizes what it receives. AoS-with-W=64 and SoA-with-W=64 are two different marshalling costs at
the identical batch width, independent of whether anything downstream is vectorized. Real systems keep this
distinction sharp because conflating it is a documented source of bad benchmarks: the Arrow C Data Interface
defines `ArrowArray`/`ArrowSchema` as a stable, language-neutral struct pair specifically so a columnar
(SoA) layout can cross a language boundary at zero copy, and the whole reason it exists as a separate spec
from "just pass a pointer" is that AoS-to-SoA transposition at a language boundary is a real, chargeable cost
that a naive zero-copy claim likes to hide. NumPy's own ufunc broadcasting machinery is the same lesson from
the other direction: broadcasting over strided (not necessarily contiguous) SoA arrays is a distinct code
path from the contiguous fast path, and the difference is measured, not assumed.

### Entry-point form: runtime-W versus per-W monomorphised (in scope)

The candidate's framing ("C has no generics, so the concrete lowering is the per-W entry set or a runtime-W")
is correct as a description but understates the mechanism. A `extern "C" fn(..., w: usize)` with an internal
loop bounded by a runtime value is a loop LLVM cannot prove a trip count for at the ABI boundary, so it
cannot unroll or auto-vectorize it to the same quality as a loop bounded by a `const W: usize` monomorphised
into a distinct symbol, the same reason ISPC, Halide, and BLAS/LAPACK-family kernels all generate per-tile or
per-width instantiations rather than one runtime-parameterised kernel for their hot inner loops. This bench
must be disassembly-confirmed the same way the interpreter arc confirmed every dispatch label: a per-W entry
claiming a monomorphisation win needs to show real unrolled or packed-vector instructions in the object dump,
or the claim is unconfirmed the same way an "obviously tail-called" handler turned out not to be, back when
nobody checked.

### Payload vectorization win surviving the boundary (in scope, the brief's headline)

This is `interpret_vertical<const W>` (`mock/benches/carrier/src/vertical.rs:35-78`), already const-generic
over W and already taking SoA `seeds: &[u64; W]`. It is the correct payload to reuse; the honesty trap is
what happens to it at the boundary, covered below.

### Output/value-transfer, sink shape under batching (in scope, against the real sink)

The candidate's fourth individual bench ("output-sink shape under batching") is the right question, but it
must be built against the ABI the design round actually settled, not a stand-in. `202607201315_topic...md:52-60`
is explicit: the sink is "a `#[repr(C)]` struct of two function pointers plus opaque userdata," an FFI wire
shape, "not a `dyn` object," with `reserve(hint)` and `commit(n)`. That shape has a call-cost profile of its
own, separate from the entry point's, and it must be modeled as such (see Honesty Trap 5).

### Residency and ownership of the batch buffers (missing from the candidate, must add)

Nowhere in the candidate's five axes is the question of who owns the memory a batched call reads from or
writes into across repeated calls. This is not a footnote. The design round itself names it as still open:
"whether the whole-value region is sized by the host up front with streaming as the overflow path (the
current lean) or ever benefits from a runtime-scratch variant, which is bench-decidable if it becomes
contentious" (`202607201315_topic...md`, closing paragraph). That sentence is a standing request for exactly
the bench this arc is designing. Real precedent for why this axis has its own cost curve, independent of W:
the CPython buffer protocol (`PyObject_GetBuffer`/`PyBuffer_Release`) negotiates borrow-versus-copy and
contiguous-versus-strided as an explicit, separately-costed step at a language C ABI boundary; JNI's
`GetPrimitiveArrayCritical` (pin, no copy, but constrains what the callee may do while holding it) versus
`GetByteArrayElements` (may copy, no such constraint) is the same tradeoff at the JVM native boundary; and
`io_uring`'s registered buffers (`io_uring_register` pins a fixed buffer set once, amortizing page-pinning
cost across every subsequent submission that reuses it) versus per-call buffer submission is the identical
tradeoff in a syscall-batching context. A host that registers its column buffer once and reuses it across many
batched calls pays a different, and probably much smaller, amortized cost than a host that hands the runtime a
fresh region every call, and the design round's own "host-lent up front" decision is exactly a residency
decision that deserves its own sweep, orthogonal to W.

### Warm versus cold, but of the runtime instance, not the program (a mislabeling to fix)

The interpreter-composition arc's warm/cold regimes mean "one program, many evals" versus "many distinct
programs" (`00_context.md`'s evidence section). If this arc silently reuses that label for the ABI, it will
be measuring a different thing under the same name: at the ABI layer, "cold" naturally means a fresh runtime
instance (a fresh residual load, a fresh region reservation, any first-tier warmup) crossed once, versus
"warm" meaning a long-lived runtime instance taking many batched calls across its life. That is a genuine,
separate axis, not the interpreter arc's program-identity axis relabeled. Wasmtime's instance-pooling
allocator exists precisely because fresh-module-instantiation-per-invocation is expensive enough that a
long-lived pool amortizing it across many invocations is worth a dedicated allocator strategy; V8's isolate
snapshot machinery solves the identical problem for a different embedding shape. Name this axis
`instance_lifecycle` (or similar) explicitly distinct from any reused `warm`/`cold` label, so a reader does not
assume the two arcs' "cold" mean the same thing.

### Cross-language host: a real Zig-compiled boundary (missing, must add)

The framework's own identity is explicit: the compiled artifact on the runtime side is Zig, "small, fast,
embeddable," speaking C ABI, and the host is by design an arbitrary embedder, not necessarily Rust. The
candidate's realisation of the boundary ("a 'runtime' cdylib... and a 'host' variant that dlopens it") as
built by the harness today is Rust-compiles-a-cdylib, Rust-dlopens-it. That is a genuine, honest floor for
the case where host and runtime happen to share a toolchain, and it is the correct floor to report as such.
It is not evidence about the real deployment shape, because a true cross-compiler boundary carries costs the
same-compiler case cannot: struct-return classification for a multi-word aggregate (the `FfiBenchCall`-style
return) is decided per-ABI (the SysV x86-64 classification algorithm versus AAPCS64's homogeneous-aggregate
rules differ in what returns in registers versus via a hidden pointer), and while both sides target the
platform's canonical C ABI, no LTO across the boundary is EVER possible for the cross-language case, whereas
the Rust/Rust cdylib case is only non-inlinable because of the `dlopen` isolation technique, not because
Rust-to-Rust LTO is structurally impossible the way Rust-to-Zig is. The interpreter-composition arc already
paid for exactly this precedent: its Zig cross-language cell ran "switch and guaranteed-tail-call threading
on identical wire bytes... confirmed at the ISA level" against the Rust cells. This arc needs the same thing
for the entry-point form and boundary-layout benches, or the cross-language floor is assumed, not measured,
and the assumption is doing the load-bearing work the whole arc claims to avoid. LuaJIT's own FFI is the
sharpest illustration of why this matters: a JIT-compiled `cdata` call inlines to a native call with
essentially no boundary tax, while an interpreter-mode C API call through `lua_call` pays a real, measured,
and very different cost, and nobody would trust a LuaJIT FFI benchmark that only ever measured one of the two
regimes and called it "the FFI cost."

### Error-path cost under a partial-batch fault (missing, named but not benched as timing)

None of the five candidate axes address what happens when record 12 of a 64-record batch traps: an
out-of-bounds access, an arithmetic fault, an effect-proof violation the runtime catches at the tier-0
interpreter level. This changes the return contract's shape (does the entry return a completed-count, an
error code plus a partial-completion count, or a per-record status vector), and the return contract's shape
is exactly what the whole cost-model fit is measured against. Real systems solve this with masking rather
than per-record traps precisely because a per-lane trap in a vectorized loop is expensive or inexpressible:
AVX-512 and ARM SVE predicate execution rather than trapping per lane; NumPy's ufunc error state is a
global flag checked after the vectorized loop runs, not a per-element exception; a vectorized query
executor's selection vector masks out rows that failed a predicate mid-pipeline rather than aborting the
batch. This matters to the ABI decision because the batched entry only beats the scalar entry if a
fault-prone program does not force a fallback to record-at-a-time processing; if it does, every real-world
program with occasional bad records pays the scalar tax anyway and the batching win is a fair-weather number.
I am not proposing a timing bench for this, because there is no honest way to time an API contract that has
not been decided; the return struct's shape has to be settled before B4/B5 lock their layout, or the numbers
this arc produces get invalidated by a later ABI change, which is exactly the claim-versus-reality drift the
workspace's own CL-claim discipline exists to catch. This is a design question to surface, not a bench to
build.

## Honesty traps

Six, each naming the naive bench, what it would actually measure, the fix, and where in the tree it bites.

**1. Reps starvation at the high end of the W sweep.** A naive bench times N individual calls at each W by
letting the harness's `timed_calibrated!` machinery pick reps to hit a duration floor. Since a call at W=256
takes proportionally longer than a call at W=1, the calibration will select fewer reps for large W to stay
near the same wall-clock floor. Fewer reps means less amortization of any first-touch cache effects specific
to a large batch's memory footprint, so the naive bench would report the high end of the W sweep, exactly the
region the "does the win survive" question cares about most, with the least statistical confidence and the
most first-touch contamination. It would look like noise on the graph and get explained away as "diminishing
returns," when it is a calibration artifact. Fix: report reps-per-cell (the harness already emits a
`batch_count` column per the worker output line format, `bench-harness/src/harness.rs` around the `run_worker`
doc block) and enforce a minimum-reps floor across the whole W sweep so every point gets comparable
statistical power, not comparable wall-clock time.

**2. `n` collision between program size and batch width.** The existing `BenchEntryFn` typedef
(`mockspace-bench-core/src/lib.rs:429`, `unsafe extern "C" fn(input: *const u8, output: *mut u8, n: usize)`)
already uses `n` as a size parameter, and every existing carrier family sweeps its own `n`-equivalent as
program size (`node_count`) via `MatrixDecl.sizes: Vec<usize>` (`mockspace-bench-matrix/src/decl.rs:65-89`).
If this arc reaches for the same slot to carry batch width W, a family cannot independently sweep program
size and batch width at once; the cost-model fit `total(k) = S + k*I` would report `I` as "cost per unit of
whichever number lives in `n`," silently conflating "wider batch" with "bigger program" if both move at once
or if the author is not careful about which one the `sizes` axis represents in a given family. Fix: hold
program size fixed per designed profile (reusing the six profiles' `GenParams::default_point`-derived sizes)
and dedicate the `sizes` slot exclusively to W within the boundary-amortization families; where a genuine
program-size-times-W cross is wanted, follow the `entgrid.rs` precedent (`mock/benches/carrier/src/bench/entgrid.rs:1-9`)
of naming each cross point as its own explicit cell rather than expecting one numeric axis to carry two
meanings.

**3. The Rust/Rust cdylib floor reported as "the ABI cost" rather than a same-toolchain floor.** Covered above
under Cross-language host; restated as a trap because it is the single easiest number to over-claim. The
`libloading`-resolved direct symbol call (`bench-harness/src/harness.rs:97-124`) is about as cheap as a C ABI
call gets on this platform: no calling-convention translation, because both sides were compiled by the same
rustc for the same target triple. Reporting that number as "the cost of the vehje C ABI" without a real
cross-language (Zig) comparator present in the same table is the honesty gap. Fix: build a genuine
Zig-compiled cdylib cell for the entry-point and boundary-layout families (Open Question 2 covers how, given
`vehje-runtime-abi` has no `extern "C"` entry points yet) and label the Rust/Rust numbers explicitly as a
lower bound, never a ceiling estimate.

**4. Charging (or hiding) the AoS-to-SoA transpose inside the "vectorization win" cell.** If the column-SoA
cell is built by transposing the harness's AoS `input` bytes into a `[u64; W]` array fresh inside the timed
call before handing it to `interpret_vertical` (`mock/benches/carrier/src/vertical.rs:35`), the transpose cost
gets silently folded into "the vectorized dispatch," either understating the real win (if a real host would
hand over SoA columns natively, making the transpose free) or overstating the crossing's true cost (if a real
host is genuinely AoS and must pay the transpose on every call). This is the interpreter arc's own "fixed cost
hidden in untimed setup" failure, recurring one layer up: the scaffold's `Measured.setup_ticks` field exists
precisely because that failure was found once and structurally fixed (`mockspace-bench-core`'s
`FfiBenchCall` doc comment, fields at `lib.rs:419-423`). `interpret_vertical` itself is innocent here: it
correctly takes an already-SoA `seeds: &[u64; W]` (`vertical.rs:35-40`), meaning the transpose is not its job
and is not accidentally included in its own cross-validated cost. Fix: split into `column_soa_native` (host
already produces SoA, no transpose in the timed region) and `column_soa_transposed` (the transpose is an
explicit, separately reported stage inside the timed per-call region, its own scratch pre-allocated in setup
per `vertical.rs:99-105`'s own discipline for the results buffer, applied here to the transpose buffer
instead).

**5. Building the sink cell as a Rust closure instead of the real two-function-pointer struct.** The scaffold's
own isolation argument is explicit that a cell must be a generic `FnMut` type parameter, "NEVER a `fn` pointer
and NEVER `&dyn Fn`," because a generic monomorphizes and inlines across the fat-LTO cdylib link
(`mockspace-bench-matrix/src/scaffold.rs:8-14`). That is exactly right for measuring dispatch, and exactly
wrong for measuring the sink, because the sink the design round settled on IS the opposite shape by design: a
`#[repr(C)]` struct of two function pointers plus opaque userdata, "an FFI wire shape... not a `dyn` object"
(`202607201315_topic...md:54-56`). If the sink cell's body is a closure the compiler inlines, the bench reports
how cheap a Rust closure is, not how much the settled ABI's reserve/commit round-trip costs, understating it by
however much an un-devirtualizable indirect call through an opaque-userdata function pointer costs relative to
a monomorphized inline. Fix: construct the actual struct-of-fn-pointers shape and call through it as an
indirect call, deliberately outside the scaffold's `FnMut` cell convention for this one axis, with the
divergence from the convention documented at the cell so a future reader does not "fix" it back to a closure.

**6. Presenting a single-instance micro-benchmark's cross-language number as deployment-representative without
a stub-fidelity caveat.** If B7 (the Zig floor) is built against a hand-written minimal Zig stub rather than
the real future runtime (unavoidable today, since `vehje-runtime-abi/src/lib.rs:52-54`'s own `FIXME` says the
entry points "land later"), the resulting number is a floor for "calling into ANY Zig-compiled `extern "C"`
symbol from this host," not a floor for "calling into vehje's eventual real runtime specifically." That gap is
fine and worth building, but it must be labeled as a stub-fidelity floor in the report, not folded into the
same table as the payload/vectorization benches (which run the actual carrier IR) without the caveat attached.

## Individual benches, then the composition matrix

Eight individual benches (six timing, one flagged non-timing per the error-path discussion, one existing
category the candidate already has correctly framed once split). Each isolates exactly one axis; the ones
downstream of an earlier one pin that earlier one's best-known point rather than re-sweeping it, the same
discipline the interpreter arc's setup/residual/output-building families already follow.

1. **Call-crossing amortization over W** (same-toolchain floor). Fixed program (one representative profile at
   a mid-size), fixed entry form (runtime-W, single entry, internal scalar loop, the simplest realisation),
   fixed layout (batched-scalar AoS). Sweep W via `sizes`. Cells: an in-process no-FFI floor (same loop, direct
   call, no `dlopen`, isolating whether there is a crossing at all from how it amortizes), the `dlopen`-resolved
   Rust/Rust floor, a null-dispatch floor. Reports the cost-model fit `total(W) = S + W*I`; `S` should trend
   toward the per-crossing fixed tax as W grows, `I` should trend toward the known scalar per-node cost since
   no vectorization is in play. Answers "at what W does the crossing tax stop mattering."

2. **Marshalling layout, held pre-vectorization.** Fixed W (this bench's own pinned point from bench 1's
   knee), fixed entry form, payload held scalar (deliberately, so layout cost is isolated from vectorization
   cost, per Trap 4). Cells: AoS batched-scalar, SoA-native (host already produces columns), SoA-transposed
   (transpose charged explicitly in the timed region). Answers "does SoA cost anything at the boundary before
   it buys anything."

3. **Column-SoA vectorization win surviving the boundary** (the brief's headline). Fixed layout at both SoA
   variants from bench 2, vary payload (scalar vs `interpret_vertical::<W>`) across the same W sweep as bench 1
   so the two are directly comparable. Reports whether the roughly-1/W per-input dispatch win (established
   in-process by the interpreter arc) survives crossing the real boundary, decomposed against bench 1's and
   bench 2's floors.

4. **Entry-point form.** Fixed boundary/layout/payload at bench 1 through 3's best-performing combination, vary
   only entry shape: scalar entry called W times; runtime-W single entry (bench 1's baseline); per-W
   monomorphised entries (`execute_w4`/`execute_w8`/... generated over a const-W macro). Disassembly-confirmed:
   the monomorphised entries must show real unrolled or packed-vector codegen or the claimed win is
   unconfirmed, same bar as the interpreter arc's dispatch-label confirmation.

5. **Output-sink shape under batching**, against the real sink (Trap 5's fix). Fixed everything else at the
   winner from benches 1 through 4. Cells: per-record reserve/commit called W times per batch, one batched
   columnar reserve/commit per batch, a null-sink floor isolating input-side cost. Built as an explicit
   struct-of-fn-pointers call, not a scaffold `FnMut` cell.

6. **Residency and ownership.** Cells: host-lent-buffer registered/pinned once outside the timed region and
   reused across many calls, versus a fresh region handed each call. This is the direct answer to the design
   round's own open "bench-decidable if it becomes contentious" question. Likely not W-swept in the same way
   as benches 1 through 5; may stand as a fixed-size family plus a note on whether the residency choice
   interacts with W (a candidate for the composition matrix's cross rather than its own sweep).

7. **Cross-language floor.** Same shape as bench 1, but the cdylib is a genuine Zig-compiled `extern "C"`
   library rather than Rust-compiled, built against a stub dispatch body if the real runtime entry points are
   not yet available (Open Question 2), reported with the stub-fidelity caveat from Trap 6.

8. **Error-path contract.** Not a timing bench. A design note surfaced to the synthesiser: the return-struct
   shape for a partial-batch fault must be decided before benches 4 and 5 lock their entry/sink layout, because
   that shape is what the whole cost-model fit is measured against. Once decided, the winning shape's added
   cost (does carrying a per-record status vector back cost anything beyond the value output already measured)
   is a straightforward add-on sweep against bench 4/5's winner, not a new category of its own.

**The composition matrix is a curated cross, not a literal cartesian product.** The harness's own
`MatrixDecl` gives one swept axis (`SweepAxis`) plus a flat `sizes` ladder plus a flat `cells: Vec<CellDecl>`
list per family (`mockspace-bench-matrix/src/decl.rs:65-89`); it does not give a native N-dimensional sweep.
The interpreter arc's own precedent for a genuine two-axis surface, `entgrid.rs`, resolves this by flattening
a 3x3 cross into nine explicitly named cells rather than asking the macro for two independent sweeps
(`mock/benches/carrier/src/bench/entgrid.rs:1-9`). A literal cartesian product of this arc's five real axes
(boundary layout x W x entry form x payload x sink, even before residency and cross-language are folded in)
produces mostly incoherent combinations (a scalar boundary paired with a vectorized payload is not a real
point in the design space) and, even restricted to coherent combinations, a cell count in the hundreds. The
right shape, following the entgrid precedent:

- `abi_boundary_w`: sweep W, cells = {no-FFI floor, Rust/Rust `dlopen` floor, Zig cross-language floor, scalar
  entry called W times, runtime-W single entry, per-W monomorphised entry} x the winning layout/payload
  combination from benches 1 through 4, at each of the six designed program profiles. This is the headline
  family: does the batching win survive, at what W, and does the answer generalize across the six profiles or
  is it an artifact of one op mix, the same discipline the interpreter arc already applies per-profile.
- `abi_sink_w`: sweep W, cells = {per-record sink, batched sink, null sink}, at the winning boundary/entry/
  payload combination.
- `abi_residency`: cells = {host-lent-reused, fresh-per-call}, crossed against a representative W subset (not
  the full sweep) to check whether residency interacts with batch width or is genuinely orthogonal.

Reported outputs, mirroring the interpreter arc's composition-matrix shape: the cost-model decomposition
(`S + k*I`) per cell per profile; floor decomposition against the no-FFI and null-dispatch floors; an oracle
envelope over everything measured (which single boundary/entry/payload/sink combination wins at each
(profile, W) point); and a selector-regret report answering whether a single fixed ABI choice (one W, one
entry form, one sink shape) loses much regret across the six profiles and the W range, or whether the optimal
choice genuinely changes per profile, which would be a real design consequence (a runtime-selectable batch
width or entry form) rather than a benchmarking footnote.

## Open questions for the synthesiser

1. The harness's one-swept-axis-per-family constraint means the "coherent cross" this arc wants either stays
   a curated, entgrid-style flat cell list (my recommendation above) or the arc needs an upstream
   `mockspace-bench-matrix` extension (a genuine multi-axis sweep declaration) before a true cross can be
   expressed without dozens of hand-named cells per family. Is that upstream work in scope for this arc, per
   the same everything-upstreamable-goes-upstream discipline that added the `stream` regime?

2. `vehje-runtime-abi` has no `extern "C"` entry points yet (`lib.rs:52-54`'s `FIXME`), and no Zig-side
   dispatch to call into. Bench 7 (the cross-language floor) therefore needs a hand-written Zig stub, not the
   real runtime. Should that stub ship as part of this arc (with the fidelity caveat from Trap 6 stated
   plainly), or does it wait until the M3-step-3 driver bindings land and become a follow-on arc?

3. The error-path/partial-batch-fault contract (category 9) is a design decision, not a bench. Does the
   synthesiser want a design round to settle the return-struct shape before benches 4 and 5 lock their layout,
   given that shape is exactly what the whole cost-model fit is measured against? Building against an
   undecided shape risks the same claim-versus-reality drift the workspace's CL-claim discipline exists to
   catch, one layer removed (a bench's committed numbers, not a locked CL, becoming the stale claim).

4. I read the `vehje-runtime-driver` wrapper layer as out of scope for this arc (the brief's boundary is
   host-to-runtime; the driver is compile-side, a third party the brief never names) and folded it into
   Category 1 as a footnote rather than a first-class bench. Confirm that reading, or say if a
   wrapper-inlining sub-bench belongs in this arc rather than a later one.
