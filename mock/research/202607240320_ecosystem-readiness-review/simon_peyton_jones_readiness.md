# vehje ecosystem readiness through the verification-soundness lens (Simon Peyton Jones)

Reviewed from source (all twelve crates, every `.rs` file, ~4400 lines), the closing design topics
(`202607240100`, `202607240015`), the per-crate DESIGN/DEEPDIVE set, and the sealed `AccessSet` machinery in
`hilavitkutin-api/src/access.rs`. The full test suite was run (15 tests, all green) and the load-bearing
soundness question was settled by a compiled probe rather than by argument.

## Verdict in one line

A Core-only consumer can prototype against this today, and the skeleton is the right skeleton; but the
verification core's central promise, "a consumer cannot skip the checks," is not yet true as landed: the
`Checked` witness is mintable for any target and any program through the public `mint_checked` with degenerate
type arguments, and closing that (a small, unforced gap distinct from the FIXME'd frontier) is the one thing I
would land before calling the core buildable-upon.

## What is sound and established

- **The type-level inclusion machinery is genuinely sealed and genuinely a proof.** `AccessSet`, `Contains`,
  `ContainsAll` are sealed on the cons-list typestate (`hilavitkutin-api/src/access.rs:18,39,55-70,109-113`),
  so no consumer can fabricate a membership impl; the blanket impls are structural induction over the list, and
  `#[diagnostic::on_unimplemented]` (access.rs:35,77,105) backs the "compile error naming the missing family"
  claim. As a set-inclusion witness, this piece is correct by construction.
- **The witness type itself is unforgeable by construction.** `Checked` has private fields and a `pub(crate)`
  constructor (`vehje-typecheck/src/lib.rs:48-64`); the only cross-crate construction path is `mint_checked`.
  The seal on the type is real; the problem (Finding 1) is the door the sanctioned mint leaves open.
- **The grade vocabulary is coherent and matches the design.** `EffectMask` / `ReachMask` / `Knowledge` over
  one `Mask<Bits<64,Hot>>` machinery, join = bitwise union, inclusion = subset test
  (`vehje-ir/src/grade.rs:24-141`); `included_in` at grade.rs:55-57 is the correct subset formulation
  (`self ∪ permitted == permitted`). This is exactly the thermometer encoding the bench topic priced
  (6.14 ns/node inference, 0.39 ns/node inclusion) and the design's "three axes, one bitmask machinery"
  actually holds in the code.
- **The graded fold's conservative direction is right where it is incomplete.** `infer`
  (`vehje-typecheck/src/lib.rs:131-216`) unions children's reach without the binder-rule drop; per the
  reachability deepdive, over-approximating reach is the sound direction (dropping without splicing is the
  unsound one), so the M-level simplification errs safe.
- **The wire boundary is disciplined.** The tier-0 encoder/serializer round-trips (tests pass), the walk is
  format-agnostic (`encode.rs:111-206`), bare widths are confined to the documented `#[repr(C)]` boundary, the
  reserve-then-commit sink and the batched-column entry match the ABI bench decision, and the value-arena's
  children-before-parents invariant is the shape the linear typed decode needs.
- **Integrity checking at the seam it covers.** `infer` bounds-checks every child handle
  (lib.rs:136-138 `DanglingRef`), `resolve` refuses unbound names with spans, and the arena's `push`/
  `alloc_list` are capacity-checked returning `Maybe` (`vehje-ir/src/arena.rs:45-71`).
- **Everything compiles green on the pinned nightly and the 44 FIXMEs are greppable and mostly honest**: each
  deferred mechanism I checked against the consolidated topic's deferral list is named at the spot with its
  unblocker, per the placeholder rule.

## Findings

1. **`mint_checked` never consults the target's declared sets: the witness is forgeable through the sanctioned
   mint.** (Soundness/contract gap, unmarked.) `mint_checked<T, Supports, Permits, Families, Effects>`
   (`vehje-typecheck/src/lib.rs:90-100`) leaves all five parameters free; `T` is "unbound here" by design, but
   `Supports`/`Permits` are also caller-chosen and nothing relates them to `T::Supports`/`T::Permits`. Since
   `Empty: ContainsAll<Empty>` holds by the blanket impl (access.rs:111),
   `mint_checked::<AnyTarget, Empty, Empty, Empty, Empty>(arena, root, PhantomData)` mints a
   `Checked<AnyTarget>` for any program. I compiled a faithful model of the sealed traits plus this signature
   and the forged mint typechecks (probe in the session scratchpad, `mint_probe/`); the bound discharge follows
   directly from the access.rs impls. The FIXME at lib.rs:87-89 acknowledges only half of this (the
   caller-supplied `Families`/`Effects`, the genuine frontier); the untied `Supports`/`Permits` half is not
   frontier, it is a wiring slip, and it silently converts `check_for`'s real bound (codegen lib.rs:67-68 does
   tie them to `T`) into advisory ergonomics, because a consumer can always route around `check_for` via the
   public mint. **Fix, small and landable now:** the dependency direction (typecheck cannot name codegen's
   `Target`) is resolved by a minimal sets-carrier trait in `vehje-ir` (say `trait TargetSets { type Supports:
   AccessSet; type Permits: AccessSet; }`), a blanket impl from `Target` in codegen, and
   `mint_checked<T: TargetSets, Families, Effects>` bounded on `T::Supports: ContainsAll<Families>`. That drops
   two forgeable parameters and preserves witness-with-the-prover. (Where the carrier trait lives is Open
   question 1.)

2. **Nothing ties the witness to the check having run.** (Contract gap; DESIGN-vs-source drift.)
   `check_for` never calls `check`; `mint_checked` takes a bare arena and root; `Checked` records no evidence
   that grading or the integrity walk happened. The shipping contract says otherwise:
   `vehje-typecheck/DESIGN.md.tmpl:21-24` has `check` returning "a `Checked` witness or a `CheckError`", and
   the consolidated topic (202607240100, the `vehje-check` entry) says the witness is minted by the prover.
   Concrete program: build an arena containing a node whose child `NodeRef` is out of range, skip `check`, call
   `check_for::<DebugTarget, Cons<Core,Empty>, Empty>`, call `emit`; `fold_core` (codegen lib.rs:90-130)
   indexes the arena unchecked and panics, inside the very machinery the witness was supposed to guard. Per
   design-is-the-oracle, the code owes the explanation: either mint inside `check`'s success path, or have the
   mint demand a proof-of-check token (e.g. take the `Outcome` of `check`, or a `Graded` receipt borrowing the
   `GradeTable`). Cheap to do now; churny to retrofit after consumers exist.

3. **Effect gating is vacuous end to end at M-level.** (Frontier, FIXME'd, but the compound effect deserves
   stating once.) Type-level: the program's `Effects` set is caller-asserted (lib.rs:87-89 FIXME), and no
   mechanism computes it. Runtime shadow: `infer` never inserts an effect operation anywhere; `Raw` is a no-op
   (lib.rs:192-195 FIXME), Core forms carry none, handled-op subtraction is FIXME'd (lib.rs:201-203), so
   `EffectMask` is identically empty and `Grade.binding`/`assurance` are constant defaults (lib.rs:205-213).
   Consequence a consumer must know: `Permits` currently enforces nothing at all; a target declaring
   `Permits = Empty` accepts a program soaked in family effects. Each piece is individually FIXME'd and
   intended; the readiness point is that the effect half of the identity has no load-bearing path yet, and the
   two-stage runtime-bitmask mint is the piece that makes it real.

4. **The structural hash is not structural over leaf content, and no FIXME marks it.** (Soundness-relevant gap,
   unmarked.) `hash_of` mixes only the discriminant for `Var` (hash.rs:67), only the tag for `Lit`
   (hash.rs:66,126-133: `Lit(Int(1))` and `Lit(Int(2))` hash equal), ignores `Let`/`Lambda`/`Project` names and
   keys (hash.rs:68-72,79), and ignores `Raw`'s payload (hash.rs:96). The module doc claims "Equal subtrees
   hash equal... so a cache hit across artifacts is sound" (hash.rs:6-8); the stated consumers are CSE
   hash-consing (`vehje-lower` `Cse::hash`, lib.rs:86-88 already exposes it as "the identity a hash-cons table
   keys on"), fixpoint dedup keying, and the manifest. With unequal-by-construction programs sharing a hash, a
   hash-keyed CSE rewrite would merge `Var(x)` with `Var(y)` (miscompilation), and `Incremental::warm_load`'s
   "a store hit is a proven-valid artifact reused as-is" (incr.rs:84-88) returns the wrong artifact for a
   colliding key. Nothing is live today because `Cse::apply` is a no-op, but this is a trap armed for exactly
   the first person who lands the FIXME'd hash-cons table. Fix: mix the leaf content now (the design question
   is *which* form: interned `Str` ids are session-local, breaking "stably across arenas"; resolving to bytes
   needs the interner in the signature; see Open question 3), add the FIXME today, and pin
   `hash_of(Var(x)) != hash_of(Var(y))` as a catalogue test.

5. **`let rec` does not resolve.** (Gap, unmarked.) `resolve`'s `Let` arm walks `value` under the *outer* scope
   and discards `rec` (`vehje-resolve/src/lib.rs:99-103`, same in `walk_record` at 220-223), so
   `let rec f = λx. f x in f` is refused `Unresolved`. The IR carries the `rec: Bool` precisely so resolution
   can push the frame around the value; four lines fix it (push `frame_for(name, at, scope)` before walking
   `value` when `rec`), plus the catalogue test. Any consumer language with recursion hits this on day one.

6. **The family extension seam, the thing a DSL consumer plugs into, is schema-only.** (Frontier, FIXME'd,
   named because it heads the minimal landing set.) `Raw`'s payload is a placeholder `NodeList`
   (node.rs:127-130), the `FamilyId`-to-`Family`-type assignment does not exist (family.rs:17-19), and the
   family hooks in resolve (lib.rs:137-139), check (lib.rs:192-195), and the fold (codegen lib.rs:88-89, Raw
   visited as a leaf) are all deferred. A family-extended consumer therefore has no working path: its nodes
   are opaque blobs the passes skip and the emitter cannot descend into. Core-only consumers are unaffected.

7. **`Arena::list` slices the whole backing region, not the filled prefix.** (Minor integrity gap.)
   arena.rs:84-86 indexes `self.pool[start..start+len]` against the full caller slice rather than
   `..pool_len`, so a corrupt `NodeList` pointing past the filled pool but within capacity silently reads
   zero-initialised entries instead of failing; `infer`'s `DanglingRef` check covers child `NodeRef`s but not
   list ranges. One-line fix plus a test. Relatedly `NodeRef::new` is `pub`, so handles forge across arenas;
   acceptable for a compiler-internal IR but worth a doc sentence on `get`'s panic contract.

8. **Unbounded recursion in the compile-side walks.** `resolve`, `infer`, `hash_of`, and `fold_core` all
   recurse on node structure. hash.rs:12-13 waves at "the depth cap", but no depth cap exists anywhere in the
   Rust code. The design's own bench finding made iterative work-stacks "the only route" for the Zig side; the
   Rust side can legitimately choose recursion, but then the cap it cites must exist, or a deep (hostile or
   generated) program takes down the dev-time compiler. Small: add the cap or the work-stack, and a FIXME
   meanwhile.

9. **The `vehje` DESIGN names `compile_language`; the source ships only a FIXME comment.**
   (`vehje/DESIGN.md.tmpl:26` backticked vs `vehje/src/lib.rs:66-73`.) Under the repo's own
   docs-are-the-contract rule, a backticked name in DESIGN must exist in source; this one slipped through
   (presumably the mismatch lint parses types, not functions, which is itself a lint gap worth filing).
   Similarly `vehje/SHAME.md.tmpl` describes a std bin crate with `std::env::args` that does not exist in this
   crate surface; stale text from a prior shape. Also recorded, correctly and openly, in the src CL: the
   `vehje-typecheck` to `vehje-check` rename was attempted, blocked by the doc-freeze gates, reverted, and
   flagged for op (src.lock.md:36-44); docs are authored for the new name while the directory keeps the old,
   which a reader should know before grepping.

10. **All fifteen tests are green and none of the known gaps is catalogued red.** The suite is honest
    smoke-level (each asserts something real; nothing "helps" the code path), but per the workspace's
    catalogue-edge-cases-as-tests rule the cases found above should exist as pinned tests now: the forged mint
    (Finding 1, as a compile-fail case), mint-without-check over a dangling arena (2), `let rec` (5),
    `hash_of` leaf sensitivity (4), the list-range bound (7). Zero `#[ignore]`-marked catalogue entries exist
    across the workspace today; for a project whose stated discipline is red-as-lifeblood, the frontier is
    currently invisible to the suite.

11. **Unconditional-success stubs on the trust path, FIXME'd but security-relevant.** `Reader::validate`
    returns `Ok` for any arena (`vehje-runtime-driver/src/lib.rs:69-72`) though the untrusted path "must run"
    the decode; `dispatch` returns `Ok` (lib.rs:38-40); `Schedule::build` returns `Is` (schedule lib.rs:71-73);
    `DifferentialCheck.verified` is a bare settable `Bool` (runtime-gen lib.rs:92-102). All are marked; none
    should be consumed as if meaningful, and a consumer-facing note to that effect belongs in the driver's
    DESIGN until the decode lands.

## The verification core specifically

**Is `Checked` genuinely unforgeable?** The type, yes: private fields, `pub(crate)` constructor, no `Default`,
no `Clone`-into-existence path; and the sealing of `ContainsAll` upstream is real, so the inclusion *machinery*
cannot be lied to. The *witness*, no: the sanctioned public mint discharges its bounds with caller-chosen sets
on both sides of the inclusion, so the theorem it proves as landed is "there exist sets S, P, F, E of the
caller's choosing with S ⊇ F and P ⊇ E", which is a tautology. Two doors: (a) `Supports`/`Permits` untied to
`T` (Finding 1, unmarked, fixable in a few lines today); (b) `Families`/`Effects` untied to the program
(FIXME'd, the acknowledged frontier whose honest closure is the runtime-bitmask second stage, or type-level
set threading through the builder). Until (a) closes, `check_for`'s correctly-bounded version is bypassable.

**Is the inclusion proof load-bearing?** For a caller who voluntarily goes through `check_for` and states the
program's sets honestly, yes, and the refusal is a compile error naming the missing family, exactly as
designed; the framework's own test exercises this honest path (`vehje/src/lib.rs:171`). As an obligation the
consumer *cannot skip*, not yet, per the above.

**Is the grade spine coherent as landed?** Yes, as a vocabulary and a skeleton: the four axes are the concrete
enums and masks the design scoped them to, the join/inclusion algebra is right, and the one place the M-level
fold simplifies (union without the binder drop) errs in the sound direction. But the spine and the witness do
not touch: `check` writes grades nobody reads, `mint_checked` reads nothing `check` wrote, and no grade
(effect, reach, binding, assurance) participates in any gate. Soundness of the core is therefore currently
*assumed* (asserted by the docs, structured for by the types) rather than *established* (enforced end to end);
the structure is genuinely close to being able to establish it, which is exactly why Findings 1 and 2 are
worth paying now, while the fix is five lines and not a consumer migration.

## Readiness for a DSL consumer

**What plugs in today, demonstrably:** a Core-only consumer. Implement `Grammar` (build Core forms via
`Builder` into a caller arena), implement `Target` with `Supports = Cons<Core, Empty>`, and run
resolve → `check` → `check_for` → `emit` → `serialize` by hand; the framework's own end-to-end test and the
tier-0 wire tests show every link of that chain working. Constraints a prototype must live with: no recursive
`let` (Finding 5), `Match` arms are bodies without patterns (node.rs FIXME), lowering is identity, `run` stops
after resolve so the pipeline is manually sequenced, the mint is trust-based, and `Permits` is decorative.

**What must land before a real language/DSL consumer (minimal set, in dependency order):**

1. Close the mint: tie `Supports`/`Permits` to `T` and require evidence-of-check (Findings 1, 2). Small,
   unblocks trusting everything downstream of `Checked`.
2. The family seam: `Raw` payload encoding, `FamilyId` assignment from `Family`, and the resolve/check/fold
   hooks (Finding 6). This *is* the extension contract a DSL consumer is built against.
3. `let rec` in resolve (Finding 5) and the `Match` pattern representation, both blocking any real surface
   language.
4. An effect derivation path, at minimum the runtime-bitmask stage over family-declared effects from
   `Signature`, so `Permits` gates something (Finding 3).
5. Leaf content in `hash_of` before anything keys on it (Finding 4).
6. `run` completing the pipeline (or a documented manual sequence as the interim contract).

Items 2 through 4 are the acknowledged frontier arriving in its planned order; item 1 is the correction this
review adds; item 5 is a latent trap to disarm before the CSE table lands.

## Open questions

1. **Where does the target-sets bound live?** Options: a `TargetSets` carrier trait in `vehje-ir` (typecheck
   bounds the mint on it; codegen blanket-impls it from `Target`); or move `Checked` and the mint into
   `vehje-codegen` (ties the bound directly to `Target` but abandons witness-with-the-prover and the
   consolidated topic's explicit move of the witness into the check crate); or a third tiny crate both depend
   on. The first preserves the design statement at the cost of one more `vehje-ir` trait; the second is
   simpler but reverses a settled design line; op's call which principle outranks.
2. **Should the mint demand evidence-of-check now or with the fixpoint inference?** Coupling now (mint takes
   `check`'s success, or a receipt borrowing the `GradeTable`) makes the witness honest immediately but will
   churn when the inference moves onto `vehje-fixpoint` and the two-stage runtime path lands; deferring keeps
   the seam quiet but ships a witness that attests nothing about the program for another arc. Cost of churn
   versus duration of the honesty gap.
3. **What is the structural hash's identity over names and literals?** Mixing interned `Str` ids is cheap but
   session-local, contradicting "stably across arenas"; resolving through the interner to bytes restores
   cross-artifact stability but puts the interner in `hash_of`'s signature and cost; splitting the roles
   (session-local structural hash for CSE, byte-image `key_of` for cross-artifact keying, and the manifest
   already hashes byte images) changes the "one identity, three consumers" design statement. Each option is
   coherent; they claim different things.
4. **Catalogue-red policy for the frontier.** Pinning the FIXME'd mechanisms as `#[ignore]`-marked catalogue
   tests (per the workspace rule) makes the frontier visible to the suite but adds an authoring round now;
   leaving the suite green-only keeps momentum but the known gaps live solely in FIXMEs. When should the
   catalogue round run relative to the item-1/item-2 landings above?
5. **The `vehje-typecheck` to `vehje-check` rename** is already flagged for op in the locked src CL; it wants
   a dedicated doc-phase round, and until then every DESIGN reference to `vehje-check` names a directory that
   does not exist.
