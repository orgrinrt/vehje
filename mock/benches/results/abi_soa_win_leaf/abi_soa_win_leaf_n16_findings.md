# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 2.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 33230% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (2.48 us) leads abi_soa_win_leaf_soa_payload (825.04 us) by 33230%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 590.6x slower than the field

abi_soa_win_leaf_scalar_payload (1.46 ms) is 590.6x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_leaf_null_entry shows alternating (throttle bounce) (autocorr -0.58)

abi_soa_win_leaf_null_entry's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 590.6x the fastest

Fastest abi_soa_win_leaf_null_entry (2.48 us) to slowest abi_soa_win_leaf_scalar_payload (1.46 ms): 590.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 2475.4 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 590.57x (fastest 2475.4 ns, slowest 1461888.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4751ns | 4676ns | 4652ns | 4674ns | 4915ns | -99.68% |
| abi_soa_win_leaf_scalar_payload | 1504334ns | 1464911ns | 1460568ns | 1463542ns | 1587403ns | base |
| abi_soa_win_leaf_soa_payload | 829508ns | 827757ns | 823712ns | 826420ns | 837038ns | -44.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 2515ns | 2468ns | 2603ns | -99.83% | 0.006 |
| abi_soa_win_leaf_scalar_payload | 1501198ns | 1457718ns | 1583773ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 826852ns | 821124ns | 834346ns | -44.92% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 26856.1 | 2603.1 | 2515.5 | n/a |
| abi_soa_win_leaf_scalar_payload | 58993.1 | 1486388.3 | 1501198.3 | n/a |
| abi_soa_win_leaf_soa_payload | 41429.2 | 825892.7 | 826852.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.006 | 99.7% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4751ns | 4751ns | -99.68% |
| abi_soa_win_leaf_scalar_payload | 1504334ns | 1504334ns | base |
| abi_soa_win_leaf_soa_payload | 829508ns | 829508ns | -44.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1461888ns | base | --- | [1457934, 1583773] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 2475ns | -1459419.8ns (-99.8%) | [-1581243, -1455385]ns | [2468, 2603] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 825043ns | -638021.5ns (-43.6%) | [-752603, -632414]ns | [821168, 834346] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1522808ns | -99.8% | -46.1% |
| 2 | 1644738ns | -99.8% | -48.9% |
| 3 | 1457718ns | -99.8% | -43.2% |
| 4 | 1459899ns | -99.8% | -43.7% |
| 5 | 1458149ns | -99.8% | -43.5% |
| 6 | 1463877ns | -99.8% | -43.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.583 | HIGH- (thermal bounce) |
| abi_soa_win_leaf_scalar_payload | 0.073 | ok |
| abi_soa_win_leaf_soa_payload | -0.203 | moderate- |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 117305.1ns | 2515.5ns | 4663.3% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4530319.4ns | 1501198.3ns | 301.8% | HIGH |
| abi_soa_win_leaf_soa_payload | 2519849.1ns | 826852.4ns | 304.8% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 2467.9-2602.7 ns)
   2467.9 |########################################
   2474.6 |#############
   2481.4 |
   2488.1 |
   2494.9 |
   2501.6 |
   2508.3 |
   2515.1 |
   2521.8 |
   2528.6 |
   2535.3 |
   2542.0 |
   2548.8 |
   2555.5 |
   2562.3 |
   2569.0 |
   2575.7 |
   2582.5 |#############
   2589.2 |
   2596.0 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1457718.3-1583773.3 ns)
  1457718.3 |########################################
  1464021.1 |
  1470323.8 |
  1476626.6 |
  1482929.3 |
  1489232.1 |
  1495534.8 |
  1501837.6 |
  1508140.3 |
  1514443.1 |
  1520745.8 |##########
  1527048.6 |
  1533351.3 |
  1539654.1 |
  1545956.8 |
  1552259.6 |
  1558562.3 |
  1564865.1 |
  1571167.8 |
  1577470.6 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 821124.2-834346.2 ns)
  821124.2 |########################################
  821785.3 |
  822446.4 |
  823107.5 |####################
  823768.6 |
  824429.7 |
  825090.8 |
  825751.9 |
  826413.0 |####################
  827074.1 |####################
  827735.2 |
  828396.3 |
  829057.4 |
  829718.5 |
  830379.6 |
  831040.7 |
  831701.8 |
  832362.9 |
  833024.0 |
  833685.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=4744.1% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=304.4% of algo (FFI overhead may distort results)
