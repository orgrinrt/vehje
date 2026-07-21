# SK19 + SK13 findings: the load-path residuals (tnum + generational-ref)

**Date:** 2026-07-21 | **Outcome:** WORKS (both) | Zig 0.16.0 | artifact `residuals.zig`

- **SK19 tnum (numeric load residual, 2001 item 4, eBPF-shaped):** tristate numbers `{value, mask}` (mask bit =
  unknown) track known/unknown bits through arithmetic without concrete evaluation. An unknown low-8-bit value
  has max 255 (provably < 256, not < 100); ANDed with 0x0F it has max 15 (provably < 16). This is the bounded
  abstract interpretation for the numeric part of load verification: prove range bounds on untrusted values
  cheaply. Confirmed.
- **SK13 generational-reference (avoidance-boundary residual, Vale):** a reference carries a generation; the
  region carries one; deref checks `ref.gen == region.gen`. A valid deref returns 42; after free+reuse (region
  gen bumped 1->2), the stale reference (gen 1) derefs to null, catching the use-after-free. This is the
  per-reference dynamic residual for exactly the references reachability inference could not place statically
  (SK4's LeaseResidual), one u32 compare at deref.

## Design impact
The load path's residuals are both cheap and buildable: tnum for the numeric bounds (bit-level abstract
interpretation), generational-ref for the un-placeable references (one compare). Together with the typed
structural decode (SK18) they are the three load-verification mechanisms (structural / numeric / lease) confirmed
by the honest-keeper, all now executable.
