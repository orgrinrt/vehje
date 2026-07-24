# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 28661% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (5.06 us) leads abi_residency_leaf_reused_buffer (1.46 ms) by 28661%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 288.7x slower than the field

abi_residency_leaf_fresh_alloc (1.46 ms) is 288.7x the fastest (5.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 288.7x the fastest

Fastest abi_residency_leaf_null_entry (5.06 us) to slowest abi_residency_leaf_fresh_alloc (1.46 ms): 288.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 5060.8 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 288.67x (fastest 5060.8 ns, slowest 1460878.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1617483ns | 1463324ns | 1459941ns | 1462628ns | 1928535ns | +10.64% |
| abi_residency_leaf_null_entry | 7444ns | 7390ns | 6945ns | 7249ns | 7987ns | -99.49% |
| abi_residency_leaf_reused_buffer | 1461927ns | 1458076ns | 1449376ns | 1456487ns | 1476363ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1614742ns | 1457528ns | 1925188ns | +10.65% | 0.000 |
| abi_residency_leaf_null_entry | 5084ns | 4752ns | 5432ns | -99.65% | 0.000 |
| abi_residency_leaf_reused_buffer | 1459325ns | 1446898ns | 1473650ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 46086.5 | 1528485.3 | 1614741.8 | n/a |
| abi_residency_leaf_null_entry | 29047.8 | 5369.6 | 5083.8 | n/a |
| abi_residency_leaf_reused_buffer | 38026.0 | 1460061.9 | 1459325.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.3% |
| abi_residency_leaf_null_entry | 0.000 | 93.9% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1617483ns | 1617483ns | +10.64% |
| abi_residency_leaf_null_entry | 7444ns | 7444ns | -99.49% |
| abi_residency_leaf_reused_buffer | 1461927ns | 1461927ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1455528ns | base | --- | [1448797, 1473650] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1460878ns | +7030.7ns (+0.5%) | [+2631, +456588]ns | [1458159, 1925188] | YES | 0.0313 | 0.0313 | 0 |
| abi_residency_leaf_null_entry | 5061ns | -1450769.9ns (-99.7%) | [-1468589, -1443365]ns | [4758, 5432] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1450697ns | +0.7% | -99.6% |
| 2 | 1455765ns | +0.1% | -99.7% |
| 3 | 1455290ns | +0.2% | -99.7% |
| 4 | 1446898ns | +38.4% | -99.6% |
| 5 | 1490302ns | +24.0% | -99.7% |
| 6 | 1456998ns | +0.3% | -99.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.143 | ok |
| abi_residency_leaf_null_entry | -0.303 | moderate- |
| abi_residency_leaf_reused_buffer | -0.296 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 0/6, lost 6/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4670699.8ns | 1614741.8ns | 289.3% | HIGH |
| abi_residency_leaf_null_entry | 126942.1ns | 5083.8ns | 2497.0% | HIGH |
| abi_residency_leaf_reused_buffer | 4442184.0ns | 1459325.1ns | 304.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1457527.9-1925188.1 ns)
  1457527.9 |########################################
  1480910.9 |
  1504293.9 |
  1527676.9 |
  1551059.9 |
  1574443.0 |
  1597826.0 |
  1621209.0 |
  1644592.0 |
  1667975.0 |
  1691358.0 |
  1714741.0 |
  1738124.0 |
  1761507.1 |
  1784890.1 |
  1808273.1 |
  1831656.1 |##########
  1855039.1 |
  1878422.1 |
  1901805.1 |
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 4752.1-5432.5 ns)
   4752.1 |########################################
   4786.1 |
   4820.1 |
   4854.2 |
   4888.2 |
   4922.2 |
   4956.2 |
   4990.2 |####################
   5024.3 |
   5058.3 |
   5092.3 |
   5126.3 |########################################
   5160.3 |
   5194.4 |
   5228.4 |
   5262.4 |
   5296.4 |
   5330.4 |
   5364.5 |
   5398.5 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1446897.5-1473650.0 ns)
  1446897.5 |####################
  1448235.1 |
  1449572.8 |####################
  1450910.4 |
  1452248.0 |
  1453585.6 |
  1454923.2 |########################################
  1456260.9 |####################
  1457598.5 |
  1458936.1 |
  1460273.8 |
  1461611.4 |
  1462949.0 |
  1464286.6 |
  1465624.2 |
  1466961.9 |
  1468299.5 |
  1469637.1 |
  1470974.8 |
  1472312.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=2485.6% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.7% of algo (FFI overhead may distort results)
