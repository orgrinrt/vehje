# C4a: incremental compilation, cold rebuild vs warm content-addressed reload

**Strength: measurement** (wall-clock via CNTVCT_EL0, 6 passes, cross-validated byte-exact).

## The audit defect this fixes

The old warm-load bench "measured" incrementality with an `if (i == 1234)` branch that
recompiled one hardcoded module and skipped the rest, so it measured a predicted branch, not a
cache. This implements the real mechanism: every module is content-hashed, a real
content-addressed cache is consulted, the N-1 unchanged modules HIT, the single input-edited
module MISSES and is recompiled. Cold and warm fold the identical final module set (the edit is
a valid regenerated program applied to both), so they cross-validate byte-exact: the warm path
is faster because it did less work, not because it computed something different.

A "module" is a small IR program; "compiling" it is `COMPILE_PASSES` (8) interpreter walks,
standing in for a front-end's parse/resolve/check/lower passes. The content hash is a fast
word-at-a-time hash (real content-addressed systems use xxhash/blake3, not a byte-serial FNV).

## Result, warm/cold ratio (< 1 = warm faster)

| n (modules) | cold | warm | warm/cold | warm speedup |
|---|---|---|---|---|
| 64    | 24.4 us  | 9.8 us   | 0.40x | 2.5x |
| 256   | 130.8 us | 39.9 us  | 0.31x | 3.3x |
| 1024  | 498.7 us | 141.5 us | 0.28x | 3.5x |
| 4096  | 2027 us  | 553 us   | 0.27x | 3.7x |
| 16384 | 8385 us  | 2355 us  | 0.28x | 3.6x |

Warm content-addressed reload after a single edit is 2.5x to 3.7x faster than a cold rebuild,
and the ratio stabilizes near 0.28x (a 3.6x speedup) as the module count grows.

## Cost-model sanity line, and the load-bearing caveat

The warm/cold ratio in the limit is `N*C / (N*H + C)` where C is compile cost and H is
content-hash cost: as N grows it approaches `C/H`. Warm skips (N-1)/N of the compiles and pays
N hashes instead. The measured 3.6x asymptote says compile costs roughly 3.6x a content hash in
this configuration (8 interpreter passes over a small module vs one fast word-at-a-time hash).

**This ratio is the whole story, and it is why the first cut of this bench measured the wrong
thing.** With a trivial one-pass "compile" and a byte-serial FNV hash, hashing cost EXCEEDED
compile cost and warm was 18x SLOWER than cold: incremental caching loses when compile is as
cheap as hashing. The honest claim is conditional: content-addressed incremental caching wins
in proportion to how much more expensive compilation is than hashing the source. For a real
front-end (parse + resolve + typecheck + lower, far more than 8 cheap passes) the ratio is much
larger than 3.6x and warm load is dominated by the single recompile plus N cheap hashes, which
is the "warm load is excellent" the design assumed. This bench establishes the mechanism and the
direction honestly; the absolute multiple scales with the real compile/hash cost ratio.

## Cross-validation

pass. Cold (compile all with the edit) and warm (hit N-1, recompile the 1 edited) fold to the
identical 8-byte result for every input; the harness confirmed identical output across both
variants at all sizes.

## Boundary

The edit is a single-module change (the common incremental case). A change touching a
dependency shared by many modules would invalidate more cache entries and shift warm toward
cold; that fan-out case is the threaded-DAG question (C4b, on the scale-runner), not this
single-edit reload.
