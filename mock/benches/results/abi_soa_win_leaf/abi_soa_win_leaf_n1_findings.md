# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.47 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 4.90 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 29723% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (4.90 us) leads abi_soa_win_leaf_soa_payload (1.46 ms) by 29723%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 299.3x slower than the field

abi_soa_win_leaf_scalar_payload (1.47 ms) is 299.3x the fastest (4.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 299.3x the fastest

Fastest abi_soa_win_leaf_null_entry (4.90 us) to slowest abi_soa_win_leaf_scalar_payload (1.47 ms): 299.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 4904.4 ns median (-99.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 299.26x (fastest 4904.4 ns, slowest 1467677.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 7229ns | 7118ns | 7006ns | 7081ns | 7563ns | -99.51% |
| abi_soa_win_leaf_scalar_payload | 1470409ns | 1470609ns | 1463362ns | 1470070ns | 1474439ns | base |
| abi_soa_win_leaf_soa_payload | 1465719ns | 1465279ns | 1461954ns | 1464445ns | 1469513ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4972ns | 4832ns | 5173ns | -99.66% | 0.000 |
| abi_soa_win_leaf_scalar_payload | 1467539ns | 1460697ns | 1471482ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 1462986ns | 1459430ns | 1466709ns | -0.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 26701.2 | 4954.8 | 4972.3 | n/a |
| abi_soa_win_leaf_scalar_payload | 48813.0 | 1467469.3 | 1467538.8 | n/a |
| abi_soa_win_leaf_soa_payload | 45354.6 | 1462242.2 | 1462985.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.000 | 98.5% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.3% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 7229ns | 7229ns | -99.51% |
| abi_soa_win_leaf_scalar_payload | 1470409ns | 1470409ns | base |
| abi_soa_win_leaf_soa_payload | 1465719ns | 1465719ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1467678ns | base | --- | [1463457, 1471482] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 4904ns | -1462773.1ns (-99.7%) | [-1466309, -1458617]ns | [4840, 5173] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 1462612ns | -5271.0ns (-0.4%) | [-6546, -1842]ns | [1459636, 1466709] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1466217ns | -99.7% | -0.5% |
| 2 | 1468424ns | -99.6% | -0.4% |
| 3 | 1474540ns | -99.7% | -0.4% |
| 4 | 1467341ns | -99.7% | -0.4% |
| 5 | 1468014ns | -99.7% | -0.2% |
| 6 | 1460697ns | -99.7% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.040 | ok |
| abi_soa_win_leaf_scalar_payload | 0.003 | ok |
| abi_soa_win_leaf_soa_payload | -0.250 | moderate- |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 124532.9ns | 4972.3ns | 2504.5% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4454488.5ns | 1467538.8ns | 303.5% | HIGH |
| abi_soa_win_leaf_soa_payload | 4437969.4ns | 1462985.6ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 4832.5-5172.7 ns)
   4832.5 |########################################
   4849.5 |
   4866.5 |
   4883.5 |####################
   4900.5 |
   4917.6 |####################
   4934.6 |
   4951.6 |
   4968.6 |
   4985.6 |
   5002.6 |
   5019.6 |
   5036.6 |
   5053.6 |####################
   5070.6 |
   5087.7 |
   5104.7 |
   5121.7 |
   5138.7 |
   5155.7 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1460697.1-1471481.9 ns)
  1460697.1 |########################################
  1461236.3 |
  1461775.6 |
  1462314.8 |
  1462854.1 |
  1463393.3 |
  1463932.5 |
  1464471.8 |
  1465011.0 |
  1465550.3 |
  1466089.5 |########################################
  1466628.7 |
  1467168.0 |########################################
  1467707.2 |########################################
  1468246.5 |########################################
  1468785.7 |
  1469324.9 |
  1469864.2 |
  1470403.4 |
  1470942.7 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 1459429.6-1466708.8 ns)
  1459429.6 |########################################
  1459793.6 |########################################
  1460157.5 |
  1460521.5 |
  1460885.4 |
  1461249.4 |
  1461613.3 |########################################
  1461977.3 |
  1462341.3 |
  1462705.2 |
  1463069.2 |########################################
  1463433.1 |
  1463797.1 |
  1464161.0 |
  1464525.0 |
  1464889.0 |########################################
  1465252.9 |
  1465616.9 |
  1465980.8 |
  1466344.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=2539.3% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=303.1% of algo (FFI overhead may distort results)
