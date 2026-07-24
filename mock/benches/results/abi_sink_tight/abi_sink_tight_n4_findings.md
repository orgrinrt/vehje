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

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2020329.4 ns median
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 2020329.4 ns, slowest 2037753.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2047390ns | 2030840ns | 2025185ns | 2029725ns | 2084989ns | +1.08% |
| abi_sink_tight_batched_sink_decode | 2103930ns | 2034471ns | 2027483ns | 2032289ns | 2249616ns | +3.87% |
| abi_sink_tight_null_sink | 2025476ns | 2023491ns | 2016802ns | 2021783ns | 2035352ns | base |
| abi_sink_tight_per_record_sink | 2042216ns | 2040532ns | 2036469ns | 2039337ns | 2049408ns | +0.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2044288ns | 2022340ns | 2081566ns | +1.08% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2100639ns | 2024645ns | 2245695ns | +3.87% | 0.000 |
| abi_sink_tight_null_sink | 2022365ns | 2014016ns | 2032048ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2039261ns | 2033582ns | 2046141ns | +0.84% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 59380.7 | 2039913.2 | 2044287.8 | n/a |
| abi_sink_tight_batched_sink_decode | 64605.0 | 2104931.6 | 2100638.6 | n/a |
| abi_sink_tight_null_sink | 58178.5 | 2022396.2 | 2022365.5 | n/a |
| abi_sink_tight_per_record_sink | 54595.0 | 2039505.5 | 2039261.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.3% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.1% |
| abi_sink_tight_null_sink | 0.000 | 99.7% |
| abi_sink_tight_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2047390ns | 2047390ns | +1.08% |
| abi_sink_tight_batched_sink_decode | 2103930ns | 2103930ns | +3.87% |
| abi_sink_tight_null_sink | 2025476ns | 2025476ns | base |
| abi_sink_tight_per_record_sink | 2042216ns | 2042216ns | +0.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2020329ns | base | --- | [2014720, 2032048] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2027759ns | no significant difference | [-2639, +59587]ns | [2023538, 2081566] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_tight_batched_sink_decode | 2031334ns | +11077.8ns (+0.5%) | [+1983, +221758]ns | [2024887, 2245695] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_sink_tight_per_record_sink | 2037753ns | +17762.9ns (+0.9%) | [+12391, +20533]ns | [2033889, 2046141] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2015423ns | +0.3% | +0.5% | +0.9% |
| 2 | 2021810ns | +0.2% | +0.7% | +0.9% |
| 3 | 2018849ns | +1.3% | +0.3% | +0.7% |
| 4 | 2025109ns | +4.5% | +21.2% | +0.9% |
| 5 | 2038986ns | -0.5% | -0.1% | +0.5% |
| 6 | 2014016ns | +0.5% | +0.6% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.041 | ok |
| abi_sink_tight_batched_sink_decode | -0.231 | moderate- |
| abi_sink_tight_null_sink | -0.234 | moderate- |
| abi_sink_tight_per_record_sink | -0.067 | ok |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 1/6, lost 5/6
- **abi_sink_tight_batched_sink_decode**: won 1/6, lost 5/6
- **abi_sink_tight_per_record_sink**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6254359.7ns | 2044287.8ns | 305.9% | HIGH |
| abi_sink_tight_batched_sink_decode | 6430890.7ns | 2100638.6ns | 306.1% | HIGH |
| abi_sink_tight_null_sink | 6130506.4ns | 2022365.5ns | 303.1% | HIGH |
| abi_sink_tight_per_record_sink | 6180258.9ns | 2039261.2ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2022340.4-2081566.2 ns)
  2022340.4 |########################################
  2025301.7 |####################
  2028263.0 |####################
  2031224.3 |
  2034185.6 |
  2037146.9 |
  2040108.2 |
  2043069.4 |
  2046030.7 |####################
  2048992.0 |
  2051953.3 |
  2054914.6 |
  2057875.9 |
  2060837.2 |
  2063798.5 |
  2066759.8 |
  2069721.1 |
  2072682.4 |
  2075643.7 |
  2078605.0 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2024645.0-2245694.8 ns)
  2024645.0 |########################################
  2035697.5 |##########################
  2046750.0 |
  2057802.5 |
  2068855.0 |
  2079907.4 |
  2090959.9 |
  2102012.4 |
  2113064.9 |
  2124117.4 |
  2135169.9 |
  2146222.4 |
  2157274.9 |
  2168327.4 |
  2179379.9 |
  2190432.3 |
  2201484.8 |
  2212537.3 |
  2223589.8 |
  2234642.3 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2014015.8-2032047.5 ns)
  2014015.8 |########################################
  2014917.4 |########################################
  2015819.0 |
  2016720.6 |
  2017622.1 |
  2018523.7 |########################################
  2019425.3 |
  2020326.9 |
  2021228.5 |########################################
  2022130.1 |
  2023031.6 |
  2023933.2 |
  2024834.8 |########################################
  2025736.4 |
  2026638.0 |
  2027539.6 |
  2028441.2 |
  2029342.7 |
  2030244.3 |
  2031145.9 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2033581.7-2046141.2 ns)
  2033581.7 |########################################
  2034209.7 |
  2034837.7 |
  2035465.6 |
  2036093.6 |####################
  2036721.6 |
  2037349.6 |
  2037977.5 |
  2038605.5 |####################
  2039233.5 |
  2039861.5 |
  2040489.5 |
  2041117.4 |
  2041745.4 |
  2042373.4 |
  2043001.4 |####################
  2043629.3 |
  2044257.3 |
  2044885.3 |
  2045513.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=302.9% of algo (FFI overhead may distort results)
