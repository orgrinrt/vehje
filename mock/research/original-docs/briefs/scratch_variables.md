# Brief — #124 X1: let-binding / scratch-variable lowering

**Status**: DRAFT 2026-04-19. Awaiting implementation. This brief
captures the design so the follow-up implementation commit can be a
straightforward translation rather than improv.

## Problem statement

`let x = expr;` today transpiles to a comment:

```
# let x = … (scratch-variable lowering pending)
```

Authors can't build intermediate values. Any expression that needs a
reused sub-result must be inlined, which either duplicates work or
requires factoring into a separate fn. Both are strictly worse than
the let-binding the author wrote.

## Clausewitz reality check

Clausewitz has no author-facing local variables. What it has:

| Scope  | Analogue                       | Clear semantics                |
|--------|--------------------------------|--------------------------------|
| Country| `@country_<name>` variables    | lives on country entity        |
| Country| `<flag>_flag` country flags    | set/clear via flag prims       |
| Country| `event_target:<name>`          | set/clear via event_target fn  |
| Global | `@<name>` scripted_variables   | lives in constants table       |

Real mod authors simulate locals with two idioms:
1. **Variables on the current scope** — e.g. `set_variable = {
   which = temp_value which = 3 }` writes `@temp_value` on whichever
   scope is current; cleared on scope exit by convention.
2. **Event targets** — `save_event_target_as = { name = hit value =
   some_scope }`; cleared with `clear_saved_event_target`.

Both are scope-attached. Neither is typed in the engine — an author
error (using variable where event_target is needed) produces a
broken-but-silent runtime.

## Allocation strategy

**Pool per fn body.** One allocator per `_lower_fn` call. Slots are
numbered from zero; name is `_scratch_<fn>_<n>`. Slot counter resets
at every fn entry — collisions across fns can't happen, since each
emits its own named Clausewitz item.

**Type → slot kind mapping:**

| Clause type | Slot kind       | Clausewitz primitive          |
|-------------|-----------------|-------------------------------|
| `bool`      | FLAG            | `set_country_flag`            |
| `i32`       | VARIABLE        | `set_variable`                |
| `f64`       | VARIABLE        | `set_variable` (float)        |
| scope type  | EVENT_TARGET    | `save_event_target_as`        |
| struct/enum | VARIABLE (ref) | emit comment, reject at check |

Non-primitive `let` bindings (tuples, closures, user structs) stay
commented out in v1 — the storage router can't pick a backend for
them. Typecheck diagnoses with `CL_SCRATCH_UNREPRESENTABLE` when an
author tries.

## Scope lifetime

**Slot releases on block exit.** A `let` inside an inner block is
only live to the end of that block. Implementation: allocator has a
stack of "current pool scope" markers, push at block entry, pop at
exit — emitted clears come from the diff between levels.

Clear emit per slot kind:

| Slot kind    | Clear primitive                              |
|--------------|----------------------------------------------|
| FLAG         | `clear_country_flag = <slot-name>`           |
| VARIABLE     | *skipped* — variables drift unless set; we  |
|              | don't bother clearing (engine idiom)         |
| EVENT_TARGET | `clear_saved_event_target = <slot-name>`     |

Variables aren't cleared because engine convention doesn't clear
them either — subsequent reads that don't happen are a logic bug,
not a cleanup one. Flags and event-targets DO get cleared because
leaking them costs cycles (flags) or state (event-targets).

## Naming + collision avoidance

Per-fn counter: `_scratch_<transpiled_fn_name>_<n>`. The fn name
already carries the crate+module prefix (`heritage_bloodline_gen_1`),
so cross-crate collisions are avoided by construction.

For macro-expanded code, the allocator seeds its counter from a
per-macro-invocation hash so two separately-invoked scratch! macros
don't collide. (This is the `std::scratch` integration.)

## Compiler-internal macro — `scratch!(name, type)`

Authors never call `scratch!` directly. The compiler's `let`
lowering synthesises an invocation that the scratch pass picks up:

```
let x: i32 = 5;
// expands to
scratch!(_scratch_f_0, i32);
set_variable = { which = _scratch_f_0 value = 5 }
// subsequent references to `x` rewrite to `_scratch_f_0`
```

This keeps the complexity contained in one pass — nothing else in
typecheck or transpile knows about scratch slots.

## References: let → use

The transpiler maintains a `{ident_name → slot_name}` mapping inside
the fn body. When lowering a subsequent expression, any `PathExpr`
whose single segment matches a bound let-name rewrites to the
slot's Clausewitz reference form (`@_scratch_f_0` for a variable,
`event_target:_scratch_f_0` for an event-target).

This rewrite happens during `_lower_expr` — no second pass. The
scratch allocator threads through as an instance attribute of the
Transpiler, scoped per fn call.

## Non-scope (entity-held) bindings

`let country = event_target:heir` doesn't allocate — the author is
just aliasing an existing event_target. The binding maps directly
to the existing target. The allocator tracks these as "aliases" to
distinguish from fresh allocations (aliases need no cleanup).

## Test matrix

1. `let x = 5; let y = x + 1;` — two set_variable against slots.
2. `let temp = self.heir; if cond { self.heir = temp; }` — scratch
   event_target, cleared on block exit.
3. Nested scopes: `{ let a = 1; { let b = 2; } }` — b slot freed at
   inner brace; a still live.
4. `let x = 5; let x = x + 1;` — fails the no-shadow check; not
   relevant to scratch.
5. Macro-generated scratch! invocation — counter seeded from macro
   hash, no collision with author's own `let`.

## Open questions

- **Mutability**: `let mut x = 5; x = x + 1;` — the second
  assignment overwrites the slot. Does the allocator need to know
  mut vs immut, or is this purely lexical? v1 answer: lexical; any
  assignment to a `let`-bound name rewrites to set_variable against
  the original slot, regardless of mut.

- **Type inference for `let`**: `let x = some_fn();` needs the fn's
  return type to pick the slot kind. This couples #124 to #119 T1.
  v1 fallback: when the type is unresolved, emit the existing
  comment placeholder + a `CL_SCRATCH_TYPE_UNRESOLVED` warning.

- **Cross-fn aliasing**: a `let x = 5;` in fn A doesn't leak to fn
  B. Each fn is its own Clausewitz item with its own slot namespace.
  No special handling needed.

## Rollout

1. Brief review (this commit).
2. Allocator + scope-stack infrastructure (follow-up commit).
3. `_lower_let` real implementation (follow-up commit).
4. Slot-reference rewriting in `_lower_expr` (follow-up commit).
5. Clear emission on block exit (follow-up commit).
6. End-to-end tests + heritage crate dogfooding (follow-up commit).

Each step ships independently with tests green.
