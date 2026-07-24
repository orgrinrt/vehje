# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_leaf_null_entry dominates: 29465% faster than the next best (abi_boundary_w_leaf_zig_runtime_w)

abi_boundary_w_leaf_null_entry (4.89 us) leads abi_boundary_w_leaf_zig_runtime_w (1.45 ms) by 29465%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_soa_runtime_w is an outlier: 299.5x slower than the field

abi_boundary_w_leaf_soa_runtime_w (1.46 ms) is 299.5x the fastest (4.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_soa_per_w shows alternating (throttle bounce) (autocorr -0.53)

abi_boundary_w_leaf_soa_per_w's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_soa_runtime_w} (29465% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_soa_runtime_w} with a 29465% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 299.5x the fastest

Fastest abi_boundary_w_leaf_null_entry (4.89 us) to slowest abi_boundary_w_leaf_soa_runtime_w (1.46 ms): 299.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 4890.2 ns median (-99.7% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 299.55x (fastest 4890.2 ns, slowest 1464857.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 7225ns | 7150ns | 6970ns | 7137ns | 7484ns | -99.51% |
| abi_boundary_w_leaf_scalar_anchor | 1456877ns | 1456044ns | 1453004ns | 1455716ns | 1460557ns | -0.99% |
| abi_boundary_w_leaf_scalar_dispatch | 1458917ns | 1458978ns | 1457875ns | 1458748ns | 1459691ns | -0.85% |
| abi_boundary_w_leaf_scalar_per_w | 1459086ns | 1458798ns | 1454320ns | 1458562ns | 1462254ns | -0.84% |
| abi_boundary_w_leaf_scalar_runtime_w | 1471445ns | 1465778ns | 1461420ns | 1465365ns | 1485576ns | base |
| abi_boundary_w_leaf_soa_dispatch | 1464240ns | 1464459ns | 1458179ns | 1463695ns | 1468088ns | -0.49% |
| abi_boundary_w_leaf_soa_per_w | 1459158ns | 1458721ns | 1455524ns | 1457832ns | 1462964ns | -0.83% |
| abi_boundary_w_leaf_soa_runtime_w | 1466934ns | 1467553ns | 1459933ns | 1466026ns | 1471797ns | -0.31% |
| abi_boundary_w_leaf_zig_runtime_w | 1448898ns | 1448585ns | 1445755ns | 1448067ns | 1451715ns | -1.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4930ns | 4763ns | 5123ns | -99.66% | 0.000 |
| abi_boundary_w_leaf_scalar_anchor | 1454195ns | 1450440ns | 1457793ns | -0.98% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1456248ns | 1455289ns | 1456970ns | -0.84% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1456394ns | 1451876ns | 1459417ns | -0.83% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1468584ns | 1458614ns | 1482518ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 1461461ns | 1455514ns | 1465136ns | -0.49% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 1456347ns | 1452912ns | 1460127ns | -0.83% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 1464223ns | 1457225ns | 1468959ns | -0.30% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1446061ns | 1442920ns | 1448806ns | -1.53% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 27835.6 | 4979.1 | 4930.2 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 44043.5 | 1452662.6 | 1454194.9 | n/a |
| abi_boundary_w_leaf_scalar_dispatch | 42616.3 | 1456379.0 | 1456247.6 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 46048.9 | 1456441.8 | 1456393.6 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 48773.1 | 1469110.3 | 1468584.4 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 46326.9 | 1459778.6 | 1461461.5 | n/a |
| abi_boundary_w_leaf_soa_per_w | 47532.4 | 1456124.2 | 1456347.2 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 44914.5 | 1465322.9 | 1464223.1 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 190909.7 | 1444474.4 | 1446061.2 | 6 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.000 | 97.4% |
| abi_boundary_w_leaf_scalar_anchor | 0.000 | 0.3% |
| abi_boundary_w_leaf_scalar_dispatch | 0.000 | 0.3% |
| abi_boundary_w_leaf_scalar_per_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_scalar_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_zig_runtime_w | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 7225ns | 7225ns | -99.51% |
| abi_boundary_w_leaf_scalar_anchor | 1456877ns | 1456877ns | -0.99% |
| abi_boundary_w_leaf_scalar_dispatch | 1458917ns | 1458917ns | -0.85% |
| abi_boundary_w_leaf_scalar_per_w | 1459086ns | 1459086ns | -0.84% |
| abi_boundary_w_leaf_scalar_runtime_w | 1471445ns | 1471445ns | base |
| abi_boundary_w_leaf_soa_dispatch | 1464240ns | 1464240ns | -0.49% |
| abi_boundary_w_leaf_soa_per_w | 1459158ns | 1459158ns | -0.83% |
| abi_boundary_w_leaf_soa_runtime_w | 1466934ns | 1466934ns | -0.31% |
| abi_boundary_w_leaf_zig_runtime_w | 1448898ns | 1448898ns | -1.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1463032ns | base | --- | [1460203, 1482518] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 4890ns | -1458149.8ns (-99.7%) | [-1477628, -1455185]ns | [4778, 5123] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1453374ns | -10575.3ns (-0.7%) | [-28919, -3675]ns | [1451418, 1457793] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1456309ns | -6987.7ns (-0.5%) | [-26188, -3835]ns | [1455463, 1456970] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1456109ns | -6206.3ns (-0.4%) | [-27019, -3347]ns | [1453655, 1459417] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 1461730ns | no significant difference | [-20648, +1573]ns | [1457518, 1465136] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_leaf_soa_per_w | 1455813ns | -4930.9ns (-0.3%) | [-29416, -2365]ns | [1453102, 1460127] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 1464857ns | no significant difference | [-19281, +5397]ns | [1458853, 1468959] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1445772ns | -18285.0ns (-1.2%) | [-35056, -14229]ns | [1443605, 1448806] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1464322ns | -99.7% | -0.3% | -0.5% | -0.3% | -0.6% | -0.8% | +0.0% | -1.2% |
| 2 | 1463792ns | -99.7% | -0.8% | -0.6% | -0.6% | -0.3% | -0.2% | +0.1% | -1.3% |
| 3 | 1462272ns | -99.7% | -0.7% | -0.4% | -0.2% | -0.0% | -0.4% | -0.3% | -1.0% |
| 4 | 1500714ns | -99.7% | -3.1% | -2.9% | -2.9% | -2.2% | -3.2% | -2.2% | -3.4% |
| 5 | 1461792ns | -99.7% | -0.8% | -0.3% | -0.7% | -0.0% | -0.1% | +0.6% | -1.3% |
| 6 | 1458614ns | -99.6% | -0.2% | -0.2% | -0.2% | +0.2% | -0.2% | +0.1% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.305 | moderate- |
| abi_boundary_w_leaf_scalar_anchor | -0.266 | moderate- |
| abi_boundary_w_leaf_scalar_dispatch | -0.081 | ok |
| abi_boundary_w_leaf_scalar_per_w | -0.018 | ok |
| abi_boundary_w_leaf_scalar_runtime_w | -0.240 | moderate- |
| abi_boundary_w_leaf_soa_dispatch | 0.169 | ok |
| abi_boundary_w_leaf_soa_per_w | -0.534 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_soa_runtime_w | -0.282 | moderate- |
| abi_boundary_w_leaf_zig_runtime_w | -0.017 | ok |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_dispatch**: won 3/6, lost 1/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 2/6, lost 2/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 124532.2ns | 4930.2ns | 2525.9% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4407776.9ns | 1454194.9ns | 303.1% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4414472.4ns | 1456247.6ns | 303.1% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4418041.0ns | 1456393.6ns | 303.4% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4460980.8ns | 1468584.4ns | 303.8% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 4428478.8ns | 1461461.5ns | 303.0% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 4418930.4ns | 1456347.2ns | 303.4% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 4445035.5ns | 1464223.1ns | 303.6% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4600655.3ns | 1446061.2ns | 318.2% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 4762.9-5122.7 ns)
   4762.9 |########################################
   4780.9 |########################################
   4798.9 |
   4816.9 |
   4834.9 |
   4852.8 |
   4870.8 |########################################
   4888.8 |########################################
   4906.8 |
   4924.8 |
   4942.8 |
   4960.8 |
   4978.8 |
   4996.8 |########################################
   5014.8 |
   5032.8 |
   5050.7 |
   5068.7 |
   5086.7 |
   5104.7 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1450440.4-1457793.4 ns)
  1450440.4 |####################
  1450808.0 |
  1451175.7 |
  1451543.3 |
  1451911.0 |
  1452278.6 |########################################
  1452646.3 |
  1453013.9 |
  1453381.6 |
  1453749.2 |
  1454116.9 |####################
  1454484.5 |
  1454852.2 |
  1455219.8 |
  1455587.5 |
  1455955.1 |####################
  1456322.8 |
  1456690.4 |
  1457058.1 |
  1457425.7 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1455288.8-1456970.4 ns)
  1455288.8 |########################################
  1455372.9 |
  1455457.0 |
  1455541.0 |
  1455625.1 |########################################
  1455709.2 |
  1455793.3 |
  1455877.4 |
  1455961.4 |
  1456045.5 |
  1456129.6 |########################################
  1456213.7 |
  1456297.8 |
  1456381.8 |
  1456465.9 |########################################
  1456550.0 |
  1456634.1 |
  1456718.2 |
  1456802.2 |########################################
  1456886.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1451876.2-1459416.9 ns)
  1451876.2 |####################
  1452253.2 |
  1452630.3 |
  1453007.3 |
  1453384.3 |
  1453761.4 |
  1454138.4 |
  1454515.4 |
  1454892.5 |
  1455269.5 |########################################
  1455646.5 |
  1456023.6 |
  1456400.6 |####################
  1456777.6 |
  1457154.7 |
  1457531.7 |
  1457908.7 |
  1458285.8 |
  1458662.8 |####################
  1459039.8 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1458614.2-1482517.9 ns)
  1458614.2 |####################
  1459809.4 |
  1461004.6 |####################
  1462199.8 |####################
  1463394.9 |########################################
  1464590.1 |
  1465785.3 |
  1466980.5 |
  1468175.7 |
  1469370.9 |
  1470566.0 |
  1471761.2 |
  1472956.4 |
  1474151.6 |
  1475346.8 |
  1476542.0 |
  1477737.2 |
  1478932.3 |
  1480127.5 |
  1481322.7 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 1455514.2-1465136.2 ns)
  1455514.2 |####################
  1455995.3 |
  1456476.4 |
  1456957.5 |
  1457438.6 |
  1457919.7 |
  1458400.8 |
  1458881.9 |
  1459363.0 |####################
  1459844.1 |
  1460325.2 |
  1460806.3 |
  1461287.4 |####################
  1461768.5 |########################################
  1462249.6 |
  1462730.7 |
  1463211.8 |
  1463692.9 |
  1464174.0 |
  1464655.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 1452912.5-1460126.7 ns)
  1452912.5 |########################################
  1453273.2 |########################################
  1453633.9 |
  1453994.6 |
  1454355.3 |
  1454716.1 |
  1455076.8 |
  1455437.5 |########################################
  1455798.2 |########################################
  1456158.9 |
  1456519.6 |
  1456880.3 |
  1457241.0 |
  1457601.7 |
  1457962.4 |
  1458323.1 |
  1458683.9 |
  1459044.6 |
  1459405.3 |
  1459766.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 1457225.4-1468959.1 ns)
  1457225.4 |########################################
  1457812.1 |
  1458398.8 |
  1458985.5 |
  1459572.1 |
  1460158.8 |########################################
  1460745.5 |
  1461332.2 |
  1461918.9 |
  1462505.6 |
  1463092.3 |
  1463679.0 |
  1464265.6 |########################################
  1464852.3 |########################################
  1465439.0 |
  1466025.7 |
  1466612.4 |########################################
  1467199.1 |
  1467785.8 |
  1468372.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1442919.6-1448806.2 ns)
  1442919.6 |########################################
  1443213.9 |
  1443508.3 |
  1443802.6 |
  1444096.9 |########################################
  1444391.3 |
  1444685.6 |
  1444979.9 |########################################
  1445274.3 |
  1445568.6 |
  1445862.9 |
  1446157.3 |
  1446451.6 |########################################
  1446745.9 |
  1447040.3 |
  1447334.6 |
  1447628.9 |
  1447923.3 |########################################
  1448217.6 |
  1448511.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=2543.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=318.4% of algo (FFI overhead may distort results)
