# Record update: always-copy vs in-place-when-unique (exact-meet)

2 variants, 6 samples per variant.
Baseline: **hx_reuse__reuse**

## Highlights

Baseline for all deltas below: **hx_reuse__reuse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_reuse__reuse dominates: 34% faster than the next best (hx_reuse__copy)

hx_reuse__reuse (1.98 us) leads hx_reuse__copy (2.65 us) by 34%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_reuse__copy shows alternating (throttle bounce) (autocorr -0.51)

hx_reuse__copy's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (hx_reuse__reuse)

The baseline hx_reuse__reuse is the fastest (1.98 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_reuse__reuse) is the fastest** at 1981.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.34x (fastest 1981.8 ns, slowest 2647.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_reuse__copy | 4837ns | 4828ns | 4759ns | 4823ns | 4896ns | +15.17% |
| hx_reuse__reuse | 4200ns | 4214ns | 4027ns | 4169ns | 4332ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_reuse__copy | 2647ns | 2595ns | 2690ns | +34.77% | 1.547 |
| hx_reuse__reuse | 1964ns | 1888ns | 2009ns | base | 2.085 |

## Performance model

- Peak throughput: **2.169 Gops/s** (hx_reuse__reuse; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_reuse__copy | 1.547 | 71.3% |
| hx_reuse__reuse | 2.067 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_reuse__copy | 4837ns | 4837ns | +15.17% |
| hx_reuse__reuse | 4200ns | 4200ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_reuse__reuse | 1982ns | base | --- | [1902, 2009] | --- | --- | --- | --- |
| hx_reuse__copy | 2647ns | +669.2ns (+33.8%) | [+602, +778]ns | [2606, 2690] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_reuse__reuse | hx_reuse__copy |
|---|---|---|
| 1 | 1888ns | +43.8% |
| 2 | 2000ns | +29.7% |
| 3 | 2011ns | +32.4% |
| 4 | 1963ns | +34.9% |
| 5 | 2006ns | +30.4% |
| 6 | 1917ns | +38.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_reuse__copy | -0.507 | HIGH- (thermal bounce) |
| hx_reuse__reuse | -0.236 | moderate- |

**Consistency summary:**

- **hx_reuse__copy**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_reuse__copy | 2.4ns | 2647.4ns | 0.1% |  |
| hx_reuse__reuse | 2.8ns | 1964.4ns | 0.1% |  |

## Distribution (algo ns)

```
hx_reuse__copy (n=6, range 2595.0-2689.6 ns)
   2595.0 |########################################
   2599.7 |
   2604.5 |
   2609.2 |
   2613.9 |########################################
   2618.7 |
   2623.4 |
   2628.1 |
   2632.8 |
   2637.6 |
   2642.3 |########################################
   2647.0 |########################################
   2651.8 |
   2656.5 |
   2661.2 |########################################
   2666.0 |
   2670.7 |
   2675.4 |
   2680.1 |
   2684.9 |
  (0 below, 1 above range)

hx_reuse__reuse (n=6, range 1888.3-2008.7 ns)
   1888.3 |########################################
   1894.3 |
   1900.3 |
   1906.4 |
   1912.4 |########################################
   1918.4 |
   1924.4 |
   1930.4 |
   1936.5 |
   1942.5 |
   1948.5 |
   1954.5 |
   1960.5 |########################################
   1966.6 |
   1972.6 |
   1978.6 |
   1984.6 |
   1990.6 |
   1996.7 |########################################
   2002.7 |########################################
  (0 below, 1 above range)

```
