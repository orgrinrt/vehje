# Roadmap: current state to the complete canonical design

Phase 7 of chart-the-path. Supersedes `roadmap-draft.md`, which stays as trail.
Updated against the phase 6 canon mirror (`xavier_leroy_canon_mirror.md`), which
found four wrong gates, one false premise, a deletion set that was not
dependency-closed, and ten canon mandates with no gate at all.

Pending the phase 8 granularity pass. No source is cut until the sketches land.

Proof status: **PROVEN** (committed sketch or bench, cited), **UNPROVEN** (needs a
TODO-sketch), **MECHANICAL** (no design risk).

## What changed from the draft, and why

The draft's endpoint was a tree-walking interpreter of one language. That is a
milestone in this canon, not its completion. `positive:45` fixes output as "a
spectrum from interpretation to transpilation" with native a real point on it,
and `positive:35` plus `negative:53` fix that spectrum's measured shape. The draft
gated none of it.

It also deleted the input side without relocating it. G0 correctly removes the
Rust `Grammar` trait, since a grammar consumes end-user scripts and Rust never
sees one. But `positive:45` still says "A grammar plugs into the input side and
lowers a surface syntax into the shared IR", and no gate rebuilt that anywhere.
The capability vanished rather than moved, which made the stdlib gate
unreachable.

Four individual corrections. G6's acceptance bar was the near-tautology
`negative:27` explicitly killed ("The kernel is one source compiled twice, so the
two binding times agreeing is near-tautology; the trust actually leaks at the
Rust-to-Zig emitter, which nothing gated"); the property wanted is seam-crossing
agreement keyed by the byte-image hash, with the manifest actually read. G10
computed `Knowledge` inside a pass G0 deletes. G14 deferred PE2's ratified "run
now" before op has answered. And G8 claimed zero harness cells when
`comptime-cost-cliff` and `partial-eval-specialization` both carry committed CSVs;
the honest statement is `positive:7`'s prototype-versus-shipped tag.

## Track structure

Two tracks. Both are required for "complete" per this canon. The draft conflated
them and then dropped the second.

**Track L, the language-completeness bar.** Standing calls 1 and 3: the full
`CLAUSE_EBNF.md` grammar, running, with a stdlib written in Clause.

**Track F, framework completeness.** Inclusion refusal, the emitter seam, the
load verifier, diagnostics, and the output spectrum. These are canon identity,
not follow-on polish.

**Sequencing between the tracks is op's call** and is stated as an open question
below rather than assumed. What follows orders within each track and marks the
cross-track dependencies.

---

## G0. Delete the wrong-locus Rust, dependency-closed

**What.** `vehje::run` and `Grammar` (`vehje/src/lib.rs:48,72`), `vehje-resolve`,
`vehje-typecheck`, `vehje-lower`, `vehje-fixpoint`, `vehje-schedule`,
`vehje-codegen::fold_core`, the per-script `encode` walk in `vehje-runtime-abi`,
and their doc templates. Roughly 3,300 to 3,700 lines.

**Canon.** `positive:45`, `negative:38`. Ruled by op 2026-07-26.

**Closure, which the draft missed.** `vehje-codegen` re-exports `Checked` from a
crate in the deletion set, and the crate carrying the inclusion-refusal machinery
had no stated fate. Before any file is removed, the set is closed transitively
and every surviving re-export re-homed. The inclusion typestate and `check_for`
are definitional and survive; where they live is part of this gate, not after it.

**Behaviour to pin before deletion.** `negative:91`'s first bug (the discarded
`GradeTable::set` failure) appears already fixed at
`vehje-typecheck/src/lib.rs:370-372`. That behaviour gets a catalogued red test
*before* the crate goes, so it is pinned wherever the pass re-lands. Per
`catalogue-edge-cases-as-tests.md`.

**Status.** MECHANICAL once closed. `vehje-runtime-driver` excluded, op's call.

## G0b. Pin the Zig toolchain

Moved ahead of everything from the draft's G9. Every Zig measurement in this
round, including both comprehension experts', is reproducible only against
whatever is on the runner's path, while `rust-toolchain.toml` is committed. Task
#54. **MECHANICAL.**

---

## Track A: make the engine sound

Independent of G0 and of the schema; runs in parallel.

### G1. A region-lending descriptor with a count
Replaces roughly 96 KiB of entry-point stack arrays (`runtime.zig:529-535,542`,
`host.zig:127`) that the module doc at `:15-17` already describes as caller-lent
arenas, and that `host.zig:160-162` calls "lent bounds like every other here",
which is false of both. `positive:45` PE3 makes host-owned memory an axiom.
Blocks everything; on a 4 to 16 KiB thread stack this overflows at the entry
point. **UNPROVEN** in ABI shape.

### G2. An explicit frame stack, `eval` defunctionalised over it
`positive:61` promotes defunctionalisation "from the principled route to the only
route"; PE3 mandates defunctionalised non-recursive passes. Reproduced: right-
nested `Let` faults in `eval` at `runtime.zig:330`, ok at 3,000 and SIGSEGV at
3,500 default, ok at 10,000 and SIGSEGV at 11,000 ReleaseFast. That shape is what
`Anf` is designed to emit. **UNPROVEN.**

### G3. Proper tail calls and environment reclamation
`Env.push` bumps with no pop (`runtime.zig:176-181`), so a three-binding loop body
dies at roughly 256 iterations. Blocks every loop, therefore the stdlib. Task #49.
**UNPROVEN**, needs G2.

### G4. A per-frame unwind carrier
The carrier is one shared mutable struct (`:132-136`); the `Handle` arm re-reads
`unwind.n` as its loop bound (`:388-394`) while passing the same pointer into a
nested `eval`. Reproduced: a two-operand clause with an inner handled computation
returns a closure where the value is 20. Silent wrong answer. **UNPROVEN.**

### G5. Diagnostics as a designed surface
New; the draft omitted it. `positive:39` requires diagnostics first-class with
"their own host-lent budget discipline, designed as deliberately as everything
else, before the conviction becomes the first place an allocator is smuggled in".
G1 and G2 produce named refusals and brush against this; nothing designs the
budget, the span mapping, or the suggestion surface. **UNPROVEN.**

---

## Track B: the keystone

### G6. The operation-semantics representation, with constructor coverage
What an operation's semantics is written in, covering introduction forms.
`positive:31`: "One declarative signature ... is the single source", false for
computation the moment meaning lives elsewhere. Consumed by everything downstream;
consumes nothing.

Known insufficient today: the four-way projection proved a five-opcode **scalar**
vocabulary and says so in its own "Does not establish"
(`202607260800_four-way-projection/findings.md:73-76`); the sketch then
hand-authored the compound operations in Zig (`eval.zig:483` on) with `OPS`
carrying five empty entries (`:34-39`) and a comment naming it (`:480-482`). The
census calls this the single escape gating every compound value
(`202607251530_language-completeness-census.md:57`).

**PARTLY PROVEN** scalar, **UNPROVEN** constructors. Acceptance: `OPS` carries no
empty entries and no compound operation has a hand-written Zig body. Any encoding
in the family is fine; a surviving hand-authored escape is not.

---

## Track C: close the seams

### G7. `generate` extracts; `build.zig` makes it comptime; the core is two-level
`generate` binds `_signature` (`vehje-runtime-gen/src/lib.rs:108`) and returns
caller slices plus a hash; `grep -c comptime` is 0 in every runtime file; the
manifest exists so the runtime "can prove at its own build that it was specialised
from exactly this package" (`lib.rs:73-76`) and nothing reads it. Needs G6.
**UNPROVEN** end to end; both ends PROVEN in isolation.

### G8. Close the emitter seam
New, and it replaces the draft's killed bar. `negative:27` moves the differential
gate off the kernel and onto the Rust-to-Zig emitter, "which nothing gated";
`positive:31` keeps closing that seam living with the mechanism open. The property
is seam-crossing agreement keyed by the byte-image hash with the manifest actually
read, not two evaluations of one definition agreeing. `negative:27` also records
that the per-slice manifest-hash unsoundness lives in this layer. **UNPROVEN**,
mechanism open, needs G7.

### G9. Family operations execute from data
Kills `NoHost` (`runtime.zig:414`) as the path for arithmetic; the host callback
survives only for foreign effects. The in-source defence at `:411-413` contradicts
standing call 2 and is still in tree. **MECHANICAL** once G6 and G7 land.

### G10. The load verifier
New. `negative:14` replaced tnum-for-structure with "a parse-don't-validate typed
structural decode"; `positive:29` puts "the load verifier's decode" in the trusted
rim. Nothing builds or hardens it. **UNPROVEN.**

---

## Track D: the graded spine, honestly

### G11. Thermometer-encode `Knowledge`, compute both hard-coded axes, in the licensed locus
Corrected from the draft, which computed it inside a pass G0 deletes and covered
one axis. `positive:27` names two hard-coded coordinates, `binding` and
`assurance`, and sets the spine's own acceptance bar: it "earns the word theorem
the day an axis-interaction rule exists in code with a red test that exercises
it". That rule and that test are part of this gate.

**Where it lives is an open canon question** (below), not something this roadmap
decides. **UNPROVEN**, and blocked on that answer.

### G12. Relocate the lease machinery
New. Reachability types primary (`positive:53`), Perceus exact-meet
(`negative:11`); the only implementation is the join fold in a crate G0 deletes.
Same relocation question as G11. **UNPROVEN.**

### G13. Inclusion refusal, rebuilt in the licensed locus
New. `positive:45`: "It refuses by inclusion, not coverage: a target declares the
family set it Supports and the effect set it Permits, and anything outside is
refused with the offending construct named, never silently degraded." Core
identity; the machinery sits in crates G0 touches. **UNPROVEN.**

---

## Track E: the input side and the language bar

### G14. The input side, in the licensed locus
New, and the draft's largest omission. `positive:45` keeps the grammar plug-in
("A grammar plugs into the input side and lowers a surface syntax into the shared
IR") while G0 removes the Rust one. Lexing, parsing, and lowering must exist
somewhere licensed, and the shipped runtime today consumes only a pre-built IR
image. Nothing downstream in this track is reachable without it. **UNPROVEN**, and
gated on the front-end locus question below.

### G15. Match, Iter, Interp
The three Core forms falling through to `Unsupported` (`runtime.zig:436`). Task
#43 owns the pattern representation. **UNPROVEN.**

### G16. The continuation model, CR1
`positive:25`: "The runtime has no continuation model yet." PE2 orders the
representation bench **now**, and this roadmap does not defer it: the
CPS/defunctionalisation candidate is buildable today and that half runs on op's
existing order. The segmented capture-and-reinstate candidate needs G2 for a frame
to segment. The two artifacts currently carrying "multi-shot" contain no handler,
no perform, and no continuation. **Sequencing is op's**, stated below.

### G17. Macro expansion and hygiene red tests
Blocked on the graded axis, not a missing Core form
(`202607251530_language-completeness-census.md:63`), so needs G11. Hygiene red
tests are ratified (XVII6) and were omitted from the draft. The N-buffer arena
fork is downstream. **UNPROVEN.**

### G18. The grammar bar, then the stdlib in Clause
Standing calls 1 and 3. Full `CLAUSE_EBNF.md`, then a stdlib in Clause loaded
through a real module system rather than the sketch's text concatenation.
**UNPROVEN.**

---

## Track F: the output spectrum

New; the draft gated none of it, which is why its endpoint was wrong.
`positive:45` fixes output as a spectrum with native a real point; `positive:35`
and `negative:53` fix the measured shape.

### G19. Fold, CSE, DCE always-on
### G20. The predecoded-register middle tier
### G21. The batched-column W>=2 C ABI entry
Recorded as the single biggest measured lever, and absent from the shipped ABI.
### G22. CFG terminator transfers
`negative:90` records the CFG residual types defined and never constructed
(`vehje-runtime-abi/src/wire/residual.rs`).
### G23. Direct isel, the native point

All **UNPROVEN**. Ordering within the track follows the measured lever sizes.

---

## The red-test catalogue, spanning every gate

`positive:57`: "every canon-grade claim carries a catalogued red test or a bench
cell at authoring"; `negative:86` makes the fix behaviour-shaped catalogued red
tests. The TODO-sketch column is not that catalogue. Each gate lands its rows as
red tests at authoring, and the second-direction audit's rows are committed as
red tests rather than prose.

## Open, and op's alone

1. **Track sequencing.** Whether Track F is deliberately after the language bar.
   If so this roadmap must say so and name the follow-on, because "complete" per
   this canon includes the spectrum.
2. **The per-script proof locus**, after "Rust knows no scripts". One expert
   derived the runtime artifact at Bundler/HostLoader binding time; that is one
   read and a second independent agreement is owed before G11 or G12 build on it.
3. **The front-end locus** for arriving scripts. Sizes G14 and therefore the
   whole of Track E.
4. **Where artifact C lives.**
5. **Whether the mechanism stays comptime**, given `positive:29` puts it in the
   certification perimeter.
6. **The comptime-cost measurement drift.** Comptime cost cannot go through the
   cdylib harness model, so standing call 5 needs either a harness compile-time
   mode or a scoped exception.
7. **`vehje-runtime-driver`'s fate.**
8. **PE2 sequencing**, given G2 blocks half the CR1 fork.
9. **The Arena-tier re-sequencing**, an agent-called change needing confirming.
