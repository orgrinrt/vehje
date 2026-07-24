# abi_boundary_w (leaf)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_leaf_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_leaf_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_leaf_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_leaf_scalar_runtime_w has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_leaf_null_entry at 4.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_leaf_null_entry dominates: 35662% faster than the next best (abi_boundary_w_leaf_zig_runtime_w)

abi_boundary_w_leaf_null_entry (4.03 us) leads abi_boundary_w_leaf_zig_runtime_w (1.44 ms) by 35662%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_leaf_null_entry beats baseline by 100% (significant)

abi_boundary_w_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_boundary_w_leaf_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_leaf_scalar_runtime_w is an outlier: 362.6x slower than the field

abi_boundary_w_leaf_scalar_runtime_w (1.46 ms) is 362.6x the fastest (4.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_leaf_soa_dispatch shows alternating (throttle bounce) (autocorr -0.59)

abi_boundary_w_leaf_soa_dispatch's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_leaf_null_entry} vs {abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_scalar_runtime_w} (35662% apart)

The field splits into a fast tier {abi_boundary_w_leaf_null_entry} and a slow tier {abi_boundary_w_leaf_zig_runtime_w, abi_boundary_w_leaf_scalar_per_w, abi_boundary_w_leaf_scalar_anchor, abi_boundary_w_leaf_scalar_dispatch, abi_boundary_w_leaf_soa_dispatch, abi_boundary_w_leaf_soa_per_w, abi_boundary_w_leaf_soa_runtime_w, abi_boundary_w_leaf_scalar_runtime_w} with a 35662% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 362.6x the fastest

Fastest abi_boundary_w_leaf_null_entry (4.03 us) to slowest abi_boundary_w_leaf_scalar_runtime_w (1.46 ms): 362.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_leaf_null_entry** at 4027.3 ns median (-99.7% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 362.60x (fastest 4027.3 ns, slowest 1460310.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 6339ns | 6362ns | 6083ns | 6309ns | 6511ns | -99.57% |
| abi_boundary_w_leaf_scalar_anchor | 1458377ns | 1456043ns | 1453065ns | 1455488ns | 1465365ns | -0.43% |
| abi_boundary_w_leaf_scalar_dispatch | 1457414ns | 1457229ns | 1452922ns | 1456715ns | 1460708ns | -0.49% |
| abi_boundary_w_leaf_scalar_per_w | 1456670ns | 1455599ns | 1455101ns | 1455565ns | 1459111ns | -0.55% |
| abi_boundary_w_leaf_scalar_runtime_w | 1464660ns | 1463085ns | 1462069ns | 1462946ns | 1468525ns | base |
| abi_boundary_w_leaf_soa_dispatch | 1457324ns | 1459275ns | 1452472ns | 1457092ns | 1460099ns | -0.50% |
| abi_boundary_w_leaf_soa_per_w | 1459201ns | 1459304ns | 1456260ns | 1458881ns | 1461151ns | -0.37% |
| abi_boundary_w_leaf_soa_runtime_w | 1460292ns | 1460849ns | 1452815ns | 1460571ns | 1463612ns | -0.30% |
| abi_boundary_w_leaf_zig_runtime_w | 1443509ns | 1443082ns | 1440065ns | 1442491ns | 1446756ns | -1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 4050ns | 3907ns | 4182ns | -99.72% | 0.001 |
| abi_boundary_w_leaf_scalar_anchor | 1455746ns | 1450654ns | 1462684ns | -0.42% | 0.000 |
| abi_boundary_w_leaf_scalar_dispatch | 1454784ns | 1450498ns | 1457937ns | -0.48% | 0.000 |
| abi_boundary_w_leaf_scalar_per_w | 1454053ns | 1452651ns | 1456476ns | -0.53% | 0.000 |
| abi_boundary_w_leaf_scalar_runtime_w | 1461838ns | 1459286ns | 1465582ns | base | 0.000 |
| abi_boundary_w_leaf_soa_dispatch | 1454590ns | 1450065ns | 1457199ns | -0.50% | 0.000 |
| abi_boundary_w_leaf_soa_per_w | 1456544ns | 1453819ns | 1458400ns | -0.36% | 0.000 |
| abi_boundary_w_leaf_soa_runtime_w | 1457563ns | 1450194ns | 1460929ns | -0.29% | 0.000 |
| abi_boundary_w_leaf_zig_runtime_w | 1440702ns | 1437413ns | 1443919ns | -1.45% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 28062.8 | 4156.5 | 4050.0 | n/a |
| abi_boundary_w_leaf_scalar_anchor | 41761.0 | 1454491.3 | 1455745.8 | n/a |
| abi_boundary_w_leaf_scalar_dispatch | 41934.4 | 1454717.1 | 1454783.9 | n/a |
| abi_boundary_w_leaf_scalar_per_w | 43636.2 | 1453964.7 | 1454052.6 | n/a |
| abi_boundary_w_leaf_scalar_runtime_w | 49135.6 | 1461584.9 | 1461838.0 | n/a |
| abi_boundary_w_leaf_soa_dispatch | 44937.0 | 1454869.0 | 1454589.7 | n/a |
| abi_boundary_w_leaf_soa_per_w | 45671.7 | 1455752.0 | 1456544.2 | n/a |
| abi_boundary_w_leaf_soa_runtime_w | 44510.8 | 1457898.8 | 1457563.5 | n/a |
| abi_boundary_w_leaf_zig_runtime_w | 187990.7 | 1439272.5 | 1440701.6 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_leaf_null_entry | 0.001 | 97.0% |
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
| abi_boundary_w_leaf_null_entry | 6339ns | 6339ns | -99.57% |
| abi_boundary_w_leaf_scalar_anchor | 1458377ns | 1458377ns | -0.43% |
| abi_boundary_w_leaf_scalar_dispatch | 1457414ns | 1457414ns | -0.49% |
| abi_boundary_w_leaf_scalar_per_w | 1456670ns | 1456670ns | -0.55% |
| abi_boundary_w_leaf_scalar_runtime_w | 1464660ns | 1464660ns | base |
| abi_boundary_w_leaf_soa_dispatch | 1457324ns | 1457324ns | -0.50% |
| abi_boundary_w_leaf_soa_per_w | 1459201ns | 1459201ns | -0.37% |
| abi_boundary_w_leaf_soa_runtime_w | 1460292ns | 1460292ns | -0.30% |
| abi_boundary_w_leaf_zig_runtime_w | 1443509ns | 1443509ns | -1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_leaf_scalar_runtime_w | 1460310ns | base | --- | [1459621, 1465582] | --- | --- | --- | --- |
| abi_boundary_w_leaf_null_entry | 4027ns | -1456212.3ns (-99.7%) | [-1461605, -1455547]ns | [3940, 4182] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_anchor | 1453363ns | no significant difference | [-13340, +2757]ns | [1451191, 1462684] | no | 0.2188 | 0.2188 | 0 |
| abi_boundary_w_leaf_scalar_dispatch | 1454575ns | -6560.0ns (-0.4%) | [-12535, -2068]ns | [1451840, 1457937] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_leaf_scalar_per_w | 1452935ns | -7481.7ns (-0.5%) | [-10090, -5785]ns | [1452746, 1456476] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_dispatch | 1456498ns | -5882.0ns (-0.4%) | [-12492, -3371]ns | [1450072, 1457199] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_per_w | 1456600ns | -4988.9ns (-0.3%) | [-8315, -2578]ns | [1454632, 1458400] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_leaf_soa_runtime_w | 1458004ns | -2725.8ns (-0.2%) | [-9190, -908]ns | [1453758, 1460929] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_leaf_zig_runtime_w | 1440236ns | -21843.8ns (-1.5%) | [-25449, -16117]ns | [1437949, 1443919] | YES | 0.0357 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_leaf_scalar_runtime_w | abi_boundary_w_leaf_null_entry | abi_boundary_w_leaf_scalar_anchor | abi_boundary_w_leaf_scalar_dispatch | abi_boundary_w_leaf_scalar_per_w | abi_boundary_w_leaf_soa_dispatch | abi_boundary_w_leaf_soa_per_w | abi_boundary_w_leaf_soa_runtime_w | abi_boundary_w_leaf_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 1460054ns | -99.7% | -0.6% | -0.1% | -0.4% | -0.2% | -0.2% | -0.7% | -1.6% |
| 2 | 1459286ns | -99.7% | -0.3% | -0.4% | -0.4% | -0.6% | -0.4% | -0.0% | -1.3% |
| 3 | 1465324ns | -99.7% | -0.9% | -1.0% | -0.8% | -0.5% | -0.6% | -0.2% | -1.6% |
| 4 | 1459956ns | -99.7% | -0.4% | -0.2% | -0.5% | -0.3% | -0.3% | -0.1% | -1.0% |
| 5 | 1465841ns | -99.7% | -1.0% | -0.7% | -0.5% | -1.1% | -0.5% | -0.6% | -1.9% |
| 6 | 1460567ns | -99.7% | +0.6% | -0.5% | -0.5% | -0.2% | -0.1% | -0.2% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_leaf_null_entry | -0.520 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_scalar_anchor | -0.166 | ok |
| abi_boundary_w_leaf_scalar_dispatch | -0.207 | moderate- |
| abi_boundary_w_leaf_scalar_per_w | -0.370 | moderate- |
| abi_boundary_w_leaf_scalar_runtime_w | -0.547 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_soa_dispatch | -0.588 | HIGH- (thermal bounce) |
| abi_boundary_w_leaf_soa_per_w | 0.080 | ok |
| abi_boundary_w_leaf_soa_runtime_w | 0.053 | ok |
| abi_boundary_w_leaf_zig_runtime_w | -0.085 | ok |

**Consistency summary:**

- **abi_boundary_w_leaf_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_anchor**: won 5/6, lost 1/6
- **abi_boundary_w_leaf_scalar_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_leaf_soa_runtime_w**: won 4/6, lost 0/6
- **abi_boundary_w_leaf_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_leaf_null_entry | 123118.2ns | 4050.0ns | 3039.9% | HIGH |
| abi_boundary_w_leaf_scalar_anchor | 4406653.6ns | 1455745.8ns | 302.7% | HIGH |
| abi_boundary_w_leaf_scalar_dispatch | 4408018.1ns | 1454783.9ns | 303.0% | HIGH |
| abi_boundary_w_leaf_scalar_per_w | 4408013.3ns | 1454052.6ns | 303.2% | HIGH |
| abi_boundary_w_leaf_scalar_runtime_w | 4438021.5ns | 1461838.0ns | 303.6% | HIGH |
| abi_boundary_w_leaf_soa_dispatch | 4411131.9ns | 1454589.7ns | 303.3% | HIGH |
| abi_boundary_w_leaf_soa_per_w | 4415072.8ns | 1456544.2ns | 303.1% | HIGH |
| abi_boundary_w_leaf_soa_runtime_w | 4419963.6ns | 1457563.5ns | 303.2% | HIGH |
| abi_boundary_w_leaf_zig_runtime_w | 4579800.6ns | 1440701.6ns | 317.9% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_leaf_null_entry (n=6, range 3906.7-4182.5 ns)
   3906.7 |########################################
   3920.5 |
   3934.3 |
   3948.1 |
   3961.9 |########################################
   3975.6 |
   3989.4 |
   4003.2 |########################################
   4017.0 |
   4030.8 |
   4044.6 |########################################
   4058.4 |
   4072.2 |
   4086.0 |
   4099.8 |
   4113.6 |
   4127.3 |
   4141.1 |########################################
   4154.9 |
   4168.7 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_anchor (n=6, range 1450653.8-1462683.6 ns)
  1450653.8 |########################################
  1451255.3 |########################################
  1451856.8 |
  1452458.3 |########################################
  1453059.8 |
  1453661.2 |########################################
  1454262.7 |
  1454864.2 |
  1455465.7 |########################################
  1456067.2 |
  1456668.7 |
  1457270.2 |
  1457871.7 |
  1458473.1 |
  1459074.6 |
  1459676.1 |
  1460277.6 |
  1460879.1 |
  1461480.6 |
  1462082.1 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_dispatch (n=6, range 1450497.9-1457937.3 ns)
  1450497.9 |########################################
  1450869.9 |
  1451241.8 |
  1451613.8 |
  1451985.8 |
  1452357.8 |
  1452729.7 |
  1453101.7 |########################################
  1453473.7 |########################################
  1453845.6 |
  1454217.6 |
  1454589.6 |
  1454961.5 |
  1455333.5 |########################################
  1455705.5 |
  1456077.4 |
  1456449.4 |
  1456821.4 |
  1457193.4 |########################################
  1457565.3 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_per_w (n=6, range 1452650.8-1456476.1 ns)
  1452650.8 |#############
  1452842.1 |########################################
  1453033.3 |
  1453224.6 |
  1453415.9 |
  1453607.1 |
  1453798.4 |
  1453989.6 |
  1454180.9 |
  1454372.2 |
  1454563.4 |
  1454754.7 |#############
  1454945.9 |
  1455137.2 |
  1455328.5 |
  1455519.7 |
  1455711.0 |
  1455902.3 |
  1456093.5 |
  1456284.8 |
  (0 below, 1 above range)

abi_boundary_w_leaf_scalar_runtime_w (n=6, range 1459286.2-1465582.4 ns)
  1459286.2 |####################
  1459601.0 |
  1459915.8 |########################################
  1460230.6 |
  1460545.4 |####################
  1460860.3 |
  1461175.1 |
  1461489.9 |
  1461804.7 |
  1462119.5 |
  1462434.3 |
  1462749.1 |
  1463063.9 |
  1463378.8 |
  1463693.6 |
  1464008.4 |
  1464323.2 |
  1464638.0 |
  1464952.8 |
  1465267.6 |####################
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_dispatch (n=6, range 1450065.4-1457199.4 ns)
  1450065.4 |########################################
  1450422.1 |
  1450778.8 |
  1451135.5 |
  1451492.2 |
  1451848.9 |
  1452205.6 |
  1452562.3 |
  1452919.0 |
  1453275.7 |
  1453632.4 |
  1453989.1 |
  1454345.8 |
  1454702.5 |
  1455059.2 |
  1455415.9 |
  1455772.6 |####################
  1456129.3 |
  1456486.0 |
  1456842.7 |########################################
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_per_w (n=6, range 1453819.2-1458400.0 ns)
  1453819.2 |########################################
  1454048.2 |
  1454277.3 |
  1454506.3 |
  1454735.4 |
  1454964.4 |
  1455193.4 |
  1455422.5 |########################################
  1455651.5 |
  1455880.6 |
  1456109.6 |
  1456338.6 |########################################
  1456567.7 |########################################
  1456796.7 |
  1457025.8 |
  1457254.8 |
  1457483.8 |
  1457712.9 |########################################
  1457941.9 |
  1458171.0 |
  (0 below, 1 above range)

abi_boundary_w_leaf_soa_runtime_w (n=6, range 1450193.8-1460928.8 ns)
  1450193.8 |####################
  1450730.5 |
  1451267.3 |
  1451804.0 |
  1452340.8 |
  1452877.5 |
  1453414.3 |
  1453951.0 |
  1454487.8 |
  1455024.5 |
  1455561.3 |
  1456098.0 |
  1456634.8 |
  1457171.5 |########################################
  1457708.3 |
  1458245.0 |####################
  1458781.8 |####################
  1459318.5 |
  1459855.3 |
  1460392.0 |
  (0 below, 1 above range)

abi_boundary_w_leaf_zig_runtime_w (n=6, range 1437413.3-1443919.1 ns)
  1437413.3 |########################################
  1437738.6 |
  1438063.9 |
  1438389.2 |########################################
  1438714.5 |
  1439039.8 |
  1439365.1 |########################################
  1439690.3 |
  1440015.6 |
  1440340.9 |
  1440666.2 |########################################
  1440991.5 |
  1441316.8 |
  1441642.1 |########################################
  1441967.4 |
  1442292.7 |
  1442618.0 |
  1442943.3 |
  1443268.6 |
  1443593.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_leaf_null_entry**: bridge=3067.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_anchor**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_scalar_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_dispatch**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_per_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_soa_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_leaf_zig_runtime_w**: bridge=318.1% of algo (FFI overhead may distort results)
