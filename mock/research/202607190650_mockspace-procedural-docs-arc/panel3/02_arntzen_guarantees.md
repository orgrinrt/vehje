# Making the guarantees hold (Hans-Kristian Arntzen)

## The judgement in one line

Four of the five reported guarantee failures dissolve with a stated invariant rather than a redesign; the
fifth (nested iteration) dissolves too, once the round admits it already has the fact it needs (static
namespace sizes) and stops treating a two-case join-order choice as if it required a real optimizer.

## Where I build on Karis, and where I part from him

Karis's inversion (loop inversion over an enumerable free variable) is the mechanism every guarantee below
leans on: a bound whose range is known before evaluation turns a product into a group-by. I extend it one
level. His own "does not invert" case, an inner binding sourced from an outer row's *computed* value, is a
join. I agree it is a join. I part on "genuinely blocked": every nested query this corpus can express joins
two relations of statically known size (namespaces are fixed tables, known the moment loading finishes, per
`10_syntax_correction.md:14-16`), so choosing an order is not a cost-based optimizer problem, it is a
comparison of two integers computed once. He measured the wall correctly and stopped one inference short of
dissolving it.

## Check before emit, recovered

Quilez's `doc & !tgt == 0` and Giesen's monotone two-level check are the same operation on two different
operands, and the fast path's soundness is exactly what Giesen states: the parse-time mask is a superset of
the exact staged mask by construction, so `parse_mask ⊆ target` implies `exact_mask ⊆ target`. That much is
proven.

What the guarantee actually says, once recovered, is narrower than `08_decisions.md:60`'s wording ("prove
that what a document contains is within what the target declares"). It proves inclusion of what *this target,
under its own staging policy,* will actually be handed, not a target-independent property of the source text.
Two targets can legitimately disagree about one document. Giesen states this correctly in one sentence
(`04_giesen_synthesis.md:415-416`); it should move from a synthesis aside into the decision's own text, since
the original phrasing is exactly what invited Muratori's three-year misreading.

One invariant nobody wrote down: the exact mask under a `Preserve`-policy target must equal the parse-time
mask restricted to the preserved subtree, not the reduced-branch mask. A `Select` that survives into emitted
Lua keeps both arms as code; if the exact pass narrows to the taken arm the way it correctly does under
`Reduce`, an unsupported construct in the untaken arm ships with no refusal, reintroduced by policy rather
than by staging order. Mask-accumulation and the reduce/preserve decision must be the same predicate over the
same node, computed once, not two independently-reasoned walks that happen to agree today. State it at the
point where staging and mask-accumulation share the fold, or a later "optimization" that separates them
reopens the hole with no test to catch it.

## The proof split, and whether it can be dissolved

It does not dissolve. Commit-gate compilation (Q3-B) is the right move; it changes *when* the check runs and
*who* it blocks, not *what kind* of proof it is. A `rustc` `const {}` block cannot be bypassed short of
editing the assertion itself, which is reviewed source. A commit-gate check is a fallible external process: it
depends on the ruleset blocking direct pushes, on the hook running the identical checker render time uses, and
on nobody invoking `--no-verify`. None of those three is proven in this round; they are workspace policy
(`branch-pr-flow.md`, `strict-by-design-quality-pressure.md`) assumed to hold here. The honest claim is "no
document that reaches `dev` through the standard flow is unchecked," not "documents are statically checked."

A second gap is unpriced. Root cause A's extension (`04_giesen_synthesis.md:273-281`) makes a document's
family set depend transitively on registry content, not on the template file alone. The commit-gate's
dirty-set computation must invalidate a document's cached check on any change to a row in its transitive read
set, not only on a template-file diff. If the dirty-set watches only `.md.tmpl` files, a `what` field growing
a `Table` mid-edit re-renders unchecked content through a stale, still-passing serialized arena. Catalogue
this as a red test now (edit a registry row a cached document reads, template untouched, confirm the gate
re-checks it) before commit-gate compilation ships, or the exact failure this split exists to close reopens
through the one input nobody watched.

## The identity machinery: what a structural hash actually guarantees

Stachowiak's collapse is sound in direction; Tatarchuk's widening (4 to 8 bytes, ~4e-12 collision at current
corpus) is the right arithmetic for "how often does an accident happen," and the wrong question for a
soundness claim. A hash of width W guarantees equal structures hash equal. It does not guarantee the converse:
different structures collide with probability roughly n²/2^(W+1), small but nonzero, and growing with the
corpus. What matters is what happens the day it fires: does the interner treat the collision as identity
(silently merges two different subtrees, wrong render, no diagnostic) or detect it?

Nobody asked, and the codebase already answers it for a narrower case. `hilavitkutin-str/src/interner.rs:81-89`,
`lookup_const_by_value`, checks `entry.hash == want && str_eq(entry.value, s)`, hash then content, because the
const table's own 28-bit truncated FNV1a does collide at that width. Node identity needs the identical
discipline: the hash is the candidate slot, an equality check on insert proves two nodes are the same node,
and it costs one branch, almost always true. Without it, "identity = hash" has a silent wrong-output failure
mode that gets more likely with scale, the opposite of the workspace's fail-loudly-at-the-bound discipline
everywhere else in this design. Write the check in the same construction pass that computes the hash; it is
free relative to what the constructor already does.

## The boundary contracts, stated

Three edges, three owed things, none written down as a contract.

mockspace to the language crates: mockspace is `std`, the crates are `no_std`/no-alloc
(`09_shape.md:223-227`). mockspace owes never forcing the no_std side to allocate to *consume* what mockspace
hands it (views, not owned buffers). No API shape for this boundary exists anywhere in this round; it is
assumed, not designed.

The language crates to the stack (`notko`, `arvo`, `hilavitkutin-str`), after P6's cycle fix: the stack owes
`no_std`, `Copy`, no heap. Missing is the reverse. `no-legacy-shims-pre-1.0.md` means arvo and hilavitkutin-str
can break their public surface with no deprecation window, at any time. Pesce's Q8 justification (own-crate,
`pandoc-types`-style independent versioning) is stability the language crates want to offer their own
consumers; they get none of it from upstream. A breaking arvo change lands in their next `cargo update` with
no migration window, the exact property Q8 is trying to buy.

The shared representation to vehje: vehje is named as the longer-range consumer and nothing here measures
against it. Source positions distinct from mockspace's, a separately-tunable frame-stack bound for larger
programs, anything vehje-specific, is unasked. Building Q8-A now, with vehje unvalidated, is a legitimate bet
on a known consumer with the door left open. Record it as a bet, not a proven fit.

## The nested-iteration gap

Not irreducible. Karis's own case, `crate::all` as the outer binder, already inverts by his own mechanism,
since a namespace has a statically known range, same shape as `self`. The case that genuinely resists a
single pass is an inner predicate keyed on a value computed by an outer *query*. That is a join, and the
"order" question is real, but it is a small, fully-determined instance of a solved problem: because both
relations in a typed-reference join have statically known row counts once the registry loads
(`10_syntax_correction.md`), choosing which side indexes and which side probes is a comparison of two
integers, computed once, deterministically, at the same commit-gate pass that already builds T1's index. No
runtime cost model is needed, because nothing about the choice depends on runtime state.

This changes the recommendation. Tatarchuk's T7 ranks method-chain-now, planner-shaped-table-later. I would
build the static half of the planner now, alongside T1, because it costs the same lookup T1 already performs,
and its absence has a silent failure mode: the first nested query anyone writes picks an accidentally
quadratic order with no diagnostic, the identical class of unpriced cost Carmack's threshold rule produced
elsewhere in this round. Deferring predicate and projection pushdown is fine. Deferring a two-integer,
zero-runtime-cost order choice is not deferring difficulty, it is deferring a bug.

## What I could not make hold, and the ground I covered

I could not make the compile-time and gate-time proofs the *same category* of guarantee. I looked for a
formulation where the distinction dissolves entirely (codegen templates into rustc-verified statics, `09_shape.md`
Q3 option C) and it costs exactly what Giesen's synthesis says: authoring latency coupled to a rebuild, a
smaller cousin of the cargo cycle re-created. The distinction is real; document it as real rather than argue
it away.

I did not price the collision-detection branch's cost at 100x corpus, though it should sit under a
microsecond per insert given the interner already pays a hash-table lookup there.

## Open provocations for the panellist after me

1. Write the reduce-versus-preserve mask invariant as a catalogue test before the two-level check ships: a
   `Preserve`-target document with an unsupported construct in the untaken arm of a preserved `Select` must be
   refused, not accepted.
2. Write the registry-row-dirty-set test before commit-gate compilation ships: editing a row inside a cached
   document's transitive read set, template untouched, must invalidate that document's cached check.
3. Add the hash-then-verify branch to the node interner's insert path and cite
   `hilavitkutin-str/src/interner.rs:81-89` as precedent in the implementing CL.
4. Name the versioning contract the language crates want from arvo and hilavitkutin-str, or accept in writing
   that Q8's stability is one-directional and the language crates inherit upstream churn.
5. Build the static join-order lookup alongside T1's index rather than after it. Bench whether it deserves a
   named function or is genuinely free to inline at every nested-query site.
