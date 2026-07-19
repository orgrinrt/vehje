# Vehje's canon, and what this arc owes it

**Date:** 2026-07-19
**Replaces:** `13_vehje_canon_gap.md`, which was wrong. That file read
`vehje/mock/design_rounds/202604191301/202604191300_topic.clause-design-seed.md` as vehje's founding canon
and concluded vehje is a Stellaris DSL whose non-goals forbid a general-purpose core. It is the seed of
**Clause**, the ancestor, in a repo that contains a round literally named `rename-to-vehje`, and that round
states design-round artefacts are historical and not retroactively rewritten. Vehje is not Clause, and
Clause is not its stellar-heritage ancestor. The error was applying
`canonical-design-outranks-intermediate-rounds.md` without first checking that the founding epic belonged to
the same project; that rule assumes lineage continuity, and here there is a rename and a redefinition.

## The actual canon

`vehje/mock/DESIGN.md.tmpl`, which is the authored design and therefore trusted under mockspace's own
docs-are-design contract:

> General-purpose scripting language for game modding and embeddable runtimes. Game-agnostic core;
> game-specific surface ships as extension repos (`vehje-jomini` for Clausewitz, future `vehje-w3`,
> `vehje-ts4`, etc.).

The agent instructions restate it: game-agnostic core, and anything naming a specific game belongs in a
sibling extension repo.

So the grander ambition is not undocumented. It is the first sentence of the design.

## What the canon already settles for this arc

**Many outputs is canon, with four named targets.** `vehje-jomini` for Clausewitz across five Paradox
titles, `vehje-w3` for Witcher 3, `vehje-ts4` for Sims 4, `vehje-lua` for generic Lua. `vehje-codegen` is
described as target-agnostic codegen, and the shipped `TargetRegistry` carries built-in and plugin targets.

**Many inputs is canon by implication.** An extension repo ships "grammar, bind targets, codegen". Grammar
is on the input side, so the extension pattern already spans both ends.

**Embeddability is canon and this arc is an instance of it.** "General-purpose scripting language for game
modding **and embeddable runtimes**." A procedural-documents language embedded in a build tool is exactly
that shape, so mockspace is a legitimate first consumer rather than a divergence.

**No `dyn`, no `TypeId`, no `std::any` in framework code**, compile-time dispatch wins, `no_std` where
practical. Our design already agrees, arrived at independently.

**Foundations are fixed**: `arvo-graph` for the pass DAG, `arvo-bitmask` for read and write sets,
hilavitkutin for the runtime's morsel-driven execution. Reuse, never reimplement.

## What the canon leaves open, and it is exactly what this arc needs

**The extension-point contract is explicitly TBD.** `DESIGN.md.tmpl` says it is "one of the first session's
design rounds". Reading the round list, it never happened. Every round after the rename is maintenance:
a doc audit, review fixes, a `must-use` sweep, two test rehabs, a codegen doc fix, and a build-config
change.

So the thing this arc has spent four review phases designing, how a grammar plugs in on the input side and a
target plugs in on the output side over a shared representation, is precisely the round vehje deferred and
never returned to. That is a much better position than the one my previous file described. The arc is not
serving an undocumented ambition; it is doing the design round vehje owes itself.

**The crate split is provisional, not binding.** Named as such in both the design and the agent
instructions.

## The fork this arc never considered

The canon's two-language strategy: a **Rust compiler** and a **Zig runtime** joined by a C ABI defined and
validated in Rust. The runtime is what executes macros, scratch variables and generative pipelines. It is
small, embeddable, speaks C ABI, and pulls no Rust runtime.

This arc assumed a Rust interpreter throughout. Under the canon, the thing that evaluates is Zig.

Three possible resolutions, and the choice is a real one:

The procedural-docs evaluator is a Rust-side const-evaluation facility in the compiler, distinct from the
Zig runtime, in which case the canon is untouched but there are two evaluators in the project.

mockspace embeds the Zig runtime over the C ABI, which matches the canon exactly and makes mockspace the
first proof that the runtime is genuinely embeddable, at the cost of a C ABI dependency in a docs tool.

The canon's runtime split is revisited, which is a vehje decision and not this arc's to make.

Nothing in four phases of review touched this, because the arc never read the canon.

## What this arc must not do

**Do not let the nine-form core and the document algebra become vehje's core.** They are one grammar and one
family, serving one consumer. Under the canon they are a plugin, on equal footing with `vehje-jomini`'s
Clausewitz grammar and a future `vehje-lua`. If the shared representation is shaped around what markdown
generation needs, the extension point is compromised for every other target, which is the specific way this
arc could limit vehje rather than serve it.

**Do not settle the extension-point contract inside mockspace's research directory.** It is vehje's
deferred round and belongs in vehje's `design_rounds/`. This arc can supply the material; it should not
supply the venue.

**Do not carry the mockspace-shaped assumptions into the contract.** Two that this arc baked in and should
re-derive generally: that a target's output is text (a native or LLVM target is named in the canon's own
backlog), and that evaluation happens at generation time in the same process (the canon's runtime is a
separate embeddable artefact behind a C ABI).

## What to do

The material this arc produced is largely the right material for vehje's deferred extension-point round: the
inclusion check, the family model, target-decided staging under its proper name of polymorphic embedding,
the total-renderer contract, and the measured cost model.

It should be re-derived from vehje's canon rather than from mockspace's use case, and land as a vehje design
round. Then mockspace's procedural-docs language is written against that contract as its first consumer,
which is what the canon says a consumer does.

## The arc inverted its own seed, and the panels inherited the inversion

Added after the maintainer pointed out that nothing in the arc's founding framing was markdown-specific or
mockspace-specific.

`00_seed.md`'s parts table assigns **ten of twelve parts to vehje crates**: `vehje-lex`, `vehje-ir`,
`vehje-syntax`, `vehje-resolve`, `vehje-typecheck`, plus the interpreter, the data sources and operators,
and a text backend named "beside the Clausewitz backend". It states plainly that "P11 and P12 are the only
parts specific to mockspace." The architecture line is "many syntaxes to one IR to many outputs", and the
registry DSL is described as "a **second front end**, not a second language".

`panel/00_context.md:12-17` says "The immediate consumer is **mockspace**" and "The longer-range consumer
is **vehje**", and names `09_shape.md` the primary artefact, a document containing a section titled "Held
against the mockspace DSL".

That is the inversion. The seed designs vehje and treats mockspace as one consumer with two parts. The panel
briefs designed mockspace and treated vehje as a future consideration. All sixteen expert files inherited it.

### What the inversion invalidated

**Carmack's central reduction.** "A relational fact base with a template splicer, not a compiler" is true of
mockspace's current corpus and false of vehje, which compiles mod source to Clausewitz, Lua, Witcher 3
script and, per its own backlog, native code. The reduction was aimed at the wrong artefact.

**Every cost measurement in the round.** 123 microseconds, 2.3 milliseconds, 229 milliseconds, the
group-by inversion, the parse-dominance finding: all measured over mockspace's registry corpus. Vehje's
workload is compiling a mod tree to game script. Different in magnitude and different in shape.

**Quilez's reduction, and the nine-form count itself.** Both were argued from a corpus with zero call syntax
and zero iteration. That corpus is mockspace's, and it lacks those constructs because the feature does not
exist yet. Vehje's language has traits, impls, generics, coherence and the orphan rule, events and
manifests. Nine forms may be far too few, and nothing in this round tested that, because nothing in this
round asked what vehje's language needs.

**The document algebra's position.** Under the seed it is P10, a text backend beside the Clausewitz backend.
This arc promoted it into the core discussion and spent four phases sizing it.

### What survives the reframing

The mechanisms that are genuinely target-agnostic, which is most of the good material: inclusion rather than
coverage for family checking, target-decided staging under its proper name of polymorphic embedding, total
renderers per declared family, structural hashing as identity, the explicit frame stack with a branch quota,
and the borrowed non-allocating scope chain. Those were derived from the shared-core problem and do not
depend on which consumer asked.

What must be re-derived rather than carried: anything sized, counted, or justified against mockspace's
corpus.

### The correct shape of the work

Design vehje's extension-point contract, which is the round vehje deferred and never held, from vehje's own
needs across its four named targets. Then write mockspace's procedural-docs language against that contract
as its first consumer, which is what `00_seed.md` said in the first place and what the canon says a consumer
does.
