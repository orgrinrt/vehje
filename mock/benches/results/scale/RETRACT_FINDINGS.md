# Retraction: recompute vs counted differential on edge deletion

**Strength: measurement** (CNTVCT_EL0, 5 runs median; recompute and counted cross-validated to the identical
post-delete reach set at every point).

## The audit item this builds

Semi-naive evaluation (`reach`, the C5 bench) handles additive edits: adding an edge only grows the fixpoint.
Deletion is the hard case, and the design names a DBSP-style counted differential as owed for non-additive
edits. This is the audit catalogue's "small bench of recompute-versus-counted on a delete." Single-target
reach-to-sink; `reach[u]` iff some live out-edge reaches the sink. The counted representation keeps `count[u]`,
the number of live out-neighbours of u that reach the sink (the derivation multiplicity), so a deletion
propagates only the retraction cascade instead of rebuilding. Both strategies reach the identical answer.

## Result: speedup (recompute_time / counted_time; > 1 = counted faster) and how many nodes actually dropped

Two graph densities: avg_out=1 (a forest, where a deletion disconnects a whole subtree, so retractions really
cascade) and avg_out=4 (densely connected to the sink, where deletions rarely drop anything).

| n | avg_out | del % | nodes dropped | recompute | counted | speedup |
|---|---|---|---|---|---|---|
| 100K | 1 | 0.01% | 27     | 2123 us | 0.4 us  | 5095x |
| 100K | 1 | 0.1%  | 765    | 1737 us | 13 us   | 130x |
| 100K | 1 | 1%    | 6183   | 1358 us | 96 us   | 14x |
| 100K | 1 | 10%   | 78944  | 291 us  | 1231 us | 0.2x (recompute wins) |
| 100K | 4 | 0.01-10% | 0    | ~2500 us | 0.5-227 us | 11-4880x |
| 1M | 1 | 0.01% | 1405    | 22298 us | 130 us   | 171x |
| 1M | 1 | 0.1%  | 9087    | 28449 us | 624 us   | 46x |
| 1M | 1 | 1%    | 88092   | 20742 us | 3880 us  | 5.3x |
| 1M | 1 | 10%   | 738865  | 5264 us  | 35275 us | 0.1x (recompute wins) |
| 1M | 4 | 0.01-10% | 0    | ~66000 us | 12-11423 us | 6-5615x |

## The finding: a clean crossover governed by how much actually drops

Counted differential wins by orders of magnitude for LOCALISED retractions and loses for CATASTROPHIC ones,
and the covariate is not the delete count but the number of nodes that actually stop reaching the sink:

- **Localised (few nodes drop):** counted is 100x to 5000x faster. Its cascade touches only the retracted
  support, so a deletion that drops 27 nodes costs 0.4 us against a 2 ms full rebuild.
- **Catastrophic (most nodes drop):** recompute WINS. At 10% edge deletion on the forest, 74-79% of nodes drop;
  the counted cascade then visits nearly the whole graph (35 ms at 1M) while recompute gets CHEAPER, not
  costlier, because the surviving graph it rebuilds over is tiny (5 ms). The crossover sits where roughly the
  same fraction of the graph drops as recompute would have to scan.
- **Dense graphs barely retract:** at avg_out=4, deleting even 10% of edges drops zero nodes (every node keeps
  another path to the sink), so counted is nearly free at any delete size. Robust connectivity makes deletion
  cheap for the counted representation regardless of batch size.

Design implication (op's call): maintain the counted (multiplicity) representation and use the differential
cascade for deletion; it is the right default because real edits are localised and dense lease graphs barely
retract. Keep recompute as the fallback for a catastrophic edit, switching when the retraction's expected drop
count approaches the graph size (a cheap estimate: if the counted cascade's frontier exceeds ~20% of nodes,
a from-scratch rebuild over the shrunken graph is faster).

## Cost-model / boundary

recompute over the forest at n=1M is a backward BFS touching ~1M edges: ~22 ms is ~22 ns/edge-visit,
memory-bound on the in-CSR + reach array, physically consistent. counted's cost tracks the dropped-node count
(~0.4 us for 27 drops to 35 ms for 739K drops), the cascade's real work. Boundary: single-target reachability
on a DAG (acyclic support, so the cascade terminates). Multi-target (the 64-bit reach words of the C5 bench)
or cyclic support would need per-bit or SCC-aware counts, a heavier differential; this bench establishes the
mechanism and the crossover for the single-target acyclic case the retraction question first needs answered.
