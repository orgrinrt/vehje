# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_scatter_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_scatter_runtime_w has the worst median (2.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_scatter_null_entry at 2.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_scatter_null_entry dominates: 78694% faster than the next best (abi_entry_form_scatter_dispatch_table)

abi_entry_form_scatter_null_entry (2.75 us) leads abi_entry_form_scatter_dispatch_table (2.17 ms) by 78694%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.18 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_runtime_w is an outlier: 791.7x slower than the field

abi_entry_form_scatter_runtime_w (2.18 ms) is 791.7x the fastest (2.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w} (78694% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w} with a 78694% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 791.7x the fastest

Fastest abi_entry_form_scatter_null_entry (2.75 us) to slowest abi_entry_form_scatter_runtime_w (2.18 ms): 791.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 2753.6 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 791.68x (fastest 2753.6 ns, slowest 2179940.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2174600ns | 2173068ns | 2166010ns | 2172011ns | 2182778ns | -0.44% |
| abi_entry_form_scatter_null_entry | 5086ns | 5087ns | 4926ns | 5063ns | 5201ns | -99.77% |
| abi_entry_form_scatter_per_w_set | 2200292ns | 2181458ns | 2167035ns | 2178404ns | 2249751ns | +0.74% |
| abi_entry_form_scatter_runtime_w | 2184220ns | 2183558ns | 2176778ns | 2181314ns | 2192300ns | base |
| abi_entry_form_scatter_scalar_anchor | 2175853ns | 2174040ns | 2167140ns | 2173362ns | 2183945ns | -0.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2171144ns | 2162792ns | 2179095ns | -0.44% | 0.000 |
| abi_entry_form_scatter_null_entry | 2764ns | 2667ns | 2852ns | -99.87% | 0.046 |
| abi_entry_form_scatter_per_w_set | 2196758ns | 2163899ns | 2245825ns | +0.74% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2180702ns | 2173462ns | 2188698ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2172589ns | 2164089ns | 2180496ns | -0.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 64534.9 | 2173985.8 | 2171143.8 | n/a |
| abi_entry_form_scatter_null_entry | 28449.9 | 2800.3 | 2764.1 | n/a |
| abi_entry_form_scatter_per_w_set | 75205.5 | 2263280.4 | 2196757.6 | n/a |
| abi_entry_form_scatter_runtime_w | 66066.9 | 2179050.1 | 2180701.8 | n/a |
| abi_entry_form_scatter_scalar_anchor | 62077.7 | 2176450.8 | 2172589.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_scatter_null_entry | 0.046 | 96.8% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.1% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.1% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2174600ns | 2174600ns | -0.44% |
| abi_entry_form_scatter_null_entry | 5086ns | 5086ns | -99.77% |
| abi_entry_form_scatter_per_w_set | 2200292ns | 2200292ns | +0.74% |
| abi_entry_form_scatter_runtime_w | 2184220ns | 2184220ns | base |
| abi_entry_form_scatter_scalar_anchor | 2175853ns | 2175853ns | -0.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2179941ns | base | --- | [2173466, 2188698] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2169626ns | no significant difference | [-21970, +856]ns | [2164710, 2179095] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_scatter_null_entry | 2754ns | -2177196.5ns (-99.9%) | [-2185945, -2170672]ns | [2686, 2852] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2178074ns | no significant difference | [-14348, +57908]ns | [2166374, 2245825] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2170790ns | -8681.5ns (-0.4%) | [-12998, -2658]ns | [2166482, 2180496] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2176865ns | -0.4% | -99.9% | -0.6% | -0.4% |
| 2 | 2192818ns | -1.0% | -99.9% | +2.7% | -0.5% |
| 3 | 2183017ns | -0.2% | -99.9% | +2.6% | -0.1% |
| 4 | 2173470ns | -0.3% | -99.9% | +0.0% | -0.1% |
| 5 | 2184579ns | -1.0% | -99.9% | -0.7% | -0.6% |
| 6 | 2173462ns | +0.3% | -99.9% | +0.4% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.309 | moderate- |
| abi_entry_form_scatter_null_entry | 0.217 | moderate+ |
| abi_entry_form_scatter_per_w_set | 0.083 | ok |
| abi_entry_form_scatter_runtime_w | -0.318 | moderate- |
| abi_entry_form_scatter_scalar_anchor | 0.190 | ok |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 5/6, lost 1/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_scatter_scalar_anchor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6587888.5ns | 2171143.8ns | 303.4% | HIGH |
| abi_entry_form_scatter_null_entry | 121036.4ns | 2764.1ns | 4378.9% | HIGH |
| abi_entry_form_scatter_per_w_set | 6774149.4ns | 2196757.6ns | 308.4% | HIGH |
| abi_entry_form_scatter_runtime_w | 6608706.9ns | 2180701.8ns | 303.1% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6589682.1ns | 2172589.4ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2162791.7-2179095.0 ns)
  2162791.7 |########################################
  2163606.9 |
  2164422.0 |
  2165237.2 |
  2166052.4 |########################################
  2166867.5 |
  2167682.7 |
  2168497.9 |########################################
  2169313.0 |
  2170128.2 |########################################
  2170943.4 |
  2171758.5 |
  2172573.7 |
  2173388.8 |
  2174204.0 |
  2175019.2 |
  2175834.3 |
  2176649.5 |
  2177464.7 |
  2178279.8 |########################################
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 2666.7-2852.5 ns)
   2666.7 |####################
   2676.0 |
   2685.3 |
   2694.6 |
   2703.9 |####################
   2713.1 |
   2722.4 |
   2731.7 |
   2741.0 |
   2750.3 |########################################
   2759.6 |
   2768.9 |
   2778.2 |
   2787.5 |
   2796.8 |
   2806.1 |
   2815.3 |####################
   2824.6 |
   2833.9 |
   2843.2 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2163899.2-2245825.2 ns)
  2163899.2 |########################################
  2167995.5 |########################################
  2172091.8 |########################################
  2176188.1 |
  2180284.4 |########################################
  2184380.7 |
  2188477.0 |
  2192573.3 |
  2196669.6 |
  2200765.9 |
  2204862.2 |
  2208958.5 |
  2213054.8 |
  2217151.1 |
  2221247.4 |
  2225343.7 |
  2229440.0 |
  2233536.3 |
  2237632.6 |########################################
  2241728.9 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2173462.1-2188698.1 ns)
  2173462.1 |########################################
  2174223.9 |
  2174985.7 |
  2175747.5 |
  2176509.3 |####################
  2177271.1 |
  2178032.9 |
  2178794.7 |
  2179556.5 |
  2180318.3 |
  2181080.1 |
  2181841.9 |
  2182603.7 |####################
  2183365.5 |
  2184127.3 |####################
  2184889.1 |
  2185650.9 |
  2186412.7 |
  2187174.5 |
  2187936.3 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2164088.8-2180496.2 ns)
  2164088.8 |########################################
  2164909.2 |
  2165729.5 |
  2166549.9 |
  2167370.3 |
  2168190.7 |########################################
  2169011.0 |
  2169831.4 |########################################
  2170651.8 |########################################
  2171472.2 |
  2172292.5 |
  2173112.9 |
  2173933.3 |
  2174753.6 |
  2175574.0 |
  2176394.4 |
  2177214.8 |
  2178035.1 |
  2178855.5 |
  2179675.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=4394.0% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)
