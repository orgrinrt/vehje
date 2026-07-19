# Panel context: a procedural document language and its shared representation

**Date:** 2026-07-19
**Format:** sequential and cumulative. Each panellist reads the context, the artefacts, and every prior
panellist's file, then adds rather than repeats.

## What is being evaluated

A design for a language whose programs produce documents, and whose shared representation is intended to
serve multiple input grammars and multiple output targets, covering both program logic and document markup.

The immediate consumer is **mockspace**, a design-round and documentation tool used by nine repositories in
this workspace. Its problem today: facts live in TOML registries, and documents restate those facts in prose,
so every change means finding every restatement. The intended fix is that documents query the registries and
stay current by construction.

The longer-range consumer is **vehje**, a language project in the same workspace whose stated architecture is
many input syntaxes lowering to one intermediate representation, and that representation emitting to many
output languages.

## The artefacts, in reading order

All paths relative to `/Users/orgrinrt/Dev/clause-dev/mockspace/mock/research/202607190000_procedural-docs-language/`.

1. `09_shape.md` is **the primary artefact under evaluation.** It carries the proposed architecture, the nine
   core forms, the document algebra, the target model, and a section holding the design against mockspace's
   actual needs.
2. `08_decisions.md` carries four settled calls and the reasoning behind each, including a reversal of an
   earlier recommendation.
3. `07_toolbox.md` carries sixteen mechanisms taken from prior art with their sources, a cost table, and the
   forks those costs price.
4. `06_prior_art.md` carries the survey that produced the above.
5. `prior_art/01` through `prior_art/08` carry the deep mechanical detail per system (LMS, Jsonnet, Dhall,
   Pandoc, Djot, Racket, Scribble, Typst, MLIR, nanopass, Terra, Rust proc macros, Zig comptime, Rust arena
   and interning crates). Consult these for specifics rather than re-researching from scratch.
6. `05_correction_and_aim.md` states what the work is for, after an earlier round reached a wrong conclusion.

## Real data to check the design against

The registries are real and populated. `ikiuni_renderer/mock/registry/` holds fifteen namespaces:
`abstraction`, `bench`, `constant`, `data_shape`, `equation`, `facet`, `field`, `law`, `reference`, `ruling`,
`spike`, `task`, `technique`, `tripwire`, plus a vocabulary file. A `task` row carries `id`, `kind`,
`milestone`, `what`, `blocked_by`, `crates`, `provenance`, `note`, and sometimes `options`. Some field values
contain unresolved references to other namespaces.

`mockspace/src/` holds the current implementation, including `document.rs` (the generation pipeline),
`registry/resolve.rs` (the current reference resolver) and `registry/resolved.rs` (the current typed value and
its per-format renderers).

## Constraints that are settled and not under review

These are the maintainer's calls. Evaluate the design given them; do not relitigate them.

- The workspace forbids heap allocation, `dyn`, and `std` in the extractable language crates. mockspace
  itself is `std` and consumes them across a boundary.
- Bare primitives are replaced by workspace types throughout (`arvo` numerics, `notko` fallibility,
  `hilavitkutin-str` interned strings).
- The four decisions in `08_decisions.md` are settled: target-decided staging, open families with typestate
  carrying the inclusion proof, renderers total over their declared family set, and semantic-hash variation
  seeding.

## The questions

Three, and the third is weighted heavily.

**Where does this design have holes?** Correctness, contracts, mechanisms that do not compose, claims that do
not hold, cases the nine core forms cannot express, places where the stated guarantee is weaker than it
sounds.

**What would a better answer look like?** Novel, clever, or unprecedented approaches, alternatives, and
improvements. The prior-art survey found no system with all the properties this design targets, so
originality is available and is wanted rather than tolerated.

**What breaks at scale, and what here is a bad choice for something that must not stay small and simple?**
This design should be judged as the foundation of something that grows: more input grammars, more output
targets, more families, larger registries, more documents, more authors, and years of accretion. Identify
what is sized for a small tool and would be the wrong foundation for a large one. Identify what looks
economical now and becomes a bottleneck, a rewrite, or a source of accumulated friction later.

## Output

Write your file to the path named in your dispatch. Cite `file:line` where you make a specific claim about
the artefacts or the source. Be concrete. A finding that names the failing case beats a finding that names a
concern.
