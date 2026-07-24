# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 2.71 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 30174% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (2.71 us) leads abi_soa_win_leaf_soa_payload (820.74 us) by 30174%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 538.3x slower than the field

abi_soa_win_leaf_scalar_payload (1.46 ms) is 538.3x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 538.3x the fastest

Fastest abi_soa_win_leaf_null_entry (2.71 us) to slowest abi_soa_win_leaf_scalar_payload (1.46 ms): 538.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 2711.1 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 538.35x (fastest 2711.1 ns, slowest 1459481.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4974ns | 5021ns | 4828ns | 4965ns | 5062ns | -99.67% |
| abi_soa_win_leaf_scalar_payload | 1486507ns | 1462329ns | 1461180ns | 1462048ns | 1535859ns | base |
| abi_soa_win_leaf_soa_payload | 825562ns | 823458ns | 819159ns | 822406ns | 833498ns | -44.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 2713ns | 2648ns | 2777ns | -99.82% | 0.047 |
| abi_soa_win_leaf_scalar_payload | 1483462ns | 1458290ns | 1532546ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 822813ns | 816650ns | 830484ns | -44.53% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 27825.0 | 2740.1 | 2712.8 | n/a |
| abi_soa_win_leaf_scalar_payload | 52468.5 | 1513713.6 | 1483462.5 | n/a |
| abi_soa_win_leaf_soa_payload | 42045.7 | 823671.0 | 822813.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.047 | 97.7% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4974ns | 4974ns | -99.67% |
| abi_soa_win_leaf_scalar_payload | 1486507ns | 1486507ns | base |
| abi_soa_win_leaf_soa_payload | 825562ns | 825562ns | -44.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1459482ns | base | --- | [1458360, 1532546] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 2711ns | -1456775.0ns (-99.8%) | [-1529863, -1455611]ns | [2651, 2777] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 820736ns | -640589.4ns (-43.9%) | [-705579, -635780]ns | [817219, 830484] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1464036ns | -99.8% | -44.1% |
| 2 | 1459846ns | -99.8% | -43.5% |
| 3 | 1459118ns | -99.8% | -44.0% |
| 4 | 1458430ns | -99.8% | -43.6% |
| 5 | 1458290ns | -99.8% | -43.8% |
| 6 | 1601056ns | -99.8% | -47.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.405 | moderate- |
| abi_soa_win_leaf_scalar_payload | -0.041 | ok |
| abi_soa_win_leaf_soa_payload | -0.222 | moderate- |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 119723.1ns | 2712.8ns | 4413.2% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4563045.6ns | 1483462.5ns | 307.6% | HIGH |
| abi_soa_win_leaf_soa_payload | 2513121.3ns | 822813.0ns | 305.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 2648.3-2776.7 ns)
   2648.3 |########################################
   2654.7 |
   2661.1 |
   2667.6 |
   2674.0 |
   2680.4 |
   2686.8 |
   2693.2 |
   2699.6 |
   2706.1 |####################
   2712.5 |####################
   2718.9 |
   2725.3 |
   2731.7 |
   2738.1 |
   2744.6 |
   2751.0 |
   2757.4 |
   2763.8 |####################
   2770.2 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1458289.6-1532546.0 ns)
  1458289.6 |########################################
  1462002.4 |##########
  1465715.2 |
  1469428.1 |
  1473140.9 |
  1476853.7 |
  1480566.5 |
  1484279.3 |
  1487992.2 |
  1491705.0 |
  1495417.8 |
  1499130.6 |
  1502843.4 |
  1506556.3 |
  1510269.1 |
  1513981.9 |
  1517694.7 |
  1521407.5 |
  1525120.4 |
  1528833.2 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 816649.6-830484.1 ns)
  816649.6 |########################################
  817341.3 |########################################
  818033.1 |
  818724.8 |
  819416.5 |########################################
  820108.2 |
  820800.0 |
  821491.7 |########################################
  822183.4 |
  822875.1 |
  823566.9 |
  824258.6 |########################################
  824950.3 |
  825642.1 |
  826333.8 |
  827025.5 |
  827717.2 |
  828409.0 |
  829100.7 |
  829792.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=4421.0% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=305.3% of algo (FFI overhead may distort results)
