# Clause Language Design Review
## Type-System Soundness and Language Design Analysis

**Reviewer role:** Language designer with experience in TypeScript, Elm, PureScript, Rust, Haxe.  
**Review target:** `docs/megapatch/design/2026-04-16-mod-language-design-draft.md` (31 sections, ~2028 lines)  
**Reference docs:** `startpoint.md`, `rust-traits-research.md`, `typed-dsl-prior-art.md`  
**Date:** 2026-04-16

---

## 1. Executive Summary

**Overall verdict: YELLOW — solid foundations, but four issues need resolution before M5 (type checker) implementation begins.**

The design makes the right macro-level decisions: nominal scopes, orphan-rule coherence, no lifetimes, no `dyn Trait`, `extern` declarations over `any`. The Elm error-philosophy commitment is correct for the audience. The architecture of `extern → #[patch] → coherence` is exactly the right shape for preventing the silent LIOS conflicts that motivate the whole project.

### Top 3 concerns

1. **The `#[patch]` bypass creates an undetected multi-crate collision class.** Two crates both marking `#[patch(target = "same::Thing")]` without knowing about each other produces a soundness hole: the compiler errors only if they're in the same compilation unit. In separate build contexts (user builds Crate A, then Crate B, then both), the conflict may not surface until Stellaris load order. This undermines the core coherence guarantee (§23.3 says "compile error forcing consolidation" but this is only true within a single compiler invocation).

2. **Kind inference (§9.4) has silent-wrong-output cases.** Specifically: methods that call other Clause methods whose kinds were inferred, and the callee's inference changes in a later edit. The compiler may re-infer the callee as a different kind and silently change the emitted construct at every call site. This is a "spooky action at a distance" soundness issue — correct-looking code produces wrong Clausewitz without any error.

3. **`Option<T>` / `Result<T, E>` codegen strategy (§12.1) is chosen silently per type, but the choice leaks across crate boundaries.** If Crate A returns `Option<PopId>` using sentinel `-1` and Crate B also reads that value expecting a flag+value layout (because it sees the type differently), both will compile cleanly but produce silently wrong runtime behavior.

### Top 3 things that work well

1. **The orphan rule as patch conflict prevention** is the best possible mechanical enforcement of the "consolidate, don't choose" principle. Using coherence law to force explicit merging is elegant and sound within a single compilation.

2. **Tenet 11 (Hide the engine, surface the intent)** is well-executed throughout. The `StringStorage`, `Registry<Self>`, `Cached<T>` abstractions are correctly described as implementation details the author never sees. This is the right model.

3. **The `extern` → `stellaris-vanilla-spec` → curated-overlay pipeline** is a mature interop story. Multi-source ingest + cross-validation + hand-curated overlays mirrors the best practices from TypeScript's DefinitelyTyped evolution. The decision to parse `trigger_docs` directly (not rely solely on cwtools-config) is correct.

---

## 2. Critical Concerns
*(would-block-implementation-level — must be resolved before M5 starts)*

### C1 — Patch collision in distributed compilation contexts (§23.3)

**The claim:** "Two patches of the same target → compile error forcing consolidation" (§23.3).

**The gap:** This only holds if both patches are visible in the same compiler invocation. In a Clausewitz workspace with Crate A and Crate B both independently declaring `#[patch(target = "vanilla::Country::produce_energy")]`, the compiler catches the collision only during a full workspace build. If Crate A is built alone (e.g., Heritage crate built for local testing), then Crate B is built alone (e.g., megapatch crate for a different author), each build succeeds. When both cratesʼ outputs land in the modlist together, Stellaris silently picks one via LIOS — which is exactly the failure mode Clause was designed to prevent.

**Concrete counterexample:**
- `crates/heritage/src/compat.cw` contains `#[patch(target = "vanilla::Country::produce_energy", reason = "heritage correction")]`
- `crates/nsc3-compat/src/lib.cw` contains `#[patch(target = "vanilla::Country::produce_energy", reason = "nsc3 correction")]`
- Heritage author builds Heritage crate: succeeds.
- NSC3-compat author builds their crate: succeeds.
- Both crates are in the workspace: compile error only appears now. If the two crates are authored by different people and distributed separately, the compile error never fires.

**Why this matters for Stellaris specifically:** The whole motivation is that mod authors are not always coordinating. The 448-mod modlist scenario is exactly the scenario where "authors don't know about each other's patches." A conflict detection mechanism that requires joint compilation falls back to the same runtime-only discovery that Clausewitz has today.

**Proposed fix:** The `clause-manifest/` system needs to record *outbound patch declarations* alongside event IDs and aliases. Every `#[patch(target = "X")]` in a crate writes a patch-declaration entry to the manifest. The `stellaris-vanilla-spec` (or a separate conflict-registry) can then serve as an aggregation point. `clause check` in workspace mode should actively cross-crate-scan for colliding patch targets. For distributed crates (not in the same workspace), the manifest entries become part of the crate's published API surface — a downstream user who imports two crates that both patch the same target gets a compile-time error at *their* build step. This is Cargo-style dependency conflict detection, applied to patch targets.

---

### C2 — Kind inference causality inversion (§9.4)

**The claim:** "Empirically classifies ~95% unambiguously" and edge cases use `#[as_X]` override.

**The gap:** The inference is body-local. But method bodies call other Clause methods, and those other methods have their own inferred kinds. When method A calls method B, the compiler infers A's kind partly based on whether B is a trigger or effect. If B's kind changes in a later commit (author adds a mutation, flipping B from `scripted_trigger` to `scripted_effect`), A's inferred kind also changes — but A has no `#[as_X]` annotation to anchor it, so A silently re-emits as a different construct at all its call sites.

**Concrete counterexample:**
```
// v1
fn is_member(who: Pop) -> bool {
    any_pop = { limit = { is_same_value = $who$ } }  // → scripted_trigger
}

// v2: author adds a debug effect
fn is_member(who: Pop) -> bool {
    log("Checking membership");                       // effect construct
    any_pop = { limit = { is_same_value = $who$ } }
}
// Now inferred as scripted_effect. Every caller that used is_member as a trigger
// now calls an effect — wrong Clausewitz semantics, no compile error.
```

This is particularly bad in Clausewitz because `scripted_trigger` and `scripted_effect` have fundamentally different calling semantics. A trigger is invokable inside `limit = { }` blocks. An effect is not. Code that calls `is_member` inside a `limit` block would produce silently invalid Clausewitz after the kind flips.

**Proposed fix:** Kind must be either explicit (via `#[as_X]`) or frozen at first successful inference and stored in the manifest. A kind change on a method that is *called by other methods* must be a breaking change that requires either re-annotating the callee with `#[as_X]` to pin it, or explicitly updating all callers. The compiler should warn: "Method `is_member` changed inferred kind from `scripted_trigger` to `scripted_effect`. Callers that use it in trigger position will emit invalid Clausewitz. Add `#[as_scripted_trigger]` to pin the kind, or update callers."

The manifest already stores symbol kinds for save-compatibility purposes (§16). Extending it to store inferred kinds costs little and prevents silent breakage.

---

### C3 — Option/Result codegen strategy is an invisible cross-crate ABI (§12.1)

**The claim:** "Compiler picks emit strategy based on inner type: Numeric inner → sentinel; Non-numeric or ambiguous → flag+value."

**The gap:** This "strategy" is actually an invisible ABI. If Crate A exposes a function returning `Option<PopId>` and uses the sentinel strategy, Crate B calling that function must also use the sentinel strategy to read the result. If Crate B was compiled against a different version of the stdlib or crate A where the strategy changed, both compile cleanly but the runtime behavior is wrong.

Worse: the sentinel value `-1` for `Option<PopId>` is undetectable as wrong at compile time. A function that returns an `int` directly and a function that returns `Option<PopId>` using the sentinel strategy produce structurally identical Clausewitz. The type information is completely erased at the ABI boundary. If a future version of stdlib changes the strategy (e.g., because `-1` is a valid PopId in some Stellaris version), every cross-crate caller silently breaks.

**Proposed fix:** The codegen strategy for `Option<T>` and `Result<T, E>` must be part of the crate's manifest and lockfile semantics. When Crate A publishes a function returning `Option<PopId>`, the manifest records which strategy was used. Crate B, when it calls that function, reads the manifest-declared strategy for that symbol and generates the matching read. Strategy changes are breaking changes that bump the manifest version and trigger a warning at cross-crate callsites. This is analogous to how Rust's ABI is versioned — the strategy is the ABI.

---

## 3. High Concerns
*(would-hurt-adoption-level — should be resolved before Phase 2 user-facing work)*

### H1 — The `:` overloading is a real readability problem (§7, §2 Open Questions)

**The design doc acknowledges this:** "Rust people will momentarily blink" (§7). The walkthrough marked it "if it's confusing in practice, reconsider later."

I am marking this as a high concern because the confusion is not superficial. The problem is:

```
trait Bloodline: Family { ... }    // : means "supertrait"
struct Bloodline: Country { ... }  // : means "bind target"
struct WorldClock: Singleton { ... } // : means "storage class"
```

These three usages of `:` do not "vibe as is-a" uniformly. `struct Bloodline: Country` does NOT mean Bloodline is-a Country — it means Bloodline's data lives on Country entities. That is a very different semantic. A user reading `struct Foo: Bar` cannot know whether Bar is:
- A trait (supertrait), meaning Foo has Bar's capabilities
- A vanilla scope (bind target), meaning Foo's variables live on Bar entities
- A carrier type like `Singleton` (storage class), meaning Foo is a global singleton

Three distinct semantics, one syntax, no syntactic differentiation. The disambiguation happens at type-check time (is Bar a trait or a Bind implementor?), but the reader cannot quickly categorize the relationship.

**Proposed fix:** Reserve `:` for supertraits only (Rust convention). Introduce an explicit bind keyword or `@` sigil for struct binding:

```
struct Bloodline on Country { ... }     // bind syntax: on, bound_to, via, @ etc.
struct WorldClock on Singleton { ... }
struct Foo { ... }                       // implicit Registry<Self> with no annotation
```

This makes the three categories syntactically distinct. Authors reading code can immediately parse the relationship type. The `on` keyword also reads naturally for the semantic: "Bloodline data lives ON Country entities."

If `on` is rejected for aesthetic reasons, `@` works: `struct Bloodline @Country { ... }`. The key is that bind-target syntax must not share a sigil with trait-bound syntax.

---

### H2 — `super.foo()` in multi-level patch chains has unspecified resolution semantics (§23.1)

**The claim:** "Chainable: a third-level patch's `super` calls the second-level, not the original."

**The gap:** The design specifies the single-chain case. It does not specify:

1. **Partial super chains:** What if Patch B only overrides method `foo()` but Patch A's impl also overrides methods `bar()` and `baz()`? Does Patch B's `super` inherit Patch A's `bar()` and `baz()`, or vanilla's?

2. **Patches across non-dependent crates:** Patch A patches `vanilla::Country::produce_energy`. Patch B (in a separate crate that does NOT depend on crate A) also patches the same target. The design says this is a compile error (§23.3). But if we fix C1 (patch manifest tracking), we need to decide: does the compile error produce a *merge request*, or does it require one patch to declare `super` of the other? And if it requires ordering, who decides which is "deeper"?

3. **`..super` struct-spread in patches-of-patches:** If Patch A uses `..super` to override specific fields in a struct definition, and Patch B's `..super` refers to Patch A's version, does `..super.modifiers` in Patch B refer to Patch A's `modifiers` (post-override) or vanilla's original? The design shows one level but does not address nested struct-update composition.

**Proposed fix:** Specify the patch chain resolution algorithm explicitly before M5. The algorithm needs to answer: "Given a directed graph of patches where each patch declares a target, what is the total order of super-chain resolution, and what does `super.foo()` resolve to at each level?" The simplest sound answer is: patch chain is a *linear* sequence ordered by crate load order (like CSS specificity), and `super.foo()` always means "the next lower in my linear chain." This must be documented as the algorithm, not left implicit.

---

### H3 — Sealed trait via private supertrait is not actually sealed for intra-crate abuse (§9.5)

**The design says:** The private supertrait pattern seals `CoreScope` from external implementation.

**The gap:** The private supertrait pattern seals against *external* crates, but NOT against items in the same crate that have access to the private module. In `stdlib/carrier.cw`:

```rust
mod private {
    pub trait Sealed {}
}

pub sealed trait Bind: private::Sealed {}
```

Any item in `stdlib/` can write `impl private::Sealed for MyType {}` and then `impl Bind for MyType {}` — because `private::Sealed` is accessible within the crate. This is by design in Rust (the pattern intentionally seals *externally*), but the design doc describes Bind as "sealed — only stdlib + the fixed set of carrier types impl this." This implies the fixed set is closed even within the stdlib crate.

If someone extends stdlib later (adding a new carrier), they simply add two impls. There is no mechanism to prevent this or audit it. The "fixed set" is a convention, not a mechanical guarantee.

**This also means:** If the megapatch crate imports stdlib and somehow gets access to the private module (e.g., via `pub(crate)` boundary misconfiguration), it could implement `Bind` on arbitrary types.

**Proposed fix:** The design doc's description of `Bind` as "sealed" should be qualified: "sealed against external (non-stdlib) crates." Within stdlib, the set of `Bind` implementors must be tracked by convention and enforced via a stdlib-internal lint. Consider a compile-time set declaration: `sealed_to! { Country, Pop, Planet, ..., Registry<T>, Singleton }` that the compiler cross-checks against actual `impl Bind` declarations in the stdlib crate. Any impl not on this list is a build error even inside stdlib.

---

### H4 — `expect`/`actual` vs `#[cfg(mod = "X")]` split has a gap class (§20.4)

**The design says:** "`#[cfg(mod = "X")]` for compile-time include/exclude; `expect`/`actual` for runtime verification with graceful degradation."

**The gap:** There is a class of cases that neither handles cleanly: *optional extensions that change behavior when present but degrade gracefully without the mod, but where the degradation logic needs compile-time generation.*

Example: Heritage wants to show NSC3 ship types in a dynasty event if NSC3 is present, and show vanilla ship types otherwise. This is neither:
- A feature that goes away entirely without NSC3 (`#[cfg]` case), nor
- A feature that requires NSC3 to work at all (`expect` case with runtime guard)

It is: compile-time conditional code generation with a default path. The right construct is something like:
```rust
#[cfg(mod = "nsc3")]
let ship_type = nsc3::ShipType::...;
#[cfg(not(mod = "nsc3"))]
let ship_type = vanilla::ShipType::...;
```

Clause does have `#[cfg(mod = "X")]` and its complement `#[cfg(not(mod = "X"))]`. But the design doc does not explicitly address using these together in a method body for conditional branching. The `#[cfg]` decorators are shown on items (traits, impls, fns), not on expressions or statements within a body. If `#[cfg]` only applies at item granularity, authors cannot write the conditional-branch-within-method pattern.

**Proposed fix:** Clarify that `#[cfg]` applies at *statement* granularity inside method bodies, not just at item granularity. Show an example of `#[cfg]`-guarded branches within a single method. If statement-level `#[cfg]` is not planned for v1, document this explicitly as a known limitation and state the workaround (separate `#[cfg]`-gated methods that call a shared helper).

---

### H5 — Lambda scope-opening syntax is semantically misleading (§24.4)

**The design shows:**
```
name(|p| { p.foo })   →   name = { foo = ... }
```

**The gap:** The `|p|` syntax in Rust universally means "closure that captures environment variables and is a callable value." In Clause, this syntax means something structurally different: it opens a scope block and binds the scope token to `p`. There is no capture, no callable value, no heap allocation, no first-class function. It does not compose with the rest of the language in the way a lambda would.

Concretely: in Rust you can do `let f = |p| { p.foo }; collection.map(f)`. In Clause, can you do `let scope_fn = |p| { p.foo }` and pass it around? Almost certainly not — there is no runtime representation. But the syntax strongly implies you can.

This creates a "false friend" problem where Rust users confidently write code that looks valid but means something different. The divergence from Rust is not documented with a prominent warning.

**Additionally:** `for x in iter { body }` (§18.3) uses `x` as if it is a binding that persists across iterations. In Clausewitz's `every_*` construct, there is no persistent binding — each iteration is a separate scope evaluation. The code `let captured = x; do_later_thing_with(captured)` after the loop end would be silently invalid.

**Proposed fix:** Use a distinct syntax that does not borrow Rust's closure syntax. Options:
- `name: { pop = p, p.foo }` (explicit scope-open with named binding)
- `name in |p| { p.foo }` (the `in` keyword signals scope traversal, not closure)
- `name.with(p: { p.foo })` (method-call shape that does not look like a lambda)

If `|p|` syntax is kept, it must be accompanied by explicit documentation and error messages that say "this is not a closure, it is a scope-binding. You cannot store it, pass it, or call it."

---

## 4. Medium Concerns
*(worth addressing, not blocking)*

### M1 — Generic monomorphization with multiple `HasModifier<T>` impls (§11)

The design allows `impl HasModifier<Unity> for Country` and `impl HasModifier<Energy> for Country` simultaneously. This is the correct generics-over-associated-types decision. However:

The design does not specify what happens when a generic method has a blanket impl and a specific impl for the same concrete type. Example:

```rust
impl<T> HasModifier<T> for Country { ... }   // blanket
impl HasModifier<Unity> for Country { ... }  // specific
```

In stable Rust this is a coherence error. In Clause, since we have no specialization, this must also be an error. But the design doc does not state this explicitly. Without explicit statement, an implementer might allow it (and produce ambiguous monomorphization) or forbid it (producing a confusing error when the author intended the specific override to win).

**Fix:** Explicitly state that blanket impls and specific impls for the same `(Type, Trait, TypeParam)` triple are a coherence error without specialization support.

---

### M2 — `event for Self` semantics are underspecified for Registry-bound structs (§15)

The design says: "For `struct Bloodline: Registry<Self>`, Self isn't a fireable engine scope — events inside such an impl must declare `for X` explicitly." This is correct behavior. But the design does not address what happens when:

1. An event inside `impl Bloodline` declares `for Country` but the registry is `Registry<KeyedBy<PlanetId>>`. The author meant "Planet" but wrote "Country." This is a compile error only if the compiler understands the relationship between the registry's key type and the expected scope.

2. An event declares `for Self` and `Self` IS a scope-bound struct (e.g., `struct CountryFlags: Country`). Should `for Self` resolve to `Country` in this case? The design says yes for `MatriarchDies` in the Bloodline example, but only because it's explicitly scoped. The general rule for struct-bind-target scope inference into events is not stated.

**Fix:** Document the event scope resolution algorithm: "The `for` clause resolves to: (a) the explicit type if given, (b) the struct's bind target if it is a vanilla scope, (c) error if the struct's bind target is a carrier — you must provide an explicit `for X`."

---

### M3 — `Cached<T>` has no cross-field invalidation story (§14.6)

The design says: "No language-level auto-tracking of read dependencies in v1. Authors who need cross-field invalidation call `.invalidate()` explicitly."

This is honest but creates a real correctness trap. The pattern:

```rust
fn add_member(...) {
    // ... modify size ...
    self.cached_lineage_depth.invalidate();   // author must remember this
}
```

If the author later adds a second mutation path that affects lineage depth but forgets `.invalidate()`, the cache silently serves stale values. In a language that claims compile-time safety, manual cache invalidation is a notable hole.

**Fix:** Consider a `#[invalidates("cached_lineage_depth")]` annotation on mutation methods, allowing the compiler to verify at build time that every `mut` method that touches fields affecting a cached value has a corresponding invalidation call. This is cheap to implement (static analysis of method bodies) and converts a silent runtime bug into a compile-time warning.

---

### M4 — Method call sugar desugaring is underspecified for multi-argument calls (§18.2)

The design shows: `self.bloodline.add_member(who = new_pop)` → `heritage_bloodline_add_member = { who = new_pop }`.

But Clausewitz scripted effects with multiple parameters have no guaranteed parameter ordering. If a method takes two parameters (`add_member(who: Pop, priority: int)`), the Clausewitz call site would be `heritage_bloodline_add_member = { who = X priority = Y }`. Since Clausewitz parameters are named (not positional), the order does not matter for Clausewitz. But the Clause call syntax `self.bloodline.add_member(X, Y)` implies positional. Authors may write `self.bloodline.add_member(X, Y)` expecting the first arg to be `who` and the second to be `priority`, but the desugaring must name them. If authors write positional args without names, the compiler needs to assign names by parameter declaration order.

**Fix:** Explicitly state whether Clause method calls require named arguments (`add_member(who = X, priority = Y)`) or allow positional (`add_member(X, Y)`). Rust allows both. If both are allowed, specify the desugaring rule for positional args. The emitted Clausewitz should always use named form.

---

### M5 — `while` loop desugaring may conflict with Clausewitz `while` semantics (§18.3)

The design maps `while cond { body }` → `while = { limit = { cond } body }`.

Clausewitz's `while` loop has engine-enforced iteration limits (typically 100 iterations max) to prevent infinite loops. Clause does not document this constraint. An author writing `while some_condition { ... }` might write logically correct Clause that silently truncates at 100 iterations in-engine.

**Fix:** Document the Clausewitz `while` iteration limit as a known constraint visible in the `while` construct. Consider a lint that fires on `while` bodies that could theoretically exceed the limit. At minimum, the `clause check` output for any `while` loop should note the iteration limit.

---

### M6 — `#[file("...")]` escape hatch creates silent collision risks (§22.6)

The design says `#[file("...")]` is lint-warned as "unusual — document the reason." But the collision detection (§22.4) must specifically handle the case where a `#[file("...")]`-routed item shares an output path with an auto-routed item. The design says collision is a compile error for same-path same-name pairs. But `#[file("...")]` is often used to target specific vanilla-structure paths where collision detection is the *desired* behavior (you WANT to stomp the vanilla file). 

This means the collision detection logic needs to distinguish: "two Clause items colliding" (always an error) vs. "Clause item targeting a vanilla file" (probably intentional, warn-not-error). The distinction is not specified.

---

### M7 — Type-level union (`A | B`) interaction with the orphan rule is undefined (§7)

The design introduces `type Diplomat = Country | Federation` (§7). But union types interact with the orphan rule in non-trivial ways:

- Can you `impl Trait for (Country | Federation)`? This would be an impl for a compound type. The orphan rule applies to concrete types — does it apply to union types?
- If `Country | Federation` is sugar for "either of these", is an `impl Trait for Diplomat` legal only if you own `Diplomat` (the type alias), or must you also own both `Country` and `Federation`?
- What if two mods both define `type Diplomat = Country | Federation`? Are these the same type for orphan purposes?

The design doc treats union types as a natural addition but does not specify their coherence rules.

---

## 5. What's Genuinely Well-Designed

### 5.1 The orphan rule as mechanical coherence enforcement (§10.1, §23)

Using Rust's orphan rule to prevent two mods from silently patching the same thing is the right structural solution. The design correctly identifies that this maps `(Trait, Type)` pairs to crate ownership and forces multi-mod conflicts to be explicitly resolved. The `#[patch]` bypass requiring a documented `reason` string is the right shape: it allows the escape but makes it auditable. This is the best possible outcome for the "consolidate, don't choose" principle — the type system enforces it mechanically rather than relying on process.

### 5.2 Kind inference with explicit override (§9.4, §14.7)

The inference-from-body-content approach (trigger constructs → `scripted_trigger`, effect constructs → `scripted_effect`, etc.) with `#[as_X]` override annotations is the right design. Authors writing domain logic should not need to think about Clausewitz emit kinds — that is compiler knowledge. The explicit override path exists for edge cases. The `#[prefer(...)]` annotation for performance-sensitive tiebreaking is appropriately distinguished from the `#[as_X]` correctness override. This two-level annotation system is clean.

### 5.3 The `stellaris-vanilla-spec` crate architecture (§25)

Multi-source ingest (trigger_docs + cwtools-config + vanilla scripts) with cross-validation, curated overlays for the top 50 scopes, and auto-generation for the long tail is a mature approach to the bootstrapping problem. The decision to own this in-repo and version it with Stellaris releases (rather than depending on a community-maintained external source) is correct for a project that needs reliable builds. The curated-overlay pattern means the high-value surface (Country, Pop, Planet) is carefully designed while the long tail is automated.

### 5.4 Source mapping through the entire pipeline (§18.4, §27.2)

`# >> heritage/bloodline.cw:42 — fn add_member()` comments in emitted Clausewitz, plus the structured `source_map.json` file, is correct infrastructure. This is essential for debugging: when something breaks in-game, authors need to trace from the Clausewitz the engine loaded back to the `.cw` source that generated it. Most DSL-to-host-language compilers treat this as an afterthought. Clause treats it as a first-class deliverable from day one. The `clause explain` command built on top of this is excellent ergonomics.

### 5.5 Manifest-driven save compatibility (§16, §17)

The combination of auto-detected renames via git diff, `#[supersedes]` / `#[deprecated]` for explicit guidance, and auto-scaffolded sham handlers for save migration is a complete and sound approach to the save compatibility problem. The fact that the manifest is reviewable in git diff (not a binary artifact) is especially good — it makes the implicit explicit, which is exactly the right design philosophy for a mod toolchain.

### 5.6 `Cached<T>` with typed refresh intervals (§14.6)

Modeling caching as a first-class language construct rather than a "call the effect on a pulse" comment is excellent. `Cached<int, refresh = "monthly">` is expressive, author-visible, and compilable to the correct `on_action` wiring without author knowledge of which on_action hook to use. This is Tenet 11 in action on a non-trivial example.

---

## 6. Open Questions the Design Marks as Resolved But Deserve More Thought

### OQ1 — The `:` ambiguity (design doc §30 item 2)

The walkthrough marked this "accepted; reconsider if confusing in practice." But "if confusing in practice" means the confusion has already cost users time. Better to decide now while the syntax is not yet implemented. The counterargument ("internally consistent is-a vibe") is weaker than it appears — `struct Bloodline: Country` does NOT semantically mean Bloodline "is-a" Country. It means Bloodline's data is *hosted by* Country, which is a hosting/delegation relationship, not an inheritance relationship. The Rust community's decision to use `:` exclusively for trait bounds/supertraits (not for type-bound hosting) is a considered ergonomic choice. Overloading `:` here is trading syntax simplicity for semantic clarity.

### OQ2 — Error message philosophy for the Clausewitz-integration errors specifically

The Elm-friendly error target is correct for type errors (`"A Country scope does not support planet_event"`). But for the class of errors unique to Clause — patch conflicts, kind-inference ambiguities, super-chain resolution failures — the errors are harder to express in plain English and the fixes are more complex. These error classes are not analogous to Elm's type mismatch errors. The design should specify some representative examples of these error messages *before* M5 implements the type checker, because the error message design constrains what diagnostic information the type checker must track.

### OQ3 — What is `Self` in a trait default method body?

The design says `Self` refers to the impl's target type. But in a trait *default method* (not yet implemented by any impl), `Self` is abstract — the trait cannot make assumptions about what `Self` is except what its supertraits guarantee. The design shows no examples of default methods. In Rust, default methods can only call methods that are declared on the same trait or its supertraits; calling `self.some_vanilla_effect()` in a default method only works if that effect is declared in the trait or a supertrait. This constraint needs to be explicitly stated for the Clause case.

### OQ4 — The `Array<T, MAX_SIZE>` const-generic indirection

The design uses `mut member_vector: Array<PopId, MAX_SIZE>` where `MAX_SIZE` is a struct-level `const`. This is a const-generic array sized by an associated constant of the owning struct. In Rust this is a known complexity point: you need to propagate the const generics through to the `Array<T, const N: int>` type, and the struct declaration order matters. Specifically: `Array<PopId, MAX_SIZE>` in the field declaration requires `MAX_SIZE` to be in scope at the point of the struct's type signature. This is fine if `const MAX_SIZE` is declared before `member_vector` in the struct body, but the design does not specify the evaluation order of struct fields for const-generic purposes. What if `MAX_SIZE` is declared in a supertrait, not the struct itself?

### OQ5 — Visibility of patch targets in `clause explain`

The design shows `clause explain heritage_bloodline_add_member` listing `patches: (none)`. What does the output look like when a method IS patched? Specifically: can a caller query "what patches apply to this method in my current workspace?" This is critical for megapatch authors debugging why a specific vanilla method is not behaving as expected. The output format for the patched case is not shown.

### OQ6 — Default method bodies in traits and their emit kind

The design says traits have method signatures only (§9) with kind inferred at the impl site. But associated constants with defaults (`const MAX_LINEAGE_DEPTH: int = 8`) are shown (§9.2). Are default *method bodies* in traits planned? If so, what is the kind inference target — is it inferred at the trait declaration site (from the default body), or at the impl site (from the overriding body if provided, or from the default body if not)? This interacts with C2 (kind inference causality).

---

## 7. Comparison Table

| Axis | TypeScript | Elm | Rust | PureScript | Clause |
|---|---|---|---|---|---|
| **Type soundness** | Intentionally unsound (`any`, bivariant functions) | Sound by design | Sound with `unsafe` escape | Sound with `foreign` escape | Target: sound — but see C1 (patch ABI), C2 (kind inference), C3 (Option codegen) |
| **Interop model** | `.d.ts` ambient declarations (excellent) | Ports (typed channels) | `extern` FFI with `unsafe` | `foreign import` (trusted annotation) | `extern` declarations + `stellaris-vanilla-spec` (well-designed, matches domain) |
| **Error ergonomics** | Good for simple, degrades for complex generics | Excellent (first-person, fix-suggesting) | Very good (structured, with code context) | Poor for type class errors (exposes unification internals) | Target: Elm-style (correct goal; hardest errors will be kind-inference and patch-chain failures) |
| **Nominal vs structural** | Structural (footgun-prone, mitigable with brands) | Nominal ADTs | Nominal structs, structural records | Structural with row poly | Nominal scopes (correct for Clausewitz) |
| **Module system** | Good (namespaces, but not coherence-enforcing) | Module-level, simple | Excellent (coherence-enforcing orphan rule, fine-grained visibility) | Module-level | Rust-clone (correct — coherence is the mechanism) |
| **Patch/override story** | No built-in concept | No built-in concept | No built-in concept (specialization unstable) | No built-in concept | First-class `#[patch]` with orphan-bypass — novel and well-motivated for the domain |
| **DSL compile target** | JavaScript (rich, well-specified) | JavaScript | Machine code / LLVM IR | JavaScript | Clausewitz (weakly-typed, constraint-poor, target is challenging) |
| **Escape hatch** | `any` (too permissive, leaks) | None (too restrictive) | `unsafe` (sound but complex) | `foreign import` (per-call trust) | `clausewitz! { }` / `#[as_X]` / `#[file(...)]` — layered correctly, most permissive only for output routing |
| **Domain fit** | General-purpose, poor fit for scope tracking | General-purpose, poor fit for scope tracking | Systems, poor fit for codegen DSL use | General-purpose | Domain-specific, scope-aware — best fit by design |
| **Save compatibility** | No concept | No concept | No concept | No concept | First-class manifests + sham handlers — genuinely novel and valuable |
| **Multi-crate conflict detection** | None | None | Via Cargo resolution (version conflicts only) | None | Orphan rule (sound within workspace; see C1 for distributed case) |

---

## 8. Summary for Implementers

**Before M5 (type checker) starts, resolve:**
- C1: Patch collision detection must work across independently-compiled crates, not just within a single workspace build. Design the patch-declaration manifest entries.
- C2: Kind inference must be frozen on first inference and stored; changes must be breaking. Design the kind-pinning mechanism.
- C3: Option/Result codegen strategy is an ABI. Store it in the manifest per-symbol and verify at cross-crate callsites.

**Before Phase 2 (user-facing Heritage in Clause) starts, resolve:**
- H1: The `:` bind syntax ambiguity. Pick a different sigil for struct binding before anyone writes real code against it.
- H2: Specify the patch chain resolution algorithm as a formal rule, not just an example.
- H4: Clarify that `#[cfg]` applies at statement granularity within method bodies.
- H5: Either rename the lambda-scope syntax or document explicitly that it is not a closure.

**During implementation, track but do not block on:**
- M1 through M7 (see §4 above for specifics).

**The design is strong on:** orphan-rule coherence, the `extern` → `stellaris-vanilla-spec` pipeline, source mapping, save compatibility, and Tenet 11. Do not change these.
