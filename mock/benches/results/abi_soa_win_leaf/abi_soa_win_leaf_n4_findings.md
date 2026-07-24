# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_leaf_null_entry dominates: 35988% faster than the next best (abi_soa_win_leaf_scalar_payload)

abi_soa_win_leaf_null_entry (4.05 us) leads abi_soa_win_leaf_scalar_payload (1.46 ms) by 35988%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_soa_payload is an outlier: 361.5x slower than the field

abi_soa_win_leaf_soa_payload (1.46 ms) is 361.5x the fastest (4.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 361.5x the fastest

Fastest abi_soa_win_leaf_null_entry (4.05 us) to slowest abi_soa_win_leaf_soa_payload (1.46 ms): 361.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 4047.9 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 361.49x (fastest 4047.9 ns, slowest 1463279.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 6374ns | 6386ns | 6089ns | 6304ns | 6621ns | -99.56% |
| abi_soa_win_leaf_scalar_payload | 1462980ns | 1463580ns | 1454389ns | 1462901ns | 1467396ns | base |
| abi_soa_win_leaf_soa_payload | 1474721ns | 1466260ns | 1456306ns | 1463734ns | 1500409ns | +0.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 4061ns | 3882ns | 4219ns | -99.72% | 0.001 |
| abi_soa_win_leaf_scalar_payload | 1460163ns | 1451796ns | 1464395ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 1471552ns | 1453816ns | 1496424ns | +0.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 27399.0 | 4178.1 | 4061.0 | n/a |
| abi_soa_win_leaf_scalar_payload | 46915.5 | 1460298.9 | 1460163.1 | n/a |
| abi_soa_win_leaf_soa_payload | 56267.9 | 1472740.2 | 1471552.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.001 | 95.9% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.3% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 6374ns | 6374ns | -99.56% |
| abi_soa_win_leaf_scalar_payload | 1462980ns | 1462980ns | base |
| abi_soa_win_leaf_soa_payload | 1474721ns | 1474721ns | +0.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1460786ns | base | --- | [1455308, 1464395] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 4048ns | -1456866.2ns (-99.7%) | [-1460227, -1451213]ns | [3916, 4219] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 1463279ns | no significant difference | [-3921, +32030]ns | [1454954, 1496424] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1462026ns | -99.7% | +1.3% |
| 2 | 1461606ns | -99.7% | -0.3% |
| 3 | 1458821ns | -99.7% | -0.2% |
| 4 | 1451796ns | -99.7% | +0.1% |
| 5 | 1459966ns | -99.7% | +0.7% |
| 6 | 1466763ns | -99.7% | +3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.417 | moderate- |
| abi_soa_win_leaf_scalar_payload | 0.102 | ok |
| abi_soa_win_leaf_soa_payload | 0.132 | ok |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 121578.2ns | 4061.0ns | 2993.8% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4430794.7ns | 1460163.1ns | 303.4% | HIGH |
| abi_soa_win_leaf_soa_payload | 4475744.1ns | 1471552.5ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 3882.5-4219.1 ns)
   3882.5 |########################################
   3899.3 |
   3916.2 |
   3933.0 |########################################
   3949.8 |########################################
   3966.7 |
   3983.5 |
   4000.3 |
   4017.2 |
   4034.0 |
   4050.8 |
   4067.7 |
   4084.5 |
   4101.3 |
   4118.2 |
   4135.0 |########################################
   4151.8 |
   4168.7 |
   4185.5 |########################################
   4202.3 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1451795.8-1464394.5 ns)
  1451795.8 |########################################
  1452425.7 |
  1453055.7 |
  1453685.6 |
  1454315.6 |
  1454945.5 |
  1455575.4 |
  1456205.4 |
  1456835.3 |
  1457465.2 |
  1458095.2 |
  1458725.1 |########################################
  1459355.0 |########################################
  1459985.0 |
  1460614.9 |
  1461244.9 |########################################
  1461874.8 |########################################
  1462504.7 |
  1463134.7 |
  1463764.6 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 1453815.8-1496424.2 ns)
  1453815.8 |####################
  1455946.2 |########################################
  1458076.6 |
  1460207.1 |
  1462337.5 |
  1464467.9 |
  1466598.3 |
  1468728.7 |####################
  1470859.2 |
  1472989.6 |
  1475120.0 |
  1477250.4 |
  1479380.8 |####################
  1481511.3 |
  1483641.7 |
  1485772.1 |
  1487902.5 |
  1490032.9 |
  1492163.4 |
  1494293.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=3012.1% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=304.1% of algo (FFI overhead may distort results)
