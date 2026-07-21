# Response to audits 1 and 2 (op and agent)

**Date:** 2026-07-20
**Position in the panel:** written after `01_carmack.md` and `02_giesen.md`, before the next experts. This is
op's and the agent's response to the two audits, capturing two op notes and the agent's answers as close to
verbatim as possible so the next experts inherit the full detail. It is required reading for the subsequent
audits, alongside the two prior audits and the shared reading material.

Both audits are accepted in their verified core. The panel converged on the corrections both experts confirmed
(certification accounting: rustc types data not a generator; Zig lazy analysis gates the second certification,
so an owned comptime forcing root is mandatory; allocator-free signatures are convention not guarantee;
built-in safety checks are a host DoS across the C ABI, so the untrusted path must be panic-free by
construction; the load rejection is a derived total check, not absent; pin Zig exactly; the negative test is
the real experiment). Giesen's two overturns of Carmack are also accepted: the lease temporal half is one
proven inference at two binding times (not certified-for-dev-content plus hand-authored-for-scripts, because
the Rust side never sees content), and the MLKit cautionary weight is reduced once vehje's per-script,
host-lent-budget, torn-down execution model is priced in. Giesen's depth-ladder lease schema, the
generation-checked refs as an armed verification harness, the arena-class handle re-scope, the name-keyed
dispatch, the linear-scan inference from post-order emission, and the wire-format-as-bridge are all taken as
the current best design, subject to the two open theory questions below and to op's blessing of the schema.
The remainder of this document is the two new notes and their answers.

## Op note 1 (near-verbatim): the comptime timing and phase distinction

> There's one distinction we have to make here too. We don't ship zig *compiler*, so the zig compilation
> happens at the same time as the rust typestate compilation. Not literally; the rust compiles first because it
> might and probably will emit zig fragments and/or data needed at comptime. But still; neither has access to
> any specific program or script at that time. If zig comptime is distinct from general const time, then that
> might or will be a problem.

## Agent answer to note 1

Not a false worry, but it splits into a part that dissolves and a part that is real and load-bearing.

**The terminological half dissolves: Zig has one comptime phase.** There is no "zig comptime" distinct from a
"general const time." Zig deliberately unifies what C++ splits across `constexpr`, `consteval`, and template
instantiation into a single `comptime` phase: `const` bindings whose right-hand side is comptime-evaluable,
`comptime` blocks, `comptime` parameters, and comptime type and code generation are all the same evaluation
phase, the same interpreter, the same rules. So if the worry is "these are two different mechanisms that might
not line up," that is a C++-shaped intuition that does not transfer; in Zig it is one phase.

**The real half: comptime-execution-equals-runtime-execution is a constraint, not a given.** This is the seam
under Giesen's "one decoder, two binding times" move. That move works only if running the shared decoder,
validator, and checker code at comptime (our build) computes the same thing as running it at the consumer's
runtime. Zig is designed for that equivalence, comptime runs the same source in a checked interpreter, but the
equivalence holds only for the comptime-legal subset: pure computation, no syscalls or runtime I/O, no
OS-backed allocator, no `volatile`, no inline asm, and within `@setEvalBranchQuota`. A decoder that allocates
has to be allocator-parametrised so that at comptime it is handed a comptime buffer and at runtime the caller's
arena; the logic is identical, only the allocator provider differs. So the constraint is concrete and statable:
any code that must run at both binding times (decode, bounds validation, mask accumulation, lease inference)
must be written in the comptime-legal subset. It is achievable, a discipline not a wall, but it is a real
constraint the design must commit to and the topic does not yet state.

**The reassurance that resolves "no script at that time."** Comptime never needs to see a script, and the
design never asks it to. At our build time comptime has the language data (Rust emitted it) and nothing else,
and that is all it needs, because comptime's job is to bake the certified checker, not to check any script. The
checker it produces then runs on scripts at the consumer's runtime. So "neither has access to any specific
program at that time" is true and fine: comptime specialises the engine to the language and emits the
checker-as-code; scripts are checked later, at load, by that baked checker. The only place comptime folds a
check to a compile error is for first-party content bundled into the binary at our build; a consumer's scripts
always take the runtime path (this is Carmack's scoping note, and note 1 is its deeper version: the reliance on
comptime is fine precisely because comptime is baking machinery, not evaluating scripts).

**What goes to the next expert.** It is a real constraint (the comptime-legal-subset discipline on the
dual-binding-time code, plus the eval-branch-quota budget for a realistic language table), not a blocker. The
genuine open question is whether the whole set of dual-binding-time code (decode plus validation plus mask plus
the depth-ladder lease inference) can all live in the comptime-legal subset and stay inside a sane build-time
quota for tens of families and hundreds of node kinds. That is real Zig-build-time theory to bundle, and it is
the same open question as Giesen's "wire-blob bridge versus generated consts," now with the equivalence and
legality constraint attached: if the dual-binding kernel does not fit the subset or blows the quota, the
fallback is generated consts plus systematic comptime re-checks (Carmack's original position), and the choice
is benchable.

## Op note 2 (near-verbatim): macros staged, sequential, disk-backed, streaming-centric

> As for the macro commentary: I think if we do it staged like rust, and sequential, it might dissolve this
> problem. And if we actively use and serialise and deserialise back and forth from and to the disk, then that
> further alleviates this specific problem. We should build the runtime from the start to be streaming-centric,
> like JIT compiled runtimes also are, no?

## Agent answer to note 2

You are right, and it also shrinks note 1's problem.

Staged, sequential, spill-to-disk macro expansion in a streaming-centric runtime does dissolve the no-alloc
macro-expander problem, for the reason JITs and big compilers already rely on it: you never hold the whole
expansion in the fixed arena. Each stage expands one layer into a bounded window, streams its output to disk
(or through the reserve/commit sink, which is the same machinery the value-transfer topic already built), and
the next stage reads it back. The arena is a fixed window; disk is the unbounded accumulator; no_alloc survives
intact, because a fixed window plus disk I/O is bounded and heap-growth-free, the unboundedness lives on disk,
not in a growing allocation. It also unifies three things the round had separate: the value-transfer sink, the
AOT/JIT cache (the cached artifact is the spilled, expanded IR), and macro expansion, all become the same
streaming spine. Building the runtime streaming-centric from the start rather than retrofitting it is the right
call, and it is how every JIT-shaped runtime is actually structured.

The two notes reinforce each other. A streaming-centric runtime that spills to disk keeps the streaming
machinery itself runtime-only, so the code that must run at both binding times shrinks to the per-window
decode, validate, and check kernel, which is exactly the small, pure, comptime-legal kernel that makes note 1's
constraint easy to satisfy. The heavy I/O and spill parts never need to be comptime-legal because they only
ever run at the consumer.

## The open theory handed to the next experts

1. **Note 1 as theory to prove or refute:** can the dual-binding-time kernel (decode, bounds validation, mask
   accumulation, depth-ladder lease inference) all live in Zig's comptime-legal subset and stay within a sane
   `@setEvalBranchQuota` budget for a realistic language table (tens of families, hundreds of node kinds)? If
   yes, the wire-format-as-bridge stands and the "certified twice" story is honest. If no, the fallback is
   generated consts plus systematic comptime re-checks, and the boundary between what runs at both times and
   what is runtime-only needs drawing. This wants real Zig-build-time experience.
2. **Note 2 as design to validate:** the streaming-centric, staged, disk-spill spine as the runtime's
   foundation from the start, unifying the value-transfer sink, the AOT/JIT cache, and macro expansion. The
   open parts: its determinism (spill and re-read must be reproducible, no ambient clock, stable ordering), its
   interaction with the depth-ladder lease inference's post-order-scan assumption (does spilling and
   re-reading preserve the post-order interval structure the linear-scan inference needs), and the cross-chunk
   lease story Giesen flagged (a cross-chunk reference across whole-subtree-boundary mini-arenas is expected to
   be a consumed-or-promoted value, but that is an expectation, not a proof).
3. **Still standing from the two audits:** the depth-ladder lease schema as the debt-2/3 resolution (op's call
   to bless), the wire-blob-bridge-versus-generated-consts benchable choice, the panic-free untrusted-path
   invariant, the owned forcing root and exact Zig pin, and the widened four-arm negative experiment before any
   doc CL. And the run-the-experiment meta-debt: none of this is executed yet.
