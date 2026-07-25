# vehje as a minimal complete implementation (Fabrice Bellard)

## Canon gate outcome

**Aligned, and I proceed.** Checked against `mock/research/canon/the-soul-of-vehje-positive-catalogue.md`,
`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md`,
`mock/research/202607260100_op-standing-design-calls.md`, the round topic
`mock/design_rounds/202607241615/202607241545_topic.the-vehje-canon.md`, the ratification record
`mock/research/202607241400_ir-to-ir-and-the-compile-runtime-line/op-ratification-answers.md`, and the three
live designer statements relayed mid-dispatch (N in / M out with interop defined in the Rust artifact; a
generic language-agnostic shared runtime core is licensed; Rust extracts the specialisations).

The dispatch itself is on the canon: it asks what the canon calls for and what the ordered path from the real
state is, it names the canon as governing, and it declares existence and locus challengeable. That is the
corrective question, not a question built on the drift. Refusing it would leave the drift in place.

**The state it builds on is misaligned in eight specific places, and those are the substance of the answer
rather than a reason to stop.** They are enumerated in section 5 with canon text and `file:line`. The single
largest is that roughly 3,700 lines of the 8,323-line Rust tree implement a per-script compile pipeline that
the canon forbids Rust from having at all, and that the Zig sketch has independently re-implemented on the
correct side of the boundary. Two implementations of the same passes, in two languages, on opposite sides of a
ratified artifact boundary, one of which can never ship.

Three points where the canon does not decide a question the answer depends on are handed back in section 7
rather than resolved here. The first of them sizes everything else in this document.

## Verdict in one line

Complete is roughly 17,000 lines across three artifacts (about 3,000 Rust that never sees a script, about
4,500 Zig of language-agnostic engine, about 9,500 Zig of front end plus the Clause stdlib written in Clause),
which is smaller than what is already on disk; the tree is not too big, it is 3,700 lines too big in Rust and
about 12,000 lines too small in Zig, and the sketch nobody has promoted is the closest thing in the repository
to the real artifact.

## What complete actually costs here

Measured comparables first, because an estimate with no comparable in it is a guess wearing a number.

QuickJS is 61,424 lines in `quickjs.c`, about 57,000 of them code, and roughly 75,000 with `libregexp`,
`libunicode` and `cutils`. That buys complete ES2020: a garbage collector, prototypes, proxies, generators,
async, BigInt, a regexp engine, and the Unicode tables, conformance-tested against test262, in 210 KiB of x86
for a hello world. Lua 5.4 is about 30,000 lines of C, of which the lexer, parser, code generator, VM, string
interner and incremental GC are roughly 17,000 and the standard library about 8,000, for a complete dynamic
language. Wren is about 8,000 lines of C for classes, fibers and a GC. TCC is about 30,000 lines for a
self-compiling C99 compiler carrying its own assembler and linker. MinCaml is about 2,000 lines of OCaml for a
complete ML compiler: Hindley-Milner inference, K-normalisation, closure conversion, register allocation, and
SPARC emission. MinCaml is the closest analogue to Clause's shape and is the honest floor.

Clause is cheaper than QuickJS on every axis that makes QuickJS large and dearer on two axes MinCaml does not
carry. It has no garbage collector (leases and arenas, canon positive:45 and :117), no dynamic property
lookup, no prototypes, no regexps, no Unicode tables, no async machinery, and no runtime types at all, because
monomorphisation erases them (`202607251530_language-completeness-census.md`, "Monomorphisation erases types").
It adds constraint-carrying inference (traits, supertraits, associated types, row polymorphism), pattern-match
compilation with real exhaustiveness over sums, and the handler discharge that carries `return` / `break` /
`continue` / `?` / `loop`.

The measurement that turns this from analogy into arithmetic is the sketch. `mock/research/sketches/202607260900_clause-in-zig`
is 5,646 lines of Zig, of which about 1,125 are test blocks, so about 4,500 lines of implementation:
`clause.zig` splits into a 156-line lexer, a 297-line IR builder, a 1,784-line parser plus lowering plus
monomorphisation, and a 33-line image writer; `check.zig` is about 950 lines of inference; `eval.zig` is about
980 lines of evaluator. `zig test eval.zig` runs 158 tests in 0.27 s wall, all passing, verified in this
dispatch. Against `mock/research/original-docs/CLAUSE_EBNF.md` and the census table, that 4,500 lines covers
somewhere around 55 to 65 percent of the productions, missing syntactic (token-stream) macros, floats and
chars, the full pattern grammar's tuple and reference and rest forms, `if let`, multi-segment path
re-exports, and the compound-value boundary.

So the finished Clause front end, all of the normative grammar, is about 8,000 to 12,000 lines of Zig, and I
would hold 9,500 as the working figure. It is bigger than Lua's 17,000-line core would suggest only because
the type system is doing work Lua does not do, and it is a third of QuickJS because it does not carry any of
what makes QuickJS big.

The generic engine is 2,155 lines today and needs the comptime specialisation stage, a table-driven operation
dispatch, the value arena with leases, the compound value boundary, and the predecoded-register tier the
benches chose (canon §7). Call it 4,500 lines of Zig.

The Rust side, under the designer's statement that Rust extracts the specialisations, is small and is almost
entirely a data model plus a typestate that erases. It is the IR and grade definitions (`vehje-ir`, 1,451
lines today, mostly right), a signature that actually carries a language (129 lines today, needs about 800), a
package and manifest emitter (146 lines today, needs about 600), the inclusion typestate (about 200), and the
wire *layout* definition as distinct from any per-program serializer (about 300). Total about 3,000 lines,
which is roughly 36 percent of what is in `mock/crates` now, and a different 36 percent.

Seventeen thousand lines. TCC territory, well under QuickJS, and the current tree already holds 16,124 across
Rust, the Zig runtime and the sketch. Nothing here is oversized. It is misplaced.

## What in the current tree is load-bearing, and what is not

The test throughout is canon positive:45, "a dev-time Rust language compiler (emits validated data and
structural proofs, **never sees an end-user script**, never ships)", together with canon negative:38, "**Rust
never runs per-script analyses, the runtime does, for all scripts**". A Rust item whose input is a program
instance fails both. The designer's third live statement ("the rust should extract the specialisations") says
the same thing positively.

1. **`vehje-ir` (1,451 lines). Load-bearing, in the right place.** The Core node algebra, the four-point
   binding-time lattice at `vehje-ir/src/grade.rs`, the set typestate, the arena and the structural hash. This
   is the middle IR the designer's first statement names as the convergence point of N inputs and M outputs,
   and it is the one crate defining rather than consuming a program. Keep essentially whole. Two corrections
   are owed inside it: the `Grade` axes `binding` and `assurance` are never computed (see item 4), and the
   thirteen node tags versus the canon's "twelve-forms-complete-by-algebra" (negative:30) wants reconciling on
   whether `Perform` and `Handle` are one form or two.

2. **`vehje-signature` (129 lines). Load-bearing in principle, almost entirely absent in fact.** Under the
   canon this is *the* thing Rust compiles: "`vehje-ir` is what a program is, `vehje-signature` is what a
   language is" (`vehje-signature/src/lib.rs:8-9`). It carries `Operation { family, effect, lease }`, a target
   list, and a two-variant grammar flag. It carries no operation *bodies*, no types, no grades, no grammar,
   and no way for two signatures to compose. `Signature::projection` at `vehje-signature/src/lib.rs:126-128`
   returns its argument unchanged, which is a method with the name of the canon's Frontier F ("one signature,
   many projections") and the behaviour of nothing. That is exactly the failure the negative catalogue names
   at Part H: "surface existence standing in for mandate satisfaction". This crate must grow by roughly six
   times and is the first thing on the path.

3. **`vehje-runtime-gen` (146 lines). Load-bearing in principle, a hash fold in fact.** `generate` at
   `vehje-runtime-gen/src/lib.rs:108` ignores its `Signature` argument entirely (it is named `_signature`) and
   returns the caller's own slices plus an xxhash of them. The doc at `:58` says the package is "the data the
   runtime's comptime specialisation reads". There is no comptime specialisation: `grep -c comptime
   mock/runtime-zig/src/*.zig` is 0 in every file. So the producer does not produce and the consumer does not
   exist. This is the whole of the certified-generation mechanism (canon positive:29, Frontier E) and it is
   zero lines on both ends.

4. **`vehje-typecheck` (830 lines, 314 of them tests). Not load-bearing where it stands.** `check` at
   `vehje-typecheck/src/lib.rs:198` takes `arena: &Arena, root: NodeRef` and its own doc at `:281` reads "The
   graded check pass over a program's IR". That is a Rust per-script analysis, which negative:38 kills by
   name. The *algorithm* is right and wanted; its locus is not, and the sketch has already re-implemented the
   equivalent job in `check.zig` on the correct side. Two notes on state, since the canon's own Part H is now
   partly stale: the discarded `GradeTable::set` failure is **fixed** at `vehje-typecheck/src/lib.rs:369-372`,
   which returns `CheckError::GradeRegionFull`; the recursion-on-IR-depth bug is **not** fixed, `infer` still
   calls itself at `:321` on the `Handle` arm and throughout. And `binding: Knowledge::empty()` and
   `assurance: Assurance::Unclaimed` are still hard-coded at `:366-368`, so the graded spine is still the two
   axes the canon admits to (negative:89), not four.

5. **`vehje-resolve` (485 lines). Not load-bearing where it stands.** Name resolution over a program's binders,
   in Rust. Same violation as item 4. The sketch does name resolution inside inference (`check.zig`) and the
   sketch's own README notes that this stops working when `use` and multi-segment paths arrive, which is a
   real finding about the Zig side, not a defence of the Rust side.

6. **`vehje-lower` (1,101 lines, 345 of them tests). Not load-bearing where it stands.** Constant folding, CSE,
   and the ANF catamorphism (`vehje-lower/src/anf.rs`) over a program's nodes, in Rust. Worth recording that
   the canon's claim at §11 and negative:90 that "`Anf` and `MacroExpand` are no-ops" is now stale: `anf.rs`
   is a real 263-line node-creating catamorphism. It is real, it is good, and it is on the wrong side.

7. **`vehje-runtime-abi` (2,003 lines, 165 of them tests). Split.** The wire *layout*, the value image, the
   tags and the entry contract are load-bearing and are the one interface that outlives every mechanism.
   `serialize` at `vehje-runtime-abi/src/wire/serialize.rs:312` takes a concrete program's `arena` and `root`
   and is therefore per-script Rust: not load-bearing. The CFG residual types are still defined and never
   constructed, with the FIXME admitting it at `vehje-runtime-abi/src/wire/residual.rs:121`; 273 lines of type
   surface with no producer.

8. **`vehje/src/lib.rs` (392 lines). The `run` entry is not load-bearing; the rest is thin.** Its own doc at
   `:60` is the clearest single admission in the tree: "Run a program from its IR through the **per-program
   compile path**". `run` at `:72` sequences resolve, check, mint, emit over a program root. That function is
   the dual-locus shape negative:38 killed. The `Grammar` trait at `:48` is shaped for Rust reading source
   (`fn lower(&self, arena: &mut Arena) -> Outcome<NodeRef, Self::Error>`), which presumes the same locus.
   `DebugTarget` at `:123` is a target living in the framework crate; it is defensible as the framework's own
   reference target and I flag it only as low severity.

9. **`vehje-codegen` (201 lines). Half load-bearing.** The inclusion typestate and `check_for` are definitional
   and belong. `fold_core` is a per-program fold and does not.

10. **`vehje-fixpoint` (591 lines). Not load-bearing today, by measurement.** Zero source callers anywhere:
    `grep -rn vehje_fixpoint mock/crates` returns nothing outside the crate, while
    `mock/crates/vehje-lower/Cargo.toml:12` and `mock/crates/vehje-typecheck/Cargo.toml:12` both declare it as
    a dependency. Canon negative:90 lists it as intended-not-shipped. Its two justified consumers (lease
    inference, load verification, canon §4) are both runtime-side under negative:38, so it is 591 lines of
    Rust waiting for consumers that the canon puts in Zig.

11. **`vehje-schedule` (77 lines). Should not exist, by a ratified decision that was not executed.** op, PE1,
    `op-ratification-answers.md:243`: "DECISION: dissolve the crate. Delete `vehje-schedule`, hardcode the
    4-pass compile sequence in `vehje::run`, drop the unused dep." The crate is still on disk, still a
    workspace member at `mock/Cargo.toml`, and the unused dep is still declared at
    `mock/crates/vehje-runtime-driver/Cargo.toml:11`.

12. **`mock/runtime-zig/src` (2,155 lines). Load-bearing, correct locus, and it passes the designer's
    language-agnosticism test.** I tested statement 2 directly. There is no Clause vocabulary anywhere in it:
    the node tags at `runtime.zig:53-64` are the Core forms, and `TAG_RAW` at `runtime.zig:410-413` carries the
    comment "The runtime does not know what the family means", dispatching to whoever declared it. That is a
    genuine generic shared core and it is licensed. Two things are wrong with it, both structural rather than
    incidental, and both in section 5.

13. **The Zig sketch (5,646 lines, 158 passing tests). The closest thing in the repository to the real
    artifact, filed as throwaway.** It is the only place where source reaches a value with neither Rust nor a
    host in the path, and it is the only place where Clause has traits with coherence and supertraits,
    associated types, monomorphisation by specialisation, enums with real exhaustiveness, row-polymorphic
    records, modules as records, `while` and `for` and `loop` desugared to recursive bindings, `break` /
    `continue` / `return` / `?` on one handler discharge, and 89 lines of standard library written in Clause.
    By `.claude/rules/cl-claim-sketch-discipline.md` a sketch is audit trail. This one is a prototype of the
    shipped artifact and the classification is wrong for it. Its README is stale in three places against its
    own body (it says "40 tests" at `README.md:29` and `:689` while 158 pass; it lists `loop`, row
    polymorphism, modules and monomorphisation as absent at `:633-636` while the body documents all four; and
    "Still owed on the checker itself" says the check gate is unwired while `eval.zig:615` and `:679` call it).

14. **`mock/benches` (about 60 cells). Load-bearing and the best-evidenced part of the project.** The canon's
    Frontier H rests on it, the corrections in negative Part F were found by auditing it, and it is the reason
    the architecture calls survived. Nothing here needs changing except the three owed cells (the native
    reconciling measurement, the cost-model k-sweep, the CR1 representation fork op ordered run "now" at
    `op-ratification-answers.md:246`).

Summed: about 3,700 lines of the 8,323 in `mock/crates` (items 4, 5, 6, 10, 11, plus `serialize` and `run` and
`fold_core`) are per-script Rust or ratified-dead, and a further roughly 275 lines (items 2 and 3) are
placeholders standing where the load-bearing mechanism should be. The remainder, chiefly `vehje-ir` and the
wire layout, is right.

## What is missing from the complete set, in dependency order

Zero comes before one because it sizes everything after it.

0. **Settle the grammar locus (designer's call, section 7 item 1).** Whether the shipped runtime carries one
   general grammar interpreter that reads each language's grammar as data, or each consumer ships a
   hand-written front end compiled against the engine, is the difference between about 2,000 lines written
   once and about 9,500 lines written per language. `vehje-signature/src/lib.rs:85-91` offers both variants and
   says nothing about where a hand-written one runs.

1. **A signature that carries a language.** Operation bodies in the closed primitive encoding the four-way
   projection sketch proved admissible, operation types, the grade vocabulary, the grammar attachment, the
   target declarations, and the composition rule for two signatures. Everything downstream reads this.

2. **`compile_language` in Rust: extract the specialisations.** The FIXME at `vehje/src/lib.rs:106-113` names
   it and declines to write it. This is the designer's third statement made real, and it replaces `run` rather
   than sitting beside it.

3. **The Zig comptime specialisation stage.** Zero lines exist. This is the first Futamura projection as a
   build step (canon Frontier A and E), the LanguageAuthor point of the lattice, and the entire content of
   "certified generation, data not source". `mock/research/sketches/202607260800_four-way-projection` proves
   the encoding is evaluable at all four discharge sites, byte-identical, at a five-opcode scale. Scaling that
   from five opcodes to a language is the single biggest unbuilt piece in the project.

4. **Family operations computed in the runtime.** Kill the host callback at `runtime.zig:414-430`. op's
   standing call 2 is unambiguous and the current shape is the exact one it corrects.

5. **The compound value boundary.** The census calls this "the widest unblock" and it gates records,
   sequences, tuples and enum variants, which gates `Project`, which gates modules-as-records, which gates the
   stdlib. Design it together with the value representation, per the census's closing paragraph.

6. **The engine's per-script passes, table-driven.** Resolve, check, monomorphise, lower, evaluate, all in the
   generic core reading the specialised tables rather than hard-coded Clause knowledge. The sketch is the
   semantic specification for these; it is not the code shape, because the code shape must be agnostic.

7. **`Handle`'s cheap discharge, and non-local control flow on it.** Census item two. The sketch already
   demonstrates `return`, `break`, `break v`, `continue` and `?` on the never-resuming discharge, so this is a
   port and a generalisation rather than a research problem.

8. **The pattern representation in the Core.** Census item three. Its trigger has fired and the sketch says so
   explicitly, offering its own choice as input and not as a decision.

9. **The binding-time axis populated.** `Knowledge::empty()` at `vehje-typecheck/src/lib.rs:366` is the reason
   the graded spine is two axes and not four, and macros are blocked on it rather than on a missing form
   (census item four).

10. **Syntactic macros over token streams.** The sketch has the constant-evaluation half and says so; the
    token-stream half with hygiene is unbuilt and the normative grammar requires it.

11. **The Clause standard library, in Clause, over the full grammar.** op's acceptance condition, and it is
    one condition and not three. 89 lines exist, written against the subset, with `0 < 1` standing in for
    `true` because boolean literals had not landed. That line is the best single measure of how far there is
    to go.

12. **Cross-language interop defined in the Rust artifact.** The designer's first statement. Nothing in the
    tree implements or designs how two consumers' family sets and effect sets compose, or what it means for
    two languages to "work between each other". `vehje-signature` has no notion of two signatures at once.
    This is a missing artifact, not a missing feature.

13. **The three owed benches.** The CR1 continuation representation fork op ordered run now (PE2), the N-buffer
    arena fork with the `NodeRef` branding decision riding it (XVII7), and the native reconciling cell
    (canon §11).

14. **Diagnostics as a first-class budget discipline.** Canon Frontier J calls it real architecture on a
    no-alloc substrate and names the risk precisely: it is the first place an allocator gets smuggled in.
    Nothing exists.

## Unlicensed mechanisms found

Stated plainly. Each names the canon text and the source.

1. **The Rust per-script compile pipeline should not exist.** Canon positive:45 says the Rust artifact "never
   sees an end-user script"; canon negative:38 kills the dual-locus framing with "Rust never runs per-script
   analyses, the runtime does, for all scripts". `mock/crates/vehje/src/lib.rs:60` documents its own entry as
   "the per-program compile path" and `:72` implements it. `vehje-typecheck/src/lib.rs:198` and `:281`,
   `vehje-resolve/src/lib.rs`, `vehje-lower/src/lib.rs`, and
   `vehje-runtime-abi/src/wire/serialize.rs:312` are the passes it sequences. This is about 3,700 lines and it
   is the largest single body of work in the repository. It cannot ship, it duplicates passes the Zig sketch
   already implements on the licensed side, and every hour spent maintaining it is spent twice on the same
   semantics in two languages. That is not a difference of approach. It is the design boundary being on the
   wrong side of several thousand lines of finished-looking, tested code, which is precisely why nobody
   noticed: the tests are green, and green certifies the increment and says nothing about the premise.

2. **The host supplies arithmetic.** op's standing call 2 (`202607260100_op-standing-design-calls.md:29-48`):
   "Host should only basically configure the runtime and give it the allocations, nothing more... it should
   *NOT* have to give the simple things like fucking arithmetics". `mock/runtime-zig/src/runtime.zig:414`
   reads `const h = host orelse return EvalError.NoHost;` and `:430` calls `h.call(...)` for every family
   operation. The memo records this as already corrected in principle and it is uncorrected in source. Note
   the trap the memo also names: moving the handler into the runtime as a default while keeping the callback
   shape is not the fix.

3. **`vehje-schedule` exists after a ratified decision to delete it.** op, PE1,
   `op-ratification-answers.md:243`. Still at `mock/crates/vehje-schedule/src/lib.rs` (77 lines), still a
   workspace member in `mock/Cargo.toml`, unused dep still at `mock/crates/vehje-runtime-driver/Cargo.toml:11`.
   A ratified deletion that did not execute is worse than an open question, because a reader finds a crate
   with a DESIGN and infers it is wanted.

4. **`Signature::projection` is a method that does nothing, carrying the name of a canon frontier item.**
   `vehje-signature/src/lib.rs:126-128`: `pub fn projection(&self, which: Projection) -> Projection { which }`.
   Canon Frontier F is "one signature, many projections". Negative catalogue Part H names this exact species:
   "a type or a crate exists (so the doc-source-mismatch lint passes on the noun) but does not do what its doc
   says". Delete it or implement it; leaving it is a false claim of completeness that a future reader will
   build on.

5. **`generate` ignores the signature.** `vehje-runtime-gen/src/lib.rs:108` binds it as `_signature`. The
   canon's whole Rust-side mandate is to compile a language definition into data. This function is named for
   that job, is documented as "the entry the dev-time language compiler calls", and passes the caller's own
   bytes through a hash.

6. **The comptime specialisation stage does not exist at all.** Canon §3: the shipped artifact is "one
   hand-authored engine, **comptime-specialised to the Rust-emitted data**"; canon Frontier A places
   ahead-of-time work at LanguageAuthor as "the first Futamura projection as a build step". Measured:
   `grep -c comptime` returns 0 for every file in `mock/runtime-zig/src/`. Nineteen occurrences exist in
   `mock/research/sketches/202607260900_clause-in-zig/eval.zig` and the mechanism is demonstrated in
   `202607260800_four-way-projection`, both outside the shipped tree. The central mechanism of the project's
   own identity has zero lines in the artifact that is supposed to carry it, while 8,323 lines exist in the
   artifact that is supposed to feed it.

7. **`vehje-fixpoint` is 591 lines with no caller and two declared dependents.** Verified by grep. Its two
   canon-justified consumers, lease inference and load verification (canon §4), are runtime-side work under
   negative:38.

8. **The CFG residual is 273 lines of type surface with no producer**, admitted at
   `vehje-runtime-abi/src/wire/residual.rs:121`. Canon §11 already lists this; it is still true, and it is
   downstream of ANF, which is itself on the wrong side per item 1.

Two smaller ones, recorded because they will mislead a reader. The canon's own negative catalogue Part H is
now stale in two rows: the discarded `GradeTable::set` failure is fixed at `vehje-typecheck/src/lib.rs:369-372`,
and `Anf` is no longer a no-op (`vehje-lower/src/anf.rs` is a real catamorphism). And the Zig sketch's README
contradicts its own body in three places (`README.md:29`, `:633-636`, `:689` and the "Still owed on the
checker" section, against `eval.zig:615`). A stale honest document is a smaller problem than a confident
wrong one, but both cost the next reader a verification pass.

## Where the canon is genuinely ambiguous

Three, each with the tradeoff and no preference of mine. The first sizes the project.

1. **Where a hand-written grammar runs, and therefore whether the shipped runtime carries one general parser
   or N language-specific ones.** The designer's live statement licenses "a very generic, not-specified shared
   runtime... as long as it does not expect any kind of language, and will work with anything". Canon §3 says
   the shipped artifact is "one hand-authored engine, comptime-specialised to the Rust-emitted data", and
   `vehje-signature/src/lib.rs:85-91` offers `GrammarHook::HandWritten` and `GrammarHook::AttributeGrammar`
   without saying where the hand-written case executes. Reading A: the engine carries a general
   attribute-grammar interpreter and Clause's grammar arrives as data, so the parser is written once and every
   consumer gets it, at the cost of a grammar formalism that must be expressive enough for Rust-shaped syntax
   including the struct-literal ambiguity and macro invocation. Reading B: each consumer ships its own front
   end against the engine's API, so `mock/research/sketches/202607260900_clause-in-zig/clause.zig` is exactly
   the right artifact in the wrong directory, at the cost of about 9,500 lines per language and of the shared
   core being genuinely agnostic only because the language-specific part sits outside it. Cost delta is
   roughly 2,000 lines once against 9,500 lines per consumer, over a census of ten-plus consumers.

2. **Whether the shipped runtime contains the type checker.** Canon positive:29 says the artifacts "run with
   no prover", and canon §2A says "one shipped artifact, the runtime, contains a compile stage, a debt the ABI
   design already confesses", placing load-time compilation at HostLoader. Canon negative:38 puts every
   per-script analysis in the runtime. If the check is a per-script analysis, the runtime contains a prover
   and "no prover" is a per-stage claim about Runtime rather than a property of the artifact. If it is not,
   something must check a script and nothing on either side is licensed to. The tradeoff: a checking runtime
   is honest about prove-then-erase happening at load and pays binary size and load latency for it; a
   non-checking runtime needs a third artifact or a trusted producer, and a trusted producer is the emitter
   seam the canon spent Frontier F trying to close.

3. **What a Rust role at Bundler binding time sees, if not a script.** Canon Frontier A places ahead-of-time
   work at "Bundler (the Rust side inlining a known constant)", and `vehje-lower/src/lib.rs` implements the
   const-fold that reading implies. A constant known at bundle time is a constant in some program, so the
   clause reads per-program, and positive:45 says Rust never sees one. Either Frontier A's parenthetical is
   the stale half and Bundler-stage folding belongs in the runtime's load-time compile stage, or positive:45's
   "script" is narrower than "program" and there is a Rust-visible bundle-time artifact the canon has not
   named. The tradeoff is whether the four-point lattice has a Rust occupant at two points or one, which
   decides whether `vehje-lower`'s fold and CSE have any licensed home in Rust at all, and therefore whether
   item 1 of section 5 is 3,700 lines or 2,600.
