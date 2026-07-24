# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 92964% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (2.33 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 92964%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_ffi_batched_scalar is an outlier: 942.0x slower than the field

abi_cross_scalar_real_ffi_batched_scalar (2.19 ms) is 942.0x the fastest (2.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} (92964% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_inproc_fnptr, abi_cross_scalar_real_ffi_batched_scalar} with a 92964% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 942.0x the fastest

Fastest abi_cross_scalar_real_null_entry (2.33 us) to slowest abi_cross_scalar_real_ffi_batched_scalar (2.19 ms): 942.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 2326.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 942.03x (fastest 2326.4 ns, slowest 2191577.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2226924ns | 2195259ns | 2178618ns | 2190042ns | 2306399ns | +2.25% |
| abi_cross_scalar_real_inproc_direct | 2177918ns | 2168500ns | 2165305ns | 2167925ns | 2199214ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2193729ns | 2191739ns | 2179036ns | 2188324ns | 2209183ns | +0.73% |
| abi_cross_scalar_real_null_entry | 4652ns | 4655ns | 4508ns | 4616ns | 4777ns | -99.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2223128ns | 2175045ns | 2302211ns | +2.23% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2174541ns | 2161801ns | 2195897ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2190308ns | 2175705ns | 2205599ns | +0.73% | 0.000 |
| abi_cross_scalar_real_null_entry | 2318ns | 2252ns | 2372ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 76918.7 | 2251208.1 | 2223127.7 | n/a |
| abi_cross_scalar_real_inproc_direct | 10790.2 | 2298284.7 | 2174541.2 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10724.0 | 2191118.4 | 2190307.5 | 0 |
| abi_cross_scalar_real_null_entry | 29839.4 | 2490.2 | 2317.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_real_null_entry | 0.014 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2226924ns | 2226924ns | +2.25% |
| abi_cross_scalar_real_inproc_direct | 2177918ns | 2177918ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2193729ns | 2193729ns | +0.73% |
| abi_cross_scalar_real_null_entry | 4652ns | 4652ns | -99.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2165080ns | base | --- | [2162646, 2195897] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2191577ns | +26496.9ns (+1.2%) | [+12949, +106314]ns | [2175595, 2302211] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2188296ns | no significant difference | [-17360, +41403]ns | [2177027, 2205599] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_null_entry | 2326ns | -2162794.0ns (-99.9%) | [-2193558, -2160318]ns | [2255, 2372] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2166593ns | +1.5% | +1.8% | -99.9% |
| 2 | 2163492ns | +0.5% | +0.6% | -99.9% |
| 3 | 2173082ns | +2.9% | +0.3% | -99.9% |
| 4 | 2218711ns | +6.8% | -1.8% | -99.9% |
| 5 | 2161801ns | +0.7% | +2.0% | -99.9% |
| 6 | 2163568ns | +1.0% | +1.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | -0.095 | ok |
| abi_cross_scalar_real_inproc_direct | -0.158 | ok |
| abi_cross_scalar_real_inproc_fnptr | 0.014 | ok |
| abi_cross_scalar_real_null_entry | 0.288 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 0/6, lost 6/6
- **abi_cross_scalar_real_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6793632.2ns | 2223127.7ns | 305.6% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6660150.8ns | 2174541.2ns | 306.3% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6586926.6ns | 2190307.5ns | 300.7% | HIGH |
| abi_cross_scalar_real_null_entry | 118124.2ns | 2317.8ns | 5096.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2175044.6-2302210.6 ns)
  2175044.6 |########################################
  2181402.9 |####################
  2187761.2 |
  2194119.5 |####################
  2200477.8 |
  2206836.1 |
  2213194.4 |
  2219552.7 |
  2225911.0 |
  2232269.3 |####################
  2238627.6 |
  2244985.9 |
  2251344.2 |
  2257702.5 |
  2264060.8 |
  2270419.1 |
  2276777.4 |
  2283135.7 |
  2289494.0 |
  2295852.3 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2161800.8-2195896.9 ns)
  2161800.8 |########################################
  2163505.6 |####################
  2165210.4 |####################
  2166915.2 |
  2168620.0 |
  2170324.8 |
  2172029.6 |####################
  2173734.4 |
  2175439.2 |
  2177144.0 |
  2178848.8 |
  2180553.6 |
  2182258.4 |
  2183963.2 |
  2185668.0 |
  2187372.8 |
  2189077.6 |
  2190782.4 |
  2192487.2 |
  2194192.0 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2175705.0-2205599.4 ns)
  2175705.0 |########################################
  2177199.7 |########################################
  2178694.4 |########################################
  2180189.2 |
  2181683.9 |
  2183178.6 |
  2184673.3 |
  2186168.0 |
  2187662.8 |
  2189157.5 |
  2190652.2 |
  2192146.9 |
  2193641.6 |
  2195136.4 |
  2196631.1 |########################################
  2198125.8 |
  2199620.5 |
  2201115.2 |
  2202610.0 |
  2204104.7 |########################################
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 2252.1-2372.3 ns)
   2252.1 |########################################
   2258.1 |
   2264.1 |
   2270.1 |
   2276.1 |
   2282.2 |
   2288.2 |
   2294.2 |
   2300.2 |
   2306.2 |
   2312.2 |####################
   2318.2 |
   2324.2 |
   2330.2 |
   2336.2 |########################################
   2342.2 |
   2348.3 |
   2354.3 |
   2360.3 |
   2366.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=5074.1% of algo (FFI overhead may distort results)
