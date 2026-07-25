# Roadmap draft: current state to the complete canonical design

Phase 5 of chart-the-path. **Draft**, pending the phase 6 canonical mirror and
the phase 8 granularity pass. No sketches written yet; the TODO-sketch column
records what each unproven gate would need.

Ordering derives from `synthesis.md`. Each gate names the canon it serves, what
it blocks, and whether it is already confirmed.

Proof status is one of: **PROVEN** (a committed sketch or bench establishes it,
cited), **UNPROVEN** (needs a TODO-sketch), **MECHANICAL** (no design risk; a
deletion or a transcription).

---

## G0. Delete the wrong-locus Rust

**What.** `vehje::run` and `Grammar` (`vehje/src/lib.rs:48,72`), `vehje-resolve`,
`vehje-typecheck`, `vehje-lower`, `vehje-fixpoint`, `vehje-schedule`,
`vehje-codegen::fold_core`, and the per-script `encode` walk in
`vehje-runtime-abi`. Roughly 3,300 to 3,700 lines plus their doc templates.

**Canon.** `positive:45` (Rust never sees an end-user script, never ships),
`negative:38` (Rust never runs per-script analyses). Ruled by op 2026-07-26.

**Blocks.** Every "where does X live" question, which currently has two plausible
answers everywhere.

**Keep, explicitly.** `vehje-ir`, `vehje-signature`, `vehje-runtime-gen`, and the
`vehje-runtime-abi` wire layout, value image, tags and entry contract.

**Status.** MECHANICAL. Excluded and needing its own call: `vehje-runtime-driver`.

---

## Phase A: make the engine sound

Independent of G0 and of the schema, so it can run in parallel with G0.

### G1. A region-lending descriptor with a count

**What.** Replace roughly 96 KiB of entry-point stack arrays (`runtime.zig:529-535,542`,
`host.zig:127`) with a host-provided descriptor.

**Canon.** `positive:45` (no heap on either side, host-owned memory via
caller-lent arenas and host-lent budgets; PE3 makes it an axiom). The module doc
at `runtime.zig:15-17` already claims caller-lent arenas; the code contradicts it,
and `host.zig:160-162` asserts two compile-time constants are "lent bounds like
every other here", which is false of both.

**Blocks.** Everything. Every later mechanism needs a region and today's answer is
the C stack. On a 4 to 16 KiB thread stack this overflows at the entry point
before depth is a question.

**Status.** MECHANICAL in shape, UNPROVEN in ABI. TODO-sketch: the descriptor
shape across the C ABI, showing a host lending N regions of declared size and the
runtime refusing rather than faulting when one is undersized.

### G2. An explicit frame stack, `eval` defunctionalised over it

**What.** `eval` (`runtime.zig:278`) currently recurses per Core node and per
argument application (`:351`). Turn frames into data.

**Canon.** `positive:61` promotes defunctionalisation "from the principled route
to the **only** route"; PE3 mandates "defunctionalised non-recursive passes".
`negative:91` catalogues this exact defect for `infer` and `structurally_equal`
on the compile side. It is uncatalogued on the artifact that ships.

**Blocks.** G3, G4, G6, the continuation model, and the completeness bar, since a
stdlib in Clause will be the deepest program in the tree.

**Evidence it is real.** Right-nested `Let`: ok at 3,000, SIGSEGV at 3,500
default; ok at 10,000, SIGSEGV at 11,000 ReleaseFast, faulting in `eval` at
`runtime.zig:330`. Right-nested `Let` is what `Anf` is designed to emit, so the
shape is intended, not adversarial. A bound that moves with the optimisation
level is not a bound.

**Status.** UNPROVEN. TODO-sketch: the defunctionalised evaluator over an explicit
frame stack, proving a right-nested `Let` chain past 100,000 returns a named
refusal rather than faulting, and that the Core forms still evaluate identically.

### G3. Proper tail calls and environment reclamation

**What.** A body in tail position reuses its frame. `Env.push` bumps with no pop
by design (`runtime.zig:176-181`), so a three-binding loop body dies at roughly
256 iterations.

**Canon.** The loop encodings in `positive:23`'s discharge set; standing call 3
(a stdlib in Clause) is unreachable without loops.

**Blocks.** Every loop, therefore the stdlib, therefore call 3. Task #49.

**Status.** UNPROVEN, and dependent on G2. TODO-sketch: a `while` of a
three-binding body running 1,000,000 iterations in constant environment slots.

### G4. A per-frame unwind carrier

**What.** The carrier is one shared mutable struct (`runtime.zig:132-136`) passed
to every `eval`; the `Handle` arm re-reads `unwind.n` as its loop bound across
iterations (`:388-394`) while passing the same pointer into a nested `eval`.

**Canon.** `positive:23` (the one handler discipline); the whole
prove-then-erase identity, which this silently violates.

**Evidence it is real.** A two-operand clause whose body evaluates an inner
handled computation returns a `closure` where the program's value is `20`. Silent
wrong answer, no error.

**Blocks.** Any handler nesting, which is `return` inside a loop, which the
completeness census makes the ordinary case.

**Status.** UNPROVEN. TODO-sketch: the adversarial program as a red test first,
then the per-frame carrier making it green, with nesting depth 3 also covered.

---

## Phase B: the keystone

### G5. The operation-semantics representation, with constructor coverage

**What.** What an operation's semantics is written in, at a coverage that
includes introduction forms.

**Canon.** `positive:31` ("One declarative signature ... is the single source"),
which is false for computation the moment meaning lives anywhere else. Frontier F
(one signature, many projections).

**Blocks.** G6, G7, G8, G9, G10. Every one of them consumes it and none is
consumed by it.

**What is already known.** The four-way projection proved admissibility for a
five-opcode **scalar** vocabulary and says so in its own "Does not establish"
(`202607260800_four-way-projection/findings.md:73-76`). The clause-in-zig sketch
then hit the wall and hand-authored the compound operations directly in Zig
(`eval.zig:483` onward for `OP_MAKE_REC`, `OP_MAKE_SEQ`, `OP_LEN`, `OP_AT`,
`OP_PUSH`), with `OPS` carrying five empty entries beside them (`eval.zig:34-39`)
and a comment naming the failure: "The vocabulary has two shapes in it, and this
is where that shows" (`:480-482`). The census calls this "the single escape
[that] gates every compound value in the language"
(`202607251530_language-completeness-census.md:52`).

**Status.** PARTLY PROVEN for scalars, UNPROVEN for constructors. TODO-sketch: a
vocabulary that expresses variable-arity construction, driven through the same
four discharge sites, with the acceptance bar being that `OPS` carries no empty
entries and no compound operation has a hand-written Zig body. Leeway: any
encoding in the family (a variadic primitive, or constructors as a second table
kind) is acceptable; what is not acceptable is a hand-authored escape surviving.

---

## Phase C: close the seam

### G6. `generate` extracts, `build.zig` makes it comptime, the core is two-level

**What.** `generate` reads the signature and emits the FamilyTable slice with
bodies; a `build.zig` step makes the package a comptime constant;
`eval(comptime lang: LanguageDef, img, ...)` with `applyOp` a comptime-generated
switch rather than a linear compare chain.

**Canon.** `positive:45` (one hand-authored engine comptime-specialised to the
Rust-emitted data), `positive:29` (certified generation), Frontier E. Standing
call 10.

**Evidence of the hole.** `generate` binds its argument as `_signature`
(`vehje-runtime-gen/src/lib.rs:108`) and returns caller slices plus a hash.
`grep -c comptime mock/runtime-zig/src/*.zig` is 0 in every file. The manifest
exists so "the specialised runtime can prove at its own build that it was
specialised from exactly this package" (`lib.rs:73-76`) and nothing reads a
manifest.

**Blocks.** Certification, the differential check, and the whole of call 10.

**Status.** UNPROVEN end to end, though both ends are PROVEN in isolation: the
four-way projection for the comptime arm, the sketch's `OPS` for the two-level
shape. TODO-sketch: Rust emits a package, `build.zig` consumes it, the runtime
specialises against it, and the differential check compares two evaluations of
one definition.

### G7. Family operations execute from data; a form for host-serviced effects

**What.** Kill `NoHost` (`runtime.zig:414`) as the path for arithmetic. The host
callback survives only for genuinely foreign effects.

**Canon.** Standing call 2, which the in-source defence at `runtime.zig:411-413`
still contradicts in the tree.

**Blocks.** Any real program, therefore the stdlib, therefore calls 1 through 3
jointly.

**Status.** Depends on G5 and G6. MECHANICAL once both land.

---

## Phase D: measure

### G8. The harness cells that do not exist

**What.** Specialisation explosion at distinct bodies; warm crossover against the
switch floor; comptime envelope at realistic operation-count times body-size.

**Why here.** Not before G6, because there is nothing to time.

**Status.** UNPROVEN and currently unmeasurable. Note that the design's central
payoff claim has zero harness cells behind it today.

### G9. Pin the Zig toolchain

**What.** A committed Zig pin beside `rust-toolchain.toml`.

**Why.** Every Zig number in this round, both experts' included, is reproducible
only against whatever is on the runner's path. Task #54.

**Status.** MECHANICAL. Should arguably move ahead of G8, possibly to G0.

---

## Phase E: the rest

### G10. Thermometer-encode `Knowledge` and compute it
Unblocks the HostLoader stage. Two of four grade coordinates are still hard-coded
constants.

### G11. HostLoader load-time lowering
`RuleTable` (`vehje-lower/src/lib.rs:95-101`) is two booleans claiming to be a
lowering table. Needs G5, G6, G10. Note this crate is in G0's deletion set, so
the capability relocates rather than survives.

### G12. Macro expansion
Blocked on the graded axis, not on a missing Core form
(`202607251530_language-completeness-census.md:63`). Needs G10. The N-buffer arena
fork is downstream of this.

### G13. Match, Iter, Interp
The three Core forms falling through to `Unsupported` (`runtime.zig:436`). Task
#43 owns the pattern representation.

### G14. The continuation model, CR1
`positive:25` says plainly "The runtime has no continuation model yet". PE2 orders
the representation bench now, but the segmented candidate is not buildable until
G2 gives it a frame to segment.

### G15. The grammar bar, then the stdlib in Clause
Standing calls 1 and 3. The full `CLAUSE_EBNF.md`, then a stdlib in Clause that
loads through a real module system rather than text concatenation.

---

## Open forks this draft does not resolve

Carried from `synthesis.md`, all op's: where artifact C lives; whether the
mechanism stays comptime given `positive:29` puts it in the certification
perimeter; the Arena-tier re-sequencing; `vehje-runtime-driver`'s fate; and the
PE2 sequencing given G2 blocks half the CR1 bench.
