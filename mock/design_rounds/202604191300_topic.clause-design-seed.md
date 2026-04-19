# Clause — Mod Authoring Language for Stellaris

**Status:** DESIGN v2 — COMPLETE + REVIEWED. All 5 chunks settled with user via interactive walkthrough on 2026-04-16. Four independent review agents (language-design, modder, compiler-impl, migration) returned findings; this document amended end-to-end to incorporate them. Implementation tasks filed; ready for M2.

**Pivot amendment (2026-04-17):** Project pivoted to standalone Clause-first (no third-party mods at runtime; playset = vanilla + our crates only). This doc's §1 "what Clause is" + §2 motivation + §28 MVP framing still apply, but the strategic context has changed: Clause's purpose is now authoring *our* gameplay from scratch on vanilla substrate, not integrating 480 third-party mods. **§26 now documents the post-pivot workspace architecture** (foundation tier `std`/`vanilla`/`dlc_*`, shared abstraction `associations`, content `_expanded` taxonomy, DLC-gated compilation, cross-cutting-domains principle, conventions-not-mandates principle) — that section supersedes the earlier megapatch/cross-mod framing in §26 and §20.2. Remaining pivot follow-ups: §1, §2, §28 framing updates (task D2 on TENETS parallels it), Heritage docs (task D4). See memory entries `project_direction_standalone_20260417`, `project_crate_workspace`, `feedback_conventions_not_mandates`, `feedback_env_specific_playset_compile`, `feedback_cross_cutting_domains`, `feedback_externs_demand_driven`, `feedback_think_fresh_not_catalogue` for the full pivot context.

**v2 amendments address**:
- Kind inference is manifest-frozen to prevent silent caller breakage (§9.4).
- Option/Result encoding strategy is a per-type manifest-tracked ABI (§12.1, §16).
- `replace_in_item` gap closed via token-aware `#[patch_body]` over G2 lexer (§23).
- Patch chain total ordering algorithm specified (§23).
- Patch declarations exposed as outbound manifest ABI for distributed-crate collision detection (§16, §23).
- Phase 1 gate strengthened: real `#[patch]` + `super.foo()`, not hello-world (§28).
- Phase 2 gate clarified: diff-approved is primary; byte-identical only where feasible (§28).
- `extern` declarations carry explicit kind field (§13).
- TranspileStack data structure specified (§24).
- `?` inside `for` is a v1 compile error with suggested `match` pattern (§24).
- `Iter<T>` codegen lookup table spec (§14).
- `Cached<T>` multi-file emit resolved (§14).
- Hidden `Registry<Self>` entity resilience (§31).
- Missing features banked as planned extensions (§31).
- Sequencing: M9 parallel with M2-M4; M11 after M6 (§29).
- MVP scope concrete (§28, §29).

Settled in chunk 2: bind syntax via `:`, single sealed `Bind` trait, Registry/Singleton stdlib bind targets, Tenet 11 (hide engine, surface intent), temporal memoization via stdlib `Cached<T>`, `#[prefer(...)]` emit-strategy hints.

Settled in chunk 3: module system deepening (Modfile.toml, workspaces, use-tree, re-exports), `expect`/`actual` for cross-mod runtime contracts alongside `#[cfg(mod = "X")]`, decorator full catalog, file routing rules with auto-layout + `#[file("...")]` escape hatch.

Settled in chunk 4: patches via `super.foo()` method overrides + `..super` struct-spread (no more find/replace), Rust-ish transpiled bodies with lambda-like scope-opening syntax, dedicated `stellaris-vanilla-spec` crate with multi-source ingest + cross-validation, `clause explain` unified inspector CLI, project-structure reframe (Clause is the toolchain; the repo is a Clause workspace of which `megapatch` is ONE crate among many).

**Predecessors:**
- `2026-04-16-mod-language-design-startpoint.md` — strawman + research consolidation, kept as reference.
- `.cache/reviews/2026-04-16/rust-traits-research.md` — Rust trait-system research.
- `.cache/reviews/2026-04-16/typed-dsl-prior-art.md` — typed-DSL-to-dynamic-target prior art.
- **Standalone pivot (2026-04-17)** — captured in memory `project_direction_standalone_20260417` and `project_crate_workspace`. Superseded design docs preserved at `.archive/design-history/megapatch-era/`. Superseded research corpus preserved at `.archive/research/modlist-audit/`.

**Name:** `clause`. **File extension:** `.cse`.

---

## 1. What Clause is

Clause is a Rust-inspired, statically-typed mod authoring language that compiles to Clausewitz script for Paradox grand-strategy games (Stellaris specifically; broader applicability later). Authors write traits, structs, and impls against abstract domain concepts (`Bloodline`, `Family`, `HasFleets`); Clause's compiler validates contracts end-to-end, routes output files automatically, and emits Clausewitz that the engine consumes unchanged.

The compiler's backend is the existing megapatch pipeline (G-batch). Clause is a new frontend that produces a token stream consumable by that pipeline alongside hand-authored Clausewitz from source mods.

---

## 2. Motivation

Six things are broken about authoring directly in Clausewitz/YAML today:

1. **No abstraction barrier.** Calls name vanilla effects/triggers literally; engine renames break everything mechanically. Refactors are grep + pray.
2. **File location is a design decision authors shouldn't make.** "Which `_declarations.yml`?" "Which mod's directory wins?" — these are routing questions the compiler should answer.
3. **Conflict resolution is invisible.** Mod A overrides X, Mod B overrides X → silent LIOS pick on filename. No compile-time signal.
4. **Cross-cutting concerns require copy-paste.** "Has fleets" gets re-implemented per scope (Country, Federation, Sector).
5. **Type errors only surface in playtest.** Pass a Fleet where a Country is expected → loading-screen hang three hours later.
6. **Save/mod-version compatibility is ad-hoc.** Renaming a scripted_effect or variable breaks existing saves silently.

Clause addresses each:
1. Authors write against trait contracts; vanilla rename → update one impl, every call site unchanged.
2. Compiler-routed output via decorators + module hierarchy.
3. Rust's orphan rule mechanically prevents two mods from impl-ing the same trait for the same type without explicit consolidation.
4. Trait + impl inheritance handles cross-cutting concerns once.
5. Compile-time type checking catches scope mismatches.
6. Manifest-managed event IDs + alias system + auto-scaffolded sham handlers preserve save compatibility across renames.

---

## 3. Goals

1. **Compile-time type safety** for everything Stellaris fails silently on.
2. **Abstraction barrier** — write against `Bloodline`, not `country.heritage_bloodline_size`.
3. **Compiler-routed output** — no file-location decisions for authors.
4. **Mechanical conflict resolution** via trait coherence (orphan rule).
5. **Refactoring safety** — rename → compiler finds all sites.
6. **Save compatibility on renames** via auto-managed manifest + alias system.
7. **Gradual adoption** — coexists with YAML patches and raw Clausewitz; doesn't require big-bang migration.
8. **Bootstrap from existing schemas** — `cwtools-stellaris-config` + Stellaris' own `script_documentation/` logs auto-generate vanilla declarations.
9. **Familiar, readable** — Rust-y syntax; bodies look like Clausewitz with type validation; mod authors ramp up by recognizing both halves.

---

## 4. Non-goals

- Replacing Clausewitz at runtime (output stays Clausewitz; engine sees no difference).
- Type-checking dynamic engine state (event_target resolution depends on save state).
- Performance optimization at the language level (backend concern; surfaces only via `#[prefer(...)]` hints — see §14).
- Replacing localisation `.yml` files (different parser, different problem).
- Replacing artwork pipelines.
- General-purpose programming language (Clause is a DSL, not Python).
- Runtime trait objects / dynamic dispatch (Clausewitz can't express it; we explicitly reject `dyn Trait`).
- Lifetime annotations (Clausewitz has no ownership model we can statically track; better no layer than fragile layer).

## 4.5. Design tenets

These are load-bearing principles that override local convenience when in tension.

### Tenet 11 — Hide the engine, surface the intent

Use whatever engine tricks the compiler needs (hidden countries for registries, fleet-rename string storage, inline_script promotion, scratch variables for `let mut`, sentinel values for `Option<int>`). Never surface them in `.cse` source. Authors write against domain concepts (`Bloodline`, `bloodline.add_member(pop)`, `name: string`); the compiler picks the engine-faithful realization. The only time engine details surface is debug output (which shows both `.cse` source AND emitted Clausewitz side-by-side via source-mapping comments).

Corollaries:
- New stdlib types may use any engine workaround under the hood, as long as the surface API is normal.
- `#[prefer(...)]` annotations are HINTS for tiebreakers, not commands — compiler still picks the most engine-correct emit.
- When a Stellaris constraint forces ugly emitted code, that's fine; what matters is the source stays clean.

### Tenet 12 — Refactoring safety is non-negotiable

Renames, moves, and structural changes are detected by the compiler (via git-diff against the last-tagged version) and propagated automatically through the manifest + alias system. Authors never hand-edit the manifest; they annotate with `#[supersedes(...)]` / `#[deprecated(...)]` for cases the compiler can't infer. Save compatibility across mod versions is a first-class concern.

---

## 5. Naming conventions

| Item | Style | Example |
|---|---|---|
| Types (struct, trait, enum) | PascalCase | `Bloodline`, `HasFleets`, `Modifier<T>` |
| Events | PascalCase | `event NewHeir for Country`, `event MatriarchDies for Self` |
| Methods, functions, free items | snake_case | `fn add_member()`, `fn contains()` |
| Fields, locals, variables | snake_case | `mut size: int`, `let new_idx` |
| Constants | SCREAMING_SNAKE_CASE | `const MAX_SIZE: int = 100` |
| Modules | snake_case | `mod heritage::bloodline` |
| Crates (= Stellaris mods) | snake_case | `crate heritage` |

The compiler converts to Clausewitz output naming automatically (`heritage_bloodline_add_member`, `heritage.dynasty.new_heir.0001`, etc. — see §13).

---

## 6. Crate model

Every Clause project has a single **crate entrypoint** at `src/lib.cse`. This file declares all top-level modules via `mod foo;` (Rust convention). Reachability from the crate root determines whether a module is in the build — files that aren't declared anywhere are dead and dropped silently (build-time warning, not error).

```
heritage/
  mod.toml                        # crate metadata (deps, features)
  src/
    lib.cse                        # crate root, declares top-level mods
    bloodline.cse                  # mod heritage::bloodline
    events/
      mod.cse                      # mod heritage::events; declares dynasty, marriage
      dynasty.cse
      marriage.cse
  clause-manifest/
    event_ids.manifest            # auto-generated; reviewable
    symbol_aliases.manifest       # auto-generated; reviewable
```

Inside `src/lib.cse`:
```rust
pub mod bloodline;
pub mod events;

// Re-exports for downstream users
pub use bloodline::Bloodline;
pub use events::dynasty::NewHeir;
```

Inside `src/events/mod.cse`:
```rust
pub mod dynasty;
pub mod marriage;
```

**Inline modules** (Rust-style braces) are also allowed:
```rust
mod private_helpers {
    fn shared_logic() -> int { ... }
}
```

**Visibility** follows Rust:
- `pub` — visible to anyone (including downstream mods that depend on this crate)
- `pub(crate)` — visible within current crate only
- `pub(super)` — visible to parent module
- `pub(in path)` — visible within specified module path
- (default) — private to current module

Visibility GATES what other mods can extend: marking a trait `pub(crate)` means no other mod can `impl` it. This is the primary mechanism for declaring API stability vs internals.

---

## 7. The type system at a glance

Five concepts:

1. **`struct`** — bundle of mutable variables + immutable constants, **bound** to a bind target (vanilla entity, `Registry<Self>`, or `Singleton`) via `:` syntax.
2. **`trait`** — contract: a set of `fn` signatures any implementor must provide.
3. **`impl`** — provides a trait's methods for a concrete type. Subject to Rust's orphan rule.
4. **`extern`** — interface-only declarations of engine-provided types and contracts. No code generated; only contract validation.
5. **Generics** — `Type<T>`, monomorphized at codegen.

Plus:
- **`Bind`** — sealed stdlib trait. Anything that can be a struct's bind target implements it. Vanilla scopes, `Registry<T>`, `Singleton`, custom mod-defined registries.
- **`Option<T>` and `Result<T, E>`** as the convention for "may be absent" / "may fail." No nullability, no nulls.
- **`enum`** for tagged unions.
- **Type-level union (`A | B`)** for multi-scope validity (e.g., `Diplomat = Country | Federation`).
- **Supertraits** for inheritance (`trait AscendantCorvette: Corvette`).
- **Sealed traits** via private-supertrait pattern, when system traits should not be implementable by mods.

### Bind syntax (the colon)

`struct Foo: Bar { ... }` declares Foo bound to Bar (Bar must impl `Bind`). Default when omitted: `Registry<Self>` (compiler-managed, hidden engine entity).

| Syntax | Meaning |
|---|---|
| `struct Foo { … }` | Implicit `: Registry<Self>`. Compiler allocates a hidden registry to hold instances. |
| `struct Foo: Country { … }` | One Foo per Country. Data lives on Country variables. |
| `struct Foo: Singleton { … }` | Single instance. Hidden global entity. |
| `struct Foo: Registry<KeyedBy<LeaderId>> { … }` | Registry indexed by LeaderId. |

Note on the `:` reuse: in Rust, `:` is for trait bounds and supertraits — not for struct binding. We're repurposing for "bound to bind target." Internally consistent (struct's `:` = bind, trait's `:` = supertrait — both vibe as "is-a"); Rust people will momentarily blink, but the natural read wins.

---

## 8. Structs

A struct represents a logical bundle of state, bound to a bind target (vanilla entity, registry, or singleton). Fields auto-generate to `@<crate>_<struct_snake>_<field>` Clausewitz variables on the bound bind target.

```rust
// Default — bound to Registry<Self>; compiler allocates a hidden registry.
struct Bloodline {
    const MAX_SIZE: int = 100,
    mut size: int = 0,
    mut last_index: int = 0,
    mut member_vector: Array<PopId, MAX_SIZE>,
    mut name: string = "Unnamed",   // string fields auto-route through the stdlib StringStorage bind target (Tenet 11)
}

// Bound to Country — one Bloodline per Country, data lives on Country variables.
struct CountryFlags: Country {
    mut last_inspection: int = 0,
}

// Singleton — single global instance.
struct WorldClock: Singleton {
    mut current_year: int = 2200,
}
```

Codegen for the default-Registry case produces:
- A hidden country (or analogous engine entity) allocated at game start as the registry root.
- `@heritage_bloodline_size_<instance_id>` etc., or fleet-named slots, depending on the optimal Stellaris workaround. Author never sees this.
- `MAX_SIZE` is compile-time, inlined at use sites; no runtime variable.

### 8.1 Field declarations

| Modifier | Meaning |
|---|---|
| (default) | Immutable. Cannot be modified after struct initialization. Compile-time scope check. |
| `mut` | Mutable. Can be modified at runtime via `change_variable` / `set_variable`. |
| `const` | Compile-time constant. Inlined at use sites. No runtime presence. |

Immutable-by-default is a compile-time scope-analysis check — cheap, no runtime cost.

### 8.2 Bind targets and the `Bind` sealed trait

The thing after `:` must implement `Bind` (sealed, declared in stdlib). Members of `Bind`:

- Vanilla entities: `Country`, `Pop`, `Planet`, `Fleet`, `Leader`, `Federation`, `Sector`, `Megastructure`, ... (every engine scope).
- Stdlib bind targets: `Registry<T>`, `Singleton`, `Registry<KeyedBy<K>>`, `Registry<Capacity<N>>`.
- Mod-defined custom bind targets (rare; advanced use case for mods that want a specific storage strategy).

The `Bind` trait is sealed via the private-supertrait pattern (§9.5) — only stdlib + the small set of declared bind targets can satisfy it. Mod authors don't bind to arbitrary types.

### 8.3 Struct types are nominal

Two structs with identical fields are NOT compatible (no structural unification). `Bloodline` and `LineageTracker` are distinct types even if they look the same — semantic intent matters more than shape.

### 8.4 Strings via implicit StringStorage (Tenet 11 in action)

Stellaris has no string variables; `string` field codegen routes through a stdlib-managed `StringStorage` bind target (typically a hidden country with fleet-rename slots, or whatever the most efficient engine workaround is). Author writes `bloodline.name = "Targaryen"`; compiler emits the appropriate fleet-rename / variable-pair pattern. Never surfaced.

---

## 9. Traits

A trait declares method signatures any implementor must provide. **Always `fn`**, no kind keywords (`trigger`/`effect`/`value`) — kind is inferred at the impl site by the compiler from body content (§9.4).

```rust
/// Tracks bloodline membership.
trait Bloodline {
    /// True if `who` is a member of this bloodline.
    fn contains(who: Pop) -> bool;

    /// Add a member; returns the new index, or CapacityFull if the
    /// bloodline is at capacity.
    fn add_member(who: Pop) -> Result<int, CapacityFull>;

    /// First/founder member, if any.
    fn matriarch() -> Option<Pop>;
}
```

### 9.1 Method receiver

Methods accept `self`, `mut self`, `&self`, or `&mut self` as their first parameter. The four forms are semantically equivalent in Clause — since the engine has no ownership model, the reference forms are erased sugar (§10.3). Authors pick whichever reads best for the method's intent; the compiler treats them identically.

The method's receiver is bound to the active scope established by `#[scope(...)]` on the struct. Inside the body, `self.field` accesses the struct's variables; the compiler expands to the auto-generated Clausewitz name.

```rust
impl Bloodline {
    fn mark(&self) { … }           // reads self
    fn step(&mut self) { … }       // mutates self
    fn consume(mut self) { … }     // would-be move; identical emit
}
```

All four call sites lower to the same Clausewitz shape — the reference-vs-value distinction is purely author-facing ergonomics.

### 9.2 Associated types and constants

```rust
trait Family {
    type Member;                          // implementor picks the type
    const MAX_LINEAGE_DEPTH: int = 8;     // default; overridable
    fn add_member(who: Self::Member) -> Result<int, CapacityFull>;
}

impl Family for Bloodline {
    type Member = Pop;
    const MAX_LINEAGE_DEPTH: int = 12;
    fn add_member(who: Pop) -> Result<int, CapacityFull> { ... }
}
```

Use **associated types** (`type Member;`) when there's exactly one answer per implementor. Use **generics** (`<T>`) when implementors choose freely.

### 9.3 Supertraits

```rust
trait Owned { fn owner() -> Country; }

// Pop is-a Owned
trait Pop: Owned { ... }

// Function works for any Owned thing
fn count_owner_resources(x: Owned) -> int {
    x.owner().total_resources()
}
```

Supertrait chaining handles "ascendant corvette inherits corvette compat" cleanly. No specialization (Rust still ships it as nightly-unstable — supertraits are sound and sufficient).

### 9.4 Purity + function decomposition (v3 replacement for kind-tracking)

Functions are classified by **purity**, not by Clausewitz kind. The VALUE contract of a function (`fn() -> int` returns an int) is stable regardless of what side effects appear in its body. What changes with side effects is WHERE the function can be called from — and the compiler handles that invisibly via auto-hoisting + decomposition.

#### 9.4.1 Purity classification

| Class | Body shape | Example |
|---|---|---|
| `pure` | No side effects; return depends only on args + reads of struct state | `fn score() -> int { return self.size + 5; }` |
| `deferred` | Marked `#[deferred(...)]`; explicit queue/drain semantics (§32.6) | `log`, telemetry, audit-trail functions |
| `impure_hoistable` | Side effects + return; value can be computed without running effects at trigger time | `fn score() -> int { log("debug"); return 5; }` |
| `impure_coupled` | Value depends on side effects (return reads state the body just wrote) | `fn alloc_id() -> int { self.next_id += 1; return self.next_id; }` |
| `impure_only` | Has side effects, no meaningful return (or return type is `()`) | `fn apply_mods() { ... }` |

Inferred automatically from the body via data-flow analysis: does the return expression read state that the body writes? If yes → coupled. If no → hoistable.

#### 9.4.2 Function decomposition at codegen

Every callable function can be decomposed into up to TWO Clausewitz constructs:

- `<fn>__value` — a `script_value` (or `scripted_trigger` for `-> bool`) returning the computation result. Always emitted for pure / hoistable / deferred / coupled functions.
- `<fn>__effect` — a `scripted_effect` running the side effects. Emitted only for hoistable / deferred / coupled / impure_only.

Call sites pick emission based on context:

**Pure call**: emit just the value reference. Valid in every context.
```
# caller: if score() > 3 { foo }
if = { limit = { check_variable = { which = score__value value > 3 } } foo = yes }
```

**Hoistable call in effect context**: run effect first, use value.
```
# caller (immediate block):
#   if score() > 3 { foo }
immediate = {
    score__effect = yes
    if = { limit = { check_variable = { which = score__value value > 3 } } foo = yes }
}
```

**Hoistable call in trigger context**: auto-hoist effect to nearest outer effect scope.
```
# caller (standalone trigger):
#   limit = { score() > 3 }     <-- no outer effect scope
```
Three sub-cases (see 9.4.3).

**Deferred call**: enqueue at call site (cheap variable write); separate drain handler runs the effect batched (§32.6).

**Coupled call**: only valid in effect contexts (value depends on effect). Trigger-context calls surface as compile errors with a suggested refactor.

#### 9.4.3 Auto-hoisting rules

For `impure_hoistable` calls in trigger contexts, the compiler walks up the IR scope stack to find the nearest enclosing effect-context block, then inserts `<fn>__effect = yes` at the last effect-valid position before the current expression. The trigger-context expression then references only `<fn>__value`.

```rust
immediate = {
    bar();
    if score() > 3 {        // score is impure_hoistable
        foo();
    }
}
```

→ emitted:
```
immediate = {
    bar = yes
    score__effect = yes          # auto-hoisted: just before first use
    if = { limit = { check_variable = { which = score__value value > 3 } } foo = yes }
}
```

Order preserved (bar → score's effect → if-check → foo). Deduped within a single expression (`score() > 3 && score() < 10` hoists once).

#### 9.4.4 Hoist-impossible contexts (the only compile errors)

Three contexts have NO outer effect scope the compiler can hoist into. Impure calls in these surface as compile errors:

1. **Inside a `scripted_trigger` body** — the author is writing a pure predicate by contract.
2. **Inside `mean_time_to_happen` modifier blocks** — engine evaluates periodically with no effect context.
3. **Inside event `trigger` fields** — evaluated before `immediate`; side effects wouldn't sequence correctly.

Error example:
```
error: impure function `score()` called in pure context
  site: heritage/bloodline.cse:42:15
  context: mean_time_to_happen modifier
  reason: `score()` writes to self.counter; cannot hoist (no effect scope available)
  fix: make `score()` pure by factoring the mutation into a separate fn,
       OR precompute the value in an on_action hook and read it here.
```

These errors are CORRECT outcomes — code that got past them would be broken anyway.

#### 9.4.5 Manifest contract

Per-function manifest entry under `clause-manifest/purity_registry.manifest`:
```yaml
methods:
  heritage::bloodline::score:
    signature: "fn() -> int"
    purity: pure
    since: "0.1.0"
  heritage::bloodline::alloc_id:
    signature: "fn() -> int"
    purity: impure_coupled
    coupled_reason: "return reads self.next_id which body writes"
    since: "0.1.0"
```

**Purity transitions as breaking changes**:
- `pure → impure_hoistable` (author added a `log()`): auto-hoisting handles most callers silently. Only the three hoist-impossible contexts break → compile errors at THOSE specific caller sites, with clear messages.
- `pure → impure_coupled`: breaks ALL trigger-context callers → compile errors at each.
- `impure → pure`: no breaking change. All existing callers keep working; trigger-context becomes newly available.
- Any transition: manifest auto-updates. Author may add explicit `#[purity_change(from = "pure", to = "impure_hoistable", since = "0.X.Y")]` to make the change visible in code review; optional, not required.

No `#[kind_change]`. No kind_registry.manifest. The old v2 machinery is gone.

#### 9.4.6 Override hint attributes (rare)

For edge cases where the author wants to force codegen:
- `#[as_inline_script]` — force emit as inline_script (RAM-efficient for small helpers).
- `#[prefer(...)]` — non-binding emit hint (tiebreaker).

Authors don't normally need these. The compiler's classifier + decomposer handles it.

### 9.5 Sealed traits

Sealed traits restrict impls to the declaring crate — downstream crates cannot add new impls. Clause supports two forms:

**Direct: the `sealed` keyword** (preferred when you're shipping the trait):

```rust
pub sealed trait Bind {}
// Only the current crate may `impl Bind for X`. Attempting the impl
// from a downstream crate emits CL_SEALED_VIOLATION.
```

**Indirect: the private-supertrait pattern** (useful when the sealed trait re-exports another trait's surface):

```rust
mod private {
    pub trait Sealed {}
}

pub trait CoreScope: private::Sealed {
    // Only impls within this crate can satisfy `private::Sealed`,
    // so no downstream mod can implement CoreScope.
}
```

Both forms are enforced — the first directly via the `sealed` modifier, the second via supertrait coherence (§9.3). Used for system traits that mods must not extend independently, including `std::bind::Bind` (the anchor for every storage-router target).

### 9.6 No `dyn Trait`

No runtime trait objects. Clausewitz can't dispatch dynamically; the engine doesn't have vtables. Rejecting `dyn Trait` simplifies the type checker and removes an entire class of design constraints (no object-safety rules, no Self-return-type restrictions, etc.).

---

## 10. Impls

For struct-bound types, methods go in an inherent `impl StructName { ... }` block. For trait satisfaction, `impl Trait for StructName { ... }`.

```rust
struct Bloodline {  // implicit: Registry<Self>
    const MAX_SIZE: int = 100,
    mut size: int = 0,
    mut last_index: int = 0,
    mut member_vector: Array<PopId, MAX_SIZE>,
}

// Inherent impl — methods directly on Bloodline.
impl Bloodline {
    fn contains(who: Pop) -> bool {
        any_pop = {
            limit = { is_same_value = $who$ }
            heritage_in_bloodline = yes
        }
    }

    fn add_member(who: Pop) -> Result<int, CapacityFull> {
        if self.size >= MAX_SIZE {
            return Err(CapacityFull);
        }
        let new_idx = self.size;
        self.last_index = new_idx;
        self.size = self.size + 1;
        return Ok(new_idx);
    }

    fn matriarch() -> Option<Pop> {
        if self.size > 0 {
            return Some(self.member_vector[0]);
        } else {
            return None;
        }
    }
}

// Trait impl — Bloodline satisfies a contract defined elsewhere.
impl Family for Bloodline {
    type Member = Pop;
    fn add_member(who: Pop) -> Result<int, CapacityFull> { ... }
}
```

### 10.1 Coherence (the orphan rule)

`impl Trait for Type` is legal only if your crate declared either `Trait` or `Type`. This is the mechanism that prevents two patch mods from silently impl-ing `Patchable for vanilla::Country` with conflicting bodies.

When consolidation IS required (a third crate that owns neither the trait nor the type wants to reconcile multiple upstream impls), the explicit `#[patch(target = "...", reason = "...")]` decorator marks the impl as an intentional override. Multiple un-marked impls of the same `(Trait, Type)` pair → compile error.

### 10.2 Method body conventions

- Bodies look like Clausewitz, with type-checking happening invisibly.
- `self.field` / `Self::CONST` access struct state; compiler expands to auto-generated names.
- `let` introduces immutable local bindings (compile-only sugar; eliminated at codegen unless a scratch variable is required).
- `let mut` introduces mutable locals (emits a scratch `_temp_*` variable when needed).
- `return X` emits a "return via convention" — caller reads a documented variable after invocation.
- Method-call sugar (`self.bloodline.add_member(who = my_pop)`) desugars to the underlying Clausewitz call (`heritage_bloodline_add_member = { who = my_pop }`).

### 10.3 References — erased ergonomic sugar

Clause has no ownership/borrow tracking: lifetimes can't be inferred reliably from Clausewitz semantics (when does the engine copy vs alias?). Rather than ship a fragile lifetime fiction, the compiler treats every reference as equivalent to its pointee. All values pass by value semantically; the compiler emits whatever Clausewitz convention is correct for the target construct.

Authors can still *write* `&self`, `&mut self`, `&T`, `&mut T`, `&expr`, `&mut expr` — the parser accepts every form and the AST records the `mutable` flag, but every layer downstream erases references to their inner target. This is purely an ergonomic accommodation for readers coming from Rust; it never produces runtime difference. In particular:

- `fn mark(&self)` is equivalent to `fn mark(self)`.
- `fn append(&mut self, item: T)` is equivalent to `fn append(mut self, item: T)`.
- `f(&x)` lowers exactly the same as `f(x)`.
- `&T` and `T` resolve to the same `ResolvedType`.

Double references need spacing (`& &T`) because the lexer tokenises `&&` as `AMP_AMP`. A future borrow-aware linter could warn on meaningless reference wrappers, but nothing in the compiler enforces borrow rules today.

*Historical note*: this section originally stated "No `&` references". The decision to accept references as erased sugar was made during #113; the language takes borrow forms to avoid surprising Rust-trained authors, not to implement them.

### 10.4 `Self` (capital S)

`Self` is type-level — refers to the impl's target type. Used in:
- Associated type references: `Self::Member`
- Event scope inheritance: `event MatriarchDies for Self` (inside an impl, expands to the impl's target type).
- Generic constraints: `where Self: Owned`.

`self` (lowercase) is the implicit receiver inside method bodies — used for field/method access only, never as a parameter.

---

## 11. Generics

First-class. Monomorphized at codegen.

```rust
struct Modifier<T>(name: string)         // T is the resource/aspect tag

trait HasModifier<T> {
    fn add_modifier(modifier: Modifier<T>, duration: int);
    fn has_modifier(modifier: Modifier<T>) -> bool;
    fn remove_modifier(modifier: Modifier<T>);
}

// Marker types — zero-sized, used purely for type-tagging
struct Unity;
struct Energy;
struct Minerals;

extern impl HasModifier<Unity> for Country;
extern impl HasModifier<Energy> for Country;
extern impl HasModifier<Minerals> for Country;
```

Calls type-check at the bound site:
```rust
let stability_boost: Modifier<Unity> = Modifier("stability_boost");
country.add_modifier(stability_boost, 360);   // ok
country.add_modifier(some_energy_modifier, 360);  // ok if Modifier<Energy>
country.add_modifier(my_string, 360);              // type error
```

### 11.1 Generic functions and impls

```rust
fn process<T: HasFleets + HasResources>(thing: T) -> int { ... }

// Multiple bounds via `where`
fn complex<T>(x: T)
where
    T: Bloodline,
    T::Member: HasGenes + HasMemories,
{ ... }
```

*Parsing status*: every generic-bearing item (fn, struct, enum, trait, impl, macro) parses both the `<T: Bound + Bound>` and `where T: Bound` forms. Bounds are retained on the AST (#115). Full enforcement — rejecting instantiations that violate the bounds — rides with the follow-up #160 G7b; for v1 the typecheck records them without acting on them, which doesn't impede compilation but also doesn't catch bound violations.

### 11.2 Const generics

```rust
struct Array<T, const N: int> { ... }
struct LineageDepthCheck<const MAX: int> { ... }
```

For things like depth limits, max chain lengths, fixed-size storage — all natural Stellaris use cases.

*Parsing status*: `<T, const N: Type>` parses cleanly (#164). Full enforcement — verifying that every instantiation passes a compile-time constant matching the declared type — rides with T3 monomorphization (#121).

### 11.3 No higher-kinded types

Clause does NOT support HKT (e.g., `trait Functor<F<_>>`). Stellaris models nothing that needs them.

---

## 12. Option, Result, no nulls

No nullability, no nulls. Use `Option<T>` and `Result<T, E>` instead.

```rust
fn matriarch() -> Option<Pop> {
    if self.size > 0 {
        return Some(self.member_vector[0]);
    } else {
        return None;
    }
}

fn add_member(who: Pop) -> Result<int, CapacityFull> {
    if self.size >= MAX_SIZE {
        return Err(CapacityFull);
    }
    // ... proceed ...
    return Ok(new_idx);
}
```

### 12.1 Codegen for Option / Result (per-type manifest ABI)

Compiler picks emit strategy based on inner type:
- **Numeric inner (Pop, int, pop_id)**: sentinel value (`-1` by default; per-type overridable) for None / Err. Compact, no extra variable.
- **Non-numeric or ambiguous**: paired flag + value. `_present` / `_value` for Option; `_ok` / `_value` / `_error_kind` enum tag for Result.

**Critical: the encoding choice is a CROSS-CRATE ABI. Stored in manifest, stable per type.**

Without manifest-tracking, the codegen strategy silently becomes cross-crate incompatible:
```
// In Crate A (heritage): Option<PopId> → sentinel -1 (compact)
// fn get_leader_pop() -> Option<PopId> { ... emits @leader_pop_id = -1 for None ... }

// In Crate B (downstream consumer of heritage): reads the result,
// expects flag+value encoding because B's codegen picks a different
// strategy for the same type.
// → B reads @leader_pop_id_present (doesn't exist) → always gets 0 → always None.
// Both crates compile cleanly; runtime behavior is silently wrong.
```

**Mechanism**:
- Every `Option<T>` / `Result<T, E>` type-instantiation that crosses a crate boundary (via a `pub` method signature, a `pub` field, or a re-exported type) gets a manifest entry under `clause-manifest/encoding_registry.manifest`:
  ```yaml
  - type: Option<PopId>
    encoding: sentinel
    sentinel_value: -1
    since: "0.1.0"
  - type: Option<Bloodline>
    encoding: flag_plus_value
    flag_suffix: _present
    value_suffix: _value
    since: "0.1.0"
  ```
- On every build, the compiler verifies the selected encoding matches the manifest. Mismatch → compile error unless `#[encoding_change(from = "...", to = "...", since = "...")]` is present.
- Private (non-`pub`) uses aren't manifest-tracked — internal encoding choices are always safe to change.

Sentinel collisions (the chosen sentinel value is a legal value of T) are detected at manifest-write time; the compiler picks an unused sentinel or falls back to flag+value automatically if no safe sentinel exists. Either way, the choice is manifest-locked once made.

### 12.2 Pattern matching

```rust
match self.bloodline.add_member(new_pop) {
    Ok(idx) => log("Added at index {}", idx),
    Err(CapacityFull) => log("Bloodline full"),
}

match self.bloodline.matriarch() {
    Some(pop) => log("Matriarch is pop {}", pop.id()),
    None => log("No matriarch yet"),
}
```

Supported pattern shapes: literal, ident, wildcard (`_`), path (`Enum::Variant`), tuple struct (`Ok(x)`), tuple (`(a, b)`), range (`0..=9`), or (`A | B`), rest (`..` inside a tuple or struct), and reference (`&p` / `&mut p` — erased at transpile, same as expressions).

Pattern matching desugars to Clausewitz `if`/`else` chains keyed on the present/ok flag (or sentinel comparison). Exhaustiveness is checked at compile time; or-patterns correctly contribute each alternative's coverage (#161 slice). Full pattern-matrix exhaustiveness for range + struct patterns is the remainder of #161.

### 12.3 The `?` operator

Standard Rust early-return for Result:

```rust
fn complex_op() -> Result<int, SomeError> {
    let idx = self.bloodline.add_member(pop)?;   // returns Err early if needed
    let other = self.do_other_thing()?;
    return Ok(idx + other);
}
```

*Parsing status*: the `?` operator parses as `QuestionExpr`. Desugaring to match-on-variant + early-return rides with #156 once match lowering (#125) lands.

---

## 13. Extern (vanilla declarations)

Engine-provided types and contracts are declared `extern` — interface only, no codegen. Like TypeScript `.d.ts` or Haxe `extern`.

```rust
// stdlib/vanilla/scopes.cse

extern scope Country;
extern scope Pop;
extern scope Planet;
extern scope Fleet;
extern scope Leader;
// ... rest of vanilla scope set ...
```

`extern scope X` declares "X is an engine-provided scope. No fields. You may `impl Trait for X` to attach behavior, but you cannot instantiate X — the engine does that."

```rust
// stdlib/vanilla/country.cse

extern trait CountryActions {
    #[as_scripted_effect] fn create_pop(species: Species) -> Pop;
    #[as_scripted_effect] fn add_modifier<T>(modifier: Modifier<T>, duration: int);
    #[as_scripted_trigger] fn has_ethic(ethic: Ethic) -> bool;
    #[as_script_value] fn total_resources() -> int;
    // ... etc.
}

extern impl CountryActions for Country;   // contract declaration; engine implements
```

`extern impl T for U` = "the engine provides this implementation; trust the contract, don't codegen, just type-check call sites."

**Critical: every extern method carries an explicit `#[as_X]` kind marker.**

Without this, Clause's kind-inference (§9.4) can't classify extern calls — every `country.add_modifier(...)` invocation would trigger "ambiguous emit" because the body is empty (we didn't write it; engine did). The kind tag closes this:
- `#[as_scripted_effect]` — side-effecting vanilla effects (`add_modifier`, `create_pop`, `give_technology`).
- `#[as_scripted_trigger]` — boolean vanilla triggers (`has_ethic`, `is_country_type`).
- `#[as_script_value]` — value-returning vanilla values (`total_resources`, `num_pops`).
- `#[as_inline_script]` — rare; reserved for inline_script-shaped vanilla helpers.

The `stellaris-vanilla-spec` ingest pipeline (§25) auto-populates these tags from `trigger_docs` output — logs explicitly distinguish effect vs trigger vs value, so no hand-annotation is needed for the generated declarations. Hand-curated overlays use the same `#[as_X]` tags for consistency.

### 13.1 Bootstrap source

The vanilla declarations are auto-generated from two sources:

1. **`cwtools-stellaris-config`** (community-maintained CWT schemas) for the type/scope/trigger/effect baseline.
2. **Stellaris' own `script_documentation/` logs** (via the `trigger_docs` debug command) for authoritative current-version surfaces. Re-parsed on every Stellaris update.

Hand-curated overlays for the ~30 most-used scope types where ergonomics matter; auto-derived for the long tail.

### 13.2 Mod-provided externs

Other mods (NSC3, ESC, etc.) ship their own `.cse` extern declarations as overlay packages. Authors `use nsc3::*` to bring them into scope. Conditional compilation handles "only if NSC3 is in modlist" cases (see chunk 4).

---

## 14. Stdlib outline

The Clause stdlib provides the abstractions authors compose against.

### 14.0 Bind targets (the `Bind` sealed trait)

```rust
// stdlib/bind.cse

mod private {
    pub trait Sealed {}
}

/// Anything that can host a struct's data. Sealed — only stdlib + the
/// fixed set of bind-target types impl this.
pub sealed trait Bind: private::Sealed {}

// Vanilla scopes (auto-generated impls; see stdlib/vanilla/scopes.cse)
impl private::Sealed for Country {}
impl Bind for Country {}
// ... and every other vanilla scope ...

// Stdlib bind targets
struct Registry<T> { /* opaque, compiler-managed hidden engine entity */ }
impl<T> private::Sealed for Registry<T> {}
impl<T> Bind for Registry<T> {}

struct Singleton { /* opaque, single hidden global entity */ }
impl private::Sealed for Singleton {}
impl Bind for Singleton {}

struct KeyedBy<K> { /* registry keying parameter */ }
struct Capacity<const N: int> { /* registry capacity parameter */ }
```

The compiler picks the optimal engine workaround per bind target (hidden country, fleet-name slots, paired variable arrays, etc.). All implementation detail. Tenet 11.

### 14.0.1 Per-field storage routing (`StorageRouter` pass) — #57 M13

A dedicated `StorageRouter` pass runs post-typecheck / pre-transpile.
For each field of a struct bound to a `Bind` target, the router picks
the optimal Clausewitz primitive based on field type + target kind:

| Backend | Shape | Picked when |
|---|---|---|
| `flag` | marker | field is `Marker`, unit, or `bool` (flags are cheaper; engine-idiomatic) |
| `variable` | i32/f32 | numeric field on an entity target |
| `scripted_variable` | global numeric | numeric field on `Singleton` / global `Registry<Self>` |
| `event_target` | scope ref | field is a scope type |
| `scripted_list` | collection of refs | field is `Vec<Scope>` |
| `StringStorage` | string | field is `String` (routes through stdlib hack per §8.3) |
| paired-flag-plus-value | wrapped | `Option<T>` / `Result<T,E>` (per §9.4) |

Authors override with `#[repr(backend)]` on individual fields. Struct-
level `#[repr(...)]` is rejected — the bind target is the colon
syntax. Invalid combinations (`#[repr(flag)]` on an `i32`) produce
`CL_REPR_MISMATCH`. Ambiguous picks emit `CL_REPR_AMBIGUOUS` as a
suggestion, never a hard error.

Decisions are locked in the crate manifest under `[storage]`, keyed
by `crate::module::Struct::field`. Code changes that would force a
different pick surface as `CL_STORAGE_DRIFT`; authors opt into a
re-pick via `#[migrate(...)]` (deferred, separate task).

See `docs/clause/briefs/storage_backend.md` for the full brief.

### 14.1 Collections

```rust
/// Fixed-size storage allocated at struct-declaration time.
/// Compiles to numbered slot variables (var_1 .. var_N).
struct Array<T, const N: int> { /* opaque */ }

impl<T, const N: int> Indexable<T> for Array<T, N> { ... }
impl<T, const N: int> Iterable<T> for Array<T, N> { ... }

/// Dynamic-size sugar over Array + length counter.
struct Vec<T, const CAP: int> { storage: Array<T, CAP>, mut len: int = 0 }

impl<T, const CAP: int> Vec<T, CAP> {
    fn push(item: T) -> Result<(), CapacityFull>;
    fn pop() -> Option<T>;
    fn len() -> int;
    fn clear();
    // ...
}

/// Two paired arrays; linear-scan lookup.
struct HashMap<K, V, const CAP: int> { /* keys + values */ }

impl<K, V, const CAP: int> HashMap<K, V, CAP> {
    fn insert(key: K, value: V) -> Option<V>;   // returns previous if any
    fn get(key: K) -> Option<V>;
    fn remove(key: K) -> Option<V>;
    fn contains_key(key: K) -> bool;
    // ...
}
```

### 14.2 Iteration traits

```rust
trait Iterable<T> {
    fn iter() -> Iter<T>;
}

trait Indexable<T> {
    fn at(idx: int) -> Option<T>;
}

/// An iterator. Maps to engine `every_*` / `any_*` constructs at codegen.
struct Iter<T> { /* opaque */ }

impl<T> Iter<T> {
    fn next() -> Option<T>;
    fn count() -> int;       // → engine count_*
    fn any(pred: ...) -> bool;
    fn every(pred: ...) -> bool;
    // ...
}
```

`for x in iter` desugars to `every_*` (effect context) or `any_*` (trigger context) depending on the surrounding inference.

**Codegen lookup table**: the `Iter<T>` type-to-engine-iterator mapping is driven by a stdlib-maintained table indexed on `(element_type, bind_scope) → engine_iterator_keyword`. Examples:

| Element type | Bind scope | Engine keyword |
|---|---|---|
| `Pop` | `Country` | `every_owned_pop` / `any_owned_pop` / `count_owned_pop` |
| `Pop` | `Planet` | `every_pop` / `any_pop` / `count_pop` |
| `Planet` | `Country` | `every_owned_planet` / `any_owned_planet` / `count_owned_planet` |
| `Fleet` | `Country` | `every_owned_fleet` / `any_owned_fleet` / `count_owned_fleet` |
| `Leader` | `Country` | `every_owned_leader` / `any_owned_leader` / `count_owned_leader` |
| ... | ... | ... (generated from `stellaris-vanilla-spec` ingest) |

The table is part of the vanilla spec (§25); ingest harvests it from `trigger_docs` output (which enumerates every_*/any_*/count_* forms per scope). A missing entry → compile error "no iterator keyword registered for `Iter<X>` on `Y` scope" with a suggestion to check the vanilla-spec ingest.

### 14.3 Numeric/ID newtypes

```rust
struct PopId(int);
struct CountryId(int);
struct LeaderId(int);
// ...
```

Newtypes prevent mixing IDs from different scope kinds. `PopId` and `CountryId` are nominally distinct.

### 14.4 Modifiers, durations, etc.

```rust
struct Modifier<T>(name: string)
struct TimedModifier<T>(modifier: Modifier<T>, duration: int)

trait HasModifier<T> {
    fn add_modifier(modifier: Modifier<T>, duration: int);
    fn has_modifier(modifier: Modifier<T>) -> bool;
    fn remove_modifier(modifier: Modifier<T>);
}
```

### 14.5 Lowercase `array` literal

`array[1, 2, 3]` is the compile-time array-literal expression. Type matches `Array<int, 3>`. Used for inline initializers; not the storage type itself.

```rust
let weights = array[10, 20, 30, 40];    // Array<int, 4>
```

### 14.6 Memoization — `Cached<T>` with refresh interval

Authors who want caching wrap a value in `Cached<T>` and pick a refresh interval. The compiler emits the recompute hook on the appropriate `on_action` (yearly_pulse, monthly_pulse, etc.).

```rust
struct Bloodline {
    mut size: int = 0,
    cached_lineage_depth: Cached<int, refresh = "monthly">,
}

impl Bloodline {
    fn lineage_depth() -> int {
        self.cached_lineage_depth.get_or_compute(|| {
            // expensive walk — runs at most once per month per instance
        })
    }

    /// Manual invalidation when state changes that the refresh interval misses.
    fn add_member(who: Pop) -> Result<int, CapacityFull> {
        // ...
        self.cached_lineage_depth.invalidate();
        return Ok(new_idx);
    }
}
```

Refresh values: `"daily"`, `"monthly"`, `"yearly"`, `"on_action(<name>)"`, `"never"` (manual-only).

No language-level auto-tracking of read dependencies in v1. Authors who need cross-field invalidation call `.invalidate()` explicitly. Compiler may add `#[cached(auto_invalidate)]` later when usage patterns reveal the right shape.

**Multi-file emit for `Cached<T>`**: the struct field is routed to the bind target.s scripted_effects/scripted_values tree, but the refresh hook must be emitted to `common/on_actions/auto/<crate>.txt` (a different output directory). Codegen (§24.5) splits the emit-plan tuples across output paths; the field-side and the on_action-side share metadata marking them as a single logical unit so future optimization passes can coalesce refresh hooks from multiple `Cached<T>` fields into fewer on_action invocations.

**Relationship to `#[deferred]`**: `Cached<T>` is a specific form of deferred computation — recompute-at-cadence semantics with the result cached between runs. The general `#[deferred]` mechanism (§32.6) handles queue/latest/accumulate drain strategies for ANY function. `Cached<T>` is the convenient API for the specific "memoize this value, refresh periodically" pattern.

### 14.7 Emit-strategy hints — `#[prefer(...)]`

Like Rust's `#[inline]` / `#[inline(always)]` / `#[inline(never)]`. Authors and stdlib hint the compiler which Clausewitz emit kind to favor. Compiler treats as tiebreaker, not command — may override with a warning if the body is logically incompatible.

```rust
#[prefer(inline_script)]    // small, frequently called → inline_script saves RAM
fn small_helper(target: int) -> int { ... }

#[prefer(scripted_effect)]  // force scripted_effect even when compiler would pick inline_script
fn rare_complex_op() { ... }

#[prefer(script_value)]     // pure value computation
fn diplomatic_score() -> int { ... }
```

Stdlib uses these aggressively for RAM-frugal emit (Stellaris parameterized scripted_effects compile to ~30 conditional branches + ~2MB RAM each; inline_scripts have no per-call overhead). Mod authors get the same surface for ergonomic optimization control.

Compiler default (when no `#[prefer]` hint):
- Body uses only trigger constructs + returns `bool` → `scripted_trigger`
- Body has effect constructs OR mutates `mut self.field` → `scripted_effect`
- Body is pure value computation → `script_value`
- Small body + many call sites + no @-var substitution needed → `inline_script` (promotion)
- Mixed → type error: "ambiguous emit; add `#[prefer(...)]`"

---

## 15. Events

Events are top-level (or impl-internal) items declared with the `event` keyword. PascalCase names. Scope via `for X` clause.

```rust
// Free-standing event — explicit `for` clause
event NewHeir for Country {
    is_triggered_only = true;
    immediate = {
        let new_pop = create_pop(species = root.species);
        match self.bloodline.add_member(new_pop) {
            Ok(idx) => log("Added at index {}", idx),
            Err(CapacityFull) => log("Bloodline full"),
        }
    }
}

// Inside an impl block — `for Self` resolves to the struct's bind target.
//
// For `struct Bloodline: Country`, Self in event scope = Country.
// For `struct Bloodline: Registry<Self>`, Self isn't a fireable engine
// scope — events inside such an impl must declare `for X` explicitly,
// where X is whatever scope the event should fire in (often the
// registry-key bind target, e.g. `for Country` if KeyedBy<CountryId>).
impl Bloodline {
    event MatriarchDies for Country {     // explicit; Bloodline lives in a registry, not a scope
        immediate = {
            // body uses Country scope
        }
    }
}
```

### 15.1 Event ID generation

Compiler auto-allocates IDs from the manifest (§16). Author writes `event NewHeir`; compiler emits:
```
country_event = {
    id = heritage.dynasty.new_heir.0001
    ...
}
```

Namespace derived from module path (`heritage::events::dynasty` → `heritage.dynasty`). Number allocated on first compile, frozen in manifest.

### 15.2 Auto-fields

Many event fields (title, desc, picture) derive automatically from the event name + module path:
- `title = auto` (or omitted entirely) → `heritage.dynasty.new_heir.t`
- `desc = auto` → `heritage.dynasty.new_heir.d`
- `picture = auto` → `GFX_evt_heritage_dynasty_new_heir`

Override by setting explicitly:
```rust
event NewHeir for Country {
    title = "heritage.dynasty.special_heir.t";   // explicit override
    is_triggered_only = true;
    immediate = { ... }
}
```

### 15.3 Scope-less events

Stellaris has scope-less hooks (`on_game_start`, certain `on_action` blocks). Declare without a `for` clause:
```rust
event GameStart {
    // body uses only globally-valid constructs
}
```

Compiler validates the body uses no scope-requiring constructs.

---

## 16. Manifests

Five manifest files, all fully compiler-managed (never hand-edited), all reviewable in git diff. Each lives under `clause-manifest/` at the crate root.

| File | Tracks | Primary purpose |
|---|---|---|
| `event_ids.manifest` | Namespace-allocated event IDs | Stable event IDs across renames for save compat |
| `symbol_aliases.manifest` | Renamed scripted_effects, scripted_triggers, etc. | Save-compat sham handlers |
| `purity_registry.manifest` | Per-function purity classification | Detect purity transitions, surface breaking changes at impacted callers |
| `encoding_registry.manifest` | Per-type Option/Result emit strategy (sentinel vs flag+value) | Cross-crate ABI stability |
| `patch_registry.manifest` | Outbound `#[patch]` / `#[patch_extend]` / `#[patch_body]` declarations | Cross-workspace patch collision detection at downstream build |

### 16.1 Schema: `clause-manifest/event_ids.manifest`

```yaml
format_version: 1

# Event ID allocations. Stable across builds per git-tagged versions.
namespaces:
  heritage.dynasty:
    new_heir:          { id: 0001, since: "0.1.0" }
    heir_died:         { id: 0002, since: "0.1.0" }
    bloodline_extinct: { id: 0003, since: "0.2.0" }
  heritage.marriage:
    wedding:           { id: 0001, since: "0.1.0" }

# Aliases for renamed/removed events
aliases:
  - deprecated:  heritage.dynasty.0099    # absent from current source
    canonical:   heritage.dynasty.0001    # the rename target
    since:       "0.2.0"
    sham:        heritage_dynasty_0099_sham   # auto-scaffolded handler
```

### 16.2 Schema: `clause-manifest/symbol_aliases.manifest`

```yaml
format_version: 1

aliases:
  - kind:        variable
    deprecated:  heritage_bloodline_count
    canonical:   heritage_bloodline_size
    since:       "0.3.0"
    sham:        heritage_migrate_bloodline_count_to_size

  - kind:        scripted_effect
    deprecated:  heritage_dynasty_old_birth
    canonical:   heritage_dynasty_birth
    since:       "0.4.0"
    sham:        heritage_dynasty_old_birth_sham
```

### 16.3 Update workflow

1. Compiler runs on every build.
2. On first build after a new `git commit`, it diffs current source against the previous-tagged-version source.
3. For each removed or renamed symbol:
   - Author has `#[supersedes(...)]` or `#[deprecated(...)]` annotation → use it; write manifest entry.
   - Compiler can detect a clean rename (same shape, same body, just renamed) → write manifest entry automatically.
   - Ambiguous (shape changed materially OR multiple plausible candidates) → leave manifest alone; emit build warning prompting `#[supersedes]` annotation.
4. Manifests are written, committed alongside the source change. Reviewable in PR.

This is git-driven: dev-time renames don't churn the manifest. Only commit-tagged renames produce manifest entries on the next build.

### 16.4 Sham handlers

Each alias entry generates a scripted_effect that:
1. Detects the deprecated state at save load (variable exists but new doesn't, etc.)
2. Migrates the state to the new format
3. Marks itself complete via a flag
4. Optionally invokes the canonical replacement

Sham handlers wire into `on_single_player_save_game_load` / `on_multiplayer_game_loaded` automatically.

**Auto-scaffold**: compiler writes the trivial-rename sham case (variable copy) by default. Author can extend the body for non-trivial migrations. The scaffolded sham + author extensions live in a generated `clause-manifest/shams/` directory, referenced from the manifest entry's `sham:` field.

---

## 17. Aliases & deprecation attributes

Two attributes for explicit author guidance on renames:

### 17.1 `#[supersedes("old_name", since = "0.X.Y")]`

Applied to the NEW symbol after a rename. Tells the compiler "this is the rename target for `old_name` as of version 0.X.Y."

```rust
#[supersedes("count", since = "0.2.0")]
mut size: int = 0,
```

The compiler will:
- Write a manifest entry mapping `count` → `size`.
- Generate a sham handler that copies `@heritage_bloodline_count` → `@heritage_bloodline_size` on save load.

### 17.2 `#[deprecated(use_instead = "new_name", since = "0.X.Y", removed_in = "1.0.0")]`

Applied to the OLD symbol when keeping it around for a transition period:

```rust
#[deprecated(use_instead = "size", since = "0.1.0", removed_in = "1.0.0")]
mut count: int = 0,

mut size: int = 0,
```

Both `count` and `size` exist in the build for the transition window. Calls to `count` produce build warnings. After `removed_in` version, `count` is dropped from the build.

### 17.3 Ambiguous renames

When the compiler can't auto-detect a rename (significant shape change, multiple plausible new names), it emits a build warning:
```
warning: symbol `heritage_bloodline_count` removed in this commit; no rename target detected.
help: if this was renamed, add `#[supersedes("count", since = "0.X.Y")]` to the new symbol.
help: if this was deleted, no action needed; saves referencing it will hit fallback behavior.
```

Author either adds the annotation or accepts the deletion semantics.

---

## 18. Conventions for body code

### 18.1 Method bodies

- Look like Clausewitz with type validation.
- `self.field` access expanded by compiler to auto-generated variable name.
- `let` / `let mut` introduce locals; mostly compile-only sugar.
- `return X` emits "return via convention" — caller reads a documented variable.

### 18.2 Method call sugar

`self.bloodline.add_member(who = new_pop)` desugars to `heritage_bloodline_add_member = { who = new_pop }`. The Clausewitz form is also valid (authors can write either; converter mode reformats for consistency).

### 18.3 Control flow sugar

| Clause | Emitted Clausewitz |
|---|---|
| `if foo() { … }` (bool fn call) | `if = { limit = { foo = yes } … }` |
| `if cond { a } else { b }` | `if = { limit = { cond } a } else = { b }` |
| `match val { Some(x) => …, None => … }` | `if`/`else` keyed on Option flag |
| `for x in iter { … }` | `every_*` (effect ctx) or `any_*` (trigger ctx) |
| `while cond { … }` | `while = { limit = { cond } … }` |
| `self.field += 1` | `change_variable = { which = … value = 1 }` |
| `self.field = X` | `set_variable = { which = … value = X }` |
| `self.method(arg)` | `<auto_name> = { arg }` |

### 18.4 Source mapping

Generated Clausewitz includes comments mapping back to the `.cse` source:

```
# >> heritage/bloodline.cse:42 — fn add_member()
heritage_bloodline_add_member = {
    # >> let new_idx = self.size
    # >> self.last_index = new_idx
    set_variable = {
        which = heritage_bloodline_last_index
        value = trigger:heritage_bloodline_size
    }
    # >> self.size = self.size + 1
    change_variable = {
        which = heritage_bloodline_size
        value = 1
    }
    # >> return new_idx
    # (return value: caller reads @heritage_bloodline_last_index)
}
```

Comments are inert to Stellaris; authors debugging in-game find the `.cse` source via these.

---

## 19. Worked example (everything together)

```rust
// crates/heritage/src/bloodline.cse

mod heritage::bloodline;
use vanilla::scopes::{Country, Pop};
use stdlib::collections::Array;
use stdlib::cache::Cached;
use stdlib::error::CapacityFull;

/// Tracks a bloodline's membership. Bloodline is a logical entity,
/// not a Country-scoped one — implicit bind = Registry<Self>.
struct Bloodline {
    const MAX_SIZE: int = 100,
    mut size: int = 0,
    mut last_index: int = 0,
    mut member_vector: Array<PopId, MAX_SIZE>,
    mut name: string = "Unnamed",                              // routes via StringStorage (Tenet 11)
    cached_lineage_depth: Cached<int, refresh = "monthly">,
}

impl Bloodline {
    /// True if `who` is in this bloodline.
    fn contains(who: Pop) -> bool {
        any_pop = {
            limit = { is_same_value = $who$ }
            heritage_in_bloodline = yes
        }
    }

    /// Add `who`; returns new index or CapacityFull.
    fn add_member(who: Pop) -> Result<int, CapacityFull> {
        if self.size >= MAX_SIZE {
            return Err(CapacityFull);
        }
        let new_idx = self.size;
        self.last_index = new_idx;
        self.size = self.size + 1;
        self.cached_lineage_depth.invalidate();
        return Ok(new_idx);
    }

    /// First/founder member, if any.
    fn matriarch() -> Option<Pop> {
        if self.size > 0 {
            return Some(self.member_vector[0]);
        } else {
            return None;
        }
    }

    /// Cached, recomputed monthly.
    fn lineage_depth() -> int {
        self.cached_lineage_depth.get_or_compute(|| {
            // expensive walk
        })
    }

    /// Bloodline isn't a fireable scope — events inside this impl
    /// declare `for Country` explicitly (the registry is keyed by Country).
    event MatriarchDies for Country {
        is_triggered_only = true;
        immediate = {
            // body uses Country scope
        }
    }
}
```

```rust
// crates/heritage/src/events/dynasty.cse

mod heritage::events::dynasty;
use heritage::bloodline::Bloodline;
use vanilla::scopes::Country;

event NewHeir for Country {
    is_triggered_only = true;
    immediate = {
        let new_pop = create_pop(species = root.species);
        match self.bloodline.add_member(new_pop) {
            Ok(idx) => log("Added at index {}", idx),
            Err(CapacityFull) => log("Bloodline full"),
        }
    }
}
```

Compiler emits:

```
# >> heritage/bloodline.cse:23 — fn contains()
heritage_bloodline_contains = {
    any_pop = {
        limit = { is_same_value = $who$ }
        heritage_in_bloodline = yes
    }
}

# >> heritage/bloodline.cse:31 — fn add_member()
heritage_bloodline_add_member = {
    # >> if self.size >= MAX_SIZE { return Err(CapacityFull); }
    if = {
        limit = { check_variable = { which = heritage_bloodline_size value >= 100 } }
        set_variable = { which = heritage_bloodline_add_member_ok value = 0 }
        set_variable = { which = heritage_bloodline_add_member_error_kind value = 1 }   # CapacityFull
    }
    else = {
        # >> let new_idx = self.size; self.last_index = new_idx;
        set_variable = {
            which = heritage_bloodline_last_index
            value = trigger:heritage_bloodline_size
        }
        # >> self.size = self.size + 1
        change_variable = { which = heritage_bloodline_size value = 1 }
        # >> return Ok(new_idx)
        set_variable = { which = heritage_bloodline_add_member_ok value = 1 }
        set_variable = {
            which = heritage_bloodline_add_member_value
            value = trigger:heritage_bloodline_last_index
        }
    }
}

# >> heritage/events/dynasty.cse:5 — event NewHeir
country_event = {
    id = heritage.dynasty.new_heir.0001
    title = heritage.dynasty.new_heir.t
    desc = heritage.dynasty.new_heir.d
    picture = GFX_evt_heritage_dynasty_new_heir
    is_triggered_only = yes

    immediate = {
        # >> let new_pop = create_pop(species = root.species);
        create_pop = { species = root.species }
        # >> match self.bloodline.add_member(new_pop) { ... }
        heritage_bloodline_add_member = { who = last_created_pop }
        if = {
            limit = { check_variable = { which = heritage_bloodline_add_member_ok value = 1 } }
            log = "Added at index $heritage_bloodline_add_member_value$"
        }
        else = {
            log = "Bloodline full"
        }
    }
}
```

---

## 20. Module system (deeper) — chunk 3a

### 20.1 Crate manifest (`mod.toml`)

Cargo-style. One per crate, at the crate root:

```toml
[mod]
name = "heritage"
version = "0.1.0"
description = "Dynasty + bloodlines for Stellaris"
clause_edition = "2026"

[dependencies]
stdlib = "1.0"
stellaris-vanilla = "4.0"          # auto-generated baseline (trigger_docs + cwtools-config)
ariphaos-unofficial = "1.2"        # another mod we build typed contracts against

[features]
default = ["dynasty"]
dynasty = []
intrigue = ["dynasty"]              # intrigue activates dynasty
```

### 20.2 Workspaces (multi-crate projects)

Stellar Heritage repo is naturally a workspace — Heritage (the mod) + Megapatch (the conflict-resolution crate) + possibly compat shims:

```toml
# Root mod.toml
[workspace]
members = [
    "crates/heritage",
    "crates/megapatch",
    "crates/nsc3-compat",
]
```

All workspace members share a lockfile and dependency resolution. Cross-crate deps within a workspace resolve to local paths, not published versions.

### 20.3 Cross-mod runtime contracts: `expect`/`actual`

Stellaris mods discover each other at **runtime** (modlist), not at compile time. The research recommended Kotlin's `expect`/`actual` pattern:

```rust
// heritage/src/compat/nsc3.cse

// Declare a runtime expectation. Compiler tracks the contract;
// runtime load-ordering verifies the actual impl exists.
expect fleet_command_integration: FleetCommand;
```

```rust
// nsc3-compat/src/lib.cse

actual impl FleetCommand for Country {
    // satisfies heritage::compat::nsc3::fleet_command_integration
}
```

Runtime: if NSC3 isn't in the modlist, the expected contract isn't fulfilled → Heritage emits a guard (via `on_single_player_save_game_load`) that disables affected code paths with a user-visible warning.

### 20.4 `#[cfg(mod = "X")]` vs `expect`/`actual` — use both

Complementary, not competing:

| Mechanism | Semantics |
|---|---|
| `#[cfg(mod = "X")]` | Compile-time include-if-present. Code NOT included in the output when X is absent. Zero runtime cost when X is missing. Use for: "integration paths that entirely go away without X." |
| `expect`/`actual` | Compile-time contract declaration + runtime verification. Code IS included; runtime checks contract fulfillment and guards feature paths. Use for: "integration paths that require X to function but want graceful degradation when X is missing." |

Rule of thumb: if the feature is meaningless without X → `#[cfg(mod = "X")]`. If the feature is degraded-but-still-valuable without X → `expect`/`actual` with runtime guards.

### 20.5 Use-tree syntax (copy Rust)

```rust
use vanilla::scopes::{Country, Pop, Planet};
use heritage::bloodline::{Bloodline, self};       // struct + module itself
use vanilla::scopes::Country as Empire;           // rename
use heritage::events::*;                           // glob — allowed but lint-warned
use super::neighbor_module::Thing;
use crate::root_level::Thing;
```

### 20.6 Re-exports

```rust
// src/lib.cse
pub use bloodline::Bloodline;
pub use events::dynasty::NewHeir;
```

Curates the crate's public API surface. Downstream mods `use heritage::Bloodline` (short) instead of `use heritage::bloodline::Bloodline` (full).

### 20.7 Dead module elimination

Modules reachable from `src/lib.cse` via the `mod` chain are compiled. Unreached `.cse` files on disk emit build-time warnings — catches "I added `foo.cse` but forgot to declare it" without silent include-everything.

---

## 21. Decorators — the full catalog (chunk 3b)

Small set. Bind syntax + kind inference + auto-naming cover most cases.

### 21.1 Name + output routing

| Decorator | Use |
|---|---|
| `#[name("explicit_name")]` | Override auto-generated Clausewitz name. Required when the name is a cross-mod API contract. |
| `#[file("path/to/output.txt")]` | Last-resort: force output to a specific file path. Skips auto-routing. |

### 21.2 Kind control

| Decorator | Use |
|---|---|
| `#[as_scripted_effect]` / `#[as_scripted_trigger]` / `#[as_script_value]` / `#[as_inline_script]` | Force a specific emit kind. Compile error if body is incompatible. |
| `#[prefer(scripted_effect \| scripted_trigger \| script_value \| inline_script)]` | Hint — compiler may override with a warning. |

### 21.3 Conditional compilation

| Decorator | Use |
|---|---|
| `#[cfg(dlc = "X")]` | Include only if DLC X is active in the compile env. |
| `#[cfg(mod = "X")]` | Include only if mod X is in modlist. |
| `#[cfg(feature = "X")]` | Include only if feature X is enabled in this crate. |
| `#[cfg(any(...))]` / `#[cfg(all(...))]` / `#[cfg(not(...))]` | Compound predicates. |
| `#[cfg(stellaris >= "4.0")]` | Gate on engine version. |
| `#[feature = "X"]` | Declaration form — declares that this crate *has* feature X (bare, no activation chain). Equivalent to a `[features]` entry with an empty list. Scanned by `clause features`. |

Features are otherwise declared in the crate's `Clause.toml` under `[features]`
(Cargo convention: each feature name maps to a list of sub-features or optional
deps it activates). The attribute form above is a convenience for features with
no activation chain — one-line declaration + gate at the use site, without
touching the manifest. `clause features` enumerates both sources per crate and
flags their origin.

### 21.4 Rename / deprecation

| Decorator | Use |
|---|---|
| `#[supersedes("old_name", since = "0.X.Y")]` | Marks rename target; compiler writes manifest entry. |
| `#[deprecated(use_instead = "...", since = "0.X.Y", removed_in = "1.0.0")]` | Transition-window marker. |

### 21.5 Patches (intentional overrides)

| Decorator | Use |
|---|---|
| `#[patch(target = "mod::path::Item", reason = "...")]` | Explicit override of an existing impl. Bypasses orphan rule with documented reason. |
| `#[patch_extend(target = "mod::path::Type")]` | Add new methods to an existing type. No conflict unless two extensions define the same name. |

### 21.6 Procedural macros

Clause has proc-macros only — no `macro_rules!`-style pattern matching.
A macro invocation looks like Rust: `name!(...)`, `name!{...}`, or `name![...]`.
The parser captures the inner token stream verbatim and hands it off to a
registered handler, which returns a replacement AST node (expression position)
or a list of items (item position). Expansion runs as a scheduler pass between
parse and typecheck.

**Shipped (slices 1 + 2)**:
- **Expression-position** invocations — anywhere an expression is valid.
  Handler returns an `Expr`.
- **Item-position** invocations — top-level or inside `impl` blocks:
  `foo!(...);`, `foo![...];`, or `foo!{ ... }` (braces self-terminate,
  no `;`). Handler returns a `list[Item]` that's spliced into the
  enclosing module's item list.
- **Eager arg-position expansion** — nested macros inside argument
  tokens are expanded before the outer handler sees them, letting
  literal-returning macros compose: `concat!(stringify!(x), "_y")`
  works. Handlers can opt out with `eager_args=False` when they need
  verbatim source (as `stringify!` does).
- Handlers are Python callables registered via `register_macro(name)`
  or `register_item_macro(name)`. A name may appear in both registries
  — the expander dispatches by invocation position.
- Built-ins:
  - `stringify!(tokens)` (raw / eager_args=False) — emits the source
    text of the input tokens as a string literal.
  - `concat!("a", "b", ...)` — concatenates string-literal arguments
    into a single string.
- Hygiene is free via STRICT1 no-shadow — any collision between a
  macro-emitted binding and a caller-visible name surfaces as
  `CL_SHADOW`. Handlers should prefix internal bindings with `__`.
- Every emitted node carries the CALL-SITE span so diagnostics point at
  the macro invocation rather than the macro body.
- Diagnostics: `CL_MACRO_UNKNOWN` (name not registered for this position),
  `CL_MACRO_POSITION` (used at wrong position — e.g. expression macro
  as item), `CL_MACRO_PATH` (qualified paths rejected in v1), plus any
  handler-raised `MacroExpansionError`.

**Slice 3 (shipped — see `docs/clause/briefs/macros_slice3.md`)**:
- User-authored `[vis] macro NAME(PARAMS) -> TokenStream { BODY }`
  declarations. No `!` on the declaration; `!` is the invocation
  marker only. Return type mandatory.
- Body is **normal Clause code** — `let`, `for`, `if`, function/method
  calls. No splice DSL, no template keywords. Tree-walking interpreter
  in Python (`grammar/clause/macro_interpreter.py`) evaluates the IR
  at macro-expansion time.
- Macro-runtime types (`TokenStream`, `Ident`, `Literal`) and the
  `quote! { ... }` macro (with `$ident` / `$(expr)` interpolation)
  live in a new `macros` crate (part of the std ecosystem, separate
  from `std`).
- Visibility fully unified (`pub macro`, `pub(crate) macro`,
  `pub(super) macro`, private) — same rules as every other item kind.
- Qualified macro paths still rejected in slice 3; lands when the
  module-resolver learns about macro bindings (post-slice 3).
- Cross-target IL variant (Clause → Python transpile) tracked as
  #101; not part of slice 3.

**Deferred (slice 4)**:
- Write-side Clausewitz-token macros that run at codegen.
- First-class scheduler-pass registration (currently expansion runs in a
  linear phase).

### 21.7 Event modifiers

| Decorator | Use |
|---|---|
| `#[hidden]` | Hidden event (`hide_window = yes`). |
| `#[on_action("yearly_pulse")]` | Tie event firing to an on_action hook. |
| `#[situation]` | Situation event (emits `situation_event` block). |

### 21.7 Auto-generation

| Decorator | Use |
|---|---|
| `#[derive(HasFleets, HasResources)]` | Auto-generate trait impls from struct metadata. Proc-macro-style; each derivable trait registers a generator in the compiler. |

### 21.8 `expect` / `actual` are keywords, not decorators

Because they're declaration-level, they get keyword status alongside `mod`, `struct`, `trait`, etc. Not part of the decorator catalog.

---

## 22. File routing (chunk 3c)

### 22.1 Default layout per crate

The compiler routes emitted items based on **crate name + module path + item kind**. Authors never specify paths; the auto-layout handles everything except `#[file(...)]` escape-hatch cases.

```
<output>/
  <crate_name>/                           # top-level dir
    common/
      scripted_effects/auto/              # compiler-allocated files
      scripted_triggers/auto/
      script_values/auto/
      inline_scripts/auto/
      buildings/auto/
      traits/auto/
      on_actions/auto/
      # ... (all vanilla common/ subdirs as needed)
    events/auto/
    localisation/<lang>/auto/
    gfx/
    interface/
```

### 22.2 File count per module

**Compiler has flexibility** — one module's items can span multiple emitted files, or multiple modules can share a file, as long as:
- Module-tree reachability is respected (items in unreached modules don't emit).
- Import/use chains work (inter-module dependencies still resolve).
- No two items collide on the same `(file, item_name)` pair.

In practice the compiler picks heuristics (per-module file, bundle small modules together, split oversized modules) for readable output. Tuning happens in later optimization passes.

### 22.3 Routing rules (per item)

Each emitted item's output path is derived in this order:

1. **`#[file("...")]` override** → full manual path; skips everything below.
2. **Crate name** → top-level directory (`heritage/`).
3. **Item kind** → Stellaris subdirectory (`scripted_effect` → `common/scripted_effects/auto/`, `event` → `events/auto/`, `building_type` → `common/buildings/auto/`, etc.).
4. **Module path** → filename hint (`heritage::events::dynasty` → `dynasty.txt` within the items-of-this-kind subdir). Compiler may override for bundling.

### 22.4 Collision detection

Two items routing to the same `(output_path, item_name)` pair → **compile error**. Catches:
- Duplicate names across crates (which would silently LIOS-stomp otherwise).
- Two modules emitting items with the same auto-generated name.
- Authored name collision via `#[name("...")]`.

### 22.5 Assets (GFX, localisation, icons)

Follow the same crate/module-path routing. Dedicated decorators declare the asset kind:

- `#[gfx_sprite_type(name = "...")]` → `interface/<crate>/auto.gfx`
- `#[localisation(key = "...", lang = "english")]` → `localisation/english/auto/<crate>.yml`
- `#[icon(file = "...")]` → `gfx/interface/icons/<crate>/auto/...`

### 22.6 `#[file("...")]` escape hatch

Last-resort manual routing. Kept for:
- Vanilla-replacement mods where exact filename affects load order.
- Non-Clause mods expecting specific file names for integration.
- Rare auto-layout collisions the compiler can't otherwise resolve.

Lint-warned as "unusual — document the reason" to discourage routine use.

---

## 23. Patches in depth (chunk 4a)

Two mechanisms. No more find/replace string-editing.

### 23.1 Method-body overrides: `super.foo()`

Inside a `#[patch]`-marked impl, `super.foo()` calls the base impl's original body. The compiler shadow-renames the base at codegen; the patched version becomes the canonical name.

```rust
// Upstream declares
impl ResourceProducer for Country {
    fn produce_energy() -> int {
        // base logic
        return 10;
    }
}

// Patch mod overrides, composing on top of base
#[patch(target = "upstream::ResourceProducer::produce_energy", reason = "orbital ring double-counting fix")]
impl ResourceProducer for Country {
    fn produce_energy() -> int {
        let base = super.produce_energy();
        if self.has_orbital_ring() {
            return base / 2;
        } else {
            return base;
        }
    }
}
```

Codegen:
- Base impl renamed to shadow: `upstream_country_produce_energy__base`.
- `super.produce_energy()` → direct call to shadow.
- Patched impl emitted as canonical `upstream_country_produce_energy`.

Chainable: a third-level patch's `super` calls the second-level, not the original.

### 23.2 Data-definition overrides: `..super` struct-spread

For data items (trait definitions, building types, modifiers — non-function items), use struct-update syntax:

```rust
#[patch(target = "vanilla::TraitDef::cold_hardy", reason = "buff environment tolerance")]
trait_definition cold_hardy {
    ..super,                                      // inherit all fields from base
    modifiers = Modifiers {
        pop_environment_tolerance: 0.15,          // override specific field (was 0.10)
        ..super.modifiers                         // keep other modifier entries
    },
}
```

`..super` spreads all base fields; explicit overrides replace individual entries. Nested `..super.field` for sub-struct update.

### 23.3 Multiple patches on the same target

**Within a single workspace build**: two patches of the same target (both marking `#[patch]` on the same `mod::path::Item`) → **compile error** forcing consolidation. A third crate must write the merged patch explicitly with both `reason`s referenced. Tenet 7 (consolidate-don't-choose) enforced mechanically.

**Across distributed crates (NOT in the same workspace)**: the orphan rule can't prevent silent collision. Mod-A and Mod-B ship independently; each declares `#[patch(target = "vanilla::Country::produce_energy", ...)]`; each builds cleanly in isolation; both land in the player's modlist; Stellaris picks one via LIOS at runtime. Exact failure mode Clause was meant to prevent.

**Mechanism to close this**: patch declarations are exposed as outbound manifest ABI.

Each crate's `clause-manifest/patch_registry.manifest` lists every `#[patch]` declaration the crate makes:
```yaml
patches:
  - target: vanilla::Country::produce_energy
    reason: "orbital ring double-counting fix"
    crate: mod_a
    since: "0.1.0"
```

When a downstream crate depends on multiple crates that each patch the same target, the downstream crate's build surfaces the collision with both `reason` strings and a forced-consolidation error:

```
error: conflicting patches detected across dependencies
  target: vanilla::Country::produce_energy
    - mod_a 0.1.0: "orbital ring double-counting fix"
    - mod_b 0.3.2: "energy multiplier for ascension path"
  help: declare an explicit consolidation patch in this crate with
  `#[patch(target = "...", reason = "...", supersedes = ["mod_a", "mod_b"])]`
```

The downstream crate (typically megapatch, or a dedicated compat crate) writes the consolidation explicitly. The patch registry captures which upstream patches the consolidation supersedes.

Cross-ecosystem cases where NEITHER Mod-A NOR Mod-B author knows the other exists (the dominant Stellaris-modlist scenario) still require someone (conventionally the megapatch crate maintainer) to write the consolidation — Clause's manifest mechanism makes the collision DETECTABLE at their build time, not silent-at-runtime.

### 23.4 Patch chain total ordering

When multiple `#[patch]` declarations form a chain (A patches vanilla, B patches A, C patches B), `super.foo()` must call the immediate predecessor, not the original vanilla. Algorithm:

**Chain construction** (at type-check time, per-target):
1. Collect all `#[patch]` impls whose target resolves to the same `(trait_id, type_id, method_id)`.
2. Topological sort by the dependency graph induced by `supersedes` declarations (via `#[patch(..., supersedes = ["crate_a"])]` attribute form OR via `mod.toml` dependency order).
3. For cases where sort order is ambiguous (two patches each declare `supersedes = ["original"]` but not each other), the algorithm tiebreaks deterministically via `(crate_name, declaration_path)` lexical ordering + emits a WARNING that this is an implicit ordering the author should make explicit.
4. The resulting linear chain: `original → patch_a → patch_b → patch_c → ...`.
5. Each `super.foo()` in `patch_c` binds to `patch_b`'s body, not `original`'s.

**Codegen**:
- Original body emits as `<target_shadow_0>`.
- patch_a emits as `<target_shadow_1>`; its `super.foo()` calls `<target_shadow_0>`.
- patch_b emits as `<target_shadow_2>`; its `super.foo()` calls `<target_shadow_1>`.
- The final canonical name (`<target>`) is aliased to the last element of the chain.

**Complexity**: O(n log n) in the number of patches per target (dict lookup + topological sort). For megapatch at ~4,500 patches total, the per-target chain is rarely > 3-4 deep; aggregate cost is negligible.

### 23.5 Token-aware body patching — `#[patch_body]`

The corpus review (migration reviewer, 2026-04-16) identified ~581 files in the megapatch YAML corpus using `replace_in_item` (string-level find/replace within Clausewitz bodies). Full-override patches via `super.foo()` don't scale to this case — copying entire blocks per patch is impractical.

Solution: **token-aware `#[patch_body]` decorator** that operates over G2's Clausewitz lexer, not string matching.

```rust
#[patch_body(
    target = "vanilla::Country::produce_resources",
    reason = "fix thermal decay accounting",
    find = { energy = 10 },              // Clausewitz-syntax pattern, lexed
    replace = { energy = 15 }            // Clausewitz-syntax pattern, lexed
)]
```

**Semantics**:
- `find` and `replace` are Clausewitz-syntax block expressions, lexed by G2 at clause-compile time.
- Matching happens on the token tree: whitespace-insensitive, comment-insensitive, structure-aware.
- `replace` substitutes in-place; the surrounding context is unchanged.
- Multiple match sites in the target: first-match-only by default; `#[patch_body(... all = true)]` to replace all occurrences.
- No-match → compile error (patch is stale; either target changed or the find pattern is wrong).
- Pattern variables allowed: `find = { energy = $X$ }` matches any value of `energy`, captures as `$X$`; `replace = { energy = $X$ * 1.5 }` references the captured value (processed at compile time where possible).

**When to use `#[patch_body]` vs full-override `#[patch]`**:
- Full override (`#[patch]` + `super.foo()`): when the patch fundamentally restructures the logic (adds a new condition, reorders operations, changes return behavior).
- Token-aware body patch (`#[patch_body]`): when the patch is a surgical value tweak (change a constant, swap one modifier for another, inject a single new field).

The migration reviewer flagged this as the single biggest gap in the v1 design. Resolved here.

### 23.6 Method extension (additive): `#[patch_extend]`

Unchanged from §21.5 — adds new methods to an existing type without conflict (unless two extensions define the same method name).

### 23.7 Patching vanilla

Vanilla types are `extern` — coherence prohibits direct `impl Trait for vanilla::Country` since we own neither side. Patches on vanilla use the local-wrapper-trait pattern:

```rust
// Declare a local trait we own
trait CountryOverrides { fn modified_produce_energy() -> int; }

// Patch via local trait + #[patch] bypass
#[patch(target = "vanilla::Country::produce_energy", reason = "...")]
impl CountryOverrides for Country {
    fn modified_produce_energy() -> int {
        let base = super.produce_energy();   // calls vanilla base
        base + 5
    }
}
```

`#[patch]` is the only legal orphan-rule bypass. Every vanilla patch carries a documented `reason` by construction.

---

## 24. Compiler architecture (chunk 4b)

### 24.1 Frontend → IR → Backend

```
┌───────────────────────────────────────────────────────────────────┐
│                CLAUSE FRONTEND (new)                              │
│                                                                   │
│  .cse source files                                                 │
│       ↓                                                           │
│  lex.py         — Clause-grammar lexer                            │
│       ↓                                                           │
│  parse.py       — typed AST                                       │
│       ↓                                                           │
│  resolve.py     — module tree, use-chain, expect/actual,          │
│                    manifest sync                                   │
│       ↓                                                           │
│  typecheck.py   — trait coherence, orphan rule, inference,        │
│                    patch super-resolution                          │
│       ↓                                                           │
│  transpile.py   — Rust-ish bodies → Clausewitz token sequences    │
│       ↓                                                           │
│  codegen.py     — emit-plan tuples (path + tokens + metadata)     │
└───────────────────────────────────────────────────────────────────┘
                              ↓
              ╔═══════════════════════════════════════╗
              ║  IR: EmitPlan stream                  ║
              ║  (emit_path, item_name, item_kind,    ║
              ║   tokens (G2-compat), source_span,    ║
              ║   metadata)                           ║
              ╚═══════════════════════════════════════╝
                              ↓
┌───────────────────────────────────────────────────────────────────┐
│              MEGAPATCH BACKEND (existing — G-batch)               │
│                                                                   │
│  Clause IR                                                        │
│       +                                                           │
│  Raw Clausewitz from source mods (G2-lexed)                       │
│       +                                                           │
│  YAML patches during migration phase                              │
│       ↓                                                           │
│  Unified IR stream                                                │
│       ↓                                                           │
│  Pass framework — conflict resolution, lint passes, dead-code     │
│  elimination, optimization (inline_script promotion, etc.)        │
│       ↓                                                           │
│  Output writer                                                    │
│       ↓                                                           │
│  .dist/ Clausewitz files                                          │
└───────────────────────────────────────────────────────────────────┘
                              ↓
                        Stellaris loads
```

### 24.2 Key properties

- **Frontend emits TOKENS, not text.** Codegen produces `Token` instances directly (G2-compatible). No parse round-trip to intermediate text.
- **Source positions preserved** through the entire pipeline — from `.cse` source line to emitted Clausewitz, visible in debug tooling.
- **Metadata carries patch state, cfg predicates, manifest info** alongside the tokens. Backend passes read both to make conflict-resolution decisions.
- **Frontend is hermetic** — reads `.cse` files, writes emit-plan; no DB/filesystem access beyond that. Deterministic; cacheable at per-crate granularity.

### 24.3 IR shape

```python
@dataclass(frozen=True)
class EmitPlan:
    emit_path: Path              # routing destination (§22)
    item_name: str               # auto-generated or #[name]-overridden
    item_kind: ItemKind          # scripted_effect | scripted_trigger | script_value | ...
    tokens: tuple[Token, ...]    # G2-compatible Clausewitz token stream
    source_span: SourceSpan      # .cse file + byte range for diagnostics
    metadata: EmitMetadata       # patch chain, cfg predicates, visibility
```

### 24.4 Transpile phase

**TranspileStack — the core data structure**. The compiler-impl reviewer identified scope-stack tracking as "the novel core challenge." Concrete representation:

```python
@dataclass
class StackFrame:
    scope_type: Type                 # e.g. Country, Pop
    bind_path: str                # engine scope path, e.g. "this.owner"
    locals: dict[str, LocalBinding]  # let bindings in scope
    scratch_prefix: str              # for unique _temp_* naming
    can_early_return: bool           # False inside loop bodies

class TranspileStack:
    frames: list[StackFrame]
    
    def push(self, scope: Type, bind: str, can_return: bool) -> None: ...
    def pop(self) -> None: ...
    def current_scope(self) -> Type: ...
    def resolve_self(self) -> StackFrame: ...
    def emit_scratch_name(self, user_name: str) -> str:
        # depth-prefixed unique name, e.g. "_t0_new_idx" at frame 0
        return f"_t{len(self.frames)}_{user_name}"
```

Every transpile pass threads a TranspileStack. Lambda scope-opening `|p| { ... }` pushes a frame; method-return / iterator-boundary pops. `self.field` access resolves against `self_frame.bind_path`. Scratch variables get unique names across scope nesting via `scratch_prefix`.

**Key transforms**:

| Clause construct | Clausewitz emit |
|---|---|
| `self.field = X` | `set_variable = { which = <auto_name> value = X }` |
| `self.field += N` | `change_variable = { which = <auto_name> value = N }` |
| `let x = expr` | compile-only binding (eliminated); emitted expr substitutes where `x` is referenced. If `let mut`, emits scratch variable. |
| `if cond { ... } else { ... }` | `if = { limit = { <cond> } <body> } else = { <body> }` |
| `match val { Pat => ..., _ => ... }` | cascade of `if`/`else` keyed on pattern-tag variables |
| `for x in iter { body }` | `every_*` (effect) or `any_*` (trigger), with `|x|` opening as scope |
| `while cond { body }` | `while = { limit = { cond } body }` |
| `return Ok(x)` / `return Err(e)` | Option/Result codegen per §12.1 (flag + value OR sentinel) |
| `self.method(args)` | `<method_auto_name> = { <args> }` |
| `super.method(args)` | `<base_shadow_name> = { <args> }` |
| `name(|p| { p.foo })` | `name = { foo = ... }` — lambda-syntax opens a scope block, `p` binds to it |
| `self.scope_a.scope_b.X` | nested: `scope_a = { scope_b = { X } }` |

Transpile is type-aware — it knows `self.size` is a variable (not a method), `self.capital_scope` is a scope transition, `my_pop.heritage_in_bloodline` is a flag check. Wrong typings surface as transpile errors with source-mapped diagnostics.

**Purity-aware call-site transpile** (per §9.4):

| Call type | Context | Emission |
|---|---|---|
| pure fn | any | only `<fn>__value` reference |
| deferred fn | any | queue-enqueue at call site (cheap var write); drain handler runs effect batched (§32.6) |
| hoistable fn | effect context | `<fn>__effect` runs, then `<fn>__value` used |
| hoistable fn | trigger context (hoistable) | `<fn>__effect` auto-hoisted to outer effect scope (inserted just before first use); trigger uses `<fn>__value` |
| hoistable fn | hoist-impossible context (§9.4.4) | compile error with refactor suggestion |
| coupled fn | effect context | `<fn>__combined` runs (effect + value in one invocation); caller reads result variable |
| coupled fn | trigger context | compile error (coupled fns cannot be called in trigger context) |
| impure_only fn | effect context | `<fn>__effect` runs; no return value |
| impure_only fn | trigger context | compile error (no value to check against) |

Auto-hoist deduplicates: `score() > 3 && score() < 10` emits ONE `score__effect = yes` + two checks of `score__value`. The compiler's expression analyzer identifies when multiple calls to the same impure function in one expression share effect semantics.

**Non-compositional cases — v1 compile errors**:

Not every Rust construct composes with every other in Clausewitz. v1 treats these as compile errors with suggested alternatives, rather than attempting brittle codegen:

- **`?` operator inside `for` loop body**: Clausewitz `every_*` loops can't early-return. Error message: *"the `?` operator requires early-return capability, which Clausewitz `every_*` loops don't provide. Rewrite with `match` + `break`, or restructure the Result-returning logic out of the loop."*
- **`return` inside a nested lambda/scope-opening block**: same reason — engine `every_*` / `any_*` don't support early-return. Error suggests flag-variable pattern or explicit state machine.
- **`self.field` mutation inside `any_*` (trigger context)**: triggers are supposed to be side-effect-free. Compile error: *"cannot mutate state inside a trigger-context iterator; use `every_*` (effect context) or remove the mutation."*
- **Mixing effect + trigger constructs in the same `if` body without an explicit kind fence**: ambiguous emission; user must split into separate methods or use `#[as_X]`.

These are explicit v1 restrictions — author encounters them, compiler suggests the fix, v2 may relax some if clean codegen is found.

### 24.5 Integration with megapatch backend

Clause IR enters the existing pass framework as another source of emit-plan tuples. Existing passes handle Clause output + raw Clausewitz uniformly:
- **Conflict resolution** — sees all emit-plans for the same output path, applies resolution rules (LIOS/FIOS per directory, patch-chain ordering, etc.).
- **Lint passes** (G3f constraint lints etc.) — run on the combined token stream.
- **Optimization passes** — inline_script promotion, dead-code elim, common-subexpression extraction. Operate purely on tokens regardless of source.

YAML-to-Clause migration is seamless from the backend's view — tokens are tokens.

---

## 25. Bootstrap — `stellaris-vanilla-spec` crate (chunk 4c)

Neither `trigger_docs` nor `cwtools-stellaris-config` is sufficient alone. We build a dedicated spec crate in THIS repo that ingests multiple sources, cross-validates, and ships a versioned authoritative spec.

### 25.1 Crate location

Inside the Clause workspace:

```
stellar-heritage/                      # repo root = Clause workspace
  crates/
    clause/                            # compiler + CLI (was tools/clause/compiler)
    stdlib/                            # Clause stdlib
    stellaris-vanilla-spec/            # vanilla declarations source
    heritage/                          # the dynasty mod (what heritage/ dir becomes)
    megapatch/                         # cross-mod conflict resolution (authored content)
    nsc3-compat/                       # example compat crate (future)
    ...
  mod.toml                             # workspace manifest
```

`stellaris-vanilla-spec` is a normal workspace member — authored alongside everything else, reviewable in git, versioned with tags.

### 25.2 Spec crate internal structure

```
crates/stellaris-vanilla-spec/
  mod.toml
  sources/
    trigger_docs/
      <version>/                       # dumped logs per Stellaris version
        effects.log
        triggers.log
        modifiers.log
        on_actions.log
        scopes.log
    cwtools_config/
      imported/                        # snapshotted from cwtools-stellaris-config
    vanilla_scripts/
      <version>/                       # relevant vanilla .txt files for cross-validation
  curated/
    country.cse                         # hand-written ergonomic declarations
    pop.cse
    planet.cse
    ... (top ~50 scopes)
  generated/
    <version>/                         # final merged spec per Stellaris version
      lib.cse
      scopes/
      triggers/
      effects/
      modifiers/
  tests/
    cross_validation/                  # verify spec vs vanilla scripts
    consistency/                       # check no contradictions between sources
  src/
    lib.cse                             # crate entrypoint; re-exports the generated bundle
```

### 25.3 Ingest pipeline

Implemented as a dedicated `clause spec` subcommand (part of the clause CLI, §27):

1. **trigger_docs ingest** — parse logs into structured records: `(name, kind, scope, params, returns, docs)`.
2. **cwtools-config ingest** — parse CWT files into similar records with richer cardinality/scope-push info.
3. **vanilla-scripts ingest** — parse actual game scripts, extract usage patterns (which triggers appear in which scopes, which effects are chained together).
4. **Merge** — reconcile per-item records from all three sources. Conflicts surface as reviewable items.
5. **Validation** — cross-check merged spec against vanilla-script usage; flag claims that don't match observed usage.
6. **Curated-overlay layering** — hand-written `curated/` files override generated declarations where present, adding doc comments, ergonomic method names, trait groupings.
7. **Emit** — write `generated/<version>/` tree. CI validates the build is clean.

### 25.4 Versioning

Spec crate versioned per Stellaris version. Mods depend:

```toml
# Heritage's mod.toml
[dependencies]
stellaris-vanilla-spec = "4.0"     # Stellaris v4.0 compatible
```

Minor bumps (4.0.1 → 4.0.2): regenerated from new trigger_docs, CI validation.
Major bumps (4.0 → 4.5): manual review + curated-overlay updates.

### 25.5 Community contribution pattern

If Clause gains adoption, `stellaris-vanilla-spec` is the obvious first candidate to be extracted to its own repo + community-maintained. Until then, lives in-repo, bumped with Stellaris releases.

---

## 26. Project structure — Clause workspace (post-pivot, 2026-04-17)

Load-bearing framing: **`clause` is the toolchain/compiler, NOT a mod.** The repo is a Clause workspace of many crates. Gameplay is authored entirely in our own crates on top of vanilla + DLCs — no third-party mods ship at runtime.

**Pivot note (2026-04-17):** An earlier version of this section (preserved in git history) listed `megapatch` and per-mod `*-compat` crates as workspace members. That direction is superseded: the megapatch/YAML corpus is preserved read-only under `.archive/` as research context, and no cross-third-party-mod integration ships. The workspace is now organized around three tiers: **foundation** (engine + vanilla + DLC bindings), **shared abstractions**, and **content** crates. The old `expect`/`actual` mechanism (§20.3) remains valid for our *own* cross-crate runtime contracts and for mod-aware compilation via `#[cfg(mod = "X")]` (§20.4) — authors who ship their own modlists downstream still benefit — but the canonical playset is standalone.

### 26.1 Three tiers

```
foundation  →  shared abstractions  →  content
  std                associations           heritage
  stellaris                                 ships_expanded
  stellaris_dlc_utopia                      galactic_diversity
  stellaris_dlc_synthetic_dawn              federations_expanded
  stellaris_dlc_apocalypse                  intrigue_expanded
  stellaris_dlc_distant_stars               ai_expanded
  stellaris_dlc_megacorp                    traditions_expanded
  stellaris_dlc_federations                 ascension_expanded
  stellaris_dlc_nemesis                     dawn_of_civilization
  stellaris_dlc_overlord                    ... (more as designed)
  stellaris_dlc_first_contact
  stellaris_dlc_toxoids
  stellaris_dlc_galactic_paragons
  stellaris_dlc_machine_age
  stellaris_dlc_grand_archive
```

Downstream depends on upstream only. A content crate may depend on `std`, `stellaris`, any `stellaris_dlc_*`, and `associations`; it may also depend on peer content crates where genuine integration exists. Foundation crates depend only on `std` (and `std` on nothing outside Clause itself).

### 26.2 Foundation layer

| Crate | Role |
|---|---|
| `std` | Engine-trick abstractions catalog. `Array`, `Vec`, `HashMap`, `Option`, `Result`, `Bind`, `Registry`, `Singleton`, `Cached`, plus every Clausewitz engine idiom made typesafe (trick-wrappers for flag-vectors, inverse-sets, variable-slots, temporal memoization, etc.). Authored in Clause. Supersedes the earlier `stdlib` naming — one crate, canonical name `std`. |
| `stellaris` | Extern declarations for base-game Stellaris. Scopes (`Country`, `Pop`, `Planet`, `Fleet`, `Leader`, `Species`, `System`, `Galaxy`, …), sealed trait anchors (`Trait`, `Civic`, `Policy`, `Technology`, `Edict`, `Building`, `District`, `Decision`, `Event`, `Modifier`, `Ethic`, `Trait`, `Origin`, …), and the effect/trigger surface. **Demand-driven**: externs grow only when our code references them. Zero busywork. Never an exhaustive pass to bind every vanilla symbol up-front. |
| `stellaris_dlc_<name>` | One crate per owned gameplay DLC (13 total: utopia, synthetic_dawn, apocalypse, distant_stars, megacorp, federations, nemesis, overlord, first_contact, toxoids, galactic_paragons, machine_age, grand_archive). Each declares the externs added by that DLC only (new traits, civics, origins, buildings, events, mechanics). Code inside a `stellaris_dlc_*` crate is implicitly gated by `#[cfg(dlc = "<name>")]`. |

**Rule (demand-driven externs):** We do not author extern declarations for vanilla/DLC symbols unless our content references them. When an author writes `use stellaris::trait::Pacifist` and the crate doesn't have it, the tooling prompts for its signature or offers to scrape it from the engine source. The `stellaris` and `stellaris_dlc_*` crates are living documents, not reference manuals. See `feedback_externs_demand_driven` in memory.

### 26.3 Shared abstractions

| Crate | Role |
|---|---|
| `associations` | Generic relational layer: relations between entities (Country↔Country, Leader↔Leader, Country↔Species, …), with the same vector/registry tricks Heritage uses for bloodlines. Extracted from Heritage so other crates (intrigue_expanded, federations_expanded, diplomacy systems) can express "who owes whom", "who remembers what", "who is bound to whom" without each reinventing the pattern. |

Future shared-abstraction crates may appear as reuse emerges — they belong at this tier when *multiple* content crates need them. A pattern that only one crate uses stays inside that crate.

### 26.4 Content layer (the `_expanded` baseline)

Each content crate adds a cohesive subsystem. The `_expanded` suffix marks crates that add to vanilla's existing domain surface (ships, federations, intrigue, AI, traditions, ascension). Crates without the suffix add something new (heritage, dawn_of_civilization, galactic_diversity).

| Crate | Surface |
|---|---|
| `heritage` | Flagship. CK-level dynasty system — bloodline vectors, family, marriage, children, legacy, trait inheritance, 6 heritage modes, association vectors. Depends on `associations` for the relational substrate. |
| `ships_expanded` | NSC3-concept-without-classes: richer sections, components, and variety on vanilla ship classes. No new ship classes (engine class-addition is brittle); all expansion is along axes the engine already supports. |
| `galactic_diversity` | More unique systems, anomalies, precursors, archaeology, encounters. Narrative surface for exploration. |
| `federations_expanded` | Federation types, laws, cohesion mechanics, internal politics. Interplays with `intrigue_expanded` and `ai_expanded`. |
| `intrigue_expanded` | Espionage, factions, political manoeuvres, assassination, intrigue-driven events. Interplays with `heritage` (who is plotting against whom), `ai_expanded` (AI personalities drive intrigue differently). |
| `ai_expanded` | Richer AI personalities, behaviour trees, economic/diplomatic/military strategy modes, ethic-aware decision-making. Cross-cutting: other crates *also* extend AI where domain-local personality differences exist (see §26.6). |
| `traditions_expanded` | More traditions, tradition categories, unity paths, cross-references with `ascension_expanded`. |
| `ascension_expanded` | More ascension perks, paths, endgame trajectories. |
| `dawn_of_civilization` | Pre-FTL narrative prologue (dream feature) — events, decisions, origin-shaping pre-spacefaring sequences that feed starting conditions. |

This list is the baseline, not a ceiling. New content crates appear as designs land. Per `feedback_think_fresh_not_catalogue`: we draw one-line awareness from source mods but author our own mechanics fresh — no deep catalogue-and-port pipeline.

### 26.5 DLC-gated compilation — `#[cfg(dlc = "...")]`

Authored code can gate on DLC presence:

```rust
#[cfg(dlc = "utopia")]
fn habitat_heritage_hook() { ... }

#[cfg(not(dlc = "nemesis"))]
fn custodian_stub_fallback() { ... }

#[cfg(all(dlc = "galactic_paragons", dlc = "nemesis"))]
fn council_custodian_interaction() { ... }
```

The compiler treats `dlc = "X"` analogously to `#[cfg(mod = "X")]` (§20.4). Code inside a `stellaris_dlc_<name>` crate is implicitly `#[cfg(dlc = "<name>")]` at the crate level; authors don't write it explicitly there.

**Env-specific playset compilation:** just before the user plays, `clause build-playset` compiles the workspace against the *actual* DLC set on that machine. Code paths referencing unavailable DLCs are DCE'd out — the shipped playset contains zero dangling references to content the engine can't load. See task W5 (compiler DCE) and W6 (build-playset command). See memory `feedback_env_specific_playset_compile`.

This makes our playset portable: the same source compiles cleanly on a machine missing DLCs (the missing DLC's content simply drops from output), and couch-co-op across two machines with different DLC sets is handled by each machine getting its own compiled playset.

### 26.6 Cross-cutting domains (principle)

**Domain crates may freely extend shared concerns — there is no forced encapsulation.**

`ai_expanded` carries the canonical AI personality scaffolding, but `heritage`, `intrigue_expanded`, `federations_expanded`, etc. may all add *their own* AI behaviour rules, personality-weight hooks, and strategic-weight modifiers where the domain requires it. Forcing every AI tweak through `ai_expanded` would bloat that crate and starve domain crates of necessary expressiveness. Integration is interplay, not encapsulation. See memory `feedback_cross_cutting_domains`.

The mechanism is trait impls: `ai_expanded` exposes traits like `AiPersonality`, `DecisionWeight`, `BehaviourHook`; any content crate may `impl` them for its own types. The orphan rule (§10.1) still applies, so authors either own the type or own the trait.

### 26.7 Conventions vs. mandates

**Clause stays general-purpose. House style lives in config, not the compiler.**

The conventions we follow for *our* workspace:
- One unit per file (one trait, one civic, one event, …) — improves authoring ergonomics and diff clarity, but nothing in the language enforces it.
- Auto-discover by directory (`traits/`, `civics/`, `policies/`, `events/…` etc.) — implemented via user-configurable `[[auto_discover]]` sections in the crate manifest, not hardcoded compiler knowledge.
- Category subdirectories for `events/` (ceremony, intrigue, heritage, …) — a pattern, not a rule.

A crate that prefers many-units-per-file, or a flat layout, or entirely different categorization, remains a valid Clause crate. The compiler has no opinion. See memory `feedback_conventions_not_mandates`, and task W7 (manifest-configured auto-discover).

### 26.8 Directory layout (target)

```
stellar-heritage/
  Clause.toml                             # workspace root manifest (task W1)
  crates/
    std/
      src/
    stellaris/
      src/
    stellaris_dlc_utopia/
      src/
    stellaris_dlc_synthetic_dawn/
      src/
    ... (one per DLC) ...
    associations/
      src/
    heritage/
      src/
    ships_expanded/
      src/
    ... (one per content crate) ...
    clause/                              # the compiler itself, as a workspace member
      src/                               # Python during bootstrap; eventually self-hosted in Clause
      bin/
  .dist/                                 # build output (gitignored)
  .cache/                                # compile cache (gitignored)
  .archive/
    research/modlist-audit/              # preserved pre-pivot audit corpus (read-only)
    design-history/megapatch-era/        # preserved pre-pivot design docs
  docs/
  megapatch/                             # legacy YAML patch corpus — read-only concept reference
  ...
```

Pre-migration: the existing `tools/clause/compiler/` stays put and is reachable as the compiler. Migration to `crates/clause/` happens when the M-batch infrastructure is real (M2-M10). The legacy `megapatch/` YAML corpus at the repo root is preserved unchanged as a concept-research artifact and is not re-compiled for shipping.

### 26.9 Workspace manifest (Clause.toml)

The workspace root manifest (spec owned by task W1) declares members, the toolchain edition, and playset-level settings:

```toml
[workspace]
edition = "2026"
members = [
    "crates/std",
    "crates/stellaris",
    "crates/stellaris_dlc_*", # glob
    "crates/associations",
    "crates/heritage",
    "crates/*_expanded",     # glob
    "crates/dawn_of_civilization",
    "crates/galactic_diversity",
    "crates/clause",
]

[workspace.playset]
# Defaults for `clause build-playset`; overridable per-env.
dlcs_available = "auto"      # detect from Stellaris installation
output = ".dist/playset"

[workspace.auto_discover]
# House-style defaults inherited by member crates unless overridden.
events = "src/events/**/*.cse"
traits = "src/traits/**/*.cse"
civics = "src/civics/**/*.cse"
# ... entirely arbitrary; compiler has no hardcoded directory knowledge.
```

Member crates have their own `Clause.toml` (or keep `mod.toml` per §20.1 — naming settled by W1). Cross-crate deps within the workspace resolve to local paths.

---

## 27. Debug tooling + CLI (chunk 4d)

### 27.1 Source-mapping comments in emitted Clausewitz

Every emitted item carries `# >>` origin comments tracing back to `.cse` source:

```
# >> heritage/bloodline.cse:42 — fn add_member()
heritage_bloodline_add_member = {
    # >> let new_idx = self.size
    # >> self.last_index = new_idx
    set_variable = { which = heritage_bloodline_last_index value = trigger:heritage_bloodline_size }
    # >> self.size = self.size + 1
    change_variable = { which = heritage_bloodline_size value = 1 }
}
```

Engine ignores comments; authors reading emitted output find source context immediately.

### 27.2 Structured source map

Alongside emitted Clausewitz, compiler writes `dist/<crate>/source_map.json`:

```json
{
  "version": 1,
  "entries": [
    {
      "emit_file": "common/scripted_effects/auto/bloodline.txt",
      "emit_line": 12,
      "source_file": "heritage/bloodline.cse",
      "source_line": 42,
      "source_col": 4,
      "context": "fn add_member > let new_idx = self.size",
      "type_at_source": "int"
    }
  ]
}
```

Tooling (IDE integrations, inspectors, crash reports) consumes this without parsing comments.

### 27.3 Clause CLI

Focused, composable commands. No `type-at` (folded into `explain`).

| Command | Purpose |
|---|---|
| `clause build` | Full compile: frontend + backend → `dist/`. |
| `clause check` | Type-check only. Fast feedback; no codegen. |
| `clause fmt` | Format `.cse` sources. |
| `clause test` | Run tests. |
| `clause doc` | Generate docs from doc comments. |
| `clause explain <target>` | Universal inspector. Target is `<file>:<line>[:<col>]` OR an item name. Returns whatever is meaningful at that location: type, source↔emit mapping, patches applied, callers, emit kind, etc. Replaces `type-at`, `trace`, and prior `explain`. |
| `clause spec <subcmd>` | Manage the `stellaris-vanilla-spec` crate: ingest logs, validate, regenerate, diff between versions. |
| `clause new <kind>` | Scaffold a new crate / struct / trait / event. |

### 27.4 `clause explain` examples

**By source position** (what's at this .cse location):
```bash
$ clause explain heritage/bloodline.cse:42:15
heritage/bloodline.cse:42:15
  context:   fn add_member (on struct Bloodline: Registry<Self>)
  expression: self.last_index = new_idx
  type:      int (LHS is mut int field; RHS is int local)
  emits to:  common/scripted_effects/auto/bloodline.txt:14
  as:        set_variable = { which = heritage_bloodline_last_index ... }
```

**By emit position** (what source produced this Clausewitz):
```bash
$ clause explain common/scripted_effects/auto/bloodline.txt:14
common/scripted_effects/auto/bloodline.txt:14
  ← heritage/bloodline.cse:42:5
  context:   fn add_member body; emission of `self.last_index = new_idx`
  applied patches: (none)
```

**By item name** (end-to-end trace):
```bash
$ clause explain heritage_bloodline_add_member
heritage_bloodline_add_member
  declared:    heritage/bloodline.cse:33 — fn add_member(who: Pop) -> Result<int, CapacityFull>
  kind:        scripted_effect (inferred from body content)
  emit file:   common/scripted_effects/auto/bloodline.txt:12
  callers (2):
    - heritage/events/dynasty.cse:18 — event NewHeir::immediate
    - heritage/events/marriage.cse:42 — event Wedding::immediate
  patches:     (none)
  super chain: (no overrides)
```

### 27.5 IDE / LSP support (future)

Language Server Protocol implementation built on top of the compiler:
- Go-to-definition, find-references.
- Hover type info.
- Inline error diagnostics, Elm-inspired error UX.
- Refactor: rename (with manifest-integrated propagation per Tenet 12).

Post-bootstrap work. Arrives after M-batch core completes.

---

## 28. Migration plan (chunk 5a)

Five phases. Each phase produces a working build at every commit — no multi-week "everything broken while we migrate" windows.

### Phase 1 — Clause compiler MVP

**Deliverable**: A working `clause build` command that lexes, parses, typechecks, and emits Clausewitz tokens for a non-trivial `.cse` input exercising the core machinery. Megapatch backend consumes the output alongside existing YAML/raw-Clausewitz sources.

**Scope** (concrete MVP — compiler-impl reviewer's sizing):
- `crates/clause/` with lex.py + parse.py + minimal typecheck.py + minimal codegen.py.
- `crates/stdlib/` skeletal: `Array`, `Vec`, `Option`, `Result`, `Bind`, `Registry`, `Singleton`. Deferred to later phases: full collections API, `Cached<T>`, iterators.
- `crates/stellaris-vanilla-spec/` hand-curated for ~10 core scopes (Country, Pop, Planet, Leader, Fleet, Species, Federation, Sector, Megastructure, Situation). Full ingest is M9.
- A non-trivial `.cse` fixture compiles end-to-end.

**Strengthened acceptance gate** (per migration + compiler-impl reviewers):

Phase 1 is NOT "hello world." The gate fixture must exercise:
1. A struct with `: Country` bind + fields with `mut`/`const`.
2. An impl block with two methods — one scripted_effect, one scripted_trigger (proves kind-inference).
3. A `#[patch]` impl with a working `super.foo()` call (proves orphan-rule bypass + shadow-rename + chain dispatch).
4. An event declared `for Country` invoking the patched method (proves event codegen + call-site type checking).
5. Emitted Clausewitz loads in Stellaris and produces the expected runtime behavior.

This gate proves the HARD parts (patch dispatch, super-chain, kind inference) work, not just the trivial parts (lexing, variable names). Without this gate, Phase 1 completion is a false victory.

**Sizing**: ~4-6 weeks to MVP with the constrained scope above; +2-3 weeks of stabilization before Phase 2 starts.

### Phase 2 — Proof of concept: one Heritage subsystem in Clause

**Deliverable**: A single Heritage subsystem (Bloodline is the obvious pick — small, self-contained, was the original motivating example) is authored in `.cse`. **Diff-approved semantic equivalence vs existing Clausewitz.**

**Scope**:
- Bloodline's scripted_triggers, scripted_effects, events, and localisation stubs all in `.cse`.
- The existing Bloodline implementation is YAML-wrapped Clausewitz (corpus reviewer identified 44 patch files, 1,058 items) — NOT plain `.txt`. The v1 gate clarifies accordingly.

**Revised gate — diff-approved, not byte-identical** (per migration reviewer):

Byte-identical output is the WRONG goal. Clause's value proposition is CLEANER output — the migrated Bloodline should collapse the 100-line if/else_if chain to `Array<T, N>` indexing, reducing the emitted Clausewitz substantially. Byte-identical would mean Clause produced identical-to-hand-written output, which contradicts the point of the abstraction.

**The correct gate**:
- Migrated output loads in Stellaris without errors.
- Runtime semantics identical: every Bloodline operation behaves identically to the pre-migration version.
- Diff reviewed line-by-line; structural changes APPROVED with explicit rationale ("Array<T, N> replaces the if-cascade; shorter + provably equivalent").
- Any UNAPPROVED divergence is a compiler bug, not an accepted outcome.

**Kind-inference ambiguity gate** (per modder reviewer): during Phase 2, measure the actual rate of `#[as_X]` annotations required on real Heritage code vs the language's 95% unambiguous-inference claim. If the real rate is <85%, revisit the inference table BEFORE committing Phase 3.

**`clause build` performance gate** (per modder reviewer): measure end-to-end compile time for the Bloodline subsystem + representative vanilla-spec. If >10s for interactive use, invest in incremental compilation / caching before Phase 3.

**Acceptance gate**: Heritage loads in-game, Bloodline plays identically to pre-migration, diff approved, inference-rate + performance gates met.

### Phase 3 — Heritage migrates

**Deliverable**: All of Heritage authored in Clause. YAML and raw Clausewitz retired for this crate.

**Scope**: subsystem-by-subsystem migration. Order: Bloodline → Family → Marriage → Succession → Intrigue → Dawn of Civilisation → events layer. Each step diff-validated.

**Acceptance gate**: Heritage at current feature parity, zero non-Clause source files.

### Phase 4 — Megapatch crate migrates

**Deliverable**: The ~4,500 cross-mod patches currently authored as YAML move to Clause `#[patch]`, `#[patch_extend]`, and `#[patch_body]` impls.

**Scope** (per corpus review: 581 replace_in_item + 812 inject_field + others):
- Full overrides (`override` YAML type) → `#[patch]` with full method body.
- Field injections (`inject_field`, ~812 files) → `#[patch_extend]` adding new methods OR `#[patch_body]` injecting into existing block.
- Body edits (`replace_in_item`, ~581 files) → `#[patch_body]` with token-aware find/replace (§23.5). This is the critical piece closed by v2 design; without it Phase 4 is infeasible.
- Prefer-mod collapses → trait impls with documented `formatting-only` rationale per existing MD3 lint.
- No-override entries → Clause representation TBD during migration (some may become comments; others may need a `#[no_override(reason)]` attribute added).

**Revised time estimate** (per migration reviewer): **700–2,100 hours** (1–4 years part-time pace), NOT the 375–750 hours originally estimated. Phase 4 is NOT "mostly mechanical" — `replace_in_item` cases require per-patch review + token-aware find/replace authoring + validation.

**Stale-patch remediation during migration** (per migration reviewer): the existing 688 stale-patch errors (per `STALE_PATCHES.md`) MUST be fixed as each patch migrates, not translated from broken-YAML to broken-Clause. A patch that is stale today stays stale in Clause unless actively fixed. This turns Phase 4 into a bundled "migrate + fix staleness" effort, which is LONGER but delivers more value.

**Precondition for committing Phase 4**: validate the `replace_in_item` → `#[patch_body]` translation strategy on **100 representative real patches** from the corpus. If the strategy fails >10%, Phase 4 is not ready; revisit `#[patch_body]` design.

**Acceptance gate**: megapatch crate is entirely Clause-authored; YAML loader marked deprecated; diff-approved build output for the megapatch artifacts; 688 pre-existing stale-patch errors drop to zero (not translated; fixed).

### Phase 5 — YAML retirement + community publication

**Deliverable**: YAML loader removed; the only way to author for the Clause-compiled workflow is `.cse`. Optionally spin out Clause + stdlib + stellaris-vanilla-spec as a standalone tool repo.

**Scope**: remove `ingest/parse_patches.py` YAML branch, strip loader from the pass framework, drop YAML tests. Publish if community interest warrants.

**Acceptance gate**: `grep -r '\.yml' crates/` returns zero for patches (localisation `.yml` files still exist — different parser).

### Timing (realistic estimates)

| Phase | Estimated effort |
|---|---|
| 1 (MVP) | 4–8 weeks |
| 2 (PoC) | 2–3 weeks |
| 3 (Heritage full) | 2–4 months (ongoing) |
| 4 (Megapatch) | 2–4 months (mostly mechanical; can overlap with Phase 3) |
| 5 (YAML retirement + publish) | 2–4 weeks |

**Total to Phase 2 acceptance**: ~6–11 weeks (~2–3 months at part-time pace). Full migration: 6–12 months. Long-term commitment; the user has explicitly chosen this path knowing the cost.

---

## 29. Task decomposition — the M-batch (chunk 5b)

The M-batch implements the design. Tasks are ordered by dependency; implementable in sequence or parallel where possible.

**Sequencing refinements** (per compiler-impl reviewer):
- **M9 (vanilla-spec ingest) starts in parallel with M2-M4**, not after M7. M9 requires running Stellaris to dump `trigger_docs` (mostly external / data-gathering work); its output feeds M5 (type-check) and M6 (transpile) testing. Starting M9 early de-risks the whole downstream pipeline.
- **M11 (Bloodline migration) starts after M6 (transpile) completes**, not after M10 (CLI). The CLI nice-to-haves (like `clause explain`) aren't required to author + test a first migration. Cutting M10 out of M11's critical path saves 1-2 weeks.

Updated order: M2 → M3 → M4 (→ M9 runs parallel to M2-M4) → M5 → M6 → M7 → (M8 + M10 in parallel) → M11 → M12.

### M1 — Design (chunks 1–5) — COMPLETE

This document. Chunks 1–5 settled. Implementation tasks below.

### M2 — Clause lexer

- Token taxonomy for `.cse` files (distinct from G2's Clausewitz lexer; Clause has its own grammar).
- Token kinds: identifiers, literals, operators (`+`, `-`, `*`, `/`, `=`, `==`, `!=`, `>=`, `<=`, `>`, `<`, `+=`, `-=`, `?`, `|`, `&&`, `||`, `!`), keywords (`mod`, `use`, `pub`, `struct`, `trait`, `impl`, `fn`, `let`, `mut`, `const`, `if`, `else`, `match`, `for`, `while`, `return`, `self`, `Self`, `super`, `extern`, `expect`, `actual`, `event`, `trait_definition`, `enum`), brackets, trivia.
- Lossless round-trip property.
- Tests: per-kind positive + edge cases.

### M3 — Clause parser + AST

- Recursive-descent parser over M2's tokens.
- AST node types: `Crate`, `Module`, `Struct`, `Trait`, `Impl`, `Fn`, `Event`, `Use`, `Expect`, `Actual`, `Expr` (match/if/while/for/let/return/call/field/method/literal/unary/binary/lambda), etc.
- Error recovery: emit partial AST with error nodes; don't fail-fast on first error.
- Tests: full-grammar coverage.

### M4 — Module resolver

- Walk `src/lib.cse` → crate-root module tree via `mod foo;` chain.
- Resolve `use` imports across the tree.
- Process `expect`/`actual` declarations; track cross-mod runtime contracts.
- Dead-module detection (warn).
- Tests: nested modules, glob imports, re-exports, cross-crate deps.

### M5 — Type checker

- Trait resolution + coherence (orphan rule per Rust).
- Patch chain resolution (`super.foo()` → base shadow name).
- Inference for `let`, method returns, lambda parameter types.
- `Bind` sealed-trait enforcement for struct bind targets.
- Generics monomorphization bookkeeping.
- Pattern-match exhaustiveness on `Option`/`Result`/enums.
- Tests: all type-system features with positive + negative cases.

### M6 — Transpile (Rust-ish body → Clausewitz tokens)

- Per-construct rewrite rules per §24.4.
- `self.field` expansion to auto-generated names.
- Control-flow desugaring (`if`/`match`/`for`/`while` → Clausewitz equivalents).
- Lambda-syntax scope-opening.
- Option/Result codegen (sentinel vs flag+value per type).
- Source-span tracking through every rewrite.
- Tests: per-construct fixtures + end-to-end body-to-tokens.

### M7 — Codegen (AST + transpiled bodies → EmitPlan stream)

- Route items per §22 (crate → kind → module path → filename).
- Emit Clausewitz tokens with source-map comments.
- Emit `source_map.json` structured file.
- Collision detection (compile error on same-path-same-name).
- Tests: routing rules, override cases, collision fixtures.

### M8 — Manifest management

- Detect renames via git-diff against last-tagged version.
- Process `#[supersedes]` / `#[deprecated]` attributes.
- Auto-write `event_ids.manifest` and `symbol_aliases.manifest`.
- Auto-scaffold sham handlers.
- Tests: rename detection, alias emission, sham scaffolding.

### M9 — stellaris-vanilla-spec ingest pipeline

- `clause spec ingest` subcommand.
- Parse trigger_docs logs into structured records.
- Parse cwtools-stellaris-config files.
- Parse relevant vanilla scripts for cross-validation.
- Merge sources; surface contradictions for review.
- Apply hand-curated overlay.
- Emit `generated/<version>/` spec tree.
- Tests: ingest fixtures, conflict resolution, version diff.

### M10 — Clause CLI

- Commands: `build`, `check`, `fmt`, `test`, `doc`, `explain`, `spec`, `new`.
- `clause explain <target>` universal inspector (file:line, item name).
- Integration with existing megapatch backend via IR stream.
- Tests: CLI harness per command.

### M11 — First migration target: Bloodline subsystem in Clause

- Port Heritage's Bloodline to `.cse`.
- Validate byte-identical output vs pre-migration baseline.
- This is the Phase-2 acceptance gate.
- Tests: diff-approved output; in-game Bloodline plays identically.

### M12 — LSP / IDE support (post-bootstrap)

- Language Server Protocol implementation.
- Go-to-def, find-refs, hover-types, rename-refactor.
- Elm-style friendly error messages.
- Deferred until Phases 1–3 complete.

### M-batch superseded / folded tasks

- **G5 (#41) — Custom patch DSL parser**: SUPERSEDED by Clause. The Clause language IS the patch DSL. Mark deleted.
- **G6 (#42) — YAML patch retirement**: reframed as **Phase 5 of Clause migration**. Keep task but retarget to "after Phase 4 acceptance."
- **G7 (#44) — CWT schema loader**: folded into **M9 (stellaris-vanilla-spec ingest)**. Mark deleted or merged.
- **TCR1 (#18) — rename tools/clause/ → tools/cli/**: reframed as part of the Clause workspace migration. The new target is `tools/clause/compiler/` → `crates/clause/`. Update task scope.

---

## 30. Open questions consolidated (chunk 5c)

These surfaced during walkthrough; none blocking M2 start, but worth explicit user judgment as implementation approaches:

1. **Language name** `clause` and extension `.cse` are set. Workspace crate names (`clause`, `stdlib`, `stellaris-vanilla-spec`, `heritage`, `megapatch`, `nsc3-compat`) are set.

2. **Trait supertrait `:` vs struct bind `:`** — the same character means different things on `trait Foo: Bar` (supertrait) vs `struct Foo: Bar` (bind). Internally consistent ("is-a" vibe); Rust people blink once. Accepted during walkthrough; flagged as "if it's confusing in practice, reconsider later."

3. **Error message philosophy** — Elm-friendly or technical? Accepted target: Elm-style (first-person, fix-suggesting, domain-language). Implementation details will emerge during M5/M6 as specific error classes get written.

4. **Transpile sugar extent** — how far does the Rust-ish body syntax stretch? Settled: everything the author writes is Rust-ish; escape hatch via `#[file(...)]` + authoring raw Clausewitz is last-resort only. New constructs added as authoring patterns emerge.

5. **LSP server** — priority TBD. Post-bootstrap.

6. **Community publication** — if Clause + stdlib + stellaris-vanilla-spec prove valuable, extraction to standalone repos is possible. Defer to Phase 5.

7. **Heritage subsystem migration order** — Bloodline → Family → Marriage → Succession → Intrigue → Dawn → events layer. Sequence can shift if migration difficulties surface; Bloodline first is locked.

---

## 31. v2 amendments — operational resilience + feature bank

Consolidates the remaining findings from the four review agents that didn't fit into existing sections.

### 31.1 Hidden Registry entity resilience (Tenet 11 operational corollary)

Per modder reviewer: structs bound to `Registry<Self>` or `Singleton` rely on a hidden engine entity (usually a hidden country) holding the data. If that entity is accidentally deleted (admin console command, mod-introduced bug, save corruption), ALL data on it vanishes silently.

**Mitigation** (part of Tenet 11's practical application):
- On `on_single_player_save_game_load` and `on_multiplayer_game_loaded`, the stdlib emits an auto-generated "registry health check" scripted_effect that verifies every expected hidden entity exists.
- Missing entity → automatic recreation with a user-visible notification ("Heritage: recreating lost Bloodline registry — existing bloodlines preserved if another registry holds them; otherwise data lost").
- Each stdlib bind target (`Registry<T>`, `Singleton`) is responsible for its own resilience logic; the compiler wires the health checks into `on_game_start` and save-load hooks automatically.

Tradeoff acknowledged: automatic recreation can't magically recover destroyed data. The mitigation is "fail loudly + recover partial state," not "no data ever lost."

### 31.2 `expect`/`actual` bootstrap sequencing

Per modder reviewer: `expect`/`actual` requires extern overlay crates for every mod Clause integrates with. Bootstrap chicken-and-egg: no extern overlays exist today.

**Resolution** (no design change needed; clarification of migration path):
- Phase 1-3 (Heritage migration): only needs vanilla extern declarations (provided by `stellaris-vanilla-spec`, M9).
- Phase 4 (megapatch migration): needs extern declarations for each integrated mod. The project authors these as `nsc3-compat`, `esc-compat`, etc. workspace crates on an as-needed basis. Each mod integrated = one compat crate authored.
- Phase 5+ (community adoption): if the language gains adoption, community contributes extern overlays for popular mods. Until then, compat crates are project-owned.

### 31.3 Kind-inference ambiguity rate — Phase 2 measurement

Per modder reviewer: the 95% unambiguous-inference claim is the language's target, not a measured fact. Phase 2 Bloodline migration measures the actual rate on real code.

**Measurement**: count `#[as_X]` annotations required per 100 methods authored during Phase 2. If <85%, the inference table (§9.4) gets a design revisit before Phase 3.

### 31.4 `clause build` performance gate

Per modder reviewer: the compiler must complete representative builds in interactive time for authors to use the language.

**Benchmark target** (soft, measured in Phase 2):
- Bloodline subsystem (single-crate) + vanilla-spec load: < 3s on dev hardware.
- Full Heritage at Phase 3 completion: < 15s cold, < 3s incremental.
- Megapatch at Phase 4 completion: < 60s cold, < 5s incremental.

If Phase 2 measurements exceed 10s for the Bloodline case, invest in per-crate caching + incremental compilation before Phase 3.

### 31.5 Missing features — planned extensions bank

Features identified by modder reviewer as likely-needed-eventually but not in v2 scope. Banked for post-M11 tasks as usage patterns emerge:

- **First-class FIOS/LIOS load-order semantics**. Today implicit in directory (via the existing `rules.py`); Clause should surface as an attribute: `#[load_order(FIOS)]` on a patch or `#[directory_rule(FIOS)]` on an emitted file. Details TBD when we hit a case where it matters.
- **Localisation key validation**. `#[localisation(key = "...", lang = "english")]` exists but doesn't validate key existence or cross-reference. Add a localisation linter pass (post-M10) that checks every emitted reference to a localisation key against a loaded-at-compile-time index.
- **Triggered-description patterns** (per existing `feedback_triggered_variety` memory). Common pattern: events/effects with species-aware / ethic-aware / gender-aware text variants. Clause should have a first-class `triggered_desc` construct that generates the engine's `triggered_desc = { trigger = { ... } text = ... }` chains ergonomically.
- **`clause migrate` scaffolding tool**. Semi-automatic YAML-patch-to-Clause translator for the mechanical cases (~30% of the corpus). Author reviews + fixes the non-trivial 70%. Ships as part of M10+ follow-up.
- **Modifier / building / trait_definition full authoring surface**. Current design focuses on scripted_effect/trigger/value. Data items (buildings, traits, modifiers, edicts) need explicit authoring surface beyond generic struct. Per-item-type Clause constructs or a generalized data-declaration syntax — decided when first data-heavy crate migrates.

### 31.6 `:` bind syntax — acknowledged concern

Per language-design reviewer: `struct Foo: Bar` reads as inheritance or type annotation; the bind semantic ("Foo's data is hosted by Bar") is less natural. User confirmed `:` in chunk-2 walkthrough with a "reconsider if confusing in practice" clause.

**Disposition**: **kept as `:` for v2**, but monitored. If Phase 2 Bloodline authoring surfaces real confusion (either from the user or contributors), revisit with candidates `struct Foo @ Bar`, `struct Foo bound Bar`, or `#[bind(Bar)] struct Foo`. Any change is a breaking language edit but before real `.cse` code exists at scale it's cheap.

---

## 32. Optimization catalog and pass framework

### 32.0 Philosophy

Clausewitz's runtime constraints are harsh: parameterized scripted_effects consume ~2MB RAM each (30 branches per occurrence); triggers evaluate left-to-right with no short-circuit optimization by the engine; variable slots are finite per scope; scope-chain traversals cost engine-side state work. Clause earns its keep by burning compile-time resources to reduce emitted Clausewitz volume, invocation count, and runtime cost.

Two principles:
1. **IR-first**: every optimization we'll ever want needs its metadata present in IR from day one. Metadata is cheap; post-hoc re-analysis is expensive.
2. **Always reuse**: pass framework is the same G-batch scheduler already built. No parallel compiler-internal machinery.

### 32.1 Tier 1 — Structural (must be in IR from day one)

These architectural decisions shape the IR. Skipping any means the matching optimization pass cannot retrofit later without an IR rewrite.

**T1.1 Monomorphization metadata.** `clamp<int>` vs `clamp<float>` specialized instances at codegen. IR carries type parameters through all transforms.

**T1.2 Reachability markers per item.** Every IR item has a `reachable` field set by a reachability pass seeded from events + extern-impl anchors + on_action handlers. Unreached → not emitted.

**T1.3 Cross-crate const propagation hooks.** IR items reference their const dependencies by ID so const values flow across crates at link time.

**T1.4 Global string intern pool.** Cross-crate intern table; string literals reference by ID, not value.

**T1.5 Manifest-driven ABI (kind/purity/encoding/patch).** Already documented (§16). Five manifest files handle contract stability.

**T1.6 Scope-graph state per item.** IR nodes carry their resolved scope stack; later passes avoid re-walking.

**T1.7 Source-span immutability through transforms.** Every pass preserves source spans or declares it breaks them; violations fail the pass framework.

**T1.8 Purity carried as IR metadata.** Per-function (pure/deferred/impure_hoistable/impure_coupled/impure_only). Auto-hoisting + deferred dispatch + cross-crate call validation all read this.

**T1.9 Variable lifetime ranges per local.** `(first_read, last_read)` per `let`/`let mut`. Enables register allocation + dead-store elimination.

**T1.10 Call graph edges in IR.** Incoming + outgoing call edges per function. Many optimizations need this; incremental construction during resolve+typecheck.

**T1.11 Per-expression invariants.** Each expression node: `type`, `effect_set`, `read_set`, `write_set`, `pure_flag`. "Does this expression commute with that one?" answered without re-analysis.

**T1.12 Patch chain pointers.** Function IR knows "I am patch level N of chain C; super resolves to shadow S."

**T1.13 Deterministic ordering invariants.** All IR decisions use stable keys: `(crate, module, declaration_order)` lexical ordering. Cross-build determinism guaranteed.

**T1.14 Compile-time vs runtime marking.** Each IR node tagged: `compile_time` (folds away) vs `runtime` (emits Clausewitz). Propagates through expression trees.

**T1.15 Reachability seeds + expansion rules.** Encoded in IR metadata; the reachability pass consumes these.

**T1.16 Cross-crate type identity.** Hash-stable type IDs match across crate builds. Option/Result ABI + const-propagation rely on this.

**T1.17 Manifest version pinning per item.** Each IR item records the manifest version that stamped its stable properties. Enables cross-version diffing.

**T1.18 Clausewitz construct taxonomy.** Dedicated IR node types for each Clausewitz concept (trigger/effect/value/inline_script/event_target/scope_transition/set_variable/...). No generic "Call" nodes.

**T1.19 Scope-change propagation.** Scope-transitioning calls (`self.owner.foo()`) record new scope at call site; propagates to subsequent expressions.

### 32.2 Tier 2 — Per-pass (post-MVP optimization library)

Implementable after Phase 1 MVP. Each is an independent pass consuming Tier 1 metadata.

**T2.1 Constant folding + arithmetic eval.** Compile-time evaluation of const expressions. Also folds `@[ expr ]` inline arithmetic where all operands are const.

**T2.2 Common subexpression elimination.** Recurring expressions within a body compute once, cache in scratch var.

**T2.3 Dead store elimination.** `self.x = 5; self.x = 10;` → only the second store emits.

**T2.4 Scope transition minimization.** Multiple operations on the same transitioned scope coalesce into one `scope = { ... }` block.

**T2.5 Effect coalescing.** Multiple `change_variable` on same variable → merged; `set_variable` followed by `change_variable` on same variable → single set_variable with combined value.

**T2.6 Inline-script promotion.** Small parameter-free scripted_effects called N>threshold times → emitted as inline_script (saves 2MB/call).

**T2.7 Shared prefix/suffix factoring.** Multiple effects sharing a prefix → factor into separate shared effect + invoke.

**T2.8 Dead-branch pruning via cfg + modlist.** `#[cfg(mod = "nsc3")]` gated code, NSC3 absent → stripped at build.

**T2.9 Event coalescing on shared `on_action`.** Multiple events on the same hook → single event with all bodies in sequence.

**T2.10 Variable slot packing.** Multiple small fields → bit-packed into single variable. Opt-in via `#[packed]`.

**T2.11 Branch ordering for triggers.** Reorder `AND`/`OR` children via `#[likely]`/`#[unlikely]` hints + profile data (Tier 3).

**T2.12 Loop-invariant code motion.** Conditions inside `for` not depending on loop var → hoisted outside.

**T2.13 Deferred drain coalescing.** Multiple `#[deferred]` fns with same `drain_on` hook → merged drain handler. Single event drains all queues in sequence.

**T2.14 Same-scope fusion.** Multiple impure calls hoisted to same outer scope → grouped compound block instead of N separate ones.

**T2.15 AST canonicalization.** `AND = { foo }` → `foo`; `NOT = { NOT = { foo } }` → `foo`; nested-redundant blocks flatten.

**T2.16 Local escape analysis.** Non-escaping `let` values fold into single use; no backing variable.

**T2.17 Field packing for small structs.** Multiple small-range int + bool fields pack into one variable via bit-encoding. Opt-in.

**T2.18 Call-site specialization.** N call sites, M share argument patterns → specialized version for the M sites + generic for rest.

**T2.19 Dead argument elimination.** Unread parameter → stripped from signature; callers updated.

**T2.20 Cross-author deduplication.** Equivalent scripted_effects in different modules → one emitted + aliased.

**T2.21 Loop unrolling for small iter counts.** `for x in array[3]` with compile-known size → unrolled into 3 sequential expressions.

**T2.22 Trigger memoization via flags.** Expensive triggers called multiple times in one tick → compute once, cache in flag.

**T2.23 Event aggregation.** Small independent events on same hook → fused into compound.

**T2.24 Empty-body elimination.** After all passes, empty-body scripted_effects/triggers → removed + callers fixed up.

**T2.25 Hot-path ordering.** Within AND/OR blocks, reorder by (cheap-first + likely-terminating-first) for short-circuit maximization.

**T2.26 Arithmetic simplification.** `x * 1 → x`, `x + 0 → x`, `x * 0 → 0`, constant-operand products pre-computed. Applied to emitted `@[ ]`.

**T2.27 `@[ ]` expression pooling.** Identical inline-arithmetic expressions across items → extracted as named scripted_variable. Respects Stellaris' first-only-@[]-per-body constraint.

**T2.28 Iterator fusion.** `every_pop.filter(P).map(Q).count()` → single `count_pop = { limit = P ... }`.

**T2.29 String literal deduplication.** Per-crate string-literal pool referenced by index.

**T2.30 Effect-expression hoisting extension.** Impure calls inside MTTH modifiers → compile error with suggestion to move to on_action precompute.

### 32.3 Tier 3 — Long-term / speculative (v3+)

**T3.1 Profile-guided optimization.** Runtime telemetry (event fire rates, trigger hits) feeds back into compile-time branch ordering + inline decisions.

**T3.2 Whole-program register allocation.** Non-overlapping `let mut` lifetimes share variable slots across the entire build. Reduces slot pressure.

**T3.3 Control-flow graph optimizations.** Full CFG per emitted function enables tail-call-like eliminations, branch-to-branch peepholing.

**T3.4 Compile-time specialization via usage analysis.** If `Option<PopId>` is always `Some` at all call sites in a crate, the None path compiles away.

**T3.5 Cross-event dependency graph + fusion.** If event A's immediate always triggers event B → merge into single compound event.

**T3.6 Inline_script parameter CSE.** Multiple invocations of same inline_script with identical `$PARAM$` values → one instance + references.

**T3.7 Speculative patch collapsing.** Chains of `#[patch]` where intermediate patches are equivalent-to-delete → collapsed at emit.

**T3.8 Whole-program type refinement.** Full flow analysis across crates; redundant `Option` wrapping eliminated.

**T3.9 Save-migration derivation.** Manifest-driven rename detection automatically generates save-migration scripts as compile artifacts.

**T3.10 Native runtime extension (door left open; not pursued).** The `extern_native` declaration form is reserved. If someone builds a native runtime (SKSE-style injection) as a separate project post-Phase 5, Clause compiles against it without language changes. IR metadata + purity + manifest ABI machinery is runtime-agnostic. **No implementation commitment**; noting the architectural door because closing it permanently would be a design mistake.

### 32.4 EmitMetadata extensions

The v2 `EmitMetadata` struct gains the Tier 1 metadata fields:

```python
@dataclass(frozen=True)
class EmitMetadata:
    # Core (pre-existing)
    patch_chain: tuple[PatchRef, ...] = ()
    cfg_predicates: tuple[CfgPredicate, ...] = ()
    visibility: Visibility = Visibility.PUBLIC
    
    # Tier 1 additions
    type_params: tuple[TypeId, ...] = ()       # T1.1 monomorphization
    reachable: bool = False                     # T1.2 DCE (set by reachability pass)
    const_deps: frozenset[ConstId] = frozenset()   # T1.3 cross-crate const
    string_refs: frozenset[StringId] = frozenset() # T1.4 intern pool indices
    scope_stack: ScopeStackState = None        # T1.6
    purity: PurityClass = PurityClass.PURE     # T1.8
    local_lifetimes: dict[LocalId, LifetimeRange] = None  # T1.9
    call_edges_out: tuple[CallEdge, ...] = ()  # T1.10
    expression_invariants: dict[ExprId, ExprInvariants] = None  # T1.11
    compile_time: bool = False                 # T1.14
    manifest_version: ManifestVersion = None   # T1.17
    scope_changes: tuple[ScopeChange, ...] = () # T1.19
```

These fields populate during the appropriate pass (reachability sets `reachable`, resolver sets `scope_stack`, typechecker sets `purity`, etc.) and remain available to all downstream passes.

### 32.5 Pass framework

Reuses the existing G-batch scheduler (see `passes/scheduler.py` in the megapatch backend). Each optimization pass declares:

```python
class OptimizationPass(Pass):
    kind: PassKind = PassKind.OPTIMIZER
    reads: frozenset[ArtifactKind]      # what IR metadata we read
    writes: frozenset[ArtifactKind]     # what we update
    preserves: frozenset[Property]      # invariants we promise to keep
    opt_level: int                       # minimum --opt level to enable this pass
```

**Optimization levels** (via CLI: `clause build --opt <N>`):

- `--opt 0` — No optimization passes. Debug builds. Fast compile; verbose output.
- `--opt 1` — Basic passes. Default for debug+release interactive dev builds. Includes: T2.1 constant folding, T2.3 dead-store elim, T2.4 scope minimization, T2.5 effect coalescing, T2.8 cfg dead-branch pruning, T2.15 AST canonicalization, T2.24 empty-body elim, T2.26 arithmetic simplification, T2.29 string dedup.
- `--opt 2` — Aggressive passes. Default for `clause build --release`. Adds: all remaining Tier 2 passes.
- `--opt 3` — Speculative passes. Opt-in; includes available Tier 3 passes when implemented.
- `--opt s` — Size-optimized (minimize emitted Clausewitz byte count). Alternate profile that prioritizes RAM savings (inline_script promotion, shared prefix factoring) over speed.

Invariant enforcement: if a pass breaks a `preserves` invariant it didn't declare, the build fails at the pass checkpoint. Optimizations never corrupt semantic correctness; the scheduler verifies.

### 32.6 `#[deferred]` dispatch mechanics

Functions marked `#[deferred]` emit via queue + drain:

```rust
#[deferred(strategy = "queue", drain_on = "on_yearly_pulse", capacity = 1024)]
pub fn log(msg: string) { ... }

#[deferred(strategy = "latest", drain_on = "on_monthly_pulse")]
pub fn update_status(s: Status) { ... }

#[deferred(strategy = "accumulate", operator = "+", drain_on = "on_game_start")]
pub fn add_xp(amount: int) { ... }
```

**Strategies**:

| Strategy | Semantics | Use case |
|---|---|---|
| `queue` | All calls recorded; drain processes in insertion order | Log, audit trail |
| `latest` | Only most-recent call's args retained; drain processes once | Status updates, UI hints |
| `accumulate` | Values folded via operator; drain reads total | XP counters, cumulative stats |

**Call-site emission**: cheap variable write + slot assignment (for queue) or overwrite (for latest) or arithmetic op (for accumulate). No effect-body execution at call site.

**Drain handler emission**: compiler auto-generates an `on_action` handler at the declared cadence. Handler reads queue/value, runs the body logic, resets queue state. Deduped across multiple `#[deferred]` functions sharing the same `drain_on` (T2.13).

**Capacity + overflow**: `capacity = N` sets queue size (fixed array). Overflow policy via `overflow = "drop_oldest" | "drop_newest" | "error"`; default `drop_oldest` for queue, not applicable for latest/accumulate.

**Save/load persistence**: `persist = true | false`. Default `false` for log (discard on save); must be explicit for achievement-like state.

**Drain priority**: multiple drains on same hook evaluate in declaration order; override via `drain_priority = N`.

### 32.7 Author-facing hint attributes (summary)

| Attribute | Purpose | When to use |
|---|---|---|
| `#[deferred(...)]` | Queue/batch side effects; drain on cadence | Log, telemetry, counters |
| `#[prefer(...)]` | Emit kind tiebreaker hint | Rare; forces RAM or speed tradeoff |
| `#[as_inline_script]` | Force inline_script emit | Small pure helpers called many times |
| `#[as_scripted_effect]` / `#[as_X]` | Override inferred emit kind | Rare escape hatch |
| `#[packed]` | Bit-pack small struct fields into one variable | Struct with many bools/small ints |
| `#[likely]` / `#[unlikely]` | Branch ordering hint | Hot-path triggers |
| `#[allow(clause::<pass_name>)]` | Opt out of a specific optimization pass for this item | Debugging, worst-case compat |

Most authors never use any of these. Stdlib uses `#[deferred]` widely (log, telemetry) and `#[prefer(inline_script)]` for small RAM-critical helpers. Everything else is Clause's default behavior.

---

## 33. Storage-backend diversity + engine introspection

Clause's biggest efficiency lever we haven't yet exploited: **Stellaris exposes many scripting primitives, each with distinct cost/capability profiles. The stdlib's `Bind` trait has many valid implementations; the compiler picks per-field based on access pattern + shape.**

### 33.1 Motivation

The design so far defaults to "hidden country + fleet-rename strings" for everything. That's ONE trick among many. Stellaris offers:

- `set_variable` — numeric state (current default)
- Flags — boolean state; engine-cached O(1) checks
- Modifiers — numeric state; engine-summed aggregation
- Policies — enum state; one-of-N per empire
- Country types — type-level classification
- Leader/character traits — per-entity tagged state
- Custom resources — numeric state integrated with resource system
- Situations (3.5+) — multi-phase stateful subsystems
- Event targets — scope references
- Edicts — time-limited boolean state

Each has different read/write cost, capacity, and semantic fit. Using one-size-fits-all hidden registries wastes Stellaris' engine-level support for these specialized primitives.

### 33.2 Multiple `Bind` implementations in stdlib

```rust
// All stdlib-provided; all impl sealed trait Bind.
pub struct Registry<T>;          // hidden country + fleet slots (current general-purpose)
pub struct Singleton;             // single hidden global entity
pub struct FlagStore;             // country/planet/pop flags
pub struct ModifierStore;         // engine-summed modifier values
pub struct PolicyStore;           // enum state via policies
pub struct TraitStore;            // per-character traits
pub struct CountryTypeStore;      // country_type-as-tag
pub struct ResourceStore;         // custom strategic resource
pub struct EventTargetStore;      // scope references
pub struct SituationStore;        // stateful subsystems (3.5+)
pub struct EdictStore;            // time-limited boolean state
pub struct StringStorage;          // fleet-rename strings (already in design)
```

Each carries different capability constraints (e.g., `FlagStore` only accepts bool; `PolicyStore` only accepts enum with ≤ ~20 variants). Sealed trait membership prevents mod authors from defining arbitrary custom bind targets.

### 33.3 Auto-pick algorithm (compiler, not author)

Authors don't pick the backend. The compiler does, per-field, via static analysis:

```rust
struct Bloodline {
    mut is_extinct: bool,              // ← compiler picks FlagStore
    mut prestige: int,                 // ← compiler picks ModifierStore (if accessed as modifier) or Registry (otherwise)
    governance: Mode,                  // ← compiler picks PolicyStore (enum, ≤20 variants)
    mut members: Vec<PopId, 100>,      // ← compiler picks Registry (complex collection)
    matriarch: Option<PopRef>,         // ← compiler picks EventTargetStore (scope reference)
    mut name: string,                  // ← compiler picks StringStorage
}
```

**Decision rules**:

| Field shape | Access pattern | Auto-picked backend |
|---|---|---|
| `bool` | read/write | `FlagStore` |
| `enum` with ≤ 20 variants on Country scope | read/write | `PolicyStore` |
| `enum`, other | read/write | `Registry` (general fallback) |
| `int` / `float` | read-frequent, write-rare, additive semantics | `ModifierStore` |
| `int` / `float` | mixed access | `Registry` (variable) |
| `string` | any | `StringStorage` (fleet-rename trick) |
| `Option<XRef>` (scope reference) | any | `EventTargetStore` |
| `Vec<T, N>` / `Array<T, N>` / complex | any | `Registry` |
| `struct WithPhases { phase: Phase, mut progress: int, ... }` | any | `SituationStore` (detected by phase field + progress-like shape) |
| Compile-time immutable tag | declaration-time | `CountryTypeStore` |

**Override**: `#[bind(X)]` on a field forces a specific backend. Rarely needed; for edge cases where the compiler's heuristic isn't optimal.

**Field-granular picking**: different fields on the same struct land on different backends. The `Bloodline` example above uses 6 different backends for 6 fields. Author writes one struct; compiler routes each field to the most efficient storage. Author never sees the split.

### 33.4 Engine introspection — systematic catalog

Binary reverse engineering is out of scope (ToS + maintenance burden; see §32.3 T3.10). Legitimate introspection sources are ample:

- **Paradox's `trigger_docs` debug command** — authoritative API dump.
- **Vanilla scripts** — every construct pattern Paradox's own devs use.
- **cwtools-stellaris-config** — community-curated schemas.
- **Community mod corpus** — patterns top modders use (via Workshop API).
- **Paradox dev diaries** — archive of technical discussions.
- **moddb + paradoxwikis** — historical + community-documented tricks.

**Harvest tool** (task M14): `clause spec harvest` runs over all above sources, cross-validates, outputs a confidence-tagged catalog that feeds into:

1. `stellaris-vanilla-spec` ingest (M9) — enriches the typed declarations.
2. Storage-backend heuristics (§33.3) — the compiler knows which primitives exist + their costs.
3. `docs/megapatch/clausewitz-tricks.md` — living reference of scripting patterns.

### 33.5 Scripting tricks reference

See `docs/megapatch/clausewitz-tricks.md` — living catalog of Clausewitz scripting patterns, storage primitives, performance tricks, and gotchas. Updated when the harvest tool discovers new patterns or authors contribute verified tricks.

Categories:
- Storage primitives (how to persist data)
- Computation tricks (how to do math / logic)
- Persistence tricks (surviving save/load)
- AI / behavior tricks (how to drive AI)
- Scheduling tricks (timing and cadence)
- String / text tricks (limited; fleet-rename is the workhorse)
- Performance tricks (RAM + CPU optimization patterns)
- Gotchas + anti-patterns
- Undocumented-but-usable patterns (community-discovered)
- Wishlist (things modders wish existed but don't)

Each entry is confidence-tagged (primary-source vs community-verified vs community-reported) and cross-referenced to Clause stdlib backends or compiler passes when applicable.

### 33.6 Ongoing exploration

This design doc captures what's known as of 2026-04-16. The exploration doesn't stop:

- New Stellaris versions expose new APIs (Situations came in 3.5; Scripted GUI also in 3.5; future versions will add more).
- Community modders discover undocumented patterns continually.
- Clause's harvest tool (M14) runs periodically on updated sources to refresh the catalog.
- Authors who find new tricks contribute via PR to `clausewitz-tricks.md`.

Task M15 (to be filed) tracks the ongoing maintenance: quarterly harvest runs + catalog updates + stdlib backend additions as new primitives warrant them.

---

## 34. Design complete

All 5 chunks resolved, all four review-agent findings addressed in v2 amendments, purity + decomposition + auto-hoist replace v2's kind-tracking machinery, §32 catalogs 49 optimization angles across three commitment tiers, §33 formalizes storage-backend diversity with compiler auto-pick + scripting-tricks reference. Implementation starts at M2 (Clause lexer).

Any remaining surprises surface during Phase 1 implementation and feed back into design v5.

---

## End of design draft (chunks 1–2)

This document is authoritative for chunks 1–2 (motivation/vision/goals + type system core). Chunks 3–5 will be appended via continued interactive walkthrough. M2 (Clause lexer implementation) starts only after all chunks are settled.
