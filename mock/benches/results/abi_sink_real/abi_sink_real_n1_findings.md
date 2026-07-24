# abi_sink (real)

4 variants, 6 samples per variant.
Baseline: **abi_sink_real_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_real_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.13 ms and 2.15 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_real_batched_sink** at 2134206.7 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2134206.7 ns, slowest 2147127.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2138084ns | 2136797ns | 2133935ns | 2135883ns | 2143460ns | -2.86% |
| abi_sink_real_batched_sink_decode | 2149452ns | 2149530ns | 2133247ns | 2148456ns | 2159050ns | -2.35% |
| abi_sink_real_null_sink | 2201137ns | 2148222ns | 2135325ns | 2146592ns | 2315859ns | base |
| abi_sink_real_per_record_sink | 2147714ns | 2147702ns | 2137185ns | 2144871ns | 2157244ns | -2.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_real_batched_sink | 2135541ns | 2131459ns | 2140845ns | -2.86% | 0.000 |
| abi_sink_real_batched_sink_decode | 2146962ns | 2130737ns | 2156493ns | -2.34% | 0.000 |
| abi_sink_real_null_sink | 2198331ns | 2132859ns | 2312652ns | base | 0.000 |
| abi_sink_real_per_record_sink | 2145166ns | 2134586ns | 2154681ns | -2.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 38444.4 | 2133914.8 | 2135540.8 | n/a |
| abi_sink_real_batched_sink_decode | 39232.8 | 2152932.4 | 2146962.2 | n/a |
| abi_sink_real_null_sink | 47533.4 | 2194725.3 | 2198330.6 | n/a |
| abi_sink_real_per_record_sink | 39617.4 | 2144662.3 | 2145166.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_real_batched_sink_decode; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_real_batched_sink | 0.000 | 99.8% |
| abi_sink_real_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_real_null_sink | 0.000 | 99.3% |
| abi_sink_real_per_record_sink | 0.000 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_real_batched_sink | 2138084ns | 2138084ns | -2.86% |
| abi_sink_real_batched_sink_decode | 2149452ns | 2149452ns | -2.35% |
| abi_sink_real_null_sink | 2201137ns | 2201137ns | base |
| abi_sink_real_per_record_sink | 2147714ns | 2147714ns | -2.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_real_null_sink | 2145653ns | base | --- | [2136686, 2312652] | --- | --- | --- | --- |
| abi_sink_real_batched_sink | 2134207ns | -7696.4ns (-0.4%) | [-178446, -2228]ns | [2131571, 2140845] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_real_batched_sink_decode | 2147127ns | no significant difference | [-171569, +12292]ns | [2137266, 2156493] | no | 1.0000 | 0.6875 | 0 |
| abi_sink_real_per_record_sink | 2145179ns | no significant difference | [-169210, +10329]ns | [2135639, 2154681] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_real_null_sink | abi_sink_real_batched_sink | abi_sink_real_batched_sink_decode | abi_sink_real_per_record_sink |
|---|---|---|---|---|
| 1 | 2468342ns | -13.5% | -13.7% | -13.1% |
| 2 | 2149656ns | -0.2% | +0.6% | -0.7% |
| 3 | 2156962ns | -1.1% | -0.3% | -0.2% |
| 4 | 2141650ns | -0.5% | +0.1% | +0.7% |
| 5 | 2140514ns | -0.2% | +0.4% | +0.2% |
| 6 | 2132859ns | -0.1% | +0.6% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_real_batched_sink | -0.240 | moderate- |
| abi_sink_real_batched_sink_decode | -0.379 | moderate- |
| abi_sink_real_null_sink | -0.020 | ok |
| abi_sink_real_per_record_sink | 0.037 | ok |

**Consistency summary:**

- **abi_sink_real_batched_sink**: won 5/6, lost 0/6
- **abi_sink_real_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_real_per_record_sink**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_real_batched_sink | 6444461.2ns | 2135540.8ns | 301.8% | HIGH |
| abi_sink_real_batched_sink_decode | 6490086.5ns | 2146962.2ns | 302.3% | HIGH |
| abi_sink_real_null_sink | 6655198.7ns | 2198330.6ns | 302.7% | HIGH |
| abi_sink_real_per_record_sink | 6476282.0ns | 2145166.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_sink_real_batched_sink (n=6, range 2131459.2-2140845.0 ns)
  2131459.2 |########################################
  2131928.5 |
  2132397.8 |
  2132867.1 |
  2133336.4 |####################
  2133805.7 |
  2134274.9 |
  2134744.2 |####################
  2135213.5 |
  2135682.8 |
  2136152.1 |
  2136621.4 |
  2137090.7 |####################
  2137560.0 |
  2138029.3 |
  2138498.5 |
  2138967.8 |
  2139437.1 |
  2139906.4 |
  2140375.7 |
  (0 below, 1 above range)

abi_sink_real_batched_sink_decode (n=6, range 2130737.1-2156493.0 ns)
  2130737.1 |########################################
  2132024.9 |
  2133312.7 |
  2134600.5 |
  2135888.3 |
  2137176.1 |
  2138463.9 |
  2139751.6 |
  2141039.4 |
  2142327.2 |
  2143615.0 |########################################
  2144902.8 |########################################
  2146190.6 |
  2147478.4 |########################################
  2148766.2 |
  2150054.0 |
  2151341.8 |########################################
  2152629.6 |
  2153917.4 |
  2155205.2 |
  (0 below, 1 above range)

abi_sink_real_null_sink (n=6, range 2132858.8-2312652.3 ns)
  2132858.8 |########################################
  2141848.5 |#############
  2150838.1 |#############
  2159827.8 |
  2168817.5 |
  2177807.2 |
  2186796.8 |
  2195786.5 |
  2204776.2 |
  2213765.9 |
  2222755.5 |
  2231745.2 |
  2240734.9 |
  2249724.6 |
  2258714.2 |
  2267703.9 |
  2276693.6 |
  2285683.3 |
  2294672.9 |
  2303662.6 |
  (0 below, 1 above range)

abi_sink_real_per_record_sink (n=6, range 2134585.8-2154681.0 ns)
  2134585.8 |####################
  2135590.6 |
  2136595.3 |####################
  2137600.1 |
  2138604.8 |
  2139609.6 |
  2140614.4 |
  2141619.1 |
  2142623.9 |
  2143628.6 |
  2144633.4 |########################################
  2145638.2 |
  2146642.9 |
  2147647.7 |
  2148652.4 |
  2149657.2 |
  2150662.0 |
  2151666.7 |####################
  2152671.5 |
  2153676.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_real_batched_sink**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_sink_real_batched_sink_decode**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_real_null_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_real_per_record_sink**: bridge=301.8% of algo (FFI overhead may distort results)
