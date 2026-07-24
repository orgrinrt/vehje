# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.02 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.9% of the fastest

All 4 variants sit between 2.02 ms and 2.04 ms - a 0.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2017245.9 ns median
- 3 variants significantly slower than baseline
- Spread: 1.01x (fastest 2017245.9 ns, slowest 2035894.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2037921ns | 2026316ns | 2023858ns | 2025861ns | 2063042ns | +0.92% |
| abi_sink_tight_batched_sink_decode | 2074329ns | 2028060ns | 2026402ns | 2027794ns | 2168096ns | +2.73% |
| abi_sink_tight_null_sink | 2019260ns | 2020246ns | 2013185ns | 2019860ns | 2021399ns | base |
| abi_sink_tight_per_record_sink | 2043349ns | 2038734ns | 2035122ns | 2038467ns | 2054786ns | +1.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2034918ns | 2020987ns | 2059921ns | +0.92% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2071178ns | 2023532ns | 2164479ns | +2.72% | 0.000 |
| abi_sink_tight_null_sink | 2016316ns | 2010471ns | 2018391ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2040445ns | 2032374ns | 2051683ns | +1.20% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 54163.9 | 2033651.5 | 2034918.3 | n/a |
| abi_sink_tight_batched_sink_decode | 60540.1 | 2096450.0 | 2071177.9 | n/a |
| abi_sink_tight_null_sink | 53075.8 | 2016128.5 | 2016316.0 | n/a |
| abi_sink_tight_per_record_sink | 53081.5 | 2039964.4 | 2040444.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.4% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_tight_null_sink | 0.000 | 99.7% |
| abi_sink_tight_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2037921ns | 2037921ns | +0.92% |
| abi_sink_tight_batched_sink_decode | 2074329ns | 2074329ns | +2.73% |
| abi_sink_tight_null_sink | 2019260ns | 2019260ns | base |
| abi_sink_tight_per_record_sink | 2043349ns | 2043349ns | +1.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2017246ns | base | --- | [2013311, 2018391] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2023245ns | +8567.9ns (+0.4%) | [+3197, +44042]ns | [2021589, 2059921] | YES | 0.0313 | 0.0313 | 0 |
| abi_sink_tight_batched_sink_decode | 2025062ns | +10857.9ns (+0.5%) | [+5674, +148054]ns | [2023993, 2164479] | YES | 0.0313 | 0.0313 | 0 |
| abi_sink_tight_per_record_sink | 2035894ns | +20496.7ns (+1.0%) | [+18216, +33673]ns | [2033756, 2051683] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2010471ns | +0.6% | +0.7% | +1.1% |
| 2 | 2018229ns | +0.2% | +0.3% | +2.0% |
| 3 | 2018553ns | +0.1% | +0.2% | +0.9% |
| 4 | 2017792ns | +0.3% | +0.4% | +1.3% |
| 5 | 2016151ns | +3.7% | +13.6% | +0.9% |
| 6 | 2016700ns | +0.6% | +1.1% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.140 | ok |
| abi_sink_tight_batched_sink_decode | -0.185 | ok |
| abi_sink_tight_null_sink | -0.086 | ok |
| abi_sink_tight_per_record_sink | -0.489 | moderate- |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 0/6, lost 6/6
- **abi_sink_tight_batched_sink_decode**: won 0/6, lost 6/6
- **abi_sink_tight_per_record_sink**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6155186.4ns | 2034918.3ns | 302.5% | HIGH |
| abi_sink_tight_batched_sink_decode | 6483455.3ns | 2071177.9ns | 313.0% | HIGH |
| abi_sink_tight_null_sink | 6105077.5ns | 2016316.0ns | 302.8% | HIGH |
| abi_sink_tight_per_record_sink | 6176922.3ns | 2040444.6ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2020987.1-2059921.2 ns)
  2020987.1 |########################################
  2022933.8 |########################################
  2024880.5 |
  2026827.2 |####################
  2028773.9 |
  2030720.6 |
  2032667.3 |
  2034614.1 |
  2036560.8 |
  2038507.5 |
  2040454.2 |
  2042400.9 |
  2044347.6 |
  2046294.3 |
  2048241.0 |
  2050187.7 |
  2052134.4 |
  2054081.1 |
  2056027.8 |
  2057974.5 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2023532.5-2164479.0 ns)
  2023532.5 |########################################
  2030579.8 |
  2037627.1 |##########
  2044674.5 |
  2051721.8 |
  2058769.1 |
  2065816.4 |
  2072863.8 |
  2079911.1 |
  2086958.4 |
  2094005.8 |
  2101053.1 |
  2108100.4 |
  2115147.7 |
  2122195.0 |
  2129242.4 |
  2136289.7 |
  2143337.0 |
  2150384.4 |
  2157431.7 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2010471.2-2018391.2 ns)
  2010471.2 |########################################
  2010867.2 |
  2011263.2 |
  2011659.2 |
  2012055.2 |
  2012451.2 |
  2012847.2 |
  2013243.2 |
  2013639.2 |
  2014035.2 |
  2014431.2 |
  2014827.2 |
  2015223.2 |
  2015619.2 |
  2016015.2 |########################################
  2016411.2 |########################################
  2016807.2 |
  2017203.2 |
  2017599.2 |########################################
  2017995.2 |########################################
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2032374.2-2051683.3 ns)
  2032374.2 |####################
  2033339.7 |
  2034305.1 |####################
  2035270.6 |########################################
  2036236.0 |
  2037201.5 |
  2038166.9 |
  2039132.4 |
  2040097.8 |
  2041063.3 |
  2042028.8 |
  2042994.2 |
  2043959.7 |####################
  2044925.1 |
  2045890.6 |
  2046856.0 |
  2047821.5 |
  2048786.9 |
  2049752.4 |
  2050717.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=302.6% of algo (FFI overhead may distort results)
