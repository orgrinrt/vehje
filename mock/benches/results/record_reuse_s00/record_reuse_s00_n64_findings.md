# Record update (0% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s00**

## Highlights

Baseline for all deltas below: **rec_reuse_s00**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 13% faster than the next best (rec_reuse_s00)

rec_mut (322 ns) leads rec_reuse_s00 (364 ns) by 13%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_copy is an outlier: 6.3x slower than the field

rec_copy (2.04 us) is 6.3x the fastest (322 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 6.3x the fastest

Fastest rec_mut (322 ns) to slowest rec_copy (2.04 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 322.3 ns median (-11.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.32x (fastest 322.3 ns, slowest 2037.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 4645ns | 4573ns | 3972ns | 4414ns | 5328ns | +56.99% |
| rec_mut | 2841ns | 2795ns | 2525ns | 2757ns | 3125ns | -3.98% |
| rec_reuse_s00 | 2959ns | 2946ns | 2622ns | 2900ns | 3215ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 2039ns | 1773ns | 2286ns | +452.73% | 0.031 |
| rec_mut | 331ns | 294ns | 368ns | -10.34% | 0.193 |
| rec_reuse_s00 | 369ns | 324ns | 403ns | base | 0.173 |

## Performance model

- Peak throughput: **0.218 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.031 | 14.4% |
| rec_mut | 0.199 | 91.2% |
| rec_reuse_s00 | 0.176 | 80.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 4645ns | 4645ns | +56.99% |
| rec_mut | 2841ns | 2841ns | -3.98% |
| rec_reuse_s00 | 2959ns | 2959ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s00 | 364ns | base | --- | [340, 403] | --- | --- | --- | --- |
| rec_copy | 2037ns | +1680.6ns (+461.5%) | [+1447, +1883]ns | [1795, 2286] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 322ns | -34.3ns (-9.4%) | [-55, -26]ns | [302, 368] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s00 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 428ns | +474.8% | -11.6% |
| 2 | 378ns | +459.0% | -5.5% |
| 3 | 355ns | +476.0% | -9.1% |
| 4 | 358ns | +466.5% | -10.1% |
| 5 | 370ns | +390.3% | -16.1% |
| 6 | 324ns | +446.9% | -9.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.304 | moderate+ |
| rec_mut | 0.416 | moderate+ |
| rec_reuse_s00 | 0.078 | ok |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21366.7ns | 2039.3ns | 1047.7% | HIGH |
| rec_mut | 21263.2ns | 330.8ns | 6428.1% | HIGH |
| rec_reuse_s00 | 21971.2ns | 369.0ns | 5955.1% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 1772.9-2286.1 ns)
   1772.9 |########################################
   1798.6 |########################################
   1824.2 |
   1849.9 |
   1875.5 |
   1901.2 |
   1926.8 |
   1952.5 |
   1978.2 |
   2003.8 |########################################
   2029.5 |########################################
   2055.1 |
   2080.8 |
   2106.4 |########################################
   2132.1 |
   2157.8 |
   2183.4 |
   2209.1 |
   2234.7 |
   2260.4 |
  (0 below, 1 above range)

rec_mut (n=6, range 293.8-367.8 ns)
    293.8 |####################
    297.5 |
    301.2 |
    304.9 |
    308.6 |####################
    312.3 |
    316.0 |
    319.7 |########################################
    323.4 |
    327.1 |
    330.8 |
    334.5 |
    338.2 |
    341.9 |
    345.6 |
    349.3 |
    353.0 |
    356.7 |####################
    360.4 |
    364.1 |
  (0 below, 1 above range)

rec_reuse_s00 (n=6, range 324.2-402.9 ns)
    324.2 |########################################
    328.1 |
    332.1 |
    336.0 |
    339.9 |
    343.9 |
    347.8 |
    351.7 |########################################
    355.7 |########################################
    359.6 |
    363.5 |
    367.5 |########################################
    371.4 |
    375.4 |########################################
    379.3 |
    383.2 |
    387.2 |
    391.1 |
    395.0 |
    399.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=1047.5% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=6517.7% of algo (FFI overhead may distort results)
- **rec_reuse_s00**: bridge=5996.2% of algo (FFI overhead may distort results)
