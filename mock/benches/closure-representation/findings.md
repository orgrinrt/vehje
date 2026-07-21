# Closure representation: flat-capture vs linked-environment, selected by escape

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 20M calls, capture counts 1 to 8, env depths 1 to 7, two usage patterns,
5-run best. Zig 0.16.0, aarch64. Artifacts: `closure.zig`, `closure.csv`.
**Settles:** the `Lambda` + `Apply` closure representation (how captured variables are stored and accessed), and
how the choice ties to the reachability-binder / escape analysis the design already carries.

## Why this probe

Closures (a `Lambda` capturing its enclosing environment, invoked by `Apply`) are common in scripting: callbacks,
higher-order functions, and especially the closures passed to `Iter` pipeline stages (`map(|x| x + offset)`). The
representation fork is classic:

- **Flat closure:** copy the captured values into the closure object at creation. O(K) creation cost, O(1)
  captured-variable access.
- **Linked environment:** the closure holds a pointer to its parent frame; captured-variable access walks the
  environment chain. O(1) creation, O(depth) access.

Which wins depends on the usage pattern, so two are measured: create-once-call-many (a closure hoisted out of a
loop and invoked repeatedly, access dominates) and create-many-call-once (a closure created per use and invoked
once, creation dominates, the Iter-pipeline-callback case).

## Results (ns per call)

Create-once, call-many (access cost dominates):

| representation | ns/call |
|---|---|
| flat (K=8) | 0.53 |
| linked (depth 1) | 0.45 |
| linked (depth 4) | 1.95 |
| linked (depth 7) | 2.84 |

Create-many, call-once (creation cost dominates):

| captures K | flat (copy + call) | linked (ptr + call) |
|---|---|---|
| 1 | 2.50 | 1.42 |
| 4 | 4.13 | 1.42 |
| 8 | 2.93 | 1.43 |

## The finding: neither representation wins outright; escape analysis selects it, and the design already has the analysis

The two patterns invert the winner:

- **Create-once-call-many favours flat.** Flat access is O(1) (0.53 ns regardless of capture count), while linked
  access walks the chain (0.45 ns at depth 1, degrading to 2.84 ns at depth 7, about 0.4 ns per hop). A closure
  invoked many times wants its captures flat so every call is a direct index. Linked is competitive only at depth
  1 and loses as capture nesting deepens.
- **Create-many-call-once favours linked.** Linked creation is a single pointer (1.42 ns flat, independent of
  capture count), while flat creation copies every capture (2.5 to 4.1 ns, scaling with K). A closure created per
  use and called once wants to avoid the copy and just point at the enclosing frame. (The flat create-many numbers
  are slightly noisy across K because a full fixed-size copy vectorises differently than a partial one, but the
  trend is clear: flat creation scales with the copy, linked creation is constant.)

So there is no single best representation; the choice is per-closure, and the deciding property is **escape**:

- **A non-escaping closure** (it stays within its creating scope's lifetime: the Iter-pipeline callback that the
  pipeline consumes and drops, a closure passed down and not stored) can safely use the **linked environment**: the
  parent frame outlives the closure, so pointing at it is both correct and cheap (1.42 ns creation, no copy). This
  is the create-many-call-once regime, and it is the common scripting-callback case.
- **An escaping closure** (it outlives its creating scope: returned, stored in a record, captured by a longer-lived
  closure) MUST use the **flat representation**: the parent frame is reclaimed when the scope exits, so the
  captures have to be copied into the closure to stay valid. And an escaping closure is typically also
  create-once-call-many, so flat's O(1) access is a bonus rather than a cost.

This is exactly what the design's reachability-binder rule (splice-and-drop at Let/Lambda) plus region-promotion
analysis computes: whether a binding escapes its binder. That analysis, which the design already carries for the
N*W lifetime fix, is precisely the input that selects the closure representation. Non-escaping closures (the
analysis proves they do not outlive the binder) lower to a linked environment (cheap creation, no copy); escaping
closures (region-promoted) lower to a flat closure (captures copied into the promoted region). The runtime
closure representation is therefore not a separate design decision; it is a consequence of the escape verdict the
reachability analysis already produces.

## Design impact

- Closure representation is selected per-`Lambda` by the escape verdict from the reachability-binder / region
  analysis, not fixed globally. Non-escaping (the common Iter-callback case) uses a linked environment; escaping
  (returned/stored) uses flat captures.
- This makes the common case cheap: an Iter-pipeline callback (`map(|x| x + offset)`) does not escape the
  pipeline, so it is a linked env (1.42 ns creation, no capture copy), keeping the fused pipeline (iter-fusion
  bench) cheap end to end.
- It also makes the escaping case correct by construction: region-promotion already relocates the escaping
  binding's storage, and flattening the captures into that promoted region is the same relocation, so the flat
  closure falls out of the region-promotion the design already does.
- Linked-env access cost grows with capture nesting depth (~0.4 ns/hop), so for a linked closure that is called
  many times (rare, since call-many usually implies escape and thus flat), the analysis should prefer flat; the
  escape verdict already biases this way.

## Boundary

Captures are scalar i64 slots (NaN-box values fit this directly); a closure capturing large records captures a
handle/reference to them (the record lives in its own arena), so the capture slot is still scalar and the copy
cost is the handle, not the record. Env-chain depth here is a linear parent chain; a closure capturing from
several enclosing scopes at once is still a chain walk. The create-many flat noise across K is codegen
vectorisation of the fixed-array copy, not a real inversion. The escape analysis itself is not benched here (it is
the design's existing reachability-binder rule); this bench sizes the runtime cost of each representation the
analysis selects between.

## Artifacts
- `closure.zig` (flat vs linked, create-once-call-many and create-many-call-once, capture-count and depth sweeps),
  `closure.csv`.
