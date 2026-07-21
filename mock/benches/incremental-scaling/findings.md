# BN3 findings: incremental scaling (D6, differential recompute)

**Date:** 2026-07-21 | **Type:** runtime bench, Zig-native | Zig 0.16.0 | data `incr.csv`
**Settles:** D6 (the framework-owned incremental scaling layer): edit one shard -> recompute ~the change, not N.

## Result (reach fixpoint over a complete binary tree of N shards, edit one leaf)

| N | full visits | incremental visits | ratio | full ns | incr ns |
|---|---|---|---|---|---|
| 1,023 | 1,023 | 10 | 0.98% | 3,000 | ~0 |
| 65,535 | 65,535 | 16 | 0.024% | 89,000 | ~0 |
| 1,048,575 | 1,048,575 | 20 | 0.002% | 1,613,000 | ~0 |

Editing one leaf and recomputing only propagates up the leaf's ancestor path, with a **differential early-out**
(stop the moment a node's recomputed value is unchanged). Incremental work is O(depth) = O(log N); full recompute
is O(N). At N=1M the full recompute is 1.6 ms and the incremental is below clock resolution (~20 node visits).

## Reading
This is the core of D6: incremental maintenance of the compile-stage query costs the size of the affected region,
not the size of the stack. Edit one mod/shard, recompute ~that change. The early-out is differential dataflow's
key mechanism (only changed tuples propagate). Combined with content-addressed dedup (unchanged shards are cache
hits), a stack of N mods costs ~sum-of-changes as D6 claims.

## Scope (honest)
This is the MONOTONE additive/strengthening case (editing a leaf ORs in a bit, propagating up until nothing
changes), which models adding a mod or strengthening a fact, the common case. Full RETRACTION (removing a fact,
non-monotone) needs counted differential dataflow (DBSP-style multiplicities) to know what to un-derive; that is
the harder machinery D6 owns for the general edit. The additive case, the majority of mod-stack growth, is
confirmed O(change) here.

## Design impact
The framework-owned incremental layer (D6) delivers O(change) recompute for localized edits via differential
propagation with early-out, over the semi-naive engine (SP5). Content-addressed shard caching handles the
independent case; the differential fixpoint handles the dependent case. Retraction needs the counted DBSP form,
the remaining engineering for general (non-additive) edits.
