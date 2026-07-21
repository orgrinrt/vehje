# Interner intern hot path: FNV vs FxHash

2 variants, 6 samples per variant.
Baseline: **hx_interner__fnv**

## Highlights

Baseline for all deltas below: **hx_interner__fnv**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (hx_interner__fnv)

The baseline hx_interner__fnv is the fastest (186.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader hx_interner__fnv vs stability leader hx_interner__fx (+9% speed for 1.1x steadier)

hx_interner__fnv is fastest (186.54 us, CV 3.0%); hx_interner__fx gives up 9.4% median for 1.1x lower variance (CV 2.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (hx_interner__fnv) is the fastest** at 186541.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 186541.8 ns, slowest 204098.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_interner__fnv | 190403ns | 189075ns | 183717ns | 187777ns | 197686ns | base |
| hx_interner__fx | 206989ns | 206652ns | 199889ns | 205100ns | 213373ns | +8.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_interner__fnv | 187805ns | 181265ns | 194912ns | base | 0.087 |
| hx_interner__fx | 204534ns | 197699ns | 211023ns | +8.91% | 0.080 |

## Performance model

- Peak throughput: **0.090 Gops/s** (hx_interner__fnv; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_interner__fnv | 0.088 | 97.2% |
| hx_interner__fx | 0.080 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_interner__fnv | 190403ns | 190403ns | base |
| hx_interner__fx | 206989ns | 206989ns | +8.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_interner__fnv | 186542ns | base | --- | [181960, 194912] | --- | --- | --- | --- |
| hx_interner__fx | 204099ns | +18262.8ns (+9.8%) | [+4662, +27262]ns | [198480, 211023] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_interner__fnv | hx_interner__fx |
|---|---|---|
| 1 | 195035ns | +2.5% |
| 2 | 182655ns | +14.0% |
| 3 | 194788ns | +2.3% |
| 4 | 183358ns | +7.8% |
| 5 | 181265ns | +15.9% |
| 6 | 189726ns | +11.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_interner__fnv | -0.453 | moderate- |
| hx_interner__fx | 0.008 | ok |

**Consistency summary:**

- **hx_interner__fx**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_interner__fnv | 5.7ns | 187804.6ns | 0.0% |  |
| hx_interner__fx | 3.7ns | 204533.6ns | 0.0% |  |

## Distribution (algo ns)

```
hx_interner__fnv (n=6, range 181265.4-194911.6 ns)
  181265.4 |########################################
  181947.7 |
  182630.0 |########################################
  183312.3 |########################################
  183994.6 |
  184677.0 |
  185359.3 |
  186041.6 |
  186723.9 |
  187406.2 |
  188088.5 |
  188770.8 |
  189453.1 |########################################
  190135.5 |
  190817.8 |
  191500.1 |
  192182.4 |
  192864.7 |
  193547.0 |
  194229.3 |########################################
  (0 below, 1 above range)

hx_interner__fx (n=6, range 197699.2-211022.7 ns)
  197699.2 |########################################
  198365.4 |
  199031.6 |########################################
  199697.7 |########################################
  200363.9 |
  201030.1 |
  201696.2 |
  202362.4 |
  203028.6 |
  203694.8 |
  204361.0 |
  205027.1 |
  205693.3 |
  206359.5 |
  207025.7 |
  207691.8 |########################################
  208358.0 |
  209024.2 |
  209690.4 |########################################
  210356.5 |
  (0 below, 1 above range)

```
