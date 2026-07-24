# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_leaf_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_leaf_scalar_payload has the worst median (1.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_leaf_null_entry at 3.20 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_leaf_null_entry dominates: 25597% faster than the next best (abi_soa_win_leaf_soa_payload)

abi_soa_win_leaf_null_entry (3.20 us) leads abi_soa_win_leaf_soa_payload (821.18 us) by 25597%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_scalar_payload is an outlier: 458.3x slower than the field

abi_soa_win_leaf_scalar_payload (1.46 ms) is 458.3x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 458.3x the fastest

Fastest abi_soa_win_leaf_null_entry (3.20 us) to slowest abi_soa_win_leaf_scalar_payload (1.46 ms): 458.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 3195.6 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 458.31x (fastest 3195.6 ns, slowest 1464583.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 5453ns | 5474ns | 5250ns | 5422ns | 5602ns | -99.63% |
| abi_soa_win_leaf_scalar_payload | 1465208ns | 1467710ns | 1452383ns | 1466167ns | 1470183ns | base |
| abi_soa_win_leaf_soa_payload | 851624ns | 823842ns | 821252ns | 823223ns | 909410ns | -41.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 3175ns | 3083ns | 3228ns | -99.78% | 0.081 |
| abi_soa_win_leaf_scalar_payload | 1462197ns | 1449983ns | 1466987ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 848573ns | 818732ns | 905495ns | -41.97% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 27811.0 | 3192.1 | 3175.3 | n/a |
| abi_soa_win_leaf_scalar_payload | 53105.0 | 1462872.2 | 1462196.8 | n/a |
| abi_soa_win_leaf_soa_payload | 51526.3 | 847305.9 | 848573.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.080 | 96.5% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 5453ns | 5453ns | -99.63% |
| abi_soa_win_leaf_scalar_payload | 1465208ns | 1465208ns | base |
| abi_soa_win_leaf_soa_payload | 851624ns | 851624ns | -41.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1464583ns | base | --- | [1455020, 1466987] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 3196ns | -1461360.1ns (-99.8%) | [-1463825, -1451879]ns | [3103, 3228] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 821184ns | -639536.9ns (-43.7%) | [-644633, -556701]ns | [819040, 905495] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1466261ns | -99.8% | -32.9% |
| 2 | 1460057ns | -99.8% | -43.8% |
| 3 | 1465044ns | -99.8% | -43.9% |
| 4 | 1449983ns | -99.8% | -43.5% |
| 5 | 1464122ns | -99.8% | -44.1% |
| 6 | 1467713ns | -99.8% | -43.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.401 | moderate+ |
| abi_soa_win_leaf_scalar_payload | -0.294 | moderate- |
| abi_soa_win_leaf_soa_payload | -0.029 | ok |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 121034.4ns | 3175.3ns | 3811.7% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4441713.6ns | 1462196.8ns | 303.8% | HIGH |
| abi_soa_win_leaf_soa_payload | 2601454.0ns | 848573.2ns | 306.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 3083.3-3227.7 ns)
   3083.3 |####################
   3090.5 |
   3097.7 |
   3105.0 |
   3112.2 |
   3119.4 |####################
   3126.6 |
   3133.8 |
   3141.1 |
   3148.3 |
   3155.5 |
   3162.7 |
   3169.9 |
   3177.2 |
   3184.4 |
   3191.6 |########################################
   3198.8 |####################
   3206.0 |
   3213.3 |
   3220.5 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1449983.3-1466987.2 ns)
  1449983.3 |########################################
  1450833.5 |
  1451683.7 |
  1452533.9 |
  1453384.1 |
  1454234.3 |
  1455084.5 |
  1455934.7 |
  1456784.9 |
  1457635.1 |
  1458485.3 |
  1459335.5 |########################################
  1460185.7 |
  1461035.9 |
  1461886.1 |
  1462736.3 |
  1463586.5 |########################################
  1464436.7 |########################################
  1465286.9 |
  1466137.1 |########################################
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 818732.5-905495.2 ns)
  818732.5 |########################################
  823070.6 |
  827408.8 |##########
  831746.9 |
  836085.0 |
  840423.2 |
  844761.3 |
  849099.4 |
  853437.6 |
  857775.7 |
  862113.8 |
  866452.0 |
  870790.1 |
  875128.3 |
  879466.4 |
  883804.5 |
  888142.7 |
  892480.8 |
  896818.9 |
  901157.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=3806.7% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=305.3% of algo (FFI overhead may distort results)
