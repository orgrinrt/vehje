# Certified generation: does it hold? (audit 1, Carmack)

**Date:** 2026-07-20
**Subject:** `202607201627_topic.compiling-the-proof-into-generated-code.md`, read after the four prior topics
of the round in chronological order, with the primary topic authoritative where it speaks.
**Method:** read the five synthesis docs and the five topics in order; re-read my parent audit
(`202607201515_metacompile-paradigm-validation/carmack_audit.md`); verified the Zig-semantics claims the
proposal leans on against current compiler behaviour (lazy analysis, exhaustive switch, `@Type` union
reification, safety-check modes); checked the shipped source state to keep the what-exists axis honest (the
Zig side today is a 46-line extern-stub skeleton at `mock/runtime-zig/src/runtime.zig`; the Rust side is
roughly 2000 lines; everything audited here is design, not code).

## Verdict in one paragraph

The resolution holds, and it holds for the most boring possible reason: "compile the proof away into the
structure of the output, certified when the producer was checked" is not a novel trick, it is what compilation
is. The topic earns real credit for refusing both dead ends cleanly, and the comptime-specialiser reframe
(Rust emits proven data, a Zig comptime metaprogram specialises the general engine to it) is the correct
architecture; it is genuinely better than the Rust-emits-Zig-source path it replaces, because it avoids the
typed-quasiquote machinery the prior art says Rust does badly, and it converges with the all-data direction the
metacompile-form sketch already pointed at. But the reframe quietly changes what the two certifications
certify, and the topic's own certification language has not caught up: under the reframe rustc no longer types
a generator, it types data, and the "well-typed generator run can only produce correct enforcement" claim
attaches to the path the topic rejected. Three of the eleven proposed mechanisms are overstated at the machine
level (allocator-free signatures are convention, not enforcement; the Zig compiler's second certification is
gated on Zig's lazy analysis and covers only what the build forces; built-in safety checks are the wrong
failure mode entirely for an embeddable library, because a Zig safety panic aborts the host process). The
lease axis, exactly as the topic fears, sits mostly outside the certified-generation umbrella: the
distinct-ref-types trick carries the spatial half only, and the temporal half for runtime-arriving scripts is
a hand-authored inference algorithm, which is dead end 1's shape unless the schema is fixed and the algorithm
is proven once. None of this breaks the resolution. All of it needs to be in the design before the one-family
experiment is built, because the experiment as currently scoped would pass without testing the parts I have
just named.

## Where it holds (name the mechanism, cite the topic/source)

**The core pattern is compilation itself, and the lineage is real.** rustc erases types and ships correct
structure; the topic lifts that one level. The dev-time-only specialisation with only the artifact shipping is
the Terra `saveobj` shape documented in the staged-metaprogramming synthesis ("the stager is not present in
the generated artifact... the generator is dev-time only; only the generated artifact ships"), and it is the
first Futamura projection applied at dev time, which my parent audit already endorsed. The synthesis's own
cross-source rule ("a generator that emits code should make the unrepresentable-value failure a
generation-time error... and the generated artifact should carry no dependency on the generator") is exactly
what the topic proposes. This part is settled ground.

**The comptime-generated tag union holding exactly the language's families is feasible and has production
prior art.** Zig reifies unions at comptime via `@typeInfo` plus `@Type`; Mitchell Hashimoto's tagged-union
subset technique (used in ghostty) is this precise mechanism: filter a union's fields at comptime, pass the
filtered field set to `@Type`, get a nominal type in which the excluded variants are unrepresentable, and
comptime type-returning functions are memoised so the same data yields the same type. The topic's
"illegal family is not a representable value" claim is machine-checkable fact, not aspiration.

**Exhaustive switch is compiler-enforced totality.** A Zig `switch` without an `else` prong must cover every
variant of an enum or tagged union; a missing case is a compile error ("enumeration value not handled in
switch"). The renderer-total-over-declared-families obligation does land as a compile-time property, provided
the generated dispatch never uses `else` and never uses a non-exhaustive enum (`enum(u8) { ..., _ }`). Since
the dispatch is generated, banning those two constructions in the emitted shape is trivially enforceable.

**The comptime-folded inclusion check is textbook Zig, and it is the strongest single idea in the list.** One
expression, `script_mask & ~target_mask == 0`; with both masks comptime-known it folds to a `@compileError` or
to nothing; with only `target_mask` comptime it residualises to one AND-plus-compare. This is the "one
evaluator, two times" property the staged-metaprogramming synthesis records for comptime ("nothing crosses"),
and it kills the two-code-paths-to-keep-consistent failure mode structurally. One honest scope note: at the
consumer, every script arrives at run time, so the `@compileError` half fires only at our build, for internal
consistency of the emitted data and for any first-party content bundled into the binary. The consumer-visible
path is always the single runtime bitwise op. That is fine, and cheap, but the topic should say it so nobody
reads "full static guarantee at zero runtime cost" as describing the consumer's scripts.

**Optionals and error unions carry non-null and rejection-path totality.** `?T` forces the null case to be
handled; an error union with an explicit (not inferred, not `anyerror`) error set can be exhaustively switched
in `catch`, so a forgotten rejection branch is a compile error. Both hold as stated, with the same
generated-code-discipline caveat as the switch: explicit error sets only.

**`extern struct` for the wire records is the right tool.** Guaranteed C-ABI layout, compiler-checked, exactly
what the value-arena and residual records need at the C ABI seam. `packed struct` for bit-level fields is
likewise well-defined (integer-backed). No objection.

**Rust emitting data rather than code is the right side of the fork.** The output-spectrum synthesis is blunt
that no real Rust compiler ships tagless-final term representation and that typed generation in Rust fights
missing HKTs; the ir-representation synthesis confirms the all-data direction. Emitting a family table, effect
masks, a lease-rule schema, and a wire layout as comptime consts is a small, inspectable, diffable surface. It
also gets determinism for free on the Zig half: comptime evaluation cannot perform I/O or read a clock, so the
specialisation is reproducible by construction, which answers the parent audit's determinism note for this
stage.

**Co-emitted `test` blocks are correct and, as it turns out, load-bearing** for a reason the topic does not
state; see the lazy-analysis finding below.

## Where it fails or falls short (the honest failure analysis)

**1. The certification story describes the rejected path, not the adopted one.** The topic's general section
says "the generator is typed by rustc... the typestate constrains what it can emit; a well-typed generator run
can only produce correct enforcement," citing LMS, typed Template Haskell, and Terra. Under the comptime
reframe there is no Zig-source-emitting generator for rustc to type. rustc types the *data structures* (the
family sets, the masks, the schema) and the typestate proves their consistency; the thing that generates the
IR type and dispatch is the comptime metaprogram, which is hand-authored Zig that rustc never sees. The
typed-staged-metaprogramming citations attach to the path the topic explicitly sidesteps. What actually
certifies what, under the reframe: rustc certifies the data is consistent; the Zig compiler certifies the
specialisation of the engine to that data (with the lazy-analysis caveat below); and the bridge between them,
the code that serialises Rust-side sets into Zig const syntax, is an unverified printer. That printer is the
same trusted-computing-base gap CompCert has always acknowledged for its pretty-printer: the proof covers
everything except the last unverified translation step. The gap is small, but it is exactly where a silent
divergence between "what rustc proved" and "what Zig received" would live. The topic's own
`comptime { if (!ok) @compileError(...) }` item is the right mitigation and should be promoted from "one item
in a list" to a systematic rule: for every Rust-side proof, emit a corresponding comptime re-check over the
received data (masks internally consistent, dispatch covers the family set, arena layout matches the wire
schema). That is translation validation, and with it the data bridge is validated on every build instead of
trusted. Without it, the "certified twice" framing overstates.

**2. The second certification is gated on Zig's lazy analysis, and the topic does not know it.** Zig only
semantically analyses code that is referenced; generic and comptime-parametrised functions are type-checked at
instantiation, not declaration, and a comptime-pruned branch (`if (comptime cond)`) is discarded without being
checked. So "the Zig compiler, compiling the specialised engine at our build time, certifies it" is true
precisely of the instantiations and branches the build actually forces, and silent about everything else. If
the engine has a code path only reached for some family shape no current language uses, that path has never
been type-checked. The consequence is not fatal but it is a hard build-gate requirement: the build must force
full instantiation coverage (a `refAllDecls`-style forcing root, plus the co-emitted tests, plus compiling
every (language, target) pair that ships), and that forcing is part of the certification, not an optional
nicety. This is also the real reason the co-emitted test blocks are load-bearing: they are not just tests of
authored semantics, they are the instrument that makes the second certification reach the code.

**3. "A function taking no allocator cannot heap-allocate" is false as a compiler guarantee.** It is a strong
Zig convention, and nothing more: any function can reach `std.heap.page_allocator` or a global without taking
a parameter. The actual enforcement available is (a) target choice, since on a freestanding target the OS-backed
allocators fail to compile when reached (lazy analysis cutting the right way for once), (b) the fact that the
engine and the generated code are ours, so a grep-gate over the Zig source for allocator globals is trivial and
should exist, and (c) fixed-buffer arenas passed explicitly. The design intent is fine; the mechanism as stated
("visible and checked in the signatures") claims a type-system property Zig does not have. Restate it as
target-plus-gate enforcement.

**4. Built-in safety checks are the wrong failure mode for the untrusted path, categorically.** Two machine
facts compound here. First, safety checks exist only in Debug and ReleaseSafe; ReleaseFast strips them, so the
guarantee depends on pinning the shipped binary's mode (or `@setRuntimeSafety` per scope). Second, and worse: a
failed safety check panics, and a Zig panic in library code does not unwind across the C ABI, it aborts the
process. For an embeddable runtime, that means a malicious or corrupt script aborts the *host*. That is not a
rejection, it is a denial of service delivered by our own guard rail. The untrusted, runtime-arriving path must
be panic-free by construction: explicit error unions on every bounds and depth and tag check, checked
arithmetic returning errors, no reliance on implicit safety checks at all, and ideally a root panic handler
that exists only to make any residual panic loud in testing. The topic already names "explicit traversal and
depth limits"; it needs to go further and demote the built-in safety checks to a test-build backstop, never a
shipped-path mechanism. Safety checks also do nothing about in-bounds-but-wrong data (a child index landing on
a node of the wrong kind, a cycle inducing non-termination); only the explicit validator catches those, which
is exactly why Cap'n Proto's traversal limits and rkyv's `bytecheck` exist in the value-transfer synthesis.

**5. The lease axis: the distinct-ref-types trick carries the spatial half only, and the temporal half for
runtime scripts is outside the certified-generation umbrella.** What comptime-distinct ref types per region
give is nominal separation: an index minted for region A cannot be passed where region B's index is expected.
That is real and worth having; it kills a whole class of cross-region confusion bugs in the engine itself. What
it cannot give is temporal validity: nothing in Zig's type system prevents holding a region-A ref across region
A's reset and using it after. Zig has no lifetime types; the topic says so. Now split the temporal half by when
the program is known. For dev-time-known content, the region open/close nesting is computed under the Rust-side
proof and baked into emitted structure; the runtime just obeys markers. That genuinely is certified generation,
and it holds. For runtime-arriving scripts, the per-script region inference runs as engine code over data that
exists only at run time; comptime cannot touch it, the generated types cannot constrain it, and its correctness
is "we wrote the inference and believe it is right," which is precisely the shape dead end 1 rejects. The only
way out that does not smuggle dead end 1 back in is the one debt 3 already named: fix the lease-rule schema,
prove the inference algorithm sound once as a metatheorem over that schema (the Tofte-Talpin/MLKit shape:
proved once on paper, implemented once, tested hard), and state the expressiveness bound the schema imposes.
That is a respectable architecture. It is not certified generation, and the design should stop implying the
Zig-end mechanisms extend to it. My parent audit's debt 2 (MLKit leaked and grew a collector; Cyclone shipped
a hybrid; Lua's mutable aliased tables break the immutability narrowing) is untouched by this topic and
remains open in full.

**6. The by-construction rejection at load is still a dynamic check, and precision here matters.** At run
time, mapping parsed input to the generated tagged union requires a switch from input family-id to union tag,
and the unmapped ids fall to an error branch. The check did not vanish; it became a derived, total,
type-enforced decode. That is the strongest available position, and it has a name in the literature: parse,
don't validate (Alexis King). The difference from dead end 1 is provenance and totality (one source of truth,
compiler-checked exhaustiveness, no hand-maintained second copy), not the absence of a runtime test. The
topic's "it is not a hand-written dynamic inclusion test" is true; a reader who takes it as "there is no
runtime branch" will mis-derive the performance and failure-surface story. Say "the runtime check is derived
and total, not absent."

## What is missing or unexamined

- **Zig toolchain pinning as part of the certification.** The workspace pins an exact nightly rustc as policy;
  the second certifier needs the identical discipline. Comptime semantics, `@Type` capabilities, and
  packed-struct layout have all shifted across Zig releases. An exact pinned Zig version, bumped deliberately,
  belongs in the same workspace rule that pins rustc.
- **The per-script macro expander is untouched.** The parent audit's hardest no-alloc instance (macro expansion
  producing new IR from old at run time with no heap) is not addressed by comptime specialisation, which acts
  only on build-time-known data. The debt stands exactly where it stood.
- **`@setEvalBranchQuota`.** Comptime-specialising an engine over a real family table will blow the default
  1000-branch quota immediately. Boundable, trivial, but the build needs a stated quota policy rather than a
  scattering of ad-hoc raises (the synthesis notes the quota must be set from the top of the governing comptime
  stack).
- **The `else`-prong and `anyerror` ban needs a gate.** The totality mechanisms hold only if the generated and
  engine code never reach for `else` on family dispatch or inferred/any error sets on rejection paths. A
  comptime reflection check or a source gate over the Zig tree enforces it; discipline alone will drift.
- **The negative test is the real experiment.** The one-family instance as scoped ("confirm the two
  certifications actually hold") passes trivially on correct data. The experiment that tests the certification
  is the broken build: emit deliberately inconsistent data (a mask missing a family the dispatch covers, a
  wire-layout mismatch) and confirm the comptime validation refuses to compile. A certification that has never
  rejected anything is untested.
- **Panic policy across the C ABI.** Beyond the untrusted path: no Zig panic may be reachable on any path a
  host can drive, because panics abort the embedding process. This is an embeddability correctness requirement
  on par with the GPU-teardown rules elsewhere in the workspace, and it deserves a stated invariant, not an
  assumption.
- **Cross-compilation is an unclaimed win.** Zig's first-class cross-compilation directly serves "one static
  binary per target" and materially strengthens the distribution story; the design may as well claim it.
- **The consumer-side comptime option.** A consumer willing to take the Zig compiler (self-contained, tens of
  MB, no rustc, no separate LLVM install) could bake their scripts in and get the folded `@compileError` path
  for their own content. That is a legitimate opt-in tier that the identity ("pre-compiled binary, nothing
  else") should either explicitly offer or explicitly decline; right now it is silently unaddressed.

## Does it pay the debts from the parent audit

**Debt 1 (the relocation reconciliation): substantially paid.** The debt-reckoning topic's "Rust proves the
runtime, not the script" framing plus this topic's certified-generation answer resolve the fidelity and
redundancy halves credibly. The four recorded consequences (two build environments, the effect-marker split,
the crate taxonomy of a runtime that contains a compile stage, the split-now-internal derivation) are honestly
recorded as obligations and remain undone; that is acceptable bookkeeping, not evasion.

**Debt 2 (the lease lineage as cautionary tale, the Lua mutability hole): not paid.** The reckoning topic holds
a literal placeholder for it, and this topic's lease content is one partial mechanism plus an honest caveat.
MLKit's leak-then-add-a-collector history, Cyclone's hybrid, and the mutable-consumer scope question are all
exactly where I left them.

**Debt 3 (leases are a different kind of proof): half-engaged, implicitly.** Emitting a "lease-rule schema" as
data quietly picks the defensible branch I named (a fixed, pre-proven schema rather than per-language soundness
checking), which is the right branch. But the choice is not stated as a choice, the metatheorem obligation
(prove the inference once over the schema) is not named, and the expressiveness bound is still undescribed.

**The run-the-experiment meta-debt: not yet paid, but the topic finally scopes the right experiment.** Ensure
it includes the negative test and the forced-instantiation gate, per above, or it will pass without testing
the load-bearing parts.

**New debts introduced:** the certification-accounting correction (finding 1), the lazy-analysis forcing
requirement (finding 2), and the panic-policy invariant. All three are cheap to pay now and expensive to
discover later.

## Open questions for op and the next experts

1. **Accept the corrected certification accounting?** Under the reframe: rustc certifies the data, Zig
   certifies the forced instantiations, and systematic comptime translation-validation checks certify the
   bridge. If yes, the topic's "typed generator" paragraphs should be rewritten to match, and the emitted-data
   printer acknowledged as the small trusted step it is.
2. **The lease fork, finally.** Fixed pre-proven schema with a once-proven inference algorithm and a named
   expressiveness bound, or a scoped-down axis (immutable-value consumers only, with the mutable-consumer story
   deferred and named as deferred). The next experts should probe whether the schema can be made small enough
   that the inference is table-driven, because that determines whether the metatheorem is tractable to state.
3. **Untrusted-path policy as invariant:** panic-free by construction, explicit error unions everywhere a host
   can drive, safety checks demoted to test builds, ReleaseSafe-versus-`@setRuntimeSafety` decided and pinned.
4. **Build gates:** forced instantiation coverage, co-emitted tests as mandatory, the `else`/`anyerror` ban,
   the comptime consistency checks as a required emission for every proof, exact Zig version pinned.
5. **For the next experts specifically:** how far can the distinct-ref-types trick be pushed before it fights
   the engine (does threading a comptime region parameter through the evaluator's types stay tractable, or does
   it monomorphise the engine combinatorially); and is there any Zig-expressible discipline for the temporal
   half short of the schema metatheorem (e.g. generation-counted region handles as a checked-at-runtime debug
   layer) worth the cost.

## Bottom line

Adopt the resolution and the reframe; both are right, and the reframe is the better half. The pattern is
compilation, the prior art is real (Terra's dev-time-only stager, comptime union reification in production,
parse-don't-validate at the load boundary), and the framework identity survives intact: no rustc, no LLVM, no
typestate at the consumer, one pre-compiled binary. Then fix the accounting before building anything: the
"certified twice" story must name what each certifier actually certifies under the reframe, the second
certification must be forced through Zig's lazy analysis rather than assumed, the untrusted path must be
panic-free rather than safety-checked, and the no-alloc claim must name its real enforcement. Scope the lease
axis honestly: spatial separation is certified generation, dev-time temporal discipline is certified
generation, and per-script temporal inference is a once-proven fixed algorithm or it is dead end 1 wearing
generated clothing. Run the one-family experiment with the broken-data negative case included. The distance
between this topic and a sound design document is small, and every step of it is named above; none of it
requires new invention, only precision about which guarantee comes from which machine.
