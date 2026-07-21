# Cheap lowering node-count reduction: fold+CSE vs fold-only (metric = node count, not time)

2 variants, 6 samples per variant.
Baseline: **cl_foldonly**

## Highlights

Baseline for all deltas below: **cl_foldonly**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cl_foldonly dominates: 50% faster than the next best (cl_foldcse)

cl_foldonly (132 ns) leads cl_foldcse (197 ns) by 50%, a clear separation rather than a photo finish. CV 36.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cl_foldonly is fastest but the noisiest (CV 36.4%)

cl_foldonly wins on median (132 ns) yet has the highest variance (CV 36.4%), while cl_foldcse is the steadiest (CV 34.7%, 197 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (cl_foldonly)

The baseline cl_foldonly is the fastest (132 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### cl_foldonly is inconsistent: worst-20% is 1.7x its best-20%

cl_foldonly's best 20% of batches run at 126 ns but its worst 20% at 213 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (cl_foldonly) is the fastest** at 131.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.50x (fastest 131.7 ns, slowest 197.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cl_foldcse | 2822ns | 2371ns | 2353ns | 2366ns | 3741ns | -0.65% |
| cl_foldonly | 2841ns | 2376ns | 2301ns | 2352ns | 3844ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cl_foldcse | 231ns | 191ns | 304ns | +47.27% | 0.277 |
| cl_foldonly | 157ns | 126ns | 213ns | base | 0.408 |

## Performance model

- Peak throughput: **0.507 Gops/s** (cl_foldonly; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cl_foldcse | 0.324 | 64.0% |
| cl_foldonly | 0.486 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cl_foldcse | 2822ns | 2822ns | -0.65% |
| cl_foldonly | 2841ns | 2841ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cl_foldonly | 132ns | base | --- | [126, 213] | --- | --- | --- | --- |
| cl_foldcse | 197ns | +65.8ns (+50.0%) | [+60, +97]ns | [192, 304] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | cl_foldonly | cl_foldcse |
|---|---|---|
| 1 | 260ns | +47.2% |
| 2 | 166ns | +36.1% |
| 3 | 132ns | +45.2% |
| 4 | 126ns | +56.8% |
| 5 | 132ns | +49.4% |
| 6 | 127ns | +52.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cl_foldcse | 0.115 | ok |
| cl_foldonly | 0.221 | moderate+ |

**Consistency summary:**

- **cl_foldcse**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cl_foldcse | 293.2ns | 231.2ns | 126.8% | HIGH |
| cl_foldonly | 126.4ns | 157.0ns | 80.5% | HIGH |

## Distribution (algo ns)

```
cl_foldcse (n=6, range 191.2-304.1 ns)
    191.2 |########################################
    196.8 |#############
    202.5 |
    208.1 |
    213.8 |
    219.4 |
    225.1 |#############
    230.7 |
    236.4 |
    242.0 |
    247.7 |
    253.3 |
    259.0 |
    264.6 |
    270.3 |
    275.9 |
    281.6 |
    287.2 |
    292.9 |
    298.5 |
  (0 below, 1 above range)

cl_foldonly (n=6, range 126.2-212.9 ns)
    126.2 |########################################
    130.5 |########################################
    134.9 |
    139.2 |
    143.5 |
    147.9 |
    152.2 |
    156.5 |
    160.9 |
    165.2 |####################
    169.6 |
    173.9 |
    178.2 |
    182.6 |
    186.9 |
    191.2 |
    195.6 |
    199.9 |
    204.2 |
    208.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **cl_foldcse**: CV=29.6% (high variance, measurements may be unstable)
- **cl_foldcse**: bridge=128.0% of algo (FFI overhead may distort results)
- **cl_foldonly**: CV=30.5% (high variance, measurements may be unstable)
- **cl_foldonly**: bridge=80.5% of algo (FFI overhead may distort results)
