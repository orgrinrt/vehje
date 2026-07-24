# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 76290% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (2.79 us) leads abi_lifecycle_scatter_held_handle (2.14 ms) by 76290%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 771.7x slower than the field

abi_lifecycle_scatter_fresh_per_batch (2.16 ms) is 771.7x the fastest (2.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (76290% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 76290% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 771.7x the fastest

Fastest abi_lifecycle_scatter_null_entry (2.79 us) to slowest abi_lifecycle_scatter_fresh_per_batch (2.16 ms): 771.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 2795.0 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 771.68x (fastest 2795.0 ns, slowest 2156856.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2187148ns | 2159443ns | 2149198ns | 2158716ns | 2248772ns | -0.79% |
| abi_lifecycle_scatter_fresh_per_column | 2213944ns | 2149965ns | 2143105ns | 2148563ns | 2347436ns | +0.42% |
| abi_lifecycle_scatter_held_handle | 2204603ns | 2137631ns | 2128707ns | 2135506ns | 2346198ns | base |
| abi_lifecycle_scatter_null_entry | 5152ns | 5105ns | 4919ns | 5102ns | 5343ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2184415ns | 2146569ns | 2245746ns | -0.79% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2211267ns | 2140490ns | 2344584ns | +0.43% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2201833ns | 2126188ns | 2342940ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 2809ns | 2693ns | 2894ns | -99.87% | 0.046 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 42780.7 | 2175311.7 | 2184415.2 | n/a |
| abi_lifecycle_scatter_fresh_per_column | 42748.8 | 2178764.4 | 2211267.0 | 0 |
| abi_lifecycle_scatter_held_handle | 44158.8 | 2212370.6 | 2201833.3 | n/a |
| abi_lifecycle_scatter_null_entry | 28277.0 | 2819.5 | 2809.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.1% |
| abi_lifecycle_scatter_null_entry | 0.046 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 2187148ns | 2187148ns | -0.79% |
| abi_lifecycle_scatter_fresh_per_column | 2213944ns | 2213944ns | +0.42% |
| abi_lifecycle_scatter_held_handle | 2204603ns | 2204603ns | base |
| abi_lifecycle_scatter_null_entry | 5152ns | 5152ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2135089ns | base | --- | [2127471, 2342940] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 2156856ns | no significant difference | [-170081, +90603]ns | [2150643, 2245746] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2147339ns | no significant difference | [-178241, +190177]ns | [2141879, 2344584] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_scatter_null_entry | 2795ns | -2132330.8ns (-99.9%) | [-2340059, -2124682]ns | [2739, 2894] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2128754ns | +1.4% | +0.8% | -99.9% |
| 2 | 2508887ns | -14.4% | -14.7% | -99.9% |
| 3 | 2176993ns | +7.0% | +16.7% | -99.9% |
| 4 | 2132562ns | +1.0% | +0.7% | -99.9% |
| 5 | 2137616ns | +1.2% | +0.6% | -99.9% |
| 6 | 2126188ns | +1.4% | +0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | -0.299 | moderate- |
| abi_lifecycle_scatter_fresh_per_column | -0.242 | moderate- |
| abi_lifecycle_scatter_held_handle | -0.166 | ok |
| abi_lifecycle_scatter_null_entry | -0.087 | ok |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 1/6, lost 5/6
- **abi_lifecycle_scatter_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 6598604.3ns | 2184415.2ns | 302.1% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6579601.8ns | 2211267.0ns | 297.5% | HIGH |
| abi_lifecycle_scatter_held_handle | 6579882.6ns | 2201833.3ns | 298.8% | HIGH |
| abi_lifecycle_scatter_null_entry | 121584.7ns | 2809.2ns | 4328.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 2146568.8-2245746.0 ns)
  2146568.8 |####################
  2151527.7 |########################################
  2156486.5 |####################
  2161445.4 |####################
  2166404.2 |
  2171363.1 |
  2176322.0 |
  2181280.8 |
  2186239.7 |
  2191198.5 |
  2196157.4 |
  2201116.3 |
  2206075.1 |
  2211034.0 |
  2215992.8 |
  2220951.7 |
  2225910.6 |
  2230869.4 |
  2235828.3 |
  2240787.1 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2140490.0-2344583.5 ns)
  2140490.0 |########################################
  2150694.7 |
  2160899.4 |
  2171104.0 |
  2181308.7 |
  2191513.4 |
  2201718.0 |
  2211922.7 |
  2222127.4 |
  2232332.1 |
  2242536.8 |
  2252741.4 |
  2262946.1 |
  2273150.8 |
  2283355.5 |
  2293560.1 |
  2303764.8 |
  2313969.5 |
  2324174.1 |
  2334378.8 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2126187.5-2342939.8 ns)
  2126187.5 |########################################
  2137025.1 |#############
  2147862.7 |
  2158700.3 |
  2169538.0 |#############
  2180375.6 |
  2191213.2 |
  2202050.8 |
  2212888.4 |
  2223726.0 |
  2234563.6 |
  2245401.3 |
  2256238.9 |
  2267076.5 |
  2277914.1 |
  2288751.7 |
  2299589.3 |
  2310427.0 |
  2321264.6 |
  2332102.2 |
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 2692.9-2893.9 ns)
   2692.9 |####################
   2703.0 |
   2713.0 |
   2723.1 |
   2733.1 |
   2743.2 |
   2753.2 |
   2763.3 |
   2773.3 |
   2783.4 |########################################
   2793.4 |####################
   2803.5 |
   2813.5 |
   2823.6 |####################
   2833.6 |
   2843.7 |
   2853.7 |
   2863.8 |
   2873.8 |
   2883.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=4287.0% of algo (FFI overhead may distort results)
