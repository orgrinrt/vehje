# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 67663% faster than the next best (abi_zig_entry_scatter_zig_dispatch)

abi_zig_entry_scatter_zig_null (3.10 us) leads abi_zig_entry_scatter_zig_dispatch (2.10 ms) by 67663%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.10 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_runtime_w is an outlier: 1252.8x slower than the field

abi_zig_entry_scatter_zig_tail_runtime_w (3.88 ms) is 1252.8x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_scatter_zig_anchor shows alternating (throttle bounce) (autocorr -0.53)

abi_zig_entry_scatter_zig_anchor's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} (67663% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} with a 67663% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1252.8x the fastest

Fastest abi_zig_entry_scatter_zig_null (3.10 us) to slowest abi_zig_entry_scatter_zig_tail_runtime_w (3.88 ms): 1252.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 3096.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1252.78x (fastest 3096.9 ns, slowest 3879731.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2107108ns | 2107422ns | 2101035ns | 2107170ns | 2110050ns | +0.29% |
| abi_zig_entry_scatter_zig_dispatch | 2107141ns | 2101114ns | 2095186ns | 2099975ns | 2123869ns | +0.29% |
| abi_zig_entry_scatter_zig_null | 5388ns | 5405ns | 5298ns | 5379ns | 5445ns | -99.74% |
| abi_zig_entry_scatter_zig_per_w_set | 2103188ns | 2102298ns | 2100960ns | 2101992ns | 2106096ns | +0.10% |
| abi_zig_entry_scatter_zig_runtime_w | 2101004ns | 2103837ns | 2090723ns | 2100018ns | 2107622ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3878207ns | 3878557ns | 3857663ns | 3876237ns | 3891434ns | +84.59% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3901819ns | 3882448ns | 3852282ns | 3880863ns | 3958020ns | +85.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2104523ns | 2098417ns | 2107485ns | +0.29% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2104583ns | 2092742ns | 2121208ns | +0.30% | 0.000 |
| abi_zig_entry_scatter_zig_null | 3103ns | 3062ns | 3143ns | -99.85% | 0.082 |
| abi_zig_entry_scatter_zig_per_w_set | 2100557ns | 2098390ns | 2103472ns | +0.10% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2098361ns | 2088138ns | 2104900ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3875361ns | 3854907ns | 3888583ns | +84.69% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3898995ns | 3849410ns | 3955103ns | +85.81% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 176809.2 | 2104251.5 | 2104523.1 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 174704.6 | 2100883.0 | 2104582.6 | 0 |
| abi_zig_entry_scatter_zig_null | 153694.9 | 3159.3 | 3103.1 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 177901.0 | 2100129.2 | 2100556.6 | n/a |
| abi_zig_entry_scatter_zig_runtime_w | 175584.7 | 2099060.3 | 2098361.4 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 198662.7 | 3880128.1 | 3875361.5 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 197559.3 | 3922971.4 | 3898995.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.084 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_null | 0.083 | 98.9% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2107108ns | 2107108ns | +0.29% |
| abi_zig_entry_scatter_zig_dispatch | 2107141ns | 2107141ns | +0.29% |
| abi_zig_entry_scatter_zig_null | 5388ns | 5388ns | -99.74% |
| abi_zig_entry_scatter_zig_per_w_set | 2103188ns | 2103188ns | +0.10% |
| abi_zig_entry_scatter_zig_runtime_w | 2101004ns | 2101004ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3878207ns | 3878207ns | +84.59% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3901819ns | 3901819ns | +85.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2101239ns | base | --- | [2088945, 2104900] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2104801ns | no significant difference | [-3061, +18414]ns | [2101284, 2107485] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2098552ns | no significant difference | [-5319, +18339]ns | [2093988, 2121208] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_null | 3097ns | -2098155.6ns (-99.9%) | [-2101786, -2085833]ns | [3069, 3143] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2099671ns | no significant difference | [-6074, +14527]ns | [2098526, 2103472] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3875699ns | +1775480.5ns (+84.5%) | [+1762085, +1793435]ns | [3861802, 3888583] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3879732ns | +1783775.7ns (+84.9%) | [+1759204, +1858922]ns | [3862151, 3955103] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2104223ns | +0.0% | +0.3% | -99.9% | -0.2% | +84.2% | +90.3% |
| 2 | 2100318ns | +0.2% | -0.4% | -99.9% | -0.1% | +83.5% | +84.5% |
| 3 | 2089753ns | +1.0% | +0.3% | -99.8% | +0.6% | +85.1% | +85.6% |
| 4 | 2105578ns | -0.3% | +1.3% | -99.9% | -0.3% | +84.0% | +82.8% |
| 5 | 2102160ns | +0.1% | -0.1% | -99.9% | -0.1% | +84.8% | +84.6% |
| 6 | 2088138ns | +0.8% | +0.5% | -99.9% | +0.8% | +86.4% | +87.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | -0.526 | HIGH- (thermal bounce) |
| abi_zig_entry_scatter_zig_dispatch | -0.299 | moderate- |
| abi_zig_entry_scatter_zig_null | 0.228 | moderate+ |
| abi_zig_entry_scatter_zig_per_w_set | -0.144 | ok |
| abi_zig_entry_scatter_zig_runtime_w | -0.279 | moderate- |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.333 | moderate+ |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.018 | ok |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 1/6, lost 4/6
- **abi_zig_entry_scatter_zig_dispatch**: won 2/6, lost 4/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 2/6, lost 2/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6553375.2ns | 2104523.1ns | 311.4% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6543384.9ns | 2104582.6ns | 310.9% | HIGH |
| abi_zig_entry_scatter_zig_null | 302927.9ns | 3103.1ns | 9762.2% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6541986.5ns | 2100556.6ns | 311.4% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6540073.8ns | 2098361.4ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11901124.9ns | 3875361.5ns | 307.1% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 12096725.6ns | 3898995.3ns | 310.3% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2098417.1-2107484.8 ns)
  2098417.1 |########################################
  2098870.5 |
  2099323.9 |
  2099777.3 |
  2100230.6 |
  2100684.0 |
  2101137.4 |
  2101590.8 |
  2102044.2 |
  2102497.6 |
  2102951.0 |
  2103404.3 |
  2103857.7 |########################################
  2104311.1 |########################################
  2104764.5 |########################################
  2105217.9 |########################################
  2105671.3 |
  2106124.6 |
  2106578.0 |
  2107031.4 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2092742.1-2121207.9 ns)
  2092742.1 |########################################
  2094165.4 |########################################
  2095588.7 |
  2097012.0 |########################################
  2098435.3 |########################################
  2099858.5 |
  2101281.8 |
  2102705.1 |
  2104128.4 |
  2105551.7 |
  2106975.0 |
  2108398.3 |
  2109821.6 |########################################
  2111244.9 |
  2112668.2 |
  2114091.5 |
  2115514.7 |
  2116938.0 |
  2118361.3 |
  2119784.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 3062.1-3142.9 ns)
   3062.1 |########################################
   3066.1 |
   3070.2 |
   3074.2 |########################################
   3078.3 |
   3082.3 |
   3086.3 |########################################
   3090.4 |
   3094.4 |
   3098.5 |
   3102.5 |########################################
   3106.5 |
   3110.6 |
   3114.6 |
   3118.7 |
   3122.7 |
   3126.7 |
   3130.8 |
   3134.8 |########################################
   3138.9 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2098390.0-2103472.5 ns)
  2098390.0 |########################################
  2098644.1 |########################################
  2098898.2 |########################################
  2099152.4 |
  2099406.5 |
  2099660.6 |
  2099914.8 |
  2100168.9 |########################################
  2100423.0 |
  2100677.1 |
  2100931.2 |
  2101185.4 |
  2101439.5 |
  2101693.6 |
  2101947.8 |
  2102201.9 |########################################
  2102456.0 |
  2102710.1 |
  2102964.2 |
  2103218.4 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2088137.5-2104900.2 ns)
  2088137.5 |########################################
  2088975.6 |########################################
  2089813.8 |
  2090651.9 |
  2091490.0 |
  2092328.2 |
  2093166.3 |
  2094004.4 |
  2094842.6 |
  2095680.7 |
  2096518.9 |
  2097357.0 |
  2098195.1 |
  2099033.3 |
  2099871.4 |########################################
  2100709.5 |
  2101547.7 |########################################
  2102385.8 |
  2103223.9 |
  2104062.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3854907.1-3888583.4 ns)
  3854907.1 |####################
  3856590.9 |
  3858274.7 |
  3859958.5 |
  3861642.4 |
  3863326.2 |
  3865010.0 |
  3866693.8 |
  3868377.6 |####################
  3870061.4 |
  3871745.2 |
  3873429.0 |
  3875112.9 |########################################
  3876796.7 |
  3878480.5 |
  3880164.3 |
  3881848.1 |
  3883531.9 |####################
  3885215.7 |
  3886899.5 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3849410.0-3955102.7 ns)
  3849410.0 |####################
  3854694.6 |
  3859979.3 |
  3865263.9 |
  3870548.5 |####################
  3875833.2 |########################################
  3881117.8 |
  3886402.4 |
  3891687.1 |
  3896971.7 |
  3902256.4 |####################
  3907541.0 |
  3912825.6 |
  3918110.3 |
  3923394.9 |
  3928679.5 |
  3933964.2 |
  3939248.8 |
  3944533.4 |
  3949818.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=9765.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=306.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=306.7% of algo (FFI overhead may distort results)
