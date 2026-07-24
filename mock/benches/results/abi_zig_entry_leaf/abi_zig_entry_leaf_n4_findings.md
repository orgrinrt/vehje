# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 36250% faster than the next best (abi_zig_entry_leaf_zig_dispatch)

abi_zig_entry_leaf_zig_null (3.94 us) leads abi_zig_entry_leaf_zig_dispatch (1.43 ms) by 36250%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.43 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 827.9x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.26 ms) is 827.9x the fastest (3.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_leaf_zig_tail_dispatch shows alternating (throttle bounce) (autocorr -0.78)

abi_zig_entry_leaf_zig_tail_dispatch's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (36250% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 36250% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 827.9x the fastest

Fastest abi_zig_entry_leaf_zig_null (3.94 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.26 ms): 827.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 3942.7 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 827.88x (fastest 3942.7 ns, slowest 3264079.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1441463ns | 1440165ns | 1434330ns | 1439229ns | 1448382ns | +0.08% |
| abi_zig_entry_leaf_zig_dispatch | 1436187ns | 1435739ns | 1434054ns | 1435521ns | 1438251ns | -0.29% |
| abi_zig_entry_leaf_zig_null | 6275ns | 6237ns | 6135ns | 6215ns | 6435ns | -99.56% |
| abi_zig_entry_leaf_zig_per_w_set | 1437899ns | 1437416ns | 1434884ns | 1436853ns | 1440976ns | -0.17% |
| abi_zig_entry_leaf_zig_runtime_w | 1440343ns | 1438067ns | 1433434ns | 1437328ns | 1448321ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3075721ns | 3076860ns | 3071372ns | 3075105ns | 3078819ns | +113.54% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3312546ns | 3266943ns | 3252148ns | 3265676ns | 3413050ns | +129.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1438810ns | 1431655ns | 1445638ns | +0.07% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1433614ns | 1431620ns | 1435642ns | -0.29% | 0.000 |
| abi_zig_entry_leaf_zig_null | 3968ns | 3884ns | 4067ns | -99.72% | 0.001 |
| abi_zig_entry_leaf_zig_per_w_set | 1435330ns | 1432388ns | 1438401ns | -0.17% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1437788ns | 1431030ns | 1445675ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3073001ns | 3068821ns | 3076080ns | +113.73% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3309611ns | 3249484ns | 3409886ns | +130.19% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 178528.1 | 1437843.9 | 1438809.7 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 171083.3 | 1433195.2 | 1433613.8 | n/a |
| abi_zig_entry_leaf_zig_null | 156807.8 | 4203.7 | 3967.8 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 172386.9 | 1433685.6 | 1435330.3 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 172944.1 | 1437300.5 | 1437788.3 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 187182.6 | 3075279.2 | 3073001.1 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 202514.6 | 3320627.1 | 3309611.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_null | 0.001 | 98.5% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.3% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1441463ns | 1441463ns | +0.08% |
| abi_zig_entry_leaf_zig_dispatch | 1436187ns | 1436187ns | -0.29% |
| abi_zig_entry_leaf_zig_null | 6275ns | 6275ns | -99.56% |
| abi_zig_entry_leaf_zig_per_w_set | 1437899ns | 1437899ns | -0.17% |
| abi_zig_entry_leaf_zig_runtime_w | 1440343ns | 1440343ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3075721ns | 3075721ns | +113.54% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3312546ns | 3312546ns | +129.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1435516ns | base | --- | [1432174, 1445675] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1437579ns | no significant difference | [-4921, +7061]ns | [1433212, 1445638] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1433169ns | no significant difference | [-12507, +1748]ns | [1432030, 1435642] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_null | 3943ns | -1431562.1ns (-99.7%) | [-1441668, -1428231]ns | [3894, 4067] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1434802ns | no significant difference | [-12193, +4911]ns | [1432788, 1438401] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3073975ns | +1637717.9ns (+114.1%) | [+1627246, +1640675]ns | [3068948, 3076080] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3264080ns | +1826933.8ns (+127.3%) | [+1817922, +1970613]ns | [3254868, 3409886] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1436149ns | -0.1% | -0.3% | -99.7% | -0.3% | +114.0% | +127.3% |
| 2 | 1434882ns | +0.2% | +0.0% | -99.7% | +0.4% | +114.3% | +127.2% |
| 3 | 1445228ns | -0.6% | -0.8% | -99.7% | -0.8% | +112.4% | +144.3% |
| 4 | 1446123ns | +0.3% | -0.9% | -99.7% | -0.9% | +112.8% | +125.7% |
| 5 | 1431030ns | +0.7% | +0.0% | -99.7% | +0.3% | +114.4% | +127.1% |
| 6 | 1433318ns | -0.1% | +0.2% | -99.7% | +0.2% | +114.5% | +129.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.030 | ok |
| abi_zig_entry_leaf_zig_dispatch | -0.292 | moderate- |
| abi_zig_entry_leaf_zig_null | -0.070 | ok |
| abi_zig_entry_leaf_zig_per_w_set | -0.446 | moderate- |
| abi_zig_entry_leaf_zig_runtime_w | 0.094 | ok |
| abi_zig_entry_leaf_zig_tail_dispatch | -0.783 | HIGH- (thermal bounce) |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.250 | moderate- |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 2/6, lost 3/6
- **abi_zig_entry_leaf_zig_dispatch**: won 3/6, lost 1/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 3/6, lost 3/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4557839.1ns | 1438809.7ns | 316.8% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4535212.1ns | 1433613.8ns | 316.3% | HIGH |
| abi_zig_entry_leaf_zig_null | 307517.4ns | 3967.8ns | 7750.4% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4537744.1ns | 1435330.3ns | 316.1% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4551211.8ns | 1437788.3ns | 316.5% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9481703.3ns | 3073001.1ns | 308.5% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10276289.4ns | 3309611.1ns | 310.5% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1431654.6-1445637.9 ns)
  1431654.6 |########################################
  1432353.8 |
  1433052.9 |
  1433752.1 |
  1434451.3 |########################################
  1435150.4 |
  1435849.6 |
  1436548.8 |########################################
  1437247.9 |
  1437947.1 |########################################
  1438646.2 |
  1439345.4 |
  1440044.6 |
  1440743.7 |########################################
  1441442.9 |
  1442142.1 |
  1442841.2 |
  1443540.4 |
  1444239.6 |
  1444938.7 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1431619.6-1435642.3 ns)
  1431619.6 |####################
  1431820.7 |
  1432021.9 |
  1432223.0 |
  1432424.1 |########################################
  1432625.3 |
  1432826.4 |
  1433027.5 |
  1433228.7 |
  1433429.8 |
  1433631.0 |
  1433832.1 |####################
  1434033.2 |
  1434234.4 |
  1434435.5 |
  1434636.6 |
  1434837.8 |
  1435038.9 |####################
  1435240.0 |
  1435441.2 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 3884.2-4066.6 ns)
   3884.2 |########################################
   3893.3 |
   3902.4 |########################################
   3911.6 |########################################
   3920.7 |
   3929.8 |
   3938.9 |
   3948.1 |
   3957.2 |
   3966.3 |########################################
   3975.4 |
   3984.5 |
   3993.7 |
   4002.8 |
   4011.9 |
   4021.0 |########################################
   4030.2 |
   4039.3 |
   4048.4 |
   4057.5 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1432387.5-1438401.0 ns)
  1432387.5 |########################################
  1432688.2 |
  1432988.9 |########################################
  1433289.5 |
  1433590.2 |########################################
  1433890.9 |
  1434191.6 |
  1434492.2 |
  1434792.9 |
  1435093.6 |
  1435394.3 |
  1435695.0 |########################################
  1435995.6 |
  1436296.3 |
  1436597.0 |########################################
  1436897.7 |
  1437198.3 |
  1437499.0 |
  1437799.7 |
  1438100.4 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1431030.0-1445675.4 ns)
  1431030.0 |########################################
  1431762.3 |
  1432494.5 |
  1433226.8 |########################################
  1433959.1 |
  1434691.4 |########################################
  1435423.6 |########################################
  1436155.9 |
  1436888.2 |
  1437620.4 |
  1438352.7 |
  1439085.0 |
  1439817.2 |
  1440549.5 |
  1441281.8 |
  1442014.0 |
  1442746.3 |
  1443478.6 |
  1444210.9 |
  1444943.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3068820.8-3076080.0 ns)
  3068820.8 |########################################
  3069183.8 |
  3069546.7 |
  3069909.7 |
  3070272.6 |
  3070635.6 |
  3070998.6 |
  3071361.5 |
  3071724.5 |
  3072087.4 |
  3072450.4 |
  3072813.4 |
  3073176.3 |
  3073539.3 |####################
  3073902.2 |####################
  3074265.2 |
  3074628.2 |
  3074991.1 |
  3075354.1 |####################
  3075717.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3249483.8-3409885.7 ns)
  3249483.8 |#############
  3257503.9 |########################################
  3265524.0 |
  3273544.1 |
  3281564.2 |#############
  3289584.3 |
  3297604.4 |
  3305624.4 |
  3313644.5 |
  3321664.6 |
  3329684.7 |
  3337704.8 |
  3345724.9 |
  3353745.0 |
  3361765.1 |
  3369785.2 |
  3377805.3 |
  3385825.4 |
  3393845.5 |
  3401865.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=316.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=316.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=7800.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=316.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=308.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=307.9% of algo (FFI overhead may distort results)
