# Certified generation: is it buildable? (audit 4, systems and Zig-build-time lens)

**Date:** 2026-07-20
**Subject:** `202607201627_topic.compiling-the-proof-into-generated-code.md`, read after the four prior topics of
the round in chronological order, then `01_carmack.md`, `02_giesen.md`, `03_op-and-agent-response.md`, and
`04_theory-review.md` as the panel input this audit builds on and closes.
**Method:** read the five synthesis docs and the five topics in order, then the four panel documents; verified
the Zig build-time claims the two open questions turn on against current compiler behaviour and reported
numbers (comptime evaluation cost and memory, `@embedFile` alignment, comptime function-pointer prohibition,
comptime recursion overflowing the compiler's own stack, eval-branch-quota semantics) and the streaming-runtime
determinism prior art (DTVM's deterministic JIT IR). Where I reason about the streaming-plus-lease interaction
and the build-time cost curve, that is first-principles systems work over this round's own material and the
measured Zig numbers, and I say so. I checked the shipped state to keep the what-exists axis honest: the Zig
side is a 46-line extern-stub skeleton (`mock/runtime-zig/src/runtime.zig`); everything here is design.

## Verdict in one paragraph

It is buildable, the certified-generation core survives the systems lens intact, and the three prior audits'
corrections all hold at the machine level. But two of the panel's cleanest stories have measured cost cliffs the
panel handled by assertion, and both land on my desk. First, "one inference at two binding times" quietly asks
Zig's comptime interpreter to run the validation kernel over bundled *content*, and comptime is measured at
roughly twenty times slower than interpreted Python with hard memory behaviour (the canonical data point, issue
#4055: a 10,000-element comptime sort took 75 seconds and 3 GB; a nontrivial JSON parse at comptime takes
minutes). Decoding the small language *table* at comptime is cheap and the wire-blob bridge is fine; folding a
real bundled script *corpus* at comptime is where the tax bites, and the right answer is a third binding time the
panel has not named: run the identical kernel, compiled to a native `build.zig` step, over content at our build,
keeping comptime for the one thing it is uniquely good at and the table is small enough for, generating the IR
type and dispatch. Second, "streaming-centric, spill to disk" bakes filesystem I/O into what must remain an
embeddable in-process library and a per-frame engine consumer (ikiuni), which cannot tolerate disk latency mid
frame and may run sandboxed with no filesystem at all; the fix is to make "disk" one instantiation of the
host-lent reserve/commit sink the transfer topic already built, with the per-frame consumer configuring a
memory-only, overflow-is-an-error backend. The genuinely good news the panel left as "open": the streaming spine
and the depth-ladder lease inference are not in tension, they are mutually reinforcing, because the same
backward-only post-order invariant that makes the linear scan work is exactly what lets the kernel spill payload
bytes while retaining a bounded lease-frontier accumulator, and a depth cap (already owed for untrusted input)
is what keeps that accumulator fixed-size so no_alloc survives streaming. The experiment is correctly scoped for
*correctness* and unscoped for *cost and for streaming*; it needs a scaling-cost probe, a forced-spill
round-trip arm, a C-host panic-free arm, and a ReleaseFast-versus-ReleaseSafe differential axis before a doc CL
relies on any of this. None of it reopens the resolution.

## The comptime-legal kernel at build-time scale (note 1 made practical)

The panel's answer to note 1 is "achievable, a discipline not a wall" (doc 03), with the theory review adding
the six-item legality condition list. Both are right about legality. Neither has priced the *cost*, and the cost
is where note 1's actual question ("stay within a sane build time for tens of families, hundreds of node kinds")
lives. The single most important thing this audit adds: the panel has been treating one comptime workload where
there are three, with very different cost profiles, and conflating them is what makes "it fits" sound safe.

**Workload A: specialise the engine to the language table (comptime, small, mandatory, fine).** This is
comptime-decoding the language-definition blob (family table, effect masks, lease bits, wire layout) and using
it to reify the IR tag union, the name-keyed dispatch namespace, and the specialised checkers. It runs once per
(language, target) at our build. Its input is the language table: tens of families, hundreds of node kinds. That
is *small data*. Decoding a few hundred fixed-width records is low thousands of backward branches, trivial under
a raised quota. The real cost inside Workload A is not the decode, it is `@Type` union reification and the
generated dispatch: constructing a nominal type with hundreds of variants plus per-variant metadata, then
forcing a name-keyed handler namespace of the same cardinality, is where Zig comptime spends memory and time.
Production prior art exists at this scale (Mitchell Hashimoto's comptime tagged-union subset technique in
ghostty, cited by Carmack), and hundreds of variants is within reach, but "within reach" here means seconds of
build time and hundreds of MB, not free, and it compounds with the forcing root (below). This is tractable and I
judge it fine, with the caveat that it must be *measured on the real cardinality curve*, not asserted from a
two-node toy.

**Workload B: fold bundled content to compile errors (comptime, potentially large, the cliff).** Giesen's "one
inference, two binding times" has the comptime interpreter run the *whole kernel* (decode, bounds validate, mask
accumulate, depth-ladder lease inference) over first-party scripts or a test corpus bundled into the binary, so
those get their errors at our build. This is the cliff. Here the input is not the small table, it is *script
content*, and the comptime interpreter is now running an interpreter (the kernel) over real program data, a
double-interpretation tax on top of the measured ~20x-slower-than-CPython base rate. The concrete precedent is
exactly on point: parsing a nontrivial JSON file at comptime "takes several minutes" (issue #4055 discussion),
and a comptime sort of 10,000 elements hit 75 seconds and 3 GB before tripping the branch limit. A bundled
corpus of a few hundred small scripts might survive in tens of seconds; a real corpus will blow both build time
and compiler memory, and it does so *superlinearly*. The panel's "achievable" is true of legality and false of
cost for this workload at scale.

**Workload C: check arriving scripts (shipped binary, runtime, the actual product).** The consumer-visible path.
Not comptime at all. No cost objection: it is a linear scan in native code (Giesen's post-order finding). This
is the path the whole design exists to serve, and it is fine.

The correction that resolves note 1: **decouple the workloads and give Workload B a third binding time.** Keep
Workload A at comptime (it must be: you cannot generate a Zig *type* anywhere but comptime, and the table is
small enough). Move Workload B *off* comptime onto a native `build.zig` step: compile the identical kernel
source to an ordinary build executable and run it over the bundled corpus at our build. This is deterministic,
runs at native speed instead of interpreter speed, has no comptime memory ceiling, and is the exact lesson the
purity synthesis already records from .NET's `[DllImport]` to `[LibraryImport]` shift, "resolve the boundary
shape ahead of time into inspectable static code; runtime codegen of the boundary was a mistake paid for in
opacity and cost." Content validation is boundary work; comptime is the runtime codegen of it; the compiled
build step is the source generator. The comptime-equals-runtime equivalence the theory review nails down is
still needed, but only for Workload A's small table decode (cheap to differential-test) and for the kernel logic
shared with Workload C. Workload B's native build step is the *same kernel source* compiled by Zig for the build
host, and because the kernel is fixed-width-integer and byte-shift throughout (theory review conditions 1, 3, 4)
it is target-agnostic, so its result agrees with the shipped binary's. This is the crossover Carmack's
generated-consts fallback was groping toward, stated precisely: comptime for type generation from the small
table (its sweet spot), a compiled build step for content validation (dodges the interpreter tax entirely),
native at the consumer. It is strictly better than "everything at comptime," and I do not think the panel has
said it.

**Three build-time hazards the panel underweights, all real, all cheap to state now.**

*The eval-branch-quota is a stack hazard, not just a budget.* The default 1000 is trivially blown and
`@setEvalBranchQuota` ratchets it up from the top of the governing comptime stack (synthesis note). The trap the
panel misses: Zig's self-hosted compiler uses its *own* stack for comptime function calls, so deep comptime
*recursion* overflows the compiler process rather than producing a clean quota error (issue #13724). Therefore
the kernel must be written *iteratively*, an explicit work-stack over the flat post-order arena, never
recursively. This is not optional style: a recursive kernel that works on shallow toy input will segfault the
compiler on a deeply-nested real input at comptime. Giesen's linear-scan-over-post-order finding is usually sold
as a cache-locality win; its load-bearing role is actually this, keeping the comptime evaluation off the
compiler's call stack. The design should mandate the iterative form for any dual-binding-time code.

*`@embedFile` overlay decode is comptime-fragile; byte-shift is the safe form, and it costs quota.* The theory
review flagged the alignment-1 hazard; I confirm and sharpen the cost consequence. `@embedFile` yields bytes of
alignment 1, and a comptime load whose bits include undefined padding is wholly undefined at comptime (Zig
type-punning rules). So the robust dual-binding decoder is explicit byte-shift-and-or per field, not
`@bitCast`/overlay. Byte-shift decode is *more* backward branches than an overlay read, so it consumes more
quota and more comptime time. For Workload A's small table this is still cheap. It is one more reason Workload B
does not belong at comptime.

*Function-pointer dispatch is forbidden at comptime, which forces the kernel allocation-free.* Confirmed:
`std.mem.Allocator` is a vtable (a runtime function-pointer table), and comptime cannot call through a runtime
function pointer. Doc 03's "allocator-parametrise the decoder" cannot mean passing a `std.mem.Allocator`; the
theory review says so and is right. The clean resolution, which the no_alloc discipline wants anyway: the
dual-binding kernel takes a caller-supplied `[]u8` scratch buffer and a `[]u8` output slice and no allocator at
all. That is comptime-legal, runtime-legal, and no_alloc by construction. State it as the kernel's signature
rule, not as an allocator-parametrisation.

**The forced-instantiation root scales linearly and stacks with A.** Giesen's owned comptime forcing walk (not
the churning std helper) forces Zig to type-check every generated decl, which for hundreds of node kinds means
type-checking hundreds of dispatch handlers plus the specialised engine per (language, target) pair. That is the
second certification actually reaching the code, so it is cost you *want*, and it is linear in the generated
surface. But it stacks with Workload A's reification cost in the same build, so the real build-time budget is
(table decode + type reification + forced instantiation) per (language, target), and the scaling probe must
measure the *sum* with forcing on, not the decode alone.

**Verdict on note 1.** The dual-binding kernel is comptime-*legal* on the theory review's six-item subset, and
its iterative-plus-byte-shift-plus-no-allocator form is buildable. The dual-binding kernel is comptime-*cheap*
only for the small language table (Workload A). Folding bundled content at comptime (Workload B) hits a measured,
superlinear cost cliff and should move to a native `build.zig` step. With that split, note 1 resolves to yes:
comptime stays within a sane build time because the expensive workload is no longer on it. Without the split,
"it fits" is an untested optimistic lean against measured evidence that it will not, at exactly the scale note 1
names. Carmack's generated-consts-plus-re-checks is the correct fallback *for the bridge* only if comptime
decode of the table disappoints, which I judge unlikely; the real fallback that matters is the build-step move
for content, and it should be the default, not a fallback.

## The streaming-centric, staged, disk-spill runtime (note 2 made concrete)

Op's instinct is correct: build the runtime streaming-centric from the start, the way JIT-shaped runtimes are,
rather than retrofitting it. The unification (the reserve/commit value sink, the AOT/JIT cache, and staged macro
expansion all becoming one spill-capable spine) is genuinely elegant and it is how a real staged runtime is
structured. But "spill to disk" as stated conflates two latency regimes and bakes an embeddability hazard, and
the determinism story has concrete filesystem traps the panel has not named. The mechanism is right; three
refinements make it actually buildable across the census.

**Refinement 1: "disk" is one instantiation of the host-lent sink, never baked I/O.** The transfer topic already
established that output memory is host-lent through a reserve/commit sink (a `#[repr(C)]` pair of function
pointers plus opaque userdata in-process; the stdout pipe out-of-process). The spill spine must reuse *exactly
that shape*: the spill backend is a host-supplied reserve/commit sink pointed at storage instead of at the
output buffer. This is not a nicety, it is a correctness requirement for two consumers. An in-process embeddable
library that calls `open()`/`write()` itself is unusable in a sandboxed host (no filesystem, seccomp) and it
violates the spirit of `vehje-runtime-abi`'s own forbidden-imports lint (`std::fs::*` banned there). And the
per-frame consumer, ikiuni's scripting language, *cannot* spill to disk mid-frame: even an SSD write is hundreds
of microseconds to milliseconds and non-deterministic, which blows a 16 ms frame budget and breaks ikiuni's
determinism requirement. With spill as a host-lent sink, ikiuni supplies a memory-only backend with a hard cap
and "budget exhausted is a graceful error"; a batch consumer supplies a disk-backed one; a sandboxed host
supplies an in-memory ring. One mechanism, host-selected policy. The streaming-centric-from-the-start decision is
right *because* it makes both fall out of one seam, but only if the seam is the sink abstraction and not a baked
path. This also means the "AOT/JIT cache" and "within-run macro staging" unification is honest only as one
*mechanism* with two *policies*: the AOT cache is a persistent disk-backed sink reused across runs (tolerates
disk latency, it is between-run), the within-run macro staging is a transient sink that stays in memory unless a
genuinely oversized expansion forces overflow. JITs cache compiled artifacts to disk *between* runs; they do not
spill hot state to disk *within* a run (kipp.ly, "JITs are not very just-in-time"). Keeping these as one
mechanism is fine; pretending they share a latency budget is not.

**Refinement 2: determinism has concrete filesystem traps, and content-addressing is the discipline.** Op's
requirements (reproducible spill and re-read, no ambient clock, stable ordering) are achievable, and DTVM (the
deterministic WebAssembly JIT, arXiv 2504.16552) is direct prior art that a JIT-shaped runtime *can* be made
deterministic, at the cost of a deliberately deterministic middle IR. The traps that must be closed by
construction: spill artifacts must be named and ordered by explicit (stage, chunk-index) or by content hash,
never by timestamp, PID, mtime, or `O_TMPFILE` random name, because any of those leaks nondeterminism into
reassembly order; and reassembly must index chunks explicitly and must *never* enumerate a spill directory,
because `readdir` order is filesystem-dependent and not reproducible. This is the same content-addressed
discipline the purity synthesis records for Dhall's semantic hash, applied to spill. It is cheap if designed in
and a silent reproducibility bug if bolted on. Disk I/O also introduces `ENOSPC`, short writes, and `EINTR`,
which are runtime error conditions that must be error-union'd and panic-free, the same invariant the untrusted
path already owes; a spill that fails must be a graceful error, never a panic across the C ABI.

**Refinement 3, the positive finding: streaming and the lease inference are compatible, and mutually
reinforcing, via the shared backward-only post-order invariant.** The panel (doc 03 open item 2, theory review
open item 6) left "does spilling and re-reading preserve the post-order interval structure the linear-scan lease
inference needs" as an open worry. Worked through, it is not a worry, it is the design's best structural
coincidence, and it should be claimed on purpose. The transfer topic mandates children-before-parents,
backward-link-only, no-back-patching emission, so the arena is a post-order traversal in memory with every link
pointing backward. That has three consequences that make streaming and lease inference fit each other exactly:

- *The lease inference is a single forward pass with a bounded frontier, so it never needs to re-read spilled
  bytes.* To compute a value's lease (the min depth over its linking uses), you carry a per-value accumulator and
  fold each later use into it as you scan forward. The set of values that can still *receive* a new linking use
  is bounded: in post-order with backward-only links, once you pass a binder its body's locals are inside a
  closed interval and nothing after can link them. So the live accumulator set is bounded by the open-body
  nesting frontier, not by total value count. The kernel spills the *payload bytes* (large) to the host-lent sink
  and retains only the small frontier accumulators (bounded). This is exactly how a streaming compiler keeps a
  symbol table resident while spilling bulk AST, and it means lease inference runs *during* emission and re-read,
  never as a post-hoc random-access scan over reconstituted bytes. The interval structure the scan needs is the
  live frontier, which is resident by construction.
- *Indices must be global-monotonic, not window-relative, and then spill/re-read is transparent to the scan.* A
  spilled node keeps its global index; a backward link uses the global index; re-read maps global index to
  (chunk, offset). This is precisely the transfer topic's (chunk, index) cross-chunk pair. With global indices
  the post-order structure is preserved across any spill boundary, because the structure is in the index order,
  not in physical contiguity.
- *A depth cap is what makes the frontier accumulator fixed-size, so no_alloc survives streaming.* The frontier
  is bounded by open-body *nesting depth*, and nesting depth is attacker-controllable on the untrusted path (a
  script with a million nested `Let`s). The traversal depth limit the untrusted path already owes (the Cap'n
  Proto 64-deep pointer-limit analogue in the value-transfer synthesis) is exactly what bounds the frontier to a
  fixed size. So the depth cap is not only an untrusted-input safety measure; it is what makes the lease-frontier
  accumulator a fixed buffer, hence what makes no_alloc hold under streaming. Another decision made for one
  reason (untrusted-input safety) paying for a second property (no_alloc under streaming). Without the cap, a
  deeply-nested input exhausts the fixed frontier buffer and must be a graceful error, not an OOM.

So no_alloc genuinely survives: fixed window for payloads, host-lent sink as the unbounded accumulator, fixed
depth-capped frontier for the lease pass. The unboundedness lives on the host's storage, never in a growing
allocation, exactly as op said.

**The one place the interaction is genuinely unproven and on the critical path: the cross-chunk lease lemma.**
Giesen flagged and theory review reiterated that "a cross-chunk reference is by construction a consumed-or
promoted value, because chunk boundaries are whole-subtree boundaries" is an expectation, not a proof. From the
systems side this is not a proof footnote, it is the load-bearing invariant that makes streaming and the
linear-scan lease inference compatible *at all*. If a cross-chunk backward ref could be an ordinary
inner-body local (not consumed, not promoted), then re-reading its chunk would be required to update its lease,
which defeats the bounded-window point and forces either retaining all spilled chunks or a random-access
re-read, either of which collapses the streaming benefit. The lemma holds if and only if chunk boundaries are
strictly whole-subtree boundaries *and* a subtree's escaping references are exactly its result value (the theory
review's crux immutability lemma). So the cross-chunk lemma and the crux immutability lemma are the same lemma
viewed at chunk granularity, and it must be proven (or the chunker constrained so it holds by construction)
*before* committing to the streaming spine, because streaming feasibility depends on it. This is the single
hardest open item in note 2 and it should block, not trail, the doc CL.

**Cost for ikiuni versus batch consumers.** For the per-frame consumer, the streaming spine degenerates to the
whole-value, memory-only case: window sized to hold the small per-frame script's working set, spill disabled,
overflow is an error, zero disk latency, deterministic. For batch consumers, the disk-backed sink handles
arbitrarily large outputs at the cost of I/O latency they can absorb. The complexity cost is the sink
abstraction and the global-index discipline, both of which the transfer topic already pays for other reasons.
The latency cost is zero for the per-frame path *if and only if* spill is a policy and not baked; that
conditional is the whole reason refinement 1 is a correctness requirement rather than a preference.

**Verdict on note 2.** The streaming-centric spine is the right foundation, and it is buildable. Spill must be a
host-lent sink policy (refinement 1), determinism must be content-addressed with no `readdir` reassembly
(refinement 2), and the streaming/lease compatibility is real and positive but rests on the cross-chunk lemma,
which is on the critical path and unproven (refinement 3). no_alloc survives, contingent on the depth cap being a
first-class runtime limit.

## Determinism and the lease post-order interaction

Consolidating the determinism thread across both notes, because it spans them and the panel treated it
piecewise. Three binding times now exist (comptime type-generation, native build-step content validation,
shipped-binary load), and a spill spine sits under the runtime. Determinism has to hold across all of it.

The comptime and build-step binding times are deterministic by construction for the reasons the theory review
gives: comptime is hermetic and target-aware (no host leakage, no clock, no I/O), and the native build step over
content is a pure function of its input bytes if it takes no ambient input. The one seam is that the build step
runs on the build *host* while the shipped binary runs on the *target*; the kernel's fixed-width-integer,
byte-shift, no-float form (theory review conditions) makes it target-agnostic, so build-host and target agree,
but this must be a stated kernel rule and covered by the differential gate, not assumed.

The runtime spill spine is where determinism is *earned*, not free, and DTVM is the evidence it is earnable: a
deterministic JIT exists, and it required a deliberately deterministic middle IR to get there. For vehje the
requirements are the content-addressed spill naming and explicit-index reassembly of refinement 2, plus: the
spill round trip must preserve the post-order interval structure the lease scan needs, which refinement 3 shows
it does *if* indices are global-monotonic and chunk boundaries are whole-subtree (so a spilled-then-reloaded
arena is bit-identical in index structure to the never-spilled one), and the spill read-back re-runs bounds
validation because spilled bytes read back from a host-supplied sink are untrusted input again (theory review
open item 6, which I endorse as the cheaper of its two options: re-validate on read-back, linear, over
declaring the spill file trusted-local). The post-order interaction is therefore sound under two conditions,
both of which the design can commit to: global indices and re-validate-on-read-back. State both; they are the
determinism-plus-safety contract of the spill spine.

## Overall implementability and the experiment

The certified-generation resolution is buildable and survives four lenses. The framework identity holds: no
rustc, no LLVM, no typestate at the consumer, one pre-compiled per-target binary, and Zig's first-class
cross-compilation (Carmack's unclaimed win) directly serves "one static binary per target," which is worth
claiming. The corrections stack cleanly and none is a redesign.

The experiment as widened by Giesen (four negative arms plus forced-instantiation root) and by the theory review
(differential-testing gate) is correctly scoped for *correctness* and *unscoped for cost and for streaming*. It
would pass on a two-node toy while testing none of the things that actually decide buildability at scale. Four
additions, in priority order:

1. **A scaling-cost probe, which is the direct instrument for note 1.** Run Workload A (table decode plus `@Type`
   reification plus forced instantiation) at three cardinalities, roughly (2 families, 4 kinds), (10, 50), (30,
   300), and measure comptime wall time and compiler peak memory at each. This is the only thing that answers "is
   the build time sane at tens of families, hundreds of node kinds," and given the measured comptime cost curve
   (75s/3GB for a 10k sort) the shape of this curve, not a single toy point, is what the doc CL must be able to
   cite. Additionally run Workload B (fold a bundled corpus) at comptime versus as a `build.zig` step at growing
   corpus size, to confirm the build-step move is the cheaper default and to find where comptime B becomes
   intolerable.
2. **A forced-spill round-trip arm, which is the direct instrument for note 2.** Emit a value larger than the
   window, force one spill through a memory-backed host sink, re-read, and confirm: global indices stay stable, a
   cross-chunk backward ref resolves, the promoted lease is assigned, the lease-frontier accumulator stayed
   bounded and resident across the spill, and re-read re-validation catches a corrupted spilled chunk. This
   exercises the cross-chunk lemma rather than only proving it on paper, which is the run-the-experiment rule
   applied to the one interaction nobody has walked.
3. **A C-host panic-free arm.** Drive the untrusted path from an actual `extern` C (or Rust-FFI) caller with the
   malicious blob and confirm the process returns an error and does *not* abort. The panic-does-not-unwind-across
   -the-C-ABI failure is invisible from inside a Zig test; it only shows at the real boundary, and it is the
   embeddability-correctness property the whole panel has flagged. This arm is small and it is the only one that
   actually tests the invariant.
4. **A ReleaseFast-versus-ReleaseSafe axis on the differential gate.** The theory review's overflow point means
   the differential test must run the runtime side in *both* modes and confirm the kernel's checked-arithmetic
   error-union results agree with the comptime results in both, because the shipped binary is ReleaseFast and
   that is where undefined overflow would hide. Running the differential gate only in Debug or ReleaseSafe would
   pass while the shipped path diverges.

The iterative-kernel requirement (no comptime recursion, issue #13724) also wants a deep-input arm confirming a
clean error rather than a compiler stack overflow, but that folds into the scaling probe's deep cases.

## Where it fails or costs more than stated

- **Comptime build-time cost at content scale is a measured cliff the panel called a discipline.** "One
  inference at two binding times" over bundled content is Workload B, and the comptime numbers (20x slower than
  CPython, JSON-parse in minutes, 10k-sort 75s/3GB) say it does not stay in budget at real corpus size. The fix
  (native build-step for content) works, but it is a third build-time locus and a piece of load-bearing
  `build.zig` infrastructure that itself must be deterministic and pinned, which the crate-taxonomy debt has not
  accounted for.
- **Spill-to-disk in an embeddable in-process library is an embeddability hazard as literally stated.** Baked
  filesystem I/O breaks sandboxed hosts and the per-frame consumer. It costs a host-lent-sink seam to fix, which
  is cheap, but the design as written in op note 2 ("serialise back and forth from and to the disk") would ship
  the hazard if taken literally.
- **no_alloc under streaming holds only with a hard depth cap.** Without it, deep nesting exhausts the fixed
  lease-frontier buffer. The cap is owed anyway for untrusted input, but the design must state that no_alloc
  *depends* on it, so it is not later relaxed as "just a safety limit."
- **The cross-chunk lease lemma is on the critical path for streaming, not a proof footnote.** If it fails,
  streaming and linear-scan lease inference are incompatible and the bounded-window benefit collapses. It must be
  proven or designed-in before the streaming spine is committed.
- **Zig std churn makes a `std.Io`-based streaming spine a moving target.** The 0.15 writer overhaul and the 0.16
  `std.Io` and type-resolution redesigns (Giesen) mean a spine built on `std.Io` will churn across pins harder
  than rustc nightly does for us. Pin exactly (0.16.0 is the natural candidate) and minimise or wrap the
  `std.Io` surface the spine touches, so a pin bump is a contained change.
- **The forcing root plus reification plus decode stack in one build.** The per-(language, target) build-time
  budget is their sum, not any one, and it multiplies by the number of shipped (language, target) pairs. The
  scaling probe must measure the sum, and the distribution story ("one binary per target") should note that each
  target pair pays this build cost.

## Open questions for op

1. **Bless moving bundled-content validation off comptime onto a native `build.zig` step**, keeping comptime for
   the small-table type/dispatch generation only? This is the concrete resolution of note 1's build-time-budget
   question, and it changes the "two binding times" framing to "one kernel source, three loci" (comptime type
   generation from the table, native build step over content, shipped binary at load).
2. **Bless spill as a host-lent reserve/commit sink policy** rather than baked filesystem I/O, with the per-frame
   consumer configuring a memory-only, overflow-is-an-error backend and batch consumers configuring disk? This is
   a correctness requirement for ikiuni and sandboxed hosts, not a preference.
3. **Accept a hard traversal depth cap as a first-class runtime limit** that no_alloc-under-streaming depends on,
   not merely an untrusted-input safety measure?
4. **Elevate the cross-chunk lease lemma to a build-blocking proof plus experimental arm**, since streaming
   feasibility depends on it, rather than leaving it in the trailing proof document?
5. **Pin Zig exactly and constrain the spine's `std.Io` surface** given the measured std churn, in the same
   workspace rule that pins rustc?

## Bottom line

Buildable, and the certified-generation core is sound at the machine level. The prior three audits' corrections
all hold. The two questions handed to this lens each have a real cost cliff the panel handled by assertion, and
each has a clean fix. Note 1: the dual-binding kernel is comptime-legal and comptime-cheap only for the small
language table; folding bundled content at comptime hits a measured superlinear cost, so move content validation
to a native `build.zig` step running the identical kernel, and keep comptime for the type generation it is
uniquely required for and the small table makes affordable. Note 2: the streaming-centric spine is the right
foundation, but "disk" must be a host-lent sink policy so the per-frame and sandboxed consumers survive,
determinism must be content-addressed with explicit-index reassembly, and no_alloc survives under a hard depth
cap; the genuinely good news is that streaming and the depth-ladder lease inference reinforce each other through
the backward-only post-order invariant, with the lease pass spilling payloads while retaining a bounded frontier,
which the panel left as an open worry and is actually the design's best structural coincidence, contingent only
on the cross-chunk lemma that is now on the critical path. Widen the experiment with a scaling-cost probe, a
forced-spill round-trip, a C-host panic-free arm, and a ReleaseFast differential axis before the doc CL; those
four test the parts that decide buildability at scale, which the correctness arms do not. None of this reopens
the resolution; all of it is precision about which cost lives on which machine at which binding time.

## Sources

- Zig comptime cost and memory: ziglang/zig issue #4055 ("improve comptime performance... same as CPython
  execution speed"), the 10,000-element comptime sort at 75 seconds and 3 GB, and the nontrivial-JSON-parse-in
  -minutes report. <https://github.com/ziglang/zig/issues/4055>
- Zig comptime recursion overflows the compiler's own stack: ziglang/zig issue #13724 (self-hosted compiler uses
  its own stack for comptime function calls). <https://github.com/ziglang/zig/issues/13724>
- Zig comptime memory-management constraints: ziglang/zig issue #5895 (comptime memory management reform) and
  #5881 (no dynamic allocation at comptime). <https://github.com/ziglang/zig/issues/5895>
- `@embedFile` alignment-1 and comptime binary decode practice: nathancraddock, "Writing a struct deserializer
  with Zig metaprogramming"; Zig packed-struct and `align(1)` docs.
  <https://nathancraddock.com/blog/deserialization-with-zig-metaprogramming/>
- Comptime cannot call through a runtime function-pointer vtable; `std.mem.Allocator` is vtable-dispatched:
  ziglang/zig issue #1291 (comptime allocator) and #1268 (comptime interfaces); the `@inComptime()` branch
  pattern. <https://github.com/ziglang/zig/issues/1291>
- `@setEvalBranchQuota` semantics (default 1000 backward branches, ratchets upward, set from the top of the
  governing comptime stack): Zig language reference on comptime.
- Deterministic JIT prior art (deterministic middle IR): "DTVM: Revolutionizing Smart Contract Execution with
  Determinism and Compatibility," arXiv:2504.16552. <https://arxiv.org/pdf/2504.16552>
- JIT runtimes cache compiled artifacts between runs rather than spilling hot state to disk within a run:
  kipp.ly, "A Deep Introduction to JIT Compilers: JITs are not very Just-in-time."
  <https://kipp.ly/p/jits-intro>
