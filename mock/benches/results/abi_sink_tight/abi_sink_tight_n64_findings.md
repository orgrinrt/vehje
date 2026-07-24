# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (20.22 us) is smaller than the fastest variant's own run-to-run std-dev (89.88 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.02 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader abi_sink_tight_null_sink vs stability leader abi_sink_tight_batched_sink_decode (+1% speed for 21.6x steadier)

abi_sink_tight_null_sink is fastest (2.02 ms, CV 4.5%); abi_sink_tight_batched_sink_decode gives up 0.6% median for 21.6x lower variance (CV 0.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.0% of the fastest

All 4 variants sit between 2.02 ms and 2.04 ms - a 1.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2015621.4 ns median
- Spread: 1.01x (fastest 2015621.4 ns, slowest 2035841.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2228588ns | 2026288ns | 2025171ns | 2025956ns | 2634243ns | +7.16% |
| abi_sink_tight_batched_sink_decode | 2029713ns | 2030466ns | 2024295ns | 2028568ns | 2034140ns | -2.40% |
| abi_sink_tight_null_sink | 2079609ns | 2018669ns | 2013179ns | 2017969ns | 2205285ns | base |
| abi_sink_tight_per_record_sink | 2054258ns | 2038826ns | 2035638ns | 2037799ns | 2088255ns | -1.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2224894ns | 2022218ns | 2629104ns | +7.17% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2026744ns | 2021410ns | 2031261ns | -2.37% | 0.000 |
| abi_sink_tight_null_sink | 2075988ns | 2010165ns | 2200474ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2051068ns | 2032515ns | 2084744ns | -1.20% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 69760.4 | 2177320.6 | 2224894.4 | n/a |
| abi_sink_tight_batched_sink_decode | 53838.4 | 2025301.7 | 2026743.7 | n/a |
| abi_sink_tight_null_sink | 67947.7 | 2086306.6 | 2075987.6 | n/a |
| abi_sink_tight_per_record_sink | 58053.7 | 2049822.3 | 2051067.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.3% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_tight_null_sink | 0.000 | 99.7% |
| abi_sink_tight_per_record_sink | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2228588ns | 2228588ns | +7.16% |
| abi_sink_tight_batched_sink_decode | 2029713ns | 2029713ns | -2.40% |
| abi_sink_tight_null_sink | 2079609ns | 2079609ns | base |
| abi_sink_tight_per_record_sink | 2054258ns | 2054258ns | -1.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2015621ns | base | --- | [2011868, 2200474] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2023329ns | no significant difference | [-31431, +469543]ns | [2022250, 2629104] | no | 0.6563 | 0.2188 | 0 |
| abi_sink_tight_batched_sink_decode | 2027352ns | no significant difference | [-175294, +18333]ns | [2021618, 2031261] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_tight_per_record_sink | 2035841ns | no significant difference | [-115730, +23974]ns | [2032618, 2084744] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2014698ns | +0.4% | +0.4% | +0.9% |
| 2 | 2013570ns | +0.4% | +0.7% | +1.1% |
| 3 | 2010165ns | +0.6% | +1.2% | +1.3% |
| 4 | 2016545ns | +0.3% | +0.6% | +0.8% |
| 5 | 2169200ns | -3.2% | -6.5% | -1.8% |
| 6 | 2231747ns | +41.5% | -9.4% | -8.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | 0.029 | ok |
| abi_sink_tight_batched_sink_decode | -0.013 | ok |
| abi_sink_tight_null_sink | 0.430 | moderate+ |
| abi_sink_tight_per_record_sink | -0.204 | moderate- |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 1/6, lost 5/6
- **abi_sink_tight_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_tight_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6688577.6ns | 2224894.4ns | 300.6% | HIGH |
| abi_sink_tight_batched_sink_decode | 6135219.1ns | 2026743.7ns | 302.7% | HIGH |
| abi_sink_tight_null_sink | 6359663.4ns | 2075987.6ns | 306.3% | HIGH |
| abi_sink_tight_per_record_sink | 6219961.9ns | 2051067.7ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2022217.9-2629104.0 ns)
  2022217.9 |########################################
  2052562.2 |
  2082906.5 |##########
  2113250.8 |
  2143595.1 |
  2173939.4 |
  2204283.7 |
  2234628.0 |
  2264972.3 |
  2295316.6 |
  2325660.9 |
  2356005.2 |
  2386349.5 |
  2416693.8 |
  2447038.1 |
  2477382.4 |
  2507726.7 |
  2538071.0 |
  2568415.3 |
  2598759.6 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2021410.4-2031260.6 ns)
  2021410.4 |########################################
  2021902.9 |
  2022395.4 |
  2022887.9 |
  2023380.4 |
  2023873.0 |
  2024365.5 |
  2024858.0 |
  2025350.5 |
  2025843.0 |
  2026335.5 |
  2026828.0 |####################
  2027320.5 |
  2027813.1 |####################
  2028305.6 |
  2028798.1 |####################
  2029290.6 |
  2029783.1 |
  2030275.6 |
  2030768.1 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2010165.4-2200473.5 ns)
  2010165.4 |########################################
  2019680.8 |
  2029196.2 |
  2038711.6 |
  2048227.0 |
  2057742.4 |
  2067257.8 |
  2076773.3 |
  2086288.7 |
  2095804.1 |
  2105319.5 |
  2114834.9 |
  2124350.3 |
  2133865.7 |
  2143381.1 |
  2152896.5 |
  2162411.9 |##########
  2171927.3 |
  2181442.7 |
  2190958.1 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2032514.6-2084744.0 ns)
  2032514.6 |########################################
  2035126.1 |#############
  2037737.5 |#############
  2040349.0 |
  2042960.5 |
  2045571.9 |
  2048183.4 |
  2050794.9 |
  2053406.3 |
  2056017.8 |
  2058629.3 |
  2061240.7 |
  2063852.2 |
  2066463.7 |
  2069075.1 |
  2071686.6 |
  2074298.1 |
  2076909.5 |
  2079521.0 |
  2082132.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=303.0% of algo (FFI overhead may distort results)
