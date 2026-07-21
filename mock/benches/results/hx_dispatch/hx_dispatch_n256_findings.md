# Interpreter dispatch shapes: switch vs fnptr-table vs computed

2 variants, 6 samples per variant.
Baseline: **hx_dispatch__switch**

## Highlights

Baseline for all deltas below: **hx_dispatch__switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_dispatch__switch dominates: 102% faster than the next best (hx_dispatch__fnptr)

hx_dispatch__switch (365 ns) leads hx_dispatch__fnptr (738 ns) by 102%, a clear separation rather than a photo finish. CV 6.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_dispatch__switch)

The baseline hx_dispatch__switch is the fastest (365 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_dispatch__switch) is the fastest** at 365.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.02x (fastest 365.4 ns, slowest 738.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_dispatch__fnptr | 3185ns | 3305ns | 2750ns | 3234ns | 3331ns | +6.58% |
| hx_dispatch__switch | 2989ns | 2940ns | 2753ns | 2935ns | 3188ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_dispatch__fnptr | 715ns | 615ns | 754ns | +92.04% | 0.358 |
| hx_dispatch__switch | 372ns | 345ns | 401ns | base | 0.687 |

## Performance model

- Peak throughput: **0.741 Gops/s** (hx_dispatch__switch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_dispatch__fnptr | 0.347 | 46.8% |
| hx_dispatch__switch | 0.701 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_dispatch__fnptr | 3185ns | 3185ns | +6.58% |
| hx_dispatch__switch | 2989ns | 2989ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_dispatch__switch | 365ns | base | --- | [351, 401] | --- | --- | --- | --- |
| hx_dispatch__fnptr | 738ns | +349.6ns (+95.7%) | [+295, +384]ns | [654, 754] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_dispatch__switch | hx_dispatch__fnptr |
|---|---|---|
| 1 | 345ns | +77.9% |
| 2 | 360ns | +108.4% |
| 3 | 357ns | +94.3% |
| 4 | 408ns | +78.4% |
| 5 | 393ns | +92.3% |
| 6 | 370ns | +101.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_dispatch__fnptr | -0.191 | ok |
| hx_dispatch__switch | 0.231 | moderate+ |

**Consistency summary:**

- **hx_dispatch__fnptr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_dispatch__fnptr | 3.3ns | 715.2ns | 0.5% |  |
| hx_dispatch__switch | 4.5ns | 372.4ns | 1.2% |  |

## Distribution (algo ns)

```
hx_dispatch__fnptr (n=6, range 614.6-753.7 ns)
    614.6 |####################
    621.6 |
    628.5 |
    635.5 |
    642.4 |
    649.4 |
    656.3 |
    663.3 |
    670.2 |
    677.2 |
    684.2 |
    691.1 |####################
    698.1 |
    705.0 |
    712.0 |
    718.9 |
    725.9 |####################
    732.8 |
    739.8 |
    746.7 |########################################
  (0 below, 1 above range)

hx_dispatch__switch (n=6, range 345.4-400.8 ns)
    345.4 |########################################
    348.2 |
    350.9 |
    353.7 |
    356.5 |########################################
    359.2 |########################################
    362.0 |
    364.8 |
    367.6 |
    370.3 |########################################
    373.1 |
    375.9 |
    378.6 |
    381.4 |
    384.2 |
    386.9 |
    389.7 |
    392.5 |########################################
    395.3 |
    398.0 |
  (0 below, 1 above range)

```
