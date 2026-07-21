# Field access: direct-offset vs linear-scan vs hash-lookup vs inline-cache

3 variants, 6 samples per variant.
Baseline: **hx_field__direct**

## Highlights

Baseline for all deltas below: **hx_field__direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_field__direct dominates: 50% faster than the next best (hx_field__hash)

hx_field__direct (46 ns) leads hx_field__hash (69 ns) by 50%, a clear separation rather than a photo finish. CV 9.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_field__direct)

The baseline hx_field__direct is the fastest (46 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_field__direct) is the fastest** at 45.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.56x (fastest 45.8 ns, slowest 71.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_field__direct | 2413ns | 2375ns | 2207ns | 2340ns | 2626ns | base |
| hx_field__hash | 2362ns | 2286ns | 2220ns | 2265ns | 2579ns | -2.10% |
| hx_field__linear | 2430ns | 2363ns | 2178ns | 2317ns | 2727ns | +0.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_field__direct | 46ns | 40ns | 51ns | base | 1.392 |
| hx_field__hash | 71ns | 66ns | 80ns | +55.29% | 0.897 |
| hx_field__linear | 71ns | 59ns | 80ns | +55.29% | 0.897 |

## Performance model

- Peak throughput: **1.584 Gops/s** (hx_field__direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_field__direct | 1.397 | 88.2% |
| hx_field__hash | 0.934 | 58.9% |
| hx_field__linear | 0.893 | 56.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_field__direct | 2413ns | 2413ns | base |
| hx_field__hash | 2362ns | 2362ns | -2.10% |
| hx_field__linear | 2430ns | 2430ns | +0.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_field__direct | 46ns | base | --- | [41, 51] | --- | --- | --- | --- |
| hx_field__hash | 69ns | +25.0ns (+54.6%) | [+20, +31]ns | [66, 80] | YES | 0.0313 | 0.0313 | 0 |
| hx_field__linear | 72ns | +27.5ns (+60.0%) | [+17, +31]ns | [62, 80] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_field__direct | hx_field__hash | hx_field__linear |
|---|---|---|---|
| 1 | 50ns | +32.4% | +31.6% |
| 2 | 49ns | +72.9% | +65.9% |
| 3 | 42ns | +65.3% | +73.2% |
| 4 | 40ns | +62.9% | +46.5% |
| 5 | 43ns | +57.3% | +64.1% |
| 6 | 52ns | +45.1% | +53.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_field__direct | 0.192 | ok |
| hx_field__hash | -0.283 | moderate- |
| hx_field__linear | -0.158 | ok |

**Consistency summary:**

- **hx_field__hash**: won 0/6, lost 6/6
- **hx_field__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_field__direct | 3.2ns | 46.0ns | 7.1% | HIGH |
| hx_field__hash | 2.4ns | 71.4ns | 3.4% |  |
| hx_field__linear | 2.6ns | 71.4ns | 3.6% |  |

## Distribution (algo ns)

```
hx_field__direct (n=6, range 40.4-50.9 ns)
     40.4 |########################################
     40.9 |
     41.4 |
     42.0 |########################################
     42.5 |########################################
     43.0 |
     43.5 |
     44.1 |
     44.6 |
     45.1 |
     45.6 |
     46.1 |
     46.7 |
     47.2 |
     47.7 |
     48.2 |########################################
     48.8 |
     49.3 |
     49.8 |########################################
     50.3 |
  (0 below, 1 above range)

hx_field__hash (n=6, range 65.8-79.6 ns)
     65.8 |########################################
     66.5 |
     67.2 |####################
     67.9 |
     68.6 |
     69.2 |####################
     69.9 |
     70.6 |
     71.3 |
     72.0 |
     72.7 |
     73.4 |
     74.1 |
     74.8 |####################
     75.5 |
     76.1 |
     76.8 |
     77.5 |
     78.2 |
     78.9 |
  (0 below, 1 above range)

hx_field__linear (n=6, range 59.2-80.0 ns)
     59.2 |########################################
     60.2 |
     61.3 |
     62.3 |
     63.4 |
     64.4 |
     65.4 |########################################
     66.5 |
     67.5 |
     68.6 |
     69.6 |########################################
     70.6 |
     71.7 |
     72.7 |########################################
     73.8 |
     74.8 |
     75.8 |
     76.9 |
     77.9 |
     79.0 |########################################
  (0 below, 1 above range)

```
