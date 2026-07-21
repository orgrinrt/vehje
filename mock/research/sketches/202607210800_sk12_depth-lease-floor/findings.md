# SK12 findings: the depth-lease floor (degenerate LIFO reach, proven-cheap metatheorem)

**Date:** 2026-07-21 | **Outcome:** WORKS | Rust nightly | artifact `depth.rs`
**Settles:** the depth-ladder lease floor (1845), the degenerate LIFO case under the full reachability lease.

## Result
A value's lease is a scalar DEPTH (the shallowest nesting level it must outlive), inferred by DEPTH-MIN
propagation over operands. A value reaching binders at depths {2,1,0} has lease 0 (the shallowest, the outermost
region it depends on). LIFO stack discipline is sound: closing regions innermost-first (depth 2, then 1, then 0),
each value is freed exactly when its lease-region closes (v2 at region 2, v1 at region 1, v_mixed at region 0),
so a value is promoted to the region equal to its lease and never outlives a shallower region than its lease.

## Reading
The depth-lease is the reachability qualifier (SK11's per-node bitmask) collapsed to a scalar: instead of a mask
over binders, a single min-depth. Inference is depth-min propagation (a min-lattice) instead of bitmask-OR, and
it converges the same way. The metatheorem (Calcagno-Helsen-Thiemann restricted to the LIFO chain, 1845) reduces
region soundness to the depth being monotone on the stack, which the LIFO free-order demonstrates: no value is
freed at a depth shallower than its lease. This is the proven-cheap floor: ship the degenerate depth-lease first
on this metatheorem, then grow the grade lattice into the full reachability bitmask (SK4/SK11) additively.

## Design impact
The lease axis ships its floor as the scalar depth-min lease (LIFO, provably sound, cheap), with the full
reachability-bitmask lease (SK4/SK11) as the additive growth. Both are the same coeffect at different grades: a
scalar min-depth (floor) vs a binder bitmask (full).
