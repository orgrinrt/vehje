# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.51 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 2.32 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 63225% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (2.32 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.47 ms) by 63225%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.51 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 651.7x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.51 ms) is 651.7x the fastest (2.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (63225% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 63225% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 651.7x the fastest

Fastest abi_cross_scalar_leaf_null_entry (2.32 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.51 ms): 651.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 2317.5 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 651.74x (fastest 2317.5 ns, slowest 1510413.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1473861ns | 1472506ns | 1465031ns | 1471039ns | 1482509ns | -2.84% |
| abi_cross_scalar_leaf_inproc_direct | 1516918ns | 1513519ns | 1500381ns | 1511476ns | 1533349ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1469134ns | 1470537ns | 1455144ns | 1470100ns | 1474682ns | -3.15% |
| abi_cross_scalar_leaf_null_entry | 4634ns | 4647ns | 4510ns | 4606ns | 4738ns | -99.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1470800ns | 1462099ns | 1479462ns | -2.84% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1513778ns | 1497448ns | 1530019ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1466107ns | 1452217ns | 1471577ns | -3.15% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 2322ns | 2257ns | 2390ns | -99.85% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 55076.6 | 1469827.0 | 1470800.4 | 1 |
| abi_cross_scalar_leaf_inproc_direct | 8358.5 | 1512070.7 | 1513778.2 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 8352.3 | 1469880.8 | 1466107.3 | n/a |
| abi_cross_scalar_leaf_null_entry | 28782.2 | 2442.4 | 2321.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.014 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1473861ns | 1473861ns | -2.84% |
| abi_cross_scalar_leaf_inproc_direct | 1516918ns | 1516918ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1469134ns | 1469134ns | -3.15% |
| abi_cross_scalar_leaf_null_entry | 4634ns | 4634ns | -99.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1510414ns | base | --- | [1500902, 1530019] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1469460ns | -35916.4ns (-2.4%) | [-66540, -26477]ns | [1463479, 1479462] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1467567ns | -47991.4ns (-3.2%) | [-64388, -30634]ns | [1459178, 1471577] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_leaf_null_entry | 2318ns | -1508112.3ns (-99.8%) | [-1527672, -1498584]ns | [2258, 2390] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1542951ns | -5.1% | -4.8% | -99.9% |
| 2 | 1504356ns | -1.7% | -2.5% | -99.8% |
| 3 | 1507522ns | -1.8% | -3.7% | -99.9% |
| 4 | 1517088ns | -3.6% | -3.4% | -99.8% |
| 5 | 1513305ns | -2.7% | -3.0% | -99.8% |
| 6 | 1497448ns | -2.1% | -1.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | -0.259 | moderate- |
| abi_cross_scalar_leaf_inproc_direct | -0.183 | ok |
| abi_cross_scalar_leaf_inproc_fnptr | 0.030 | ok |
| abi_cross_scalar_leaf_null_entry | -0.216 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4468408.4ns | 1470800.4ns | 303.8% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4551924.9ns | 1513778.2ns | 300.7% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4415574.8ns | 1466107.3ns | 301.2% | HIGH |
| abi_cross_scalar_leaf_null_entry | 117483.1ns | 2321.9ns | 5059.8% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1462099.2-1479461.9 ns)
  1462099.2 |########################################
  1462967.3 |
  1463835.5 |
  1464703.6 |########################################
  1465571.7 |
  1466439.9 |########################################
  1467308.0 |
  1468176.1 |
  1469044.3 |
  1469912.4 |
  1470780.5 |
  1471648.7 |########################################
  1472516.8 |
  1473385.0 |
  1474253.1 |
  1475121.2 |
  1475989.4 |
  1476857.5 |
  1477725.6 |########################################
  1478593.8 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1497447.9-1530019.1 ns)
  1497447.9 |########################################
  1499076.5 |
  1500705.0 |
  1502333.6 |
  1503962.1 |########################################
  1505590.7 |
  1507219.3 |########################################
  1508847.8 |
  1510476.4 |
  1512105.0 |########################################
  1513733.5 |
  1515362.1 |
  1516990.6 |########################################
  1518619.2 |
  1520247.8 |
  1521876.3 |
  1523504.9 |
  1525133.5 |
  1526762.0 |
  1528390.6 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1452217.1-1471577.0 ns)
  1452217.1 |########################################
  1453185.1 |
  1454153.1 |
  1455121.1 |
  1456089.1 |
  1457057.1 |
  1458025.1 |
  1458993.1 |
  1459961.1 |
  1460929.1 |
  1461897.1 |
  1462865.1 |
  1463833.1 |
  1464801.1 |
  1465769.1 |########################################
  1466737.1 |########################################
  1467705.1 |########################################
  1468673.1 |########################################
  1469641.1 |
  1470609.1 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 2257.1-2390.0 ns)
   2257.1 |########################################
   2263.7 |
   2270.4 |
   2277.0 |
   2283.7 |
   2290.3 |
   2297.0 |
   2303.6 |
   2310.3 |####################
   2316.9 |####################
   2323.6 |
   2330.2 |
   2336.8 |
   2343.5 |####################
   2350.1 |
   2356.8 |
   2363.4 |
   2370.1 |
   2376.7 |
   2383.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=5086.0% of algo (FFI overhead may distort results)
