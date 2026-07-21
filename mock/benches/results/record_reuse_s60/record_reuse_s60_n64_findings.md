# Record update (60% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s60**

## Highlights

Baseline for all deltas below: **rec_reuse_s60**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 317% faster than the next best (rec_reuse_s60)

rec_mut (341 ns) leads rec_reuse_s60 (1.42 us) by 317%, a clear separation rather than a photo finish. CV 27.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 75% (significant)

rec_mut is -1.07 us (75%) faster than baseline rec_reuse_s60, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 5.9x slower than the field

rec_copy (2.02 us) is 5.9x the fastest (341 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 27.8%)

rec_mut wins on median (341 ns) yet has the highest variance (CV 27.8%), while rec_reuse_s60 is the steadiest (CV 4.3%, 1.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.9x the fastest

Fastest rec_mut (341 ns) to slowest rec_copy (2.02 us): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### rec_mut is inconsistent: worst-20% is 1.5x its best-20%

rec_mut's best 20% of batches run at 308 ns but its worst 20% at 464 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rec_mut** at 341.0 ns median (-76.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.93x (fastest 341.0 ns, slowest 2021.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 4435ns | 4498ns | 3982ns | 4461ns | 4622ns | +13.03% |
| rec_mut | 2948ns | 2960ns | 2822ns | 2914ns | 3063ns | -24.86% |
| rec_reuse_s60 | 3924ns | 3898ns | 3793ns | 3892ns | 4036ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 1997ns | 1788ns | 2099ns | +39.82% | 0.032 |
| rec_mut | 374ns | 308ns | 464ns | -73.82% | 0.171 |
| rec_reuse_s60 | 1428ns | 1323ns | 1499ns | base | 0.045 |

## Performance model

- Peak throughput: **0.208 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.032 | 15.2% |
| rec_mut | 0.188 | 90.2% |
| rec_reuse_s60 | 0.045 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 4435ns | 4435ns | +13.03% |
| rec_mut | 2948ns | 2948ns | -24.86% |
| rec_reuse_s60 | 3924ns | 3924ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s60 | 1423ns | base | --- | [1362, 1499] | --- | --- | --- | --- |
| rec_copy | 2022ns | +608.3ns (+42.7%) | [+448, +650]ns | [1869, 2099] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 341ns | -1068.5ns (-75.1%) | [-1118, -976]ns | [316, 464] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s60 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 1323ns | +35.1% | -76.8% |
| 2 | 1478ns | +44.4% | -78.0% |
| 3 | 1426ns | +43.9% | -76.0% |
| 4 | 1520ns | +28.4% | -61.6% |
| 5 | 1401ns | +42.2% | -75.7% |
| 6 | 1421ns | +45.3% | -75.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.330 | moderate- |
| rec_mut | -0.147 | ok |
| rec_reuse_s60 | -0.344 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 20384.8ns | 1996.7ns | 1020.9% | HIGH |
| rec_mut | 21507.0ns | 373.8ns | 5753.1% | HIGH |
| rec_reuse_s60 | 22131.5ns | 1428.0ns | 1549.8% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 1787.9-2099.0 ns)
   1787.9 |########################################
   1803.5 |
   1819.0 |
   1834.6 |
   1850.1 |
   1865.7 |
   1881.2 |
   1896.8 |
   1912.3 |
   1927.9 |
   1943.5 |########################################
   1959.0 |
   1974.6 |
   1990.1 |########################################
   2005.7 |
   2021.2 |
   2036.8 |########################################
   2052.3 |########################################
   2067.9 |
   2083.4 |
  (0 below, 1 above range)

rec_mut (n=6, range 307.5-464.4 ns)
    307.5 |#############
    315.3 |
    323.2 |#############
    331.0 |
    338.9 |########################################
    346.7 |
    354.6 |
    362.4 |
    370.3 |
    378.1 |
    386.0 |
    393.8 |
    401.6 |
    409.5 |
    417.3 |
    425.2 |
    433.0 |
    440.9 |
    448.7 |
    456.6 |
  (0 below, 1 above range)

rec_reuse_s60 (n=6, range 1323.3-1498.5 ns)
   1323.3 |####################
   1332.1 |
   1340.8 |
   1349.6 |
   1358.3 |
   1367.1 |
   1375.9 |
   1384.6 |
   1393.4 |####################
   1402.2 |
   1410.9 |
   1419.7 |########################################
   1428.5 |
   1437.2 |
   1446.0 |
   1454.7 |
   1463.5 |
   1472.3 |####################
   1481.0 |
   1489.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=1019.3% of algo (FFI overhead may distort results)
- **rec_mut**: CV=25.4% (high variance, measurements may be unstable)
- **rec_mut**: bridge=6329.5% of algo (FFI overhead may distort results)
- **rec_reuse_s60**: bridge=1538.7% of algo (FFI overhead may distort results)
