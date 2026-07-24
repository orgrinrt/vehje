# abi_cross_scalar (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_scatter_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_scatter_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_scatter_null_entry dominates: 84416% faster than the next best (abi_cross_scalar_scatter_inproc_direct)

abi_cross_scalar_scatter_null_entry (2.57 us) leads abi_cross_scalar_scatter_inproc_direct (2.17 ms) by 84416%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_scatter_null_entry beats baseline by 100% (significant)

abi_cross_scalar_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_cross_scalar_scatter_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_scatter_inproc_fnptr is an outlier: 853.8x slower than the field

abi_cross_scalar_scatter_inproc_fnptr (2.19 ms) is 853.8x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_scatter_null_entry} vs {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} (84416% apart)

The field splits into a fast tier {abi_cross_scalar_scatter_null_entry} and a slow tier {abi_cross_scalar_scatter_inproc_direct, abi_cross_scalar_scatter_ffi_batched_scalar, abi_cross_scalar_scatter_inproc_fnptr} with a 84416% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 853.8x the fastest

Fastest abi_cross_scalar_scatter_null_entry (2.57 us) to slowest abi_cross_scalar_scatter_inproc_fnptr (2.19 ms): 853.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_scatter_null_entry** at 2567.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 853.84x (fastest 2567.9 ns, slowest 2192585.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2184767ns | 2180244ns | 2162277ns | 2175276ns | 2210249ns | -0.20% |
| abi_cross_scalar_scatter_inproc_direct | 2189044ns | 2173470ns | 2160558ns | 2171581ns | 2229483ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2225930ns | 2195955ns | 2171140ns | 2190383ns | 2306646ns | +1.69% |
| abi_cross_scalar_scatter_null_entry | 4887ns | 4871ns | 4732ns | 4850ns | 5019ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2181306ns | 2159097ns | 2206508ns | -0.20% | 0.000 |
| abi_cross_scalar_scatter_inproc_direct | 2185692ns | 2157417ns | 2225865ns | base | 0.000 |
| abi_cross_scalar_scatter_inproc_fnptr | 2222684ns | 2168060ns | 2303354ns | +1.69% | 0.000 |
| abi_cross_scalar_scatter_null_entry | 2574ns | 2504ns | 2635ns | -99.88% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 65914.4 | 2181159.1 | 2181306.3 | 13 |
| abi_cross_scalar_scatter_inproc_direct | 7676.2 | 2186029.2 | 2185691.9 | n/a |
| abi_cross_scalar_scatter_inproc_fnptr | 7458.3 | 2210960.2 | 2222683.8 | 0 |
| abi_cross_scalar_scatter_null_entry | 27762.9 | 2650.3 | 2573.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_scatter_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_scatter_null_entry | 0.006 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 2184767ns | 2184767ns | -0.20% |
| abi_cross_scalar_scatter_inproc_direct | 2189044ns | 2189044ns | base |
| abi_cross_scalar_scatter_inproc_fnptr | 2225930ns | 2225930ns | +1.69% |
| abi_cross_scalar_scatter_null_entry | 4887ns | 4887ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_scatter_inproc_direct | 2170299ns | base | --- | [2160912, 2225865] | --- | --- | --- | --- |
| abi_cross_scalar_scatter_ffi_batched_scalar | 2176788ns | no significant difference | [-44182, +28244]ns | [2160624, 2206508] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_scatter_inproc_fnptr | 2192586ns | no significant difference | [-31746, +128931]ns | [2172111, 2303354] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_scatter_null_entry | 2568ns | -2167714.1ns (-99.9%) | [-2223296, -2158344]ns | [2518, 2635] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_scatter_inproc_direct | abi_cross_scalar_scatter_ffi_batched_scalar | abi_cross_scalar_scatter_inproc_fnptr | abi_cross_scalar_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2267290ns | -3.8% | -3.0% | -99.9% |
| 2 | 2169097ns | +0.2% | +0.8% | -99.9% |
| 3 | 2164408ns | -0.1% | +8.6% | -99.9% |
| 4 | 2184439ns | +2.2% | +3.3% | -99.9% |
| 5 | 2171501ns | +0.4% | +0.2% | -99.9% |
| 6 | 2157417ns | +0.1% | +0.5% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | -0.238 | moderate- |
| abi_cross_scalar_scatter_inproc_direct | -0.066 | ok |
| abi_cross_scalar_scatter_inproc_fnptr | 0.059 | ok |
| abi_cross_scalar_scatter_null_entry | 0.290 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_scatter_ffi_batched_scalar**: won 2/6, lost 3/6
- **abi_cross_scalar_scatter_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_scatter_ffi_batched_scalar | 6616813.5ns | 2181306.3ns | 303.3% | HIGH |
| abi_cross_scalar_scatter_inproc_direct | 6563918.8ns | 2185691.9ns | 300.3% | HIGH |
| abi_cross_scalar_scatter_inproc_fnptr | 6673169.6ns | 2222683.8ns | 300.2% | HIGH |
| abi_cross_scalar_scatter_null_entry | 118368.1ns | 2573.8ns | 4599.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_scatter_ffi_batched_scalar (n=6, range 2159097.1-2206507.7 ns)
  2159097.1 |####################
  2161467.6 |####################
  2163838.2 |
  2166208.7 |
  2168579.2 |
  2170949.8 |####################
  2173320.3 |
  2175690.8 |
  2178061.3 |
  2180431.9 |########################################
  2182802.4 |
  2185172.9 |
  2187543.5 |
  2189914.0 |
  2192284.5 |
  2194655.1 |
  2197025.6 |
  2199396.1 |
  2201766.6 |
  2204137.2 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_direct (n=6, range 2157416.7-2225864.6 ns)
  2157416.7 |########################################
  2160839.1 |
  2164261.5 |########################################
  2167683.9 |########################################
  2171106.3 |########################################
  2174528.7 |
  2177951.1 |
  2181373.5 |########################################
  2184795.9 |
  2188218.3 |
  2191640.7 |
  2195063.0 |
  2198485.4 |
  2201907.8 |
  2205330.2 |
  2208752.6 |
  2212175.0 |
  2215597.4 |
  2219019.8 |
  2222442.2 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_inproc_fnptr (n=6, range 2168060.0-2303354.4 ns)
  2168060.0 |########################################
  2174824.7 |########################################
  2181589.4 |########################################
  2188354.2 |
  2195118.9 |########################################
  2201883.6 |
  2208648.3 |
  2215413.0 |
  2222177.7 |
  2228942.5 |
  2235707.2 |
  2242471.9 |
  2249236.6 |
  2256001.3 |########################################
  2262766.0 |
  2269530.8 |
  2276295.5 |
  2283060.2 |
  2289824.9 |
  2296589.6 |
  (0 below, 1 above range)

abi_cross_scalar_scatter_null_entry (n=6, range 2504.2-2635.4 ns)
   2504.2 |####################
   2510.8 |
   2517.3 |
   2523.9 |
   2530.4 |########################################
   2537.0 |
   2543.6 |
   2550.1 |
   2556.7 |
   2563.2 |
   2569.8 |
   2576.4 |
   2582.9 |
   2589.5 |
   2596.0 |####################
   2602.6 |
   2609.2 |
   2615.7 |
   2622.3 |
   2628.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_scatter_ffi_batched_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_direct**: bridge=300.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_inproc_fnptr**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_scatter_null_entry**: bridge=4592.9% of algo (FFI overhead may distort results)
