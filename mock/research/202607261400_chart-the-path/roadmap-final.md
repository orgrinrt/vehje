# The path: current state to the complete canonical design

Phase 9 of chart-the-path. Supersedes `roadmap.md` and `roadmap-draft.md`, both
kept as trail. Folds in twenty-two op rulings (2026-07-26) and the phase 8
granularity pass (`fabian_giesen_granularity.md`).

Status tags, now three rather than two, per Giesen's finding that one UNPROVEN
conflated two different kinds of owed work:

- **MECHANICAL** — no design risk.
- **SKETCH-OWED** — the design is settled; feasibility needs a committed sketch.
- **DESIGN-OPEN** — a design question is unresolved, so no sketch can be
  specified yet. A TODO-sketch over a DESIGN-OPEN gate invites resolving the
  question by construction, which is this project's documented failure mode.

## What the rulings changed

**Ordering is not op's** (call 11). Track order, PE2 sequencing, and the
Arena-tier re-sequencing are all decided here rather than escalated. The order
below is therefore a claim this roadmap owns.

**Everything that can be data, is data, with no exceptions** (call 12), and the
reason is the load-bearing part: the typestate enforced on the Rust side survives
specialisation *implicitly*, because any shape drifting from the enforced-valid
ones is unrepresentable and simply fails. Enforce once; nothing invalid can be
built. op holds this as conviction, not proof.

**Reclamation is proven statically or the project is moot** (call 18). No runtime
liveness check, no assurance-dial fallback.

Those two together mean the project's whole novelty rests on one assumption, and
that assumption is testable. It becomes S1, the first sketch, ahead of every gate.

**The framework is opinionless about modules** (call 20). It owes abstractions
such that no invalid composition is *representable*; it does not verify
compositions, because verifying requires the instance and the instance is a
program. What a module is belongs to each language.

**Two consumer tiers** (call 15). The language author receives a full runtime,
their own `deno`. Their users receive that runtime plus stdlib, docs, ecosystem.
"Hand-authored" means authored once, generally, never per language.

Also settled: artifact C is `mock/crates/clause-lang`, in-tree, and need not be
Rust (call 14). Comptime cost is not benched (call 16). `vehje-runtime-driver` is
deleted, zero bench and zero crate consumers confirmed (call 17). Resumption never
crosses the C ABI; only the host-lent buffer count is open (call 19). The
emitter-seam mechanism returns to research rather than to op (call 22).

---

## S1. The proof-transfer sketch, before any gate

**The claim under test.** After specialisation, is every shape that violates a
Rust-side typestate invariant *unrepresentable*, such that it cannot be built
rather than merely being rejected when checked?

Phrased that way deliberately. The question is not "can we verify the specialised
output" — that is per-instance verification and needs an instance. It is whether
the construction admits no invalid shape at all.

**Why first.** Calls 12 and 18 both rest on it, and call 18 states the
consequence of its failure plainly: the project is moot and the honest move is to
delete it and use something existing. A conviction carrying that much weight is
worth ten hours before it carries four thousand lines.

**Acceptance.** A Rust-side invariant, carried into a specialised Zig artifact,
where the invalid construction fails to compile rather than failing a check.
Leeway: any one representative invariant suffices; a family is better.

**SKETCH-OWED.** Nothing below is safe to build until this returns.

---

## G0. Delete the wrong-locus Rust, dependency-closed

`vehje::run`, `Grammar`, `vehje-resolve`, `vehje-typecheck`, `vehje-lower`,
`vehje-fixpoint`, `vehje-schedule`, `vehje-codegen::fold_core`, the per-script
`encode` walk in `vehje-runtime-abi`, and `vehje-runtime-driver`. Roughly **4,200
lines** with their doc templates.

`positive:45`, `negative:38`, ruled by op. Closure first: `vehje-codegen`
re-exports `Checked` from a crate in the set, and the inclusion typestate and
`check_for` are definitional and must be re-homed rather than deleted.

Before deletion, `negative:91`'s already-fixed `GradeTable::set` behaviour
(`vehje-typecheck/src/lib.rs:370-372`) gets a catalogued red test, so it is pinned
wherever the pass re-lands.

**MECHANICAL** after closure. Giesen was right that it was not mechanical while
the locus was open; the locus is now settled, so it is.

## G0b. Pin the Zig toolchain
Every Zig number in this round, both experts' included, is reproducible only
against whatever is on the runner's path. **MECHANICAL.** Task #54.

---

## Track A: the engine

### G1. Region-lending descriptor with a count
Replaces ~96 KiB of entry-point stack arrays (`runtime.zig:529-535,542`,
`host.zig:127`) that the module doc at `:15-17` already calls caller-lent arenas.
Blocks everything. **SKETCH-OWED** on the ABI shape.

### G2. The frame stack, the two-level core, and defunctionalised `eval`, as one gate
Merged per Giesen, who found the draft split one rewrite of `runtime.zig:278`
along expert-lens lines: the frame-stack defunctionalisation and the
`eval(comptime lang, ...)` two-level rewrite are the same function. The synthesis
had said so and the roadmap re-split it anyway.

Fixes the reproduced segfault (right-nested `Let`, ok at 3,000, SIGSEGV at 3,500;
ok at 10,000, SIGSEGV at 11,000 ReleaseFast) and satisfies `positive:61`'s
defunctionalisation-only rule. **SKETCH-OWED**, needs S1 and G1.

### G3. Tail calls and environment reclamation, statically proven
`Env.push` bumps with no pop (`:176-181`); a three-binding loop body dies at ~256
iterations. Per call 18 the non-escape obligation is discharged statically or
reported as an existential failure. **DESIGN-OPEN** until S1 returns, then
SKETCH-OWED.

### G4. Per-frame unwind carrier
The carrier is one shared struct (`:132-136`); the `Handle` arm re-reads
`unwind.n` as its bound (`:388-394`) while passing the same pointer into a nested
`eval`. Reproduced silent wrong answer. **SKETCH-OWED**, red test first.

### G5. Diagnostics with a host-lent budget
`positive:39` requires this designed "before the conviction becomes the first
place an allocator is smuggled in". **SKETCH-OWED.**

---

## Track B: the keystone

### G6. Operation semantics as data, constructors included
Call 12 settles the bar: no hand-authored escape, no exceptions. Today the sketch
hand-authors five compound operations (`eval.zig:483` on) with `OPS` carrying five
empty entries (`:34-39`) and a comment naming it (`:480-482`); the census calls it
the single escape gating every compound value (`census:57`).

Giesen's catch: the bar as stated is satisfiable by a Zig function-pointer table,
data in name only. The acceptance is therefore that the semantics are expressed in
a closed vocabulary the specialiser evaluates, not that a table exists.

**SKETCH-OWED** for constructor coverage. Scalar coverage is PROVEN
(`202607260800_four-way-projection/`).

---

## Track C: the seams

### G7. `generate` extracts; `build.zig` makes the package comptime
`generate` binds `_signature` (`vehje-runtime-gen/src/lib.rs:108`); nothing reads
the manifest. **SKETCH-OWED**, needs G6.

### G8. Close the emitter seam
`negative:27` moved the differential gate here and left the mechanism open. Three
candidate mechanisms were put to op and all three read as wrong to op, which
means the option set was the defect. **DESIGN-OPEN**, and the owed work is prior-art
research (CompCert-style translation validation, proof-carrying code, typed
decoders), not another round of options.

### G9. Family operations from data, and the host-effect form
Restored per Giesen: the draft dropped "a form for host-serviced effects", which
is the non-mechanical half. `TAG_RAW` currently conflates family operations with
foreign effects; killing `NoHost` for arithmetic (`:414`) requires splitting them.
**SKETCH-OWED**, not mechanical, needs G6 and G7.

### G10. The load verifier
`negative:14`'s parse-don't-validate typed structural decode; `positive:29` puts
it in the trusted rim. **SKETCH-OWED.**

---

## Track D: the graded spine

### G11. `Knowledge` and `assurance` computed, with the axis-interaction rule
`positive:27` sets the spine's own bar: it "earns the word theorem the day an
axis-interaction rule exists in code with a red test that exercises it". Both
hard-coded axes, and that rule and test. **DESIGN-OPEN** on locus until a second
independent read agrees with the derivation that this is runtime-side.

### G12. Lease machinery relocated
Reachability primary (`positive:53`), Perceus exact-meet (`negative:11`). Same
locus question. **DESIGN-OPEN.**

### G13. Inclusion refusal in the licensed locus
`positive:45`: refuses by inclusion, names the offending construct, never silently
degrades. Core identity. **SKETCH-OWED.**

### G14. Module-composition abstractions
Per call 20: typestate and cons-list abstractions such that no invalid composition
is representable, established with no program in sight. The framework builds these
and stops there. **SKETCH-OWED.**

---

## Track E: the language

### G15. The input side, in the licensed locus
The draft's largest omission. `positive:45` keeps the grammar plug-in while G0
removes the Rust one; lexing, parsing, and lowering must exist somewhere licensed.
**SKETCH-OWED.**

### G16. Match, Iter, Interp
The three Core forms falling to `Unsupported` (`:436`). Task #43 owns patterns.
**SKETCH-OWED.**

### G17. CR1, resumption runtime-internal
Per call 19 the ABI shape is fixed: continuations never cross. The open variable
is the host-lent buffer count, which is the N-buffer bench. The
CPS/defunctionalised candidate is buildable now; the segmented candidate needs G2.
**Sequencing decided here per call 11: G2 first, then both candidates together**,
because benching half a fork against an unbuildable other half measures nothing.

### G18. Loop encoding
Per call 21 this is mechanism, resolved by bench and prior art, not escalated. The
sketch's non-resuming Handle discharge is the incumbent and is proven runnable.
**Agent-owned.**

### G19. Macro expansion and hygiene red tests
Blocked on the graded axis, not a Core form (`census:63`). Needs G11.
**DESIGN-OPEN** by inheritance.

### G20. The grammar bar, then the stdlib in Clause
Standing calls 1 and 3. Module shape is Clause's own call per call 20.
**SKETCH-OWED.**

---

## Track F: the output spectrum

`positive:35` and `negative:53` fix the measured shape. **Sequencing decided here
per call 11:** G21 moves early because the batched-column entry is recorded as the
single biggest lever and is an ABI shape expensive to retrofit once consumers bind
to the current entry. The rest follows the language bar.

**G21.** Batched-column W>=2 C ABI entry, absent from the shipped ABI. Early.
**G22.** Fold, CSE, DCE always-on.
**G23.** Predecoded-register middle tier.
**G24.** CFG terminator transfers. `negative:90` records the residual types defined
and never constructed.
**G25.** Direct isel, the native point.

All **SKETCH-OWED**, and G22 to G25 gate on a corrected findings generator: Giesen
found a null-winner defect that poisons the very lever sizes this track orders by.

---

## Track G: the uncatalogued defects

New, per Giesen; the draft had no home for these.

**G26.** Version and tier refusal. Three version constants carry two different
values and nothing compares them.
**G27.** ABI unification.
**G28.** The `the_session` process global (`runtime.zig:477`), against the
host-lent discipline.
**G29.** The findings-generator null-winner defect. Blocks Track F's ordering.
**G30.** The shipped runtime carries zero `FIXME` markers, against
`mark-placeholders-fixme.md`.

---

## The red-test catalogue

`positive:57` requires every canon-grade claim to carry a catalogued red test or a
bench cell at authoring; `negative:86` makes the fix behaviour-shaped red tests.
Each gate lands its rows at authoring rather than after.

## Still genuinely open

Only three, and none blocks S1 or G0:

1. The per-script proof locus (G11, G12), where one expert's derivation awaits a
   second independent agreement.
2. The emitter-seam mechanism (G8), owed prior-art research before it is an op
   question again.
3. The Arena-tier re-sequencing, now agent-owned per call 11 and folded into the
   G2 merge.
