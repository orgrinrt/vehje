# SP5 + SK11 findings: no-alloc semi-naive relational engine, reach as a bitmask column

**Date:** 2026-07-21 | **Outcome:** WORKS | Zig 0.16.0 | artifact `engine.zig`
**Settles:** SP5 (the no-alloc semi-naive relational-fixpoint engine substrate, 2055) scoped to the lease
reachability query, and SK11 (reachability as a fixed-capacity bitmask lattice column, 2001 item 3).

## Result
The lease reachability query runs as a monotone semi-naive fixpoint with the IDB (Reach) stored as a per-node
u64 bitmask over <=64 binders (SK11's lattice column). Base rule seeds `Reach[n] |= 1<<b` from VarUse facts;
the recursive rule `Reach[p] |= Reach[c]` propagates over Child edges until no bit changes. On an 8-node DAG with
3 binders it converges in **2 rounds** (<= tree height) to the correct reach sets: reach[7]=0b111 (all three
binders reached through the DAG), reach[5]=0b011. No-alloc: fixed `[N]u64` columns, whole-column OR, no heap.

## Reading
Two things are validated together, because they are one mechanism:
- **The engine substrate (SP5):** a monotone semi-naive fixpoint is no-alloc-buildable and terminates in bounded
  rounds (lattice height, here tree height). Lease inference is a query on it, confirming 2055's "lease inference
  is a query on the relational engine," on the pinned toolchain.
- **The bitmask lattice column (SK11):** representing Reach as a per-node u64 mask makes the fixpoint a
  bitmask-OR propagation, the cheapest possible form, and gives the `N*W` bound (one W-wide mask per node)
  directly (SK4's naming-set bound realised as the column width).

## Scope (honest)
This is the reachability query (the load-bearing lease case) and the bitmask-column representation. A FULL
relational engine over arbitrary Datalog rules with joins (leapfrog-triejoin, AGM bounds) is larger, and the
eqsat query (congruence closure) is a different shape (union-find + hash-cons, SP6-adjacent); this spike does not
build those. It also uses whole-column OR each round rather than per-tuple delta tracking; true semi-naive
tracks the changed-node delta to avoid re-scanning, an efficiency refinement over the monotone fixpoint shown
here. The load-bearing feasibility (no-alloc monotone fixpoint + bitmask column for the lease query) holds.

## Design impact
The lease-inference query is a no-alloc bitmask-column semi-naive fixpoint, bounded rounds, cheap. The engine
substrate is buildable no-alloc; the full multi-relation-join + eqsat forms are the remaining engine work. SK4's
corrected binder rule plugs in as the base-rule/propagation refinement over this column.
