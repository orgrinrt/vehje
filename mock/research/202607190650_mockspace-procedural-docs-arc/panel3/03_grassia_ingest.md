# Fact ingest and composition (F. Sebastian Grassia)

## The judgement in one line

The migration this round calls unpriced has already been performed once and left its receipt in the data:
`ikiuni_renderer`'s 2686 rows were drained from a frozen 203,414-word prose corpus under 3475 line citations,
which means the mechanism for facts with no machine-readable source exists, has run at scale, and the round
has been pricing the wrong reference population by 5.7x while ignoring it.

## Where I build on Karis and Arntzen, and where I part from them

Karis measured 441 arvo rows derivable at zero authoring cost and correctly identified the remainder as schema
design for domains with no machine-readable source. I part from his framing of that remainder as unpriced. It
is priced, in this repository, by an executed instance.

Arntzen's discipline of demanding the stated invariant rather than the redesign is the one I apply to schema
evolution below, where the round has 144 hand-declared field slots and no invariant at all.

Both, and every panellist before them, priced references at 1007. That is the count in `.md.tmpl` files.
Measured inside `mock/registry/`: **2286 `crates::` references and 3475 `seed::` citations, 5761 total**. The
references overwhelmingly live in the data, not in the prose, and every scale, index, and cycle-detection
argument in this round has been sized against 17 percent of the population it must handle.

## Facts with no machine-readable source (the mechanism, and its real cost)

Pesce's "there is no migrated corpus anywhere to learn from" is false. `ikiuni_renderer/mock/research/seed/`
is 7 files and 203,414 words, declared at `mock/mockspace.toml:383-389` as a reference root with
`frozen = true`, and the comment states why: the corpus is the oracle and is not edited, which is what makes
line citations into it honest. 2071 of 2686 rows carry a `provenance` array of `seed::FILE::LINE` triples.

That is semi-automatic extraction, already designed. The mechanism, generalised:

1. **Freeze the prose as a corpus, do not migrate it.** The prose is not converted and deleted; it becomes an
   immutable oracle addressed by line. This is the move that makes extraction incremental, because the corpus
   stays wholly usable while zero, half or all of it has been drained.
2. **Every extracted row cites its lines.** A citation is checkable (the file must exist, the line must be
   inside it) so extraction cannot silently invent a fact.
3. **A row is drained, not moved.** `seed_id`, declared `visibility = "internal"` in 15 namespaces, exists so a
   second pass can tell an already-drained source item from a new one. That field is the incremental-extraction
   cursor, and it is why the human's role is adjudication over a diminishing frontier rather than authoring.

The human's role is: propose (tool), adjudicate (human), cite (tool, from the adjudicated span). Cost, measured
against the only executed instance: **75.7 source words per row**, not Pesce's 46, whose denominator was
template words rather than corpus words. arvo's 51,486 words imply ~680 rows against Pesce's ~1100 estimate,
and Karis removes 441 of those, leaving roughly 240 genuinely hand-adjudicated rows for a repository.

The gap that is real and that I will not paper over: `frozen = true` requires a corpus that is *finished*.
arvo's 67 templates are live documents under active edit. Freezing a snapshot as the extraction oracle while
the live templates keep moving means the two diverge during the drain. Nobody has designed that, and it is the
one place where ikiuni_renderer's executed instance does not transfer.

## Composition: what USD teaches and what it over-imposes

USD's composition model answers precisely this problem: a value at one address assembled from many contributing
sources under a deterministic strength ordering. It teaches two things and over-imposes four.

**Take: resolution is per-field, not per-row.** In USD the weakest layer supplying a value wins only where no
stronger one speaks. A row whose `crates` comes from a source scan and whose `note` is hand-written is not two
rows; it is one address with two contributors. The registry needs exactly this, and today has no notion of it.

**Take: strength ordering is declared once, globally, not per-address.** LIVRPS is a fixed sequence, which is
why USD composition is debuggable at all. The registry's ordering should be three layers and fixed:
`derived < schema-default < authored`. An authored cell always wins; a derived cell fills silence; a
schema-default fills what neither supplied. Three, declared once.

**Over-imposes: variants, inherits, specialises, and references-as-arcs.** All four exist to support asset
instancing, where one prim is stamped into many scenes under per-scene overrides. There are no scenes here.
There is one registry, read by many documents, and a document is a reader, not an override site. Importing
those arcs would let a document mutate the registry it reads, which is the two-sources-of-truth failure the
project exists to remove.

**Over-imposes: composition as a graph.** USD's arcs compose recursively and the index is a real datastructure.
Three fixed strengths over a flat address space is an array lookup with a first-non-empty scan. Adopt the value
resolution; refuse the index.

## Derived facts: stored or computed, and what provenance owes a reader

**Computed, and the system already proves it works.** `crates` is a reserved root
(`mockspace/src/registry/model.rs:285`) with **zero authored rows and 2286 references**, resolved by a
filesystem directory probe at `resolve.rs:359-368`. A namespace carrying more references than any authored
namespace already exists with nothing stored. Karis's 441 rows are the same shape: derive at load, into the
same row store, indistinguishable to a query.

Storing them is wrong for the reason it is always wrong: the derived copy and the source drift, and the copy
is what the reader sees. Computing them costs the load-time scan Karis already measured inside the 19.6 ms
cold path and zero on the warm path if the snapshot carries them.

**What a reader owes.** The `provenance` field cannot carry this. It is already taken: it means "which lines of
the frozen oracle this row was drained from," and it is `internal = true`, dropped on the way out. Derivation
is a different axis and needs its own: a per-cell layer tag, three values, set by the resolver, never authored.
The reader-facing consequence is one sentence: a derived cell may not be hand-edited, and the diagnostic when
someone tries names the source file that owns it.

**Staleness.** A derived cell has no staleness, because it is not stored. What has staleness is the *reference*
into a derived namespace: rename a crate directory and 2286 references resolve to `None`. That is already a
dangling-reference diagnostic and is the correct failure.

## Schema evolution across years and nine repositories

The round treats this as a future problem. Measured, it has already happened.

`ikiuni_renderer/mock/mockspace.toml` is 1212 lines declaring 15 namespaces and **144 field slots**. Five field
names account for **73 of the 144, 51 percent**: `note` 16 times, `seed_id` 15, `status` 14, `provenance` 14,
`crates` 14. They are copy-paste, and they have already forked: `crates` carries **5 distinct descriptions**
across its 14 declarations, `note` 5 across 16, `provenance` 3 across 14. `vocab` declares `note` **twice**,
at lines 423-425 and 447-449, with different text. That is a duplicate field declaration shipped today.

So the mechanism is not speculative, it is overdue:

**A shared-field library, and namespaces cite it.** `provenance`, `crates`, `status`, `note`, `seed_id` are
declared once and included by name. 73 slots collapse to 5 plus 73 one-line citations. A description edit is
one edit, and the fork that exists cannot recur.

**The first migration is already specified and is a retype.** `10_syntax_correction.md:70-78` requires `crates`
to move from `string[]` to `ref<root>[]`. Today that is 14 declarations, 5 descriptions, 1658 populated cells
and 2286 reference occurrences. What makes it safe is only this: the retype is *checked against the data before
it lands*. A field retype declares the old type, the new type, and runs the new type's validator across every
populated cell, reporting the cells that fail. 1658 cells is a millisecond. Landing a retype whose validator
was never run against the corpus is the unsafe act, and nothing today would stop it.

**Deprecation, not deletion.** A field marked deprecated still loads, still resolves, and reports its
population count. A field with 0 populated cells across nine repositories is removable; a field with 1658 is
not, and the count is the gate rather than a judgement.

## The interchange-format question, decided

**No schema language.** The registry schema is 15 namespaces of TOML tables read by one tool into Rust structs,
and the entire benefit an external schema language offers is validation the tool performs anyway plus
interchange with consumers that do not exist. JSON Schema would buy a `$ref` mechanism the shared-field library
supplies in twenty lines and impose an open-world type model on a closed 15-namespace vocabulary.

The one part worth taking from that world is `$ref`, and it is a citation-by-name, not a format.

The genuine interchange boundary is elsewhere and the round has not named it: the **snapshot**. Karis's
persisted row store plus CSR index is the artefact that crosses from mockspace's `std` side to the no-alloc
crates and, eventually, to vehje. That is where a format decision has decades of consequence, and its
requirements are mmap-ability, no pointer fix-up and a version field. TOML is the authoring surface and should
stay one; the snapshot is the interchange format and should be specified as one.

## What I could not make work, and the ground I covered

**Freezing a live corpus.** The extraction mechanism requires `frozen = true`, and every repository except
`ikiuni_renderer` has a corpus under active edit. I looked for a shape where the oracle can move under a
partial drain (content-addressed spans, a diff-tracked cursor, citing headings rather than lines as
`mockspace.toml:386-388` recommends for non-frozen roots) and each one weakens the check that makes a citation
honest. Heading citations survive edits and cannot be verified as precisely. This is the single unsolved
prerequisite for extending extraction beyond one repository, and I would run it as the probe rather than
Pesce's step 3.

**I did not price adjudication.** 75.7 words per row is the measured *corpus* ratio for one execution. Nothing
records how long a human took per row, and my extraction cost is therefore a volume, not a schedule.

## Open provocations for the synthesiser

1. Re-run every scale argument in this round against 5761 references rather than 1007. The index sizing, the
   cycle check and the composition resolver all face the data-side population, not the prose-side one.
2. Fix `vocab`'s duplicate `note` declaration (`mockspace.toml:423` and `:447`) and add the lint that would
   have caught it. It is the schema-evolution failure mode, present, shipped, unnoticed for the whole round.
3. Specify the shared-field library before the `crates` retype, not after. The retype touches 14 declarations
   today and 14-per-repository forever otherwise.
4. Decide whether the snapshot is the interchange format and version it. It is the only artefact here that
   crosses a boundary and outlives the tool.
5. Run the freeze-a-live-corpus probe on arvo before any extraction schedule is believed.
