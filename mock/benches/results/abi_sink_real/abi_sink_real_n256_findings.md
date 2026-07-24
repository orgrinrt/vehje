# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (14.86 us) is smaller than the fastest variant's own run-to-run std-dev (50.04 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.7% of the fastest

All 4 variants sit between 2.14 ms and 2.16 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2141309.8 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 2141309.8 ns, slowest 2156168.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2164960ns | 2144148ns | 2131919ns | 2142675ns | 2214908ns | +0.67% |
| abi_sink_real_batched_sink_decode | 2211559ns | 2149321ns | 2135200ns | 2148054ns | 2344994ns | +2.83% |
| abi_sink_real_null_sink | 2150591ns | 2150858ns | 2141305ns | 2150043ns | 2156055ns | base |
| abi_sink_real_per_record_sink | 2169798ns | 2159088ns | 2145389ns | 2157748ns | 2200079ns | +0.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2162023ns | 2129066ns | 2211881ns | +0.67% | 0.000 |
| abi_sink_real_batched_sink_decode | 2208384ns | 2132539ns | 2341146ns | +2.83% | 0.000 |
| abi_sink_real_null_sink | 2147660ns | 2138434ns | 2153089ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2166729ns | 2142598ns | 2196607ns | +0.89% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 56306.0 | 2143125.1 | 2162022.7 | n/a |
| abi_sink_real_batched_sink_decode | 58808.3 | 2211139.9 | 2208383.7 | n/a |
| abi_sink_real_null_sink | 54611.1 | 2148591.0 | 2147659.6 | n/a |
| abi_sink_real_per_record_sink | 60284.1 | 2166813.0 | 2166728.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.4% |
| abi_sink_real_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_real_null_sink | 0.000 | 99.1% |
| abi_sink_real_per_record_sink | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2164960ns | 2164960ns | +0.67% |
| abi_sink_real_batched_sink_decode | 2211559ns | 2211559ns | +2.83% |
| abi_sink_real_null_sink | 2150591ns | 2150591ns | base |
| abi_sink_real_per_record_sink | 2169798ns | 2169798ns | +0.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2147868ns | base | --- | [2142022, 2153089] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2141310ns | no significant difference | [-12756, +60984]ns | [2132877, 2211881] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_real_batched_sink_decode | 2146400ns | no significant difference | [-9783, +190249]ns | [2137605, 2341146] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_real_per_record_sink | 2156169ns | +6109.1ns (+0.3%) | [+4791, +46307]ns | [2147410, 2196607] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2138434ns | -0.4% | +0.2% | +0.2% |
| 2 | 2151188ns | -0.7% | -0.9% | +0.3% |
| 3 | 2154991ns | +5.5% | +17.4% | +3.3% |
| 4 | 2146803ns | +0.2% | +0.3% | +0.3% |
| 5 | 2148932ns | -0.5% | -0.0% | +0.3% |
| 6 | 2145610ns | -0.0% | -0.0% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.168 | ok |
| abi_sink_real_batched_sink_decode | -0.241 | moderate- |
| abi_sink_real_null_sink | -0.106 | ok |
| abi_sink_real_per_record_sink | -0.228 | moderate- |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 3/6, lost 2/6
- **abi_sink_real_batched_sink_decode**: won 1/6, lost 3/6
- **abi_sink_real_per_record_sink**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6490272.2ns | 2162022.7ns | 300.2% | HIGH |
| abi_sink_real_batched_sink_decode | 6707963.2ns | 2208383.7ns | 303.7% | HIGH |
| abi_sink_real_null_sink | 6505620.2ns | 2147659.6ns | 302.9% | HIGH |
| abi_sink_real_per_record_sink | 6560669.4ns | 2166728.6ns | 302.8% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2129066.2-2211881.5 ns)
  2129066.2 |########################################
  2133207.0 |########################################
  2137347.7 |########################################
  2141488.5 |########################################
  2145629.2 |
  2149770.0 |########################################
  2153910.8 |
  2158051.5 |
  2162192.3 |
  2166333.1 |
  2170473.8 |
  2174614.6 |
  2178755.4 |
  2182896.1 |
  2187036.9 |
  2191177.6 |
  2195318.4 |
  2199459.2 |
  2203599.9 |
  2207740.7 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2132539.2-2341146.5 ns)
  2132539.2 |##########################
  2142969.6 |########################################
  2153399.9 |
  2163830.3 |
  2174260.7 |
  2184691.0 |
  2195121.4 |
  2205551.7 |
  2215982.1 |
  2226412.5 |
  2236842.8 |
  2247273.2 |
  2257703.6 |
  2268133.9 |
  2278564.3 |
  2288994.6 |
  2299425.0 |
  2309855.4 |
  2320285.7 |
  2330716.1 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2138434.2-2153089.4 ns)
  2138434.2 |########################################
  2139167.0 |
  2139899.7 |
  2140632.5 |
  2141365.2 |
  2142098.0 |
  2142830.7 |
  2143563.5 |
  2144296.3 |
  2145029.0 |########################################
  2145761.8 |
  2146494.5 |########################################
  2147227.3 |
  2147960.0 |
  2148692.8 |########################################
  2149425.6 |
  2150158.3 |
  2150891.1 |########################################
  2151623.8 |
  2152356.6 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2142598.3-2196607.1 ns)
  2142598.3 |########################################
  2145298.7 |
  2147999.2 |
  2150699.6 |########################################
  2153400.1 |########################################
  2156100.5 |########################################
  2158800.9 |
  2161501.4 |
  2164201.8 |
  2166902.3 |########################################
  2169602.7 |
  2172303.1 |
  2175003.6 |
  2177704.0 |
  2180404.5 |
  2183104.9 |
  2185805.3 |
  2188505.8 |
  2191206.2 |
  2193906.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=302.8% of algo (FFI overhead may distort results)
