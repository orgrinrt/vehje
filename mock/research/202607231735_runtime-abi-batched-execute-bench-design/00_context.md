# Context: designing the runtime C ABI batched-execute bench arc

**Date:** 2026-07-23
**What this is:** the shared brief for a small set of independent design forks. The task is to design a bench
arc that measures the vehje runtime C ABI's batched/column invocation boundary, in the same two-stage shape the
interpreter-composition arc used: isolated individual benches first, then a composition matrix built on what the
individuals teach. The forks read this brief, then each writes its own design file in this directory.

This brief is factual. It states the boundary, the question, the evidence that bears on it, the machinery already
in the tree to reuse, and one candidate decomposition to react to. It does not state which shapes will win or
which decomposition is correct; that is the forks' job to work out and, where they disagree with the candidate,
to say so.

## The artifact under test: the host to runtime C ABI

vehje is a two-artifact framework. A dev-time Rust language compiler proves a language definition sound and emits
validated data; a shipped Zig composed runtime, specialised to that data, runs every script. The **runtime C
ABI** is the boundary between an embedding **host** and the composed **runtime**. It is the subject here.

The current open design round (`mock/design_rounds/2026072*_topic.*`, TOPIC phase, newest wins on conflict)
re-voiced this boundary away from an older program-centric shape. The load-bearing facts, from the round:

- **In**: a script parsed inside the runtime, or a bundled tier-0 IR arena (the binding-time-neutral image). The
  `Tier` tag narrows to the internal execution form (interpret-arena vs run-bytecode). "Native" is not a tier; it
  is one execution strategy chosen per region by proof-directed strategy selection.
- **Out**: the produced value crosses as a region-structured **value-arena** through a **reserve(hint) /
  commit(n) two-function sink** (in-process: a `#[repr(C)]` struct of two function pointers plus opaque userdata;
  out-of-process: the exe's stdout pipe), streaming with backpressure, with a typed structural decode on the
  untrusted path.
- **Execution** is a bounded strategy portfolio: interpret the tier-0 arena, tier hot regions up to
  copy-and-patch native where it provably wins.

The current shipping `mock/crates/vehje-runtime-abi/src/lib.rs` is minimal: a `Tier` enum and a `Residual { tier }`
descriptor with a `FIXME` that the runtime-environment interface descriptor and the `extern "C"` entry points land
later. So the invocation entry points do not exist in source yet. This arc benches *proposed* entry-point shapes so
the ABI decision is evidence-backed before it is committed.

## The question the arc must answer

vehje's real runtime workload is **per-record column evaluation**: one compiled unit (residual) run over a column
of many records. The one non-deferrable ABI decision is whether, and in what shape, the host-to-runtime boundary
exposes a **batched/column entry** (feed W records in one call, get W outputs), versus a scalar per-record entry.

Why it is load-bearing: the largest single performance result the prior arc measured is vertical / SoA SIMD
interpretation, which amortises one dispatch over a vector of W inputs and maps directly onto column evaluation.
Vectorising across records requires the runtime to receive a column of records in one call; a boundary crossed
once per record forecloses it structurally. So the batched-arity ABI is the enabler of the biggest measured win,
and the arc must measure whether that win survives the real boundary, at what batch width it pays, and in what
entry-point shape.

## Evidence from the interpreter-composition arc that bears on this

From `mock/design_rounds/202607211347_topic.bench-and-sketch-evidence-synthesis.md` and the composition-matrix
arc (all on this branch, cross-validated, PMU-timed, cdylib-isolated):

- **Vertical / SoA SIMD interpretation is the standout win**: one dispatch over W inputs, per-input dispatch cost
  1/W of scalar. Cross-validated per lane. It is `interpret_vertical<const W>` in `mock/benches/carrier/src/vertical.rs`,
  already const-generic over W, taking a `[u64; W]` column of seeds and filling SoA `[Simd<u64, W>]` scratch.
- Plain `switch` is the fastest scalar dispatch for vehje's ~25-op vocabulary (tail-threading demoted).
- The node record is 24 bytes / 3 inline operands; pool indirection is the expensive access shape.
- The composition arc's measurement backbone: model each composition as a cost line `total(k) = S + k * I` (fixed
  setup S, marginal per-item I), swept over a geometric k-ladder with an R^2 fit; decompose every slope against a
  native-ceiling floor and a null-dispatch floor; measure a warm regime (one program, many evals) and a cold-many
  regime (many distinct programs); report native-normalised per designed program profile.
- The fairness discipline: one shared unchecked operand primitive so an axis varies only its own dimension; every
  dispatch label ISA-confirmed by disassembly; per-cell cdylib isolation (`lto=fat`, `codegen-units=1`, subprocess
  dlopen) so the optimizer cannot partial-evaluate across the boundary; byte-exact cross-validation before any
  timing is trusted; floors exempt from cross-validation.

## Machinery already in the tree to reuse (do not rebuild)

- **`mock/benches/carrier/`**: the shared IR, the generators (six designed program profiles P_real / P_madd /
  P_tight / P_scatter / P_wideselect / P_leaf), the scalar interpreters, and `interpret_vertical<const W>` (the SoA
  payload). Cells are typed `pub fn`s called by path from variant cdylibs.
- **`mockspace-bench-matrix`** (merged to mockspace dev, rev `688db96`): the `bench_matrix!` macro and the
  `scaffold::{warm, cold_cycle, stream}` regimes, the `MatrixDecl` data, `generate_all`, and the cost-model /
  floor / regime harness features. The carrier matrix was just migrated onto it (the dogfood). New regimes or
  scaffold shapes that this arc needs go upstream into `mockspace-bench-matrix`, per the everything-upstream
  discipline, the same way the `stream` regime was added.
- **`mockspace-bench-harness`**: cdylib variant isolation, cross-validation, the cost-model regression fit, floor
  decomposition, CSV plus `.bench_history` trail, PMU columns (perf-counters feature).

The harness's per-variant cdylib-plus-dlopen isolation is itself a real C-ABI cross. A candidate realisation of
the boundary under test is a two-object shape: a "runtime" cdylib exposing the `extern "C"` entry points behind the
payload interpreter, and a "host" variant that dlopens it and drives batched calls in the timed region, so each
batch crossing is a genuine cross-object C-ABI call the optimizer cannot inline away. Whether that is the right
realisation, or whether the harness runner should drive the crossings directly, is a design question for the forks.

## One candidate decomposition to react to (not the answer)

Offered so the forks have a concrete target to extend, sharpen, or replace. The axes of the ABI design space, as
one reading:

- **Boundary form**: scalar per-record call; batched-scalar-invocation (N/W calls, W records each, runtime loops
  internally, scalar interp per record); column-SoA (N/W calls, W records as SoA columns, runtime vectorises).
- **Batch width W**: the sweep axis (1, 2, 4, 8, 16, 32, 64, ...); W=1 is the scalar baseline; the curve gives both
  the FFI-call amortisation knee and the SoA vectorisation knee.
- **Entry-point form**: single scalar entry; runtime-W entry (one `extern "C"` fn, W passed as an argument, internal
  loop); per-W monomorphised entries (`execute_w4` / `execute_w8` / ..., concrete `extern "C"` per W, each
  unrollable/vectorisable). This is the "generic `execute<const W>` monomorphised and mapped at the C boundary"
  vehicle: C has no generics, so the concrete lowering is the per-W entry set or a runtime-W-with-max.
- **Payload interp**: scalar interp; vertical / SoA interp (vert4 / vert8), the latter meaningful only for the
  column-SoA boundary.
- **Output-side sink shape**: per-record reserve/commit (W cycles per batch); batched columnar reserve/commit (one
  reserve for the whole batch's W outputs); a null-sink floor (no output, to isolate input-side cost).
- **Floors**: an in-process no-FFI call (the boundary-free floor); the native ceiling; null-dispatch.

The candidate individual benches (each varies one axis, others pinned): (1) boundary amortisation over W; (2)
column-SoA win across the boundary; (3) entry-point form (the monomorphisation-versus-runtime-W vehicle); (4)
output-sink shape under batching. The candidate composition matrix (built after the individuals): the coherent
cross of boundary form x W x entry form x payload x sink, over the designed program profiles and warm/cold regimes,
with the cost-model lines, floor decomposition, oracle envelope, and a selector, the same shape as the interpreter
composition matrix.

## The question for each fork

Design the ABI batched-execute bench arc through your lens. Concretely: enumerate the categories of ABI-boundary
cost question this arc must cover (name whole categories the candidate decomposition misses, if any, rather than
permuting the axes already listed); state the measurement backbone and the floors that make the numbers legible;
name the fairness traps and confounds specific to an FFI-boundary bench (a shape that measures something other than
the boundary cost it claims); and decompose the arc into isolated individual benches first and then a composition
matrix, saying where you agree with the candidate decomposition and where you would change it and why. Deliver by
writing your design to `<persona_slug>_<goal>.md` in this directory.
