# Cheap lowering node-count reduction: fold+CSE vs fold-only (metric = node count, not time)

2 variants, 6 samples per variant.
Baseline: **cl_foldonly**

## Highlights

Baseline for all deltas below: **cl_foldonly**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cl_foldonly dominates: 95% faster than the next best (cl_foldcse)

cl_foldonly (1.51 us) leads cl_foldcse (2.94 us) by 95%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (cl_foldonly)

The baseline cl_foldonly is the fastest (1.51 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cl_foldonly) is the fastest** at 1512.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.95x (fastest 1512.7 ns, slowest 2942.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cl_foldcse | 5206ns | 5127ns | 5033ns | 5104ns | 5445ns | +40.24% |
| cl_foldonly | 3712ns | 3684ns | 3658ns | 3680ns | 3787ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cl_foldcse | 2983ns | 2870ns | 3131ns | +95.56% | 0.343 |
| cl_foldonly | 1525ns | 1500ns | 1561ns | base | 0.671 |

## Performance model

- Peak throughput: **0.683 Gops/s** (cl_foldonly; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cl_foldcse | 0.348 | 51.0% |
| cl_foldonly | 0.677 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cl_foldcse | 5206ns | 5206ns | +40.24% |
| cl_foldonly | 3712ns | 3712ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cl_foldonly | 1513ns | base | --- | [1502, 1561] | --- | --- | --- | --- |
| cl_foldcse | 2943ns | +1435.8ns (+94.9%) | [+1336, +1600]ns | [2875, 3131] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | cl_foldonly | cl_foldcse |
|---|---|---|
| 1 | 1504ns | +99.8% |
| 2 | 1512ns | +90.5% |
| 3 | 1565ns | +83.3% |
| 4 | 1514ns | +97.0% |
| 5 | 1500ns | +93.6% |
| 6 | 1557ns | +109.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cl_foldcse | -0.115 | ok |
| cl_foldonly | -0.304 | moderate- |

**Consistency summary:**

- **cl_foldcse**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cl_foldcse | 2482.3ns | 2982.8ns | 83.2% | HIGH |
| cl_foldonly | 1487.8ns | 1525.2ns | 97.5% | HIGH |

## Distribution (algo ns)

```
cl_foldcse (n=6, range 2870.0-3130.6 ns)
   2870.0 |########################################
   2883.0 |
   2896.1 |####################
   2909.1 |
   2922.1 |
   2935.2 |
   2948.2 |
   2961.2 |
   2974.3 |####################
   2987.3 |
   3000.3 |####################
   3013.4 |
   3026.4 |
   3039.4 |
   3052.5 |
   3065.5 |
   3078.5 |
   3091.6 |
   3104.6 |
   3117.6 |
  (0 below, 1 above range)

cl_foldonly (n=6, range 1500.0-1561.1 ns)
   1500.0 |########################################
   1503.1 |########################################
   1506.1 |
   1509.2 |########################################
   1512.2 |########################################
   1515.3 |
   1518.3 |
   1521.4 |
   1524.4 |
   1527.5 |
   1530.5 |
   1533.6 |
   1536.6 |
   1539.7 |
   1542.7 |
   1545.8 |
   1548.8 |
   1551.9 |
   1554.9 |########################################
   1558.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **cl_foldcse**: bridge=82.7% of algo (FFI overhead may distort results)
- **cl_foldonly**: bridge=98.3% of algo (FFI overhead may distort results)
