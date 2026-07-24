# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 53151% faster than the next best (abi_zig_entry_real_zig_dispatch)

abi_zig_entry_real_zig_null (3.88 us) leads abi_zig_entry_real_zig_dispatch (2.07 ms) by 53151%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.07 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 995.8x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.87 ms) is 995.8x the fastest (3.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_tail_dispatch shows warm-up / thermal drift (autocorr +0.51)

abi_zig_entry_real_zig_tail_dispatch's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (53151% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 53151% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 995.8x the fastest

Fastest abi_zig_entry_real_zig_null (3.88 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.87 ms): 995.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 3881.7 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 995.76x (fastest 3881.7 ns, slowest 3865194.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2077241ns | 2076396ns | 2068801ns | 2074890ns | 2084987ns | -0.01% |
| abi_zig_entry_real_zig_dispatch | 2070737ns | 2069641ns | 2066870ns | 2069133ns | 2075076ns | -0.32% |
| abi_zig_entry_real_zig_null | 6169ns | 6179ns | 6077ns | 6148ns | 6248ns | -99.70% |
| abi_zig_entry_real_zig_per_w_set | 2072048ns | 2071377ns | 2069475ns | 2070746ns | 2075287ns | -0.26% |
| abi_zig_entry_real_zig_runtime_w | 2077453ns | 2077898ns | 2075187ns | 2077542ns | 2078452ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3105668ns | 3105638ns | 3101328ns | 3104224ns | 3110003ns | +49.49% |
| abi_zig_entry_real_zig_tail_runtime_w | 3868350ns | 3868241ns | 3853712ns | 3866958ns | 3877756ns | +86.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2074563ns | 2066085ns | 2082230ns | -0.01% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2068100ns | 2064348ns | 2072474ns | -0.32% | 0.000 |
| abi_zig_entry_real_zig_null | 3886ns | 3848ns | 3926ns | -99.81% | 0.002 |
| abi_zig_entry_real_zig_per_w_set | 2069401ns | 2066884ns | 2072551ns | -0.26% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2074720ns | 2072426ns | 2075683ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3102931ns | 3098651ns | 3107145ns | +49.56% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3865460ns | 3850953ns | 3874976ns | +86.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 178502.7 | 2074178.7 | 2074562.7 | n/a |
| abi_zig_entry_real_zig_dispatch | 180051.1 | 2069232.5 | 2068100.4 | n/a |
| abi_zig_entry_real_zig_null | 154739.0 | 4099.2 | 3885.9 | n/a |
| abi_zig_entry_real_zig_per_w_set | 182218.2 | 2068152.2 | 2069401.2 | n/a |
| abi_zig_entry_real_zig_runtime_w | 183429.4 | 2074815.0 | 2074719.8 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 186862.2 | 3100850.4 | 3102930.9 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 201668.4 | 3870335.1 | 3865460.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_real_zig_null | 0.002 | 99.1% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2077241ns | 2077241ns | -0.01% |
| abi_zig_entry_real_zig_dispatch | 2070737ns | 2070737ns | -0.32% |
| abi_zig_entry_real_zig_null | 6169ns | 6169ns | -99.70% |
| abi_zig_entry_real_zig_per_w_set | 2072048ns | 2072048ns | -0.26% |
| abi_zig_entry_real_zig_runtime_w | 2077453ns | 2077453ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3105668ns | 3105668ns | +49.49% |
| abi_zig_entry_real_zig_tail_runtime_w | 3868350ns | 3868350ns | +86.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2075157ns | base | --- | [2073320, 2075683] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2073779ns | no significant difference | [-6908, +6612]ns | [2067680, 2082230] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_dispatch | 2067018ns | -8204.8ns (-0.4%) | [-10238, -1415]ns | [2064810, 2072474] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_null | 3882ns | -2071252.9ns (-99.8%) | [-2071819, -2069430]ns | [3850, 3926] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2068749ns | -6407.7ns (-0.3%) | [-7945, -1603]ns | [2066903, 2072551] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3102959ns | +1028110.6ns (+49.5%) | [+1024799, +1031724]ns | [3098688, 3107145] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3865194ns | +1790345.6ns (+86.3%) | [+1782517, +1799359]ns | [3856210, 3874976] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2075352ns | +0.4% | -0.1% | -99.8% | -0.3% | +49.3% | +87.1% |
| 2 | 2072426ns | +0.2% | -0.0% | -99.8% | +0.1% | +49.5% | +86.3% |
| 3 | 2074213ns | -0.2% | -0.5% | -99.8% | -0.4% | +49.6% | +86.3% |
| 4 | 2074961ns | -0.4% | -0.4% | -99.8% | -0.3% | +49.8% | +85.6% |
| 5 | 2075882ns | +0.2% | -0.5% | -99.8% | -0.3% | +49.6% | +86.3% |
| 6 | 2075483ns | -0.2% | -0.4% | -99.8% | -0.4% | +49.5% | +86.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.038 | ok |
| abi_zig_entry_real_zig_dispatch | 0.251 | moderate+ |
| abi_zig_entry_real_zig_null | -0.174 | ok |
| abi_zig_entry_real_zig_per_w_set | -0.330 | moderate- |
| abi_zig_entry_real_zig_runtime_w | 0.096 | ok |
| abi_zig_entry_real_zig_tail_dispatch | 0.512 | HIGH+ (drift/warm-up) |
| abi_zig_entry_real_zig_tail_runtime_w | -0.130 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 3/6, lost 3/6
- **abi_zig_entry_real_zig_dispatch**: won 5/6, lost 0/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 5/6, lost 1/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6468282.6ns | 2074562.7ns | 311.8% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6453510.4ns | 2068100.4ns | 312.1% | HIGH |
| abi_zig_entry_real_zig_null | 304388.6ns | 3885.9ns | 7833.2% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6457098.8ns | 2069401.2ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6479374.9ns | 2074719.8ns | 312.3% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 9566759.0ns | 3102930.9ns | 308.3% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11881036.1ns | 3865460.1ns | 307.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2066085.4-2082229.6 ns)
  2066085.4 |########################################
  2066892.6 |
  2067699.8 |
  2068507.0 |########################################
  2069314.2 |
  2070121.4 |
  2070928.7 |########################################
  2071735.9 |
  2072543.1 |
  2073350.3 |
  2074157.5 |
  2074964.7 |
  2075771.9 |########################################
  2076579.1 |
  2077386.3 |
  2078193.6 |
  2079000.8 |
  2079808.0 |########################################
  2080615.2 |
  2081422.4 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2064347.5-2072473.9 ns)
  2064347.5 |########################################
  2064753.8 |
  2065160.1 |########################################
  2065566.5 |
  2065972.8 |########################################
  2066379.1 |
  2066785.4 |
  2067191.8 |
  2067598.1 |########################################
  2068004.4 |
  2068410.7 |
  2068817.0 |
  2069223.4 |
  2069629.7 |
  2070036.0 |
  2070442.3 |
  2070848.7 |
  2071255.0 |
  2071661.3 |
  2072067.6 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 3848.3-3925.6 ns)
   3848.3 |########################################
   3852.2 |########################################
   3856.0 |
   3859.9 |
   3863.8 |
   3867.6 |
   3871.5 |
   3875.4 |########################################
   3879.2 |
   3883.1 |
   3886.9 |########################################
   3890.8 |
   3894.7 |
   3898.5 |
   3902.4 |
   3906.3 |
   3910.1 |
   3914.0 |
   3917.9 |########################################
   3921.7 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2066884.2-2072551.2 ns)
  2066884.2 |########################################
  2067167.6 |
  2067450.9 |
  2067734.3 |
  2068017.6 |####################
  2068301.0 |
  2068584.3 |
  2068867.7 |
  2069151.0 |####################
  2069434.4 |
  2069717.7 |
  2070001.1 |
  2070284.4 |####################
  2070567.8 |
  2070851.1 |
  2071134.5 |
  2071417.8 |
  2071701.2 |
  2071984.5 |
  2072267.9 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2072425.8-2075682.9 ns)
  2072425.8 |########################################
  2072588.7 |
  2072751.5 |
  2072914.4 |
  2073077.2 |
  2073240.1 |
  2073402.9 |
  2073565.8 |
  2073728.6 |
  2073891.5 |
  2074054.4 |########################################
  2074217.2 |
  2074380.1 |
  2074542.9 |
  2074705.8 |
  2074868.6 |########################################
  2075031.5 |
  2075194.3 |########################################
  2075357.2 |########################################
  2075520.0 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3098650.8-3107145.4 ns)
  3098650.8 |########################################
  3099075.5 |
  3099500.3 |
  3099925.0 |
  3100349.7 |
  3100774.5 |
  3101199.2 |
  3101623.9 |
  3102048.6 |####################
  3102473.4 |
  3102898.1 |
  3103322.8 |####################
  3103747.6 |
  3104172.3 |
  3104597.0 |
  3105021.8 |
  3105446.5 |
  3105871.2 |
  3106295.9 |####################
  3106720.7 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3850953.3-3874976.2 ns)
  3850953.3 |########################################
  3852154.4 |
  3853355.6 |
  3854556.7 |
  3855757.9 |
  3856959.0 |
  3858160.2 |
  3859361.3 |
  3860562.5 |########################################
  3861763.6 |
  3862964.8 |########################################
  3864165.9 |
  3865367.1 |
  3866568.2 |########################################
  3867769.4 |########################################
  3868970.5 |
  3870171.7 |
  3871372.8 |
  3872574.0 |
  3873775.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=7852.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=308.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.3% of algo (FFI overhead may distort results)
