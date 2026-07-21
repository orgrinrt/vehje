# Interner intern hot path: FNV vs FxHash

2 variants, 6 samples per variant.
Baseline: **hx_interner__fnv**

## Highlights

Baseline for all deltas below: **hx_interner__fnv**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_interner__fnv dominates: 11% faster than the next best (hx_interner__fx)

hx_interner__fnv (7.78 us) leads hx_interner__fx (8.66 us) by 11%, a clear separation rather than a photo finish. CV 9.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_interner__fnv is fastest but the noisiest (CV 9.6%)

hx_interner__fnv wins on median (7.78 us) yet has the highest variance (CV 9.6%), while hx_interner__fx is the steadiest (CV 5.4%, 8.66 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_interner__fnv shows alternating (throttle bounce) (autocorr -0.75)

hx_interner__fnv's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (hx_interner__fnv)

The baseline hx_interner__fnv is the fastest (7.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_interner__fnv) is the fastest** at 7780.6 ns median
- Spread: 1.11x (fastest 7780.6 ns, slowest 8662.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_interner__fnv | 10518ns | 10215ns | 9499ns | 10163ns | 11559ns | base |
| hx_interner__fx | 10966ns | 11179ns | 9658ns | 11137ns | 11364ns | +4.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_interner__fnv | 8007ns | 7078ns | 8975ns | base | 0.128 |
| hx_interner__fx | 8504ns | 7485ns | 8813ns | +6.20% | 0.120 |

## Performance model

- Peak throughput: **0.145 Gops/s** (hx_interner__fnv; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_interner__fnv | 0.132 | 91.0% |
| hx_interner__fx | 0.118 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_interner__fnv | 10518ns | 10518ns | base |
| hx_interner__fx | 10966ns | 10966ns | +4.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_interner__fnv | 7781ns | base | --- | [7266, 8975] | --- | --- | --- | --- |
| hx_interner__fx | 8663ns | no significant difference | [-313, +1236]ns | [8035, 8813] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_interner__fnv | hx_interner__fx |
|---|---|---|
| 1 | 7078ns | +5.8% |
| 2 | 8899ns | -1.9% |
| 3 | 7455ns | +18.4% |
| 4 | 9052ns | -5.1% |
| 5 | 7480ns | +14.8% |
| 6 | 8081ns | +8.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_interner__fnv | -0.745 | HIGH- (thermal bounce) |
| hx_interner__fx | -0.077 | ok |

**Consistency summary:**

- **hx_interner__fx**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_interner__fnv | 4.0ns | 8007.4ns | 0.1% |  |
| hx_interner__fx | 3.3ns | 8503.6ns | 0.0% |  |

## Distribution (algo ns)

```
hx_interner__fnv (n=6, range 7077.5-8975.5 ns)
   7077.5 |########################################
   7172.4 |
   7267.3 |
   7362.2 |########################################
   7457.1 |########################################
   7552.0 |
   7646.9 |
   7741.8 |
   7836.7 |
   7931.6 |
   8026.5 |########################################
   8121.4 |
   8216.3 |
   8311.2 |
   8406.1 |
   8501.0 |
   8595.9 |
   8690.8 |
   8785.7 |
   8880.6 |########################################
  (0 below, 1 above range)

hx_interner__fx (n=6, range 7485.0-8813.3 ns)
   7485.0 |####################
   7551.4 |
   7617.8 |
   7684.2 |
   7750.7 |
   7817.1 |
   7883.5 |
   7949.9 |
   8016.3 |
   8082.7 |
   8149.1 |
   8215.6 |
   8282.0 |
   8348.4 |
   8414.8 |
   8481.2 |
   8547.6 |########################################
   8614.1 |
   8680.5 |####################
   8746.9 |####################
  (0 below, 1 above range)

```
