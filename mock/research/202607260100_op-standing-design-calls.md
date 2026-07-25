# op's standing design calls for the vehje arc

**Date:** 2026-07-26
**Status:** standing directives, not a design round artifact
**Scope:** the whole vehje framework arc and the first-party Clause language

These are op's own words and the calls they carry, written down so they survive
compaction and so no later round re-litigates them. They are canon on the
intent axis. Where a call names a specific mechanism, the mechanism is a
proposal and the measurement decides; where it names a property, the property
is a mandate. That split is itself one of the calls (call 4 below).

## 1. The completeness bar is the full pre-redesign grammar

> "Real language and running isn't the bar, but completeness for the 'vehje the
> language' that was specced long time ago, before we did the re-design and
> updating and changing the identity wholesale. The old syntax for the vehje
> authoring lang is still the benchmark and dogfood target, not any arbitrary
> lang. So nothing is done until we can express the full intended lang,
> generics, macros, assoc types and such, as intended."

The normative grammar is `mock/research/original-docs/CLAUSE_EBNF.md`. A
runnable subset is not a milestone worth claiming. Generics, macros, associated
types, and traits are in scope, not deferred surface. The census that walks the
grammar against the Core is `202607251530_language-completeness-census.md`.

## 2. The runtime is computationally self-sufficient

> "Of course the runtime should be standalone sufficient to compute and do all
> of it. Host should only basically configure the runtime and give it the
> allocations, nothing more. It does receive the values, but it should *NOT*
> have to give the simple things like fucking arithmetics..."

The host's job is configuration, lent allocations, and receiving results. That
is the whole of it. The host does not supply arithmetic, comparison, or any
other elementary computation, and a design that requires it to is backwards.

This corrects the shape that had shipped: arithmetic lowered to
`Raw(ARITH, ...)` and crossed the C ABI to a host-provided handler, which meant
the language's elementary operators existed only as a function in a test file.
The correction is not "move the handler into the runtime as a default"; it is
that computation belongs to the runtime and never depended on the host.

Note the boundary this does NOT move: the framework still owns no family. vehje
the framework is generic over families. The first-party Clause runtime is a
different artifact from the framework, and it ships its own computation. The
separation of concerns survives; what changes is which side of it arithmetic
was on.

## 3. Clause ships a standard library, written in Clause

> "We need a stdlib to ship with clause of course, it's not complete until that
> is true. And we have to write it *in* clause, so it dogfoods this thing"

Two requirements, both load-bearing:

- Completeness includes a standard library. Without one the language is not
  done, regardless of what the compiler and runtime can do.
- The stdlib is authored in Clause, not in Rust or Zig. Writing it in Clause is
  how the language proves it can carry real code, and every gap it hits is a
  finding about the language rather than a missing convenience.

The stdlib is therefore both a deliverable and the acceptance test for calls 1
and 2. It cannot be written until the language expresses enough to write it,
and it cannot run until the runtime computes on its own.

## 4. Intent is canon; the named vehicle is bench-decided

> "We don't just want to make a naive runtime. We want to bake in all the
> optimisations we know will be beneficial to vast majority of cases, and make
> it opt-out for those that want to hand-bake them. Host-config is again the
> thing that controls this, but we should give sensible and ergonomic defaults
> either way."

> "if string interning as such would be *less* efficient and optimal than
> something else, then obviously we'd do the better one. When in doubt, reach
> for benches. We have a brilliant harness to design and run benches on!"

The mandate is the property: known wins are on by default, opt-out by host
config, ergonomic either way. The named candidates (automatic string interning,
interning of equality-heavy compounds, active deduping, aliasing and
reclamation as values leave scope) are proposals to measure, not a
specification to implement. Two of the four have already been refused by
measurement; see `202607251610_baked-in-optimisation-mandate.md`.

## 5. Every bench runs in the harness

> "Even for zig-authored benches, you *NEED* to plug them into the harness.
> It's crucial that we get all data as cvs results, analysable"

Applies regardless of which language authored the measured code. A standalone
timing loop is not a bench: it produces numbers nobody can re-run, compare, or
audit. The harness gives per-variant cdylib isolation, a shared realistic
workload, calibrated repetition, and a committed CSV trail.

## 6. Optimal over fast

> "we aren't about 'fastest', we are about 'most ideal and optimal'."

Sequencing follows the ideal shape, not the shortest path to something running.
This is why a stopgap that greens a gate is worse than the red gate it would
have replaced.

## 7. No bylines

No `Co-Authored-By` trailer on any commit, PR body, PR comment, issue body, or
issue comment. op's reasoning: the design is human-authored and human-overseen,
and the agent is executing it mechanically, so the byline does not stand. The
`Generated with` advertising suffix is forbidden everywhere for the same
reason.

## How these interact

Calls 1, 2, and 3 compose into one acceptance condition rather than three
separate ones: **a standard library, written in Clause, exercising the full
grammar, running on a runtime that computes without host help.** Each of the
three is a prerequisite for that sentence being true, and none of them is
individually sufficient.

Calls 4, 5, and 6 govern how the work gets done rather than what gets built.

## Calls 8, 9, 10: the artifact split (2026-07-26)

Three further statements, in op's own words, given while charting the path.

**Call 8. The identity, restated.** "Remember the soul and identity of vehje: N
in, M out, all lower to IR in middle and work between each other if defined in
the rust artifact."

The middle IR is the convergence point. N input languages lower to it; M output
targets come from it. Interoperation *between* the input languages is defined in
the Rust artifact, not negotiated at runtime.

**Call 9. A generic shared runtime core is licensed.** "We've allowed a general
shared runtime core in the designs already, so a very generic, not-specified
shared runtime is fine, as long as it does not expect any kind of language, and
will work with anything."

The core may be hand-written and shipped rather than generated per language. Its
legitimacy condition is language-agnosticism, and that condition is testable: the
core must not expect any particular language and must work with anything.

**Call 10. Rust extracts the specialisations.** "The rust should extract the
specialisations."

This names the Rust artifact's job. Rust does not run per-script passes and does
not emit runtime source. It compiles a language definition into the
specialisations, and the generic core consumes them.

### What calls 9 and 10 resolve

They settle an apparent conflict between call 2 (the runtime is computationally
self-sufficient; the host must not supply arithmetic) and the framework's
family-freedom requirement.

`mock/runtime-zig/src/runtime.zig:410-435` currently dispatches every family
operation to a host callback, and its in-source comment defends this as "what
keeps the framework family-free". Under calls 9 and 10 that defence is wrong.
Family-freedom is a property of the core's *source*; it is not a reason to route
computation through the host at run time. The correct shape is that Rust extracts
the family operations as specialisation data and the core consumes them at
comptime, leaving the host callback for genuinely foreign effects rather than for
`+`.

The current code fails call 2 provably: `runtime.zig:414` returns `NoHost`, so a
program that adds two integers cannot run without a host supplying the addition.

The consumption point does not exist yet. `grep -c comptime
mock/runtime-zig/src/runtime.zig` returns 0, while
`mock/crates/vehje-runtime-gen/src/lib.rs` already describes its output as "the
data the runtime's comptime specialisation reads: data, not runtime source". The
Rust side documents a contract the Zig side has no machinery to receive. That is
task #52.

## See also

`canon/the-soul-of-vehje-positive-catalogue.md` and
`canon/the-inverse-of-vehje-negative-catalogue.md` (the identity oracle),
`202607251530_language-completeness-census.md` (the grammar walk),
`202607251610_baked-in-optimisation-mandate.md` (call 4's measurements),
`202607252100_handle-clause-representation.md` (the effect discharge design),
`original-docs/CLAUSE_EBNF.md` (call 1's normative grammar).

## Calls 11 to 18: the chart-the-path rulings (2026-07-26)

Eight calls settled while charting the path, in op's own words or close
paraphrase where the answer was long.

**Call 11. Ordering is not op's.** "I've never set any sequence... I don't care
*how* or in what order, as long as the design I've drafted gets done. Ordering is
just impl detail, best answered by the people doing the mechanical work."

This retires every sequencing question as an agent decision: track order, PE2
bench sequencing, the Arena-tier re-sequencing. Stop parking them with op.

**Call 12. Everything that can be data, is data. No exceptions.** The reasoning
matters as much as the ruling: the typestate proofs enforced on the Rust side
transfer through comptime specialisation *implicitly*, because any shape that
drifts from the enforced-valid ones becomes unrepresentable and errors. "enforce
once; any invalid shape would be inrepresentable and cause errors." So the proof
survives erasure without being re-stated, and anything unsound fails to run.

op flags this as a held conviction ("my gut says it holds"), not a proven one. It
is the assumption the whole certified-generation story rests on and it owes a
sketch.

**Call 13. The mandate is the proof surviving, not the mechanism.** "Zig comptime
is the only practical answer to how to maintain the same statically proven
soundness and typestate proofs on zig runtime's side. If that is wrong, I'm more
than happy to hear any alternatives and bench them. So I don't care about the
mechanism, be it comptime or something else; as long as the typestate soundness
and proof remains." Comptime is the incumbent on weeks of prior research;
alternatives are welcome only with a bench and only if the proof survives them.

**Call 14. Artifact C lives in-tree, and need not be Rust.** "For now, it should
be a clause-lang crate within `mock/` here... Despite its name being 'crates', the
mockspace can host any arbitrary libs and bins projects, not just rust (already
does this for zig and c++ in our projects)."

**Call 15. Two consumer tiers, and "hand-authored" means authored once.** The
**language author** (who defines a language using vehje) receives **a full
runtime**, equivalent to `dotnet` or `deno`: ready to take scripts or projects and
run, build, and analyse them. The **users of that language** receive that runtime
plus its stdlib, its documentation, and the ecosystem built on it.

op corrects a misreading the canon's wording invites: "The 'hand authored' there
is misleading: that only means the engine is general, not 'hand-authored' per
language or spec. Literally once, as a general engine for all of the infinite
combinations of M-to-N language specs and output and input definitions and all.
Never hand-authored per the language spec/definition. One static general runtime
that gets expanded and specialised by the comptime data generated by the rust dev
compiler, which compiles the literal language and runtime, NOT the scripts, ever,
nor any IR, ever. It compiles and bakes the runtime and the language defs into
it."

**Call 16. Comptime cost is a non-issue.** "Compile time cost is paid only once,
ever." Benching it is optional and unmotivated; if it ever needs new harness
machinery, that machinery lands upstream in mockspace first and is dogfooded
here. Standing call 5 is not breached, because there is no bench owed.

**Call 17. `vehje-runtime-driver` is deleted.** op: "Check the benches. If benches
use it, it may stay, but just relocated to `benches/`. If not, it's dead weight
and fucking gone." Checked: zero references in `mock/benches/`, zero consumers in
`mock/crates/`. It is gone, 917 lines, joining the ruled cut.

**Call 18. Reclamation is proven statically, or the project is moot.** "The whole
idea here is that we can proof it statically on the runtime+lang def time. If we
can't, then that's a bummer, and we'll have to re-assess. A valid answer is of
course to just state that it can't be done, which makes this whole project moot,
and delete it all and use something that exists that can do the same thing we'd
regress to if we dropped our novelty and identity."

No runtime liveness check, no assurance-dial fallback. The static proof is the
identity, and its failure is an existential finding to report rather than a
degradation to engineer around. This is the sharpest acceptance bar in the
project and it belongs to the same assumption as call 12.

## Calls 19 to 22, and a correction to the escalation filter (2026-07-26)

**Call 19. Resumption never crosses the C ABI.** Continuations are captured and
resumed entirely inside the runtime; the host sees completed results. op: "Option
1, as already designed. The thing to bench is whether to do it recursively with a
scratch buffer/allocation given to us by the host, or several. The mechanism
itself is pretty clear: we get allocations and we use them."

So the open question is not *where* continuations live but *how many* host-lent
buffers the capture uses, which is the N-buffer arena bench already owed. The ABI
shape is not a variable.

**Call 20. The framework is opinionless about modules.** op: "Stdlib is the same
as user modules. But what modules are, is entirely dependent on the lang, not our
concern. We are opinionless on that front. The only thing we need is to create the
proper typestate, cons-list abstractions to statically prove they will be sound
and valid, once we write the actual language representation and contract suite in
the rust side. Some language wants to do it on their statical parts (aot or jit)
and some want to do it in flight dynamically; we shouldn't try or attempt to
control the loading, nor the definition of modules or libraries. That's all
language specific."

What the framework owes is the typestate and cons-list abstractions **such that no
invalid module composition is representable**. op corrected an earlier wording of
this line that said the abstractions "prove module composition sound", and the
correction is not stylistic: "It does not prove module composition sound, it
proves that no invalid module composition can ever be represented, by design.
There's a very big difference. The former has to be on the runtime or see the
program. The latter can happen entirely disjoint, without knowing any specific
programs or modules or anything like that."

Proving a composition sound is per-instance verification: it needs the instance,
so it puts the prover on the program side, which is forbidden for Rust. Making an
invalid composition unrepresentable is a property of the abstractions, established
once, with no program in sight. This is call 12's principle applied to modules,
and the distinction is the whole reason Rust can do this work without ever seeing
a script.

What a module *is*, and how it loads, belongs to each language definition. The
stdlib is not a special case; it is a module like any other, and its shape is
Clause's call, made when Clause's definition is written.

**Call 21. Loop encoding is an implementation detail, not an op call.** op: "This
sounds like an impl detail that shouldn't be brought to me. Rather, seen what
canon says for the *intent*, and for the *impl detail* of how to achieve it,
benches and research of prior art should be the answers, as always."

**Call 22. The emitter-seam mechanism is not yet an op question.** Presented with
three candidate mechanisms, op: "This needs more context, I can't make the call
based on this. Intuitively, all three sound wrong, but I might be missing
context." The option set was the defect, not the answer. It returns to research
and prior art before it is put to op again, and it is **not** re-asked with a
softer framing.

### The escalation filter, corrected

Twice in one session the agent escalated something the canon had already settled
at the intent level, and once it escalated a question that belonged to a consumer
language rather than to the framework. The filter that was in use was "am I
uncertain". The filter that applies is:

1. **Does the canon state the intent?** If yes, and what remains is *how* to
   achieve it, that is a mechanism question. Mechanism questions are answered by
   benches and prior-art research, by the agent. Ordering, encoding choices, and
   representation are all mechanism.
2. **Is this the framework's question at all?** Some questions belong to a
   consumer language and are answered when that language's definition is written.
   Asking op to settle them at framework level imports a consumer's concern into
   the framework, which is the thing the extension pattern exists to prevent.
3. **Only then**, if the canon is genuinely silent on the intent, or the call is
   irreversible and outward-facing, does it go to op, with an option set the agent
   has enough context to have built honestly.

Being uncertain is not the trigger. Uncertainty is usually the signal to go read
the canon, run a bench, or survey prior art.
