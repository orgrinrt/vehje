# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 53828% faster than the next best (abi_zig_entry_leaf_zig_per_w_set)

abi_zig_entry_leaf_zig_null (2.67 us) leads abi_zig_entry_leaf_zig_per_w_set (1.44 ms) by 53828%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.44 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 1225.2x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.27 ms) is 1225.2x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_leaf_zig_anchor shows alternating (throttle bounce) (autocorr -0.74)

abi_zig_entry_leaf_zig_anchor's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (53828% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 53828% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1225.2x the fastest

Fastest abi_zig_entry_leaf_zig_null (2.67 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.27 ms): 1225.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 2667.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1225.18x (fastest 2667.9 ns, slowest 3268666.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1444493ns | 1444357ns | 1438532ns | 1443393ns | 1449124ns | -0.14% |
| abi_zig_entry_leaf_zig_dispatch | 1441827ns | 1442034ns | 1434299ns | 1441358ns | 1446294ns | -0.33% |
| abi_zig_entry_leaf_zig_null | 4985ns | 4986ns | 4900ns | 4969ns | 5051ns | -99.66% |
| abi_zig_entry_leaf_zig_per_w_set | 1449563ns | 1441594ns | 1436953ns | 1440623ns | 1469277ns | +0.21% |
| abi_zig_entry_leaf_zig_runtime_w | 1446583ns | 1445986ns | 1436622ns | 1443949ns | 1455516ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3239150ns | 3267743ns | 3098214ns | 3263765ns | 3272694ns | +123.92% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3274246ns | 3272027ns | 3249512ns | 3270756ns | 3291849ns | +126.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1441555ns | 1435968ns | 1445946ns | -0.14% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1439024ns | 1431755ns | 1443429ns | -0.32% | 0.000 |
| abi_zig_entry_leaf_zig_null | 2670ns | 2639ns | 2695ns | -99.82% | 0.048 |
| abi_zig_entry_leaf_zig_per_w_set | 1446602ns | 1434175ns | 1466044ns | +0.21% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1443606ns | 1434069ns | 1452249ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3235952ns | 3094754ns | 3269491ns | +124.16% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3271004ns | 3246776ns | 3288447ns | +126.59% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 197028.2 | 1440764.2 | 1441554.7 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 192480.1 | 1440185.8 | 1439023.6 | n/a |
| abi_zig_entry_leaf_zig_null | 156683.1 | 2770.5 | 2669.8 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 202638.2 | 1447831.5 | 1446602.2 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 200330.0 | 1442843.2 | 1443605.5 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 223367.1 | 3236016.4 | 3235952.2 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 223110.6 | 3272703.1 | 3271004.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_null | 0.048 | 98.9% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1444493ns | 1444493ns | -0.14% |
| abi_zig_entry_leaf_zig_dispatch | 1441827ns | 1441827ns | -0.33% |
| abi_zig_entry_leaf_zig_null | 4985ns | 4985ns | -99.66% |
| abi_zig_entry_leaf_zig_per_w_set | 1449563ns | 1449563ns | +0.21% |
| abi_zig_entry_leaf_zig_runtime_w | 1446583ns | 1446583ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3239150ns | 3239150ns | +123.92% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3274246ns | 3274246ns | +126.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1442913ns | base | --- | [1435654, 1452249] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1441356ns | no significant difference | [-7128, +4371]ns | [1437362, 1445946] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1439063ns | no significant difference | [-13186, +4560]ns | [1434579, 1443429] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_null | 2668ns | -1440262.1ns (-99.8%) | [-1449554, -1432991]ns | [2646, 2695] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1438748ns | no significant difference | [-9188, +20817]ns | [1435014, 1466044] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3264670ns | +1820651.4ns (+126.2%) | [+1728851, +1827537]ns | [3173696, 3269491] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3268666ns | +1821971.9ns (+126.3%) | [+1814513, +1845712]ns | [3255900, 3288447] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1437240ns | -0.1% | -0.4% | -99.8% | -0.2% | +126.3% | +125.9% |
| 2 | 1452449ns | -0.3% | -0.9% | -99.8% | +2.6% | +113.1% | +125.3% |
| 3 | 1443668ns | -0.3% | +0.0% | -99.8% | -0.2% | +126.1% | +128.9% |
| 4 | 1434069ns | +0.7% | +0.6% | -99.8% | +0.2% | +127.7% | +127.7% |
| 5 | 1442158ns | -0.1% | -0.3% | -99.8% | -0.4% | +126.5% | +126.4% |
| 6 | 1452049ns | -0.6% | -0.9% | -99.8% | -0.8% | +125.4% | +125.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.739 | HIGH- (thermal bounce) |
| abi_zig_entry_leaf_zig_dispatch | 0.131 | ok |
| abi_zig_entry_leaf_zig_null | -0.172 | ok |
| abi_zig_entry_leaf_zig_per_w_set | -0.236 | moderate- |
| abi_zig_entry_leaf_zig_runtime_w | -0.193 | ok |
| abi_zig_entry_leaf_zig_tail_dispatch | -0.147 | ok |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.095 | ok |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 4/6, lost 1/6
- **abi_zig_entry_leaf_zig_dispatch**: won 4/6, lost 1/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 4/6, lost 2/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4594132.6ns | 1441554.7ns | 318.7% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4579677.3ns | 1439023.6ns | 318.2% | HIGH |
| abi_zig_entry_leaf_zig_null | 305829.9ns | 2669.8ns | 11454.9% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4616570.9ns | 1446602.2ns | 319.1% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4602822.1ns | 1443605.5ns | 318.8% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 10015620.8ns | 3235952.2ns | 309.5% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10121489.8ns | 3271004.3ns | 309.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1435968.3-1445946.1 ns)
  1435968.3 |########################################
  1436467.2 |
  1436966.1 |
  1437465.0 |
  1437963.9 |
  1438462.7 |########################################
  1438961.6 |
  1439460.5 |
  1439959.4 |########################################
  1440458.3 |
  1440957.2 |
  1441456.1 |
  1441954.9 |
  1442453.8 |########################################
  1442952.7 |
  1443451.6 |
  1443950.5 |########################################
  1444449.4 |
  1444948.3 |
  1445447.2 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1431754.6-1443429.0 ns)
  1431754.6 |####################
  1432338.3 |
  1432922.0 |
  1433505.8 |
  1434089.5 |
  1434673.2 |
  1435256.9 |
  1435840.6 |
  1436424.3 |
  1437008.1 |####################
  1437591.8 |
  1438175.5 |
  1438759.2 |########################################
  1439342.9 |
  1439926.6 |
  1440510.4 |
  1441094.1 |
  1441677.8 |
  1442261.5 |
  1442845.2 |####################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 2639.2-2695.2 ns)
   2639.2 |####################
   2642.0 |
   2644.8 |
   2647.6 |
   2650.4 |
   2653.2 |####################
   2656.0 |
   2658.8 |
   2661.6 |####################
   2664.4 |
   2667.2 |
   2670.0 |
   2672.8 |########################################
   2675.6 |
   2678.4 |
   2681.2 |
   2684.0 |
   2686.8 |
   2689.6 |
   2692.4 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1434174.6-1466044.2 ns)
  1434174.6 |########################################
  1435768.1 |########################################
  1437361.6 |########################################
  1438955.0 |########################################
  1440548.5 |########################################
  1442142.0 |
  1443735.5 |
  1445329.0 |
  1446922.4 |
  1448515.9 |
  1450109.4 |
  1451702.9 |
  1453296.4 |
  1454889.8 |
  1456483.3 |
  1458076.8 |
  1459670.3 |
  1461263.8 |
  1462857.2 |
  1464450.7 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1434068.8-1452249.2 ns)
  1434068.8 |########################################
  1434977.8 |
  1435886.8 |
  1436795.9 |########################################
  1437704.9 |
  1438613.9 |
  1439522.9 |
  1440431.9 |
  1441341.0 |########################################
  1442250.0 |
  1443159.0 |########################################
  1444068.0 |
  1444977.0 |
  1445886.1 |
  1446795.1 |
  1447704.1 |
  1448613.1 |
  1449522.1 |
  1450431.2 |
  1451340.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3094753.7-3269490.8 ns)
  3094753.7 |#############
  3103490.6 |
  3112227.4 |
  3120964.3 |
  3129701.1 |
  3138438.0 |
  3147174.8 |
  3155911.7 |
  3164648.6 |
  3173385.4 |
  3182122.3 |
  3190859.1 |
  3199596.0 |
  3208332.8 |
  3217069.7 |
  3225806.6 |
  3234543.4 |
  3243280.3 |
  3252017.1 |#############
  3260754.0 |########################################
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3246776.2-3288446.9 ns)
  3246776.2 |####################
  3248859.7 |
  3250943.3 |
  3253026.8 |
  3255110.3 |
  3257193.9 |
  3259277.4 |
  3261360.9 |
  3263444.5 |########################################
  3265528.0 |
  3267611.5 |
  3269695.1 |
  3271778.6 |########################################
  3273862.1 |
  3275945.7 |
  3278029.2 |
  3280112.7 |
  3282196.3 |
  3284279.8 |
  3286363.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=318.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=318.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=11498.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=319.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=319.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=309.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=309.4% of algo (FFI overhead may distort results)
