# Sharded interner merge: single-threaded vs sharded parallel + merge tail

**Strength: measurement** (CNTVCT_EL0, 5 runs median; single and sharded cross-validated to the identical
canonical first-occurrence signature, so the equivalence classes match).

## The named lever

Interning maps each token's bytes to a stable dense id, deduping; it runs on every identifier/literal during
lex/parse. The named lever is to shard it: each thread interns its slice of the token stream into a private
table, then a sequential merge folds the shard tables into one global table (re-interning each shard's distinct
strings, resolving cross-shard duplicates, remapping local ids to global). The parallel intern is cheaper, but
the merge is a tail. This measures whether shard-plus-merge beats single-threaded, and how the answer depends
on the dedup rate. Interning is real: each token owns a byte copy, so the interner hashes and byte-compares.

## Result (2M tokens, Apple M1), single vs parallel-intern + merge

| regime | distinct | S | single | parallel | merge | total | speedup |
|---|---|---|---|---|---|---|---|
| high-dedup | 256    | 2 | 36.2 ms | 17.0 ms | 1.3 ms  | 18.3 ms | 1.98x |
| high-dedup | 256    | 4 | 36.2 ms | 9.7 ms  | 1.3 ms  | 11.0 ms | 3.28x |
| high-dedup | 256    | 8 | 36.2 ms | 8.6 ms  | 1.4 ms  | 10.0 ms | 3.63x |
| med-dedup  | 68743  | 2 | 40.6 ms | 35.7 ms | 4.7 ms  | 40.5 ms | 1.00x |
| med-dedup  | 68743  | 4 | 40.6 ms | 21.5 ms | 7.8 ms  | 29.3 ms | 1.38x |
| med-dedup  | 68743  | 8 | 40.6 ms | 17.7 ms | 10.7 ms | 28.4 ms | 1.43x |
| low-dedup  | 124298 | 2 | 61.0 ms | 37.9 ms | 12.0 ms | 49.9 ms | 1.22x |
| low-dedup  | 124298 | 4 | 61.0 ms | 22.1 ms | 16.8 ms | 38.9 ms | 1.57x |
| low-dedup  | 124298 | 8 | 61.0 ms | 16.3 ms | 18.5 ms | 34.8 ms | 1.75x |

## The finding: the merge tail is the limiter, and the dedup rate sets its size

Sharding pays off, but the merge tail caps the win, and how hard it caps depends on the dedup rate:

- **High dedup (few distinct strings): sharding scales to 3.6x.** The merge tail is tiny (1.3 ms) and constant
  in the shard count, because there are only 256 distinct strings to re-intern no matter how the tokens were
  partitioned. The limit here is the parallel intern's sub-linear scaling (8.6 ms at S=8, not 36/8; M1's 4
  performance + 4 efficiency cores and the memory-bound hash table, the same big.LITTLE ceiling C4b showed).
- **Medium/low dedup (tens of thousands distinct): sharding gives only 1.4-1.75x.** The merge tail GROWS with
  the shard count (4.7 -> 7.8 -> 10.7 ms at med; 12 -> 16.8 -> 18.5 ms at low), because more shards means each
  shard independently rediscovers more of the vocabulary, so the merge re-interns more distinct strings and
  resolves more cross-shard duplicates. At S=2 med-dedup the tail exactly cancels the parallel gain (1.00x).
- The merge tail is not a fixed cost: it is O(sum of per-shard distinct counts), which rises with both the
  shard count and the vocabulary size. High dedup keeps per-shard distinct counts small; low dedup does not.

Design implication (op's call): sharded interning is worth it, but the merge must be cheap, which it is only
when the token stream deduplicates heavily (the common case for identifiers/keywords, less so for literals). A
practical loader should shard the identifier/keyword stream (high dedup, 3.6x) and consider leaving the
literal stream single-threaded or sharing a global literal table, since a low-dedup stream's merge tail eats
most of the parallel gain. Sharding beyond 4 threads earns little on M1 (big.LITTLE plus a growing tail).

## Cost-model / boundary

single at low-dedup is 61 ms for 2M interns ~ 30 ns/intern (a 3..11-byte FNV hash, an open-addressed probe,
and a byte compare on a hit, over a 16 MB pool past cache), physically consistent; high-dedup is 18 ns/intern
because the 256-entry table stays hot in cache. Boundary: the merge is timed sequentially (a real loader could
parallelise the merge itself, a further lever not taken here); the parallel intern's wall time is the slowest
shard, so shard imbalance would widen it (the contiguous partition here is balanced by construction).
