# Mod Authoring Language — Design Startpoint (Batch M1)

**Status:** **STARTPOINT, not the design.** This document is a strawman
draft + research consolidation, kept as a reference for the actual
interactive design walkthrough (which produces a separate
`design-draft` document). Per user 2026-04-16: "we'll do it section by
section interactively now, instead of editing that one, because editing
will force us into its chosen mold, whereas we might want something
entirely different."

**Date:** 2026-04-16
**Driver:** User redirect 2026-04-16. Replaces and supersedes G5
(custom patch DSL — task #41) and G6 (YAML retirement — #42 — folds in
naturally as the migration end-state). The G batch (G2-G4, G7) survives
as the **backend** that consumes this language's compiled output.

**Working title:** "Modlang" (placeholder — final name TBD by user;
candidates: `stellaris-rs`, `mp-lang`, `Forge`, `Trait`, `Heritage Lang`).
File extension: `.mp` (placeholder — `.hr`, `.tr`, or `.forge` are
also reasonable). This doc uses "Modlang" throughout for consistency.

---

## 1. Why this matters

### What's broken about authoring directly

Today's mod authoring (whether in Clausewitz, YAML patches, or the
mix we currently use):

- **No abstraction barrier.** When you write `add_modifier =
  { name = thermal duration = -1 }`, you're naming a vanilla
  effect. If Stellaris renames it, every script breaks. If a mod adds a
  new modifier system, you can't write code against the abstract
  concept "thing that can carry a thermal modifier."
- **File location is a design decision.** "Does this patch go in
  `_declarations.yml` or its own file?" "Which mod's directory does
  the override land in?" These should be compiler routing decisions,
  not author decisions.
- **Conflict resolution is invisible.** Mod A overrides X, Mod B
  overrides X. Today: silent LIOS / FIOS pick based on alphabetical
  filename. Authors have no way to *declare* that they're overriding
  the same thing — no compile error, no forced consolidation.
- **Cross-cutting concerns require boilerplate.** "Every entity that
  has a fleet" → write the same trigger/effect by hand against
  Country, Federation, Sector, etc. There's no way to express the
  abstract concept "has a fleet" once.
- **Refactoring is grep + pray.** Rename a scripted_trigger →
  ripgrep across the corpus, hope you find every call site. No
  type-checking, no usage analysis built in.
- **Type errors surface 3 hours into playtest.** Pass a Fleet where a
  Country was expected → silent runtime failure → loading screen hang
  → no diagnostic that says "you used `add_modifier` on a Fleet,
  which doesn't have modifiers."

### What a trait-based language fixes

- **Authors write against abstract APIs**: `bloodline.add_member(person)`.
  The compiler resolves the call to the underlying scripted_effect via
  the impl mapping. Stellaris renames an effect → update one impl,
  every call site is unchanged.
- **File routing is automatic**: `#[scripted_effect("name")]` on an
  impl method → compiler emits to `common/scripted_effects/<crate>/auto.txt`.
  Authors never name files.
- **Conflict resolution is mechanical**: two mods can't both
  `impl Patchable for vanilla::Country` (orphan rule violation). The
  compiler forces them to write a third crate that *consolidates*
  both — Tenet 7 enforced by the type system.
- **Cross-cutting concerns become traits**: `trait HasFleets` with one
  method. `impl HasFleets for Country`, `impl HasFleets for
  Federation`. Code that calls `.fleets()` works on both.
- **Refactoring is safe**: rename a trait method → compiler finds
  every call site → fix or fail. Rename a scripted_effect mapping →
  internal change, no caller affected.
- **Type errors at compile time**: `add_modifier` is declared
  `fn(self: HasModifiers, modifier: Modifier)`. Calling
  `fleet.add_modifier(...)` errors at compile time because Fleet
  doesn't implement HasModifiers. No 3-hour playtest.

---

## 2. Vision

### A complete worked example

```rust
// crates/heritage/src/bloodline.mp

/// A bloodline is a chain of related characters across generations.
/// Members can be added; lookups are O(1) via the per-character index.
trait Bloodline {
    /// What kind of entity is a member? For Stellaris this is Pop,
    /// but the abstract trait doesn't care.
    type Member: HasGenes;

    /// Returns true if `who` is a member of this bloodline.
    fn contains(&self, who: Self::Member) -> bool;

    /// Add a new member. Returns the index in the bloodline.
    fn add_member(&mut self, who: Self::Member) -> int;

    /// First/founder member of the bloodline.
    fn matriarch(&self) -> Option<Self::Member>;

    /// All members in chronological order.
    fn members(&self) -> List<Self::Member>;
}

/// A Stellaris-Pop-based concrete impl.
#[clausewitz_scope("country")]   // bloodlines live on country scope
impl Bloodline for HeritageBloodline {
    type Member = Pop;

    #[clausewitz_trigger("heritage_bloodline_contains")]
    fn contains(&self, who: Pop) -> bool {
        // Body is either:
        //   (a) Pure declarative: maps to a single existing trigger.
        //   (b) Inline impl: emits Clausewitz directly here.
        //
        // For (a), the function body is `extern;` and the decorator
        // names the trigger. For (b), the body is Modlang code that
        // gets translated.
        extern;
    }

    #[clausewitz_effect("heritage_bloodline_add_member")]
    fn add_member(&mut self, who: Pop) -> int {
        // The decorator says: emit a scripted_effect named
        // `heritage_bloodline_add_member` that takes WHO=<pop_id>.
        // The body translates to that effect's content.
        //
        // The `int` return is the new index — implemented as
        // `set_variable = { which = HERITAGE_LAST_INDEX value = ... }`
        // by the codegen.
        let next_idx = self.members().len();
        self.heritage_bloodline_vector(next_idx) = who.id();
        self.heritage_bloodline_size = next_idx + 1;
        next_idx
    }

    #[clausewitz_field("heritage_bloodline_matriarch")]
    fn matriarch(&self) -> Option<Pop> {
        // A field accessor — maps to reading the value off the
        // country scope.
        if self.heritage_bloodline_size > 0 {
            Some(self.heritage_bloodline_vector(0).resolve_pop())
        } else {
            None
        }
    }

    #[clausewitz_iterator("every_pop_in_heritage_bloodline")]
    fn members(&self) -> List<Pop> {
        // Iterator scope — codegen produces `every_*` enumerator.
        extern;
    }
}

/// A trait expressing "this entity has genetic material."
/// HeritageBloodline::Member must satisfy this.
trait HasGenes {
    fn species(&self) -> Species;
    fn trait_set(&self) -> TraitSet;
}

#[clausewitz_scope("pop")]
impl HasGenes for Pop {
    #[clausewitz_field("species")]
    fn species(&self) -> Species { extern; }

    #[clausewitz_iterator("every_owned_trait")]
    fn trait_set(&self) -> TraitSet { extern; }
}
```

### Calling it from a patch

```rust
// crates/heritage/src/events/dynasty.mp

use heritage::Bloodline;
use heritage::HeritageBloodline;
use heritage::HasGenes;
use stellaris::events::CountryEvent;

#[country_event(id = "heritage.dynasty.0001", title = "A New Heir")]
fn new_heir_event(ctx: CountryEvent) {
    // ctx.scope() is statically typed as Country.
    let country: Country = ctx.scope();
    let bloodline: HeritageBloodline = country.heritage_bloodline();

    // The compiler knows:
    //   - country.create_pop() returns Pop (vanilla effect)
    //   - HeritageBloodline::Member = Pop
    //   - bloodline.add_member expects Pop
    // So this type-checks.
    let new_pop = country.create_pop(species = bloodline.matriarch().species());
    let idx = bloodline.add_member(new_pop);

    notify_player("Heir created at index {}", idx);
}
```

### Emitted Clausewitz (illustrative)

Compiler generates (in routing-correct directories):

```
common/scripted_effects/heritage/bloodline.txt:
  heritage_bloodline_add_member = {
      WHO_pop_id = "$WHO$"
      set_variable = {
          which = heritage_last_index
          value = heritage_bloodline_size
      }
      ...
  }

common/scripted_triggers/heritage/bloodline.txt:
  heritage_bloodline_contains = {
      ...
  }

events/heritage_dynasty.txt:
  country_event = {
      id = heritage.dynasty.0001
      title = "A New Heir"
      ...
      immediate = {
          create_pop = { species = ... }
          heritage_bloodline_add_member = { WHO = last_created_pop }
          ...
      }
  }
```

The author never wrote any of those file paths. The compiler routed
based on the decorators.

---

## 3. Goals + non-goals

### Goals

1. **Compile-time type safety** for everything Stellaris forces into
   "silent runtime failure" today.
2. **Abstraction barrier** so authors write against domain concepts
   (`Bloodline`, `Family`, `Marriage`) rather than vanilla Clausewitz
   primitives.
3. **Compiler-routed file output** so authors never name files /
   directories.
4. **Mechanical conflict resolution** via trait coherence (orphan
   rules) — two mods can't silently override the same thing.
5. **Refactoring safety** — rename a method, find every call.
6. **Gradual adoption** — coexists with YAML patches and direct
   Clausewitz; doesn't require a big-bang migration.
7. **Bootstrap from CWT** — `cwtools-stellaris-config` provides the
   vanilla type definitions for free. Mods extend.

### Non-goals

- **Replacing Clausewitz at runtime.** The output is still Clausewitz;
  the engine sees no difference.
- **Type-checking dynamic engine behavior.** `country_event_target:foo`
  resolution depends on save-game state we can't see at compile time.
  Best-effort with explicit "trust me" annotations.
- **Performance optimization.** Optimization is a backend / compiler
  concern (existing megapatch passes); the language's job is
  authoring ergonomics + correctness.
- **Replacing localisation `.yml` files.** Different parser, different
  problem space.
- **Replacing artwork pipelines.** Out of scope.
- **A general-purpose programming language.** Modlang is a DSL for
  Stellaris-shaped code. Adding general-purpose features only when
  authoring requires them.

---

## 4. Type system core

### 4.1 Traits

```rust
trait TraitName {
    // Associated types (Rust-style)
    type AssocName;
    type AssocWithBound: SuperTrait;

    // Associated constants
    const MAX_DEPTH: int = 5;

    // Required methods (no body)
    fn required(&self, x: Foo) -> Bar;

    // Default methods (have a body)
    fn default(&self) -> int { 0 }

    // Methods that mutate state — desugars to `&mut self` codegen
    fn mutating(&mut self, x: Foo);
}

// Super-traits (Rust syntax)
trait Subtype: Supertype { ... }
```

**Traits are pure abstraction.** No runtime object exists. Trait
methods are dispatched at compile time via the impl resolver. There
is no `dyn Trait` (no dynamic dispatch — Clausewitz can't express it
sensibly).

### 4.2 Impls

```rust
// Implement a trait for a type
impl TraitName for ConcreteType {
    type AssocName = SomeOtherType;
    const MAX_DEPTH: int = 7;

    fn required(&self, x: Foo) -> Bar { ... }
    fn mutating(&mut self, x: Foo) { ... }
}

// Inherent impl (no trait — just methods on a type)
impl ConcreteType {
    fn method(&self) -> int { ... }
}
```

**Coherence rule** (per Rust): you can `impl Trait for Type` only if
*either* the trait is local to your crate *or* the type is. This
prevents two mods from independently impl-ing the same trait for the
same type. When that's required (e.g. consolidating two patches),
authors write a third crate that explicitly merges.

**Open question O1**: do we want **specialization** (RFC 1210
behavior — `impl Foo for T` *and* `impl Foo for SpecificT`, with the
specific one winning when applicable)? This maps directly onto our
"specialized corvette inherits corvette compat" memory. Recommend
**yes, with the lattice rule** (Rust still calls this nightly, but
our smaller scope makes it tractable).

### 4.3 Associated types vs generics

```rust
// Associated type form
trait Bloodline {
    type Member: HasGenes;
    fn add_member(&mut self, m: Self::Member);
}

// Generic parameter form
trait Bloodline<M: HasGenes> {
    fn add_member(&mut self, m: M);
}
```

**Recommendation** (per Rust convention, validated by research-pending):
use **associated types** when there's "one canonical Member type per
implementor" (e.g. `HeritageBloodline::Member = Pop` always). Use
**generics** when implementors choose freely (rare in our domain).

### 4.4 Union types

Stellaris scopes naturally form unions: `Country | Federation` for
diplomatic actions, `Pop | Country | Leader` for species-typed
entities, `Country | Sector | Megastructure` for things that can
"have fleets."

```rust
// Type-level union (TypeScript-style discriminated union)
type Diplomat = Country | Federation;

// Function accepting either
fn declare_war(aggressor: Diplomat, target: Country) { ... }

// Pattern match to narrow
fn handle(d: Diplomat) {
    match d {
        Country(c) => c.country_specific_logic(),
        Federation(f) => f.federation_specific_logic(),
    }
}
```

**Open question O2**: tagged unions (Rust-style `enum`) vs untagged
(TypeScript-style `|`)? Tagged is safer (compiler forces match);
untagged is more ergonomic for "either of these, both have the
methods I need" usage. Recommend **untagged unions for trait
satisfaction** (`(Country | Federation): HasFleets` if both impl
HasFleets) **plus tagged enums for explicit discrimination** when
authors need it.

### 4.5 Subtyping via super-traits

```rust
trait Owned { fn owner(&self) -> Country; }
trait Pop: Owned { ... }     // Pop is-a Owned
trait Leader: Owned { ... }  // Leader is-a Owned

// Function works for any Owned thing
fn count_owner_resources(x: impl Owned) -> int {
    x.owner().total_resources()
}
```

This handles the "specialized corvette inherits corvette compat"
case via super-trait declaration. No nominal hierarchy needed.

### 4.6 Generics with trait bounds

```rust
fn process<T: HasFleets + HasResources>(thing: T) -> int { ... }

// Multiple bounds
where
    T: Bloodline,
    T::Member: HasGenes + HasMemories,
{
    ...
}
```

### 4.7 Sealed traits

```rust
// Mark trait as sealed — only this crate can impl it
sealed trait CoreScope { ... }
```

Useful for our `Country`, `Pop`, etc. core types — we don't want mods
to impl `Country` on something else.

### 4.8 Open questions for type system

- **O1**: specialization yes/no?
- **O2**: tagged enums + untagged unions, or one or the other?
- **O3**: lifetime-like annotations? Probably no — Clausewitz scopes
  don't have lifetimes per se.
- **O4**: const generics for things like `Trait<MAX = 5>`? Probably
  yes — trigger depth limits, max chain lengths, etc. all want this.
- **O5**: derive macros (`#[derive(HasFleets)]`)? Yes — boilerplate
  reduction is essential for vanilla wrapper types.

---

## 5. Module system

### 5.1 Module declarations

```rust
// crates/heritage/src/lib.mp — crate root
mod bloodline;       // → crates/heritage/src/bloodline.mp
mod family {         // inline module
    mod marriage;    // → crates/heritage/src/family/marriage.mp
    mod succession;
}

pub use bloodline::Bloodline;        // re-export for downstream users
pub use family::marriage::Marriage;
```

### 5.2 Visibility

- `pub` — visible to anyone
- `pub(crate)` — visible within current crate
- `pub(super)` — visible within parent module
- `pub(in path)` — visible within specified module
- (default) — private to current module

**Critical for our case**: visibility GATES what other mods can
extend / patch. Marking a trait `pub(crate)` means no other mod can
impl it. Marking it `pub` (and not `sealed`) means anyone can.

### 5.3 Use directives

```rust
use stellaris::scopes::{Country, Pop, Leader};
use heritage::{Bloodline, HasGenes};
use heritage::events::dynasty::*;       // glob import
use vanilla::common::traits as t;       // alias

// Import with renaming
use stellaris::scopes::Country as Empire;
```

### 5.4 Crates

A "crate" is a compilation unit. Roughly: one Stellaris mod = one
crate. Crate metadata lives in `Cargo.toml`-equivalent
(working name: `Modfile.toml` or `mod.toml`):

```toml
[mod]
name = "heritage"
version = "0.1.0"
description = "Dynasty + bloodlines for Stellaris"

[dependencies]
stellaris-vanilla = "4.0"           # auto-generated from CWT
ariphaos-unofficial = "1.2"         # other mods we depend on

[features]
default = ["dynasty"]
dynasty = []
intrigue = []
```

### 5.5 Cross-crate dispatch

```rust
// In crate `nsc3-compat`:
use stellaris::Country;
use nsc3::components::FleetCommand;

impl FleetCommand for Country { ... }   // Coherence: ok if either
                                         // FleetCommand or Country
                                         // is defined in our crate.
```

When `nsc3-compat` is included in the player's modlist, this impl
becomes available; otherwise it's silently absent. This is how our
language naturally expresses "this patch only applies if NSC3 is
loaded" — the impl simply doesn't exist without the dependency.

### 5.6 Open questions for module system

- **O6**: Cargo.toml-style metadata or simpler? Recommend
  Cargo-style — well-understood, mature semantics.
- **O7**: workspace concept (multi-crate projects)? Yes — Heritage +
  Megapatch are naturally a workspace.
- **O8**: feature flags syntax — copy Cargo's `[features]` exactly,
  or simplify? Recommend copy.

---

## 6. Trait → Clausewitz mapping (decorators)

Decorators tell the compiler "this Modlang item maps to that
Clausewitz construct." A canonical set:

### 6.1 Routing decorators (where output goes)

| Decorator | Maps to | Example |
|---|---|---|
| `#[scripted_effect("name")]` | `common/scripted_effects/<crate>/auto.txt` | `heritage_bloodline_add_member` |
| `#[scripted_trigger("name")]` | `common/scripted_triggers/<crate>/auto.txt` | `heritage_bloodline_contains` |
| `#[script_value("name")]` | `common/script_values/<crate>/auto.txt` | `heritage_dynasty_score` |
| `#[country_event(id = "x", ...)]` | `events/<crate>/country_events.txt` | event blocks |
| `#[trait_definition(id = "x")]` | `common/traits/<crate>.txt` | character traits |
| `#[building_type(id = "x", base_buildtime = 360)]` | `common/buildings/<crate>.txt` | etc. |
| `#[on_action("on_yearly_pulse")]` | `common/on_actions/<crate>.txt` | |
| `#[localisation(key = "...", lang = "english")]` | `localisation/english/<crate>_l_english.yml` | |
| `#[gfx_sprite_type(name = "...")]` | `interface/<crate>.gfx` | |

### 6.2 Scope decorators

| Decorator | Meaning | Example |
|---|---|---|
| `#[clausewitz_scope("country")]` | The `self` of an impl is this scope | `impl X for Y` where Y is country-scoped |
| `#[push_scope("planet")]` | Method body opens a `planet = { ... }` block | `every_owned_planet { ... }` |
| `#[scope_change("owner")]` | Method scope-transitions via owner | |

### 6.3 Field-access decorators

| Decorator | Meaning | Example |
|---|---|---|
| `#[clausewitz_field("name")]` | This trait method is a value lookup, not an effect | `pop.species()` → reads `species` |
| `#[clausewitz_variable("name")]` | This is a numeric variable on the scope | `country.heritage_bloodline_size` |
| `#[clausewitz_flag("name")]` | This is a boolean flag | `country.has_dynasty()` |
| `#[clausewitz_event_target("name")]` | Returns an event target | |

### 6.4 Conditional compilation

```rust
#[cfg(mod = "nsc3")]
impl FleetCommand for Country { ... }

#[cfg(any(mod = "nsc3", mod = "esc"))]
trait FleetCompat { ... }

#[cfg(not(mod = "ariphaos"))]
fn vanilla_specific_thing() { ... }
```

`cfg(mod = X)` resolves at modlist-resolution time. Items gated on
absent mods are pruned from the codegen entirely.

### 6.5 Inline emission decorators

When the impl body needs explicit Clausewitz that the codegen can't
generate from Modlang:

```rust
#[scripted_trigger("complex_thing")]
fn complex_thing(&self) -> bool {
    clausewitz! {
        any_owned_planet = {
            limit = {
                has_modifier = "thermal_decay"
                NOT = { has_modifier = "thermal_resistance" }
            }
            count > @[ self.threshold ]
        }
    }
}
```

The `clausewitz! { ... }` macro is an escape hatch — the body is
parsed by the G2 lexer + sanity-checked, but otherwise passed through
to codegen verbatim. This is the equivalent of `unsafe` in Rust:
necessary, marked clearly, audited at PR time.

### 6.6 Open questions for decorators

- **O9**: decorator syntax — `#[name(args)]` (Rust attribute style)
  or `@name(args)` (Python decorator style)? Recommend Rust style for
  consistency.
- **O10**: decorator argument grammar — same as Modlang expressions,
  or restricted? Recommend restricted (just literals + identifiers)
  for predictability.

---

## 7. File routing

Compiler decides where output lands based on:
1. **Crate name** — top-level directory (`crates/heritage/` →
   `<output>/heritage/...`).
2. **Module path** — subdirectory hint (`heritage::events::dynasty`
   → `events/dynasty/...`).
3. **Decorator type** — the Stellaris directory (`scripted_effects` →
   `common/scripted_effects/`).
4. **Override of routing** — `#[file("path/to/specific.txt")]` for
   when authors need exact control.

Conflict-free output guarantee: two crates can't write to the same
output path because:
- Crate names are namespace prefixes.
- Within a crate, the module hierarchy uniquely identifies each item.
- The codegen pass enforces "no two emit-sites land on the same
  file" → compile error if violated.

This *kills* the "which `_declarations.yml`?" question entirely.

---

## 8. Patches as trait method dispatch

Today: a "patch" is a YAML entry that says "override this item, or
inject this field." The semantic is positional and mod-aware.

In Modlang: a patch is just an impl. Two flavors:

### 8.1 Override (replace existing impl)

```rust
// vanilla declares:
trait HasFleets { ... }
impl HasFleets for Country {
    fn fleets(&self) -> List<Fleet> {
        clausewitz! { ... vanilla impl ... }
    }
}

// Mod overrides — but this won't compile due to coherence:
// "conflicting impl of HasFleets for Country in crate `mymod`"

// Instead: use #[patch] to mark intentional override
#[patch(target = "vanilla::Country::fleets", reason = "fix sort order")]
impl HasFleets for Country {
    fn fleets(&self) -> List<Fleet> {
        clausewitz! { ... patched impl ... }
    }
}
```

The `#[patch(target, reason)]` attribute is required for any impl
that conflicts with an existing one. The compiler keeps both visible
but the patched version wins. Multiple patches on the same target →
compile error, forcing consolidation.

### 8.2 Inject (extend existing impl with new methods)

```rust
// vanilla impl exists; we add a new method
#[patch_extend(target = "vanilla::Country")]
impl Country {
    fn heritage_bloodline_count(&self) -> int { ... }
}
```

Multiple `#[patch_extend]` are fine; each adds new methods. Conflict
arises only if two extensions define the same method name.

### 8.3 No more `_declarations.yml`

The whole `_declarations.yml` concept disappears. Patches are
type-checked impls. Every patch carries a `reason` string (lint can
enforce non-empty rationale).

---

## 9. Conditional compilation revisited

```rust
// Per-mod-presence
#[cfg(mod = "nsc3")] impl FleetCommand for Country { ... }

// Per-stellaris-version
#[cfg(stellaris >= "4.0")] fn use_v4_thing() { ... }

// Per-feature (within a crate)
#[cfg(feature = "intrigue")] mod intrigue;

// Combined
#[cfg(all(mod = "nsc3", feature = "advanced"))]
fn nsc3_advanced_path() { ... }
```

Resolved at modlist-bake time (when the player's modlist is known).
Items in inactive cfg branches are NOT emitted. This eliminates the
"compatibility patch only loads when both mods present" wiring we
currently do via separate compat folders.

---

## 10. Compiler architecture

```
┌─────────────────────────────────────────────────┐
│           MODLANG FRONTEND (NEW — M batch)      │
│                                                 │
│  .mp source files                               │
│       ↓                                         │
│  Lexer (M2)         — Modlang syntax            │
│       ↓                                         │
│  Parser (M3)        — produces typed AST        │
│       ↓                                         │
│  Type checker (M4)  — trait resolution +        │
│                       coherence checking        │
│       ↓                                         │
│  Module resolver    — use directives, crate     │
│                       deps, modlist baking      │
│       ↓                                         │
│  Codegen (M5)       — Modlang AST → emit-plan:  │
│                       (file_path, clausewitz_   │
│                        token_stream) tuples     │
└─────────────────────────────────────────────────┘
                       ↓
           ╔═══════════════════════════╗
           ║  CLAUSEWITZ TOKEN STREAM  ║
           ║   (G2 lexer-compatible)   ║
           ╚═══════════════════════════╝
                       ↓
┌─────────────────────────────────────────────────┐
│      MEGAPATCH BACKEND (EXISTING — G batch)     │
│                                                 │
│  Token stream from Modlang                      │
│       +                                         │
│  Token stream from Clausewitz source mods       │
│       +                                         │
│  Token stream from raw author Clausewitz        │
│       ↓                                         │
│  Conflict resolution (existing pass framework)  │
│       ↓                                         │
│  Output writer                                  │
│       ↓                                         │
│  .dist/ Clausewitz files                        │
└─────────────────────────────────────────────────┘
                       ↓
                Stellaris loads
```

**Key insight**: Modlang's codegen output IS a G2-compatible token
stream. We don't go through "emit text → re-lex." Codegen produces
tokens directly. This:

- Saves a parse pass.
- Preserves Modlang's source positions through to error messages.
- Lets the megapatch conflict-resolver consider Modlang output and
  raw-Clausewitz patches uniformly.

---

## 11. Migration plan

### Phase 1 — Modlang exists, nothing migrates yet (M2-M7)

- Lexer, parser, type checker, codegen built.
- Bootstrapped with vanilla types from CWT (M10).
- Heritage's most-frequently-touched modules ported as
  proof-of-concept (M8a).
- YAML patches still work, raw Clausewitz still works. Modlang
  coexists.

### Phase 2 — Heritage migrates (M8b-M8z)

- Heritage modules ported, one subsystem at a time. Each migration
  produces byte-identical compiled output (validation gate).
- Heritage-specific traits expand (Bloodline, Family, Marriage,
  Succession, Intrigue). Vocabulary stabilizes.

### Phase 3 — Megapatch's first-party patches migrate (M9)

- The patches we author for cross-mod consolidation move to Modlang.
- YAML loader marked deprecated.

### Phase 4 — YAML retirement (G6 — superseded by this batch)

- Final YAML purge. Loader removed.
- The G6 task (#42) folds into this phase.

### Phase 5 — Possible community adoption

- If Modlang proves valuable, publish as a standalone Stellaris
  modding tool. Probably warrants its own repo + LSP server.

---

## 12. Bootstrap

**Vanilla types** come from CWT (G7 task — #44). Every CWT
declaration generates a Modlang trait or impl:

```cwt
# cwtools-stellaris-config: scopes.cwt
scope = {
  name = country
  display = Country
  ...
}

trigger = "any_owned_planet" {
  scope = country
  ...
}
```

→ generated Modlang:

```rust
#[clausewitz_scope("country")]
pub struct Country;

impl Country {
    #[scripted_trigger("any_owned_planet")]
    pub fn any_owned_planet(&self, condition: impl Fn(Planet) -> bool) -> bool {
        extern;
    }
}
```

This is auto-derived by the M10 task. Authors never write vanilla
trait definitions by hand.

**Mod types** (NSC3, ESC, Heritage, etc.) come from authored
overlay CWT files OR native Modlang declarations. NSC3's
`FleetCommand` trait is either:
- Imported from a `cwtools-config` overlay shipped by NSC3-compat
  mod, or
- Written natively in Rust if the mod author opts in.

Heritage starts native (no CWT yet), grows its own type vocabulary
naturally.

---

## 13. Open questions for user judgment

These need decisions before M2 starts implementing.

### Naming
- **N1**: Language name? (Modlang / Forge / Trait / stellaris-rs / mp-lang / Heritage Lang / other)
- **N2**: File extension? (.mp / .hr / .tr / .forge / .modlang / other)
- **N3**: Crate metadata file? (Modfile.toml / mod.toml / Forge.toml)

### Type system
- **O1**: Specialization (RFC 1210 lattice rule) — yes/no?
- **O2**: Tagged enums + untagged unions, or one or the other?
- **O3**: Lifetime annotations — no, right? (recommend no)
- **O4**: Const generics for `Trait<MAX = 5>` — yes?
- **O5**: Derive macros (`#[derive(HasFleets)]`) — yes?

### Module system
- **O6**: Cargo.toml-style metadata, or simpler?
- **O7**: Workspace (multi-crate projects)?
- **O8**: Cargo-style features?

### Decorators
- **O9**: Rust attribute syntax (`#[x]`) or Python decorator (`@x`)?
- **O10**: Decorator arg grammar — restricted or full Modlang exprs?

### Type system (advanced)
- **O11**: Higher-kinded types (e.g. `trait Functor<F<_>>`)? Probably
  no — Stellaris doesn't model anything that needs them.
- **O12**: Trait objects (`dyn Trait`) — explicitly reject?
  Recommend yes (Clausewitz can't dispatch dynamically).
- **O13**: Async / lifetime / GAT analogues — no, right?

### Compilation model
- **O14**: Strict total typing (Elm-style, no escape hatch) or
  gradual (TypeScript-style `unknown`/`any`)? Recommend gradual with
  the escape hatch being `clausewitz! { ... }` — explicit, marked,
  auditable.
- **O15**: Incremental compilation strategy — per-crate or
  per-module?
- **O16**: LSP support eventually — yes? When?

### Bootstrap
- **O17**: How aggressive should auto-derive from CWT be? Generate
  every trigger/effect as a method, or only the ones used? Recommend
  on-demand (generate only when imported).
- **O18**: Which subset of vanilla types do we hand-curate vs
  auto-generate? Recommend: hand-curate the ~30 core scopes
  (Country, Pop, Leader, Planet, etc.) for the right ergonomics;
  auto-generate the long tail.

### Migration
- **O19**: Acceptable risk threshold for byte-identical-output
  validation? 100% byte-identical, or "semantically equivalent"?
  Recommend 100% byte-identical for Phase 1; relax to semantically
  equivalent for Phase 2+.
- **O20**: Heritage migration order — what subsystem first?
  Recommend: bloodline (smallest, well-isolated, the original test
  case for this design).

---

## 14. Cross-references

### Research dispatched (background) — integrated in M1.5

- Rust trait system → `.cache/reviews/2026-04-16/rust-traits-research.md`
  (impl resolution, coherence/orphan rules, associated types vs
  generics, specialization, module visibility).
- Typed DSLs to dynamic targets →
  `.cache/reviews/2026-04-16/typed-dsl-prior-art.md`
  (TypeScript / Elm / PureScript / Haxe / etc. lessons; gradual vs
  total; structural vs nominal; interop story; error UX).

### Project context

- **Supersedes**: #41 (G5 — patch DSL parser) and reframes #42
  (G6 — YAML retirement) as the migration end-state.
- **Builds on**: #33 (G2 — Clausewitz lexer) as backend, #44
  (G7 — CWT loader) as bootstrap source.
- **Independent of**: G3a-G3f, G4a, G4b — those are pure backend work,
  unaffected.
- **Memory references**:
  - `feedback_upward_compatibility` — specialization
  - `feedback_consolidate_not_choose` — coherence enforces this
  - `feedback_extensible_compiler` — Modlang IS this taken to its
    logical conclusion
  - `feedback_strict_build_linters` — gradual typing escape hatch
    must be lint-flagged
- **Related design docs**:
  - `2026-04-16-clausewitz-grammar.md` (G1) — backend grammar
  - `compiler-passes.md` — pass framework Modlang's codegen plugs into

### Source tree pointers (when M2 starts)

- New top-level: `tools/megapatch/megapatch_compiler/modlang/` —
  frontend (lex, parse, typecheck, codegen).
- New top-level: `crates/` — Modlang source crates (heritage, etc.).
- Existing `tools/megapatch/megapatch_compiler/` — backend
  (passes, conflict resolution, output).

---

## 15. What this doc is NOT

- A complete language specification. Many syntactic details are
  unspecified — they emerge during M2-M5.
- A commitment to ship in any timeframe. Estimate: 6-12 months of
  focused work to reach Phase 2 (Heritage migrated). Should be
  pursued as a long-running parallel track to the G batch.
- A claim that this is the right answer. There are 20 open questions
  in §13 — read them, redirect any that look wrong before M2 starts.

---

## 16. Realistic scope warning

This doc proposes building a **new programming language with a real
type system**. That's an order of magnitude more work than the G
batch. Every feature in §4-§9 is months of design + impl + test.

Mitigations:
- Rust-inspired so we can crib heavily from Rust's design + literature.
- Compile target is constrained (Clausewitz, not arbitrary backends).
- Type system can be much simpler than Rust's (no lifetimes, no async,
  no const fn, etc.).
- Backend is already built (megapatch pipeline).

Realistic milestones:
- M1 (this doc): days
- M1.5 (research integration): days
- M2-M5 (lexer, parser, typecheck, codegen — minimal): 4-8 weeks
- M6-M7 (module system, file routing): 2-4 weeks
- M8 (Heritage migration begins): ongoing
- M9-M10 (patches in Modlang, CWT bootstrap): 4-8 weeks
- Total to Phase 2: ~6 months at part-time pace

This is a multi-month commitment. The user has explicitly chosen this
path knowing the cost. Document the cost so future-self doesn't
underestimate.

---

---

## 17. Research consolidated — Rust trait system

Full report: `.cache/reviews/2026-04-16/rust-traits-research.md`. Most actionable findings for our case:

### 17.1 Trait declaration ports cleanly
Strip lifetimes — everything else (associated types, associated constants, default methods, supertraits, where-clauses) survives the port to a Clausewitz-targeted language without modification.

### 17.2 Impl resolution simplifies for us
Rust's algorithm: build candidate receiver types (the value type + its derefs + `&T`/`&mut T`), search inherent methods first, then in-scope trait methods. **For us**: no deref chain exists in Clausewitz, so the algorithm collapses to "inherent first, then trait." Disambiguation: `<Type as Trait>::method()` syntax works identically.

### 17.3 Orphan rule is the conflict-resolution mechanism
`impl Trait for Type` is legal only if your crate declared either the Trait or the Type. This is **the** mechanism for preventing two mods from silently impl-ing `Patchable for vanilla::Country` with conflicting bodies. When consolidation is required, a third crate that owns *neither* the trait nor the type must declare an explicit "override impl" with documented reason.

### 17.4 Associated types vs generics — clear decision rule
Use `type Item;` (associated) when there's exactly **one** answer per implementor (e.g., `impl Bloodline for HeritageBloodline { type Member = Pop; }`). Use `<T>` (generic) when implementors choose freely (`impl HasModifier<Unity> for Empire` *and* `impl HasModifier<Energy> for Empire` are both valid).

### 17.5 Specialization: NOT recommended
Still nightly-unstable in Rust due to associated-type soundness holes. **For "ascendant corvette inherits corvette" use supertrait chaining** (`trait AscendantCorvette: Corvette`) instead. Cleaner, sound, no lattice-rule complexity.

### 17.6 `#[derive]` is high-value boilerplate elimination
A `#[derive(HasFleets)]` macro auto-generates the impl from the type's fields. Directly applicable: vanilla wrapper types should auto-derive most trait impls from CWT-declared metadata.

### 17.7 Sealed traits via private supertrait pattern
`pub trait Core: private::Sealed {}` in a `mod private { pub trait Sealed {} }` block. External code can't name `Sealed`, so can't implement `Core`. Use for system traits that mods must not implement independently (e.g., the core scope traits).

### 17.8 No object safety needed
We're explicitly rejecting `dyn Trait` (Clausewitz can't dispatch dynamically). That removes a whole class of design constraints (Self-return-types, generic methods, etc.) and simplifies the type checker materially.

### 17.9 Module visibility is the gate
`pub`, `pub(crate)`, `pub(super)`, `pub(in path)` — all directly applicable. Visibility GATES what other mods can extend / patch. Marking a trait `pub(crate)` means no other mod can impl it. This controls "is this mod's API extensible?" mechanically.

---

## 18. Research consolidated — Typed DSLs to dynamic targets

Full report: `.cache/reviews/2026-04-16/typed-dsl-prior-art.md`. Most actionable findings for our case:

### 18.1 The interop story is THE make-or-break decision
TypeScript won (50%+ of new web projects) because it could absorb existing untyped JS incrementally via `.d.ts` ambient declaration files. Elm is loved but isolated because it can't. **For us**: the right model is **ambient declarations** (TypeScript `.d.ts` style / Haxe `extern` blocks), NOT FFI, NOT `any`, NOT ports.

### 18.2 Total typing, not gradual
The domain has no legitimate use for `any` — a Country scope is never "anything." The agent recommends **total typing** with NO TypeScript-style escape hatch. This tensions with the strawman's `clausewitz! { … }` macro escape — but that escape is fine IF restricted to "raw Clausewitz that we still parse + sanity-check, but type-check best-effort." The key is the escape isn't a free-form `any` propagating types out into typed code.

### 18.3 Nominal scopes, structural data
**Country** and **Fleet** must NOT structurally unify even if both have `.owner`. Nominal types prevent semantic confusion. But for *data values* (numbers, strings, lists), structural compatibility is fine. **Use union types** (Ceylon-style `Planet | Country`) for multi-scope validity — this exactly captures Stellaris' natural shape ("any of these scopes can do X").

### 18.4 Auto-generate declarations from `trigger_docs`
Stellaris ships `script_documentation/` logs (via the `trigger_docs` debug command) that enumerate every effect, trigger, and scope constraint. **Bootstrap by parsing these into typed declarations** — not by hand, not by relying solely on cwtools-stellaris-config (which trails the game by months). Re-parse on every Stellaris update to keep current.

### 18.5 Scope stack as a typed structure
THIS / ROOT / PREV / FROM are typed stack references, not untyped keywords. The compiler tracks the scope stack at every code point. **This is the novel core challenge** — no prior language we surveyed solves exactly this. Closest parallels: Hindley-Milner inference for the implicit `this` in OO calls, but with a more constrained scope graph.

### 18.6 Cross-mod dependencies via Kotlin `expect`/`actual`
`expect modifier stability_boost` declares "I require this thing to exist; the build system verifies it appears in the load order." `actual` declarations come from another mod (or vanilla). Cleaner than Cargo features for our use case because Stellaris mods are discovered runtime-style (modlist), not compile-time-style (Cargo).

### 18.7 Elm's error-message philosophy
First-person, domain-language, fix-suggesting. *"You wrote `add_modifier` on a Fleet, but `add_modifier` is only available on things that have modifiers. Fleets don't have modifiers — did you mean `add_fleet_modifier`?"* — vs Haskell's *"No instance for `HasModifiers Fleet` arising from a use of `add_modifier`."* Test every error message with a non-programmer mod author.

### 18.8 Per-language prior-art lessons (one each)
- **TypeScript**: incremental adoption + `.d.ts` interop is the killer feature.
- **Elm**: total typing + no escape hatch IS feasible; communities that adopt it stay adopted.
- **PureScript**: type classes (= Rust traits) work in dynamic targets; the interop pain is real.
- **Haxe**: cross-target codegen via `extern` is mature; useful reference for our auto-generated declarations.
- **ReScript**: prioritized JS interop ergonomics over ML purity; succeeded.
- **Kotlin**: `expect`/`actual` mechanism is the right shape for cross-mod requirements.
- **Ceylon**: union types pioneered here; directly applicable.
- **roblox-ts**: typed-language-on-game-engine prior art exists; mod community has tried this shape before (small but successful).

### 18.9 Seven open questions raised by the agent
1. Escape-hatch granularity (none / per-block / per-line)
2. Who owns third-party mod declarations (us / mod author / community overlay)
3. Feature-flags vs `expect`/`actual` for mod deps
4. Scope inference depth (how aggressive)
5. Versioned vs version-neutral declarations
6. Error message style (Elm-friendly vs technical)
7. PREV chain depth limits (4 hops vs deeper)

---

## End of design startpoint

This file is reference. Actual design proceeds interactively in a separate `2026-04-16-mod-language-design-draft.md` once syntax direction is settled with the user.
