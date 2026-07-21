# Resolve pass name resolution: scope-chain walk vs flat shadow-stack

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 4M references, 12 nested scopes x 8 bindings, refs biased to inner scopes,
5-run best. Zig 0.16.0, aarch64. Artifacts: `resolve.zig`, `resolve.csv`.
**Settles:** the `vehje-resolve` pass name-resolution strategy (converting each variable reference to its binder),
the last named compile-side pass not yet sized. Settles the scope-lookup fork.

## Why this probe

The resolve pass converts each variable reference (an interned name id) to its binder, a (scope-depth, slot)
pair naming the nearest enclosing scope that binds the name. The resulting binder is what the interpreter reads as
a direct slot index (Var access is then O(1), already benched at ~0.5 ns as a register/slot read), so resolution
is a compile-side pass that makes runtime variable access free. Its throughput is a compile-time (dev-iteration)
cost, and the lookup strategy is a real fork:

- **Linear scope-chain walk:** for each ref, walk enclosing scopes innermost-out, linear-scanning each scope's
  name list. O(depth x scope-width) per ref.
- **Hashed scopes:** each scope carries a name-to-slot hash; walk scopes but hash within each. O(depth) per ref.
- **Flat shadow-stack symbol table:** one hash from name to a stack of binders; push on scope-enter, pop on
  scope-exit; each ref is one hash lookup to the top (innermost) binder. O(1) per ref, O(1) amortised push/pop.

## Results (ns per reference)

12 nested scopes, 8 bindings each, references biased 60% to the innermost 3 scopes (the local-heavy real shape):

| strategy | ns/ref | throughput | complexity |
|---|---|---|---|
| linear scope-chain | 13.05 | 77 M-ref/s | O(depth x width) |
| hashed scopes | 5.43 | 184 M-ref/s | O(depth) |
| flat shadow-stack | **0.91** | **1104 M-ref/s** | O(1) |

## The finding: use a flat shadow-stack symbol table (O(1) per reference), 6x to 14x faster than scope-chain walks

The flat shadow-stack crushes the scope-chain strategies: 0.91 ns/ref versus 5.43 ns (hashed scopes) and 13.05 ns
(linear). The reason is asymptotic: the shadow-stack resolves each reference in a single hash lookup to the
current top binder of that name, independent of how deeply the reference is nested, while both scope-chain
strategies pay O(depth) (walking outward scope by scope). Even hashing within each scope only removes the
scope-width factor, not the depth factor, so it is still 6x slower than the flat table.

The shadow-stack works by maintaining one hash `name -> stack-of-binders`: entering a scope pushes its bindings
(each name's new binder onto its stack), exiting pops them. A reference is one lookup to the top of the name's
stack. The push/pop cost is O(1) amortised per binding (each binding is pushed once and popped once over the whole
resolve), so the total resolve is O(references + bindings), dominated by the O(1)-per-reference lookups.

So the resolve pass should use the flat shadow-stack:

- Each reference resolves in ~0.91 ns regardless of nesting depth, so a program with 500K references resolves in
  ~0.45 ms, negligible on the compile side.
- The scope-chain walks (both variants) pay O(depth), which compounds for deeply-nested code (deeply-nested
  templating blocks, nested comprehensions), exactly where they are slowest and the flat table's O(1) matters
  most.
- The output binder (depth, slot) makes runtime Var access a direct slot index (O(1), ~0.5 ns, already benched),
  so the compile-side resolve buys free runtime variable access.

## Design impact

- The `vehje-resolve` pass uses a flat shadow-stack symbol table (one hash `name -> binder stack`, push/pop on
  scope enter/exit), not a per-scope name list or per-scope hash. Resolution is O(1) per reference, ~0.91 ns, and
  independent of nesting depth.
- Resolution converts names to (depth, slot) binders once at compile time, so the runtime never does name lookup
  for statically-bound variables (Var access is a slot read). This is the compile/runtime split applied to name
  resolution: the names live at compile time, the residual carries slot indices.
- This is distinct from dynamic field access (the Project bench): field access on a dynamic-shape record needs a
  runtime name lookup (the inline-cache / hash there), because the shape is not known at compile time; but lexical
  variable references ARE statically bound, so they resolve to slots at compile time and never pay a runtime
  lookup. The two are the static (resolve-time) and dynamic (runtime) halves of name-to-slot mapping.
- Interned name ids (u32) are compared, not strings (the interner is the prerequisite, already sized), so both the
  hash and the compare are integer operations.

## Boundary

12-deep scopes with 8 bindings each; deeper nesting widens the scope-chain walks' disadvantage (their O(depth)
grows) while the flat table stays flat, so the flat table's lead grows with nesting. The bench measures the
lookup throughput assuming the table is maintained; the push/pop maintenance is O(1) amortised per binding and not
separately timed (it is dominated by the reference lookups in any program with more references than bindings,
which is the normal case). Shadowing (an inner scope rebinding an outer name) is handled by the stack (the top
binder wins, popped on scope exit); the bench's distinct-name-per-scope setup does not stress shadowing, but the
stack structure handles it by construction. Forward references and mutually-recursive bindings need a two-pass
resolve (collect then resolve), which uses the same flat table.

## Artifacts
- `resolve.zig` (linear, hashed-scope, and flat-shadow-stack resolution over nested scopes), `resolve.csv`.
