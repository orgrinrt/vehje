# Count our chickens, take 2: after the two fresh expert audits

**Date:** 2026-07-24
**Author:** main agent (the post-audit review step of op's vacation directive)
**Purpose:** review what the two fresh experts (Lattner, then the Torvalds sequential double-take) added on top of the already-ratified inventory, state what we now know and what we ended up with, and consolidate the still-open decisions into the question set op will answer next. This is the "review and count our chickens" step. It is not the canon; the canon (the two positive/negative catalogues, from scratch, synthesised) follows after op answers the questions below.

The two fresh audits are `chris_lattner_soul-and-intent-audit.md` and `linus_torvalds_soul-double-take.md` in this directory. op had already ratified the whole inventory (Groups I-XVII, `op-ratification-answers.md`); these two are the owed outside-the-detail expert check op asked for before the canon is written.

## What the two fresh experts concluded (the headline)

Both confirm the soul-and-intent holds. The identity (embeddable multi-input/multi-output IR framework; small closed Core plus open families; inclusion-not-coverage; the four-point binding-time lattice as the compile/runtime structure; certified generation as data not source; bench-decided forks) is coherent, well grounded, and the one-sentence soul survived a sixth and seventh reader. Every headline defect the panel found was already caught, op-ratified, and corrected in the shipped canon; both experts verified the source fixes landed. Neither found a rogue architecture or a lost identity.

They diverge in emphasis, and the divergence is the value:

- **Lattner (infrastructure lens):** the defects were characteristic, not structural (over-pinned bench numbers, CR1 presented as more proven than the sketches show, the §8 shadows framing being unify-by-analogy). The correction machinery working on its own outputs is the strongest evidence for the document's deepest claim. The commitment-tier prior-art device is the directory's best invention.
- **Torvalds (systems / embeddability / earn-its-keep lens):** the theory earns its keep exactly where it collapsed into a boring data structure (the binding-time lattice is four bits; effect inclusion is a bitwise OR and a subset test; the batched-column ABI and the fold+CSE+DCE reducer are bench-picked winners), and has not earned it anywhere it is still prose. The risk is not wrongness; it is a project that has become better at auditing its intent than at executing it. The whole framework is ~3,760 lines of Rust across twelve crates; the design corpus dwarfs it. The fix is the next arc being code, which the canon already orders.

## What the double-take added that the panel had not seen

1. **Two concrete source bugs, missed by all seven prior readers.**
   - `check` discards `GradeTable::set`'s failure `Bool` (`vehje-typecheck/src/lib.rs:314`): an undersized grade region silently drops grades, `check` still returns `Ok`, a `Graded` is minted over ungraded nodes. A silent-truncation hole in the exact pass whose identity is "the type system is the verification layer." Fix shape: a `must_use` outcome, or a constructor that refuses a region smaller than the arena so the illegal state cannot be built.
   - `infer` (`typecheck:217-316`) and `structurally_equal` (`vehje-lower/src/lib.rs:296-363`) recurse on IR depth, violating the project's own bench-proven "defunctionalization is the only route" rule. A deep-skewed 50k-node program (the project's own capstone size) overflows the host call stack in exactly the embedded contexts an embeddable framework courts. The lesson was learned on Zig comptime at depth ~2500 and not applied to the Rust passes written afterward.
   Both are actionable now: catalogue each as a red test, then fix. (My call: these land in the §11 IMPL rounds, red test first.)

2. **The theory-not-yet-cashed ledger, priced against the census.** The "one soundness theorem" is currently a vocabulary over two live axes (`effect`, `lease`) plus two hardcoded constants (`binding = Knowledge::empty()`, `assurance = Unclaimed`); no theorem exists as an artifact. CR1's multi-shot half has no census consumer that backtracks (the canon justifies it with generators/coroutines/events/resumable-errors, all one-shot). `vehje-schedule` is a DAG scheduler for four statically-ordered passes with no consumer family pass in existence. `vehje-fixpoint` has zero in-tree callers. None indicts the identity; all price the ambitious-tail theory the authoring majority never files a bug for.

3. **The correction machinery does not scale the way the panel implied.** A dozen expert artifacts auditing ~85KB of prose over 3.7k lines of code, with the maintainer ratifying seventeen groups, caught every headline defect but is a cost curve that does not survive an implementation phase. What scales is the bench harness (opacity boundaries, cdylib isolation, cross-validation) and catalogued red tests. The workspace has exactly ONE catalogued red test against 23 audit rows of doc-over-claim. Torvalds: red tests scale, adversarial prose audits do not; the `design-doc-source-mismatch` lint verifies nouns exist, not that behaviour is satisfied, which is exactly how "surface existence standing in for mandate satisfaction" walks the gate.

4. **The embedder API is the product and the least-designed surface in the repo.** For an embeddable framework, the contract that decides adoption is what a third-party grammar/family/host author writes, not the proof spine. Today `Grammar::lower` does not take a source input (FIXME), a family author writes nightly no_std no-alloc Rust against a cons-list typestate under a WATCH-tier `generic_const_exprs` pin, and front-end scaffolding is "offered opt-in, not shipped." All ten census consumers are first-party; the "framework serving a census equally" identity has an unpriced dependency (either the seam gets author-clearable scaffolding, or the census stays whoever op is).

5. **The ABI is the one interface that outlives every mechanism, and its compatibility policy is design work owed before the first consumer, not after.** Interfaces are forever; proofs are replaceable. The canon prices the proof axes explicitly and does not price the ABI's versioning / refusal-of-a-newer-residual semantics with the same explicitness.

6. **Diagnostics-first-class (op's N1) has no cost model on a no-alloc substrate.** Message formatting, suggestion synthesis, and span mapping want buffers the discipline forbids; either diagnostics get their own host-lent budget discipline designed now, or the conviction becomes the first place an allocator is smuggled in.

7. **The spirit-over-letter precedence clause is a twenty-year relitigation hazard.** "The canonical spirit outranks the canonical letter" worked for a months-long round with a live maintainer; as a standing rule in a document written to be read years from now, whoever claims to channel the spirit wins any argument. Twenty years of maintenance says the letter governs and gets amended by process; the drift test (Section 10) is the letter.

## The strongest single recommendation, from the systems lens

One real consumer end to end (mockspace's procedural-documents language: a real consumer, real user, in this workspace, matching the census-majority compile-heavy/trivial-execute profile) lowered through a real grammar to a real residual executed by the real runtime would decide more of the canon's Section 11 open forks (CR1 representation, N-buffer arenas, terminator mechanics, record width, the schedule crate's existence) than the next three panels. The project's own precedence rule says hard data outranks topics; a running consumer is the hardest data there is.

## What we ended up with (the net)

A ratified, twice-freshly-audited identity that holds; a paper trail that is honest about the design-vs-shipped gap; a precise, prioritised set of §11 IMPL rounds (the canon's own Section 11, now sharpened by two concrete bugs to catalogue-then-fix and a strong "build one consumer" steer); and a small set of genuinely-open decisions that are op's to make, consolidated below into the question batch. The identity is not in question. The open items are execution-shape and governance calls.

## The consolidated open decisions for op (feeds the question batch)

- **vehje-schedule:** build to DESIGN / freeze-contingent / dissolve into the one-reducer discipline.
- **CR1:** one-shot-first sequencing vs run the multi-shot representation bench-fork now.
- **Compile-side no-alloc:** stack-wide axiom (accept + mandate the mitigations) vs policy (a bounded bump allocator on the dev-time side that never ships).
- **Spirit-over-letter precedence clause:** keep as written vs narrow to intermediate-artifact reading, letter amendable only by a superseding round.
- **First consumer:** which (mockspace-docs vs jomini) and when (before vs after the Section 11 wiring).
- **Next canon-grade round gate:** require a catalogued red test / bench cell per claim at authoring (prose as an index over executables) vs keep the adversarial-panel shape.
- **Centre-of-gravity / one-liner:** confirm the fusion distillation direction (op delegated the phrasing to me; op flagged it and the I6 metacompiler shape for confirmation).

Low-stakes items I will simply apply (not op-questions): banner the two candidate docs as superseded-by-`202607241545` (one line each); extend the verb ladder with the measured-artifact distinction (measured-prototype vs measured-shipped-crate); catalogue the two new bugs and the 23 audit rows as red tests during the IMPL rounds.
