# The soul of vehje, a systems double-take (Linus Torvalds)

**Author lens:** long-lived embeddable systems code; what survives real hardware, real third-party
authors, and twenty years of maintenance. Sequential double-take on Lattner's audit.
**Read myself, not on anyone's say-so:** both `canonical_candidate_` docs, Lattner's audit, the Wingo /
SPJ / Leroy reads, `op-ratification-answers.md`, the shipped canon `202607241545`, the second-direction
audit, and the source: `vehje-typecheck/src/lib.rs` (all 458 lines), `vehje-ir/src/grade.rs`,
`vehje-schedule/src/lib.rs`, `vehje-lower/src/lib.rs`, `vehje-fixpoint/src/{lib,engine}.rs`,
`vehje/src/lib.rs`, `vehje-signature/src/lib.rs`, `benches/results/RUN_SUMMARY.md`,
`carrier_native_ceiling/FINDINGS.md`. One number nobody in the panel printed: the entire shipped
framework is **about 3,760 lines of Rust across twelve crates** (`wc -l` over `mock/crates/*/src`).
Hold that number against the size of the design corpus while reading everything below.

## Verdict in one paragraph

The theory earns its keep in exactly the places where it collapsed into a boring data structure, and
has not yet earned it anywhere it is still prose. The binding-time lattice is four bits in a mask
(`grade.rs:105-160`) and it answers the real architectural question (where does each discharge live);
the effect-inclusion check is a bitwise OR and a subset test gated by trait bounds
(`grade.rs:55-58`, `typecheck:158-168`); the batched-column ABI and the fold+CSE+DCE reducer are
bench-picked boring winners. That 80% is good taste: the special cases disappeared because the data
structures are right, and I would merge it. The other 20% (the one-soundness-theorem framing over two
axes that are hardcoded constants, CR1's multi-shot half with no consumer in the census that needs it,
an assurance lattice nobody computes, a relational engine with zero in-tree callers, a pass-scheduler
crate that is three empty types) is theory the census has not cashed and, on the project's own bench
record, keeps failing to cash. Will it ship? The correction machinery demonstrably works at 3.7k lines
with the maintainer reading everything; whether the maximal-shape ethos survives the implementation
phase is untested, and the single thing that would settle most of the open panel arguments faster than
any further canon prose is one real consumer running end to end. The design is honest, the discipline
is real, the identity is right; the risk is not wrongness, it is a project that has become better at
auditing its intent than at executing it, and the fix is the next arc being code, which the canon's
own Section 11 already orders. Good. Do that.

## Where Lattner is right

Conceded without ceremony: the framework identity is the MLIR bet with a real differentiator, and
inclusion-not-coverage with the refused construct named is the correct contract shape (silent
degradation is how downstream users get betrayed; naming the refusal is keeping the promise). The
binding-time lattice's independent reconstruction by Rompf is corroboration money cannot buy. The
two-artifact split is forced, not fiat. The commitment-tier prior-art device is genuinely good. His
finding 3 (dense line-cited provenance creates the appearance of verification without being
verification) is the deepest process sentence in his file and generalises to every project that ever
wrote a design doc. His finding 5 (surface existence standing in for mandate satisfaction) names the
root cause of every one of the second-direction audit's 23 rows. And his read of the panel is fair:
Wingo's sk2 catch is the sharpest technical finding in the directory, Leroy's trusted-rim enumeration
is the deepest addition, Fallin's public self-correction is the discipline working.

## Where the theory does not pay for itself

1. **The "one soundness theorem" is currently a vocabulary, and its own novelty audit said that is
not enough.** Claim: one graded (co)modal judgment, per-axis results as corollaries, axis interactions
as typing rules (canon 2D). Cost: the heaviest theoretical commitment in the project, carried by every
doc surface. Reality in source: the four axes are two 64-bit masks, a 4-bit mask, and a defaulted enum
(`grade.rs`); `binding` is hardcoded `Knowledge::empty()` and `assurance` hardcoded `Unclaimed`
(`typecheck:311-312`); effect subtraction under `Handle` is a FIXME (`:301-303`); no theorem exists as
an artifact anywhere. The round's own audit rejected unify-by-analogy with "a filing system gives you
one vocabulary; it does not give you one theorem." Right now the spine IS a filing system. What a
working system does instead: ship the three checks as three checks (they already are, mechanically),
and let the unification earn the word "theorem" the day an axis-interaction rule exists in code and a
red test exercises it. The verb ladder ("intended") keeps the canon honest; my point is sharper: the
payoff of unification is at axis interactions, the templating majority (compile-heavy,
trivial-execute, per the project's own capstone: 3.24 ms compile, 127 us run) barely exercises any,
so the expensive half of the spine is being carried for the ambitious tail. Legitimate choice, but
price it: every axis-interaction typing rule is an obligation the census never files a bug for.

2. **CR1's hard half is a research project with zero customers, and the canon justifies it with
one-shot use cases.** Canon Section 11: the continuation representation is "the deep unblocking piece
for every generator, coroutine, event, and resumable-error consumer." Look at that list. Generators,
coroutines, events, resumable errors: all four are one-shot resumption. OCaml 5 shipped one-shot by
default for exactly the soundness reason Wingo names (multi-shot replay of a non-pure body). The
census (documents, dotfiles, four game-script dialects, a lint DSL) contains no named consumer that
backtracks. So the genuinely novel claim (no-alloc bounded multi-shot *reinstatement*) is the
unproven half, the unsound-under-effects half, and the half nothing asked for. What a working system
does: ship one-shot resumption plus the sk2 re-execution form for bounded search (both proven-shaped,
both cheap), catalogue multi-shot reinstatement as a red test with no schedule, and let the first
consumer who actually backtracks pay for the bench fork op ordered. op's XVII4 ("needs a bench, not a
blind decision") is right as far as it goes; the cheaper question to answer first is "who resumes
twice," and the census's answer today is nobody.

3. **`vehje-schedule` is speculative generality wearing a finished DESIGN doc, and the audit ranked
building it first.** Source fact: `pub trait Pass: core::fmt::Debug {}`, a unit-struct `PassDag`, and
`Schedule::build()` returning `Maybe::Is(Self)` unconditionally (`vehje-schedule/src/lib.rs:27-73`).
The compile pipeline it would schedule is resolve, check, lower, emit: four passes with a fixed order,
called in sequence from `vehje::run`. A DAG scheduler with `BitMatrix` adjacency and topological sort
over four statically-ordered function calls is machinery for a problem that does not exist. LLVM's
pass manager earns its existence with hundreds of passes and dynamic pipelines; vehje has no consumer
family pass, not one. The crate's own DESIGN concedes it may dissolve into the handler discipline.
The second-direction audit ranked building it first by unblock-value; I dispute that flatly: a pass
scheduler unblocks nothing when there are no passes to schedule. What a working system does: delete
the crate or mark its DESIGN contingent, hardcode the call sequence in `vehje::run`, and re-open the
crate the day a consumer registers a family pass whose ordering is not obvious. That is not YAGNI
cowardice; the maximal-shape ethos demands designing the unification, not pre-building a graph engine
for a straight line.

4. **`vehje-fixpoint` is infrastructure built ahead of both of its justifying consumers.** The engine
has zero in-tree callers (`grep vehje_fixpoint::` across consumer crates: nothing), the load verifier
it half-exists for does not exist, its crate doc calls it semi-naive while its own FIXME says only
whole-column ships (`engine.rs:143-148`), and the 8M-node/50ms number that justified it was measured
on a standalone Zig prototype, not this crate. Meanwhile the lease inference it will serve runs, for
the census majority, over programs whose *entire compile* is 3.4 ms at 50k nodes; a worklist walk
would do. The relational engine may well be right for the ambitious end; today it is a mechanism with
no client, which is the definition of unverifiable. What a working system does: wire the reach
inference in `check` through it (the FIXME at `typecheck:186-189` already names this) before adding
one more feature to the engine, so the engine has one real consumer keeping it honest.

5. **No-alloc on the dev-time compile side is a tax the docs never price.** The runtime side's
no-heap discipline is exactly right for an embeddable artifact: bounded, deterministic, host-owned.
The compile side is a dev-time Rust tool the design says never ships, yet it carries the same
caller-lent-region discipline, and the cost shows in every API: three parallel hand-sized arrays to
build an eight-node test program, a `GradeTable` the caller must size "to at least the node arena's
capacity" by convention. And here is the bug-shaped consequence, in shipped source today:
`GradeTable::set` returns `Bool(false)` when the region is undersized, and the one in-tree caller
**discards that return** (`typecheck:314`, `grades.set(at.index(), grade);` as a bare statement). An
undersized grade region silently drops grades, `check` still returns `Ok`, a `Graded` is minted, and
downstream reads `Maybe::Isnt`. That is a silent-truncation hole in the exact pass whose identity is
"the type system is the verification layer." Fix: make the failure impossible to ignore (a must_use
outcome, or better, a constructor that refuses a region smaller than the arena so the illegal state
cannot be built). The stack's own discipline, applied to itself.

6. **The shipped check pass violates the project's own bench-proven wall.** Bench finding v:
"recursion is a hard compiler wall; the kernels are iterative; defunctionalization promoted from the
principled route to the only route," binding the comptime kernel, the load verifier, and the
extractor. But `infer` recurses on IR depth through the `child` closure (`typecheck:217-316`), and
`structurally_equal` recurses in lower (`vehje-lower/src/lib.rs:296-363`). The Rust call stack is the
host's, unbounded by any lease, and a deep-skewed 50k-node program (the project's own capstone size)
will overflow it in precisely the embedded host contexts an embeddable framework courts. The lesson
was learned on Zig comptime at depth ~2500 and then not applied to the Rust passes written afterward.
Nobody in the panel caught this. It is the cheapest possible demonstration that the design record,
however audited, does not verify code; a red test (`check` on a right-leaning 100k-node chain) would
have. Catalogue it now, per the workspace's own rule.

7. **The doc-corpus-to-code ratio is itself a finding, and the gate that should catch it verifies
nouns, not behaviour.** Twelve DESIGN docs, several describing module trees that do not exist and
schedulers that are unit structs, over 3.7k lines of code, needed three audits in one day to stay
honest. The `design-doc-source-mismatch` lint checks that backticked type names exist, and
`Pass`/`PassDag`/`Schedule` all exist, which is exactly how "surface existence standing in for mandate
satisfaction" walks through the gate. The mechanical fix is not more prose discipline, it is the one
the workspace already mandates and did not apply: behaviour-shaped catalogued red tests. The whole
workspace contains exactly ONE `#[ignore = "catalogue: ..."]` test (the slot-collision one,
`typecheck:390`) against 23 audit rows of doc-over-claim. `schedule_detects_cycle`,
`macro_expansion_splices`, `anf_names_intermediates`, `distribution_composes_two_targets`: each is
ten minutes to write, stays red, and makes the present-tense DESIGN prose harmless because the test
states the truth executable. Red tests scale; adversarial prose audits do not.

## Where Lattner was too kind or too harsh

**Too kind on the correction machinery's scalability.** "The discipline actually works on its own
outputs" is true and he proves it, but the demonstration is a dozen expert artifacts and multiple
adversarial passes spent auditing roughly 85KB of identity prose describing 3.7k lines of code, with
the maintainer personally ratifying seventeen groups of items. That machinery caught every headline
defect, and it is also a cost curve that does not survive contact with a real implementation phase,
where the bugs live in code and no panel of five reads every function. The thing in this project that
scales is the bench harness (opacity boundaries, cdylib isolation, cross-validation: executable,
mechanical, repeatable) and the red-test catalogue rule. Lattner crowns the commitment-tier device
the directory's best contribution; I say it is the harness's opacity-boundary rule, because one is a
filing convention and the other is a machine that catches lies.

**Too kind on "the gaps are wiring, not direction."** Mostly true, but findings 5 and 6 above are
direction-shaped: a silent-failure API in the verification pass and a recursion pattern the project's
own benches banned are not missing wires, they are the code contradicting the design's stated
character, in the small amount of code that exists. The drift forks audited prose against prose; the
one pass that audited source found 23 rows. The ratio of prose-audit to source-audit effort in this
round was badly inverted.

**Too harsh, or at least too bureaucratic, on nothing much.** His open question 1 (banner the
candidates) should just be done, one line each, supersession is the norm here and audit-trail purity
is not threatened by a pointer. His question 5 (worked examples per drift-test clause) can wait for
the first real dispute, as he himself suspects.

**A miss rather than a kindness: he read the precedence rule and did not flinch.** "The canonical
spirit outranks the canonical letter" is written into the canon's own preamble. For a
months-long round with a live maintainer it worked. As a standing rule in a document "written to be
read years from now," it is a maintenance timebomb: spirit-over-letter is how every long-lived
project relitigates settled decisions, because whoever claims to channel the spirit wins any
argument. Twenty years of kernel maintenance says the letter governs and the letter gets amended by
process. The drift test (Section 10) is the letter; keep sharpening it and demote the spirit clause
to what it really is, a rule about how to read stale intermediate topics, not a licence over the
canon itself.

## What his lens missed

**The embedder API is the product, and it is the least-designed surface in the repo.** For an
embeddable framework the contract that decides adoption is not the proof spine, it is what a
third-party grammar, family, and host author must write. Today: `Grammar::lower` does not yet take a
source input (FIXME at `vehje/src/lib.rs:54-56`); a family author writes nightly no_std no-alloc Rust
against a cons-list `AccessSet` typestate, under a toolchain pinned to an exact nightly with
`generic_const_exprs` (a WATCH-tier feature the const-generics team calls fundamentally flawed) in
the dependency stack; front-end scaffolding is explicitly "offered opt-in, not shipped now." MLIR won
adoption with tablegen and C++ that ordinary compiler people already write. Every one of vehje's ten
census consumers is first-party. That is fine for a first-party stack; it means the "framework serving
a census equally" identity has an unpriced dependency: either the extension seam gets scaffolding that
ordinary authors clear, or the census stays whoever op is. The panel audited the soul's theory
frontier exhaustively and never asked what it costs to write `vehje-jomini`.

**The toolchain is part of the trusted base and part of the maintenance surface, and neither doc
prices it.** Leroy enumerated the trusted rim (rustc, Zig, the C ABI marshalling, the hash, the
emitter seam) for *soundness*. The maintenance version of that list is scarier: an exact nightly pin
whose load-bearing feature may never stabilise, a pre-1.0 Zig whose comptime behaviour IS the
certification mechanism for the shipped artifact, and a C ABI that becomes forever the day the first
external consumer ships a residual. The design correctly treats the wire format as
measure-then-lock pre-1.0; the canon should state, with the same explicitness it gives the proof
axes, that the ABI is the one interface that will outlive every mechanism in the document, and that
its compatibility policy (versioning, refusal semantics for a residual from a newer compiler) is
design work owed *before* the first consumer, not after. Interfaces are forever; proofs are
replaceable.

**One consumer end to end is worth more than the next three panels.** The fastest resolver for the
open forks (CR1 representation, N-buffer arenas, terminator mechanics, record width, the schedule
crate's existence) is not another adversarial read, it is mockspace's procedural-documents language
(a real consumer with a real user: this workspace) lowered through a real grammar to a real residual
executed by the real runtime. Every fork listed in canon Section 11 either gets decided or gets
deleted by that exercise. The project's own precedence rule says hard data outranks topics; a running
consumer is the hardest data there is. Talk is cheap. This round produced excellent talk. The next
one should produce the consumer.

**Diagnostics-first-class (op's N1) currently has no cost model.** "Informative errors AND actionable
suggestions with span links that integrate with LSP" is the right conviction and is also, on a
no_std no-alloc substrate, real architecture: message formatting, suggestion synthesis, and span
mapping all want buffers and string building the discipline forbids. Either diagnostics get their own
host-lent budget discipline designed now (the same shape as everything else, fine), or the conviction
quietly becomes the first place someone smuggles in an allocator. Name it before it names itself.

## Open questions for the maintainer

1. **`vehje-schedule`: build, dissolve, or freeze.** Building it now is cheap and honours the DESIGN;
dissolving it bets on the one-reducer unification and deletes a crate whose need is unproven either
way; freezing (DESIGN marked contingent, stub stays, red test catalogued) defers the bet. The audit
ranked build-first by unblock-value; finding 3 above disputes that there is anything to unblock. The
three options cost respectively: work that may be discarded, a reversal if a consumer pass appears,
and a finished-looking doc sitting on a stub longer.

2. **CR1 sequencing: one-shot first, or the multi-shot bench fork as ordered.** Shipping one-shot
resumption plus re-execution covers every named census use and defers the novel half indefinitely;
running the XVII4 bench fork now keeps the novelty claim on schedule but spends the next sketch
budget on a mechanism with no consumer. The tradeoff is time-to-a-working-Handle versus
time-to-the-headline-claim.

3. **Compile-side no-alloc: axiom or policy.** If it is a stack-wide axiom, the ergonomic and
safety costs (finding 5's silent-drop hole, the region-sizing conventions, recursion pressure toward
finding 6) should be named in canon as accepted costs with their mitigations mandated (must_use
outcomes, size-refusing constructors, defunctionalised passes). If it is policy, the dev-time
compile side could take a bounded bump allocator and delete a class of API hazard. The first option
keeps one discipline everywhere; the second trades symmetry for a smaller trusted-convention surface
on the side that never ships.

4. **What gates the next canon-grade prose round.** This round's yield was real and its cost was
real. Options: require every future canon-grade claim to carry a catalogued red test or bench cell
at authoring time (prose becomes an index over executables); or keep the adversarial-panel shape for
identity documents and accept the cost at the cadence identity actually changes. The first slows
canon writing and speeds drift detection; the second is what just happened.

5. **The spirit-over-letter precedence clause.** Keep it as written (flexibility for a design still
in motion, at the cost of future relitigation surface), or narrow it to intermediate-artifact
reading only, with the canon's letter amendable solely by a superseding round. Governance calls are
the maintainer's; twenty years is the horizon on which this one matters.

6. **First consumer selection and timing.** mockspace-docs is in-workspace, compile-heavy,
trivially-executing, and matches the census majority profile; jomini is the adoption showcase but
needs the most front-end. Building either before the Section 11 wiring completes would drive the
wiring order from real need; building after keeps the framework's surface stable while wiring.
Which risk is preferred: wiring shaped by a consumer's accidents, or wiring shaped by design guesses
a consumer later contradicts.
