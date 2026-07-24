# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_leaf_null_entry dominates: 31792% faster than the next best (abi_boundary_w_leaf_soa_dispatch)

abi_boundary_w_leaf_null_entry (2.56 us) leads abi_boundary_w_leaf_soa_dispatch (816.16 us) by 31792%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_per_w is an outlier: 566.7x slower than the field

abi_boundary_w_leaf_scalar_per_w (1.45 ms) is 566.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_zig_runtime_w shows alternating (throttle bounce) (autocorr -0.73)

abi_boundary_w_leaf_zig_runtime_w's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_per_w} (31792% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_per_w} with a 31792% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 566.7x the fastest

Fastest abi_boundary_w_leaf_null_entry (2.56 us) to slowest abi_boundary_w_leaf_scalar_per_w (1.45 ms): 566.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 2559.1 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 566.67x (fastest 2559.1 ns, slowest 1450185.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4802ns | 4845ns | 4592ns | 4796ns | 4917ns | -99.67% |
| abi_boundary_w_leaf_scalar_anchor | 1450816ns | 1448936ns | 1447781ns | 1448802ns | 1455355ns | -0.40% |
| abi_boundary_w_leaf_scalar_dispatch | 1451344ns | 1451251ns | 1444578ns | 1450802ns | 1455539ns | -0.37% |
| abi_boundary_w_leaf_scalar_per_w | 1451363ns | 1452559ns | 1445897ns | 1451248ns | 1454269ns | -0.37% |
| abi_boundary_w_leaf_scalar_runtime_w | 1456695ns | 1451687ns | 1447677ns | 1450868ns | 1469946ns | base |
| abi_boundary_w_leaf_soa_dispatch | 818819ns | 818491ns | 816988ns | 818298ns | 820515ns | -43.79% |
| abi_boundary_w_leaf_soa_per_w | 818660ns | 818839ns | 816832ns | 818338ns | 820056ns | -43.80% |
| abi_boundary_w_leaf_soa_runtime_w | 819783ns | 818589ns | 816988ns | 818373ns | 823296ns | -43.72% |
| abi_boundary_w_leaf_zig_runtime_w | 1436838ns | 1435877ns | 1432189ns | 1435256ns | 1441535ns | -1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 2524ns | 2426ns | 2569ns | -99.83% | 0.025 |
| abi_boundary_w_leaf_scalar_anchor | 1448377ns | 1445358ns | 1452835ns | -0.40% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1448970ns | 1442288ns | 1453116ns | -0.36% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1448976ns | 1443545ns | 1451856ns | -0.36% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1454197ns | 1445330ns | 1467208ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 816491ns | 814735ns | 818130ns | -43.85% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 816363ns | 814381ns | 817819ns | -43.86% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 817386ns | 814665ns | 820855ns | -43.79% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1434298ns | 1429681ns | 1438970ns | -1.37% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 27402.6 | 2747.3 | 2524.4 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 34600.5 | 1449436.2 | 1448376.7 | n/a |
| abi_boundary_w_leaf_scalar_dispatch | 33486.8 | 1449492.2 | 1448969.9 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 34582.1 | 1449976.9 | 1448975.8 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 34918.3 | 1454753.7 | 1454196.5 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 30474.4 | 816376.5 | 816490.9 | n/a |
| abi_boundary_w_leaf_soa_per_w | 30553.2 | 816198.3 | 816362.6 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 30841.7 | 816790.1 | 817386.1 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 174672.5 | 1433504.6 | 1434297.8 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.025 | 94.8% |
| abi_boundary_w_leaf_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_leaf_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4802ns | 4802ns | -99.67% |
| abi_boundary_w_leaf_scalar_anchor | 1450816ns | 1450816ns | -0.40% |
| abi_boundary_w_leaf_scalar_dispatch | 1451344ns | 1451344ns | -0.37% |
| abi_boundary_w_leaf_scalar_per_w | 1451363ns | 1451363ns | -0.37% |
| abi_boundary_w_leaf_scalar_runtime_w | 1456695ns | 1456695ns | base |
| abi_boundary_w_leaf_soa_dispatch | 818819ns | 818819ns | -43.79% |
| abi_boundary_w_leaf_soa_per_w | 818660ns | 818660ns | -43.80% |
| abi_boundary_w_leaf_soa_runtime_w | 819783ns | 819783ns | -43.72% |
| abi_boundary_w_leaf_zig_runtime_w | 1436838ns | 1436838ns | -1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1449258ns | base | --- | [1446124, 1467208] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 2559ns | -1446766.4ns (-99.8%) | [-1464693, -1443557]ns | [2445, 2569] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1446559ns | no significant difference | [-14373, +434]ns | [1445737, 1452835] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1448931ns | no significant difference | [-16949, +3358]ns | [1444862, 1453116] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1450185ns | no significant difference | [-19505, +4257]ns | [1444886, 1451856] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 816162ns | -632747.9ns (-43.7%) | [-651577, -628792]ns | [815181, 818130] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 816565ns | -633738.5ns (-43.7%) | [-649849, -629915]ns | [814704, 817819] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 816246ns | -634167.5ns (-43.8%) | [-647902, -628362]ns | [815058, 820855] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1433349ns | -16875.2ns (-1.2%) | [-32374, -10447]ns | [1430574, 1438970] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1446918ns | -99.8% | +0.0% | +0.0% | -0.0% | -43.5% | -43.5% | -43.7% | -1.1% |
| 2 | 1453121ns | -99.8% | -0.4% | -0.3% | -0.0% | -43.8% | -43.7% | -43.8% | -1.2% |
| 3 | 1451533ns | -99.8% | -0.4% | +0.2% | -0.6% | -43.8% | -43.8% | -43.8% | -1.4% |
| 4 | 1446983ns | -99.8% | -0.1% | -0.3% | +0.3% | -43.4% | -43.7% | -43.6% | -0.4% |
| 5 | 1445330ns | -99.8% | +0.1% | +0.3% | +0.3% | -43.6% | -43.6% | -43.3% | -1.1% |
| 6 | 1481294ns | -99.8% | -1.5% | -1.9% | -2.1% | -45.0% | -44.8% | -44.5% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.123 | ok |
| abi_boundary_w_leaf_scalar_anchor | -0.035 | ok |
| abi_boundary_w_leaf_scalar_dispatch | -0.451 | moderate- |
| abi_boundary_w_leaf_scalar_per_w | -0.658 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_runtime_w | -0.158 | ok |
| abi_boundary_w_leaf_soa_dispatch | -0.220 | moderate- |
| abi_boundary_w_leaf_soa_per_w | 0.306 | moderate+ |
| abi_boundary_w_leaf_soa_runtime_w | 0.324 | moderate+ |
| abi_boundary_w_leaf_zig_runtime_w | -0.732 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 4/6, lost 0/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_leaf_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 114346.0ns | 2524.4ns | 4529.7% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4381217.4ns | 1448376.7ns | 302.5% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4382356.3ns | 1448969.9ns | 302.4% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4385295.9ns | 1448975.8ns | 302.6% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4401062.9ns | 1454196.5ns | 302.6% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 2481930.1ns | 816490.9ns | 304.0% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 2481456.0ns | 816362.6ns | 304.0% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 2483431.2ns | 817386.1ns | 303.8% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4543766.9ns | 1434297.8ns | 316.8% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 2425.8-2569.2 ns)
   2425.8 |####################
   2433.0 |
   2440.1 |
   2447.3 |
   2454.5 |
   2461.7 |####################
   2468.8 |
   2476.0 |
   2483.2 |
   2490.3 |
   2497.5 |
   2504.7 |
   2511.8 |
   2519.0 |
   2526.2 |
   2533.3 |
   2540.5 |
   2547.7 |
   2554.9 |########################################
   2562.0 |####################
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1445358.3-1452834.6 ns)
  1445358.3 |####################
  1445732.1 |
  1446105.9 |########################################
  1446479.7 |
  1446853.6 |########################################
  1447227.4 |
  1447601.2 |
  1447975.0 |
  1448348.8 |
  1448722.6 |
  1449096.5 |
  1449470.3 |
  1449844.1 |
  1450217.9 |
  1450591.7 |
  1450965.5 |
  1451339.3 |
  1451713.2 |
  1452087.0 |
  1452460.8 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1442288.3-1453116.2 ns)
  1442288.3 |########################################
  1442829.7 |
  1443371.1 |
  1443912.5 |
  1444453.9 |
  1444995.3 |
  1445536.7 |
  1446078.1 |
  1446619.5 |
  1447160.9 |########################################
  1447702.3 |########################################
  1448243.7 |
  1448785.1 |
  1449326.5 |########################################
  1449867.9 |
  1450409.3 |
  1450950.7 |
  1451492.1 |
  1452033.5 |########################################
  1452574.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1443545.4-1451855.8 ns)
  1443545.4 |########################################
  1443960.9 |
  1444376.4 |
  1444792.0 |
  1445207.5 |
  1445623.0 |
  1446038.5 |########################################
  1446454.0 |
  1446869.6 |
  1447285.1 |
  1447700.6 |
  1448116.1 |
  1448531.6 |
  1448947.2 |
  1449362.7 |
  1449778.2 |########################################
  1450193.7 |########################################
  1450609.2 |########################################
  1451024.8 |
  1451440.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1445330.4-1467207.5 ns)
  1445330.4 |####################
  1446424.3 |########################################
  1447518.1 |
  1448612.0 |
  1449705.8 |
  1450799.7 |####################
  1451893.5 |
  1452987.4 |####################
  1454081.2 |
  1455175.1 |
  1456268.9 |
  1457362.8 |
  1458456.7 |
  1459550.5 |
  1460644.4 |
  1461738.2 |
  1462832.1 |
  1463925.9 |
  1465019.8 |
  1466113.6 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 814735.4-818129.6 ns)
  814735.4 |########################################
  814905.1 |
  815074.8 |
  815244.5 |
  815414.2 |
  815583.9 |########################################
  815753.7 |########################################
  815923.4 |
  816093.1 |
  816262.8 |
  816432.5 |########################################
  816602.2 |
  816771.9 |
  816941.6 |
  817111.3 |########################################
  817281.1 |
  817450.8 |
  817620.5 |
  817790.2 |
  817959.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 814381.2-817818.9 ns)
  814381.2 |########################################
  814553.1 |
  814725.0 |
  814896.9 |########################################
  815068.8 |
  815240.6 |
  815412.5 |
  815584.4 |
  815756.3 |########################################
  815928.2 |
  816100.1 |
  816272.0 |
  816443.8 |
  816615.7 |
  816787.6 |
  816959.5 |
  817131.4 |########################################
  817303.3 |########################################
  817475.2 |
  817647.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 814665.4-820854.6 ns)
  814665.4 |########################################
  814974.9 |
  815284.3 |########################################
  815593.8 |########################################
  815903.2 |
  816212.7 |
  816522.1 |########################################
  816831.6 |
  817141.1 |
  817450.5 |
  817760.0 |
  818069.4 |
  818378.9 |
  818688.3 |
  818997.8 |
  819307.3 |
  819616.7 |########################################
  819926.2 |
  820235.6 |
  820545.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1429681.2-1438970.2 ns)
  1429681.2 |########################################
  1430145.6 |
  1430610.1 |
  1431074.6 |########################################
  1431539.0 |########################################
  1432003.4 |
  1432467.9 |
  1432932.4 |
  1433396.8 |
  1433861.2 |
  1434325.7 |
  1434790.2 |########################################
  1435254.6 |
  1435719.1 |
  1436183.5 |########################################
  1436648.0 |
  1437112.4 |
  1437576.9 |
  1438041.3 |
  1438505.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=4446.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=316.9% of algo (FFI overhead may distort results)
