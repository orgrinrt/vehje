# abi_sink (tight)

4 variants, 6 samples per variant.
Baseline: **abi_sink_tight_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_tight_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_sink_tight_null_sink is fastest but the noisiest (CV 18.6%)

abi_sink_tight_null_sink wins on median (2.03 ms) yet has the highest variance (CV 18.6%), while abi_sink_tight_per_record_sink is the steadiest (CV 0.6%, 2.03 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (12.31 us) is smaller than the fastest variant's own run-to-run std-dev (377.58 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (abi_sink_tight_null_sink)

The baseline abi_sink_tight_null_sink is the fastest (2.03 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader abi_sink_tight_null_sink vs stability leader abi_sink_tight_per_record_sink (+0% speed for 33.4x steadier)

abi_sink_tight_null_sink is fastest (2.03 ms, CV 18.6%); abi_sink_tight_per_record_sink gives up 0.3% median for 33.4x lower variance (CV 0.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.03 ms and 2.04 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_tight_null_sink) is the fastest** at 2029098.1 ns median
- Spread: 1.01x (fastest 2029098.1 ns, slowest 2041409.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2062784ns | 2039370ns | 2032620ns | 2038555ns | 2114209ns | -10.06% |
| abi_sink_tight_batched_sink_decode | 2075819ns | 2044208ns | 2038962ns | 2043455ns | 2142794ns | -9.49% |
| abi_sink_tight_null_sink | 2293593ns | 2031884ns | 2019845ns | 2028741ns | 2827745ns | base |
| abi_sink_tight_per_record_sink | 2041208ns | 2037077ns | 2032592ns | 2036438ns | 2052671ns | -11.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_tight_batched_sink | 2059846ns | 2030118ns | 2110560ns | -10.05% | 0.000 |
| abi_sink_tight_batched_sink_decode | 2062449ns | 2036335ns | 2108170ns | -9.94% | 0.000 |
| abi_sink_tight_null_sink | 2290073ns | 2017240ns | 2822743ns | base | 0.000 |
| abi_sink_tight_per_record_sink | 2038389ns | 2029829ns | 2049610ns | -10.99% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 49973.3 | 2063888.5 | 2059846.3 | n/a |
| abi_sink_tight_batched_sink_decode | 50353.2 | 2061433.8 | 2062448.7 | n/a |
| abi_sink_tight_null_sink | 69240.0 | 2177746.0 | 2290072.7 | n/a |
| abi_sink_tight_per_record_sink | 48307.5 | 2039438.7 | 2038389.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_tight_null_sink; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_tight_batched_sink | 0.000 | 99.0% |
| abi_sink_tight_batched_sink_decode | 0.000 | 98.8% |
| abi_sink_tight_null_sink | 0.000 | 99.4% |
| abi_sink_tight_per_record_sink | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_tight_batched_sink | 2062784ns | 2062784ns | -10.06% |
| abi_sink_tight_batched_sink_decode | 2075819ns | 2075819ns | -9.49% |
| abi_sink_tight_null_sink | 2293593ns | 2293593ns | base |
| abi_sink_tight_per_record_sink | 2041208ns | 2041208ns | -11.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_tight_null_sink | 2029098ns | base | --- | [2018377, 2822743] | --- | --- | --- | --- |
| abi_sink_tight_batched_sink | 2036773ns | no significant difference | [-712182, +13829]ns | [2032206, 2110560] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_tight_batched_sink_decode | 2041410ns | no significant difference | [-714573, +19390]ns | [2037767, 2108170] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_tight_per_record_sink | 2034311ns | no significant difference | [-773543, +12868]ns | [2031245, 2049610] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_tight_null_sink | abi_sink_tight_batched_sink | abi_sink_tight_batched_sink_decode | abi_sink_tight_per_record_sink |
|---|---|---|---|---|
| 1 | 2017240ns | +0.6% | +0.9% | +0.6% |
| 2 | 2029572ns | +0.3% | +0.6% | +0.2% |
| 3 | 2019514ns | +0.7% | +1.0% | +0.7% |
| 4 | 2777535ns | -26.6% | -25.2% | -25.7% |
| 5 | 2028624ns | +0.5% | +0.7% | +0.4% |
| 6 | 2867951ns | -23.9% | -25.4% | -29.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_tight_batched_sink | -0.013 | ok |
| abi_sink_tight_batched_sink_decode | -0.136 | ok |
| abi_sink_tight_null_sink | -0.314 | moderate- |
| abi_sink_tight_per_record_sink | -0.169 | ok |

**Consistency summary:**

- **abi_sink_tight_batched_sink**: won 2/6, lost 4/6
- **abi_sink_tight_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_tight_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_tight_batched_sink | 6240629.1ns | 2059846.3ns | 303.0% | HIGH |
| abi_sink_tight_batched_sink_decode | 6245819.1ns | 2062448.7ns | 302.8% | HIGH |
| abi_sink_tight_null_sink | 6585539.5ns | 2290072.7ns | 287.6% | HIGH |
| abi_sink_tight_per_record_sink | 6186616.9ns | 2038389.0ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_tight_batched_sink (n=6, range 2030117.5-2110560.4 ns)
  2030117.5 |####################
  2034139.6 |########################################
  2038161.8 |########################################
  2042183.9 |
  2046206.1 |
  2050228.2 |
  2054250.4 |
  2058272.5 |
  2062294.7 |
  2066316.8 |
  2070338.9 |
  2074361.1 |
  2078383.2 |
  2082405.4 |
  2086427.5 |
  2090449.7 |
  2094471.8 |
  2098494.0 |
  2102516.1 |
  2106538.3 |
  (0 below, 1 above range)

abi_sink_tight_batched_sink_decode (n=6, range 2036335.4-2108169.6 ns)
  2036335.4 |########################################
  2039927.1 |########################################
  2043518.8 |
  2047110.5 |
  2050702.2 |
  2054293.9 |
  2057885.7 |
  2061477.4 |
  2065069.1 |
  2068660.8 |
  2072252.5 |
  2075844.2 |####################
  2079435.9 |
  2083027.6 |
  2086619.3 |
  2090211.1 |
  2093802.8 |
  2097394.5 |
  2100986.2 |
  2104577.9 |
  (0 below, 1 above range)

abi_sink_tight_null_sink (n=6, range 2017240.4-2822742.9 ns)
  2017240.4 |########################################
  2057515.5 |
  2097790.6 |
  2138065.8 |
  2178340.9 |
  2218616.0 |
  2258891.1 |
  2299166.3 |
  2339441.4 |
  2379716.5 |
  2419991.6 |
  2460266.8 |
  2500541.9 |
  2540817.0 |
  2581092.1 |
  2621367.3 |
  2661642.4 |
  2701917.5 |
  2742192.6 |##########
  2782467.8 |
  (0 below, 1 above range)

abi_sink_tight_per_record_sink (n=6, range 2029828.7-2049610.0 ns)
  2029828.7 |########################################
  2030817.8 |
  2031806.8 |########################################
  2032795.9 |########################################
  2033785.0 |
  2034774.0 |########################################
  2035763.1 |########################################
  2036752.2 |
  2037741.2 |
  2038730.3 |
  2039719.4 |
  2040708.4 |
  2041697.5 |
  2042686.5 |
  2043675.6 |
  2044664.7 |
  2045653.7 |
  2046642.8 |
  2047631.9 |
  2048620.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_tight_batched_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_tight_batched_sink_decode**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_tight_null_sink**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_sink_tight_per_record_sink**: bridge=302.7% of algo (FFI overhead may distort results)
