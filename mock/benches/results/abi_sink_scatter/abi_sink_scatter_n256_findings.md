# abi_sink (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_sink_scatter_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_scatter_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.6% of the fastest

All 4 variants sit between 2.13 ms and 2.14 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_scatter_batched_sink_decode** at 2131745.9 ns median (-0.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2131745.9 ns, slowest 2144717.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2216720ns | 2137309ns | 2134740ns | 2136838ns | 2377533ns | +3.11% |
| abi_sink_scatter_batched_sink_decode | 2134450ns | 2134468ns | 2129617ns | 2134285ns | 2137115ns | -0.72% |
| abi_sink_scatter_null_sink | 2149956ns | 2140731ns | 2136510ns | 2139942ns | 2171700ns | base |
| abi_sink_scatter_per_record_sink | 2145733ns | 2147421ns | 2138952ns | 2145416ns | 2149598ns | -0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 2213685ns | 2131969ns | 2374189ns | +3.10% | 0.000 |
| abi_sink_scatter_batched_sink_decode | 2131781ns | 2126921ns | 2134504ns | -0.71% | 0.000 |
| abi_sink_scatter_null_sink | 2147057ns | 2133721ns | 2168676ns | base | 0.000 |
| abi_sink_scatter_per_record_sink | 2142941ns | 2136190ns | 2146690ns | -0.19% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 54886.4 | 2190104.4 | 2213685.1 | n/a |
| abi_sink_scatter_batched_sink_decode | 43403.9 | 2133197.9 | 2131781.1 | n/a |
| abi_sink_scatter_null_sink | 47472.0 | 2147518.5 | 2147056.5 | n/a |
| abi_sink_scatter_per_record_sink | 45936.6 | 2142065.1 | 2142940.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_scatter_batched_sink_decode; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_scatter_batched_sink | 0.000 | 99.6% |
| abi_sink_scatter_batched_sink_decode | 0.000 | 99.8% |
| abi_sink_scatter_null_sink | 0.000 | 99.5% |
| abi_sink_scatter_per_record_sink | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_scatter_batched_sink | 2216720ns | 2216720ns | +3.11% |
| abi_sink_scatter_batched_sink_decode | 2134450ns | 2134450ns | -0.72% |
| abi_sink_scatter_null_sink | 2149956ns | 2149956ns | base |
| abi_sink_scatter_per_record_sink | 2145733ns | 2145733ns | -0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_scatter_null_sink | 2137803ns | base | --- | [2134691, 2168676] | --- | --- | --- | --- |
| abi_sink_scatter_batched_sink | 2134426ns | no significant difference | [-6916, +210706]ns | [2132440, 2374189] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_scatter_batched_sink_decode | 2131746ns | -8270.9ns (-0.4%) | [-35224, -2331]ns | [2129093, 2134504] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_sink_scatter_per_record_sink | 2144718ns | no significant difference | [-27160, +10945]ns | [2137414, 2146690] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_scatter_null_sink | abi_sink_scatter_batched_sink | abi_sink_scatter_batched_sink_decode | abi_sink_scatter_per_record_sink |
|---|---|---|---|---|
| 1 | 2137029ns | -0.2% | -0.5% | +0.4% |
| 2 | 2135661ns | -0.2% | -0.2% | +0.1% |
| 3 | 2191537ns | +19.1% | -2.6% | -2.0% |
| 4 | 2133721ns | +0.1% | -0.0% | +0.6% |
| 5 | 2145815ns | -0.4% | -0.7% | -0.4% |
| 6 | 2138577ns | -0.3% | -0.3% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_scatter_batched_sink | -0.236 | moderate- |
| abi_sink_scatter_batched_sink_decode | 0.145 | ok |
| abi_sink_scatter_null_sink | -0.390 | moderate- |
| abi_sink_scatter_per_record_sink | -0.423 | moderate- |

**Consistency summary:**

- **abi_sink_scatter_batched_sink**: won 4/6, lost 1/6
- **abi_sink_scatter_batched_sink_decode**: won 5/6, lost 0/6
- **abi_sink_scatter_per_record_sink**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_scatter_batched_sink | 6666876.0ns | 2213685.1ns | 301.2% | HIGH |
| abi_sink_scatter_batched_sink_decode | 6443821.1ns | 2131781.1ns | 302.3% | HIGH |
| abi_sink_scatter_null_sink | 6593701.0ns | 2147056.5ns | 307.1% | HIGH |
| abi_sink_scatter_per_record_sink | 6473799.7ns | 2142940.6ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_scatter_batched_sink (n=6, range 2131968.8-2374189.1 ns)
  2131968.8 |########################################
  2144079.8 |
  2156190.8 |
  2168301.9 |
  2180412.9 |
  2192523.9 |
  2204634.9 |
  2216745.9 |
  2228856.9 |
  2240968.0 |
  2253079.0 |
  2265190.0 |
  2277301.0 |
  2289412.0 |
  2301523.0 |
  2313634.1 |
  2325745.1 |
  2337856.1 |
  2349967.1 |
  2362078.1 |
  (0 below, 1 above range)

abi_sink_scatter_batched_sink_decode (n=6, range 2126921.2-2134504.0 ns)
  2126921.2 |####################
  2127300.3 |
  2127679.5 |
  2128058.6 |
  2128437.8 |
  2128816.9 |
  2129196.0 |
  2129575.2 |
  2129954.3 |
  2130333.5 |
  2130712.6 |
  2131091.7 |########################################
  2131470.9 |
  2131850.0 |####################
  2132229.2 |
  2132608.3 |
  2132987.4 |
  2133366.6 |####################
  2133745.7 |
  2134124.9 |
  (0 below, 1 above range)

abi_sink_scatter_null_sink (n=6, range 2133721.2-2168675.9 ns)
  2133721.2 |####################
  2135468.9 |########################################
  2137216.7 |####################
  2138964.4 |
  2140712.1 |
  2142459.9 |
  2144207.6 |####################
  2145955.3 |
  2147703.1 |
  2149450.8 |
  2151198.5 |
  2152946.3 |
  2154694.0 |
  2156441.7 |
  2158189.5 |
  2159937.2 |
  2161684.9 |
  2163432.7 |
  2165180.4 |
  2166928.1 |
  (0 below, 1 above range)

abi_sink_scatter_per_record_sink (n=6, range 2136190.4-2146690.2 ns)
  2136190.4 |########################################
  2136715.4 |
  2137240.4 |
  2137765.4 |
  2138290.4 |########################################
  2138815.4 |
  2139340.3 |
  2139865.3 |
  2140390.3 |
  2140915.3 |
  2141440.3 |
  2141965.3 |
  2142490.3 |
  2143015.3 |########################################
  2143540.3 |
  2144065.2 |
  2144590.2 |
  2145115.2 |
  2145640.2 |########################################
  2146165.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_scatter_batched_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_scatter_batched_sink_decode**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_sink_scatter_null_sink**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_sink_scatter_per_record_sink**: bridge=301.5% of algo (FFI overhead may distort results)
