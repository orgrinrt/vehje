# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (16.97 us) is smaller than the fastest variant's own run-to-run std-dev (43.85 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.03 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.8% of the fastest

All 4 variants sit between 2.03 ms and 2.04 ms - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2026084.0 ns median
- Spread: 1.01x (fastest 2026084.0 ns, slowest 2043053.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2066986ns | 2036431ns | 2033385ns | 2035633ns | 2130815ns | +0.85% |
| abi_sink_tight_batched_sink_decode | 2049727ns | 2038851ns | 2036394ns | 2038387ns | 2073404ns | +0.00% |
| abi_sink_tight_null_sink | 2049636ns | 2029081ns | 2022178ns | 2027346ns | 2096800ns | base |
| abi_sink_tight_per_record_sink | 2055963ns | 2046205ns | 2037403ns | 2043436ns | 2084032ns | +0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2063797ns | 2030579ns | 2127017ns | +0.85% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2046662ns | 2033318ns | 2069997ns | +0.02% | 0.000 |
| abi_sink_tight_null_sink | 2046331ns | 2019433ns | 2092774ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2052758ns | 2034469ns | 2080502ns | +0.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 60200.7 | 2063231.9 | 2063797.4 | 0 |
| abi_sink_tight_batched_sink_decode | 59472.4 | 2046825.1 | 2046661.8 | 6 |
| abi_sink_tight_null_sink | 61415.3 | 2045855.6 | 2046331.1 | n/a |
| abi_sink_tight_per_record_sink | 60667.5 | 2051025.2 | 2052758.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.3% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_tight_null_sink | 0.000 | 99.7% |
| abi_sink_tight_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2066986ns | 2066986ns | +0.85% |
| abi_sink_tight_batched_sink_decode | 2049727ns | 2049727ns | +0.00% |
| abi_sink_tight_null_sink | 2049636ns | 2049636ns | base |
| abi_sink_tight_per_record_sink | 2055963ns | 2055963ns | +0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2026084ns | base | --- | [2020136, 2092774] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2033442ns | no significant difference | [-2427, +44029]ns | [2030933, 2127017] | no | 0.6563 | 0.2188 | 0 |
| abi_sink_tight_batched_sink_decode | 2036031ns | no significant difference | [-24315, +15895]ns | [2033957, 2069997] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_tight_per_record_sink | 2043054ns | no significant difference | [-17631, +27131]ns | [2034719, 2080502] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2142846ns | +3.5% | -1.8% | -1.5% |
| 2 | 2042701ns | -0.5% | -0.5% | -0.1% |
| 3 | 2029038ns | +0.3% | +0.4% | +0.3% |
| 4 | 2020839ns | +0.5% | +0.7% | +0.7% |
| 5 | 2023130ns | +0.6% | +0.6% | +1.2% |
| 6 | 2019433ns | +0.6% | +0.8% | +1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.040 | ok |
| abi_sink_tight_batched_sink_decode | -0.065 | ok |
| abi_sink_tight_null_sink | 0.119 | ok |
| abi_sink_tight_per_record_sink | -0.016 | ok |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 1/6, lost 5/6
- **abi_sink_tight_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_tight_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6256327.0ns | 2063797.4ns | 303.1% | HIGH |
| abi_sink_tight_batched_sink_decode | 6196840.1ns | 2046661.8ns | 302.8% | HIGH |
| abi_sink_tight_null_sink | 6213760.1ns | 2046331.1ns | 303.7% | HIGH |
| abi_sink_tight_per_record_sink | 6219518.0ns | 2052758.1ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2030578.7-2127017.3 ns)
  2030578.7 |########################################
  2035400.6 |##########
  2040222.6 |
  2045044.5 |
  2049866.4 |
  2054688.3 |
  2059510.3 |
  2064332.2 |
  2069154.1 |
  2073976.1 |
  2078798.0 |
  2083619.9 |
  2088441.9 |
  2093263.8 |
  2098085.7 |
  2102907.6 |
  2107729.6 |
  2112551.5 |
  2117373.4 |
  2122195.4 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2033317.9-2069996.7 ns)
  2033317.9 |##########################
  2035151.8 |########################################
  2036985.8 |
  2038819.7 |
  2040653.7 |
  2042487.6 |
  2044321.5 |
  2046155.5 |
  2047989.4 |
  2049823.4 |
  2051657.3 |
  2053491.2 |
  2055325.2 |
  2057159.1 |
  2058993.1 |
  2060827.0 |
  2062660.9 |
  2064494.9 |
  2066328.8 |
  2068162.8 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2019432.9-2092773.5 ns)
  2019432.9 |########################################
  2023099.9 |####################
  2026767.0 |####################
  2030434.0 |
  2034101.0 |
  2037768.0 |
  2041435.1 |####################
  2045102.1 |
  2048769.1 |
  2052436.2 |
  2056103.2 |
  2059770.2 |
  2063437.3 |
  2067104.3 |
  2070771.3 |
  2074438.4 |
  2078105.4 |
  2081772.4 |
  2085439.4 |
  2089106.5 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2034469.2-2080501.6 ns)
  2034469.2 |########################################
  2036770.8 |
  2039072.4 |####################
  2041374.1 |
  2043675.7 |
  2045977.3 |####################
  2048278.9 |####################
  2050580.6 |
  2052882.2 |
  2055183.8 |
  2057485.4 |
  2059787.0 |
  2062088.7 |
  2064390.3 |
  2066691.9 |
  2068993.5 |
  2071295.2 |
  2073596.8 |
  2075898.4 |
  2078200.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=303.0% of algo (FFI overhead may distort results)
