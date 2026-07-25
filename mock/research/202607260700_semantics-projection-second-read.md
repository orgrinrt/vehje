# Semantics Projection: The Second Independent Read

**Date:** 2026-07-26
**Status:** unratified agent output. A second independent read of the question posed by
`mock/design_rounds/202607260500_topic.the-languageauthor-specialisation-stage.md`.
**Ordering discipline:** Section 2 was written and saved before
`mock/research/202607260600_semantics-projection-design-space.md` was opened. Section 3
onward is the reconciliation.

## 1. Gate outcome

The gate passed. What I checked it against, and what I found.

I read both catalogues in full (`mock/research/canon/the-soul-of-vehje-positive-catalogue.md`,
`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md`) and
`mock/research/202607260100_op-standing-design-calls.md`, then the primary sources: the
signature crate, the runtime-gen crate, `vehje-ir/src/grade.rs`, `vehje-ir/src/node.rs`,
`vehje-lower/src/lib.rs`, `mock/runtime-zig/src/runtime.zig`, the SK1 sketch, and the
bench cells under `mock/benches/`.

The assigned work is licensed. The canon fixes the property, the architecture class, and
the certification perimeter, and does not fix the data schema. I verified the negative
independently rather than inheriting it: grepping both catalogues for `semantic`,
`comput`, `arithmetic`, `opcode`, `primitive vocabulary`, and `evaluator` returns Frontier
A, D, E, G, Section 1, and the PE-as-extraction kill, all of which constrain the *stage*
and the *medium*, and none of which names a schema for what an operation computes. The
lead designer's ruling that this is an implementation detail is marked inline at
`202607260500_topic.the-languageauthor-specialisation-stage.md:101`, which is the
governing rung for that narrow point; the surrounding prose of that topic is agent output
and I have not treated it as authority.

One caveat on the licence, which I flag rather than treat as a blocker. The ruling says
the answer must not override canon. Section 5 of this document names three places where a
candidate answer *would* override canon, and one place where the current tree already
does. Those are reported, not resolved.

## 2. Independent reading, written before the prior exploration was opened

### 2.1 The question, restated in staging terms

An operation's semantics is a function from operands to a value. The design question is
not "what does that function look like" but "which part of it is known at which binding
time." Every candidate representation `R` comes with an evaluator `eval_R : R x Args ->
Value`. The whole space is the set of choices for `R`, and each choice fixes, by
construction, what is left to compute after `R` is supplied.

That reframing is the first thing to get right, because it makes the projection identity
visible. Supplying `R` early and leaving `Args` late is the first Futamura projection:
`mix(eval_R, R)` is a compiler from `R` to residual code, and it is the same definition as
`eval_R(R, .)`, not a second one. Frontier A names exactly this and places it at
LanguageAuthor: "specialising the general Zig runtime to the language definition at our
build, which is the first Futamura projection as a build step" (positive catalogue:21).

### 2.2 The category enumeration

Eight categories, derived from where the meaning lives and what the projection mechanism
is, not from permuting the shapes already in the record.

**Category 0, no representation: the host computes.** The data carries a family id and the
runtime calls out. This is what ships today (`mock/runtime-zig/src/runtime.zig:410-434`).
Named so the baseline is on the record.

**Category 1, nominal binding.** The data carries a name or id; the computation is a
hand-authored function in some namespace, and the projection is a comptime name-to-function
bind. The data selects; it does not describe.

**Category 2, closed primitive vocabulary.** The framework fixes a small universe of
primitive operators over the value domain it already commits to, and the language
definition's data *assigns* each family operation a primitive. The meaning is in the
framework's vocabulary, so both artifacts can evaluate it.

**Category 3, body as a term in a purpose-built expression language.** The data carries a
small first-order expression tree per operation over its operand slots. Both sides
evaluate it; Zig comptime can unfold it into straight-line code.

**Category 4, body as a rewrite into the existing Core forms.** The operation's definition
does not describe a computation; it says how the operation elaborates into Core nodes that
already have meaning, plus a smaller residue. The data language is `vehje-ir` itself.

**Category 5, body as target-level residual.** The data carries the already-lowered form
per point on the output spectrum (predecoded register form, an isel-selected instruction
sequence, a copy-and-patch stencil).

**Category 6, body as specification with the implementation derived or checked against
it.** What crosses is an algebraic law set or a denotation; the executable is synthesised
from it, or an executable from another category is checked against it.

**Category 7, body as a handler clause.** No new representation: the language definition
ships a `Handle` clause whose body services the operation, and the LanguageAuthor stage is
the stage that provides the handler. Frontier B licenses reading it this way, since a
handler is "one dispatch discipline the system has no opinion about" and the discharges
"differ only in operational feasibility" (positive catalogue:23).

### 2.3 The structural result that reorganises the list

Categories 4, 6, and 7 are not peers of 1, 2, 3, and 5. They are composition layers, and
each needs one of the others underneath.

Define semantics by rewriting into Core forms (4) and you have deferred the question to
whatever the Core forms bottom out at. Define them as a handler clause (7) and the clause
body must itself compute, in terms of something. Define them by a law set (6) and the laws
constrain an implementation that has to exist. In every case the recursion must terminate
at an irreducible base whose meaning is *not* given by the IR. That base is the actual
design decision, and it is a choice among categories 1, 2, 3, and 5.

This is not a stylistic observation. It says that the interesting fork is narrower than
the eight-way list suggests, that "define the language in the language" answers a
different question than the one asked, and that a proposal which offers only an
elaboration layer has not answered at all.

### 2.4 What canon forecloses

**Category 0 is foreclosed.** Standing call 2 is explicit: the host "does not supply
arithmetic, comparison, or any other elementary computation, and a design that requires it
to is backwards" (`202607260100_op-standing-design-calls.md:35-36`). The canon's Section 1
puts the same fact structurally: "what remains ships as validated data. That data is one
signature projected into two certified artifacts, which a single general runtime
specialises to itself" (positive catalogue:13). The host is not one of the two artifacts.

**Pure Category 1 is foreclosed as the home of semantics.** Two lines do it. Frontier F:
"One declarative signature (a graded algebraic signature carrying its own grammar) is the
single source" (positive catalogue:31). Section 1: "That data is one signature projected
into two certified artifacts." If the meaning of `+` lives in a hand-authored namespace
that the signature merely names, the signature is not the single source of it, and the
namespace is a third source that no projection relates to the other two. That is the
condition Frontier F was written to exclude.

The foreclosure is of Category 1 *as the home of semantics*, not of nominal binding as a
mechanism. An operation whose meaning genuinely is opaque (a host call, a platform
intrinsic) is nominally bound today and should stay that way; that is what `Perform` and
the host boundary are for. The kill lands on using nominal binding for computation the
language owns.

**Category 5 is foreclosed as a source of meaning, alive as a projection.** The canon
already treats the compiled form as downstream: Frontier F names "the introduction-compiled
stencils" as one of three *projections* of the one algebra (positive catalogue:31), and
the negative catalogue killed "value-kinds as a parallel closed set" in favour of "the
introduction-form projection of the one algebra" (negative catalogue:29). A target-level
encoding as the definition inverts that. It is also the shape the benches demoted:
copy-and-patch is "demoted below direct isel" (positive catalogue:61) and the
privileged-stencil framing "moves from soul to inverse" (negative catalogue:52).

**Category 3 is under a live precedent it must answer.** A purpose-built expression
language is a second closed set of forms running parallel to the Core forms, and the
negative catalogue killed exactly that shape once: "value-kinds as a parallel closed set
-> the introduction-form projection of the one algebra" (negative catalogue:29). It is not
foreclosed, because the kill was about value kinds and this is about operation bodies, but
a Category 3 proposal owes an argument for why its expression language is not the same
mistake. The clean answer to that objection is to make the body a term of the Core algebra
itself, which is Category 4 over a Category 2 base.

**Nothing forecloses Category 2, 4, 6, or 7.** In particular, a closed primitive vocabulary
is not a family. `vehje-ir/src/node.rs:72-81` already commits the framework to a value
domain with `Bool`, `Int<64, Hot>`, and interned `Str`, and the framework already ships
`If` over `Literal::Bool` folding in `vehje-lower/src/lib.rs:150-160`. A base vocabulary
over a value domain the framework already owns does not make the framework own a family.

### 2.5 The constraint the topic's framing misses: the lattice has four points

This is the finding I consider load-bearing, and it is not in the topic's statement of the
question.

The topic asks how semantics cross "from Rust into the Zig runtime"
(`202607260500_topic...:60-61`). That is one edge of a four-node lattice. The lattice is
`{LanguageAuthor, Bundler, HostLoader, Runtime}` (`vehje-ir/src/grade.rs:105-115`), and
Frontier A says ahead-of-time work lives at LanguageAuthor *and* at Bundler, "the Rust side
inlining a known constant" (positive catalogue:21), while "load-time compilation, not
profile-guided JIT, lives at HostLoader."

The same operation's semantics must therefore be discharged at up to four sites, and this
tree already contains three of them:

- Bundler, in Rust: `mock/crates/vehje-lower/src/lib.rs:126-128` carries the FIXME "fold a
  family primitive over literal operands once the family payload encoding lands." The
  const-folder needs to know what `+` means.
- HostLoader, in Zig: `mock/crates/vehje-lower/src/lib.rs:92-94` states that the same rule
  table "`vehje-runtime-gen` packages for the runtime's load-time lowering of arriving
  scripts, so the Rust crate is the definition and the Zig stage is the specialised
  execution."
- Runtime, in Zig: `mock/runtime-zig/src/runtime.zig:278`, the tree-walk `eval`.
- LanguageAuthor: the comptime specialisation that does not exist yet.

The consequence is a hard admissibility test on any candidate. **A representation must be
evaluable by rustc-compiled Rust and by Zig, at comptime and at runtime, or the operation's
meaning gets written twice.** Two hand-written copies of one meaning is precisely what the
canon's centre of gravity forbids, and the drift test's fastest question is whether a thing
"serve[s] the one binding-time-directed handler discipline that ships certified data, or
does it fragment it" (positive catalogue:65).

This test alone kills Category 1: Rust cannot evaluate a Zig function at Bundler time. It
is a sharper and more mechanical argument than the Frontier F argument above, and it is
invisible to any framing that presents the space as a two-point choice between comptime
and runtime.

I also record what the lattice does not currently support. Of the four grade coordinates,
`binding` is hard-coded `Knowledge::empty()`, which I verified at
`mock/crates/vehje-typecheck/src/lib.rs:367` (the canon cites 311-312; the lines have moved
since it was written). So the axis this whole design leans on is present as a type and
absent as a computation. Any candidate whose discharge is chosen by the binding-time grade
is depending on an unbuilt input, and that dependency is owed a red test before the round
proceeds.

### 2.6 The Futamura identity, tested rather than asserted

The claim I was asked to test independently is that runtime interpretation and comptime
specialisation are one representation under two binding-time discharges. My reading, formed
before opening the prior exploration:

**Sound as a staging identity.** For a fixed `R`, `eval_R(R, args)` and `mix(eval_R, R)(args)`
are the same function, and in a two-level language they are literally the same source text
with one parameter's binding time changed. This is not analogy; it is the mix equation, and
the canon already treats it as an equation rather than a preference (negative
catalogue:13, the PE-as-extraction kill: "the mix equation is an equation not an
optimisation").

I confirmed it on the pinned toolchain rather than citing it. Zig 0.16.0, one 40-line
source, a first-order body encoded as data. Marking two parameters `comptime` turns the
tree-walker into a specialiser; the body stays byte-identical:

```zig
fn build(comptime body: []const N, comptime at: u32, args: [*]const i64) i64 {
    const n = body[at];
    return switch (n.tag) {
        0 => args[n.a],
        1 => n.k,
        2 => build(body, n.a, args) + build(body, n.b, args),
        // ...
```

Across an opacity boundary (`export`, so nothing upstream can specialise it away, per the
canon's own Futamura-artifact warning at negative catalogue:68), a six-node body for
`if (x+y) < 100 then x+y else 100` generates:

```
_staged_clamp_add:
  ldp  x8, x9, [x0]
  add  x8, x9, x8
  mov  w9, #0x64
  cmp  x8, #0x64
  csel x0, x8, x9, lt
  ret
```

Zero dispatch, zero tree walk, one branchless select. The same source unstaged compiles to
a roughly 70-instruction recursive walker. So the identity holds, on this toolchain, for a
Rust-emittable encoding, and it holds *constructively*: the specialiser is not a second
program to write.

**Unsound as a design conclusion, in three ways.**

First, the identity is a theorem about a pair sharing one `R`. It says nothing about two
candidates with different `R`. If one candidate is "the data names a Zig function" and
another is "the data carries a body," those have different `eval_R`, and calling them one
representation is a category error that hides the decision instead of making it. The
framing is a tool for checking whether a fork is real, not a solvent that dissolves any
fork put next to it.

Second, it names two binding times where the lattice has four (Section 2.5). The two it
omits are the two that live in Rust, and they are the ones that carry the argument
eliminating Category 1.

Third, the two discharges are equal denotationally and unequal operationally, and treating
"one representation" as "so the choice does not matter" is exactly the move Section 6 of
the canon forbids: "A which-is-faster or which-shape-is-cheaper fork is resolved by
building the candidates and measuring." The two discharges differ in comptime cost, in
code size, and in warm dispatch, and each of those already has partial data (Section 2.8).

### 2.7 Subsidiary questions, answered from the existing structure

`Raw` stays the vehicle. `mock/crates/vehje-ir/src/node.rs:155-159` gives the typing reason
`Perform` was split out: "arithmetic lowers to `Raw`, so performing through `Raw` would give
`1 + 2` an effect, and that is a grade the handler discipline would then have to subtract
everywhere." Moving the computation from the host into the specialised runtime changes
which arm services a `Raw`, not what grade it carries. Purity is already carried
declaratively by `Operation.effect` (`vehje-signature/src/lib.rs:29`) and joined at
`vehje-typecheck/src/lib.rs` in the `Raw` arm via `hook.effect_of_raw(family)`. Nothing new
is needed and nothing shifts.

What `Signature::Operation` gains is one field: whatever the chosen base representation
is, keyed per operation, alongside the family, effect, and lease it already carries.

`SliceKind::FamilyTable` carries the operation catalogue including that field.
`SliceKind::LoweringRules` carries the rewrite layer if a Category 4 elaboration is
adopted, which is consistent with what `vehje-lower` already says that slice is
(`vehje-lower/src/lib.rs:89-94`).

The emitter seam does not close by choosing a representation, and no candidate should
claim it does. Frontier E states the perimeter precisely: rustc certifies well-formedness,
Zig comptime certifies the specialisation type-checks, "neither certifies semantic
preservation of any lowering" (positive catalogue:29). What a shared, both-sides-evaluable
representation buys is narrower and worth stating exactly: it makes the differential check
at `vehje-runtime-gen/src/lib.rs:92-96` a check between two *evaluations of one definition*
rather than between two hand-written implementations. That is a genuine reduction in what
the check has to catch, and it is not a proof.

### 2.8 What must be measured, and what the committed data already says

The committed cells that bear on this, read from `mock/benches/`:

`comptime-cost-cliff/findings.md` measures the comptime side. Type generation via `@Enum`
is roughly linear at about 950 microseconds per field at n=20000, "fine for a realistic
language table (tens of families, sub-second)." O(N) content folding is cheap, 20000 records
in about 918 ms. The cliff is superlinearity, not comptime itself. Read against this
question: a per-operation body unfold is O(body size) per operation, so a language with
tens of operations and small bodies sits far inside the cheap region, and the cell already
tells us where the boundary is.

`runtime-binary-size/findings.md` measures the size side and is the cell most likely to be
misread here. Its number, roughly 1 KB plus about 1 byte per family, is measured on
"trivial-identical families, so the family dispatch collapses (the compiler dedups
identical arms)." It therefore measures the *dispatch skeleton*, and explicitly not the
specialisation of distinct real bodies. The size question this design actually raises is
unmeasured.

`interp-dispatch/findings.md` and `dispatch-heavy-ops/findings.md` fix the baseline the
specialised form must beat: plain switch at about 4.58 ns/op for the lean set, and switch
faster than tail threading at every body weight tested. Any claim that specialisation wins
is a claim against that number, not against a strawman interpreter.

`partial-eval-specialization/findings.md` establishes that the staged unfold terminates by
the well-founded binding-time grade and costs about 100 to 230 ns per node on the compile
side.

What none of them measures, and what this fork actually needs:

1. **Specialisation explosion.** Code size of the composed runtime as a function of
   (operation count) x (body size), with *distinct* bodies so no dedup collapses the arms.
   The existing size cell cannot answer this and says so.
2. **The warm crossover.** Specialised arm versus data-driven interpretation of the same
   body, across body size, against the switch baseline, across an opacity boundary. My
   probe gives one point (6 instructions against about 70) and one point is not a curve.
3. **Comptime cost at realistic shape.** Not `@Enum` field count, which the cliff cell
   already has, but per-operation body unfold cost as bodies grow, and whether any candidate
   drives a superlinear comptime algorithm, which is the measured cliff.
4. **Bundler-side fold.** Rust-side evaluation cost of the same representation, since
   Section 2.5 makes it a required consumer and no cell measures it.
5. **The elaboration layer's cost, if a Category 4 layer is adopted.** Expansion is a
   node-creating catamorphism over host-lent arenas (Frontier G), and Frontier G's own
   N-arena question is named as owed and unbenched in the negative catalogue at line 82.
   A Category 4 answer inherits that open bench rather than sidestepping it, and that
   should be stated by whoever proposes it.

### 2.9 Where my own reading lands, held as hypothesis

Stated so the reconciliation can be checked against something falsifiable, and not as a
call. The categories that survive canon are 2, 3, 4, 6, and 7; of those, 4, 6, and 7 are
composition layers needing a base from 2 or 3; and the admissibility test in Section 2.5
(evaluable by both rustc and Zig, at both comptime and runtime) is satisfied by 2 and 3 and
failed by 1. So the live fork is narrow: **a closed primitive base, with or without an
elaboration layer over the Core algebra, and with the base's data language being either the
Core forms themselves or a purpose-built first-order expression form that owes an answer to
the parallel-closed-set kill.** Which of those is cheaper is a measurement, and the bench
plan in Section 4 is what I would run to decide it.

## 3. Reconciliation with the prior exploration

Written after reading `mock/research/202607260600_semantics-projection-design-space.md`.


### 3.1 Where we agree, and on what grounding

Six agreements, each of which I reached from the canon and the source before reading the
prior document, and each of which it grounds in the same or an equivalent citation.

**The gate passes and the canon does not fix the schema.** Its check
(`202607260600:12-21`) and mine (Section 1) are independent and land the same way. This is
the one point on which two independent canon-grounded reads now exist.

**Host-callback dispatch for elementary computation is foreclosed.** Same citation,
standing call 2 at `202607260100:35-36`, same identification of `runtime.zig:410-434` as
the shape being replaced.

**Rust emitting Zig source is foreclosed.** Same citation, Frontier E at `positive:29` plus
`negative:37`.

**`Raw` stays the vehicle and arithmetic is not promoted to a new Core form.** Same
reasoning, from `node.rs:155-159` and the by-algebra-not-by-census precedent at
`positive:23`.

**A closed primitive vocabulary emitted as data and specialised at Zig comptime fits the
canon.** Its categories II and III, my categories 2 and 3. Both reads ground this in
Frontier A's placement of ahead-of-time work at LanguageAuthor (`positive:21`) and Frontier
E's licence for "Zig comptime certifies the specialisation type-checks against that data"
(`positive:29`). This is the agreement the round actually needs, and it now has two
independent grounded reads.

**The composite shape (declarative default, nominal escape for genuinely opaque
operations) is the right target.** Its category IV, my Category 2 base with Category 1
surviving only where meaning is genuinely opaque. Its additional grounding in standing call
4's "known wins are on by default, opt-out" (`202607260100:79-80`) applied to declarativity
is a good move that I did not make, and I adopt it.

Two catches I did not make and credit without reservation. The dynamically-loaded
per-language shared object is a real category from the wider literature, it is genuinely
foreclosed, and its foreclosure sits on a different axis from the dual-locus kill
(`202607260600:87-108`). And the Zig toolchain is unpinned: I confirmed independently that
no `build.zig.zon` and no `.zig-version` exists anywhere in the repo, against a committed
`rust-toolchain.toml` on the other side. Every Zig number in this round, mine included,
inherits that.

### 3.2 Verdict on the one-representation-two-discharges framing

Asked to test rather than accept, so tested three ways.

**It is correct for the pair it is applied to, and I confirmed the underlying identity on
the pinned toolchain before reading it.** Its categories II and III do share one `R`, the
ALU composition tree, so `eval_R(R, .)` and `mix(eval_R, R)` are the same definition under
two binding-time assignments. The demonstration is in Section 2.6: one 40-line Zig source,
two parameters marked `comptime`, and a six-node body compiling to six instructions across
an opacity boundary with the tree walk gone. The framing is not an analogy. It is the mix
equation, which this canon already insists is "an equation not an optimisation"
(`negative:13`).

**It is stated without the precondition that makes it true.** The identity holds between an
evaluator and its own specialisation. It says nothing about two candidates with different
`R`, and applied to such a pair it is a category error that hides a fork instead of
resolving it. The document presents the framing as a general finding, "the canon's own
binding-time machinery already anticipates exactly this choice, for exactly this kind of
data" (`202607260600:210-214`), with no statement of when it does not apply. As written it
is a tool that the next reader will misapply.

**It is applied over a truncated lattice, and that truncation costs it a design
conclusion.** This is the substantive failure, developed in 3.3.

To its credit, and this matters, the document does not commit the error the framing most
invites. It does not conclude that because II and III are one representation the fork
dissolves. It keeps them as separately falsifiable sub-shapes with distinct bench cells and
flags III as the one carrying real risk. So the framing buys real clarity about what is and
is not being decided, and the criticism below is about what it left out, not about a wrong
turn it took here.

### 3.3 The disagreements

**D1. The lattice has four points and the document uses two.** `Bundler` and `HostLoader`
appear nowhere in `202607260600`. The consequence is not cosmetic. Frontier A places
ahead-of-time work at LanguageAuthor *and* at Bundler, "the Rust side inlining a known
constant" (`positive:21`), and this tree contains the Bundler site as a written FIXME:
`mock/crates/vehje-lower/src/lib.rs:126-128`, "fold a family primitive over literal
operands once the family payload encoding lands." That line is the single most direct piece
of evidence in the repository that the representation has a Rust-side consumer, and the
exploration does not cite it. The HostLoader site is equally explicit at
`vehje-lower/src/lib.rs:92-94`, where the same rule table is stated to be packaged "for the
runtime's load-time lowering of arriving scripts."

Restoring the two missing points yields the admissibility test in Section 2.5: **a
representation must be evaluable by rustc-compiled Rust and by Zig, at comptime and at
runtime, or the operation's meaning is written twice.** That test is mechanical, it decides
Category I on the spot, and no two-point framing can see it.

**D2. The ALU-as-consumer-opt-in-Zig-module hedge is wrong, and it is the concrete cost of
D1.** At `202607260600:231-245` the document argues that the primitive vocabulary "has to
be modelled as an optional, reusable Zig module the *specialised, per-consumer* artifact
opts into," and marks the argument as its own reasoning rather than canon-settled. Under
the canon-defence rule that marking is where to look first, and the reasoning does not
hold.

If the vocabulary is a Zig module, the Rust const-folder at Bundler time cannot evaluate
it, so arithmetic gets written twice: once in the Zig module, once in `vehje-lower`. That
is the Category I failure re-created inside the category that was supposed to avoid it, and
it is the exact condition Frontier F excludes by naming one signature "the single source"
(`positive:31`).

The document's own Section 1 already contains the refutation and does not apply it. It
observes that `vehje-ir::Family` ships `Core` (`family.rs:20-24`, which I confirmed) and
correctly concludes that "the framework owns no family" is a statement about consumer
families. Add to that: `vehje-ir/src/node.rs:72-81` already commits the framework to a value
domain of `Bool`, `Int<64, Hot>`, and interned `Str`, and `vehje-lower/src/lib.rs:150-160`
already ships a framework-level fold over `Literal::Bool`. A closed primitive vocabulary
over a value domain the framework already owns is framework-level by construction. The
hedge was unnecessary and it is harmful.

**D3. "Runtime binding time" for category II misuses the lattice.** `grade.rs:105-115`
grades what is *known*: Runtime is "Known only at runtime." The ALU tree is known at
LanguageAuthor in both II and III; what differs is whether the discharge is taken there or
deferred. Frontier A is precise about this and the document quotes the very sentence
without applying it: "Interpretation is a Runtime execution form of the already-specialised
residual" (`positive:21`). An execution form is not a binding time. II and III share a
binding-time grade.

**D4. The burden of proof between II and III is inverted, and this is where a candidate
answer would override canon.** Section 1 of the positive catalogue states the mandate: the
system "discharges every operation it knows [...] at the earliest binding time that
operation's inputs are known" (`positive:13`). Frontier A states it again for this exact
stage: ahead-of-time work "lives at LanguageAuthor (specialising the general Zig runtime to
the language definition at our build)" and "literal const-inlining therefore mandates a
specialisation *stage* at the binding time the input becomes known" (`positive:21`). The
operation body is the language definition. III performs that stage on it. II specialises
the dispatch skeleton and leaves the body to a generic tree-walker, which is a partial
discharge of the stage this round exists to build.

So III is the canon default and II is the fallback that has to earn its place by showing
III does not work or does not pay. The exploration presents them as symmetric peers and
describes III as "the category with the least existing evidence and the most risk"
(`202607260600:326-327`). Under the canon's letter, adopting II as the default without a
measurement showing III fails would override canon. I flag it under the gate rather than
resolving it, since the ruling at `202607260500:101` licenses an implementation-detail call
only where it does not override canon.

**D5. The Cell 2 sketch tests a failure mode that cannot occur, and misses the ones that
can.** The document's falsification condition is whether "Zig's inliner actually collapses
a comptime-recursive tree-walk into straight-line code," failing "most likely past some
tree depth or Zig's default inline-cost heuristic" (`202607260600:356-372`), and it imports
a Deegen warning about "relying on a general-purpose optimiser to fully specialise an
interpretation loop away" (`202607260600:378-382`).

A `comptime` parameter in Zig is not an optimiser hint. It forces monomorphisation at
semantic analysis: `build(comptime body, comptime at, args)` is instantiated per
`(body, at)` pair, the `switch` scrutinee `n.tag` is comptime-known, and the untaken arms
are not analysed at all. The tree walk is evaluated by the compiler and is gone before LLVM
sees anything. There is no inline-cost heuristic in the path, so the named failure mode
cannot occur, and a sketch built to look for it would have returned WORKS while certifying
the wrong property.

I ran the sketch instead of theorising about it. WORKS, generated code in Section 2.6, six
instructions with a branchless `csel` and no residual structure. The Deegen citation is
about a real and different hazard: an optimiser being trusted to specialise a *runtime*
interpretation loop, where there is no forced staging boundary. Zig's `comptime` is the
forced boundary. Importing the warning across that difference mislabels the risk.

The risks that are real, and that the sketch should have been designed to find: comptime
evaluation cost as body size grows (`@setEvalBranchQuota` is a hard compiler budget that
must be managed, and at 20000 body nodes a scratch probe of mine took roughly 9 seconds to
compile, so the cost is real even where the shape works); per-node function instantiation
and the code-size growth that follows; and comptime recursion depth on a deep body. None of
these appear in the document's falsification condition.

I also record a trap I fell into while probing this, because it bears directly on the bench
plan. My first scaling probe used a synthetic chain of constant-folding arithmetic, and
LLVM reassociated the whole chain into two instructions at large sizes, so the numbers
measured strength reduction rather than specialisation. That is the canon's own
Futamura-artifact class (`negative:68`) in miniature. Any size or speed cell here must use
bodies that cannot be reassociated away, and must cross an opacity boundary. I am not
reporting my scaling numbers as a measurement for exactly this reason.

**D6. The empty-CSV finding is correct in the particular, incomplete in the general, and
looked in the wrong place.** `mock/benches/dispatch-heavy-ops/heavy.csv` is header-only. I
confirmed it. Three corrections.

First, `mock/benches/interp-dispatch/dispatch.csv` is header-only by the identical standard,
and the exploration leans on BN1 far more heavily than on BN1-heavy: it cites the 4.48 to
47.60 ns/op and 4.5 to 7 ns/op figures throughout Sections 2 and 3. The same audit applied
to its own primary citation would have flagged it.

Second, it looked only in the per-cell scratch directory. The harness-resident dispatch data
exists, at `mock/benches/results/zig_dispatch/` with per-size CSV, `meta.json`, per-size
findings, and a `FINDINGS.md`, plus `.bench_history/zig_dispatch_*.tsv`. It measures the
same switch-versus-tail question over a size sweep and reports tail at 1.64x to 1.90x of
switch, which agrees directionally with the standalone cells.

Third, and this is the finding that should replace the one it made: `dispatch-heavy-ops` and
`interp-dispatch` are standalone Zig cells sitting outside the harness, which is what op's
standing call 5 forbids in terms that name this case, "Even for zig-authored benches, you
*NEED* to plug them into the harness" (`202607260100:88-89`). The harness-resident
replacement already exists. The correct action is not to re-run the standalone cell but to
cite `results/zig_dispatch/` and to fold any body-weight sweep into that carrier variant.

**D7. Minor, and reported because citation drift is how a canon stops being checkable.** The
positive catalogue cites the hardcoded grade axes at `vehje-typecheck/src/lib.rs:311-312`
(`positive:27`); they are at 367 and 368 today. The canon's letter governs and the fact it
states is still true, so this is a pointer to repair, not a substantive error.

### 3.4 The two-expert ledger

Stated plainly, because a call about what the canon permits needs two independent
agreements, each grounded in quoted canon.

Agreed by both reads, independently grounded: host-callback dispatch is foreclosed; Rust
emitting Zig source is foreclosed; a dynamically loaded per-language semantics module is
foreclosed; `Raw` remains the vehicle and no new Core form is added; a closed primitive
vocabulary emitted as data and specialised at Zig comptime fits the canon; the composite
shape of a declarative default with a nominal escape for genuinely opaque operations fits
the canon.

Not agreed, one read each, and therefore the human's call rather than a third expert's:
whether Category I (nominal binding to a hand-authored implementation) is foreclosed as the
home of semantics or merely costly; whether Category II is a coequal default or a fallback
that must earn its place against the Frontier A mandate; and whether the primitive
vocabulary is framework-level or a consumer-opt-in Zig module.

The first and third of those disagreements have the same root, the missing Bundler point,
and would likely collapse into agreement if the prior read were extended over the full
lattice. I am not going to record that as agreement, because agreement reached by one read
adopting the other's frame is confirmation, not corroboration.

## 4. My bench plan, and how it differs

Every cell runs in `mock/benches/` under the harness, with per-variant isolation, byte-exact
cross-validation against an oracle, committed CSV in `results/`, and a `.bench_history/`
entry. No standalone timing loop, per standing call 5 and the workspace rule. Every cell
crosses an opacity boundary and uses bodies that cannot be reassociated away, per the trap
in D5.

Ordered by burden of proof rather than by ease, which is the first structural difference
from the prior plan.

**Cell A, the four-way differential, first and not optional.** One operation definition,
four evaluations: the Rust const-fold at Bundler, the Zig comptime-specialised arm at
LanguageAuthor, the Zig load-time-lowered form at HostLoader, and the Zig tree-walk at
Runtime. Drive a corpus through all four and cross-validate byte-exact. This is a bench,
not merely a deliverable, because the harness's cross-validation is what turns "one
definition, many projections" from prose into a re-runnable artifact, and because a variant
that diverges must be rejected rather than timed. The prior plan explicitly excludes the
differential harness from its bench plan (`202607260600:522-529`), reasoning that it is not
a which-is-faster fork. I disagree: it is the cell that makes every other cell's comparison
meaningful, and canon Section 6 names byte-exact cross-validation against an oracle as a
harness feature precisely for this.

**Cell B, the comptime envelope for the specialised discharge.** Two axes measured
separately per BN0's method: comptime seconds via `-fno-emit-bin`, and object bytes, both as
a function of (operation count) times (body nodes per operation), with distinct
non-reassociable bodies. Watch for three things the prior plan does not name: BN0's
superlinearity signature, the `@setEvalBranchQuota` wall, and per-node instantiation growth.
This replaces the prior Cell 2 sketch, whose question I have already answered WORKS with
generated code, and whose falsification condition tested a mode that cannot occur.

**Cell C, warm crossover, in the harness carrier.** Specialised arm against data-driven
interpretation of the same body, swept over body size, against the switch floor. Implement
as new variants of the harness-resident `zig_dispatch` carrier, not as an extension of the
standalone `dispatch-heavy-ops` cell the prior plan names (`202607260600:453-467`), because
that cell is outside the harness and its CSV is header-only. One point exists already, from
my probe: six instructions against roughly seventy for a six-node body. One point is not a
curve.

**Cell D, size with real distinct bodies.** Agreed with the prior plan's Cell 3, same
justification, same target of the stated scope gap in `runtime-binary-size/findings.md`. My
only addition is the non-reassociable-body requirement, without which the cell measures
LLVM's strength reduction, as my scratch probe demonstrates it will.

**Cell E, the Bundler-side fold.** Rust-side evaluation cost of the same representation over
the same corpus, absent from the prior plan entirely because the lattice point is absent.
Required to know whether the representation that is cheap for Zig comptime is also cheap for
the const-folder that has to run on every bundled program.

**Cell F, coverage, not a timing cell.** Agreed with the prior plan's Cell 4 without
modification: draft the vocabulary, walk the first family's operation set against it, and
report the escape-hatch fraction as a number.

Two things owed alongside, neither of them cells. The red test the topic already names, a
program computing `1 + 2` under a null host returning 3, which must be red now and stays red
until the stage lands. And a red test for the `binding` grade axis, which every discharge
decision here depends on and which is hard-coded `Knowledge::empty()` at
`vehje-typecheck/src/lib.rs:367`.

## 5. Unlicensed mechanisms and drift, including outside this question

Reported under the standing gate obligation, with citations, in descending order of cost.

**`vehje-schedule` still exists.** The negative catalogue states in the present tense, as an
op-ratified post-audit call, that "The crate is deleted, the four-pass sequence hardcoded in
`vehje::run`, the unused dep dropped" (`negative:88`). The crate is present at
`mock/crates/vehje-schedule/`, is a workspace member at `mock/Cargo.toml:11` and `:53`, and
is still a dependency at `mock/crates/vehje-runtime-driver/Cargo.toml:11`. Either the canon
is describing a decision not yet executed in a tense that reads as executed, which is the
Part H failure mode the canon itself names, or the deletion is owed. It should be resolved
before the next round cites the canon's Part H as an accurate ledger.

**The runtime's own comment defends the foreclosed shape.**
`mock/runtime-zig/src/runtime.zig:412-413` justifies host-callback dispatch as "what keeps
the framework family-free." Standing call 2 forecloses that shape and names the reasoning
backwards (`202607260100:35-42`). The topic already caught this; I confirm it is still
present and note that a comment defending a foreclosed design is worse than the code, since
the code is scheduled for replacement and the comment teaches the next reader the killed
rationale.

**Two bench findings documents whose cells carry no data.**
`mock/benches/dispatch-heavy-ops/heavy.csv` and `mock/benches/interp-dispatch/dispatch.csv`
are both header-only while their `findings.md` files report full result tables. Under
standing call 5 and canon Section 6 these are claims with the evidence detached. The
harness-resident `results/zig_dispatch/` covers the same question properly and should be the
citation; the standalone cells should be folded into it or marked as superseded.

**The Zig toolchain is unpinned.** No `build.zig.zon`, no `.zig-version`, anywhere in the
repo, against a committed `rust-toolchain.toml` on the Rust side. Confirmed independently.
Every Zig-side number in this round, the prior exploration's and mine, is reproducible only
against whatever `zig` is on the runner's path. Credit to the prior read for surfacing it.

**The canon's grade-axis citation has drifted** from `vehje-typecheck/src/lib.rs:311-312` to
367 and 368. Repair the pointer; the fact it states is unchanged.

## 6. What this document does not do

It does not pick a category, and it does not treat its own convergence with the prior read
as a resolution. Two independent grounded agreements now exist for the six items listed in
Section 3.4, and three disagreements remain that belong to the lead designer rather than to
a third expert. The narrow live fork after both reads is whether the primitive base's data
language is the Core forms themselves or a purpose-built first-order expression form, and
that is a measurement, run per Section 4.
