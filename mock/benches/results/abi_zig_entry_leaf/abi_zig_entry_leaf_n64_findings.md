# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 57682% faster than the next best (abi_zig_entry_leaf_zig_dispatch)

abi_zig_entry_leaf_zig_null (2.48 us) leads abi_zig_entry_leaf_zig_dispatch (1.43 ms) by 57682%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.43 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 1313.8x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.25 ms) is 1313.8x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_leaf_zig_null shows alternating (throttle bounce) (autocorr -0.53)

abi_zig_entry_leaf_zig_null's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (57682% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 57682% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1313.8x the fastest

Fastest abi_zig_entry_leaf_zig_null (2.48 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.25 ms): 1313.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 2476.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1313.81x (fastest 2476.4 ns, slowest 3253593.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1438603ns | 1438266ns | 1434802ns | 1437550ns | 1442083ns | +0.13% |
| abi_zig_entry_leaf_zig_dispatch | 1433889ns | 1433505ns | 1430982ns | 1433064ns | 1436580ns | -0.19% |
| abi_zig_entry_leaf_zig_null | 4792ns | 4775ns | 4690ns | 4765ns | 4884ns | -99.67% |
| abi_zig_entry_leaf_zig_per_w_set | 1435414ns | 1434697ns | 1433862ns | 1434432ns | 1437664ns | -0.09% |
| abi_zig_entry_leaf_zig_runtime_w | 1436669ns | 1437139ns | 1432182ns | 1436804ns | 1438709ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3199367ns | 3252478ns | 3079771ns | 3199593ns | 3258826ns | +122.69% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3277709ns | 3256437ns | 3246082ns | 3255427ns | 3326945ns | +128.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1435952ns | 1432283ns | 1439405ns | +0.13% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1431313ns | 1428352ns | 1434011ns | -0.19% | 0.000 |
| abi_zig_entry_leaf_zig_null | 2493ns | 2462ns | 2539ns | -99.83% | 0.026 |
| abi_zig_entry_leaf_zig_per_w_set | 1432852ns | 1431373ns | 1435066ns | -0.08% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1434036ns | 1429624ns | 1435962ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3196532ns | 3077088ns | 3256000ns | +122.90% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3274785ns | 3243471ns | 3323699ns | +128.36% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 174841.4 | 1434365.1 | 1435952.2 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 170692.7 | 1431415.1 | 1431312.7 | n/a |
| abi_zig_entry_leaf_zig_null | 154478.9 | 2718.3 | 2493.5 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 172871.6 | 1432561.9 | 1432851.9 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 173274.9 | 1433392.0 | 1434035.7 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 189592.3 | 3195721.1 | 3196532.5 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 200612.5 | 3289950.6 | 3274785.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_null | 0.026 | 99.4% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1438603ns | 1438603ns | +0.13% |
| abi_zig_entry_leaf_zig_dispatch | 1433889ns | 1433889ns | -0.19% |
| abi_zig_entry_leaf_zig_null | 4792ns | 4792ns | -99.67% |
| abi_zig_entry_leaf_zig_per_w_set | 1435414ns | 1435414ns | -0.09% |
| abi_zig_entry_leaf_zig_runtime_w | 1436669ns | 1436669ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3199367ns | 3199367ns | +122.69% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3277709ns | 3277709ns | +128.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1434557ns | base | --- | [1431588, 1435962] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1435617ns | no significant difference | [-1272, +4428]ns | [1432835, 1439405] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1430946ns | no significant difference | [-5996, +1340]ns | [1428981, 1434011] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_null | 2476ns | -1432054.1ns (-99.8%) | [-1433449, -1429123]ns | [2465, 2539] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1432112ns | no significant difference | [-4164, +2648]ns | [1431378, 1435066] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3249665ns | +1815608.8ns (+126.6%) | [+1649141, +1822741]ns | [3083932, 3256000] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3253593ns | +1818502.9ns (+126.8%) | [+1811635, +1892111]ns | [3247064, 3323699] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1435212ns | +0.4% | -0.4% | -99.8% | +0.0% | +115.4% | +126.5% |
| 2 | 1434370ns | +0.2% | -0.1% | -99.8% | -0.2% | +114.5% | +126.1% |
| 3 | 1429624ns | +0.2% | +0.1% | -99.8% | +0.3% | +127.3% | +128.2% |
| 4 | 1436713ns | -0.2% | -0.4% | -99.8% | -0.4% | +126.2% | +126.3% |
| 5 | 1433552ns | +0.1% | +0.1% | -99.8% | -0.1% | +127.4% | +136.2% |
| 6 | 1434743ns | +0.2% | -0.4% | -99.8% | -0.2% | +126.7% | +127.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.212 | moderate+ |
| abi_zig_entry_leaf_zig_dispatch | -0.514 | HIGH- (thermal bounce) |
| abi_zig_entry_leaf_zig_null | -0.532 | HIGH- (thermal bounce) |
| abi_zig_entry_leaf_zig_per_w_set | -0.412 | moderate- |
| abi_zig_entry_leaf_zig_runtime_w | -0.504 | HIGH- (thermal bounce) |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.420 | moderate+ |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.216 | moderate- |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 1/6, lost 4/6
- **abi_zig_entry_leaf_zig_dispatch**: won 4/6, lost 1/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 4/6, lost 1/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4544827.9ns | 1435952.2ns | 316.5% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4531508.2ns | 1431312.7ns | 316.6% | HIGH |
| abi_zig_entry_leaf_zig_null | 297592.0ns | 2493.5ns | 11934.9% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4534404.2ns | 1432851.9ns | 316.5% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4542979.3ns | 1434035.7ns | 316.8% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9847897.5ns | 3196532.5ns | 308.1% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10150148.1ns | 3274785.5ns | 309.9% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1432282.9-1439405.0 ns)
  1432282.9 |########################################
  1432639.0 |
  1432995.1 |
  1433351.2 |########################################
  1433707.3 |
  1434063.4 |########################################
  1434419.5 |
  1434775.6 |
  1435131.7 |
  1435487.8 |
  1435843.9 |
  1436200.1 |
  1436556.2 |########################################
  1436912.3 |
  1437268.4 |
  1437624.5 |
  1437980.6 |########################################
  1438336.7 |
  1438692.8 |
  1439048.9 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1428352.5-1434011.1 ns)
  1428352.5 |########################################
  1428635.4 |
  1428918.4 |
  1429201.3 |
  1429484.2 |########################################
  1429767.1 |
  1430050.1 |
  1430333.0 |
  1430615.9 |########################################
  1430898.8 |
  1431181.8 |########################################
  1431464.7 |
  1431747.6 |
  1432030.6 |
  1432313.5 |
  1432596.4 |########################################
  1432879.3 |
  1433162.3 |
  1433445.2 |
  1433728.1 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 2462.1-2539.1 ns)
   2462.1 |########################################
   2466.0 |########################################
   2469.8 |########################################
   2473.7 |
   2477.5 |
   2481.4 |########################################
   2485.2 |
   2489.1 |
   2492.9 |
   2496.8 |
   2500.6 |
   2504.5 |
   2508.3 |
   2512.2 |
   2516.0 |
   2519.9 |########################################
   2523.7 |
   2527.6 |
   2531.4 |
   2535.3 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1431372.9-1435065.6 ns)
  1431372.9 |########################################
  1431557.5 |####################
  1431742.2 |
  1431926.8 |
  1432111.4 |
  1432296.1 |
  1432480.7 |####################
  1432665.4 |
  1432850.0 |
  1433034.6 |
  1433219.3 |
  1433403.9 |
  1433588.5 |
  1433773.2 |
  1433957.8 |
  1434142.5 |
  1434327.1 |####################
  1434511.7 |
  1434696.4 |
  1434881.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1429623.8-1435962.5 ns)
  1429623.8 |########################################
  1429940.7 |
  1430257.7 |
  1430574.6 |
  1430891.5 |
  1431208.5 |
  1431525.4 |
  1431842.3 |
  1432159.3 |
  1432476.2 |
  1432793.1 |
  1433110.1 |
  1433427.0 |########################################
  1433744.0 |
  1434060.9 |########################################
  1434377.8 |
  1434694.8 |########################################
  1435011.7 |########################################
  1435328.6 |
  1435645.6 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3077088.3-3256000.2 ns)
  3077088.3 |#############
  3086033.9 |#############
  3094979.5 |
  3103925.1 |
  3112870.7 |
  3121816.3 |
  3130761.9 |
  3139707.5 |
  3148653.1 |
  3157598.7 |
  3166544.2 |
  3175489.8 |
  3184435.4 |
  3193381.0 |
  3202326.6 |
  3211272.2 |
  3220217.8 |
  3229163.4 |
  3238109.0 |
  3247054.6 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3243470.8-3323699.4 ns)
  3243470.8 |####################
  3247482.2 |########################################
  3251493.7 |
  3255505.1 |####################
  3259516.5 |####################
  3263527.9 |
  3267539.4 |
  3271550.8 |
  3275562.2 |
  3279573.6 |
  3283585.1 |
  3287596.5 |
  3291607.9 |
  3295619.4 |
  3299630.8 |
  3303642.2 |
  3307653.6 |
  3311665.1 |
  3315676.5 |
  3319687.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=316.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=316.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=12022.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=316.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=307.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=308.3% of algo (FFI overhead may distort results)
