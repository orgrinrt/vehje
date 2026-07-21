# Iterator fusion: materialized intermediates vs fused pipeline

2 variants, 6 samples per variant.
Baseline: **hx_iter_fusion__fused**

## Highlights

Baseline for all deltas below: **hx_iter_fusion__fused**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_iter_fusion__fused dominates: 679% faster than the next best (hx_iter_fusion__materialized)

hx_iter_fusion__fused (299 ns) leads hx_iter_fusion__materialized (2.33 us) by 679%, a clear separation rather than a photo finish. CV 9.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_iter_fusion__fused is fastest but the noisiest (CV 9.2%)

hx_iter_fusion__fused wins on median (299 ns) yet has the highest variance (CV 9.2%), while hx_iter_fusion__materialized is the steadiest (CV 6.1%, 2.33 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_iter_fusion__fused)

The baseline hx_iter_fusion__fused is the fastest (299 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.8x the fastest

Fastest hx_iter_fusion__fused (299 ns) to slowest hx_iter_fusion__materialized (2.33 us): 7.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_iter_fusion__fused) is the fastest** at 298.5 ns median
- 1 variant significantly slower than baseline
- Spread: 7.79x (fastest 298.5 ns, slowest 2325.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 2860ns | 2872ns | 2360ns | 2868ns | 3098ns | base |
| hx_iter_fusion__materialized | 5140ns | 5149ns | 4516ns | 5133ns | 5462ns | +79.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_iter_fusion__fused | 299ns | 250ns | 324ns | base | 3.427 |
| hx_iter_fusion__materialized | 2303ns | 2075ns | 2438ns | +670.57% | 0.445 |

## Performance model

- Peak throughput: **4.096 Gops/s** (hx_iter_fusion__fused; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_iter_fusion__fused | 3.430 | 83.7% |
| hx_iter_fusion__materialized | 0.440 | 10.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_iter_fusion__fused | 2860ns | 2860ns | base |
| hx_iter_fusion__materialized | 5140ns | 5140ns | +79.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_iter_fusion__fused | 299ns | base | --- | [274, 324] | --- | --- | --- | --- |
| hx_iter_fusion__materialized | 2326ns | +2024.6ns (+678.1%) | [+1870, +2117]ns | [2144, 2438] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_iter_fusion__fused | hx_iter_fusion__materialized |
|---|---|---|
| 1 | 250ns | +730.0% |
| 2 | 298ns | +642.6% |
| 3 | 298ns | +683.0% |
| 4 | 303ns | +667.1% |
| 5 | 345ns | +638.3% |
| 6 | 299ns | +677.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_iter_fusion__fused | 0.059 | ok |
| hx_iter_fusion__materialized | 0.248 | moderate+ |

**Consistency summary:**

- **hx_iter_fusion__materialized**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_iter_fusion__fused | 5.3ns | 298.8ns | 1.8% |  |
| hx_iter_fusion__materialized | 24.4ns | 2302.6ns | 1.1% |  |

## Distribution (algo ns)

```
hx_iter_fusion__fused (n=6, range 250.0-324.0 ns)
    250.0 |####################
    253.7 |
    257.4 |
    261.1 |
    264.8 |
    268.5 |
    272.2 |
    275.9 |
    279.6 |
    283.3 |
    287.0 |
    290.7 |
    294.4 |########################################
    298.1 |####################
    301.8 |####################
    305.5 |
    309.2 |
    312.9 |
    316.6 |
    320.3 |
  (0 below, 1 above range)

hx_iter_fusion__materialized (n=6, range 2075.0-2438.3 ns)
   2075.0 |####################
   2093.2 |
   2111.3 |
   2129.5 |
   2147.7 |
   2165.8 |
   2184.0 |
   2202.2 |####################
   2220.3 |
   2238.5 |
   2256.7 |
   2274.8 |
   2293.0 |
   2311.2 |########################################
   2329.3 |####################
   2347.5 |
   2365.7 |
   2383.8 |
   2402.0 |
   2420.2 |
  (0 below, 1 above range)

```
