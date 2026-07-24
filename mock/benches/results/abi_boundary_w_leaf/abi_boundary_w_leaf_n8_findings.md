# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_leaf_null_entry dominates: 27022% faster than the next best (abi_boundary_w_leaf_soa_per_w)

abi_boundary_w_leaf_null_entry (3.02 us) leads abi_boundary_w_leaf_soa_per_w (819.76 us) by 27022%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_per_w is an outlier: 482.3x slower than the field

abi_boundary_w_leaf_scalar_per_w (1.46 ms) is 482.3x the fastest (3.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_scalar_runtime_w shows alternating (throttle bounce) (autocorr -0.70)

abi_boundary_w_leaf_scalar_runtime_w's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_per_w} (27022% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_runtime_w, abi_boundary_w_leaf_scalar_per_w} with a 27022% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 482.3x the fastest

Fastest abi_boundary_w_leaf_null_entry (3.02 us) to slowest abi_boundary_w_leaf_scalar_per_w (1.46 ms): 482.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 3022.5 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 482.34x (fastest 3022.5 ns, slowest 1457877.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5298ns | 5261ns | 5124ns | 5228ns | 5492ns | -99.64% |
| abi_boundary_w_leaf_scalar_anchor | 1457937ns | 1458027ns | 1454811ns | 1457277ns | 1460490ns | -0.18% |
| abi_boundary_w_leaf_scalar_dispatch | 1458782ns | 1457168ns | 1453652ns | 1456286ns | 1465092ns | -0.13% |
| abi_boundary_w_leaf_scalar_per_w | 1459570ns | 1460494ns | 1454600ns | 1458863ns | 1463115ns | -0.07% |
| abi_boundary_w_leaf_scalar_runtime_w | 1460622ns | 1459860ns | 1453943ns | 1459580ns | 1465525ns | base |
| abi_boundary_w_leaf_soa_dispatch | 827072ns | 826641ns | 824996ns | 826298ns | 829269ns | -43.38% |
| abi_boundary_w_leaf_soa_per_w | 821983ns | 822351ns | 818295ns | 821579ns | 824431ns | -43.72% |
| abi_boundary_w_leaf_soa_runtime_w | 826574ns | 825205ns | 821838ns | 824658ns | 831815ns | -43.41% |
| abi_boundary_w_leaf_zig_runtime_w | 1443177ns | 1442352ns | 1440410ns | 1441928ns | 1446433ns | -1.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 3043ns | 2930ns | 3161ns | -99.79% | 0.003 |
| abi_boundary_w_leaf_scalar_anchor | 1455227ns | 1452008ns | 1457717ns | -0.18% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1455964ns | 1450778ns | 1462093ns | -0.13% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1456871ns | 1452012ns | 1460267ns | -0.07% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1457877ns | 1451324ns | 1462630ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 824614ns | 822588ns | 826834ns | -43.44% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 819449ns | 815837ns | 821849ns | -43.79% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 823991ns | 819526ns | 828956ns | -43.48% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1440305ns | 1437596ns | 1443436ns | -1.21% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 26943.6 | 3142.5 | 3043.0 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 46200.4 | 1455439.7 | 1455226.8 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 46581.0 | 1456078.4 | 1455964.1 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 45215.9 | 1456236.0 | 1456871.0 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 46037.5 | 1458482.8 | 1457876.9 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 35454.0 | 825148.3 | 824614.4 | n/a |
| abi_boundary_w_leaf_soa_per_w | 39287.2 | 820660.6 | 819448.8 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 40904.5 | 824835.4 | 823991.4 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 191106.9 | 1439152.1 | 1440305.1 | 8 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.003 | 96.9% |
| abi_boundary_w_leaf_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_leaf_soa_dispatch | 0.000 | 0.4% |
| abi_boundary_w_leaf_soa_per_w | 0.000 | 0.4% |
| abi_boundary_w_leaf_soa_runtime_w | 0.000 | 0.4% |
| abi_boundary_w_leaf_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5298ns | 5298ns | -99.64% |
| abi_boundary_w_leaf_scalar_anchor | 1457937ns | 1457937ns | -0.18% |
| abi_boundary_w_leaf_scalar_dispatch | 1458782ns | 1458782ns | -0.13% |
| abi_boundary_w_leaf_scalar_per_w | 1459570ns | 1459570ns | -0.07% |
| abi_boundary_w_leaf_scalar_runtime_w | 1460622ns | 1460622ns | base |
| abi_boundary_w_leaf_soa_dispatch | 827072ns | 827072ns | -43.38% |
| abi_boundary_w_leaf_soa_per_w | 821983ns | 821983ns | -43.72% |
| abi_boundary_w_leaf_soa_runtime_w | 826574ns | 826574ns | -43.41% |
| abi_boundary_w_leaf_zig_runtime_w | 1443177ns | 1443177ns | -1.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1457140ns | base | --- | [1453860, 1462630] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 3022ns | -1454030.0ns (-99.8%) | [-1459630, -1450842]ns | [2945, 3161] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1455336ns | no significant difference | [-5968, +1240]ns | [1452627, 1457717] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1454412ns | no significant difference | [-10372, +7930]ns | [1451388, 1462093] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1457877ns | no significant difference | [-7030, +3297]ns | [1452469, 1460267] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 824114ns | -633618.9ns (-43.5%) | [-636393, -629775]ns | [822895, 826834] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 819758ns | -636208.5ns (-43.7%) | [-644339, -634736]ns | [816739, 821849] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 822644ns | -633700.7ns (-43.5%) | [-640554, -627402]ns | [820374, 828956] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1439585ns | -17291.0ns (-1.2%) | [-20415, -15009]ns | [1437895, 1443436] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1457002ns | -99.8% | -0.3% | +0.2% | +0.3% | -43.5% | -43.7% | -42.8% | -1.1% |
| 2 | 1459875ns | -99.8% | -0.2% | -0.4% | -0.0% | -43.4% | -44.0% | -43.7% | -1.1% |
| 3 | 1451324ns | -99.8% | +0.3% | +0.9% | +0.1% | -43.2% | -43.8% | -43.5% | -0.9% |
| 4 | 1465386ns | -99.8% | -0.5% | -1.0% | -0.6% | -43.5% | -44.1% | -43.8% | -1.5% |
| 5 | 1456396ns | -99.8% | -0.3% | -0.3% | +0.2% | -43.5% | -43.5% | -43.4% | -1.2% |
| 6 | 1457278ns | -99.8% | -0.2% | -0.2% | -0.4% | -43.6% | -43.6% | -43.6% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.280 | moderate+ |
| abi_boundary_w_leaf_scalar_anchor | -0.244 | moderate- |
| abi_boundary_w_leaf_scalar_dispatch | -0.351 | moderate- |
| abi_boundary_w_leaf_scalar_per_w | -0.156 | ok |
| abi_boundary_w_leaf_scalar_runtime_w | -0.697 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_soa_dispatch | -0.095 | ok |
| abi_boundary_w_leaf_soa_per_w | 0.342 | moderate+ |
| abi_boundary_w_leaf_soa_runtime_w | -0.079 | ok |
| abi_boundary_w_leaf_zig_runtime_w | -0.514 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 5/6, lost 1/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 4/6, lost 2/6
- **abi_boundary_w_leaf_scalar_per_w**: won 2/6, lost 3/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 119275.6ns | 3043.0ns | 3919.6% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4414047.1ns | 1455226.8ns | 303.3% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4415398.7ns | 1455964.1ns | 303.3% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4417404.9ns | 1456871.0ns | 303.2% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4423668.9ns | 1457876.9ns | 303.4% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 2513553.3ns | 824614.4ns | 304.8% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 2500121.1ns | 819448.8ns | 305.1% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 2516631.4ns | 823991.4ns | 305.4% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4579359.2ns | 1440305.1ns | 317.9% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 2930.0-3161.4 ns)
   2930.0 |########################################
   2941.6 |
   2953.1 |########################################
   2964.7 |
   2976.3 |
   2987.9 |
   2999.4 |########################################
   3011.0 |
   3022.6 |
   3034.2 |########################################
   3045.7 |
   3057.3 |
   3068.9 |
   3080.4 |
   3092.0 |
   3103.6 |########################################
   3115.2 |
   3126.7 |
   3138.3 |
   3149.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1452007.5-1457717.3 ns)
  1452007.5 |########################################
  1452293.0 |
  1452578.5 |
  1452864.0 |
  1453149.5 |########################################
  1453434.9 |
  1453720.4 |
  1454005.9 |
  1454291.4 |
  1454576.9 |########################################
  1454862.4 |
  1455147.9 |
  1455433.4 |
  1455718.9 |
  1456004.4 |########################################
  1456289.8 |
  1456575.3 |
  1456860.8 |
  1457146.3 |
  1457431.8 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1450777.9-1462092.9 ns)
  1450777.9 |########################################
  1451343.7 |
  1451909.4 |########################################
  1452475.2 |
  1453040.9 |
  1453606.7 |########################################
  1454172.4 |
  1454738.2 |########################################
  1455303.9 |
  1455869.7 |
  1456435.4 |
  1457001.2 |
  1457566.9 |
  1458132.7 |
  1458698.4 |
  1459264.2 |########################################
  1459829.9 |
  1460395.7 |
  1460961.4 |
  1461527.2 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1452011.7-1460267.3 ns)
  1452011.7 |########################################
  1452424.5 |
  1452837.3 |########################################
  1453250.0 |
  1453662.8 |
  1454075.6 |
  1454488.4 |
  1454901.2 |
  1455313.9 |
  1455726.7 |
  1456139.5 |
  1456552.3 |########################################
  1456965.1 |
  1457377.8 |
  1457790.6 |
  1458203.4 |
  1458616.2 |
  1459029.0 |########################################
  1459441.7 |########################################
  1459854.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1451324.2-1462630.4 ns)
  1451324.2 |####################
  1451889.5 |
  1452454.8 |
  1453020.1 |
  1453585.4 |
  1454150.8 |
  1454716.1 |
  1455281.4 |
  1455846.7 |####################
  1456412.0 |
  1456977.3 |########################################
  1457542.6 |
  1458107.9 |
  1458673.2 |
  1459238.5 |
  1459803.8 |####################
  1460369.2 |
  1460934.5 |
  1461499.8 |
  1462065.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 822587.9-826834.2 ns)
  822587.9 |########################################
  822800.2 |
  823012.5 |########################################
  823224.8 |########################################
  823437.2 |
  823649.5 |
  823861.8 |
  824074.1 |
  824286.4 |
  824498.7 |
  824711.1 |
  824923.4 |########################################
  825135.7 |
  825348.0 |
  825560.3 |
  825772.6 |
  825984.9 |
  826197.3 |########################################
  826409.6 |
  826621.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 815837.1-821849.2 ns)
  815837.1 |########################################
  816137.7 |
  816438.3 |
  816738.9 |
  817039.5 |
  817340.1 |
  817640.7 |########################################
  817941.3 |
  818241.9 |
  818542.5 |
  818843.1 |########################################
  819143.7 |
  819444.3 |
  819744.9 |
  820045.5 |
  820346.1 |########################################
  820646.7 |
  820947.3 |
  821247.9 |########################################
  821548.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 819525.8-828955.7 ns)
  819525.8 |########################################
  819997.3 |
  820468.8 |
  820940.3 |########################################
  821411.8 |
  821883.3 |
  822354.8 |########################################
  822826.2 |########################################
  823297.7 |
  823769.2 |########################################
  824240.7 |
  824712.2 |
  825183.7 |
  825655.2 |
  826126.7 |
  826598.2 |
  827069.7 |
  827541.2 |
  828012.7 |
  828484.2 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1437596.2-1443435.6 ns)
  1437596.2 |####################
  1437888.2 |
  1438180.1 |########################################
  1438472.1 |
  1438764.1 |
  1439056.1 |
  1439348.0 |
  1439640.0 |
  1439932.0 |
  1440223.9 |
  1440515.9 |####################
  1440807.9 |
  1441099.8 |
  1441391.8 |
  1441683.8 |
  1441975.8 |
  1442267.7 |
  1442559.7 |
  1442851.7 |
  1443143.6 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=3957.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=304.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=305.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=305.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=317.8% of algo (FFI overhead may distort results)
