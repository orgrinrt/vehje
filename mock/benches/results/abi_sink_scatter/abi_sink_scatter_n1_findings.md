# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (11.46 us) is smaller than the fastest variant's own run-to-run std-dev (13.96 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_scatter_null_sink)

The baseline abi_sink_scatter_null_sink is the fastest (2.15 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.5% of the fastest

All 4 variants sit between 2.15 ms and 2.16 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_scatter_null_sink) is the fastest** at 2150469.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 2150469.0 ns, slowest 2161929.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2158453ns | 2156940ns | 2149580ns | 2155082ns | 2167948ns | +0.10% |
| abi_sink_scatter_batched_sink_decode | 2165061ns | 2164994ns | 2150694ns | 2162663ns | 2175840ns | +0.41% |
| abi_sink_scatter_null_sink | 2156190ns | 2153270ns | 2142504ns | 2149796ns | 2172623ns | base |
| abi_sink_scatter_per_record_sink | 2163707ns | 2158249ns | 2150252ns | 2157046ns | 2180427ns | +0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2155611ns | 2146813ns | 2164842ns | +0.11% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2162046ns | 2148191ns | 2172484ns | +0.41% | 0.000 |
| abi_sink_scatter_null_sink | 2153309ns | 2140000ns | 2169300ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2160725ns | 2147579ns | 2177118ns | +0.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 49503.4 | 2153675.6 | 2155611.1 | 0 |
| abi_sink_scatter_batched_sink_decode | 54306.3 | 2164223.6 | 2162045.8 | n/a |
| abi_sink_scatter_null_sink | 49954.7 | 2152197.1 | 2153309.1 | n/a |
| abi_sink_scatter_per_record_sink | 51579.1 | 2160806.0 | 2160724.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_null_sink; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.3% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.0% |
| abi_sink_scatter_null_sink | 0.000 | 99.5% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2158453ns | 2158453ns | +0.10% |
| abi_sink_scatter_batched_sink_decode | 2165061ns | 2165061ns | +0.41% |
| abi_sink_scatter_null_sink | 2156190ns | 2156190ns | base |
| abi_sink_scatter_per_record_sink | 2163707ns | 2163707ns | +0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2150469ns | base | --- | [2140158, 2169300] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2154237ns | no significant difference | [-9722, +8769]ns | [2147755, 2164842] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_scatter_batched_sink_decode | 2161929ns | no significant difference | [-9553, +25121]ns | [2151724, 2172484] | no | 0.2188 | 0.2188 | 0 |
| abi_sink_scatter_per_record_sink | 2155322ns | +8229.2ns (+0.4%) | [+3265, +10753]ns | [2149734, 2177118] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2154725ns | +0.4% | +0.6% | +0.4% |
| 2 | 2157902ns | +0.4% | +0.3% | +0.0% |
| 3 | 2180698ns | -1.2% | -1.2% | +0.4% |
| 4 | 2146212ns | +0.3% | +0.6% | +0.3% |
| 5 | 2140000ns | +0.4% | +1.7% | +0.6% |
| 6 | 2140316ns | +0.3% | +0.4% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.481 | moderate+ |
| abi_sink_scatter_batched_sink_decode | -0.441 | moderate- |
| abi_sink_scatter_null_sink | 0.176 | ok |
| abi_sink_scatter_per_record_sink | -0.117 | ok |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 1/6, lost 5/6
- **abi_sink_scatter_batched_sink_decode**: won 1/6, lost 5/6
- **abi_sink_scatter_per_record_sink**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6509906.8ns | 2155611.1ns | 302.0% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6547880.0ns | 2162045.8ns | 302.9% | HIGH |
| abi_sink_scatter_null_sink | 6510633.3ns | 2153309.1ns | 302.4% | HIGH |
| abi_sink_scatter_per_record_sink | 6542326.7ns | 2160724.7ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2146812.9-2164841.9 ns)
  2146812.9 |########################################
  2147714.4 |
  2148615.8 |########################################
  2149517.2 |
  2150418.7 |
  2151320.1 |
  2152221.6 |
  2153123.0 |########################################
  2154024.5 |########################################
  2154925.9 |
  2155827.4 |
  2156728.9 |
  2157630.3 |
  2158531.8 |
  2159433.2 |
  2160334.6 |
  2161236.1 |
  2162137.5 |########################################
  2163039.0 |
  2163940.4 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2148190.8-2172483.5 ns)
  2148190.8 |########################################
  2149405.4 |
  2150620.1 |
  2151834.7 |
  2153049.3 |
  2154264.0 |########################################
  2155478.6 |
  2156693.3 |
  2157907.9 |
  2159122.5 |########################################
  2160337.2 |
  2161551.8 |
  2162766.4 |
  2163981.1 |########################################
  2165195.7 |
  2166410.4 |
  2167625.0 |########################################
  2168839.6 |
  2170054.3 |
  2171268.9 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2140000.0-2169300.2 ns)
  2140000.0 |########################################
  2141465.0 |
  2142930.0 |
  2144395.0 |
  2145860.0 |####################
  2147325.0 |
  2148790.1 |
  2150255.1 |
  2151720.1 |
  2153185.1 |
  2154650.1 |####################
  2156115.1 |
  2157580.1 |####################
  2159045.1 |
  2160510.1 |
  2161975.2 |
  2163440.2 |
  2164905.2 |
  2166370.2 |
  2167835.2 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2147579.2-2177117.7 ns)
  2147579.2 |########################################
  2149056.1 |
  2150533.1 |########################################
  2152010.0 |########################################
  2153486.9 |
  2154963.8 |
  2156440.8 |
  2157917.7 |########################################
  2159394.6 |
  2160871.5 |
  2162348.5 |
  2163825.4 |########################################
  2165302.3 |
  2166779.2 |
  2168256.2 |
  2169733.1 |
  2171210.0 |
  2172686.9 |
  2174163.9 |
  2175640.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=302.6% of algo (FFI overhead may distort results)
