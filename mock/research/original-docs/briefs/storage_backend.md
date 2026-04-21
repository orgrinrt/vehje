# Brief — #57 M13: storage-backend diversity

**Status**: APPROVED 2026-04-18. Ready to implement.
**Supersedes**: earlier draft that invented `impl Bind[T] for S` syntax
(rejected — never part of the design; DESIGN.md §8.2, §14 are clear).

## Locked decisions

- **Attribute spelling**: `#[repr(flag)]`, `#[repr(variable)]`,
  `#[repr(event_target)]`, etc. Switched from `bind` to `repr` because
  it's semantically accurate (representation choice) and avoids
  overloading the `Bind` trait name for a concept that isn't about
  implementing `Bind`. Matches Rust's `#[repr(transparent)]`
  convention.
- **Default for `bool`**: `flag` (flags are cheaper; engine-idiomatic).
  Author can force numeric with `#[repr(variable)]`.
- **Ambiguity policy**: pick one + warn with a suggested
  `#[repr(...)]` to make it explicit. Never break the build on a tie.
- **Manifest format**: flat table under `[storage]`, keyed by
  `crate::module::Struct::field`. One row per decision.
- **`#[repr(...)]` at struct level: rejected**. Struct-level bind
  target is the colon syntax (`struct X : Country`). Attribute at
  struct level would duplicate that.

## Audit of the actual design

The prior brief fabricated an `impl Bind[T] for Country` syntax.
That's not what the language designs. What DESIGN.md actually says:

- **`Bind` is a sealed marker trait on bind-target types**, not on
  field types. Only `Country`, `Pop`, `Planet`, `Registry<T>`,
  `Singleton`, etc. implement `Bind` — and all of those impls live
  in `std`.
- **Structs choose a bind target via colon syntax**:
  `struct Bloodline : Country { mut gen: i32 = 0, ... }`. Bloodline
  is bound to Country; its fields auto-generate as
  `@<crate>_bloodline_<field>` Clausewitz variables on the bound
  Country entity.
- **The compiler auto-routes each field to the optimal Clausewitz
  primitive** based on the field's type, the bind target, and access
  patterns. §14.0: "The compiler picks the optimal engine workaround
  per bind target (hidden country, fleet-name slots, paired variable
  arrays, etc.). All implementation detail. Tenet 11."
- **`#[prefer(...)]` is an emit hint** (from chunks 2 + §9.4.6), not
  a binding directive. It biases the auto-pick when tie-breaking.
- Tenet 11 (hide engine, surface intent): authors don't pick storage
  primitives. The compiler is responsible for landing intent on the
  best available engine construct.

So M13's actual job is: **make the auto-router smart enough to
pick the right primitive per field, plus expose `#[repr(...)]` as an
author override when the auto-pick guesses wrong, plus manifest-lock
the decisions for ABI stability**.

## Goal

1. **Per-field auto-routing**: given a struct's bound target + each
   field's type, pick the optimal Clausewitz primitive.
2. **Author override**: `#[repr(flag)]` on a field forces a specific
   backend; `CL_REPR_MISMATCH` if the requested backend can't carry
   the field's type.
3. **Manifest lock**: once a field's backend is picked for a published
   crate, the manifest records it; rebuilds never silently change
   encoding even if the heuristic improves (Tenet: save/mod-version
   compatibility).
4. **Diagnostics for ambiguity**: when two backends are equally valid,
   surface a soft warning with guidance on which `#[repr(...)]` to
   add.

## Backend catalog (v1)

Primary Clausewitz primitives the router picks among:

| Backend | Shape | When picked |
|---|---|---|
| `flag` | marker (no value) | field type is `Marker`, unit-like, or `bool` where the "false" case is indistinguishable from "unset" |
| `variable` | numeric | field type is `i32`/`f32` and bind target is an entity |
| `scripted_variable` | global numeric | field type is numeric and bind target is `Singleton` or `Registry<Self>` with global semantics |
| `event_target` | entity ref | field type is a scope type (`Country`, `Pop`, etc.) |
| `scripted_list` | collection of entity refs | field type is `Vec<Scope>` |
| `StringStorage` (stdlib workaround) | string | field type is `String` — routes through the stdlib-managed string hack (DESIGN.md §8.3) |
| paired-flag-plus-value | any `Option<T>`/`Result<T, E>` | per §9.4 encoding-registry, inherits the chosen backend of `T` plus a flag |

The list is extensible — adding a new primitive later means adding a
new router arm + a manifest-compatible emit path. No trait gymnastics.

## Mechanism: where the decision lives

One new pass — `StorageRouter` — runs post-typecheck, pre-transpile.
For each `StructDecl`:
1. Resolve the bind target (the type after `:`).
2. For each field, examine the field's type + the target's kind +
   any `#[repr(...)]` / `#[prefer(...)]` attributes.
3. Pick a backend per the heuristic in the catalog above.
4. Validate: if the author wrote `#[repr(flag)]` but the field is an
   `i32`, emit `CL_REPR_MISMATCH`.
5. Record the decision on the field's AST node (`field.storage_backend`
   is a new attribute the transpile pass reads).
6. On first build: emit the decision into the crate manifest under
   `[storage]`. On subsequent builds: read the manifest and enforce
   the recorded decision unless a `#[migrate(...)]` attribute
   explicitly authorizes a re-pick.

Transpile reads `field.storage_backend` and emits the appropriate
Clausewitz primitive at field read/write sites.

## The `#[repr(...)]` attribute shape

Used as an author override on individual fields:

```clause
struct Bloodline : Country {
    mut gen: i32 = 0,

    #[repr(flag)]
    mut is_elder: bool = false,       // force flag encoding; compiler
                                      // would also pick this, but make
                                      // it explicit for contract stability

    #[repr(event_target)]
    mut heir: Option<Pop> = None,     // force event_target even though
                                      // Option<Pop> would normally route
                                      // through paired-flag-plus-value
}
```

Invalid combinations (`#[repr(flag)]` on an `i32`) → `CL_REPR_MISMATCH`
with a diagnostic naming the valid choices for the field's type.

`#[repr(...)]` at the struct level is rejected in v1 — bind-target
choice is the colon-syntax, not an attribute. An attribute there
would duplicate a syntactic construct.

## Scope of #57 v1

In:
- `StorageRouter` pass (post-typecheck, pre-transpile).
- Auto-pick heuristic covering the catalog above.
- `#[repr(backend)]` author override on fields.
- `#[prefer(backend)]` soft-hint on fields (tie-breaker only).
- Manifest emission under `[storage]`; read on subsequent builds;
  `CL_STORAGE_DRIFT` diagnostic when code changes would force a
  different pick than the manifest has locked.
- Codegen wiring: transpile reads `field.storage_backend` and emits
  the right Clausewitz for reads + writes.
- Diagnostics: `CL_REPR_MISMATCH`, `CL_REPR_AMBIGUOUS`,
  `CL_STORAGE_DRIFT`.

Out (deferred):
- `#[migrate(...)]` for intentional backend re-picks + sham handlers.
  Tracked separately — plugs into existing rename/sham machinery.
- Custom author-defined backends (stdlib-only in v1).
- Cross-crate coherence for `Bind` target impls. Related to the
  existing cross-crate orphan-rule xfail; not unblocked by this
  task.
- `scripted_list` iteration ergonomics (the user-facing loop API
  over stored collections) — ship storage first, iteration next.

## Open questions — all resolved 2026-04-18 (see Locked decisions at top)

## Verification

- Per-backend unit tests: each catalog entry → correct Clausewitz emit
  for read and write.
- Heuristic tests: for each (field type, bind target) pair, router
  picks the documented backend.
- Author-override tests: `#[repr(X)]` forces X; mismatched pair → error.
- Manifest tests: first build writes; second build reads + respects;
  code change that would force a different pick → `CL_STORAGE_DRIFT`.
- Integration: Bloodline (already ported per M11) compiles with the
  router active, produces byte-stable Clausewitz against a golden
  fixture.
