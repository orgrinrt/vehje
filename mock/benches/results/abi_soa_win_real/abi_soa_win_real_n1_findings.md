# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_real_null_entry dominates: 42899% faster than the next best (abi_soa_win_real_scalar_payload)

abi_soa_win_real_null_entry (5.05 us) leads abi_soa_win_real_scalar_payload (2.17 ms) by 42899%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.17 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_soa_payload is an outlier: 430.2x slower than the field

abi_soa_win_real_soa_payload (2.17 ms) is 430.2x the fastest (5.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 430.2x the fastest

Fastest abi_soa_win_real_null_entry (5.05 us) to slowest abi_soa_win_real_soa_payload (2.17 ms): 430.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 5049.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 430.23x (fastest 5049.8 ns, slowest 2172598.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 7417ns | 7359ns | 7096ns | 7272ns | 7796ns | -99.67% |
| abi_soa_win_real_scalar_payload | 2219257ns | 2174578ns | 2165898ns | 2172285ns | 2316393ns | base |
| abi_soa_win_real_soa_payload | 2190004ns | 2176081ns | 2157160ns | 2172102ns | 2233280ns | -1.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5042ns | 4858ns | 5216ns | -99.77% | 0.000 |
| abi_soa_win_real_scalar_payload | 2216073ns | 2162713ns | 2312934ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 2186659ns | 2154485ns | 2229671ns | -1.33% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 29037.4 | 5096.9 | 5041.5 | n/a |
| abi_soa_win_real_scalar_payload | 62340.1 | 2196518.1 | 2216073.4 | n/a |
| abi_soa_win_real_soa_payload | 64592.1 | 2189692.0 | 2186658.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.000 | 96.2% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_real_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 7417ns | 7417ns | -99.67% |
| abi_soa_win_real_scalar_payload | 2219257ns | 2219257ns | base |
| abi_soa_win_real_soa_payload | 2190004ns | 2190004ns | -1.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2171386ns | base | --- | [2163900, 2312934] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 5050ns | -2166527.1ns (-99.8%) | [-2307723, -2158846]ns | [4859, 5216] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 2172599ns | no significant difference | [-90940, +3738]ns | [2157706, 2229671] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2165087ns | -99.8% | -0.0% |
| 2 | 2383050ns | -99.8% | -7.2% |
| 3 | 2162713ns | -99.8% | -0.1% |
| 4 | 2165300ns | -99.8% | -0.5% |
| 5 | 2177473ns | -99.8% | +0.1% |
| 6 | 2242818ns | -99.8% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | 0.201 | moderate+ |
| abi_soa_win_real_scalar_payload | -0.362 | moderate- |
| abi_soa_win_real_soa_payload | -0.085 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 126966.3ns | 5041.5ns | 2518.4% | HIGH |
| abi_soa_win_real_scalar_payload | 6709121.9ns | 2216073.4ns | 302.7% | HIGH |
| abi_soa_win_real_soa_payload | 6633107.3ns | 2186658.6ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 4857.5-5215.6 ns)
   4857.5 |########################################
   4875.4 |
   4893.3 |####################
   4911.2 |
   4929.1 |
   4947.0 |
   4964.9 |
   4982.8 |
   5000.7 |
   5018.6 |
   5036.6 |
   5054.5 |
   5072.4 |
   5090.3 |
   5108.2 |
   5126.1 |
   5144.0 |
   5161.9 |
   5179.8 |
   5197.7 |########################################
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2162713.3-2312933.8 ns)
  2162713.3 |########################################
  2170224.3 |#############
  2177735.3 |
  2185246.4 |
  2192757.4 |
  2200268.4 |
  2207779.4 |
  2215290.5 |
  2222801.5 |
  2230312.5 |
  2237823.5 |#############
  2245334.5 |
  2252845.6 |
  2260356.6 |
  2267867.6 |
  2275378.6 |
  2282889.7 |
  2290400.7 |
  2297911.7 |
  2305422.7 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 2154485.0-2229670.9 ns)
  2154485.0 |########################################
  2158244.3 |########################################
  2162003.6 |########################################
  2165762.9 |
  2169522.2 |
  2173281.5 |
  2177040.8 |########################################
  2180800.0 |
  2184559.3 |
  2188318.6 |
  2192077.9 |
  2195837.2 |
  2199596.5 |
  2203355.8 |
  2207115.1 |
  2210874.4 |########################################
  2214633.7 |
  2218393.0 |
  2222152.3 |
  2225911.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=2482.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=303.4% of algo (FFI overhead may distort results)
