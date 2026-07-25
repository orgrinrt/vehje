# Dedup vs reuse composition, 20% shared: the templating norm

4 variants, 6 samples per variant.
Baseline: **dru_u_plain**

## Highlights

Baseline for all deltas below: **dru_u_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dru_u_reuse dominates: 149% faster than the next best (dru_u_plain)

dru_u_reuse (600 ns) leads dru_u_plain (1.49 us) by 149%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dru_u_reuse beats baseline by 59% (significant)

dru_u_reuse is -885 ns (59%) faster than baseline dru_u_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### dru_u_dedup is an outlier: 14.3x slower than the field

dru_u_dedup (8.59 us) is 14.3x the fastest (600 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {dru_u_reuse, dru_u_plain, dru_u_both} vs {dru_u_dedup} (355% apart)

The field splits into a fast tier {dru_u_reuse, dru_u_plain, dru_u_both} and a slow tier {dru_u_dedup} with a 355% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 14.3x the fastest

Fastest dru_u_reuse (600 ns) to slowest dru_u_dedup (8.59 us): 14.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dru_u_reuse** at 600.4 ns median (-59.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 14.32x (fastest 600.4 ns, slowest 8594.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dru_u_both | 4063ns | 4046ns | 3988ns | 4029ns | 4153ns | +11.27% |
| dru_u_dedup | 10970ns | 10769ns | 10362ns | 10760ns | 11589ns | +200.39% |
| dru_u_plain | 3652ns | 3650ns | 3610ns | 3644ns | 3684ns | base |
| dru_u_reuse | 2781ns | 2768ns | 2755ns | 2766ns | 2815ns | -23.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dru_u_both | 1883ns | 1813ns | 1926ns | +26.50% | 0.034 |
| dru_u_dedup | 8776ns | 8246ns | 9317ns | +489.66% | 0.007 |
| dru_u_plain | 1488ns | 1461ns | 1507ns | base | 0.043 |
| dru_u_reuse | 601ns | 593ns | 610ns | -59.61% | 0.106 |

## Performance model

- Peak throughput: **0.108 Gops/s** (dru_u_reuse; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dru_u_both | 0.034 | 31.4% |
| dru_u_dedup | 0.007 | 6.9% |
| dru_u_plain | 0.043 | 39.7% |
| dru_u_reuse | 0.107 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dru_u_both | 4063ns | 4063ns | +11.27% |
| dru_u_dedup | 10970ns | 10970ns | +200.39% |
| dru_u_plain | 3652ns | 3652ns | base |
| dru_u_reuse | 2781ns | 2781ns | -23.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dru_u_plain | 1495ns | base | --- | [1464, 1507] | --- | --- | --- | --- |
| dru_u_both | 1889ns | +424.2ns (+28.4%) | [+325, +434]ns | [1832, 1926] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_dedup | 8595ns | +7117.1ns (+476.2%) | [+6908, +7837]ns | [8415, 9317] | YES | 0.0313 | 0.0313 | 0 |
| dru_u_reuse | 600ns | -885.4ns (-59.2%) | [-907, -869]ns | [593, 610] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | dru_u_plain | dru_u_both | dru_u_dedup | dru_u_reuse |
|---|---|---|---|---|
| 1 | 1512ns | +22.4% | +467.6% | -60.0% |
| 2 | 1501ns | +20.8% | +449.3% | -59.4% |
| 3 | 1500ns | +28.6% | +541.0% | -60.5% |
| 4 | 1466ns | +29.5% | +486.4% | -59.5% |
| 5 | 1490ns | +29.2% | +476.9% | -59.0% |
| 6 | 1461ns | +28.7% | +517.5% | -59.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dru_u_both | 0.030 | ok |
| dru_u_dedup | -0.440 | moderate- |
| dru_u_plain | 0.064 | ok |
| dru_u_reuse | -0.281 | moderate- |

**Consistency summary:**

- **dru_u_both**: won 0/6, lost 6/6
- **dru_u_dedup**: won 0/6, lost 6/6
- **dru_u_reuse**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dru_u_both | 15987.6ns | 1882.6ns | 849.2% | HIGH |
| dru_u_dedup | 13836.2ns | 8775.5ns | 157.7% | HIGH |
| dru_u_plain | 16396.4ns | 1488.2ns | 1101.7% | HIGH |
| dru_u_reuse | 16407.7ns | 601.2ns | 2729.3% | HIGH |

## Distribution (algo ns)

```
dru_u_both (n=6, range 1813.3-1926.5 ns)
   1813.3 |########################################
   1819.0 |
   1824.6 |
   1830.3 |
   1835.9 |
   1841.6 |
   1847.2 |########################################
   1852.9 |
   1858.6 |
   1864.2 |
   1869.9 |
   1875.5 |########################################
   1881.2 |
   1886.8 |
   1892.5 |
   1898.2 |########################################
   1903.8 |
   1909.5 |
   1915.1 |
   1920.8 |########################################
  (0 below, 1 above range)

dru_u_dedup (n=6, range 8246.2-9317.3 ns)
   8246.2 |#############
   8299.8 |
   8353.3 |
   8406.9 |
   8460.4 |
   8514.0 |
   8567.5 |########################################
   8621.1 |
   8674.6 |
   8728.2 |
   8781.8 |
   8835.3 |
   8888.9 |
   8942.4 |
   8996.0 |#############
   9049.5 |
   9103.1 |
   9156.6 |
   9210.2 |
   9263.7 |
  (0 below, 1 above range)

dru_u_plain (n=6, range 1461.2-1506.7 ns)
   1461.2 |########################################
   1463.5 |
   1465.7 |########################################
   1468.0 |
   1470.3 |
   1472.6 |
   1474.8 |
   1477.1 |
   1479.4 |
   1481.7 |
   1483.9 |
   1486.2 |
   1488.5 |########################################
   1490.7 |
   1493.0 |
   1495.3 |
   1497.6 |########################################
   1499.8 |########################################
   1502.1 |
   1504.4 |
  (0 below, 1 above range)

dru_u_reuse (n=6, range 592.9-610.0 ns)
    592.9 |########################################
    593.8 |
    594.6 |####################
    595.5 |
    596.3 |
    597.2 |
    598.0 |
    598.9 |
    599.7 |
    600.6 |
    601.5 |
    602.3 |
    603.2 |
    604.0 |
    604.9 |####################
    605.7 |
    606.6 |
    607.4 |
    608.3 |
    609.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **dru_u_both**: bridge=819.7% of algo (FFI overhead may distort results)
- **dru_u_dedup**: bridge=158.5% of algo (FFI overhead may distort results)
- **dru_u_plain**: bridge=1089.3% of algo (FFI overhead may distort results)
- **dru_u_reuse**: bridge=2734.2% of algo (FFI overhead may distort results)
