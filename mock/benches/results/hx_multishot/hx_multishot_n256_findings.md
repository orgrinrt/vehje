# Bounded multi-shot enumeration vs single-shot

2 variants, 6 samples per variant.
Baseline: **hx_multishot__single**

## Highlights

Baseline for all deltas below: **hx_multishot__single**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_multishot__single dominates: 818% faster than the next best (hx_multishot__multishot)

hx_multishot__single (19 ns) leads hx_multishot__multishot (176 ns) by 818%, a clear separation rather than a photo finish. CV 10.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_multishot__single shows alternating (throttle bounce) (autocorr -0.69)

hx_multishot__single's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (hx_multishot__single)

The baseline hx_multishot__single is the fastest (19 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.2x the fastest

Fastest hx_multishot__single (19 ns) to slowest hx_multishot__multishot (176 ns): 9.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hx_multishot__multishot is inconsistent: worst-20% is 4.1x its best-20%

hx_multishot__multishot's best 20% of batches run at 168 ns but its worst 20% at 686 ns (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (hx_multishot__single) is the fastest** at 19.2 ns median
- 1 variant significantly slower than baseline
- Spread: 9.18x (fastest 19.2 ns, slowest 176.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_multishot__multishot | 2956ns | 2584ns | 2297ns | 2503ns | 3964ns | +26.71% |
| hx_multishot__single | 2333ns | 2281ns | 2195ns | 2253ns | 2521ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_multishot__multishot | 344ns | 168ns | 686ns | +1680.93% | 0.744 |
| hx_multishot__single | 19ns | 17ns | 22ns | base | 13.253 |

## Performance model

- Peak throughput: **15.329 Gops/s** (hx_multishot__single; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_multishot__multishot | 1.452 | 9.5% |
| hx_multishot__single | 13.333 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_multishot__multishot | 2956ns | 2956ns | +26.71% |
| hx_multishot__single | 2333ns | 2333ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_multishot__single | 19ns | base | --- | [17, 22] | --- | --- | --- | --- |
| hx_multishot__multishot | 176ns | +158.3ns (+824.5%) | [+149, +666]ns | [170, 686] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_multishot__single | hx_multishot__multishot |
|---|---|---|
| 1 | 19ns | +776.6% |
| 2 | 22ns | +742.7% |
| 3 | 17ns | +928.1% |
| 4 | 21ns | +705.7% |
| 5 | 17ns | +6817.5% |
| 6 | 19ns | +841.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_multishot__multishot | -0.235 | moderate- |
| hx_multishot__single | -0.689 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hx_multishot__multishot**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_multishot__multishot | 4.9ns | 344.0ns | 1.4% |  |
| hx_multishot__single | 2.9ns | 19.3ns | 14.8% | HIGH |

## Distribution (algo ns)

```
hx_multishot__multishot (n=6, range 168.3-686.2 ns)
    168.3 |########################################
    194.2 |
    220.1 |
    246.0 |
    271.9 |
    297.8 |
    323.7 |
    349.6 |
    375.5 |
    401.4 |
    427.3 |
    453.2 |
    479.1 |
    505.0 |
    530.9 |
    556.8 |
    582.7 |
    608.6 |
    634.5 |
    660.4 |
  (0 below, 1 above range)

hx_multishot__single (n=6, range 16.7-21.9 ns)
     16.7 |####################
     17.0 |####################
     17.2 |
     17.5 |
     17.7 |
     18.0 |
     18.2 |
     18.5 |
     18.8 |
     19.0 |########################################
     19.3 |
     19.5 |
     19.8 |
     20.0 |
     20.3 |
     20.6 |
     20.8 |
     21.1 |####################
     21.3 |
     21.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **hx_multishot__multishot**: CV=109.1% (high variance, measurements may be unstable)
- **hx_multishot__multishot**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **hx_multishot__single**: bridge=14.1% of algo (FFI overhead may distort results)
