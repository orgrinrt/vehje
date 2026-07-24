# abi_soa_win (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_leaf_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_leaf_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_leaf_null_entry dominates: 43446% faster than the next best (abi_soa_win_leaf_scalar_payload)

abi_soa_win_leaf_null_entry (3.37 us) leads abi_soa_win_leaf_scalar_payload (1.47 ms) by 43446%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_leaf_null_entry beats baseline by 100% (significant)

abi_soa_win_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_soa_win_leaf_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_leaf_soa_payload is an outlier: 435.8x slower than the field

abi_soa_win_leaf_soa_payload (1.47 ms) is 435.8x the fastest (3.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 435.8x the fastest

Fastest abi_soa_win_leaf_null_entry (3.37 us) to slowest abi_soa_win_leaf_soa_payload (1.47 ms): 435.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_leaf_null_entry** at 3365.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 435.83x (fastest 3365.8 ns, slowest 1466938.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 7026ns | 5620ns | 5549ns | 5603ns | 9899ns | -99.55% |
| abi_soa_win_leaf_scalar_payload | 1548268ns | 1468499ns | 1460372ns | 1466680ns | 1714597ns | base |
| abi_soa_win_leaf_soa_payload | 1485774ns | 1469704ns | 1456730ns | 1466493ns | 1529219ns | -4.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 3423ns | 3356ns | 3544ns | -99.78% | 0.001 |
| abi_soa_win_leaf_scalar_payload | 1545318ns | 1457846ns | 1711246ns | base | 0.000 |
| abi_soa_win_leaf_soa_payload | 1482737ns | 1454077ns | 1525580ns | -4.05% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 28653.7 | 3507.0 | 3423.0 | n/a |
| abi_soa_win_leaf_scalar_payload | 54082.8 | 1492131.5 | 1545318.2 | n/a |
| abi_soa_win_leaf_soa_payload | 51912.7 | 1483857.1 | 1482737.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_leaf_null_entry | 0.001 | 99.7% |
| abi_soa_win_leaf_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_leaf_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_leaf_null_entry | 7026ns | 7026ns | -99.55% |
| abi_soa_win_leaf_scalar_payload | 1548268ns | 1548268ns | base |
| abi_soa_win_leaf_soa_payload | 1485774ns | 1485774ns | -4.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_leaf_scalar_payload | 1465684ns | base | --- | [1459025, 1711246] | --- | --- | --- | --- |
| abi_soa_win_leaf_null_entry | 3366ns | -1462261.0ns (-99.8%) | [-1707759, -1455665]ns | [3359, 3544] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_leaf_soa_payload | 1466939ns | no significant difference | [-185666, +2117]ns | [1455693, 1525580] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_leaf_scalar_payload | abi_soa_win_leaf_null_entry | abi_soa_win_leaf_soa_payload |
|---|---|---|---|
| 1 | 1799305ns | -99.8% | -14.3% |
| 2 | 1623187ns | -99.8% | -7.0% |
| 3 | 1460204ns | -99.8% | -0.1% |
| 4 | 1461928ns | -99.8% | -0.3% |
| 5 | 1469440ns | -99.8% | +0.3% |
| 6 | 1457846ns | -99.8% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_leaf_null_entry | -0.498 | moderate- |
| abi_soa_win_leaf_scalar_payload | 0.338 | moderate+ |
| abi_soa_win_leaf_soa_payload | 0.315 | moderate+ |

**Consistency summary:**

- **abi_soa_win_leaf_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_leaf_soa_payload**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_leaf_null_entry | 121471.8ns | 3423.0ns | 3548.7% | HIGH |
| abi_soa_win_leaf_scalar_payload | 4681796.7ns | 1545318.2ns | 303.0% | HIGH |
| abi_soa_win_leaf_soa_payload | 4509833.3ns | 1482737.2ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_leaf_null_entry (n=6, range 3355.8-3543.8 ns)
   3355.8 |########################################
   3365.2 |#############
   3374.6 |
   3384.0 |
   3393.4 |
   3402.8 |
   3412.2 |
   3421.6 |
   3431.0 |
   3440.4 |
   3449.8 |
   3459.2 |
   3468.6 |
   3478.0 |#############
   3487.4 |
   3496.8 |
   3506.2 |
   3515.6 |
   3525.0 |
   3534.4 |
  (0 below, 1 above range)

abi_soa_win_leaf_scalar_payload (n=6, range 1457845.8-1711245.9 ns)
  1457845.8 |########################################
  1470515.8 |
  1483185.8 |
  1495855.8 |
  1508525.8 |
  1521195.8 |
  1533865.8 |
  1546535.8 |
  1559205.8 |
  1571875.8 |
  1584545.8 |
  1597215.8 |
  1609885.8 |
  1622555.8 |##########
  1635225.8 |
  1647895.8 |
  1660565.8 |
  1673235.8 |
  1685905.8 |
  1698575.8 |
  (0 below, 1 above range)

abi_soa_win_leaf_soa_payload (n=6, range 1454076.7-1525580.2 ns)
  1454076.7 |########################################
  1457651.9 |####################
  1461227.1 |
  1464802.2 |
  1468377.4 |
  1471952.6 |####################
  1475527.8 |
  1479102.9 |
  1482678.1 |
  1486253.3 |
  1489828.4 |
  1493403.6 |
  1496978.8 |
  1500554.0 |
  1504129.1 |
  1507704.3 |####################
  1511279.5 |
  1514854.7 |
  1518429.8 |
  1522005.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_leaf_null_entry**: bridge=3578.7% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_scalar_payload**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_soa_win_leaf_soa_payload**: bridge=303.2% of algo (FFI overhead may distort results)
