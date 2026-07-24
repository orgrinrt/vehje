# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_leaf_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_leaf_scalar_runtime_w has the worst median (1.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_leaf_null_entry at 2.80 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_leaf_null_entry dominates: 29000% faster than the next best (abi_boundary_w_leaf_soa_runtime_w)

abi_boundary_w_leaf_null_entry (2.80 us) leads abi_boundary_w_leaf_soa_runtime_w (814.62 us) by 29000%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_runtime_w is an outlier: 518.8x slower than the field

abi_boundary_w_leaf_scalar_runtime_w (1.45 ms) is 518.8x the fastest (2.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_scalar_dispatch shows alternating (throttle bounce) (autocorr -0.72)

abi_boundary_w_leaf_scalar_dispatch's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_runtime_w} (29000% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_runtime_w} with a 29000% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 518.8x the fastest

Fastest abi_boundary_w_leaf_null_entry (2.80 us) to slowest abi_boundary_w_leaf_scalar_runtime_w (1.45 ms): 518.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 2799.4 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 518.81x (fastest 2799.4 ns, slowest 1452319.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 5113ns | 5130ns | 4965ns | 5099ns | 5209ns | -99.65% |
| abi_boundary_w_leaf_scalar_anchor | 1452024ns | 1450288ns | 1448233ns | 1449715ns | 1457382ns | -0.25% |
| abi_boundary_w_leaf_scalar_dispatch | 1450724ns | 1450390ns | 1448615ns | 1450318ns | 1452387ns | -0.34% |
| abi_boundary_w_leaf_scalar_per_w | 1453675ns | 1453003ns | 1449713ns | 1452773ns | 1457009ns | -0.14% |
| abi_boundary_w_leaf_scalar_runtime_w | 1455716ns | 1454773ns | 1449678ns | 1453174ns | 1462549ns | base |
| abi_boundary_w_leaf_soa_dispatch | 819653ns | 818948ns | 817844ns | 818877ns | 821721ns | -43.69% |
| abi_boundary_w_leaf_soa_per_w | 820143ns | 818978ns | 818137ns | 818813ns | 823139ns | -43.66% |
| abi_boundary_w_leaf_soa_runtime_w | 817144ns | 816903ns | 816248ns | 816855ns | 818026ns | -43.87% |
| abi_boundary_w_leaf_zig_runtime_w | 1435667ns | 1434140ns | 1431821ns | 1433568ns | 1440737ns | -1.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 2789ns | 2675ns | 2860ns | -99.81% | 0.046 |
| abi_boundary_w_leaf_scalar_anchor | 1449622ns | 1446018ns | 1454852ns | -0.25% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1448292ns | 1446153ns | 1449967ns | -0.34% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1451209ns | 1447440ns | 1454440ns | -0.14% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1453257ns | 1447360ns | 1459972ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 817348ns | 815564ns | 819426ns | -43.76% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 817792ns | 815837ns | 820715ns | -43.73% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 814868ns | 813958ns | 815734ns | -43.93% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1433149ns | 1429338ns | 1438107ns | -1.38% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 27617.9 | 2856.2 | 2788.7 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 36019.2 | 1448773.1 | 1449622.4 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 34757.9 | 1447698.1 | 1448291.5 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 36883.6 | 1451201.7 | 1451208.7 | 1 |
| abi_boundary_w_leaf_scalar_runtime_w | 35621.4 | 1451429.4 | 1453257.4 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 32312.6 | 817759.7 | 817348.2 | n/a |
| abi_boundary_w_leaf_soa_per_w | 34718.8 | 817970.4 | 817791.8 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 32163.4 | 815172.5 | 814868.0 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 172406.6 | 1433190.9 | 1433148.8 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.046 | 95.6% |
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
| abi_boundary_w_leaf_null_entry | 5113ns | 5113ns | -99.65% |
| abi_boundary_w_leaf_scalar_anchor | 1452024ns | 1452024ns | -0.25% |
| abi_boundary_w_leaf_scalar_dispatch | 1450724ns | 1450724ns | -0.34% |
| abi_boundary_w_leaf_scalar_per_w | 1453675ns | 1453675ns | -0.14% |
| abi_boundary_w_leaf_scalar_runtime_w | 1455716ns | 1455716ns | base |
| abi_boundary_w_leaf_soa_dispatch | 819653ns | 819653ns | -43.69% |
| abi_boundary_w_leaf_soa_per_w | 820143ns | 820143ns | -43.66% |
| abi_boundary_w_leaf_soa_runtime_w | 817144ns | 817144ns | -43.87% |
| abi_boundary_w_leaf_zig_runtime_w | 1435667ns | 1435667ns | -1.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1452320ns | base | --- | [1447481, 1459972] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 2799ns | -1449603.1ns (-99.8%) | [-1457165, -1444638]ns | [2707, 2860] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1447863ns | no significant difference | [-8401, +383]ns | [1446152, 1454852] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1447968ns | no significant difference | [-12004, +1358]ns | [1446940, 1449967] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1450508ns | no significant difference | [-9464, +5519]ns | [1448678, 1454440] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 816606ns | -636276.4ns (-43.8%) | [-643006, -628445]ns | [816013, 819426] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 816664ns | -635846.6ns (-43.8%) | [-642629, -627921]ns | [815996, 820715] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 814624ns | -637721.2ns (-43.9%) | [-644675, -632772]ns | [814246, 815734] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1431661ns | -20527.7ns (-1.4%) | [-30294, -9504]ns | [1429678, 1438107] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1447360ns | -99.8% | +0.1% | +0.2% | +0.0% | -43.2% | -43.1% | -43.8% | -1.1% |
| 2 | 1447601ns | -99.8% | -0.1% | -0.1% | +0.2% | -43.6% | -43.6% | -43.7% | -0.5% |
| 3 | 1457018ns | -99.8% | -0.8% | -0.5% | -0.3% | -44.0% | -43.9% | -44.1% | -1.7% |
| 4 | 1447621ns | -99.8% | -0.1% | +0.0% | +0.6% | -43.7% | -43.6% | -43.7% | -0.8% |
| 5 | 1458470ns | -99.8% | -0.3% | -0.7% | -0.6% | -44.0% | -43.9% | -44.0% | -2.0% |
| 6 | 1461474ns | -99.8% | -0.4% | -0.9% | -0.7% | -44.1% | -44.1% | -44.3% | -2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.112 | ok |
| abi_boundary_w_leaf_scalar_anchor | 0.402 | moderate+ |
| abi_boundary_w_leaf_scalar_dispatch | -0.716 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_per_w | 0.079 | ok |
| abi_boundary_w_leaf_scalar_runtime_w | 0.021 | ok |
| abi_boundary_w_leaf_soa_dispatch | -0.045 | ok |
| abi_boundary_w_leaf_soa_per_w | -0.264 | moderate- |
| abi_boundary_w_leaf_soa_runtime_w | -0.289 | moderate- |
| abi_boundary_w_leaf_zig_runtime_w | -0.253 | moderate- |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 3/6, lost 1/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_leaf_scalar_per_w**: won 3/6, lost 2/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 120292.4ns | 2788.7ns | 4313.5% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4384235.3ns | 1449622.4ns | 302.4% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4379820.8ns | 1448291.5ns | 302.4% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4391453.1ns | 1451208.7ns | 302.6% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4391044.5ns | 1453257.4ns | 302.2% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 2486296.0ns | 817348.2ns | 304.2% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 2489438.7ns | 817791.8ns | 304.4% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 2478188.0ns | 814868.0ns | 304.1% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4536592.9ns | 1433148.8ns | 316.5% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 2675.4-2860.2 ns)
   2675.4 |########################################
   2684.6 |
   2693.9 |
   2703.1 |
   2712.4 |
   2721.6 |
   2730.8 |########################################
   2740.1 |
   2749.3 |
   2758.6 |
   2767.8 |
   2777.0 |########################################
   2786.3 |
   2795.5 |
   2804.8 |
   2814.0 |########################################
   2823.2 |
   2832.5 |########################################
   2841.7 |
   2851.0 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1446017.5-1454852.1 ns)
  1446017.5 |########################################
  1446459.2 |
  1446901.0 |
  1447342.7 |
  1447784.4 |
  1448226.1 |
  1448667.9 |
  1449109.6 |#############
  1449551.3 |
  1449993.1 |
  1450434.8 |
  1450876.5 |
  1451318.3 |
  1451760.0 |
  1452201.7 |
  1452643.5 |
  1453085.2 |
  1453526.9 |
  1453968.6 |#############
  1454410.4 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1446153.3-1449967.0 ns)
  1446153.3 |####################
  1446344.0 |
  1446534.7 |
  1446725.4 |
  1446916.1 |
  1447106.7 |
  1447297.4 |
  1447488.1 |
  1447678.8 |########################################
  1447869.5 |
  1448060.2 |####################
  1448250.9 |
  1448441.5 |
  1448632.2 |
  1448822.9 |
  1449013.6 |
  1449204.3 |
  1449395.0 |
  1449585.7 |
  1449776.4 |####################
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1447439.6-1454440.2 ns)
  1447439.6 |####################
  1447789.6 |
  1448139.7 |
  1448489.7 |
  1448839.7 |
  1449189.8 |
  1449539.8 |
  1449889.8 |########################################
  1450239.8 |
  1450589.9 |
  1450939.9 |####################
  1451289.9 |
  1451640.0 |
  1451990.0 |
  1452340.0 |####################
  1452690.1 |
  1453040.1 |
  1453390.1 |
  1453740.1 |
  1454090.2 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1447360.0-1459971.9 ns)
  1447360.0 |########################################
  1447990.6 |
  1448621.2 |
  1449251.8 |
  1449882.4 |
  1450513.0 |
  1451143.6 |
  1451774.2 |
  1452404.8 |
  1453035.4 |
  1453665.9 |
  1454296.5 |
  1454927.1 |
  1455557.7 |
  1456188.3 |
  1456818.9 |#############
  1457449.5 |
  1458080.1 |#############
  1458710.7 |
  1459341.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 815563.7-819426.1 ns)
  815563.7 |####################
  815756.8 |
  815949.9 |
  816143.1 |
  816336.2 |########################################
  816529.3 |####################
  816722.4 |
  816915.5 |
  817108.6 |
  817301.8 |####################
  817494.9 |
  817688.0 |
  817881.1 |
  818074.2 |
  818267.3 |
  818460.5 |
  818653.6 |
  818846.7 |
  819039.8 |
  819232.9 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 815837.1-820715.4 ns)
  815837.1 |####################
  816081.0 |########################################
  816324.9 |
  816568.9 |
  816812.8 |
  817056.7 |####################
  817300.6 |
  817544.5 |
  817788.4 |
  818032.4 |
  818276.3 |####################
  818520.2 |
  818764.1 |
  819008.0 |
  819251.9 |
  819495.9 |
  819739.8 |
  819983.7 |
  820227.6 |
  820471.5 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 813957.9-815734.2 ns)
  813957.9 |####################
  814046.7 |
  814135.5 |
  814224.3 |
  814313.2 |
  814402.0 |
  814490.8 |####################
  814579.6 |########################################
  814668.4 |
  814757.2 |
  814846.1 |
  814934.9 |
  815023.7 |####################
  815112.5 |
  815201.3 |
  815290.1 |
  815378.9 |
  815467.8 |
  815556.6 |
  815645.4 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1429338.3-1438107.3 ns)
  1429338.3 |########################################
  1429776.8 |########################################
  1430215.2 |
  1430653.7 |
  1431092.1 |########################################
  1431530.6 |########################################
  1431969.0 |
  1432407.4 |
  1432845.9 |
  1433284.4 |
  1433722.8 |
  1434161.2 |
  1434599.7 |
  1435038.2 |
  1435476.6 |
  1435915.1 |########################################
  1436353.5 |
  1436791.9 |
  1437230.4 |
  1437668.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=4321.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=316.0% of algo (FFI overhead may distort results)
