# Effect lattice inference + inclusion: the correctness core, and the encoding that makes it free

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 8M-node DAG, thermometer-encoded effect grades, 5-run best. Zig 0.16.0,
aarch64. Artifacts: `effect.zig`, `effect.csv`.
**Settles:** the discharge cost of the effect lattice (the load-bearing correctness mechanism: certified
generation rejects any construct whose effect exceeds the target's permits), and the encoding that makes the
ordered lattice cost the same as a flat set.

## Why this probe

The effect system is the correctness core of the certified-generation model: every construct carries an effect,
effects join up the program (a node's effect is the join of its children's), and the whole program's effect must
be included in the target's permitted effects, checked at compile time. The lattice is ordered per effect-family
(none < Reads < Writes), which a naive binary encoding (0/1/2) makes awkward: the per-family join is a max, not a
bitwise OR, and the inclusion test is a per-family comparison. The question is whether the ordered lattice costs
more than a flat effect set, and how fast the whole-program inclusion gate runs.

## The encoding insight: thermometer/unary grades make the lattice free

Encode each family's grade in thermometer (unary) form rather than binary: none = `00`, read = `01`, write =
`11`. Then the ordered-lattice operations collapse to trivial bitwise ops:

- **Lattice join (per-family max) becomes bitwise OR.** The max of two thermometer codes is their OR
  (`max(01, 11) = 11`, `max(00, 01) = 01`), verified correct.
- **Graded-monad bind (effect sequencing) is the same join** (a later write dominates an earlier read on the same
  family), so sequencing is also an OR.
- **Inclusion (script <= target per family) becomes a subset test** `(script & ~target) == 0`: every thermometer
  bit the script sets must be permitted by the target.

So the ordering, which a binary encoding would pay for with per-lane max and per-lane comparison, is free under
thermometer encoding: the ordered lattice costs exactly a flat bitwise op. (An earlier binary-encoded attempt with
SWAR lane-compare tricks was both slower and buggy; the thermometer encoding is the correct and fast formulation.)

## Results

8M-node DAG, 24 effect families (2 thermometer bits each, packed in a u64):

| operation | ns/node | throughput |
|---|---|---|
| effect join (= bitwise OR) | 6.14 | 163 M-node/s |
| inclusion check (subset test) | 0.39 | 2557 M-node/s |

Correctness verified: `join(read, write) == write`, `read <= write` holds, `write <= read` does not.

## The finding: the effect proof discharges at bitmask-propagation cost, and the inclusion gate is essentially free

The correctness core is cheap:

- **Effect inference (bottom-up join) is bitmask propagation.** The join operation itself is a single OR; the
  6.14 ns/node here is memory-bound (the random gather of children's effects across an 8M-node array, the same
  cache-cold DAG-walk cost the lease-fixpoint bench measured), not the OR. When the walk is cache-hot the rate is
  the lease-fixpoint bench's ~630 M-edge-OR/s. So effect inference costs exactly what any bitmask propagation over
  the program DAG costs, no more: the ordered lattice adds zero compute over a flat effect set because thermometer
  encoding makes the join an OR.
- **The inclusion gate is essentially free: 0.39 ns/node, 2.5 G-node/s.** Checking whether every node's effect is
  within the target's permits is a sequential scan of one AND-NOT plus a compare per node. A whole 8M-node
  program's effect-inclusion proof discharges in ~3 ms. This is the gate that certified generation rests on
  (reject any construct whose effect exceeds the target's permits), and it is effectively costless.

So the effect proof, the mechanism the entire certified-generation correctness argument depends on, is affordable
to the point of being negligible on the compile side, provided the grades are thermometer-encoded. The encoding is
the load-bearing choice: it turns the ordered lattice into flat bitwise operations.

## Design impact

- Encode effect grades in thermometer/unary form (none = 00, read = 01, write = 11, and higher grades as longer
  thermometer codes if the lattice grows). This makes the lattice join a bitwise OR, the graded-monad bind the
  same OR, and the inclusion check a subset test, so the ordered effect lattice costs exactly a flat effect set.
- Effect inference is bitmask propagation over the program DAG, the same shape and cost as the lease-fixpoint
  (reachability) inference, so the two analyses share the propagation machinery (bottom-up join over the DAG) and
  differ only in the lattice payload.
- The inclusion gate (0.39 ns/node) is free, so certified generation can check effect inclusion over the whole
  program without a compile-budget concern. The gate is the certified-generation correctness boundary, and it is
  costless.
- The graded-monad bind reusing the join means sequencing effects composes at OR cost, so the effect grading
  along the proof spine (binding-time modality, effect graded monad, lease coeffect) shares one cheap OR-based
  propagation for the effect coordinate.

## Boundary

24 families x 2-bit thermometer grades in a u64 (room for 32); a richer effect lattice (more grades per family, or
more families) uses a wider bitset (u128 or a small fixed array of u64), keeping the OR/subset operations, just
wider. The per-node join here gathers two children; a node with many children (an n-ary construct) ORs all of
them, still one OR per edge. The 6.14 ns/node is the cache-cold DAG-gather cost; the effect inference in practice
interleaves with the other DAG passes (resolve, check, lease) so the gather is amortised across them. Thermometer
encoding assumes a total order per family (none < read < write); a family with an unordered effect set (rare) uses
a flat OR sub-field, which is the same operation.

## Artifacts
- `effect.zig` (thermometer-encoded effect join and inclusion over an 8M-node DAG, with correctness checks),
  `effect.csv`.
