# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.02 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.1% of the fastest

All 4 variants sit between 2.02 ms and 2.04 ms - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2017535.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 2017535.2 ns, slowest 2039786.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2093130ns | 2030397ns | 2024538ns | 2028686ns | 2224091ns | +3.51% |
| abi_sink_tight_batched_sink_decode | 2053993ns | 2027684ns | 2020818ns | 2027168ns | 2110818ns | +1.57% |
| abi_sink_tight_null_sink | 2022203ns | 2020526ns | 2015422ns | 2019193ns | 2030110ns | base |
| abi_sink_tight_per_record_sink | 2060713ns | 2042977ns | 2037243ns | 2042021ns | 2100486ns | +1.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2089684ns | 2021512ns | 2219865ns | +3.50% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2050606ns | 2017882ns | 2106814ns | +1.56% | 0.000 |
| abi_sink_tight_null_sink | 2019071ns | 2012141ns | 2026773ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2057340ns | 2034187ns | 2096661ns | +1.90% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 67435.9 | 2088852.2 | 2089683.9 | n/a |
| abi_sink_tight_batched_sink_decode | 65880.6 | 2048701.4 | 2050606.1 | n/a |
| abi_sink_tight_null_sink | 59906.8 | 2019829.2 | 2019071.0 | n/a |
| abi_sink_tight_per_record_sink | 65536.2 | 2054650.9 | 2057340.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.3% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.4% |
| abi_sink_tight_null_sink | 0.000 | 99.7% |
| abi_sink_tight_per_record_sink | 0.000 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2093130ns | 2093130ns | +3.51% |
| abi_sink_tight_batched_sink_decode | 2053993ns | 2053993ns | +1.57% |
| abi_sink_tight_null_sink | 2022203ns | 2022203ns | base |
| abi_sink_tight_per_record_sink | 2060713ns | 2060713ns | +1.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2017535ns | base | --- | [2012905, 2026773] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2027319ns | +5761.4ns (+0.3%) | [+1058, +205019]ns | [2021868, 2219865] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_sink_tight_batched_sink_decode | 2024507ns | no significant difference | [-1328, +91201]ns | [2020497, 2106814] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_tight_per_record_sink | 2039786ns | +22235.6ns (+1.1%) | [+11315, +81256]ns | [2035573, 2096661] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2016402ns | +0.6% | +0.3% | +0.9% |
| 2 | 2028335ns | -0.1% | -0.2% | +0.4% |
| 3 | 2025211ns | +0.2% | +0.0% | +0.7% |
| 4 | 2012141ns | +19.8% | +8.7% | +6.2% |
| 5 | 2013670ns | +0.4% | +0.2% | +1.3% |
| 6 | 2018668ns | +0.2% | +0.3% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.235 | moderate- |
| abi_sink_tight_batched_sink_decode | -0.251 | moderate- |
| abi_sink_tight_null_sink | 0.140 | ok |
| abi_sink_tight_per_record_sink | -0.247 | moderate- |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 0/6, lost 5/6
- **abi_sink_tight_batched_sink_decode**: won 1/6, lost 4/6
- **abi_sink_tight_per_record_sink**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6347753.0ns | 2089683.9ns | 303.8% | HIGH |
| abi_sink_tight_batched_sink_decode | 6226353.8ns | 2050606.1ns | 303.6% | HIGH |
| abi_sink_tight_null_sink | 6123033.6ns | 2019071.0ns | 303.3% | HIGH |
| abi_sink_tight_per_record_sink | 6238323.1ns | 2057340.1ns | 303.2% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2021512.1-2219865.0 ns)
  2021512.1 |########################################
  2031429.7 |
  2041347.4 |
  2051265.0 |
  2061182.7 |
  2071100.3 |
  2081018.0 |
  2090935.6 |
  2100853.2 |
  2110770.9 |
  2120688.5 |
  2130606.2 |
  2140523.8 |
  2150441.5 |
  2160359.1 |
  2170276.7 |
  2180194.4 |
  2190112.0 |
  2200029.7 |
  2209947.3 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2017882.1-2106814.4 ns)
  2017882.1 |##########
  2022328.7 |########################################
  2026775.3 |
  2031221.9 |
  2035668.6 |
  2040115.2 |
  2044561.8 |
  2049008.4 |
  2053455.0 |
  2057901.6 |
  2062348.2 |
  2066794.9 |
  2071241.5 |
  2075688.1 |
  2080134.7 |
  2084581.3 |
  2089027.9 |
  2093474.6 |
  2097921.2 |
  2102367.8 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2012140.8-2026772.7 ns)
  2012140.8 |########################################
  2012872.4 |
  2013604.0 |########################################
  2014335.6 |
  2015067.2 |
  2015798.8 |########################################
  2016530.4 |
  2017262.0 |
  2017993.6 |########################################
  2018725.2 |
  2019456.8 |
  2020188.3 |
  2020919.9 |
  2021651.5 |
  2022383.1 |
  2023114.7 |
  2023846.3 |
  2024577.9 |########################################
  2025309.5 |
  2026041.1 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2034187.1-2096661.1 ns)
  2034187.1 |########################################
  2037310.8 |########################################
  2040434.5 |
  2043558.2 |
  2046681.9 |
  2049805.6 |
  2052929.3 |
  2056053.0 |####################
  2059176.7 |
  2062300.4 |
  2065424.1 |
  2068547.8 |
  2071671.5 |
  2074795.2 |
  2077918.9 |
  2081042.6 |
  2084166.3 |
  2087290.0 |
  2090413.7 |
  2093537.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=303.6% of algo (FFI overhead may distort results)
