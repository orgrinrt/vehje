# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (25.59 us) is smaller than the fastest variant's own run-to-run std-dev (61.56 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.01 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader abi_sink_tight_null_sink vs stability leader abi_sink_tight_batched_sink (+1% speed for 3.5x steadier)

abi_sink_tight_null_sink is fastest (2.01 ms, CV 3.1%); abi_sink_tight_batched_sink gives up 0.6% median for 3.5x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.3% of the fastest

All 4 variants sit between 2.01 ms and 2.04 ms - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2014683.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 2014683.0 ns, slowest 2040273.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2036355ns | 2028994ns | 2021317ns | 2028724ns | 2055322ns | -0.38% |
| abi_sink_tight_batched_sink_decode | 2086799ns | 2030546ns | 2024829ns | 2029082ns | 2204359ns | +2.09% |
| abi_sink_tight_null_sink | 2044029ns | 2017614ns | 2011399ns | 2017405ns | 2100281ns | base |
| abi_sink_tight_per_record_sink | 2098809ns | 2043185ns | 2032490ns | 2042188ns | 2216902ns | +2.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2033311ns | 2018478ns | 2052176ns | -0.38% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2083569ns | 2021895ns | 2200621ns | +2.09% | 0.000 |
| abi_sink_tight_null_sink | 2040988ns | 2008693ns | 2096849ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2095781ns | 2029840ns | 2213578ns | +2.68% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 57676.9 | 2030939.8 | 2033310.6 | n/a |
| abi_sink_tight_batched_sink_decode | 63474.3 | 2084885.6 | 2083568.9 | n/a |
| abi_sink_tight_null_sink | 58789.0 | 2023311.1 | 2040988.0 | n/a |
| abi_sink_tight_per_record_sink | 57424.5 | 2103631.8 | 2095781.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.1% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.1% |
| abi_sink_tight_null_sink | 0.000 | 99.7% |
| abi_sink_tight_per_record_sink | 0.000 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2036355ns | 2036355ns | -0.38% |
| abi_sink_tight_batched_sink_decode | 2086799ns | 2086799ns | +2.09% |
| abi_sink_tight_null_sink | 2044029ns | 2044029ns | base |
| abi_sink_tight_per_record_sink | 2098809ns | 2098809ns | +2.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2014683ns | base | --- | [2011432, 2096849] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2026004ns | no significant difference | [-51722, +17058]ns | [2021752, 2052176] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_tight_batched_sink_decode | 2027485ns | +13432.1ns (+0.7%) | [+10051, +104260]ns | [2022600, 2200621] | YES | 0.0469 | 0.0313 | 0 |
| abi_sink_tight_per_record_sink | 2040273ns | +28353.4ns (+1.4%) | [+18809, +117216]ns | [2033492, 2213578] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2015147ns | +0.9% | +0.6% | +1.2% |
| 2 | 2014172ns | +0.6% | +0.9% | +2.1% |
| 3 | 2014572ns | +0.6% | +0.7% | +0.8% |
| 4 | 2014794ns | +0.2% | +0.4% | +1.1% |
| 5 | 2008693ns | +0.8% | +0.7% | +1.6% |
| 6 | 2178551ns | -4.9% | +8.7% | +8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.014 | ok |
| abi_sink_tight_batched_sink_decode | -0.049 | ok |
| abi_sink_tight_null_sink | -0.066 | ok |
| abi_sink_tight_per_record_sink | -0.036 | ok |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 1/6, lost 5/6
- **abi_sink_tight_batched_sink_decode**: won 0/6, lost 6/6
- **abi_sink_tight_per_record_sink**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6152806.3ns | 2033310.6ns | 302.6% | HIGH |
| abi_sink_tight_batched_sink_decode | 6351901.6ns | 2083568.9ns | 304.9% | HIGH |
| abi_sink_tight_null_sink | 6186958.5ns | 2040988.0ns | 303.1% | HIGH |
| abi_sink_tight_per_record_sink | 6264463.1ns | 2095781.0ns | 298.9% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2018477.9-2052175.8 ns)
  2018477.9 |####################
  2020162.8 |
  2021847.7 |
  2023532.6 |####################
  2025217.5 |########################################
  2026902.4 |
  2028587.3 |
  2030272.2 |
  2031957.1 |####################
  2033642.0 |
  2035326.9 |
  2037011.7 |
  2038696.6 |
  2040381.5 |
  2042066.4 |
  2043751.3 |
  2045436.2 |
  2047121.1 |
  2048806.0 |
  2050490.9 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2021895.4-2200620.9 ns)
  2021895.4 |########################################
  2030831.7 |##########
  2039767.9 |
  2048704.2 |
  2057640.5 |
  2066576.8 |
  2075513.0 |
  2084449.3 |
  2093385.6 |
  2102321.9 |
  2111258.1 |
  2120194.4 |
  2129130.7 |
  2138066.9 |
  2147003.2 |
  2155939.5 |
  2164875.8 |
  2173812.0 |
  2182748.3 |
  2191684.6 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2008692.9-2096848.8 ns)
  2008692.9 |##########
  2013100.7 |########################################
  2017508.5 |
  2021916.3 |
  2026324.1 |
  2030731.9 |
  2035139.7 |
  2039547.4 |
  2043955.2 |
  2048363.0 |
  2052770.8 |
  2057178.6 |
  2061586.4 |
  2065994.2 |
  2070402.0 |
  2074809.8 |
  2079217.6 |
  2083625.4 |
  2088033.2 |
  2092441.0 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2029840.4-2213577.7 ns)
  2029840.4 |########################################
  2039027.3 |########################################
  2048214.1 |####################
  2057401.0 |
  2066587.9 |
  2075774.7 |
  2084961.6 |
  2094148.5 |
  2103335.3 |
  2112522.2 |
  2121709.0 |
  2130895.9 |
  2140082.8 |
  2149269.6 |
  2158456.5 |
  2167643.4 |
  2176830.2 |
  2186017.1 |
  2195204.0 |
  2204390.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=302.7% of algo (FFI overhead may distort results)
