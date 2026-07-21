# Eqsat associativity cap-explosion: does the bounded window catch it?

**Strength: measurement** (CNTVCT_EL0 timing, 3 runs median; cross-validated on extracted value AND cost).

## The question

The design lowers via equality saturation under a no-alloc bound. The open question: build the case
that is supposed to blow an e-graph up (a long associative-commutative chain, whose reassociations are
super-exponentially many), and watch the bounded window catch it. Does the bound prevent the explosion,
and at what cost to extraction quality?

The engine is a real e-graph (`carrier::eqsat`): hash-consed e-nodes, a union-find over e-classes, a
congruence-closure rebuild, associativity `op(op(x,y),z) = op(x,op(y,z))` and commutativity
`op(a,b) = op(b,a)` rewrites, and greedy min-cost extraction. The workload is a left-leaning associative
chain of K leaves. Bounded caps e-nodes at 512; unbounded runs to a 2M safety ceiling.

## Result

| K | bounded e-nodes | bounded cost | bounded us | unbounded e-nodes | unbounded cost | unbounded us | equal value |
|---|---|---|---|---|---|---|---|
| 8  | 512 | 15 | 410  | 2552   | 15 | 9712    | yes |
| 10 | 514 | 19 | 341  | 14782  | 19 | 46299   | yes |
| 12 | 513 | 23 | 174  | 78906  | 23 | 357642  | yes |
| 14 | 512 | 27 | 173  | 398552 | 27 | 3004131 | yes |

Beyond the harness/scale table, a standalone probe confirmed the explosion continues: K=16 unbounded is
1.93M e-nodes and ~23 s; K>=18 hits the 2M ceiling (`hit_cap`).

## The finding

The bounded window catches the explosion completely, at zero extraction-quality cost on this workload:

1. **The bound holds.** Bounded e-node count stays at ~512 regardless of K; unbounded explodes
   2552 -> 398552 over K=8..14 (156x) and is on track to 2M+ by K=18.
2. **The bound loses nothing.** Bounded extraction cost (15, 19, 23, 27) is IDENTICAL to the fully
   saturated unbounded cost at every K. For a left-leaning associative chain the optimal reassociation is
   reachable within a few hundred e-nodes; the millions of additional e-nodes the unbounded run creates are
   redundant equivalent forms that do not improve the extraction.
3. **The bound is far faster.** Bounded is 24x faster at K=8 (0.41 ms vs 9.7 ms) and 17000x faster at K=14
   (0.17 ms vs 3004 ms), because it never pays for the exponential rebuild.
4. **Cross-validation.** Bounded and unbounded extract forms that evaluate to the identical value at every K
   (all reassociations are equal), so the comparison is of two evaluation strategies over the same semantics,
   not of two different computations.

Design implication: the design's bounded e-graph is vindicated on the exact case meant to break it.
Associativity-reassociation, the classic saturation-explosion driver, is fully handled by a small bounded
window that finds the optimal extraction. The bound is not a quality compromise here; it is free.

## Cost-model / boundary

The unbounded time grows faster than the e-node count (300x time over K=8..14 vs 156x e-nodes) because the
congruence-closure rebuild is superlinear in e-node count. This is the cost the bound avoids. Boundary: the
"zero quality cost" result is specific to associativity/commutativity on a chain, where the optimal form is
low-e-node. A rewrite set whose best extraction genuinely needs a large e-graph (deep multi-way rewrites
that only pay off after wide exploration) could make a 512-node bound lossy; that case is not this one, and
the design should re-run this bench with its actual rewrite set when it lands.
