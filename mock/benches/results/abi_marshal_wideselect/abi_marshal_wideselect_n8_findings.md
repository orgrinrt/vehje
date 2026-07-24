# abi_marshal (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_wideselect_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_wideselect_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_wideselect_marshal_null dominates: 9808% faster than the next best (abi_marshal_wideselect_aos)

abi_marshal_wideselect_marshal_null (20.84 us) leads abi_marshal_wideselect_aos (2.06 ms) by 9808%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_wideselect_marshal_null beats baseline by 99% (significant)

abi_marshal_wideselect_marshal_null is -2.04 ms (99%) faster than baseline abi_marshal_wideselect_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_wideselect_soa_transposed is an outlier: 100.6x slower than the field

abi_marshal_wideselect_soa_transposed (2.10 ms) is 100.6x the fastest (20.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_wideselect_soa_transposed shows alternating (throttle bounce) (autocorr -0.60)

abi_marshal_wideselect_soa_transposed's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_wideselect_marshal_null} vs {abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_native, abi_marshal_wideselect_soa_transposed} (9808% apart)

The field splits into a fast tier {abi_marshal_wideselect_marshal_null} and a slow tier {abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_native, abi_marshal_wideselect_soa_transposed} with a 9808% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 100.6x the fastest

Fastest abi_marshal_wideselect_marshal_null (20.84 us) to slowest abi_marshal_wideselect_soa_transposed (2.10 ms): 100.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_wideselect_marshal_null** at 20838.8 ns median (-99.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 100.56x (fastest 20838.8 ns, slowest 2095514.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2074892ns | 2067201ns | 2059663ns | 2065893ns | 2096006ns | base |
| abi_marshal_wideselect_marshal_null | 23171ns | 23159ns | 22230ns | 23042ns | 23834ns | -98.88% |
| abi_marshal_wideselect_soa_native | 2083061ns | 2076793ns | 2074016ns | 2076409ns | 2097562ns | +0.39% |
| abi_marshal_wideselect_soa_transposed | 2096055ns | 2097994ns | 2085273ns | 2095276ns | 2102616ns | +1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2072287ns | 2056992ns | 2093283ns | base | 0.000 |
| abi_marshal_wideselect_marshal_null | 20874ns | 20067ns | 21489ns | -98.99% | 0.000 |
| abi_marshal_wideselect_soa_native | 2080491ns | 2071546ns | 2094949ns | +0.40% | 0.000 |
| abi_marshal_wideselect_soa_transposed | 2093557ns | 2082857ns | 2100056ns | +1.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 42341.8 | 2078527.2 | 2072286.7 | n/a |
| abi_marshal_wideselect_marshal_null | 29476.6 | 21071.1 | 20874.1 | n/a |
| abi_marshal_wideselect_soa_native | 41145.8 | 2078719.3 | 2080491.2 | 0 |
| abi_marshal_wideselect_soa_transposed | 38792.6 | 2093932.0 | 2093557.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_wideselect_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_wideselect_aos | 0.000 | 1.0% |
| abi_marshal_wideselect_marshal_null | 0.000 | 96.3% |
| abi_marshal_wideselect_soa_native | 0.000 | 1.0% |
| abi_marshal_wideselect_soa_transposed | 0.000 | 1.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_wideselect_aos | 2074892ns | 2074892ns | base |
| abi_marshal_wideselect_marshal_null | 23171ns | 23171ns | -98.88% |
| abi_marshal_wideselect_soa_native | 2083061ns | 2083061ns | +0.39% |
| abi_marshal_wideselect_soa_transposed | 2096055ns | 2096055ns | +1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2064685ns | base | --- | [2058892, 2093283] | --- | --- | --- | --- |
| abi_marshal_wideselect_marshal_null | 20839ns | -2043885.0ns (-99.0%) | [-2072246, -2038106]ns | [20295, 21489] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_wideselect_soa_native | 2074146ns | no significant difference | [-5965, +20000]ns | [2072378, 2094949] | no | 0.2188 | 0.2188 | 0 |
| abi_marshal_wideselect_soa_transposed | 2095515ns | +26208.8ns (+1.3%) | [+3150, +34453]ns | [2085101, 2100056] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_wideselect_aos | abi_marshal_wideselect_marshal_null | abi_marshal_wideselect_soa_native | abi_marshal_wideselect_soa_transposed |
|---|---|---|---|---|
| 1 | 2112228ns | -99.0% | -0.6% | -0.7% |
| 2 | 2074338ns | -99.0% | +0.0% | +1.0% |
| 3 | 2066562ns | -99.0% | +1.1% | +1.4% |
| 4 | 2060792ns | -99.0% | +0.5% | +1.3% |
| 5 | 2062808ns | -99.0% | +0.5% | +1.9% |
| 6 | 2056992ns | -99.0% | +0.8% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_wideselect_aos | 0.187 | ok |
| abi_marshal_wideselect_marshal_null | -0.267 | moderate- |
| abi_marshal_wideselect_soa_native | -0.206 | moderate- |
| abi_marshal_wideselect_soa_transposed | -0.602 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_marshal_wideselect_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_wideselect_soa_native**: won 1/6, lost 4/6
- **abi_marshal_wideselect_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 6299149.4ns | 2072286.7ns | 304.0% | HIGH |
| abi_marshal_wideselect_marshal_null | 169053.8ns | 20874.1ns | 809.9% | HIGH |
| abi_marshal_wideselect_soa_native | 6284980.4ns | 2080491.2ns | 302.1% | HIGH |
| abi_marshal_wideselect_soa_transposed | 6321765.5ns | 2093557.3ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_marshal_wideselect_aos (n=6, range 2056992.5-2093282.9 ns)
  2056992.5 |########################################
  2058807.0 |
  2060621.5 |########################################
  2062436.1 |########################################
  2064250.6 |
  2066065.1 |########################################
  2067879.6 |
  2069694.1 |
  2071508.7 |
  2073323.2 |########################################
  2075137.7 |
  2076952.2 |
  2078766.7 |
  2080581.3 |
  2082395.8 |
  2084210.3 |
  2086024.8 |
  2087839.3 |
  2089653.9 |
  2091468.4 |
  (0 below, 1 above range)

abi_marshal_wideselect_marshal_null (n=6, range 20066.7-21488.8 ns)
  20066.7 |########################################
  20137.8 |
  20208.9 |
  20280.0 |
  20351.1 |
  20422.2 |
  20493.3 |########################################
  20564.4 |########################################
  20635.5 |
  20706.6 |
  20777.7 |
  20848.8 |
  20919.9 |
  20991.0 |########################################
  21062.1 |
  21133.2 |
  21204.3 |
  21275.4 |
  21346.5 |
  21417.6 |########################################
  (0 below, 1 above range)

abi_marshal_wideselect_soa_native (n=6, range 2071545.8-2094949.1 ns)
  2071545.8 |####################
  2072716.0 |########################################
  2073886.1 |####################
  2075056.3 |
  2076226.5 |
  2077396.6 |
  2078566.8 |
  2079737.0 |
  2080907.1 |
  2082077.3 |
  2083247.5 |
  2084417.6 |
  2085587.8 |
  2086758.0 |
  2087928.1 |
  2089098.3 |####################
  2090268.5 |
  2091438.6 |
  2092608.8 |
  2093779.0 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_transposed (n=6, range 2082857.1-2100056.0 ns)
  2082857.1 |####################
  2083717.0 |
  2084577.0 |
  2085436.9 |
  2086296.9 |
  2087156.8 |####################
  2088016.8 |
  2088876.7 |
  2089736.7 |
  2090596.6 |
  2091456.6 |
  2092316.5 |
  2093176.4 |
  2094036.4 |
  2094896.3 |########################################
  2095756.3 |
  2096616.2 |####################
  2097476.2 |
  2098336.1 |
  2099196.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_wideselect_aos**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_marshal_null**: bridge=812.3% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_native**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_transposed**: bridge=301.7% of algo (FFI overhead may distort results)
