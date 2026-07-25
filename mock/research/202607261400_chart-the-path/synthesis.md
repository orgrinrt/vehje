# Synthesis: what the complete thing is, and the one order that builds it

Phase 4 of chart-the-path. Synthesises two independent comprehension reads
(`tiark_rompf_binding_time.md`, `andy_wingo_runtime.md`) with this agent's own
verification. Written after a first attempt at this routine skipped from phase 3
to cutting code.

## Where the muddle was

Three bodies of code existed with no settled account of which was the artifact:
8,323 lines of Rust, 2,155 lines of shipped Zig, and a 5,646-line Zig sketch with
158 passing tests that was the only one that ran a language. The question "where
does X live" had two plausible answers for every X.

It had two plausible answers because the agent's own memory index labelled the
round that *produced* the canon as the canon, and quoted that round's section 11
as the standing agenda. Section 11 is a work plan with no counterpart in either
catalogue, committing to wire binding time into the Rust check pass, give
`fold_core` a node-creating catamorphism, wire the Rust check to the signature,
and lower CFG downstream of `Anf`. Every item builds the Rust per-script pipeline
`positive:45` and `negative:38` forbid. One expert panel cited "canon §11" in
good faith on that basis.

The strategic view resolves it in one sentence: **Rust's only binding time is
LanguageAuthor** (canon `positive:21` as corrected by op, 2026-07-26). Rust knows
no constants, no scripts, and no IR. Everything per-script is the runtime's.

## What the complete thing is

Two artifacts, per `positive:45`, plus the first consumer.

**The dev-time Rust language compiler.** Input: a language definition. Output:
validated data and structural proofs. Never sees a script, never ships. Its job,
per op's standing call 10, is to *extract the specialisations*.

**The shipped Zig runtime.** One hand-authored, language-agnostic engine,
specialised at our build to the Rust-emitted data. It does all per-script work:
parsing, analysis, lowering, execution. It is licensed to be generic and
hand-written (op, live call 2), on the condition that it expects no particular
language. **Tested and confirmed: it currently meets that condition.** Its
vocabulary is the 13 Core tags plus generic value tags, with no consumer-language
surface.

**Clause, the first-party consumer.** A language *definition* in the signature
form, the same artifact shape a Lua or C# definition would take, plus a standard
library written in Clause (standing calls 1 and 3), meeting the full
`CLAUSE_EBNF.md` grammar bar.

The canon's frontier names ten items, A through J, that this must satisfy. The
binding-time lattice (A), the one handler discipline over three discharges (B),
CR1 continuations (C), the graded proof spine (D), certified generation (E), one
signature many projections (F), and IR-to-IR staged expansion (G) are the seven
with build consequences.

## The two chains, and why they interlock

The reads produced two different keystones. They are not rivals; they are two
axes, and each expert saw the one their lens exposes.

**Rompf, the seam axis.** The keystone is the language-definition schema:
specifically what an operation's semantics is written in, at a coverage that
includes introduction forms. Everything from the comptime specialisation to the
runtime checker to the load-time folder to the differential check is a *consumer*
of it, and nothing is a consumer in the other direction.

**Wingo, the engine axis.** The keystone is that there is no control stack. Six
separate canon mandates (the third discharge, CR1's segments, the
defunctionalisation rule, the loop encodings, the diagnosable depth cap, the
propagated unwind) are each a statement about frames the engine does not own.

They interlock at three points, which is what makes one order derivable:

- Rompf's item 3 ("make the generic core two-level ... and the evaluator
  defunctionalised") **is** Wingo's items 1 and 2, named from the other side.
- Rompf's item 2 (connect the seam) **is** Wingo's item 6.
- Wingo's item 5 (family operations executed from data) **needs** Rompf's item 1,
  because the data has to exist before it can be executed from.

## The merged order

**Item zero: the ruled deletion.** Roughly 3,300 to 3,700 lines. `vehje::run`,
`Grammar`, `vehje-resolve`, `vehje-typecheck`, `vehje-lower`, `vehje-fixpoint`,
`vehje-schedule`, `vehje-codegen`'s `fold_core`, and the per-script `encode` walk
in `vehje-runtime-abi`. Ruled by op 2026-07-26 on Bellard's enumeration.

It goes **first**, and Rompf gives the reason neither chain makes obvious on its
own: while it exists, every "where does X live" question has two plausible
answers. The deletion is not cleanup at the end, it is what makes the rest of the
order unambiguous. This settles the ordering fork the first attempt at this
routine got wrong by wanting to delete last.

**Phase A: make the engine sound.** Wingo 1 through 4, and independent of both
the schema and the deletion, so it can run in parallel with item zero.

1. A region-lending descriptor with a count, replacing roughly 96 KiB of stack
   arrays that the module doc already describes as caller-lent arenas. Blocks
   everything, because every later mechanism needs a region and the current
   answer is the C stack.
2. An explicit frame stack, `eval` defunctionalised over it. Fixes the reproduced
   segfault and satisfies the canon's own defunctionalisation-only rule
   (`positive:61`, PE3), which the shipped runtime violates uncatalogued.
3. Proper tail calls and an environment reclamation rule. Without it every loop
   burns slots that never come back; a three-binding body dies at roughly 256
   iterations.
4. A per-frame unwind carrier. Fixes the reproduced silent wrong answer.

**Phase B: the keystone.** The operation-semantics representation, at a coverage
that includes constructors. The scalar-only vocabulary is already known
insufficient: the sketch hand-authors five compound operations in Zig with `OPS`
carrying five empty entries beside them, and its own comment names the failure.

**Phase C: close the seam.** `generate` actually reads the signature; a
`build.zig` step makes the package a comptime constant; family operations execute
from that data. This is where the `NoHost` violation of standing call 2 dies.

**Phase D: measure.** Specialisation explosion, warm crossover against the switch
floor, comptime envelope. Not before C, because there is nothing to time.

**Phase E: the rest.** Thermometer-encoded `Knowledge`, HostLoader load-time
lowering, macro expansion, the grammar bar, the stdlib in Clause.

## Proven, assumed, and reproduced

**Proven, re-run independently by Rompf.** The four-way projection: one operation
definition through all four discharge sites byte-identically, the comptime arm
disassembling to three arithmetic instructions across an `export` boundary
against 244 bytes of walker. The two-level split in one file, 158/158.

**Reproduced by Wingo, mechanism verified independently by this agent.** The
right-nested `Let` segfault (ok at 3,000, SIGSEGV at 3,500 default; ok at 10,000,
SIGSEGV at 11,000 ReleaseFast) at `runtime.zig:330`, which is recursive host
evaluation of a `Let` body with no bound. The silent wrong answer at
`runtime.zig:387-394`, where the loop bound `unwind.n` is re-read each iteration
while the same carrier pointer is passed into a nested `eval`.

**Assumed, with zero evidence.** That specialisation pays: no harness cell
measures it. That it does not explode code size: the size cell measures
identical-dedup families and says so. That the closed vocabulary covers a real
family: the running sketch already contradicts this.

**Not reproducible at all.** Every Zig number in this round, including both
experts', is measured against whatever toolchain is on the runner's path. There
is no Zig pin against a committed `rust-toolchain.toml`.

## Open, and op's alone

1. Where artifact C lives: separate repo or a consumer tree here. Sizes the
   schedule more than anything else in either read.
2. Whether the specialisation mechanism stays `comptime`. Not free wording:
   `positive:29` puts it inside the certification perimeter, so a different
   mechanism needs a different certification story rather than an amendment.
3. The Arena-tier-first re-sequencing, an agent-called change.
4. `vehje-runtime-driver` (917 lines): per-script `dispatch` on its face, with a
   defensible dev-time role as the differential harness `positive:29` licenses.
   Excluded from the ruled cut.
5. The PE2 sequencing. The CR1 bench cannot run as specified: the
   CPS/defunctionalisation candidate is buildable today, segmented
   capture-and-reinstate is not, because there is no segment to capture until
   phase A lands.

## Next

Phase 5, the roadmap draft, then the phase 6 critical mirror against the canon,
then granularity, then the sketches. No source is cut until the roadmap is
proven.
