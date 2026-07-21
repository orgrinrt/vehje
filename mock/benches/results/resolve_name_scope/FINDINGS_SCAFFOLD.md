# Resolve name resolution (harness): flat shadow-stack vs hashed-per-scope vs linear scope-chain

Scaffold. The main agent fills the medians and the cost-model line after the serialized bench run.

## What this measures

The `vehje-resolve` pass converts each variable reference (an interned name id) to its binder, a (depth,
slot) pair naming the nearest enclosing scope that binds the name. Three lookup strategies over an identical
program of 12 nested scopes x 8 bindings, references biased 60% to the innermost three scopes (the local-heavy
real shape):

- `resolve_flat` (baseline): flat shadow-stack symbol table, one open-addressed hash name -> current binder,
  O(1) per reference.
- `resolve_hashed`: hashed-per-scope, walk enclosing scopes innermost-out with a hash lookup in each, O(depth).
- `resolve_linear`: linear scope-chain walk, walk scopes innermost-out linear-scanning each name list,
  O(depth x width).

Every strategy resolves each reference to the byte-identical binder `((DEPTH-1-d) << 8) | slot`; the harness
cross-validates that (offline check confirmed all three yield identical accumulators).

## The audit defect this fixes

The standalone `resolve-name-scope/resolve.zig` ran outside the harness with a hand-timed 4M-ref loop and a
header-only CSV, so its numbers were not harness-reproducible. The earlier `hx_resolve` port had two further
defects, both fixed here: it rebuilt the lookup table INSIDE the timed region (measuring construction, not
lookup), and its variants produced DIFFERENT binder encodings (flat returned the bare name, linear a packed
depth/slot), so they were not cross-validatable. Here the scope structure and reference stream are built once
via `OnceLock` (runtime state, not const-foldable) outside the timed region, the reference index is folded with
the FFI input byte so nothing hoists, and every strategy emits the identical binder.

## Result (fill after run)

Medians, normalised against `resolve_flat` (mode = subtract):

| n | resolve_flat (base) | resolve_hashed | resolve_linear |
|---|---|---|---|
| 64 | | | |
| 256 | | | |
| 1024 | | | |
| 4096 | | | |
| 16384 | | | |

Expected shape (from the prior standalone run, to confirm or refute): flat decisively fastest (O(1) per ref),
hashed several times slower (O(depth)), linear slowest (O(depth x width)). The prior standalone measured 0.91 /
5.43 / 13.05 ns/ref respectively.

## The finding (fill after run)

Cross-validation: [pass/fail]. The name resolution pass should use [strategy], because [asymptotic reason].

## Cost-model sanity line (fill after run)

At n=16384, [median] for 16384 references is [ns/ref], about [N] cycles at ~3.2 GHz (Apple Silicon) for a
[hash probe / scope walk]. Time scales [linearly?] with N (from [n=64] to [n=16384]), confirming the timed work
is the resolution and not a hoisted constant.

## Boundary

12-deep scopes with 8 bindings each; deeper nesting widens the O(depth) strategies' disadvantage while the flat
table stays flat. Push/pop maintenance of the shadow stack is O(1) amortised per binding and not separately
timed here (the lookup dominates any program with more references than bindings). Shadowing is handled by the
stack structure by construction; the distinct-name-per-scope setup does not stress it.
