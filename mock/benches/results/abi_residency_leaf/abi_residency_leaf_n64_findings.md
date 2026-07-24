# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 58999% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (2.45 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 58999%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 594.0x slower than the field

abi_residency_leaf_fresh_alloc (1.46 ms) is 594.0x the fastest (2.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 594.0x the fastest

Fastest abi_residency_leaf_null_entry (2.45 us) to slowest abi_residency_leaf_fresh_alloc (1.46 ms): 594.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 2450.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 594.04x (fastest 2450.0 ns, slowest 1455392.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1476871ns | 1457946ns | 1453751ns | 1457138ns | 1518032ns | -1.44% |
| abi_residency_leaf_null_entry | 4770ns | 4662ns | 4641ns | 4656ns | 5006ns | -99.68% |
| abi_residency_leaf_reused_buffer | 1498404ns | 1450449ns | 1446933ns | 1449904ns | 1596889ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1473955ns | 1451292ns | 1514378ns | -1.46% | 0.000 |
| abi_residency_leaf_null_entry | 2489ns | 2418ns | 2594ns | -99.83% | 0.026 |
| abi_residency_leaf_reused_buffer | 1495858ns | 1444527ns | 1594238ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 42914.9 | 1465502.0 | 1473955.4 | 0 |
| abi_residency_leaf_null_entry | 27947.6 | 2721.2 | 2488.8 | n/a |
| abi_residency_leaf_reused_buffer | 37261.1 | 1472272.3 | 1495857.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.026 | 98.7% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1476871ns | 1476871ns | -1.44% |
| abi_residency_leaf_null_entry | 4770ns | 4770ns | -99.68% |
| abi_residency_leaf_reused_buffer | 1498404ns | 1498404ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1447932ns | base | --- | [1445404, 1594238] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1455393ns | no significant difference | [-128289, +54846]ns | [1452096, 1514378] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_leaf_null_entry | 2450ns | -1445495.8ns (-99.8%) | [-1591644, -1442968]ns | [2422, 2594] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1444527ns | +0.8% | -99.8% |
| 2 | 1446546ns | +0.6% | -99.8% |
| 3 | 1713939ns | -15.1% | -99.9% |
| 4 | 1474537ns | +6.7% | -99.8% |
| 5 | 1446282ns | +0.5% | -99.8% |
| 6 | 1449318ns | +0.1% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | -0.234 | moderate- |
| abi_residency_leaf_null_entry | 0.060 | ok |
| abi_residency_leaf_reused_buffer | -0.165 | ok |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4458971.1ns | 1473955.4ns | 302.5% | HIGH |
| abi_residency_leaf_null_entry | 113224.6ns | 2488.8ns | 4549.5% | HIGH |
| abi_residency_leaf_reused_buffer | 4451116.6ns | 1495857.9ns | 297.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1451291.7-1514377.7 ns)
  1451291.7 |##########################
  1454446.0 |########################################
  1457600.3 |
  1460754.6 |
  1463908.9 |
  1467063.2 |
  1470217.5 |
  1473371.8 |
  1476526.1 |
  1479680.4 |
  1482834.7 |
  1485989.0 |
  1489143.3 |
  1492297.6 |
  1495451.9 |
  1498606.2 |
  1501760.5 |
  1504914.8 |
  1508069.1 |
  1511223.4 |
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 2418.3-2594.2 ns)
   2418.3 |########################################
   2427.1 |
   2435.9 |
   2444.7 |####################
   2453.5 |####################
   2462.3 |
   2471.1 |
   2479.9 |
   2488.7 |
   2497.5 |
   2506.2 |
   2515.0 |####################
   2523.8 |
   2532.6 |
   2541.4 |
   2550.2 |
   2559.0 |
   2567.8 |
   2576.6 |
   2585.4 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1444526.7-1594237.7 ns)
  1444526.7 |########################################
  1452012.2 |
  1459497.8 |
  1466983.3 |
  1474468.9 |##########
  1481954.4 |
  1489440.0 |
  1496925.6 |
  1504411.1 |
  1511896.6 |
  1519382.2 |
  1526867.8 |
  1534353.3 |
  1541838.8 |
  1549324.4 |
  1556809.9 |
  1564295.5 |
  1571781.1 |
  1579266.6 |
  1586752.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=4585.5% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.6% of algo (FFI overhead may distort results)
