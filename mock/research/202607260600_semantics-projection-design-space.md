# The design space for projecting an operation's semantics into the specialised runtime

**Date:** 2026-07-26
**Dispatched by:** `mock/design_rounds/202607260500_topic.the-languageauthor-specialisation-stage.md`,
which names this file as its deliverable and states the question is settled "by
benching and expert exploration," not by the canon (`202607260500:101-106`),
and briefs the exploration to enumerate categories rather than permute the
three shapes already on paper (`202607260500:115-119`).

## Gate outcome: passed

Checked against `mock/research/canon/the-soul-of-vehje-positive-catalogue.md`,
`mock/research/canon/the-inverse-of-vehje-negative-catalogue.md`, and
`mock/research/202607260100_op-standing-design-calls.md`. The concrete data
representation for an operation's semantics is not fixed anywhere in either
catalogue; `202607260400_family-semantics-as-data.md` verified this
independently by grepping both documents for the vocabulary the one prior
proposal used and found zero matches (`202607260400:162-166`). The dispatching
topic's own gate already ran this check and the lead designer ruled the
question an implementation detail (`202607260500:101-106`), so under this
dispatch's instructions that ruling stands and is not re-litigated here.

Three canon statements bound the space and are treated as hard constraints
throughout, not preferences: Rust emits data, never Zig source
(`positive:29`, `negative:37`); the runtime computes without host help for
elementary operations (`202607260100:34-42`); assurance is a dial and no
mechanism here can claim to certify semantic preservation
(`positive:29`). One further finding surfaced while re-deriving these from
source rather than trusting the prior memo's restatement: `vehje-ir::Family`
(`mock/crates/vehje-ir/src/family.rs:20-24`) shows the framework already owns
one family, `Core`, so "the framework owns no family" (call 2,
`202607260100:44`) is a statement about *consumer* families, not a claim that
the framework's crate is empty of family-shaped code. That distinction turns
out to matter for category III below and is flagged there, not assumed.

I also verified the toolchain claim in the brief rather than trusting it: the
installed `zig` is `0.16.0` (`zig version`), matching what
`202607210300_sk1_certified-generation-core/findings.md` measured, and its
`engine.zig` demonstrably uses `@Enum` in place of the removed `@Type`
(`engine.zig:11-19`, confirmed by reading the file). One gap worth flagging
outside this question's scope: unlike Rust's committed `rust-toolchain.toml`,
nothing in this repo pins a Zig version (`mock/runtime-zig/` has no
`build.zig.zon` and no `.zig-version`; grepped and confirmed absent). Every
Zig-side number in this document and in the cited benches is only as
reproducible as whatever `zig` happens to be on the runner's `PATH`. This is
not blocking for the question asked, but it is a real gap and I am not going
to bury it in a footnote: raise it as its own small fix (a `build.zig.zon`
`minimum_zig_version` pin) alongside whichever category ships.

## 1. What is actually in the space, named honestly

The brief's three examples are one axis of one category (how a hand-authored
implementation is selected) crossed with a deferral order. Read broadly, the
question is: *given a `vehje-signature::Operation` and no operation-body
field on it, what crosses the LanguageAuthor-stage boundary from Rust to Zig
so that a specialised evaluator exists for that operation, and at what
binding time does the specialisation happen?* That second clause, binding
time, is the axis the existing three examples collapse and the canon itself
already names as a first-class dimension (Frontier A's four-point lattice,
`positive:21`). Splitting on it is what turns three examples into a real
space.

### Foreclosed, named so nobody re-proposes them

**Host-callback dispatch for elementary computation.** What ships today
(`mock/runtime-zig/src/runtime.zig:410-434`): every `Raw` node, arithmetic
included, evaluates its operands and calls `host.call(...)`, returning
`EvalError.NoHost` with no host attached. Killed by op's standing call 2 in
terms that name this exact shape: "The host does not supply arithmetic,
comparison, or any other elementary computation, and a design that requires
it to is backwards" (`202607260100:35-36`). Not a candidate; it is the thing
being replaced.

**Rust emits Zig source text.** A code generator that prints a `.zig` file
per language, containing hand-shaped `if`/`switch` bodies for each family's
operations, compiled as ordinary Zig source. Foreclosed twice over: Frontier
E states the medium is data, never source, citing exactly this route as
"the LMS-hard, hard-to-certify route, rejected at topic 1627"
(`positive:29`), and the negative catalogue lists it by name as killed
(`negative:37`). No representation in this document emits Zig text; every
surviving category emits bytes that Zig comptime reads as data via
`@embedFile` or an equivalent const-import, matching the crossing
`vehje-runtime-gen::Slice.bytes: &'a [u8]`
(`mock/crates/vehje-runtime-gen/src/lib.rs:53`) already uses for every other
package slice.

**A dynamically loaded per-language shared object, resolved at HostLoader
time.** Instead of comptime-specialising one static Zig binary per language,
ship one generic runtime and `dlopen` a per-language module of semantics at
host-load time. This is a real category in the wider literature (it is how
most embeddable VMs with a plugin story work) and the canon's own lattice
names HostLoader as a real binding-time stage (`positive:21`), so it deserves
naming rather than silent omission. It is foreclosed by two independent
pieces of ratified text, not one: Section 3 fixes the shipped artifact as
"one hand-authored engine comptime-specialised to the Rust-emitted data"
(`positive:45`), which is a build-time, not load-time, specialisation; and
the same section fixes the runtime as embeddable, "small, fast, embeddable
[...] Pulls no `std`, no Rust runtime" with no heap on either side
(`positive:45`, restated at `202607260400:236-240` from the module doc at
`runtime.zig:17`). `dlopen`, or any dynamic-loading equivalent, needs libc
and OS loader support that a `no_std`, no-heap, single-static-binary
identity does not carry, and it would mean the runtime is *not* specialised
at build (it stays generic and defers resolution to load time), directly
contradicting the fixed Section 3 language. This is not the same objection as
the "dual-locus" kill in the negative catalogue (`negative:38`, which is
about *who runs per-script analysis*, Rust vs the runtime); it is named
separately because it is a distinct wrong turn on the *binding-time* axis
rather than the *analysis-locus* axis, and the two are easy to conflate.

**Widening `vehje-ir::Raw`'s payload, or promoting arithmetic to a new Core
form, as a side effect of this design.** Both are out of scope, not
foreclosed as wrong. `Node::Raw`'s payload is explicitly an owed FIXME,
"the payload is a family-interpreted handle [...] until the first family
defines its payload encoding" (`mock/crates/vehje-ir/src/node.rs:130-132`),
and whatever category is picked settles that FIXME as a side effect, which
`202607260400:289-295` already flagged as the lead designer's call, not this
document's. Promoting arithmetic to a new Core form (rather than `Raw`) is a
different, structurally larger move: Core forms are added "by algebra, not
by another census cut" (`positive:23`, the `Handle` precedent), and
`node.rs:156-159` already gives the standing reason arithmetic is `Raw` and
not `Perform` (typing, not ergonomics: routing arithmetic through the effect
form would grade `1 + 2` as an effect). Nothing in this exploration reopens
that; every surviving category below keeps arithmetic as a `Raw`-family
operation.

## 2. The surviving categories

Three categories survive, one of them with two binding-time realisations
that are the same data shape discharged at different stages of the lattice
Frontier A already names. A fourth is the hybrid of the first and the third.

### Category I: hand-authored Zig, selected by name-keyed comptime dispatch

The shape already in the record (`202607201641_certified-generation-panel/02_giesen.md:74-77`,
restated at `202607260400:185-199`), evaluated here rather than re-described.

**1. What crosses, what Zig does with it.** Per operation, Rust emits an id
or name plus the structural metadata `vehje-signature::Operation` already
carries (`family: FamilyId`, `effect: EffectMask`, `lease: LeaseRule`,
`mock/crates/vehje-signature/src/lib.rs:24-32`). Zig comptime generates an
exhaustive tag enum from the emitted ids via `@Enum`, exactly as SK1
demonstrates for family tags (`engine.zig:11-19`), and resolves each tag to a
hand-authored function in a per-consumer Zig namespace via `@field(ns,
@tagName(tag))`, so a missing declaration is a compile error rather than a
silent fallthrough.

**2. Cost.** Build: comptime cost is dominated by `@Enum` type generation,
measured near-linear and cheap at realistic sizes by BN0
(`mock/benches/comptime-cost-cliff/findings.md`: ~950us/field at n=20000,
sub-second for the tens-of-families scale any real language reaches).
Runtime: dispatch to a hand-authored function body is exactly what BN1 and
its heavy-body expansion already measured, because the interpreter's outer
dispatch loop does not care whether the body is hand-Zig arithmetic or
anything else; switch dispatch wins at every body weight tested, 4.48-47.60
ns/op across body weight 1 to 64
(`mock/benches/dispatch-heavy-ops/findings.md`). Binary size: the composed
runtime's specialisation skeleton is ~1 KB plus ~1 byte per family for the
trivial-identical-family case (`mock/benches/runtime-binary-size/findings.md`),
but that bench explicitly scoped out real per-family bodies ("real families
add their distinct semantics code [...] not measured here",
`runtime-binary-size/findings.md:27-28`), so category I's actual size cost
(N hand-authored functions, one per operation, however similar) is
unmeasured.

**3. What it forecloses.** Every family author writes Zig, for every
operation, always. No purely declarative family is possible under this
category alone; `202607260400:220-227` already names this residual and does
not resolve it.

**4. Effect on the emitter trust seam.** The seam grows per operation and
never shrinks. `vehje-runtime-gen::Manifest` hashes bytes, not meaning
(`hash_package` folds `xxhash3_64` over each slice's raw bytes,
`mock/crates/vehje-runtime-gen/src/lib.rs:114-124`; verified independently by
reading the fold, not merely citing the prior memo's claim), so a
transposed-but-present name passes the manifest check and only the census
differential check (`vehje-runtime-gen::DifferentialCheck`, currently an
unimplemented stub, `lib.rs:83-96`) can catch it. The panel that proposed
name-keyed dispatch is explicit that it narrows only the routing risk: "A
total, name-keyed, exhaustively-switched dispatch can still route to a
handler whose body computes the wrong semantics"
(`202607201641_certified-generation-panel/04_theory-review.md:72-73`, quoted
at `202607260400:200-205`). Every new hand-authored function is a new
unverified surface; the surface grows without bound as families and
operations grow, and it grows per consumer, not once.

**5. What would falsify it, precisely.** Not much is left to bench here for
the runtime-dispatch axis; BN1/BN1-heavy already answered it for switch
dispatch to a function body of any weight, and their finding generalises to
category I's hand-Zig bodies without a new cell. What is genuinely unmeasured
and would need a new cell: binary-size growth under *real*, non-identical
per-operation bodies (see Bench Plan, Cell 3), and the comptime cost of
`@field`-resolving names across the full cross product of families and
operations a real language reaches (tens of families, but potentially
hundreds of operations across them), which BN0 measured only for flat
type-generation, not for per-operation name resolution against a namespace.
A result that would argue against shipping category I as the *only*
mechanism: if the differential-check harness (owed regardless, per Frontier
E) finds that hand-Zig bodies drift from their declared semantics at a
non-trivial rate as the family count grows, that is direct evidence the
growing unverified surface in point 4 is a real cost, not a theoretical one.

### Category II and III: a fixed micro-op algebra as data, discharged at two binding times

These are one representation, not two categories, because the data Rust
emits is identical in both; only the binding time at which Zig turns it into
executable behaviour differs, and that axis is exactly Frontier A's lattice
(`positive:21`): "Interpretation is a Runtime execution form of the
already-specialised residual" versus ahead-of-time work at LanguageAuthor,
"the first Futamura projection as a build step." Presenting them as one
representation with two discharges, rather than two independent proposals,
is itself a finding: the canon's own binding-time machinery already
anticipates exactly this choice, for exactly this kind of data, and naming
them separately (as the record's third example implicitly does, by treating
"interpretable data encoding" as one undifferentiated thing) hides that.

**Shape.** A small, closed, framework-hand-authored-once vocabulary of
primitive computational operations (add, sub, mul, div, mod, the comparisons,
a field projection, a bounded host-call primitive; on the order of the ~25
primitives SP1 already measured as vehje's realistic op-vocabulary size,
`positive:35` item H). Per family operation, Rust emits a small, fixed-capacity
composition tree over that vocabulary (operand-wiring plus which primitive
each node applies), respecting the no-alloc constraint as a `[AluInstr;
MAX_ALU_LEN]`-shaped array with a used-length, never a `Vec`. This is *not*
the unrestricted "interpretable micro-language" the dispatching topic warns
against ("something richer risks becoming an interpreter for a second IR",
`202607260500:69-70`): the vocabulary is closed, small, and bounded by
construction, the same discipline that already bounds `vehje-ir`'s own
Core forms and SP1's measured op count, rather than an open-ended second IR
that could grow without limit.

**A framing worth stating precisely because it cuts against a naive reading
of category II/III:** the ALU cannot be modelled as a new Core form or as
framework-owned computation in the sense op's call 2 forecloses. Call 2 is
explicit that arithmetic's home changes sides (host to runtime-artifact) but
"the framework still owns no family [...] The first-party Clause runtime is a
different artifact from the framework [...] and it ships its own computation"
(`202607260100:44-47`). The ALU has to be modelled as an optional, reusable
Zig module the *specialised, per-consumer* artifact opts into by having its
data reference the ALU's ops, analogous to how `vehje-runtime-gen::SliceKind`
composes stage contributions "without knowing their internal shape"
(`mock/crates/vehje-runtime-gen/src/lib.rs:29-46`), not as a thirteenth Core
form the framework's general engine hardcodes. This is my own reasoning
applying call 2's letter to a mechanism the call does not itself describe; it
is not a canon-settled fact, and I am flagging it as exactly that rather than
folding it into the "fixed" list above.

#### Category II: interpreted at Runtime binding time

**1. What crosses, what Zig does with it.** The ALU composition tree, as
described above, plus a fixed `AluOpTag` enum Zig comptime generates once
(via `@Enum`, independent of the language: the vocabulary is
framework-fixed, so the enum's shape does not vary per language, only which
subset is *used* does, and DCE removes the unused arms). The shipped runtime
carries one generic, hand-authored tree-walking evaluator, called at each
`Raw` dispatch, the same shape the existing `eval` function already is for
Core forms (`runtime.zig:278` onward).

**2. Cost.** Runtime: this adds a second dispatch layer nested inside the
outer `Raw` case, structurally identical to what BN1/BN1-heavy already
measured as "one extra dispatch layer" (their comparison of switch versus
tail/fnptr *is* a comparison of dispatch-layer costs). Their finding that
extra machinery "costs more than the branch misprediction it saves" for
vehje's small, arithmetic-heavy op bodies (`interp-dispatch/findings.md:29-30`)
is a direct, if indirect, warning against II: a tree-walk interpreter nested
inside the family dispatch is exactly an extra dispatch layer, and the
existing evidence says extra layers are not free even when the branch
predicts well. Build and size cost: near-zero beyond a fixed, one-time ALU
implementation (unlike category I, the ALU is written once, not once per
operation), so binary size should scale with the DATA (each operation's tree,
likely tens of bytes) rather than with code.

**3. What it forecloses.** Nothing, for operations that decompose into the
ALU. A family author writes zero Zig for those; the family can be purely
declarative to the extent its operations fit the vocabulary. What it does not
solve: an operation needing something outside the fixed vocabulary (an
effect the ALU has no primitive for) has nowhere to go under II alone, which
is exactly the residual category IV below closes.

**4. Effect on the trust seam.** This shrinks the open seam rather than
widening it, and does so structurally, not by discipline. The ALU is a fixed,
small, hand-authored-once surface, verified once, and reused by every family
and every language; the per-operation data is then only a *structural*
question (well-formed operand wiring, arity match, no cycles), which is
precisely the class of thing "rustc certifies well-formedness, inclusion, and
grading of the emitted data" (`positive:29`) already covers, and which
`vehje-runtime-gen`'s existing manifest/hash discipline is well-suited to
extend (a structural check is a decidable property of the bytes, unlike "is
this hand-Zig body semantically right"). This is the sharpest advantage II/III
have over I: the unverified surface does not grow per operation, it is fixed
at the ALU's size.

**5. What would falsify it.** The BN1/heavy-body finding (extra dispatch
layers cost more than they save, for vehje's regime) is evidence against, not
proof against, because it was measured for a *different* extra layer
(tail-threading vs switch at the SAME dispatch level), not for a *nested*
interpretation layer inside an already-dispatched `Raw` case. A dedicated cell
is needed (Bench Plan, Cell 1): measure ns/op for "switch to hand-Zig body"
(category I, already known from BN1) against "switch to a generic ALU
tree-walk" (category II) on the *same* shared workload BN1 already uses, so
the comparison inherits the harness's existing reference-floor discipline. A
result that falsifies II as the *default*: if the tree-walk overhead is a
material fraction of the ~4.5-7 ns/op switch-dispatch floor BN1 measured
(the same regime where BN1-heavy already showed a ~2-4 ns/op constant
overhead "never gets repaid"), category II loses to category I on raw speed
for operations simple enough that a hand-Zig body would have been just as
cheap, and the design should route toward III (see next) or toward IV routing
the hot common case through I.

#### Category III: specialised at LanguageAuthor binding time (comptime)

**1. What crosses, what Zig does with it.** The identical ALU composition
tree as II, but consumed as a `comptime`-known parameter, not a runtime
slice. A comptime-recursive generic Zig function walks the tree at compile
time and, relying on Zig's inliner collapsing the fully comptime-known branch
structure, is intended to produce a dedicated, specialised function per
operation with no residual runtime tree-walk, only the ALU primitive calls
the tree actually names, in sequence. This is literally Frontier A's named
technique, "the first Futamura projection as a build step"
(`positive:21`), applied to the operation body rather than only to the
family dispatch skeleton SK1 already demonstrated it for. It does not cross
the line into "Rust emits Zig source": the data crossing is unchanged from
II (a tree of ids), and what changes is only that Zig's own comptime
evaluator, not a Rust text-printer, turns that data into code, which is
exactly the "Zig comptime certifies the specialisation type-checks against
that data" mechanism Frontier E already licenses (`positive:29`).

**2. Cost.** This is the category with the least existing evidence and the
most risk, and I am not going to understate that. Runtime: if the inliner
does what the mechanism intends, dispatch to a specialised operation is a
direct call with the interpretation overhead removed entirely, strictly
cheaper than both I and II. Build: BN0's O(N) content-fold finding is
reassuring at the macro scale (thousands of items stay sub-second,
`comptime-cost-cliff/findings.md`), but BN0 measured a flat fold, not a
recursive comptime function walking a tree and calling itself, which is
exactly the pattern BN0's own O(N^2) cliff warns can blow up "regardless of
it being 'content'" if the recursion accidentally re-walks shared structure
(`comptime-cost-cliff/findings.md:22`). Nothing in the existing benches
measures this pattern; it needs its own cell before any cost number can be
trusted (Bench Plan, Cell 2).

**3. What it forecloses.** The same as II on the declarative-family axis
(nothing, for ALU-expressible operations); it adds no new foreclosure over
II, only a different runtime cost profile if the specialisation mechanism
actually works as intended.

**4. Effect on the trust seam.** Identical to II's structural argument (the
ALU is the fixed, once-verified surface), plus one additional obligation II
does not carry: the claim that comptime specialisation preserves the tree's
semantics through the inlining process is itself a new thing to verify, not
merely assume. This is squarely inside "neither certifies semantic
preservation of any lowering" (`positive:29`); III adds a lowering step (data
tree to specialised code) that II does not have, so its assurance grade
needs its own differential-check coverage, not a free ride on the ALU's own
one-time verification.

**5. What would falsify it, and this is the one that needs a sketch before a
bench.** Per `cl-claim-sketch-discipline.md`, whether Zig 0.16's inliner
actually collapses a comptime-recursive tree-walk into straight-line code is
a WORKS/FAILS feasibility question, not yet a which-is-faster question, and
SK1 did not test it (its `dispatch` function is a stub that only returns
`@tagName`, `202607260400:268-271`). The sketch's falsification condition is
concrete and checkable without a timer: compile a small comptime-known ALU
tree through the proposed `specialize` function and read the resulting
disassembly (`zig build -Doptimize=ReleaseFast` then `objdump`/`nm`, the same
discipline BN0 used `-fno-emit-bin` to isolate one variable at a time). If
the disassembly shows straight-line code with no residual branch or loop
structure for the tree-walk itself, III is real and worth benching against
II. If it shows a runtime loop (the inliner gave up, most likely past some
tree depth or Zig's default inline-cost heuristic), III has silently
collapsed into II's cost profile while carrying II's manifest-check overhead
plus an unproven extra lowering claim, which is strictly worse than just
building II, and III should be dropped in favour of II or I. This is exactly
the discipline this persona's own prior work depends on (the disassembly is
the claim, not the abstraction); asserting III works because "comptime
usually inlines things" without reading the generated code is precisely the
kind of unearned confidence the canon's own Section 6 discipline
("audit your own benches adversarially [...] report the actual generated
assembly") exists to prevent, and precisely the mistake Deegen's own design
had to be built around rather than hoped past: relying on a general-purpose
optimiser to fully specialise an interpretation loop away, without a forced,
explicit lowering pass, is a documented failure mode in the wider
interpreter-generator literature, not a hypothetical one.

### Category IV: the ALU as the default, hand-authored Zig as the escape hatch

The residual category II/III leave open (an operation whose meaning is not
expressible in the fixed ALU) is closed by falling back to category I's
name-keyed dispatch for exactly those operations, using the same dispatch
discipline (an exhaustive tag enum, `@field` resolution against a
per-consumer namespace) so the escape hatch is not a second, differently
shaped mechanism bolted onto the first. This is the record's own third
example ("the first with the second deferred", `202607260500:117-119`)
inverted: rather than shipping hand-Zig first and treating the declarative
path as a future addition, IV makes the ALU (II, III, or both) the default
path and hand-Zig the residual, which is the shape op's standing call 4
actually asks for, "known wins are on by default [...] opt-out"
(`202607260100:79-80`) applied to declarativity itself rather than to a
performance optimisation.

**1-2.** Composes I's and II/III's costs and mechanisms directly; no new
mechanism to describe.

**3.** The strongest answer to the dispatch brief's own question. A purely
declarative family is possible whenever every one of its operations
decomposes into the ALU; a family can mix declarative and hand-Zig
operations, decided per operation, not per family, which matches how real
languages are unlikely to be uniformly simple or uniformly exotic across
their whole operator set.

**4.** Inherits II/III's shrunk seam for the covered fraction and I's growing
seam only for the residual, so total seam exposure is proportional to how
much of a real family's operations actually need the escape hatch, an
empirical, currently unmeasured fraction (Bench Plan, Cell 4 names this as a
coverage question, not a timing one).

**5.** Falsified piecewise by whichever of I, II, III falsifies; IV's own
open question is coverage, not cost: what fraction of a realistic family
(arithmetic, comparison, simple structural access) the ALU actually reaches
without escaping to Zig. This is answerable now, cheaply, without waiting for
a runtime bench: write the arithmetic family's operation set against a
concretely-sized draft ALU and count how many operations need the escape
hatch. If arithmetic and comparison, vehje's first real family per the
dispatching topic (`202607260500:11`), needs zero escapes, that is strong
evidence for IV as the default shape; if it needs several, the ALU's
vocabulary is under-sized and should grow before the mechanism is judged.

## 3. Bench plan

Every cell below runs in `mock/benches/` under the harness (per-variant
cdylib isolation, calibrated repetition, committed CSV, per
`202607260100:87-94` and the workspace-wide bench-in-harness rule), never as
a standalone timing loop.

**One pre-existing gap, flagged and not fixed here because it is out of this
document's scope but blocks trusting a sibling bench:**
`mock/benches/dispatch-heavy-ops/heavy.csv` contains only a header row,
`variant,body_weight,ns_per_op,checksum`, with zero data rows, despite
`findings.md` reporting a results table as if the CSV backed it
(`dispatch-heavy-ops/heavy.csv`, checked directly, confirmed empty of data).
The findings document's own numbers may be real (the compiled `b_switch_*` /
`b_tail_*` binaries are present, suggesting the bench was actually run), but
the committed artifact does not back the claim the way
`202607260100:87-94` ("we get all data as csv results, analysable") requires.
This is not this document's mechanism to fix, but it is exactly the kind of
finding this workspace's canon defence asks to be surfaced rather than
quietly worked around: a findings document whose own CSV does not contain
its numbers is a claim without its evidence attached, and it should be
re-run and re-committed before it is cited as settled evidence for anything
downstream (including some of the reasoning in Section 2 above, which cites
it for the *shape* of the result, extra-dispatch-layer cost, not for its
exact numbers).

**Cell 1: family-op dispatch shapes (I vs II).**
Directory: extend `mock/benches/dispatch-heavy-ops/` (same harness, same
shared workload) with two new variants: `hand_zig` (switch dispatches
directly to N hand-authored Zig functions computing simple arithmetic,
already effectively what `b_switch_*` measures) and `alu_tree` (switch
dispatches to a generic tree-walk evaluator over a small comptime-const ALU
tree of the same arithmetic). Workload: the same body-weight sweep (1, 4,
16, 64) BN1-heavy already uses, so the new variants slot into the existing
reference-floor column rather than starting a fresh baseline. Metric: ns/op,
median of 7+ runs, checksum-verified for correctness. Falsifies II as
worth building at all if `alu_tree`'s overhead over `hand_zig` exceeds
roughly the same 2-4 ns/op constant BN1-heavy already found "never gets
repaid" for tail-threading; a smaller or workload-proportional gap would
instead support II as a viable default given its trust-seam advantage
(Section 2, point 4).

**Cell 2: comptime specialisation feasibility (III), sketch first.**
Location: `mock/research/sketches/<timestamp>_alu-comptime-specialisation/`,
per `cl-claim-sketch-discipline.md`, because this is a WORKS/FAILS/
INCONCLUSIVE question before it is a timing question. Build a minimal
comptime-recursive `specialize(comptime tree: AluTree)` over a two-to-four
node ALU tree (e.g. `(a + b) * c`), compile at `-Doptimize=ReleaseFast`, and
read the disassembly. Outcome recorded plainly: WORKS (straight-line code,
no residual branch/loop for the tree structure) or FAILS (a runtime
interpretation loop survives). Only if WORKS does this graduate to a bench
cell (below); if it FAILS, category III is dropped from further
consideration and the record says so, rather than being quietly left as an
untested aspiration.

**Cell 2b (conditional on Cell 2 = WORKS): comptime specialisation cost and
runtime win (III vs I vs II).**
Directory: `mock/benches/alu-comptime-specialization/`. Two things measured
separately, per BN0's own methodology of isolating comptime cost from
runtime cost: (a) comptime evaluation time via `-fno-emit-bin`, swept over
total ALU-tree node count across a realistic family's whole operation table
(tens to low hundreds of nodes), watching specifically for BN0's
superlinearity signature (any node count where time growth outpaces the
node-count growth) rather than assuming BN0's flat-fold linearity transfers;
(b) runtime ns/op of the specialised path against the `hand_zig` and
`alu_tree` variants from Cell 1, same shared workload. Falsifies III as the
default if either axis fails: superlinear comptime cost at realistic family
sizes (BN0's own cliff pattern), or a runtime win over `alu_tree` too small
to justify the extra lowering-correctness obligation named in Section 2
point 4.

**Cell 3: binary size with real (non-identical) per-operation bodies (I vs
II/III).**
Directory: extend `mock/benches/runtime-binary-size/` past its own stated
scope gap ("real families add their distinct semantics code [...] not
measured here", `runtime-binary-size/findings.md:27-28`). Build a small,
realistic arithmetic-and-comparison family (roughly 10-15 operations, per
SP1's ~25-primitive vocabulary scale) under category I (N distinct hand-Zig
functions) and under II/III (the fixed ALU plus N small data trees),
measuring object size for both. Falsifies I's embeddability parity with
II/III if I's size grows materially faster than II/III's fixed-ALU-plus-data
growth, reinforcing the Section-3 "small [...] embeddable" identity claim
(`positive:45`) as a live cost, not a theoretical one, for whichever
category is not chosen.

**Cell 4: ALU coverage for the first real family (IV), not a timing cell.**
Not a bench in the timing sense; a coverage sketch. Draft a concrete ALU
vocabulary (the arithmetic and comparison operators the dispatching topic
names as the first family, `202607260500:11`) and enumerate that family's
actual operation set against it, per operation, recording whether it fits
the ALU or needs the hand-Zig escape hatch. Result: a fraction, committed
alongside whichever design round follows this one, since it directly answers
the dispatch brief's own third question ("whether a purely declarative
family is possible") with a number rather than an assertion.

**What is deliberately not a new cell.** The differential-check harness
(`vehje-runtime-gen::DifferentialCheck`, currently unimplemented,
`lib.rs:83-96`) that runs a census corpus through both a Rust-side reference
evaluator and the specialised Zig artifact is owed under Frontier E
regardless of which category wins (`positive:29`, Section 6). It is not a
which-is-faster fork and does not belong in this bench plan; it is a
prerequisite deliverable for whichever category ships, and should be scoped
in the doc CL that follows this exploration, not benched here.

## 4. What this document does not do

It does not pick a category. Per the dispatching topic's own protocol,
whatever the benches select still needs two independent expert agreements
that it fits the canon before the round records a resolution
(`202607260500:110-113`), and I am one read, not two. What this document
adds beyond `202607260400_family-semantics-as-data.md` is the category space
that memo correctly declined to invent (it stopped at reporting the one
proposal on record and naming the constraints), a from-first-principles
comparison against my own closest prior work (Deegen's one-semantic-definition,
multiple-derived-tiers shape, which maps most directly onto category III and
carries a specific, hard-won warning about trusting an optimiser to specialise
an interpretation loop away without a forced, checked lowering step), and a
concrete, falsifiable bench plan sized to the harness this workspace already
has, rather than a preference among the three shapes the record happened to
contain.
