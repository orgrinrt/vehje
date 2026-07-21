# Interner intern hot path: FNV vs FxHash

2 variants, 6 samples per variant.
Baseline: **hx_interner__fnv**

## Highlights

Baseline for all deltas below: **hx_interner__fnv**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_interner__fnv dominates: 18% faster than the next best (hx_interner__fx)

hx_interner__fnv (1.80 us) leads hx_interner__fx (2.12 us) by 18%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_interner__fnv shows alternating (throttle bounce) (autocorr -0.69)

hx_interner__fnv's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (hx_interner__fnv)

The baseline hx_interner__fnv is the fastest (1.80 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_interner__fnv) is the fastest** at 1800.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.18x (fastest 1800.7 ns, slowest 2117.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_interner__fnv | 4479ns | 4464ns | 4291ns | 4410ns | 4678ns | base |
| hx_interner__fx | 4686ns | 4815ns | 3879ns | 4734ns | 5017ns | +4.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_interner__fnv | 1791ns | 1698ns | 1865ns | base | 0.143 |
| hx_interner__fx | 2066ns | 1745ns | 2204ns | +15.36% | 0.124 |

## Performance model

- Peak throughput: **0.151 Gops/s** (hx_interner__fnv; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_interner__fnv | 0.142 | 94.3% |
| hx_interner__fx | 0.121 | 80.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_interner__fnv | 4479ns | 4479ns | base |
| hx_interner__fx | 4686ns | 4686ns | +4.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_interner__fnv | 1801ns | base | --- | [1709, 1865] | --- | --- | --- | --- |
| hx_interner__fx | 2118ns | +329.5ns (+18.3%) | [+100, +396]ns | [1877, 2204] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_interner__fnv | hx_interner__fx |
|---|---|---|
| 1 | 1835ns | -4.9% |
| 2 | 1698ns | +22.3% |
| 3 | 1894ns | +17.6% |
| 4 | 1770ns | +23.3% |
| 5 | 1832ns | +17.8% |
| 6 | 1719ns | +16.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_interner__fnv | -0.690 | HIGH- (thermal bounce) |
| hx_interner__fx | 0.143 | ok |

**Consistency summary:**

- **hx_interner__fx**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_interner__fnv | 4.6ns | 1791.3ns | 0.3% |  |
| hx_interner__fx | 4.8ns | 2066.4ns | 0.2% |  |

## Distribution (algo ns)

```
hx_interner__fnv (n=6, range 1698.3-1864.6 ns)
   1698.3 |####################
   1706.6 |
   1714.9 |####################
   1723.2 |
   1731.6 |
   1739.9 |
   1748.2 |
   1756.5 |
   1764.8 |####################
   1773.1 |
   1781.4 |
   1789.8 |
   1798.1 |
   1806.4 |
   1814.7 |
   1823.0 |
   1831.3 |########################################
   1839.7 |
   1848.0 |
   1856.3 |
  (0 below, 1 above range)

hx_interner__fx (n=6, range 1744.6-2204.2 ns)
   1744.6 |########################################
   1767.6 |
   1790.6 |
   1813.5 |
   1836.5 |
   1859.5 |
   1882.5 |
   1905.5 |
   1928.4 |
   1951.4 |
   1974.4 |
   1997.4 |########################################
   2020.4 |
   2043.3 |
   2066.3 |########################################
   2089.3 |
   2112.3 |
   2135.3 |
   2158.2 |########################################
   2181.2 |########################################
  (0 below, 1 above range)

```
