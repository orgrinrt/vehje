# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (11.35 us) is smaller than the fastest variant's own run-to-run std-dev (14.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.02 ms and 2.03 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_tight_batched_sink** at 2022367.9 ns median (-0.0% vs baseline)
- Spread: 1.01x (fastest 2022367.9 ns, slowest 2033719.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2027966ns | 2025506ns | 2012805ns | 2022967ns | 2043045ns | -0.14% |
| abi_sink_tight_batched_sink_decode | 2036167ns | 2031143ns | 2025458ns | 2030445ns | 2050103ns | +0.26% |
| abi_sink_tight_null_sink | 2030790ns | 2026548ns | 2017904ns | 2025186ns | 2045641ns | base |
| abi_sink_tight_per_record_sink | 2038993ns | 2036624ns | 2031380ns | 2035324ns | 2048304ns | +0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2024841ns | 2010111ns | 2039669ns | -0.13% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2032939ns | 2022281ns | 2046718ns | +0.27% | 0.000 |
| abi_sink_tight_null_sink | 2027562ns | 2014747ns | 2042417ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2035936ns | 2028588ns | 2044920ns | +0.41% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 58770.3 | 2027925.3 | 2024840.8 | n/a |
| abi_sink_tight_batched_sink_decode | 65291.7 | 2043005.6 | 2032938.9 | n/a |
| abi_sink_tight_null_sink | 60442.6 | 2031254.9 | 2027562.3 | n/a |
| abi_sink_tight_per_record_sink | 56655.2 | 2036081.2 | 2035936.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_batched_sink; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.4% |
| abi_sink_tight_batched_sink_decode | 0.000 | 99.1% |
| abi_sink_tight_null_sink | 0.000 | 99.3% |
| abi_sink_tight_per_record_sink | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2027966ns | 2027966ns | -0.14% |
| abi_sink_tight_batched_sink_decode | 2036167ns | 2036167ns | +0.26% |
| abi_sink_tight_null_sink | 2030790ns | 2030790ns | base |
| abi_sink_tight_per_record_sink | 2038993ns | 2038993ns | +0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2023349ns | base | --- | [2016921, 2042417] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2022368ns | no significant difference | [-25844, +18196]ns | [2012485, 2039669] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_tight_batched_sink_decode | 2028023ns | no significant difference | [-17245, +27921]ns | [2024075, 2046718] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_tight_per_record_sink | 2033719ns | no significant difference | [-10516, +23447]ns | [2029170, 2044920] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2019094ns | +0.3% | +0.3% | +1.5% |
| 2 | 2048321ns | -1.9% | -1.0% | -0.7% |
| 3 | 2036513ns | -0.7% | -0.7% | -0.3% |
| 4 | 2023851ns | +1.5% | +0.2% | +0.8% |
| 5 | 2022847ns | -0.1% | +1.3% | +0.5% |
| 6 | 2014747ns | +0.0% | +1.5% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.070 | ok |
| abi_sink_tight_batched_sink_decode | 0.399 | moderate+ |
| abi_sink_tight_null_sink | 0.070 | ok |
| abi_sink_tight_per_record_sink | -0.093 | ok |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 2/6, lost 2/6
- **abi_sink_tight_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_tight_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6142522.1ns | 2024840.8ns | 303.4% | HIGH |
| abi_sink_tight_batched_sink_decode | 6182588.2ns | 2032938.9ns | 304.1% | HIGH |
| abi_sink_tight_null_sink | 6159798.5ns | 2027562.3ns | 303.8% | HIGH |
| abi_sink_tight_per_record_sink | 6166748.1ns | 2035936.2ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2010110.8-2039669.1 ns)
  2010110.8 |########################################
  2011588.7 |
  2013066.6 |
  2014544.6 |########################################
  2016022.5 |
  2017500.4 |
  2018978.3 |
  2020456.2 |########################################
  2021934.1 |########################################
  2023412.1 |########################################
  2024890.0 |
  2026367.9 |
  2027845.8 |
  2029323.7 |
  2030801.6 |
  2032279.6 |
  2033757.5 |
  2035235.4 |
  2036713.3 |
  2038191.2 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2022280.8-2046718.4 ns)
  2022280.8 |####################
  2023502.7 |
  2024724.6 |####################
  2025946.4 |
  2027168.3 |########################################
  2028390.2 |
  2029612.1 |
  2030833.9 |
  2032055.8 |
  2033277.7 |
  2034499.6 |
  2035721.5 |
  2036943.3 |
  2038165.2 |
  2039387.1 |
  2040609.0 |
  2041830.8 |
  2043052.7 |####################
  2044274.6 |
  2045496.5 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2014747.1-2042417.0 ns)
  2014747.1 |########################################
  2016130.6 |
  2017514.1 |
  2018897.6 |########################################
  2020281.1 |
  2021664.6 |########################################
  2023048.1 |########################################
  2024431.6 |
  2025815.1 |
  2027198.6 |
  2028582.1 |
  2029965.6 |
  2031349.1 |
  2032732.6 |
  2034116.1 |
  2035499.6 |########################################
  2036883.1 |
  2038266.6 |
  2039650.1 |
  2041033.6 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2028587.9-2044919.6 ns)
  2028587.9 |########################################
  2029404.5 |########################################
  2030221.1 |
  2031037.7 |
  2031854.2 |
  2032670.8 |########################################
  2033487.4 |########################################
  2034304.0 |
  2035120.6 |
  2035937.2 |
  2036753.8 |
  2037570.3 |
  2038386.9 |
  2039203.5 |########################################
  2040020.1 |
  2040836.7 |
  2041653.3 |
  2042469.8 |
  2043286.4 |
  2044103.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=302.7% of algo (FFI overhead may distort results)
