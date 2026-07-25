# Panel summary: what the two comprehension reads agree on, and what they do not

Two experts, dispatched independently on one neutral brief
(`00_context.md`), both running the canon gate, both grounding in quoted
canon. Full reads at `fabrice_bellard_minimal_complete.md` and
`chris_lattner_artifact_boundaries.md`.

## Where they independently agree

Four items. Each was reached separately by both, each cites canon or a
ratified call, and each is therefore a settled finding under the
two-expert rule.

1. **The specialisation seam connects nothing to nothing.** Both verified
   it the same way and separately: `vehje-runtime-gen::generate` binds its
   signature argument as `_signature` (`vehje-runtime-gen/src/lib.rs:108`),
   so the producer produces nothing; `grep -c comptime` returns 0 in every
   file under `mock/runtime-zig/src/`, and the runtime reads no package and
   no manifest, so the consumer consumes nothing. Both name this the
   defining unbuilt job of the whole design. Lattner: "the seam the whole
   design exists for connects nothing to nothing." Bellard: "zero lines in
   the artifact meant to carry it, while 8,323 lines exist in the artifact
   meant to feed it."

2. **The shared runtime core passes the language-agnosticism condition.**
   Both tested it directly against op's live call. Its whole vocabulary is
   the 13 Core tags plus generic value tags; `runtime.zig:410-413` states
   the runtime does not know what a family means; grep hits for "clause"
   are handler clauses. The hand-written generic core is licensed and is a
   keeper.

3. **Arithmetic is still host-serviced, violating standing call 2.**
   `runtime.zig:410-433` routes every family operation to a host callback,
   and `:414` returns `NoHost` when none is supplied. A program that adds
   two integers cannot run without a host supplying the addition. Separate
   from finding 2 and not excused by it: family-freedom is a property of
   the core's source, not a licence to route computation through the host.

4. **`vehje-schedule` is on disk after a ratified deletion.** 77 lines, a
   workspace member at `mock/Cargo.toml:11`, an unused dependency at
   `vehje-runtime-driver/Cargo.toml:11`. Deletion ratified at
   `op-ratification-answers.md:243`; `negative:88` is now false in tree.

## Where they disagree, and why that is the answer

**The per-script analysis locus.** The two reads diverge, and the
divergence is about the canon rather than about the code.

- Bellard reads it as clear-cut: roughly 3,700 lines of the Rust tree are a
  per-script compile pipeline (`vehje/src/lib.rs:60` documents itself as
  "the per-program compile path"), forbidden by `positive:45` ("never sees
  an end-user script") and `negative:38` ("Rust never runs per-script
  analyses, the runtime does, for all scripts"). Two implementations of
  resolve, check, and lower now exist in two languages on opposite sides of
  a ratified boundary, one of which can never ship.

- Lattner reads the canon as internally in tension: the same ratified canon
  that kills the dual locus at `negative:38` also extends the Rust check
  pass as the graded spine's live implementation (`positive:27`, effect and
  lease "computed by the join fold in the shipped check"; §11 wires
  binding-time into the check pass and check to signature). Three readings
  fit different subsets of the letter, so the call is op's.

Under the two-expert rule a disagreement goes to the lead designer rather
than to a tiebreak. No unwinding proceeds until it is resolved. This is
also the item that decides whether the sketch's `check.zig` is a
consumer-side prototype or a wrong-side artifact.

## The five calls handed back to op

Stated by Lattner, each with its tradeoff and no preference. Reproduced
here as the gate on finalising any roadmap.

1. **The per-script analysis locus.** Reading (a): the Rust `check`/`infer`
   is interim scaffolding, the one analysis implementation lands
   runtime-side driven by the extracted Analyzers slice, Rust invokes it
   over FFI for bundled scripts. Reading (b): analyses are one *definition*
   with generated projections into both artifacts, the shape the canon
   already fixes for the relational engine ("emitting into both loci",
   `202607241545:193`). Reading (c): the Rust pass is the permanent
   Bundler-grade analysis home and the runtime carries only the load
   verifier.

2. **The front-end locus for arriving scripts.** Round `202607260500` holds
   this open explicitly (`:122-124`). The tree demonstrates both shapes.
   Runtime-side parsing serves HostLoader-arriving scripts with one
   implementation but puts a parser under the no-alloc discipline for every
   consumer; dev-time parsing keeps the runtime lean but reintroduces a
   second per-script locus for any script arriving after ship.

3. **Where artifact C lives.** Nothing ratified says separate repo versus a
   consumer tree inside this one. The Clause sketch pair is squatting in
   `mock/research/sketches/` and needs a named home before round
   `202607260500`'s src CL lands Clause semantics anywhere permanent.

4. **The letter of "comptime-specialised" versus the licensed generic
   core.** If the property is mandate and the mechanism is bench-decided
   (as the round recorded at `:101-106`), `positive:45`'s "comptime-
   specialised" wording needs amending by a superseding round, because the
   canon is letter-governed per PE4. Otherwise the next auditor reads
   today's zero-comptime core, or a winning non-comptime mechanism, as
   drift.

5. **The Arena-tier-first sequencing.** An agent-called re-sequencing that
   needs confirming or reversing. One sentence closes it.

## Single-expert findings: defects, not design calls

Verifiable directly and not subject to the two-expert rule, since they are
claims about code rather than about what the canon permits.

- The C ABI entry provisions its own fixed arenas instead of taking
  host-lent ones (`runtime.zig:529-542`), against the host-lends-allocations
  call.
- The Zig evaluator recurses on IR depth, reintroducing the defect class
  PE3 closed on the Rust side.
- `dispatch` returns unconditional `Ok` at a refusal boundary.
- `Signature::projection` returns its argument
  (`vehje-signature/src/lib.rs:126-128`), carrying the name of Frontier F
  and the behaviour of nothing.
- Two op-ordered benches do not exist: the CR1 continuation-representation
  fork and the N-buffer arena fork.
- The measured batched-column W entry is absent from the shipped ABI.

## Two canon rows now stale in the tree's favour

Recorded so a later auditor does not re-flag them. The `GradeTable::set`
silent-truncation bug is fixed (`vehje-typecheck/src/lib.rs:369-372`), and
`Anf` is a real catamorphism (`vehje-lower/src/anf.rs`). The
recursion-on-IR-depth bug is not fixed, and now exists on the Zig side too.

## Sizing

Bellard ran `zig test eval.zig`: 158 tests, 0.27 s, all pass. Sized
against QuickJS (about 57,000 lines), Lua 5.4 (about 30,000), TCC (about
30,000), MinCaml (about 2,000), and the sketch's own 4,500 implementation
lines at roughly 55 to 65 percent grammar coverage. Verdict: complete is
about 17,000 lines across three artifacts, less than the 16,124 already on
disk. The tree is not too big; it is about 3,700 lines too big in Rust and
about 12,000 lines too small in Zig.

That sizing is contingent on call 1. Under reading (c) the Rust pipeline is
not surplus at all.
