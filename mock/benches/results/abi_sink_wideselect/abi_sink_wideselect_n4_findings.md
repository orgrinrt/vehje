# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3.52 us) is smaller than the fastest variant's own run-to-run std-dev (63.53 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader abi_sink_wideselect_batched_sink vs stability leader abi_sink_wideselect_per_record_sink (+0% speed for 31.4x steadier)

abi_sink_wideselect_batched_sink is fastest (2.07 ms, CV 3.1%); abi_sink_wideselect_per_record_sink gives up 0.2% median for 31.4x lower variance (CV 0.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.2% of the fastest

All 4 variants sit between 2.07 ms and 2.07 ms - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_batched_sink** at 2071362.3 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 2071362.3 ns, slowest 2074885.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2100599ns | 2074287ns | 2065355ns | 2072257ns | 2160733ns | +0.43% |
| abi_sink_wideselect_batched_sink_decode | 2077230ns | 2076594ns | 2067602ns | 2075150ns | 2085166ns | -0.68% |
| abi_sink_wideselect_null_sink | 2091526ns | 2075466ns | 2071668ns | 2074223ns | 2127409ns | base |
| abi_sink_wideselect_per_record_sink | 2077683ns | 2077959ns | 2074777ns | 2077012ns | 2080142ns | -0.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2097542ns | 2062589ns | 2157295ns | +0.44% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2074281ns | 2064782ns | 2082084ns | -0.68% | 0.000 |
| abi_sink_wideselect_null_sink | 2088426ns | 2068761ns | 2123920ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2074711ns | 2072064ns | 2077088ns | -0.66% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 59456.8 | 2087126.2 | 2097541.7 | 0 |
| abi_sink_wideselect_batched_sink_decode | 57777.3 | 2074889.4 | 2074281.0 | n/a |
| abi_sink_wideselect_null_sink | 61357.9 | 2094811.3 | 2088425.9 | n/a |
| abi_sink_wideselect_per_record_sink | 56452.2 | 2077202.1 | 2074711.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.6% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.5% |
| abi_sink_wideselect_null_sink | 0.000 | 99.5% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2100599ns | 2100599ns | +0.43% |
| abi_sink_wideselect_batched_sink_decode | 2077230ns | 2077230ns | -0.68% |
| abi_sink_wideselect_null_sink | 2091526ns | 2091526ns | base |
| abi_sink_wideselect_per_record_sink | 2077683ns | 2077683ns | -0.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2072571ns | base | --- | [2068786, 2123920] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2071362ns | no significant difference | [-13274, +40005]ns | [2063968, 2157295] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2073728ns | no significant difference | [-49823, +6231]ns | [2067031, 2082084] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_wideselect_per_record_sink | 2074885ns | no significant difference | [-50402, +7357]ns | [2072161, 2077088] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2164897ns | +3.4% | -4.4% | -4.2% |
| 2 | 2068811ns | +0.3% | -0.2% | +0.2% |
| 3 | 2082944ns | -0.7% | +0.2% | -0.5% |
| 4 | 2068761ns | -0.2% | +0.4% | +0.4% |
| 5 | 2074440ns | -0.6% | +0.1% | +0.0% |
| 6 | 2070702ns | +0.2% | +0.1% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.010 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.165 | ok |
| abi_sink_wideselect_null_sink | -0.106 | ok |
| abi_sink_wideselect_per_record_sink | 0.035 | ok |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 3/6, lost 3/6
- **abi_sink_wideselect_batched_sink_decode**: won 2/6, lost 2/6
- **abi_sink_wideselect_per_record_sink**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6315399.1ns | 2097541.7ns | 301.1% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6285165.6ns | 2074281.0ns | 303.0% | HIGH |
| abi_sink_wideselect_null_sink | 6412759.4ns | 2088425.9ns | 307.1% | HIGH |
| abi_sink_wideselect_per_record_sink | 6283819.7ns | 2074711.4ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2062588.7-2157294.8 ns)
  2062588.7 |########################################
  2067324.0 |####################
  2072059.3 |########################################
  2076794.6 |
  2081529.9 |
  2086265.2 |
  2091000.5 |
  2095735.8 |
  2100471.1 |
  2105206.4 |
  2109941.8 |
  2114677.1 |
  2119412.4 |
  2124147.7 |
  2128883.0 |
  2133618.3 |
  2138353.6 |
  2143088.9 |
  2147824.2 |
  2152559.5 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2064782.5-2082083.9 ns)
  2064782.5 |########################################
  2065647.6 |
  2066512.6 |
  2067377.7 |
  2068242.8 |
  2069107.9 |########################################
  2069972.9 |
  2070838.0 |
  2071703.1 |########################################
  2072568.2 |
  2073433.2 |
  2074298.3 |
  2075163.4 |########################################
  2076028.4 |########################################
  2076893.5 |
  2077758.6 |
  2078623.7 |
  2079488.7 |
  2080353.8 |
  2081218.9 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2068761.2-2123920.2 ns)
  2068761.2 |########################################
  2071519.2 |
  2074277.1 |#############
  2077035.1 |
  2079793.0 |
  2082551.0 |#############
  2085308.9 |
  2088066.9 |
  2090824.8 |
  2093582.8 |
  2096340.7 |
  2099098.7 |
  2101856.6 |
  2104614.6 |
  2107372.5 |
  2110130.5 |
  2112888.4 |
  2115646.4 |
  2118404.3 |
  2121162.3 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2072064.2-2077088.3 ns)
  2072064.2 |########################################
  2072315.4 |
  2072566.6 |
  2072817.8 |
  2073069.0 |
  2073320.2 |
  2073571.4 |
  2073822.6 |
  2074073.8 |
  2074325.0 |
  2074576.2 |####################
  2074827.5 |####################
  2075078.7 |
  2075329.9 |
  2075581.1 |
  2075832.3 |
  2076083.5 |
  2076334.7 |
  2076585.9 |####################
  2076837.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=302.8% of algo (FFI overhead may distort results)
