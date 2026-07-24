# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.01 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.3% of the fastest

All 4 variants sit between 2.01 ms and 2.04 ms - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2013340.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 2013340.0 ns, slowest 2038743.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2085762ns | 2026661ns | 2024027ns | 2026071ns | 2206168ns | +3.06% |
| abi_sink_tight_batched_sink_decode | 2025984ns | 2026015ns | 2021793ns | 2024650ns | 2030079ns | +0.10% |
| abi_sink_tight_null_sink | 2023919ns | 2016025ns | 2013305ns | 2015454ns | 2041924ns | base |
| abi_sink_tight_per_record_sink | 2056768ns | 2041676ns | 2037060ns | 2040248ns | 2091402ns | +1.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2082725ns | 2021125ns | 2202641ns | +3.05% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2023094ns | 2018892ns | 2027112ns | +0.10% | 0.000 |
| abi_sink_tight_null_sink | 2021061ns | 2010542ns | 2038820ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2053642ns | 2034202ns | 2087940ns | +1.61% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 56645.6 | 2082924.5 | 2082724.8 | n/a |
| abi_sink_tight_batched_sink_decode | 49908.8 | 2023935.8 | 2023093.6 | 0 |
| abi_sink_tight_null_sink | 50524.4 | 2021325.1 | 2021060.6 | n/a |
| abi_sink_tight_per_record_sink | 56100.8 | 2068095.3 | 2053641.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.3% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.4% |
| abi_sink_tight_null_sink | 0.000 | 99.9% |
| abi_sink_tight_per_record_sink | 0.000 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2085762ns | 2085762ns | +3.06% |
| abi_sink_tight_batched_sink_decode | 2025984ns | 2025984ns | +0.10% |
| abi_sink_tight_null_sink | 2023919ns | 2023919ns | base |
| abi_sink_tight_per_record_sink | 2056768ns | 2056768ns | +1.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2013340ns | base | --- | [2011022, 2038820] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2023882ns | +10623.0ns (+0.5%) | [+7109, +167260]ns | [2021652, 2202641] | YES | 0.0469 | 0.0313 | 0 |
| abi_sink_tight_batched_sink_decode | 2023174ns | no significant difference | [-16341, +12466]ns | [2018994, 2027112] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_tight_per_record_sink | 2038744ns | +26984.4ns (+1.3%) | [+17674, +53085]ns | [2034242, 2087940] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2011501ns | +0.6% | +0.4% | +1.3% |
| 2 | 2012018ns | +0.5% | +0.7% | +1.4% |
| 3 | 2014662ns | +0.4% | +0.5% | +1.0% |
| 4 | 2010542ns | +0.7% | +0.6% | +1.6% |
| 5 | 2059168ns | +15.6% | -2.0% | +3.6% |
| 6 | 2018472ns | +0.3% | +0.5% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.223 | moderate- |
| abi_sink_tight_batched_sink_decode | -0.361 | moderate- |
| abi_sink_tight_null_sink | -0.161 | ok |
| abi_sink_tight_per_record_sink | -0.221 | moderate- |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 0/6, lost 6/6
- **abi_sink_tight_batched_sink_decode**: won 1/6, lost 5/6
- **abi_sink_tight_per_record_sink**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6314009.6ns | 2082724.8ns | 303.2% | HIGH |
| abi_sink_tight_batched_sink_decode | 6126672.0ns | 2023093.6ns | 302.8% | HIGH |
| abi_sink_tight_null_sink | 6248160.9ns | 2021060.6ns | 309.2% | HIGH |
| abi_sink_tight_per_record_sink | 6247476.0ns | 2053641.9ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2021125.0-2202641.0 ns)
  2021125.0 |########################################
  2030200.8 |
  2039276.6 |
  2048352.4 |
  2057428.2 |
  2066504.0 |
  2075579.8 |
  2084655.6 |
  2093731.4 |
  2102807.2 |
  2111883.0 |
  2120958.8 |
  2130034.6 |
  2139110.4 |
  2148186.2 |
  2157262.0 |
  2166337.8 |
  2175413.6 |
  2184489.4 |
  2193565.2 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2018892.1-2027112.5 ns)
  2018892.1 |########################################
  2019303.1 |
  2019714.1 |
  2020125.2 |
  2020536.2 |
  2020947.2 |
  2021358.2 |
  2021769.2 |####################
  2022180.3 |
  2022591.3 |
  2023002.3 |
  2023413.3 |
  2023824.3 |
  2024235.4 |####################
  2024646.4 |
  2025057.4 |
  2025468.4 |####################
  2025879.4 |
  2026290.5 |
  2026701.5 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2010542.1-2038820.2 ns)
  2010542.1 |########################################
  2011956.0 |####################
  2013369.9 |####################
  2014783.8 |
  2016197.7 |
  2017611.6 |####################
  2019025.5 |
  2020439.4 |
  2021853.3 |
  2023267.2 |
  2024681.1 |
  2026095.1 |
  2027509.0 |
  2028922.9 |
  2030336.8 |
  2031750.7 |
  2033164.6 |
  2034578.5 |
  2035992.4 |
  2037406.3 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2034202.1-2087940.0 ns)
  2034202.1 |########################################
  2036889.0 |####################
  2039575.9 |####################
  2042262.8 |####################
  2044949.7 |
  2047636.6 |
  2050323.5 |
  2053010.4 |
  2055697.3 |
  2058384.2 |
  2061071.1 |
  2063757.9 |
  2066444.8 |
  2069131.7 |
  2071818.6 |
  2074505.5 |
  2077192.4 |
  2079879.3 |
  2082566.2 |
  2085253.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=302.4% of algo (FFI overhead may distort results)
