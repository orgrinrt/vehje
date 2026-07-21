# C5: reachability fixpoint, whole-column vs real semi-naive

**Strength: measurement** (wall-clock via CNTVCT_EL0; harness 6 passes, scale-runner 5 runs; both cross-validated byte-exact on the reach checksum).

## The audit defect this fixes

The old reach bench's "delta semi-naive" scanned the full edge list every round
(`for e in edges { if !dirty[e.dst] { continue } ... }`), so it was O(E) per round like
whole-column, only with a skip. Its "index edges by child" comment described a CSR index it
never built, and it reported whole-column throughput as `edges*rounds/ms` but delta as
`edges/ms`, two different units. That is not semi-naive, and the comparison was never fair at
any scale. This re-measures with a real semi-naive: a CSR in-edge index plus a frontier
worklist, so a round touches only the edges incident to nodes whose reach changed. Both
solvers live in `carrier::reach`, run over one shared graph per shape, and cross-validate:
they must reach the identical fixpoint (a unit test and the harness/scale cross-check both
assert it), so any measured gap is the evaluation strategy and nothing else.

Reachability is a u64 word per node (up to 64 tracked targets); propagation is
`reach[src] |= reach[dst]` along each edge.

## Harness results (small scale, crossover direction), semi/whole ratio (< 1 = semi faster)

| shape | n=64 | n=256 | n=1024 | n=4096 | n=16384 |
|---|---|---|---|---|---|
| chain depth 8   | 0.45x | 0.34x | 0.27x | 0.28x | 0.21x |
| chain depth 32  | 0.40x | 0.33x | 0.43x | 0.82x | 0.84x |
| chain depth 128 | 0.41x | 0.31x | 0.16x | 0.29x | 0.51x |
| fan-in (1 round)| 1.25x | 0.73x | 0.47x | 0.59x | 0.52x |
| random DAG      | 0.34x | 0.24x | 0.22x | 0.37x | 0.39x |

## Scale-runner results (millions of nodes, the regime the harness cap excludes)

| shape | n | edges | whole | semi | semi/whole |
|---|---|---|---|---|---|
| random DAG | 1M | 4.0M  | 267 ms  | 109 ms  | 0.41x |
| random DAG | 4M | 16.0M | 2346 ms | 618 ms  | 0.26x |
| random DAG | 8M | 32.0M | 5964 ms | 1287 ms | 0.22x |
| layered32  | 1M | 2.9M  | 215 ms  | 170 ms  | 0.79x |
| layered32  | 4M | 11.6M | 853 ms  | 712 ms  | 0.83x |
| fan-in     | 8M | 8.0M  | 11.3 ms | 3.6 ms  | 0.32x |

## Cost-model sanity line

Whole-column does `rounds x edges` edge-ops (a load, an OR, a compare, a branch each). At scale
the measured cost per edge-op rises from 7.9 cyc/op (random DAG 1M) to 18.6 cyc/op (8M): the
8M reach array is 64 MB and the 32M-edge working set exceeds cache, so the fixpoint goes
memory-bound, which is exactly what a rising cyc/edge-op reports. At ~3.2 GHz the 8M whole-column
figure (5.96 s for 32M edges x 32 rounds ~ 1.0e9 edge-ops) is ~19 cyc/op, physically consistent
with a DRAM-bound scatter/gather, not an artifact.

## The finding (a shape-dependent answer, not a blanket one)

Semi-naive wins across almost every shape, but the margin is governed by three axes the old
bench never separated: chain depth, layer width, and the tracked-target count (reach-word
density).

- **Narrow, deep graphs (chain depth 8, random DAG): semi wins 2x to 5x**, and the margin grows
  with scale (random DAG 0.41x at 1M to 0.22x at 8M). This is the regime semi-naive is for.
- **Wide graphs with many tracked targets (chain depth 32 at large n, layered32 at millions):
  semi wins only ~1.2x.** Whole-column's `reach[src] |= reach[dst]` amortizes all 64 targets in
  one u64 OR per edge-op, while semi-naive re-queues a node every time its word gains any bit
  (up to 64 revisits). Wide reach-words favor whole-column and erode semi's advantage. This is
  the same effect in the harness (chain32 0.84x at n=16384) and at scale (layered32 0.83x).
- **Fan-in at tiny n: whole-column wins (1.25x at n=64)**, because semi-naive's frontier
  bookkeeping costs more than a single O(E) pass when there is essentially one round of work.

Design implication for the SP5 lease reachability fixpoint: the original "whole-column chosen
for the shallow-graph regime on simplicity" is vindicated for the wide, shallow, many-target
regime, where whole-column is within ~1.2x of semi-naive and far simpler. For deep, narrow
propagation (few tracked leases, long dependency chains) semi-naive is 2x to 5x better and
should be preferred. The choice is not whole-vs-semi in the abstract; it is a function of the
lease-graph shape the compiler actually produces, which the earlier bench could not have shown
because its "delta" was not semi-naive.

## Boundary

Reach tracks up to 64 targets (one u64 word). More targets need multi-word reach and would
shift the bitset-amortization balance further toward whole-column. Deep chains at multi-million
scale are absent from the scale-runner by construction (whole-column would take millions of
rounds); the harness covers the deep regime at small n, where semi-naive's win is already clear.
