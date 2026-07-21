# Record update (20% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s20**

## Highlights

Baseline for all deltas below: **rec_reuse_s20**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 137% faster than the next best (rec_reuse_s20)

rec_mut (295 ns) leads rec_reuse_s20 (699 ns) by 137%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 58% (significant)

rec_mut is -405 ns (58%) faster than baseline rec_reuse_s20, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 6.2x slower than the field

rec_copy (1.82 us) is 6.2x the fastest (295 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 6.2x the fastest

Fastest rec_mut (295 ns) to slowest rec_copy (1.82 us): 6.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 294.8 ns median (-57.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.18x (fastest 294.8 ns, slowest 1820.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 4262ns | 4107ns | 3971ns | 4087ns | 4670ns | +37.98% |
| rec_mut | 2678ns | 2542ns | 2513ns | 2538ns | 2971ns | -13.30% |
| rec_reuse_s20 | 3089ns | 2973ns | 2866ns | 2938ns | 3427ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 1893ns | 1764ns | 2082ns | +162.97% | 0.034 |
| rec_mut | 310ns | 288ns | 344ns | -56.99% | 0.207 |
| rec_reuse_s20 | 720ns | 643ns | 807ns | base | 0.089 |

## Performance model

- Peak throughput: **0.222 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 15.8% |
| rec_mut | 0.217 | 97.7% |
| rec_reuse_s20 | 0.092 | 41.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 4262ns | 4262ns | +37.98% |
| rec_mut | 2678ns | 2678ns | -13.30% |
| rec_reuse_s20 | 3089ns | 3089ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s20 | 699ns | base | --- | [654, 807] | --- | --- | --- | --- |
| rec_copy | 1821ns | +1124.0ns (+160.8%) | [+1121, +1276]ns | [1777, 2082] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 295ns | -405.2ns (-58.0%) | [-463, -363]ns | [290, 344] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s20 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 692ns | +161.8% | -57.4% |
| 2 | 820ns | +149.0% | -58.7% |
| 3 | 643ns | +174.3% | -54.2% |
| 4 | 665ns | +169.2% | -56.7% |
| 5 | 706ns | +159.1% | -58.5% |
| 6 | 794ns | +167.4% | -56.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.230 | moderate- |
| rec_mut | -0.230 | moderate- |
| rec_reuse_s20 | -0.256 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 19968.0ns | 1893.4ns | 1054.6% | HIGH |
| rec_mut | 20041.1ns | 309.7ns | 6472.2% | HIGH |
| rec_reuse_s20 | 21240.7ns | 720.0ns | 2950.1% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 1763.7-2082.5 ns)
   1763.7 |########################################
   1779.6 |########################################
   1795.6 |
   1811.5 |########################################
   1827.5 |########################################
   1843.4 |
   1859.3 |
   1875.3 |
   1891.2 |
   1907.2 |
   1923.1 |
   1939.0 |
   1955.0 |
   1970.9 |
   1986.9 |
   2002.8 |
   2018.7 |
   2034.7 |########################################
   2050.6 |
   2066.6 |
  (0 below, 1 above range)

rec_mut (n=6, range 287.9-343.8 ns)
    287.9 |####################
    290.7 |####################
    293.5 |########################################
    296.3 |
    299.1 |
    301.9 |
    304.7 |
    307.4 |
    310.2 |
    313.0 |
    315.8 |
    318.6 |
    321.4 |
    324.2 |
    327.0 |
    329.8 |
    332.6 |
    335.4 |
    338.2 |####################
    341.0 |
  (0 below, 1 above range)

rec_reuse_s20 (n=6, range 642.9-806.9 ns)
    642.9 |########################################
    651.1 |
    659.3 |########################################
    667.5 |
    675.7 |
    683.9 |
    692.1 |########################################
    700.3 |########################################
    708.5 |
    716.7 |
    724.9 |
    733.1 |
    741.3 |
    749.5 |
    757.7 |
    765.9 |
    774.1 |
    782.3 |
    790.5 |########################################
    798.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=1067.8% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=6500.6% of algo (FFI overhead may distort results)
- **rec_reuse_s20**: bridge=2910.1% of algo (FFI overhead may distort results)
