# SK16 findings: Perceus-derived exact-meet + in-place reuse

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifact `perceus.zig`
**Settles:** the collector-free memory model for produced values (2055), the honest-keeper's "Perceus-derived,
not verbatim" (compile/emit-time counting, no runtime refcounts).

## Result
For a 1,000,000-node fibonacci-shaped value DAG (each node i>=2 references i-1 and i-2), exact-meet emit-time
reuse keeps **peak live slots = 3** (the live-frontier width) and high-water allocation = 3, versus 1,000,000
total nodes (peak/N = 0.000003). A node's arena slot is reclaimed exactly when its LAST referrer is emitted
(referrer count-down known at emit time), and reused by the next allocation.

## Reading
The memory model is: peak arena memory = the live frontier, not the total node count, with no garbage collector
and no runtime reference counts (the counting is at emit time). This is the exact-meet between the leak (promote
everything, peak = N) and the blowup (copy everything): reclaim precisely at the last referrer. It composes with
the chunker (a chunk root is a promoted shared node, its meet finalised at chunk close) and with the value-arena
(SK20): the reclaimed slots are arena slots. Confirms the produced-value memory model is bounded, exact, and
collector-free on the pinned toolchain.

## Design impact
The value-arena's peak memory is frontier-bounded via emit-time referrer-count-down reuse; no collector, no
runtime refcounts. In-place reuse (a unique node's slot reused for the next same-shape allocation) is the same
mechanism. Ship it as the produced-value memory model.
