# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_leaf_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_leaf_scalar_runtime_w has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_leaf_null_entry at 2.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_leaf_null_entry dominates: 32205% faster than the next best (abi_boundary_w_leaf_soa_runtime_w)

abi_boundary_w_leaf_null_entry (2.54 us) leads abi_boundary_w_leaf_soa_runtime_w (820.69 us) by 32205%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_runtime_w is an outlier: 573.8x slower than the field

abi_boundary_w_leaf_scalar_runtime_w (1.46 ms) is 573.8x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_null_entry shows alternating (throttle bounce) (autocorr -0.87)

abi_boundary_w_leaf_null_entry's per-pass series has lag-1 autocorrelation -0.87, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w} (32205% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_runtime_w} with a 32205% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 573.8x the fastest

Fastest abi_boundary_w_leaf_null_entry (2.54 us) to slowest abi_boundary_w_leaf_scalar_runtime_w (1.46 ms): 573.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 2540.4 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 573.83x (fastest 2540.4 ns, slowest 1457763.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4819ns | 4830ns | 4672ns | 4814ns | 4899ns | -99.67% |
| abi_boundary_w_leaf_scalar_anchor | 1459615ns | 1457169ns | 1454280ns | 1456863ns | 1466410ns | -0.01% |
| abi_boundary_w_leaf_scalar_dispatch | 1459001ns | 1460438ns | 1454282ns | 1458830ns | 1461615ns | -0.05% |
| abi_boundary_w_leaf_scalar_per_w | 1455484ns | 1452937ns | 1450054ns | 1452077ns | 1463310ns | -0.29% |
| abi_boundary_w_leaf_scalar_runtime_w | 1459724ns | 1460582ns | 1454893ns | 1460213ns | 1461407ns | base |
| abi_boundary_w_leaf_soa_dispatch | 827311ns | 825954ns | 824643ns | 825519ns | 831332ns | -43.32% |
| abi_boundary_w_leaf_soa_per_w | 823278ns | 823535ns | 819738ns | 822426ns | 826327ns | -43.60% |
| abi_boundary_w_leaf_soa_runtime_w | 822824ns | 823157ns | 820373ns | 822631ns | 824340ns | -43.63% |
| abi_boundary_w_leaf_zig_runtime_w | 1442643ns | 1441939ns | 1440325ns | 1441417ns | 1445643ns | -1.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 2542ns | 2483ns | 2592ns | -99.83% | 0.006 |
| abi_boundary_w_leaf_scalar_anchor | 1456849ns | 1451662ns | 1463568ns | -0.01% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1456278ns | 1451536ns | 1458815ns | -0.05% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1452877ns | 1447605ns | 1460547ns | -0.28% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1456977ns | 1452150ns | 1458722ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 824753ns | 822169ns | 828759ns | -43.39% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 820806ns | 817307ns | 823816ns | -43.66% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 820355ns | 817960ns | 821830ns | -43.69% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1439730ns | 1437430ns | 1442649ns | -1.18% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 27595.3 | 2669.4 | 2541.7 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 46717.8 | 1460233.1 | 1456848.7 | 13 |
| abi_boundary_w_leaf_scalar_dispatch | 44789.7 | 1455406.7 | 1456277.8 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 43657.4 | 1454762.7 | 1452876.6 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 45108.2 | 1455506.4 | 1456977.3 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 36213.8 | 824030.2 | 824753.3 | n/a |
| abi_boundary_w_leaf_soa_per_w | 36457.2 | 821607.2 | 820805.9 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 36035.5 | 821864.8 | 820355.2 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 195517.2 | 1440157.9 | 1439730.0 | 9 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.006 | 97.8% |
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
| abi_boundary_w_leaf_null_entry | 4819ns | 4819ns | -99.67% |
| abi_boundary_w_leaf_scalar_anchor | 1459615ns | 1459615ns | -0.01% |
| abi_boundary_w_leaf_scalar_dispatch | 1459001ns | 1459001ns | -0.05% |
| abi_boundary_w_leaf_scalar_per_w | 1455484ns | 1455484ns | -0.29% |
| abi_boundary_w_leaf_scalar_runtime_w | 1459724ns | 1459724ns | base |
| abi_boundary_w_leaf_soa_dispatch | 827311ns | 827311ns | -43.32% |
| abi_boundary_w_leaf_soa_per_w | 823278ns | 823278ns | -43.60% |
| abi_boundary_w_leaf_soa_runtime_w | 822824ns | 822824ns | -43.63% |
| abi_boundary_w_leaf_zig_runtime_w | 1442643ns | 1442643ns | -1.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1457764ns | base | --- | [1454446, 1458722] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 2540ns | -1455211.4ns (-99.8%) | [-1456174, -1451922]ns | [2493, 2592] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1454336ns | no significant difference | [-4386, +5805]ns | [1452642, 1463568] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1457760ns | no significant difference | [-2692, +858]ns | [1452258, 1458815] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1450336ns | no significant difference | [-10069, +2784]ns | [1447746, 1460547] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 823310ns | -634240.8ns (-43.5%) | [-636531, -625900]ns | [822191, 828759] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 821041ns | -634735.8ns (-43.5%) | [-640374, -633405]ns | [817560, 823816] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 820688ns | -637840.2ns (-43.8%) | [-639197, -632830]ns | [818548, 821830] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1439014ns | -17175.8ns (-1.2%) | [-20430, -14136]ns | [1437527, 1442649] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1458745ns | -99.8% | -0.3% | +0.0% | -0.6% | -43.6% | -43.5% | -43.8% | -1.4% |
| 2 | 1457169ns | -99.8% | +0.1% | +0.1% | -0.0% | -43.3% | -43.9% | -43.6% | -1.4% |
| 3 | 1452150ns | -99.8% | -0.0% | -0.0% | -0.1% | -42.7% | -43.6% | -43.4% | -1.0% |
| 4 | 1458698ns | -99.8% | -0.3% | -0.1% | -0.8% | -43.6% | -43.9% | -43.7% | -1.3% |
| 5 | 1458358ns | -99.8% | +0.7% | +0.0% | +0.4% | -43.5% | -43.5% | -43.7% | -1.0% |
| 6 | 1456743ns | -99.8% | -0.2% | -0.3% | -0.6% | -43.5% | -43.5% | -43.9% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.868 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_anchor | -0.354 | moderate- |
| abi_boundary_w_leaf_scalar_dispatch | -0.280 | moderate- |
| abi_boundary_w_leaf_scalar_per_w | -0.595 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_runtime_w | -0.218 | moderate- |
| abi_boundary_w_leaf_soa_dispatch | -0.138 | ok |
| abi_boundary_w_leaf_soa_per_w | -0.155 | ok |
| abi_boundary_w_leaf_soa_runtime_w | 0.027 | ok |
| abi_boundary_w_leaf_zig_runtime_w | 0.432 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 3/6, lost 1/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 2/6, lost 1/6
- **abi_boundary_w_leaf_scalar_per_w**: won 3/6, lost 1/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 118713.5ns | 2541.7ns | 4670.6% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4434343.4ns | 1456848.7ns | 304.4% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4414544.4ns | 1456277.8ns | 303.1% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4406902.4ns | 1452876.6ns | 303.3% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4416010.3ns | 1456977.3ns | 303.1% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 2509640.6ns | 824753.3ns | 304.3% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 2501965.5ns | 820805.9ns | 304.8% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 2502116.6ns | 820355.2ns | 305.0% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4587816.8ns | 1439730.0ns | 318.7% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 2483.3-2591.9 ns)
   2483.3 |########################################
   2488.7 |
   2494.2 |
   2499.6 |########################################
   2505.0 |
   2510.5 |########################################
   2515.9 |
   2521.3 |
   2526.7 |
   2532.2 |
   2537.6 |
   2543.0 |
   2548.5 |
   2553.9 |
   2559.3 |
   2564.8 |########################################
   2570.2 |
   2575.6 |
   2581.0 |
   2586.5 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1451662.1-1463568.3 ns)
  1451662.1 |####################
  1452257.4 |
  1452852.7 |
  1453448.0 |########################################
  1454043.3 |
  1454638.7 |####################
  1455234.0 |
  1455829.3 |
  1456424.6 |
  1457019.9 |
  1457615.2 |####################
  1458210.5 |
  1458805.8 |
  1459401.1 |
  1459996.4 |
  1460591.8 |
  1461187.1 |
  1461782.4 |
  1462377.7 |
  1462973.0 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1451536.2-1458814.8 ns)
  1451536.2 |########################################
  1451900.1 |
  1452264.1 |
  1452628.0 |########################################
  1452991.9 |
  1453355.9 |
  1453719.8 |
  1454083.7 |
  1454447.6 |
  1454811.6 |
  1455175.5 |
  1455539.4 |
  1455903.4 |
  1456267.3 |
  1456631.2 |
  1456995.1 |########################################
  1457359.1 |
  1457723.0 |
  1458086.9 |########################################
  1458450.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1447605.0-1460547.4 ns)
  1447605.0 |########################################
  1448252.1 |
  1448899.2 |
  1449546.4 |####################
  1450193.5 |
  1450840.6 |####################
  1451487.7 |
  1452134.9 |
  1452782.0 |
  1453429.1 |
  1454076.2 |
  1454723.3 |
  1455370.5 |
  1456017.6 |####################
  1456664.7 |
  1457311.8 |
  1457959.0 |
  1458606.1 |
  1459253.2 |
  1459900.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1452149.6-1458721.9 ns)
  1452149.6 |########################################
  1452478.2 |
  1452806.8 |
  1453135.4 |
  1453464.1 |
  1453792.7 |
  1454121.3 |
  1454449.9 |
  1454778.5 |
  1455107.1 |
  1455435.7 |
  1455764.3 |
  1456093.0 |
  1456421.6 |########################################
  1456750.2 |
  1457078.8 |########################################
  1457407.4 |
  1457736.0 |
  1458064.6 |########################################
  1458393.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 822169.2-828758.9 ns)
  822169.2 |########################################
  822498.7 |####################
  822828.2 |
  823157.7 |
  823487.1 |
  823816.6 |####################
  824146.1 |
  824475.6 |
  824805.1 |
  825134.6 |
  825464.1 |####################
  825793.6 |
  826123.0 |
  826452.5 |
  826782.0 |
  827111.5 |
  827441.0 |
  827770.5 |
  828100.0 |
  828429.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 817307.1-823816.0 ns)
  817307.1 |########################################
  817632.5 |########################################
  817958.0 |
  818283.4 |
  818608.9 |
  818934.3 |
  819259.8 |########################################
  819585.2 |
  819910.7 |
  820236.1 |
  820561.6 |
  820887.0 |
  821212.4 |
  821537.9 |
  821863.3 |
  822188.8 |
  822514.2 |########################################
  822839.7 |
  823165.1 |########################################
  823490.6 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 817959.6-821829.6 ns)
  817959.6 |########################################
  818153.1 |
  818346.6 |
  818540.1 |
  818733.6 |
  818927.1 |
  819120.6 |########################################
  819314.1 |
  819507.6 |
  819701.1 |
  819894.6 |
  820088.1 |
  820281.6 |########################################
  820475.1 |
  820668.6 |
  820862.1 |########################################
  821055.6 |
  821249.1 |
  821442.6 |########################################
  821636.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1437430.0-1442649.1 ns)
  1437430.0 |########################################
  1437691.0 |####################
  1437951.9 |
  1438212.9 |
  1438473.8 |
  1438734.8 |
  1438995.7 |
  1439256.7 |
  1439517.7 |
  1439778.6 |
  1440039.6 |####################
  1440300.5 |
  1440561.5 |
  1440822.4 |####################
  1441083.4 |
  1441344.4 |
  1441605.3 |
  1441866.3 |
  1442127.2 |
  1442388.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=4676.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=318.7% of algo (FFI overhead may distort results)
