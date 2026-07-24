# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_real_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_real_runtime_w has the worst median (2.20 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_real_null_entry at 2.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_real_null_entry dominates: 79575% faster than the next best (abi_entry_form_real_per_w_set)

abi_entry_form_real_null_entry (2.73 us) leads abi_entry_form_real_per_w_set (2.18 ms) by 79575%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.20 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_runtime_w is an outlier: 806.5x slower than the field

abi_entry_form_real_runtime_w (2.20 ms) is 806.5x the fastest (2.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_runtime_w} (79575% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_runtime_w} with a 79575% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 806.5x the fastest

Fastest abi_entry_form_real_null_entry (2.73 us) to slowest abi_entry_form_real_runtime_w (2.20 ms): 806.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 2733.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 806.52x (fastest 2733.1 ns, slowest 2204288.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2205123ns | 2193076ns | 2180058ns | 2191296ns | 2238397ns | -4.12% |
| abi_entry_form_real_null_entry | 5073ns | 5047ns | 4994ns | 5030ns | 5177ns | -99.78% |
| abi_entry_form_real_per_w_set | 2202676ns | 2180992ns | 2173438ns | 2179301ns | 2252358ns | -4.23% |
| abi_entry_form_real_runtime_w | 2299910ns | 2208489ns | 2185265ns | 2201032ns | 2505551ns | base |
| abi_entry_form_real_scalar_anchor | 2236868ns | 2193958ns | 2186541ns | 2191913ns | 2329463ns | -2.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2201319ns | 2176740ns | 2234251ns | -4.11% | 0.000 |
| abi_entry_form_real_null_entry | 2746ns | 2714ns | 2790ns | -99.88% | 0.047 |
| abi_entry_form_real_per_w_set | 2199142ns | 2170210ns | 2248437ns | -4.21% | 0.000 |
| abi_entry_form_real_runtime_w | 2295712ns | 2181980ns | 2500534ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2233120ns | 2183380ns | 2325241ns | -2.73% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 77347.1 | 2207483.8 | 2201319.3 | n/a |
| abi_entry_form_real_null_entry | 29873.0 | 2783.9 | 2746.0 | n/a |
| abi_entry_form_real_per_w_set | 70247.6 | 2206671.8 | 2199141.8 | n/a |
| abi_entry_form_real_runtime_w | 84752.6 | 2305253.3 | 2295711.8 | n/a |
| abi_entry_form_real_scalar_anchor | 77747.4 | 2242710.7 | 2233119.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.047 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.047 | 99.3% |
| abi_entry_form_real_per_w_set | 0.000 | 0.1% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2205123ns | 2205123ns | -4.12% |
| abi_entry_form_real_null_entry | 5073ns | 5073ns | -99.78% |
| abi_entry_form_real_per_w_set | 2202676ns | 2202676ns | -4.23% |
| abi_entry_form_real_runtime_w | 2299910ns | 2299910ns | base |
| abi_entry_form_real_scalar_anchor | 2236868ns | 2236868ns | -2.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2204288ns | base | --- | [2182313, 2500534] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2189293ns | no significant difference | [-266913, +7610]ns | [2180414, 2234251] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_real_null_entry | 2733ns | -2201539.1ns (-99.9%) | [-2497774, -2179585]ns | [2715, 2790] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2177596ns | -26692.2ns (-1.2%) | [-261796, -1222]ns | [2171392, 2248437] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| abi_entry_form_real_scalar_anchor | 2190352ns | no significant difference | [-203816, +21867]ns | [2183766, 2325241] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2182647ns | +0.4% | -99.9% | +0.3% | +0.3% |
| 2 | 2206371ns | -1.0% | -99.9% | -1.4% | -0.6% |
| 3 | 2244161ns | -2.4% | -99.9% | -3.3% | +1.7% |
| 4 | 2756906ns | -17.4% | -99.9% | -16.3% | -14.1% |
| 5 | 2181980ns | +0.3% | -99.9% | -0.4% | +0.1% |
| 6 | 2202206ns | -1.2% | -99.9% | -1.1% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.161 | ok |
| abi_entry_form_real_null_entry | 0.024 | ok |
| abi_entry_form_real_per_w_set | -0.322 | moderate- |
| abi_entry_form_real_runtime_w | -0.197 | ok |
| abi_entry_form_real_scalar_anchor | 0.079 | ok |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 5/6, lost 1/6
- **abi_entry_form_real_scalar_anchor**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6699245.8ns | 2201319.3ns | 304.3% | HIGH |
| abi_entry_form_real_null_entry | 122146.9ns | 2746.0ns | 4448.2% | HIGH |
| abi_entry_form_real_per_w_set | 6691255.4ns | 2199141.8ns | 304.3% | HIGH |
| abi_entry_form_real_runtime_w | 6992374.6ns | 2295711.8ns | 304.6% | HIGH |
| abi_entry_form_real_scalar_anchor | 6898602.9ns | 2233119.8ns | 308.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2176740.4-2234251.0 ns)
  2176740.4 |#############
  2179615.9 |
  2182491.5 |#############
  2185367.0 |
  2188242.5 |########################################
  2191118.1 |
  2193993.6 |
  2196869.1 |
  2199744.7 |
  2202620.2 |
  2205495.7 |
  2208371.3 |
  2211246.8 |
  2214122.3 |
  2216997.9 |
  2219873.4 |
  2222748.9 |
  2225624.5 |
  2228500.0 |
  2231375.5 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 2714.2-2789.8 ns)
   2714.2 |########################################
   2718.0 |
   2721.8 |####################
   2725.5 |
   2729.3 |
   2733.1 |
   2736.9 |
   2740.7 |####################
   2744.4 |
   2748.2 |
   2752.0 |
   2755.8 |
   2759.6 |
   2763.3 |
   2767.1 |
   2770.9 |
   2774.7 |
   2778.5 |
   2782.2 |####################
   2786.0 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2170210.4-2248436.9 ns)
  2170210.4 |########################################
  2174121.7 |####################
  2178033.0 |####################
  2181944.4 |
  2185855.7 |####################
  2189767.0 |
  2193678.4 |
  2197589.7 |
  2201501.0 |
  2205412.3 |
  2209323.7 |
  2213235.0 |
  2217146.3 |
  2221057.6 |
  2224969.0 |
  2228880.3 |
  2232791.6 |
  2236702.9 |
  2240614.3 |
  2244525.6 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2181979.6-2500533.7 ns)
  2181979.6 |########################################
  2197907.3 |########################################
  2213835.0 |
  2229762.7 |####################
  2245690.4 |
  2261618.1 |
  2277545.8 |
  2293473.5 |
  2309401.2 |
  2325328.9 |
  2341256.7 |
  2357184.4 |
  2373112.1 |
  2389039.8 |
  2404967.5 |
  2420895.2 |
  2436822.9 |
  2452750.6 |
  2468678.3 |
  2484606.0 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2183380.0-2325241.2 ns)
  2183380.0 |########################################
  2190473.1 |#############
  2197566.1 |
  2204659.2 |
  2211752.2 |
  2218845.3 |
  2225938.4 |
  2233031.4 |
  2240124.5 |
  2247217.6 |
  2254310.6 |
  2261403.7 |
  2268496.8 |
  2275589.8 |#############
  2282682.9 |
  2289775.9 |
  2296869.0 |
  2303962.1 |
  2311055.1 |
  2318148.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=4479.1% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=304.3% of algo (FFI overhead may distort results)
