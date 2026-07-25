# The complete vehje through the staging lens (Tiark Rompf)

## Canon gate outcome

**Aligned, with one bounded ambiguity isolated and handed back.**

Checked against `mock/research/canon/the-soul-of-vehje-positive-catalogue.md` and
`the-inverse-of-vehje-negative-catalogue.md` (both read in full before any code),
`mock/research/202607260100_op-standing-design-calls.md`, and the ratification record at
`mock/research/202607241400_ir-to-ir-and-the-compile-runtime-line/op-ratification-answers.md`.
I did not open `202607241545_topic.the-vehje-canon.md`; nothing below cites a section
numbered above 8 in either catalogue.

Charting the path is licensed: the canon fixes the artifacts (`positive:45`), the stages
(`positive:21`), and the discharge rule (`positive:13`), and asks for exactly the ordering
work this dispatch assigns. The state it builds on is misaligned in ways already ruled on,
which I re-verify independently below rather than inherit.

**The one point I refuse to resolve, because the canon's letter contradicts itself on it
and my answer's shape depends on it.**

`positive:21`: "Ahead-of-time work lives at LanguageAuthor ... and at Bundler (**the Rust
side inlining a known constant**)."

`positive:45`: a dev-time Rust compiler that "**never sees an end-user script**, never
ships."

`negative:28`: the lattice exists rather than a chain "because a **partly-bundled
partly-arriving script** is a join, not a point on a chain."

Bundling is a script-level operation. If the Rust side performs it, Rust sees a script. If
it does not, `positive:21`'s parenthetical is wrong and needs a superseding round, because
`positive:5` makes the letter govern and forbids amendment by appeal to spirit. Standing
call 10 (`202607260100:147`, "Rust does not run per-script passes") post-dates the canon
and pushes toward the second reading, but a standing call is not a superseding round and
does not silently edit `positive:21`.

This is not a stylistic tension. It decides whether the operation-semantics representation
must be **evaluable by rustc-compiled Rust** or only by Zig at two binding times, which is
the single largest constraint on the schema that is the keystone of everything below. I
mark every conclusion that depends on it. Section "Where the canon is genuinely silent",
item A.

## Verdict in one line

The design's staging story is coherent and the hard half is already proven runnable, but
the tree has its binding-time boundary in the wrong place twice over: the artifact that
runs at our build computes over inputs it will never have (a script), and the artifact
that runs at the consumer's does not consume the one input it was designed around (the
language definition), so the only seam the whole design exists for, `mix` applied to the
generic core with the language definition as the static argument, is a zero-line hole
between a producer that ignores its argument (`vehje-runtime-gen/src/lib.rs:108`) and a
consumer with no `comptime` anywhere in it (`grep -c comptime mock/runtime-zig/src/runtime.zig`
returns 0).

## What the canon calls for, as a staging problem

Four binding times, two artifacts. That asymmetry is the design, and every artifact
boundary question is the question of which lattice points a given artifact spans.

> "The 'line' between compile and runtime is not a line; it is the lattice
> `{LanguageAuthor, Bundler, HostLoader, Runtime}`" (`positive:21`; the type is
> `vehje-ir/src/grade.rs:105-115`, which I read.)

> "It ships two artifacts with Rust in only one: a dev-time Rust language compiler (emits
> validated data and structural proofs, never sees an end-user script, never ships) and a
> shipped Zig composed runtime (one hand-authored engine comptime-specialised to the
> Rust-emitted data)." (`positive:45`)

> "discharges every operation it knows ... with one handler discipline **at the earliest
> binding time that operation's inputs are known**" (`positive:13`)

### The five seams, what crosses each, and in what form

**Seam 1, language definition into the Rust artifact.** Binding time LanguageAuthor. The
input is what a language author writes. Today's carrier is `Signature`
(`vehje-signature/src/lib.rs:99-107`): operations, target declarations, grammar hook.

**Seam 2, Rust artifact out to the language package.** Still LanguageAuthor grade: the data
is fully known at our build, which is precisely what makes `positive:29`'s "Both artifacts
are certified at *our* build, neither at the consumer" true rather than aspirational. The
form is fixed by canon as data, not source: `negative:37` kills "Rust emits Zig source" and
`positive:29` names the LMS-hard emission route as rejected. What must cross, derived from
the consumers rather than from the current struct:

1. Per-operation **semantics**, in a form some later stage can evaluate. `Operation`
   (`vehje-signature/src/lib.rs:25-32`) carries `family`, `effect`, `lease` and **no such
   field**. This is the hole.
2. Per-operation effect grade and lease rule. Present.
3. The **introduction forms** (constructors), which `positive:31` names as one of the three
   projections of the one algebra ("the introduction value-domain, content-as-values").
   Absent from the schema entirely.
4. The wire layout, the lowering rule table, the target declarations, the grammar hook.
   `SliceKind` names all four (`vehje-runtime-gen/src/lib.rs:29-40`).

**Seam 3, package into the Zig build's comptime.** This is `mix`. It is the whole of
Frontier A and it does not exist: zero `comptime` in the shipped runtime. The staging point
that is not stated anywhere in the tree and that governs the entire build: **we do not have
to write `mix`. The Zig compiler is `mix`.** That is what makes this design cheap and it is
the structural advantage over the closest production comparable. weval (Fallin, PLDI 2025)
had to build an SSA-level partial evaluator over a CFG because it takes existing,
almost-unmodified C/C++ interpreters and cannot restructure them; it earns 3-5x over the
generic SpiderMonkey interpreter for that trouble. vehje authors its own engine, so it can
write the two-level interpreter directly and get the specialiser from the toolchain. The
canon already cites weval as the precedent (`positive:21`); the correct reading of the
precedent is that we get its result without its machinery, and the cost is a discipline
weval does not need: **the generic core must be written in two-level style or the
specialisation is simply not reachable.**

**Seam 4, host into runtime.** Configuration, lent allocations, and a script image.
Standing call 2 (`202607260100:32-36`) fixes it: "Host should only basically configure the
runtime and give it the allocations, nothing more."

**Seam 5, runtime out to host.** The value-arena image through reserve/commit.

### Where the two-level split actually falls inside the runtime

This is the part no artifact states and it is the design decision the core's source shape
depends on. In `eval(comptime lang, img, ...)`:

- `lang` is **LanguageAuthor grade** and therefore `comptime`. The operation table, the
  operation bodies, the value domain, the effect masks, the target sets. Specialised away
  entirely: the dispatch over families becomes a jump table with one arm per operation, and
  each arm's body-walk vanishes.
- `img` is the **script**, arriving at HostLoader or later, and therefore a runtime
  parameter. It stays interpreted or gets lowered by the load-time stage.

That is `Rep[T]` versus `T` with the marker being Zig's `comptime` keyword rather than a
type constructor, and it is the entire content of "one hand-authored engine
comptime-specialised to the Rust-emitted data". Both halves are already demonstrated, in
one running file: `mock/research/sketches/202607260900_clause-in-zig/eval.zig:95-99` is
`inline for (OPS, 0..)` over a comptime table calling `runOp(prog, args)` at `:45` where
`prog` is `comptime`, while `img` stays a runtime `[]const u8`. The script is data; the
language is not. That is correct and it is the shape to build.

### What the binding-time *analysis* is and is not for

A correction that reorders the whole path, and it inverts what several unratified memos in
this tree recommend.

The LanguageAuthor-versus-everything split needs **no** binding-time analysis. It is
structural: a parameter is `comptime` or it is not. `proj.zig` specialises with no grade
computed anywhere, and I reproduced it. So the fact that `check` hard-codes
`binding: Knowledge::empty()` (`vehje-typecheck/src/lib.rs:367`, canon cites 311-312 at
`positive:27`, drifted) **does not block the keystone**.

The analysis is needed exactly where the split is not syntactically evident, which is the
join `negative:28` names: a partly-bundled partly-arriving script, whose constants may
become known at Bundler, at HostLoader, or only at Runtime. That is the load-time
compilation stage (`positive:21`, "Load-time compilation, not profile-guided JIT, lives at
HostLoader"), and the grade is a prerequisite for **that** and only that.

One defect in the axis that must be fixed before it computes anything. `Knowledge`
(`grade.rs:119-148`) is a free subset mask over four slots, so `{LanguageAuthor, Runtime}`
without the middle two is representable. Binding-time availability is monotone: known early
implies known late. "Earliest binding time its inputs are known" (`positive:13`) is
well-defined only on a chain. `EffectMask` in the same file is already documented as
"Thermometer-shaped so the lattice join is a bitwise union" (`grade.rs:21-22`); `Knowledge`
is not. This is a type-level fix that makes the illegal state unbuildable, which is the
discipline PE3 already mandates for grade regions (`op-ratification-answers.md:249`).

## The dependency order, and the keystone

Read as a staging problem rather than a feature list. Each item names what it unblocks.

**0. Remove the wrong-stage artifact.** `vehje::run` (`mock/crates/vehje/src/lib.rs:72-104`)
takes an `Arena` and a program `root` and runs resolve, check, mint, emit over it. Its own
doc comment at `:60` calls it "the per-program compile path". `negative:38` kills the dual
locus by name: "Rust never runs per-script analyses, the runtime does, for all scripts."
Already ruled by op on 2026-07-26. The count from my own `wc`: `vehje-resolve` 485,
`vehje-typecheck` 830, `vehje-lower` 1101, `vehje-codegen` 201, `vehje-fixpoint` 591 (no
in-tree callers, `negative:90`), `vehje-schedule` 77 (ratified deleted, still on disk), plus
the per-script `encode` walk inside `vehje-runtime-abi`. Roughly 3,300 to 3,700 lines.
This is item zero rather than cleanup because while it exists every "where does X live"
question has two plausible answers, including the ambiguity in my gate section.

**1. Fix the operation-semantics representation, at a coverage that includes introduction
forms.** The keystone. Detail below.

**2. Connect seam 2 to seam 3.** `generate` reads the signature and emits the FamilyTable
slice with bodies; a `build.zig` step makes it a comptime constant. Today `generate` binds
its argument as `_signature` (`vehje-runtime-gen/src/lib.rs:108`) and `OPS` in the sketch is
a hardcoded Zig literal (`eval.zig:30-41`). Both ends of the seam exist as shapes and the
wire between them does not. Blocks: every one-definition claim, the differential check
(`vehje-runtime-gen/src/lib.rs:92-96`), and the reduction `positive:29`'s trusted-rim
enumeration is written to buy (a check between two *evaluations of one definition* rather
than between two hand-written implementations).

**3. Make the generic core two-level.** `eval(comptime lang: LanguageDef, img, ...)`,
`applyOp` a comptime-generated `switch` rather than a linear compare chain, arenas taken
from the host rather than provisioned in the entry, and the evaluator defunctionalised.
Blocks: any measurement of the specialisation, and the truth of `positive:45`.

**4. The harness cells.** Specialisation explosion at distinct bodies, warm crossover
against the switch floor, comptime envelope at realistic operation-count times body-size.
Not before 3, because there is nothing to time.

**5. Thermometer-encode `Knowledge` and compute it.** Unblocks the HostLoader stage.

**6. The HostLoader load-time lowering.** `RuleTable` (`vehje-lower/src/lib.rs:95-101`) is
today two booleans, `const_fold` and `cse`, and `:92-94` claims it is "the same table
`vehje-runtime-gen` packages for the runtime's load-time lowering of arriving scripts". Two
bits do not carry a lowering. Needs 1, 2, 5.

**7. Macro expansion.** The completeness census reaches the same conclusion from the
grammar side that staging reaches from the lattice side: "macros are not blocked on a
missing Core form; they are blocked on an axis the graded spine already declares and does
not yet populate" (`202607251530_language-completeness-census.md:63`). Needs 5. The N-buffer
arena fork (`negative:82`, owed) is downstream of this, because there is no node-creating
catamorphism to allocate for until expansion exists at the right locus.

**8. The rest of the grammar bar, then the stdlib in Clause.** Standing calls 1 and 3.

### The keystone

**The language-definition schema, specifically what an operation's semantics is written
in, at a coverage that includes constructors.** Items 2 through 8 are all consumers of it;
item 0 is a deletion.

Why it is that and not the binding-time axis, stated as the falsifiable claim: every other
missing piece has a consumer relationship to this one and none of them has it in the other
direction. The Zig comptime specialisation has nothing to specialise against without it.
The runtime checker needs operation *types*, and the clause-in-zig sketch already draws
them "from the same signature data that carries the operation bodies" (its README). The
load-time folder needs to know what `+` means over literals. The differential check needs
one definition to check two evaluations of. And `positive:31`'s "One declarative signature
... is the single source" is false for computation the moment the meaning lives anywhere
else.

The coverage qualifier is the part that is easy to drop and must not be. The four-way sketch
proved admissibility for a five-opcode **scalar** vocabulary and says so in its own
"Does not establish" (`findings.md:73-76`). The clause-in-zig sketch then hit the wall and
hand-authored the compound operations directly in Zig: `eval.zig:483` onward matches
`OP_MAKE_REC`, `OP_MAKE_SEQ`, `OP_LEN`, `OP_AT`, `OP_PUSH` as opcodes with hand-written
bodies, and `OPS` carries five empty entries (`eval.zig:34-39`) where those would be. Its
own comment names it: "The vocabulary has two shapes in it, and this is where that shows"
(`eval.zig:480-482`). So today one-definition holds for arithmetic and fails for every
compound value, and the census calls exactly that the widest unblock in the language:
"this single escape gates every compound value in the language"
(`202607251530_language-completeness-census.md:52`).

## What is proven, what is assumed

### Proven, each re-run by me

- **Four-way admissibility for a scalar closed vocabulary.**
  `mock/research/sketches/202607260800_four-way-projection/`. `zig test proj.zig` (Zig
  0.16.0): 2/2 pass. `zig build-lib proj.zig -O ReleaseFast` then `objdump -d libproj.a`
  reproduces the specialised export at `_proj.diffsq_specialised` as `sub`, `add`, `mul`
  and nothing else, across an `export` boundary so the optimiser cannot see the call site.
  This is the mix equation demonstrated, not asserted, at the exact site `positive:21`
  places it.
- **The whole two-level split, in one running artifact.**
  `mock/research/sketches/202607260900_clause-in-zig/`, `zig test eval.zig`: 158/158 pass.
  Comptime operation table (`eval.zig:30`), comptime-specialised bodies (`:45`), runtime
  script image. Source text to value with no Rust and no host in the path.
- **Comptime cost envelope.** `mock/benches/comptime-cost-cliff/findings.md`: O(N) content
  folding is ~27 us/record to n=20000 (918 ms); the cliff is superlinearity, not
  comptime-over-content; `@Enum` type-gen ~950 us/field at n=20000, "fine for a realistic
  language table (tens of families, sub-second)". This is the cell that says the
  LanguageAuthor stage is affordable at realistic family counts.
- **Compile-side PE cost and termination.**
  `mock/benches/partial-eval-specialization/findings.md`: ~100 to 230 ns/node, and
  block-structured (templating-shaped) residual reduction 3.8x to 7.0x against 1.2x to 2.9x
  for a random static/dynamic mix. Read the honest boundary the cell states itself: "The
  grade here is the tree depth (a simple well-founded measure); the design's binding-time
  grade is richer (a lattice)." Termination is proven for a proxy measure, not for the
  four-point lattice.
- **The floor the specialised form must beat.** `mock/benches/results/zig_dispatch/`, plus
  `positive:35`'s summary: plain switch fastest for the lean ~25-primitive straight-line IR,
  tail-threading 1.64x to 1.90x of switch.

### Assumed, with no bench and no sketch

- **That specialisation pays.** No harness cell exists. The 3-instruction versus 244-byte
  gap in the four-way sketch is a code-size observation and the sketch explicitly refuses
  to call it a measurement (`findings.md:78-83`). Standing call 5 (`202607260100:88-89`)
  and `positive:57` both say a claim without a harness cell is not evidence.
- **That specialisation does not explode code size.**
  `mock/benches/runtime-binary-size/findings.md` reports ~1 KB plus ~1 byte per family, and
  states its own scope: "trivial-identical-family case, so the family dispatch collapses
  (the compiler dedups identical arms)". It measures the dispatch skeleton. The question
  this design raises is unmeasured.
- **That the vocabulary covers a real family.** Assumed and currently contradicted by the
  running artifact, per the keystone section above.
- **That comptime cost stays inside `@setEvalBranchQuota`** at realistic operation-count
  times body-size. Untested; the cliff cell measured a different axis.
- **The Bundler leg's cost, and its existence.** Conditional on gate item A.
- **That the four-point join is well-defined.** `Knowledge` is a free powerset
  (`grade.rs:119-148`), so it is not.
- **The entire HostLoader stage.** Nothing exists to measure.

Two rows in `negative` Part H are now stale in the tree's favour and I confirm them so a
later auditor does not re-flag: the `GradeTable::set` silent-truncation bug named at
`negative:91` is fixed (`vehje-typecheck/src/lib.rs:369-372`), and `Anf` is a real
node-creating catamorphism (`mock/crates/vehje-lower/src/anf.rs`) rather than the no-op
`negative:90` names. The recursion-on-IR-depth defect from the same row is **not** fixed and
now exists on the Zig side as well (`runtime.zig:278`, `eval` calls `eval`).

## What can be proven now by a sketch, and what cannot yet

### Provable now

**S1. Extended-vocabulary admissibility including introduction forms.** Extend
`proj.zig` with constructor opcodes over a value arena; run the same four ways; `diff` the
corpus outputs. Must show: a constructor's meaning is data, evaluable at all four sites,
and the Zig comptime arm still specialises the walk away. Acceptable leeway in the result:
the opcode encoding, whether constructors are a second array or a tagged arm of one, the
arity convention. Not acceptable: a result that reaches WORKS by leaving the constructor
hand-authored on one side, which is the failure mode the current sketch already exhibits.
Conditional on gate item A only in one respect: if Bundler stays in Rust, the sketch's Rust
leg must include a `const fn` evaluation of the constructor, which constrains the encoding
much harder than the Zig legs do.

**S2. The two-level core shape.** `eval(comptime lang: LanguageDef, img: []const u8, ...)`
with `applyOp` generated as a `switch` over the comptime table and the arenas taken as
parameters. Must show three things: it compiles at a realistic family count; `objdump`
shows a jump table rather than the linear compare chain the sketch currently emits
(`eval.zig:96`, `if (prog.len > 0 and op == @as(u32, i))` inside an `inline for`, which is
O(operations) compares per application); and the script stays a runtime parameter. Leeway:
the `LanguageDef` field layout. This is the prerequisite for every cell in item 4.

**S3. The seam round trip.** Rust `generate` emits a FamilyTable byte image from a real
`Signature`; a `build.zig` step turns it into the comptime `OPS`; the S1 four-way diff
re-runs against the **emitted** table rather than a hardcoded one. Must show: `OPS` is
derived, not written. This is the grade-0 form of the differential check `positive:29`
names, and it is the first artifact in which "one signature, many projections" is true
rather than intended. The crossing mechanism is a call for op, gate item B.

**S4. Thermometer `Knowledge`.** A red test asserting `{LanguageAuthor, Runtime}` without
the intervening sources is unrepresentable, then the type change that makes it pass. Cheap,
and it is the precondition for "earliest binding time" being a defined operation rather
than a phrase.

**S5. Host-lent allocation at the entry.** `vehje_runtime_execute`
(`mock/runtime-zig/src/runtime.zig:521-542`) provisions `slots[1024]`, `vslots[1024]`,
`vpool[2048]`, `vblob[8192]`, and `buf[16384]` on its own stack. Standing call 2 says the
host gives the allocations. Fixing it is mechanical and it is a prerequisite for S2, since
the arenas become part of the runtime parameter set the specialisation must not capture.

### Not provable until an earlier step lands

- **Whether specialisation pays.** Needs S2 for a specialised runtime to time against the
  `zig_dispatch` switch floor. Design the cell against the two traps this repo has already
  paid for: cross an opacity boundary (`negative:68`, the Futamura-artifact trap, where the
  framework's own central mechanism hid inside its own benchmark) and use bodies that cannot
  be reassociated away, or the cell measures LLVM's strength reduction.
- **Anything about the HostLoader stage.** No load-time lowering exists to measure. Blocked
  behind S4 plus item 6.
- **The CR1 continuation-representation fork**, which op ordered run **now** (PE2,
  `op-ratification-answers.md:246`) and which does not exist. Partially unblocked: the
  non-resuming discharge is already demonstrated (the clause-in-zig sketch runs `break`,
  `return`, `continue`, and `?` through it), so the one-shot half is measurable today. The
  multi-shot representation fork needs the host-lent budget model first, and the budget
  model needs S5.
- **The N-buffer arena fork** (`negative:82`, owed, no precedent). Needs an IR-to-IR
  expansion pass to exist at the correct locus. Today `MacroExpand` is a FIXME no-op at
  `mock/crates/vehje-lower/src/lib.rs:432`, on the wrong side of the artifact boundary.

One owed sketch has lost its locus. Ratification XVII1
(`op-ratification-answers.md:27-29`) orders a sketch on "whether macro-expansion cyclicity
is statically detectable **on the Rust typestate**". Under the artifact split there is no
Rust typestate over a script. The question survives; the locus named in the ratified text
does not. See gate item E.

## Unlicensed mechanisms found

Ordered by cost. Every one verified by me in the tree.

1. **The Rust per-script compile pipeline.** `mock/crates/vehje/src/lib.rs:72-104`, self
   described at `:60` as "the per-program compile path", plus `vehje-resolve`,
   `vehje-typecheck`, `vehje-lower`, `vehje-codegen`. `negative:38`: "Rust never runs
   per-script analyses, the runtime does, for all scripts." Already ruled. Roughly 3,300 to
   3,700 lines. The cost is not the lines, it is that resolve, check, and lower now exist
   in two languages on opposite sides of a ratified boundary, one of which can never ship,
   which is the duplicated-meaning failure the whole design exists to abolish.
2. **The producer produces nothing.** `generate(_signature, slices)`,
   `mock/crates/vehje-runtime-gen/src/lib.rs:108`. The underscore is the finding. The
   LanguageAuthor stage's output is not derived from the language definition.
3. **The consumer consumes nothing.** `grep -c comptime mock/runtime-zig/src/runtime.zig`
   returns 0. `positive:45` states "one hand-authored engine comptime-specialised to the
   Rust-emitted data" in the present tense. It is false in tree, on both ends.
4. **Arithmetic is host-serviced.** `mock/runtime-zig/src/runtime.zig:414`,
   `const h = host orelse return EvalError.NoHost`. Standing call 2
   (`202607260100:32-36`): the host "does *NOT* have to give the simple things like
   fucking arithmetics". A program that adds two integers cannot run.
5. **A source comment defends the foreclosed shape.** `runtime.zig:412-413`: routing to the
   host "is what keeps the framework family-free". Standing call 2 names that reasoning
   backwards in terms (`202607260100:158-163`): "Family-freedom is a property of the core's
   *source*; it is not a reason to route computation through the host at run time." A
   comment teaching a killed rationale outlives the code it defends.
6. **The entry provisions its own memory.** `runtime.zig:521-542`, five stack buffers.
   Standing call 2: the host gives the allocations.
7. **`vehje-schedule` is on disk after a ratified deletion.** `op-ratification-answers.md:243`
   ratified it; `negative:88` states in the present tense "The crate is deleted". It is a
   workspace member at `mock/Cargo.toml:11` and `:53` and an unused dependency at
   `mock/crates/vehje-runtime-driver/Cargo.toml:11`. A canon row that reads as executed and
   is not is the exact Part H failure mode the catalogue was written to prevent
   (`negative:86`), now committed in the catalogue itself.
8. **`Signature::projection` returns its argument.** `mock/crates/vehje-signature/src/lib.rs:126-128`.
   It carries the name of Frontier F and the behaviour of nothing, so the doc-source-mismatch
   lint passes on the noun. `negative:86`, "surface existence standing in for mandate
   satisfaction", verbatim.
9. **The measured biggest lever is not in the shipped ABI.** `positive:35` calls the
   batched-column entry "the single biggest lever". `BatchedColumnEntry` is a Rust type
   alias only (`mock/crates/vehje-runtime-abi/src/entry.rs:56`, with the export FIXME'd at
   `:58-64`), and the Zig side exports the single-record `vehje_runtime_execute`.
10. **The Zig evaluator recurses on IR depth.** `runtime.zig:278`, `eval` calling `eval`.
    PE3 (`op-ratification-answers.md:249`) mandates defunctionalised non-recursive passes,
    and `positive:45` makes no-heap "an axiom on both sides". The reason for the mandate,
    host stack overflow in exactly the embedded contexts an embeddable framework courts, is
    artifact-independent. Present in the sketch too, which names it honestly ("Loops are
    bounded by the evaluator's stack ... task #49").
11. **The Zig toolchain is unpinned.** No `build.zig.zon` and no `.zig-version` anywhere in
    the repo, against a committed `rust-toolchain.toml` pinning `nightly-2026-05-28`. Every
    Zig number in this round, including the two sketch runs I reproduced above, is
    reproducible only against whatever `zig` is on the runner's path. `positive:57` makes
    re-runnability the point of the harness discipline.
12. **Two findings documents whose cells carry no data.**
    `mock/benches/dispatch-heavy-ops/heavy.csv` and `mock/benches/interp-dispatch/dispatch.csv`
    are header-only while their `findings.md` report full tables, and both are standalone
    cells outside the harness, against standing call 5 (`202607260100:88-89`) in terms that
    name the case.
13. **`vehje-fixpoint`, 591 lines, no in-tree callers** (`negative:90` names it and it is
    still true).
14. **The clause-in-zig sketch hand-authors five family operations in Zig**
    (`eval.zig:483` onward). Nominal binding for computation the language owns. It is a
    sketch, so it is audit trail rather than shipped drift, but the shape must not
    graduate: `positive:31` makes one signature "the single source", and a meaning that
    lives in hand-written Zig is a source no projection relates to the other two.
15. **The clause-in-zig README contradicts itself.** Its "What it does not establish"
    section lists `loop`, row polymorphism, multi-method traits, and monomorphisation as
    absent; earlier sections of the same file demonstrate all four with passing examples. A
    reader taking that section as the honest ledger undercounts what exists by a large
    margin. Unratified prose, but it is the document a future round will cite.
16. **Canon citation drift.** `positive:27` cites the hard-coded grade axes at
    `vehje-typecheck/src/lib.rs:311-312`; they are at 367 and 368. The fact is unchanged;
    the pointer is not, and a canon whose pointers do not resolve stops being checkable.

One more, aimed at myself. My own prior deliverable in this tree,
`mock/research/202607241400_ir-to-ir-and-the-compile-runtime-line/tiark_rompf_staging-and-the-line.md`,
recommends a fixpoint expansion driver, macro hygiene via a gensym threaded through
`Builder`, and folding CSE into a hash-consing `Builder`, all sited **in the Rust
artifact**. It predates the artifact-split calls. On the locus axis it is now drift and
should not be built as written. Its finding 1 (thermometer-encode `Knowledge`) and its
finding 0 (the manifest hash must be the byte-image hash, not the interner-local structural
one, because it crosses the interner boundary) survive the relocation and are still owed.

## Where the canon is genuinely silent

Five calls. Each with its tradeoff and no preference of mine.

**A. The Bundler locus.** Stated in full in the gate section. `positive:21` puts Bundler AOT
at "the Rust side inlining a known constant"; `positive:45` says Rust never sees an end-user
script; standing call 10 says Rust runs no per-script passes; `negative:28` makes bundling a
script-level join. Keeping Bundler in Rust preserves the four-point lattice as written and
imposes a hard constraint on the keystone schema, that the representation must be evaluable
by rustc-compiled Rust at both const and runtime, which rules out several encodings on the
spot. Moving Bundler into the Zig artifact collapses Rust to a single lattice point,
narrows the admissibility test to the two Zig legs, and requires a superseding round to
amend `positive:21`, since PE4 (`op-ratification-answers.md:252`) makes the letter govern
and forbids resolving this by appeal to intent. This is the call the schema's shape depends
on and it should be made before S1 is designed.

**B. The form of the crossing at seam 3.** `negative:37` kills "Rust emits Zig source",
described there as "a Rust metaprogram text-printing the specialised engine". A generated
file containing only `pub const OPS = ...` is not the specialised engine, but it is Zig
source, and the kill's letter names the engine rather than the medium. A byte image decoded
at comptime keeps one wire format and keeps the byte-image hash as the cross-artifact
identity `positive:31` requires, at the cost of comptime decode work measured at roughly
27 us per record; a generated data-only const file is free at comptime and trivially
auditable, at the cost of a second emitter surface and Zig syntax inside the trusted base
`positive:29` enumerates.

**C. Whether the closed vocabulary must cover introduction forms.** `positive:31` names "the
introduction value-domain (content-as-values)" as one of three projections of the one
algebra, which reads as putting constructors inside the one signature; nothing states a
coverage obligation on the vocabulary that carries them. A vocabulary covering constructors
keeps one-definition across the census's compound escape, which the census calls the widest
unblock in the language, at the cost of a substantially larger primitive set and a harder
admissibility proof. Hand-authored constructors keep the vocabulary small and provably
admissible and write every compound value's meaning twice, once per artifact. This decides
whether item 1 is one mechanism or two, and the running sketch has already taken the second
road by default rather than by decision.

**D. Whether `Knowledge` is thermometer-ordered.** The canon calls the lattice four points
and a join and never states monotonicity. "Discharge at the earliest binding time"
(`positive:13`) is defined only on a chain. Thermometer encoding makes "earliest" the lowest
set bit and makes illegal grades unbuildable, matching the discipline PE3 mandates
elsewhere, at the cost of forbidding a non-monotone knowledge source should one ever be
wanted. The free powerset is more general and leaves the canon's own discharge rule
undefined on part of its own domain.

**E. The locus of the owed macro-cyclicity sketch.** XVII1 ordered it "on the Rust
typestate" and fixed the fallback as "a host-configurable recursion-depth cap, default 255"
(`op-ratification-answers.md:16-29`). Under the artifact split there is no Rust typestate
over a script. Either the static-detection question moves to the runtime's expansion stage,
where it is a different and probably harder problem, or the depth cap becomes the mechanism
outright rather than the fallback. The ratified text names Rust, so this is an amendment
rather than an inference.

## Sources

- [weval, the WebAssembly partial evaluator](https://github.com/bytecodealliance/weval)
- [Fallin, Partial Evaluation, Whole-Program Compilation, PLDI 2025](https://cfallin.org/pubs/pldi2025_weval.pdf)
- [Compilation of JavaScript to Wasm, Part 3: Partial Evaluation](https://cfallin.org/blog/2024/08/28/weval/)
