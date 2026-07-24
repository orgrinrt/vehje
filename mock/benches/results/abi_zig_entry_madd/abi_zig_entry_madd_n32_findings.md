# abi_zig_entry (madd)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_madd_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_madd_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_madd_zig_null dominates: 117924% faster than the next best (abi_zig_entry_madd_zig_runtime_w)

abi_zig_entry_madd_zig_null (2.29 us) leads abi_zig_entry_madd_zig_runtime_w (2.71 ms) by 117924%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_madd_zig_null beats baseline by 100% (significant)

abi_zig_entry_madd_zig_null is -2.70 ms (100%) faster than baseline abi_zig_entry_madd_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_madd_zig_tail_runtime_w is an outlier: 1395.4x slower than the field

abi_zig_entry_madd_zig_tail_runtime_w (3.20 ms) is 1395.4x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_madd_zig_anchor shows alternating (throttle bounce) (autocorr -0.80)

abi_zig_entry_madd_zig_anchor's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_madd_zig_null} vs {abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} (117924% apart)

The field splits into a fast tier {abi_zig_entry_madd_zig_null} and a slow tier {abi_zig_entry_madd_zig_runtime_w, abi_zig_entry_madd_zig_dispatch, abi_zig_entry_madd_zig_per_w_set, abi_zig_entry_madd_zig_anchor, abi_zig_entry_madd_zig_tail_dispatch, abi_zig_entry_madd_zig_tail_runtime_w} with a 117924% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1395.4x the fastest

Fastest abi_zig_entry_madd_zig_null (2.29 us) to slowest abi_zig_entry_madd_zig_tail_runtime_w (3.20 ms): 1395.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_madd_zig_null** at 2293.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1395.39x (fastest 2293.8 ns, slowest 3200681.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2741850ns | 2753618ns | 2702995ns | 2738425ns | 2766417ns | +1.10% |
| abi_zig_entry_madd_zig_dispatch | 2716089ns | 2717060ns | 2709091ns | 2716376ns | 2719157ns | +0.15% |
| abi_zig_entry_madd_zig_null | 4667ns | 4617ns | 4584ns | 4616ns | 4785ns | -99.83% |
| abi_zig_entry_madd_zig_per_w_set | 2722312ns | 2722806ns | 2713637ns | 2721339ns | 2728110ns | +0.38% |
| abi_zig_entry_madd_zig_runtime_w | 2712121ns | 2710387ns | 2701101ns | 2708849ns | 2722539ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3159356ns | 3180556ns | 3073742ns | 3151656ns | 3213713ns | +16.49% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3207184ns | 3204190ns | 3177858ns | 3200778ns | 3231457ns | +18.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2738247ns | 2699819ns | 2762780ns | +1.08% | 0.000 |
| abi_zig_entry_madd_zig_dispatch | 2712853ns | 2705970ns | 2715896ns | +0.14% | 0.000 |
| abi_zig_entry_madd_zig_null | 2308ns | 2277ns | 2347ns | -99.91% | 0.014 |
| abi_zig_entry_madd_zig_per_w_set | 2719003ns | 2710457ns | 2724706ns | +0.37% | 0.000 |
| abi_zig_entry_madd_zig_runtime_w | 2708953ns | 2698432ns | 2719189ns | base | 0.000 |
| abi_zig_entry_madd_zig_tail_dispatch | 3155812ns | 3070629ns | 3210090ns | +16.50% | 0.000 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3203750ns | 3175017ns | 3227845ns | +18.27% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 249234.6 | 2740781.9 | 2738247.4 | n/a |
| abi_zig_entry_madd_zig_dispatch | 217741.1 | 2715368.3 | 2712853.1 | n/a |
| abi_zig_entry_madd_zig_null | 162323.0 | 2628.8 | 2307.9 | n/a |
| abi_zig_entry_madd_zig_per_w_set | 237402.8 | 2721224.6 | 2719002.7 | n/a |
| abi_zig_entry_madd_zig_runtime_w | 215365.7 | 2709884.2 | 2708952.6 | n/a |
| abi_zig_entry_madd_zig_tail_dispatch | 247327.8 | 3144746.7 | 3155812.4 | n/a |
| abi_zig_entry_madd_zig_tail_runtime_w | 232132.5 | 3202588.6 | 3203749.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_zig_entry_madd_zig_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_null | 0.014 | 99.3% |
| abi_zig_entry_madd_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 2741850ns | 2741850ns | +1.10% |
| abi_zig_entry_madd_zig_dispatch | 2716089ns | 2716089ns | +0.15% |
| abi_zig_entry_madd_zig_null | 4667ns | 4667ns | -99.83% |
| abi_zig_entry_madd_zig_per_w_set | 2722312ns | 2722312ns | +0.38% |
| abi_zig_entry_madd_zig_runtime_w | 2712121ns | 2712121ns | base |
| abi_zig_entry_madd_zig_tail_dispatch | 3159356ns | 3159356ns | +16.49% |
| abi_zig_entry_madd_zig_tail_runtime_w | 3207184ns | 3207184ns | +18.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_madd_zig_runtime_w | 2707187ns | base | --- | [2700482, 2719189] | --- | --- | --- | --- |
| abi_zig_entry_madd_zig_anchor | 2749704ns | no significant difference | [-13482, +61041]ns | [2702258, 2762780] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_madd_zig_dispatch | 2713798ns | no significant difference | [-5593, +13316]ns | [2708865, 2715896] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_madd_zig_null | 2294ns | -2704850.0ns (-99.9%) | [-2716894, -2698190]ns | [2283, 2347] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_per_w_set | 2719465ns | +8860.0ns (+0.3%) | [+276, +21014]ns | [2712838, 2724706] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_madd_zig_tail_dispatch | 3176958ns | +466477.1ns (+17.2%) | [+371199, +502903]ns | [3080390, 3210090] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_madd_zig_tail_runtime_w | 3200682ns | +493541.7ns (+18.2%) | [+468746, +522103]ns | [3182721, 3227845] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_madd_zig_runtime_w | abi_zig_entry_madd_zig_anchor | abi_zig_entry_madd_zig_dispatch | abi_zig_entry_madd_zig_null | abi_zig_entry_madd_zig_per_w_set | abi_zig_entry_madd_zig_tail_dispatch | abi_zig_entry_madd_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2715848ns | +0.9% | -0.2% | -99.9% | +0.1% | +13.8% | +17.7% |
| 2 | 2702533ns | +2.1% | +0.4% | -99.9% | +0.3% | +13.6% | +19.0% |
| 3 | 2708951ns | -0.2% | +0.3% | -99.9% | +0.6% | +18.1% | +19.6% |
| 4 | 2698432ns | +2.4% | +0.6% | -99.9% | +1.0% | +17.3% | +18.8% |
| 5 | 2722529ns | -0.8% | -0.3% | -99.9% | -0.1% | +17.1% | +17.2% |
| 6 | 2705422ns | +2.1% | +0.0% | -99.9% | +0.4% | +19.1% | +17.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_madd_zig_anchor | -0.799 | HIGH- (thermal bounce) |
| abi_zig_entry_madd_zig_dispatch | -0.114 | ok |
| abi_zig_entry_madd_zig_null | -0.287 | moderate- |
| abi_zig_entry_madd_zig_per_w_set | -0.031 | ok |
| abi_zig_entry_madd_zig_runtime_w | -0.593 | HIGH- (thermal bounce) |
| abi_zig_entry_madd_zig_tail_dispatch | 0.254 | moderate+ |
| abi_zig_entry_madd_zig_tail_runtime_w | 0.287 | moderate+ |

**Consistency summary:**

- **abi_zig_entry_madd_zig_anchor**: won 2/6, lost 4/6
- **abi_zig_entry_madd_zig_dispatch**: won 2/6, lost 3/6
- **abi_zig_entry_madd_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_madd_zig_per_w_set**: won 0/6, lost 4/6
- **abi_zig_entry_madd_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_madd_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_madd_zig_anchor | 8569389.7ns | 2738247.4ns | 313.0% | HIGH |
| abi_zig_entry_madd_zig_dispatch | 8441117.0ns | 2712853.1ns | 311.2% | HIGH |
| abi_zig_entry_madd_zig_null | 304626.5ns | 2307.9ns | 13199.2% | HIGH |
| abi_zig_entry_madd_zig_per_w_set | 8479309.6ns | 2719002.7ns | 311.9% | HIGH |
| abi_zig_entry_madd_zig_runtime_w | 8418388.9ns | 2708952.6ns | 310.8% | HIGH |
| abi_zig_entry_madd_zig_tail_dispatch | 9772166.0ns | 3155812.4ns | 309.7% | HIGH |
| abi_zig_entry_madd_zig_tail_runtime_w | 9931213.8ns | 3203749.6ns | 310.0% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_madd_zig_anchor (n=6, range 2699819.2-2762779.8 ns)
  2699819.2 |####################
  2702967.2 |####################
  2706115.3 |
  2709263.3 |
  2712411.3 |
  2715559.4 |
  2718707.4 |
  2721855.4 |
  2725003.4 |
  2728151.5 |
  2731299.5 |
  2734447.5 |
  2737595.6 |####################
  2740743.6 |
  2743891.6 |
  2747039.6 |
  2750187.7 |
  2753335.7 |
  2756483.7 |
  2759631.8 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_dispatch (n=6, range 2705969.6-2715895.9 ns)
  2705969.6 |########################################
  2706465.9 |
  2706962.2 |
  2707458.5 |
  2707954.9 |
  2708451.2 |
  2708947.5 |
  2709443.8 |
  2709940.1 |
  2710436.4 |
  2710932.7 |
  2711429.0 |########################################
  2711925.4 |
  2712421.7 |
  2712918.0 |########################################
  2713414.3 |
  2713910.6 |########################################
  2714406.9 |
  2714903.2 |
  2715399.5 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_null (n=6, range 2277.1-2347.3 ns)
   2277.1 |####################
   2280.6 |
   2284.1 |
   2287.6 |########################################
   2291.1 |
   2294.7 |####################
   2298.2 |
   2301.7 |
   2305.2 |
   2308.7 |####################
   2312.2 |
   2315.7 |
   2319.2 |
   2322.7 |
   2326.2 |
   2329.8 |
   2333.3 |
   2336.8 |
   2340.3 |
   2343.8 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_per_w_set (n=6, range 2710457.1-2724705.9 ns)
  2710457.1 |########################################
  2711169.5 |
  2711882.0 |
  2712594.4 |
  2713306.9 |
  2714019.3 |
  2714731.7 |########################################
  2715444.2 |
  2716156.6 |
  2716869.0 |
  2717581.5 |########################################
  2718293.9 |
  2719006.4 |
  2719718.8 |
  2720431.2 |########################################
  2721143.7 |
  2721856.1 |
  2722568.5 |
  2723281.0 |
  2723993.4 |########################################
  (0 below, 1 above range)

abi_zig_entry_madd_zig_runtime_w (n=6, range 2698432.1-2719188.8 ns)
  2698432.1 |########################################
  2699469.9 |
  2700507.8 |
  2701545.6 |########################################
  2702583.4 |
  2703621.3 |
  2704659.1 |########################################
  2705696.9 |
  2706734.8 |
  2707772.6 |
  2708810.4 |########################################
  2709848.3 |
  2710886.1 |
  2711923.9 |
  2712961.8 |
  2713999.6 |
  2715037.4 |########################################
  2716075.3 |
  2717113.1 |
  2718150.9 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_dispatch (n=6, range 3070629.2-3210089.6 ns)
  3070629.2 |########################################
  3077602.2 |
  3084575.2 |########################################
  3091548.3 |
  3098521.3 |
  3105494.3 |
  3112467.3 |
  3119440.3 |
  3126413.4 |
  3133386.4 |
  3140359.4 |
  3147332.4 |
  3154305.4 |
  3161278.5 |########################################
  3168251.5 |
  3175224.5 |
  3182197.5 |########################################
  3189170.5 |
  3196143.6 |########################################
  3203116.6 |
  (0 below, 1 above range)

abi_zig_entry_madd_zig_tail_runtime_w (n=6, range 3175016.7-3227845.5 ns)
  3175016.7 |########################################
  3177658.1 |
  3180299.6 |
  3182941.0 |
  3185582.5 |
  3188223.9 |########################################
  3190865.3 |
  3193506.8 |
  3196148.2 |########################################
  3198789.6 |
  3201431.1 |
  3204072.5 |########################################
  3206714.0 |
  3209355.4 |
  3211996.8 |
  3214638.3 |########################################
  3217279.7 |
  3219921.1 |
  3222562.6 |
  3225204.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_madd_zig_anchor**: bridge=313.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_dispatch**: bridge=310.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_null**: bridge=13099.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_per_w_set**: bridge=311.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_runtime_w**: bridge=311.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_dispatch**: bridge=309.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_madd_zig_tail_runtime_w**: bridge=310.0% of algo (FFI overhead may distort results)
