# Value-arena throughput (expansion): encode / zero-copy read / validate

**Date:** 2026-07-21 | Zig 0.16.0 | data `arena.csv` | expands SK20.

## Result (16M nodes = 192 MB nodes + 85 MB pool)
| stage | time | throughput |
|---|---|---|
| encode (build, children-first, RNG children) | 79.3 ms | 2.4 GB/s |
| zero-copy read / walk (pool-indirected) | 171.4 ms | 1.1 GB/s |
| typed validate (linear, scalar) | 19.9 ms | 9.7 GB/s |

## Reading
- **Encode is cheap and RNG-bound.** 2.4 GB/s includes generating random child indices per node; a real encoder
  copying already-known children would be memcpy-bound (much faster). So encode is not a transport bottleneck.
- **Validate is cheap.** 9.7 GB/s scalar; the SIMD version (BN4) hits 33 GB/s over the same aligned format. The
  untrusted structural decode is not a bottleneck.
- **The walk (read) is the dominant cost, and it is COMPUTE, not transport overhead.** 1.1 GB/s (10.7 ns/node)
  because this arena uses a POOL INDIRECTION for every record (`res[pool[n.a]] + res[pool[n.a+1]]`), the 16B-style
  spill path. This corroborates BN2 from another angle: the pool indirection is expensive, and the 24-byte inline
  operand record (BN2) avoids it, which is why the interp benches (inline operands) hit 4.4 ns/node vs 10.7 here.

## Design impact
Transport itself (encode + validate) is cheap (2.4 / 9.7-33 GB/s); the cost is interpretation (the walk), which
is the actual compute. The walk cost is dominated by operand access, so the 24-byte inline-operand record (BN2)
matters: it turns the 10.7 ns/node pool-indirected walk into the ~4.4 ns/node inline walk. Encode and validate
are not on the critical path.
