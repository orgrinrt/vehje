# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 46721% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (3.10 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 46721%, a clear separation rather than a photo finish. CV 30.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 469.3x slower than the field

abi_residency_leaf_fresh_alloc (1.45 ms) is 469.3x the fastest (3.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_leaf_null_entry is fastest but the noisiest (CV 30.3%)

abi_residency_leaf_null_entry wins on median (3.10 us) yet has the highest variance (CV 30.3%), while abi_residency_leaf_reused_buffer is the steadiest (CV 2.7%, 1.45 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 469.3x the fastest

Fastest abi_residency_leaf_null_entry (3.10 us) to slowest abi_residency_leaf_fresh_alloc (1.45 ms): 469.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_residency_leaf_null_entry is inconsistent: worst-20% is 1.5x its best-20%

abi_residency_leaf_null_entry's best 20% of batches run at 2.92 us but its worst 20% at 4.39 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 3097.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 469.25x (fastest 3097.1 ns, slowest 1453299.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1493429ns | 1455905ns | 1448782ns | 1454526ns | 1574108ns | +1.61% |
| abi_residency_leaf_null_entry | 6188ns | 5447ns | 5068ns | 5367ns | 7981ns | -99.58% |
| abi_residency_leaf_reused_buffer | 1469762ns | 1452579ns | 1451081ns | 1452369ns | 1505192ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1490677ns | 1446358ns | 1570845ns | +1.60% | 0.000 |
| abi_residency_leaf_null_entry | 3474ns | 2917ns | 4386ns | -99.76% | 0.002 |
| abi_residency_leaf_reused_buffer | 1467193ns | 1448652ns | 1502471ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 46102.1 | 1479054.6 | 1490677.2 | n/a |
| abi_residency_leaf_null_entry | 36301.1 | 3624.2 | 3474.5 | n/a |
| abi_residency_leaf_reused_buffer | 41605.7 | 1495971.7 | 1467193.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.003 | 94.2% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1493429ns | 1493429ns | +1.61% |
| abi_residency_leaf_null_entry | 6188ns | 6188ns | -99.58% |
| abi_residency_leaf_reused_buffer | 1469762ns | 1469762ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1450081ns | base | --- | [1449027, 1502471] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1453300ns | no significant difference | [-52671, +120764]ns | [1447887, 1570845] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_leaf_null_entry | 3097ns | -1447044.0ns (-99.8%) | [-1498233, -1445879]ns | [2940, 4386] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1449402ns | -0.2% | -99.8% |
| 2 | 1448652ns | +0.1% | -99.8% |
| 3 | 1554448ns | -6.6% | -99.6% |
| 4 | 1450179ns | +16.3% | -99.8% |
| 5 | 1449983ns | +0.4% | -99.8% |
| 6 | 1450495ns | +0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | -0.211 | moderate- |
| abi_residency_leaf_null_entry | -0.190 | ok |
| abi_residency_leaf_reused_buffer | -0.240 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4497484.6ns | 1490677.2ns | 301.7% | HIGH |
| abi_residency_leaf_null_entry | 130540.9ns | 3474.5ns | 3757.1% | HIGH |
| abi_residency_leaf_reused_buffer | 4518792.9ns | 1467193.3ns | 308.0% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1446357.9-1570844.6 ns)
  1446357.9 |########################################
  1452582.2 |##########################
  1458806.6 |
  1465030.9 |
  1471255.2 |
  1477479.6 |
  1483703.9 |
  1489928.2 |
  1496152.6 |
  1502376.9 |
  1508601.2 |
  1514825.6 |
  1521049.9 |
  1527274.3 |
  1533498.6 |
  1539722.9 |
  1545947.3 |
  1552171.6 |
  1558395.9 |
  1564620.3 |
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 2916.7-4386.4 ns)
   2916.7 |########################################
   2990.2 |
   3063.7 |########################################
   3137.2 |
   3210.6 |####################
   3284.1 |
   3357.6 |
   3431.1 |
   3504.6 |
   3578.1 |
   3651.6 |
   3725.1 |
   3798.5 |
   3872.0 |
   3945.5 |
   4019.0 |
   4092.5 |
   4166.0 |
   4239.5 |
   4313.0 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1448652.1-1502471.4 ns)
  1448652.1 |########################################
  1451343.1 |
  1454034.0 |
  1456725.0 |
  1459416.0 |
  1462106.9 |
  1464797.9 |
  1467488.9 |
  1470179.8 |
  1472870.8 |
  1475561.8 |
  1478252.7 |
  1480943.7 |
  1483634.7 |
  1486325.6 |
  1489016.6 |
  1491707.6 |
  1494398.5 |
  1497089.5 |
  1499780.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: CV=27.0% (high variance, measurements may be unstable)
- **abi_residency_leaf_null_entry**: bridge=3895.3% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.7% of algo (FFI overhead may distort results)
