# Family-operation semantics as data: gate and analysis

**Date:** 2026-07-26
**Dispatched by:** `mock/design_rounds/202607260500_topic.the-languageauthor-specialisation-stage.md`,
which names this file as its deliverable and states plainly: "if the canon
leaves it open, the call is the lead designer's and this topic records the
question rather than an answer" (`202607260500:92-94`). That is the outcome
below.

## Gate outcome: passed, with one pre-existing misalignment already named by canon

The question this file answers, by what representation a family operation's
semantics cross from Rust to Zig as data, is licensed work. It is not a
reopening of a settled call; it is the literal content of op's standing call 2
(`mock/research/202607260100_op-standing-design-calls.md:27-48`), which states
the property and leaves the mechanism to be designed. Nothing here proposes to
build outside the canon's artifact boundaries.

One misalignment is already on the record and is restated here only because
the dispatch brief asked me to check it, not because it is new information.
`mock/runtime-zig/src/runtime.zig:410-414` dispatches every `Raw` node,
arithmetic included, to a host callback:

```
TAG_RAW => {
    // A family operation. The runtime does not know what the family
    // means; it evaluates the operands and hands them to whoever
    // declared it, which is what keeps the framework family-free.
    const h = host orelse return EvalError.NoHost;
```

That is exactly the shape op's standing call 2 names and rejects: "This
corrects the shape that had shipped: arithmetic lowered to `Raw(ARITH, ...)`
and crossed the C ABI to a host-provided handler, which meant the language's
elementary operators existed only as a function in a test file"
(`202607260100:38-40`). The shape was built deliberately and recently.
`mock/design_rounds/202607251140/202607251140_topic.family-operations-reach-their-handler.md`
(locked doc and src CL, dated 2026-07-25) states its own scope in the same
breath as the thing it built: "A `Raw` node evaluates its payload and
dispatches to a host-provided handler, which is how arithmetic, comparison,
and every host service reach a running program" (`202607251140:5`), and its
own deferral list files the corrective mechanism as an optimisation to add
later: "Comptime specialisation of known families into the generated runtime,
which is an optimisation of this seam rather than a different one"
(`202607251140:37`). Op's call, one day later, says plainly that framing was
wrong: "The correction is not 'move the handler into the runtime as a
default'; it is that computation belongs to the runtime and never depended on
the host" (`202607260100:41-42`). A full mockspace round (topic, doc CL, src
CL, both locked) was spent shipping a mechanism whose own governing call
rejects it the next day. That is not a small thing to have gotten wrong: the
comment left in the code, "keeps the framework family-free," reads as a
principled defence of the exact shape the design record kills, because it
quotes the framework's real identity (positive:45, below) to justify a
decision the identity does not license.

One nuance, stated so the correction is not overstated: op's call scopes to
"elementary computation" ("arithmetic, comparison, and any other elementary
computation," `202607260100:35-36`). It does not forbid a host callback
outright for operations that are genuinely runtime-effects the host services
(file I/O, environment queries, anything Frontier B's letter actually
classifies as a runtime-effect operation, `positive:23`). What is wrong is the
202607251140 round's blanket framing that put arithmetic in the same bucket as
those, and the shipped `Raw` payload having nowhere else to go. The fix this
gate motivates is narrowing the scope of what reaches the host, not deleting
host dispatch as a concept.

The rest of this file is the analysis the topic asked for.

## 1. What the canon fixes, and what it leaves open

**Fixed: the property.** The runtime computes without the host's help. "The
host's job is configuration, lent allocations, and receiving results. That is
the whole of it. The host does not supply arithmetic, comparison, or any other
elementary computation, and a design that requires it to is backwards"
(`202607260100:34-36`). This is a mandate, not a proposal (per the standing
calls doc's own precedence rule, `202607260100:9-11`: "where a call names a
property, the property is a mandate").

**Fixed: which artifact owns the computation.** Not the framework's general
engine. "The framework still owns no family. vehje the framework is generic
over families. The first-party Clause runtime is a different artifact from
the framework's general engine, and it ships its own computation. The
separation of concerns survives; what changes is which side of it arithmetic
was on" (`202607260100:44-48`). Read against Section 3's description of the
shipped artifact, "a shipped Zig composed runtime (one hand-authored engine
comptime-specialised to the Rust-emitted data)" (`positive:45`), this fixes
that the computation lives in the *specialised* artifact, not in the generic
engine that exists before specialisation. Frontier A names the stage this
specialisation happens at: "Ahead-of-time work lives at LanguageAuthor
(specialising the general Zig runtime to the language definition at our
build, which is the first Futamura projection as a build step, the technique
weval demonstrates for wasm interpreters in production)" (`positive:21`).

**Fixed: the medium.** Whatever crosses, it is data, never Zig source text.
Frontier E: "The proof is discharged in Rust before any lowering, then
compiled into two artifacts that carry no prover... Rust emits validated
*data*, never Zig *source* (the LMS-hard, hard-to-certify route, rejected at
topic 1627)" (`positive:29`). The negative catalogue kills the alternative by
name: "'Rust emits Zig source' (a Rust metaprogram text-printing the
specialised engine). Killed at topic 1627 as the hard-to-certify LMS-hard
route, Rust emits data, Zig comptime specialises" (`negative:37`).

**Fixed: the certification perimeter, which bounds how much the mechanism can
be trusted to prove regardless of its shape.** "rustc certifies
well-formedness, inclusion, and grading of the emitted data; Zig comptime
certifies the specialisation type-checks against that data; **neither
certifies semantic preservation of any lowering**" (`positive:29`, emphasis in
source). Whatever representation is chosen for a family operation's meaning,
the architecture does not promise that the specialised evaluator computes the
declared semantics correctly. That is measured, not proven, consistent with
"assurance a dial" in the same Frontier and with the census-corpus
differential check named in Frontier E and Section 6.

**Fixed: the handler-discipline coordinate, negatively.** Frontier B states
the axis a handled operation is placed on: "a host-call is a runtime-effect
operation the host handles" (`positive:23`). `vehje-ir`'s own `Raw`/`Perform`
split states that arithmetic is deliberately *not* modelled as an effect:
"Deliberately not folded into `Node::Raw`, and the reason is typing rather
than ergonomics: arithmetic lowers to `Raw`, so performing through `Raw`
would give `1 + 2` an effect, and that is a grade the handler discipline
would then have to subtract everywhere" (`mock/crates/vehje-ir/src/node.rs:156-159`).
Read together, these two statements are jointly load-bearing for op's
correction rather than merely consistent with it: arithmetic is declared pure
by the framework's own type, so routing it to a runtime-effect handler (a
host call) was never a coherent reading of Frontier B's own letter, independent
of op's call. Op's call states the property; the Core's own typing already
denies the mechanism that violated it.

**Open: the concrete data representation for what an operation computes.**
Nothing in either catalogue or in the standing calls doc specifies a data
shape from which Zig comptime derives arbitrary computational behaviour. The
closest the canon comes is naming the technique class (comptime specialisation,
the first Futamura projection) and the artifacts that must carry it (Rust data,
a hand-authored Zig engine); it does not name the schema for a "family table"
entry, does not say whether an operation's meaning is a selector into a
hand-authored implementation set, an interpretable micro-language, or
something else, and does not resolve whether `vehje-signature::Operation`
gains a semantics field at all, or whether semantics live entirely outside
`vehje-signature`.

Three independent primary sources, spanning the entire life of this design,
converge on stating this openly rather than resolving it:

- The proposal that introduced the mechanism class states the question as
  unresolved: "it is an open question whether the certified-generation
  technique reaches only the structural discipline (with family semantics
  authored and tested like any runtime's interpreter) or can be pushed
  further into the semantics"
  (`mock/design_rounds/202607240130/202607201627_topic.compiling-the-proof-into-generated-code.md:126-128`).
- The panel that audited that proposal adopted, as its own unifying frame,
  the reading that keeps semantics outside the certified system: "family
  semantics sit outside the verification system (tested)"
  (`mock/design_rounds/202607240130/202607201845_topic.certified-generation-panel-outcome.md:52`).
  This is a locked topic in a closed design round, not canon; it is evidence
  of where the question stood, not an answer the current canon restates.
- The most recent audit, written after the canon and explicitly unratified,
  says the same thing in the present tense: "Whether Clause arithmetic is the
  first exercise of that certgen path or a hand-authored family in a consumer
  repo decides a large amount of build order, and no memo takes a position"
  (`mock/research/202607260200_comprehension_lattner.md:86`).

Grepping the two canon catalogues and the standing calls doc for the
vocabulary the 1627/1845 line of work used to describe a candidate mechanism
("name-keyed", "@field", "handler namespace", "family semantics") returns
zero matches. Whatever was proposed there did not get carried forward into
ratified text.

## 2. Skipped

Per the gate: the representation is open, so there is no fixed concrete shape
to report, and no statement of what `generate` must emit for arithmetic
specifically. See item 3.

## 3. The representation is open. Stopping here, per the gate.

I am not proposing a resolution. The topic that dispatched this question says
plainly that if the canon leaves the representation open, "the call is the
lead designer's" (`202607260500:93-94`), and the workspace's canon-defence
discipline says the same thing more generally: where the canon is ambiguous,
"stop and hand the call back rather than resolving it yourself." Both apply
here without qualification. What follows is not a recommendation; it is a
report of what the design record already contains, so the lead designer is
choosing between real, dated proposals rather than starting from nothing.

One concrete mechanism was proposed, audited, and never ratified. The 1627
proposal's Zig-end enhancements included "name-keyed comptime dispatch
(`inline else` over the tag, `@field` on the handler namespace by tag name)"
as the fix for a distinct problem (a switch that is exhaustive but whose
tag-to-handler wiring can still transpose), stated by the panel reviewer:
"The exhaustive-switch discussion certifies totality of dispatch but not
correctness of the tag-to-handler mapping... The fix is structural: name-keyed
comptime dispatch (`inline else` over the tag, `@field` on the handler
namespace by tag name), so a mismatch is a missing-decl compile error rather
than a silent transposition"
(`mock/research/202607201641_certified-generation-panel/02_giesen.md:74-77`).
Under this shape, an operation's *meaning* would live as a hand-authored Zig
function in a per-consumer namespace, and what Rust emits as data would be
the operation's name (or id) plus its structural metadata (arity, effect
grade, lease rule), which comptime resolves against that namespace by name.
The same panel is explicit that this closes a routing question, not a
semantic one: "A total, name-keyed, exhaustively-switched dispatch can still
route to a handler whose body computes the wrong semantics; audit 2's
name-keyed dispatch narrows the transposition risk and closes none of [the
semantic-correctness question]"
(`mock/research/202607201641_certified-generation-panel/04_theory-review.md:72-73`).
That residual (semantic correctness is tested, not proven) is consistent with
the certification perimeter the canon does fix (`positive:29`, above), which
is the one point of contact between this unratified mechanism and ratified
text. It is not itself ratified, and the two most recent audits (`comprehension_lattner.md`
and this file's own reading) found no canon text that adopts it.

Two structural facts about this candidate, reported without endorsing it,
because they bound what any resolution has to answer regardless of which
concrete mechanism is picked: if semantics live in a hand-authored Zig
namespace, the framework's general engine stays family-free exactly as
positive:45 requires (the namespace is part of the specialised, per-consumer
artifact, never the framework's own source), and the "arbitrary operation"
problem the dispatch brief names as the crux dissolves because the data never
tries to encode arbitrary computation. It only encodes which name to bind.
The corresponding question a hand-authored-namespace answer has to close, and
does not itself answer, is what happens for a family whose author does not
want to hand-write Zig at all, which is a real case if any consumer wants a
purely-declarative family; nothing in the record picks between "every family
author writes Zig for its semantics" and "some families get a genuinely
interpretable data language, priced at the interpreter-for-an-interpreter
cost the topic itself names" (`202607260500:69-70`: "something richer risks
becoming an interpreter for a second IR").

## 4. The constraints that bind whatever the lead designer picks

**`no_std`, no alloc, on both sides.** `vehje-signature` and `vehje-runtime-gen`
are `#![no_std]` with no `alloc::*` (`mock/crates/vehje-signature/src/lib.rs:13`,
`mock/crates/vehje-runtime-gen/src/lib.rs:14`, both `#![deny(unused, ...)]`
crates reading from caller-provided slices). The Zig runtime allocates
nothing; the module doc states it plainly: "Nothing is freed within a run;
exhausting the lent arena is `EnvFull`" (`runtime.zig:17`), and the file's own
`ARG_CAP`/scratch/session machinery is uniformly host-lent, fixed-capacity,
never growing. Any representation for family-operation semantics has to fit
this: a caller-lent slice or a comptime-known constant, never a Rust-side
`Vec` of anything or a Zig-side dynamically sized table.

**The C ABI.** Whatever crosses from the Rust-emitted package to the Zig
build is bytes (`vehje-runtime-gen::Slice.bytes: &'a [u8]`,
`mock/crates/vehje-runtime-gen/src/lib.rs:53`), consumed by Zig at comptime
via `@embedFile` or an equivalent const-import, not by a runtime FFI call.
This is a different crossing from the existing runtime C ABI
(`vehje_runtime_execute`, `host.zig`'s `VehjeHost`/`VehjeOperand`), which is
the crossing op's call forbids using for elementary computation. The two must
not be conflated: the LanguageAuthor-time data crossing (build time, into
comptime) is the one this design owes; the existing runtime C ABI (call time,
`extern fn`) is what remains for genuine runtime-effect host calls.

**Zig 0.16 comptime's actual capabilities, not the design record's stale
assumptions about it.** The one sketch that touched this ground directly
found a load-bearing toolchain gap between what the pre-1845 design record
assumed and what the pinned compiler does: "Topics 1627 and 1845 name comptime
`@Type` reification as the mechanism that generates the IR type from data...
Zig 0.16 removed the monolithic `@Type` builtin. The full 0.16 builtin set has
no `@Type` and no `Reify`; reification is now a family of specialised
builtins: `@Enum`, `@Union`, `@Struct`, `@Int`, `@Fn`, `@Pointer`, `@Frame`,
`@Tuple`, `@Vector`, `@EnumLiteral`. The capability survives (this sketch uses
`@Enum(u8, .exhaustive, names, values)` and it works), so cert-gen is not
blocked"
(`mock/research/sketches/202607210300_sk1_certified-generation-core/findings.md:39-46`).
SK1's `engine.zig` demonstrates the surviving capability concretely: a
`FamilyTag` enum generated from `data.FAMILIES` via `@Enum`
(`mock/research/sketches/202607210300_sk1_certified-generation-core/engine.zig:11-19`)
and an exhaustive `switch` over it (`engine.zig:24-28`). What SK1 does **not**
demonstrate is a dispatch arm whose body does anything beyond `@tagName`; its
`dispatch` function is a stub. Whatever representation is chosen, it inherits
SK1's demonstrated (not merely claimed) floor, comptime-generated enums and
exhaustive switches over Rust-emitted id/name data, and still owes its own
sketch for whatever the switch arms actually do.

**No trusted Rust-to-Zig emitter seam.** Frontier F names the seam this
design cannot lean on as already sound: "The trust seam is the Rust-to-Zig
emitter; closing it (round-trip validation, or the lens projection, or a
self-checking typed manifest) is *intended*, the mechanism open"
(`positive:31`). Whatever crosses as the semantics representation adds to
what that seam has to validate. A representation whose correctness depends on
Rust and Zig agreeing on an implicit convention (for example, an id ordering
that is never checked, or a name string whose spelling is never verified
against the Zig namespace) widens the unclosed seam rather than living inside
it. The `vehje-runtime-gen::Manifest` machinery (content-addressing each
slice, `mock/crates/vehje-runtime-gen/src/lib.rs:66-81`) is the seam's current
belt; it hashes bytes, not meaning, so it catches a changed package but not a
package whose semantics field was always wrong.

**Nothing here is licence to widen `vehje-ir::Raw`'s payload without a design
round.** `Node::Raw`'s payload is explicitly marked incomplete in source:
"the payload is a family-interpreted handle; M0 carries only the family id
and a node-list placeholder until the first family defines its payload
encoding" (`mock/crates/vehje-ir/src/node.rs:130-132`). Whatever the lead
designer picks changes this FIXME into a real encoding; it is not something
this file, or the dispatching topic, has authority to settle by itself.

## What this file does not do

It does not choose between the hand-authored-namespace shape and any
alternative. It does not touch `vehje-signature::Operation`, `vehje-runtime-gen::generate`,
or `runtime.zig`. Per `202607260500`'s own stated protocol, a second
independent read is owed before the topic records anything as resolved, and
even that second read only closes the question if it finds canon text this
one missed; absent that, the call is the lead designer's, not two experts'
agreement on which unratified proposal reads best.
