# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 35673% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (4.06 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 35673%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 359.1x slower than the field

abi_residency_leaf_fresh_alloc (1.46 ms) is 359.1x the fastest (4.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 359.1x the fastest

Fastest abi_residency_leaf_null_entry (4.06 us) to slowest abi_residency_leaf_fresh_alloc (1.46 ms): 359.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 4055.2 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 359.10x (fastest 4055.2 ns, slowest 1456225.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1468291ns | 1458700ns | 1448844ns | 1456698ns | 1495405ns | +0.88% |
| abi_residency_leaf_null_entry | 6410ns | 6332ns | 6240ns | 6324ns | 6625ns | -99.56% |
| abi_residency_leaf_reused_buffer | 1455491ns | 1453099ns | 1448261ns | 1452799ns | 1463145ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1465721ns | 1446440ns | 1492593ns | +0.87% | 0.000 |
| abi_residency_leaf_null_entry | 4094ns | 4007ns | 4211ns | -99.72% | 0.001 |
| abi_residency_leaf_reused_buffer | 1453026ns | 1445923ns | 1460541ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 46862.1 | 1455260.9 | 1465720.9 | n/a |
| abi_residency_leaf_null_entry | 28405.1 | 4218.7 | 4094.0 | n/a |
| abi_residency_leaf_reused_buffer | 38055.9 | 1450337.8 | 1453026.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.3% |
| abi_residency_leaf_null_entry | 0.001 | 98.8% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1468291ns | 1468291ns | +0.88% |
| abi_residency_leaf_null_entry | 6410ns | 6410ns | -99.56% |
| abi_residency_leaf_reused_buffer | 1455491ns | 1455491ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1450685ns | base | --- | [1447853, 1460541] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1456225ns | no significant difference | [-8255, +40798]ns | [1448345, 1492593] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_leaf_null_entry | 4055ns | -1446543.2ns (-99.7%) | [-1456417, -1443836]ns | [4016, 4211] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1463417ns | -1.2% | -99.7% |
| 2 | 1450485ns | +0.5% | -99.7% |
| 3 | 1450885ns | +0.2% | -99.7% |
| 4 | 1457666ns | +0.6% | -99.7% |
| 5 | 1445923ns | +5.1% | -99.7% |
| 6 | 1449782ns | +0.0% | -99.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | -0.159 | ok |
| abi_residency_leaf_null_entry | -0.346 | moderate- |
| abi_residency_leaf_reused_buffer | -0.203 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 1/6, lost 4/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4418440.9ns | 1465720.9ns | 301.5% | HIGH |
| abi_residency_leaf_null_entry | 123389.0ns | 4094.0ns | 3013.9% | HIGH |
| abi_residency_leaf_reused_buffer | 4392449.8ns | 1453026.2ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1446440.4-1492592.8 ns)
  1446440.4 |########################################
  1448748.0 |########################################
  1451055.6 |
  1453363.3 |########################################
  1455670.9 |
  1457978.5 |########################################
  1460286.1 |
  1462593.7 |
  1464901.3 |########################################
  1467209.0 |
  1469516.6 |
  1471824.2 |
  1474131.8 |
  1476439.4 |
  1478747.0 |
  1481054.7 |
  1483362.3 |
  1485669.9 |
  1487977.5 |
  1490285.1 |
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 4006.7-4210.6 ns)
   4006.7 |########################################
   4016.9 |########################################
   4027.1 |
   4037.3 |
   4047.5 |########################################
   4057.7 |########################################
   4067.9 |
   4078.1 |
   4088.3 |
   4098.5 |
   4108.6 |
   4118.8 |
   4129.0 |
   4139.2 |
   4149.4 |
   4159.6 |
   4169.8 |
   4180.0 |
   4190.2 |########################################
   4200.4 |
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1445923.3-1460541.2 ns)
  1445923.3 |####################
  1446654.2 |
  1447385.1 |
  1448116.0 |
  1448846.9 |
  1449577.8 |####################
  1450308.7 |########################################
  1451039.6 |
  1451770.5 |
  1452501.4 |
  1453232.3 |
  1453963.2 |
  1454694.1 |
  1455425.0 |
  1456155.9 |
  1456886.8 |
  1457617.7 |####################
  1458348.6 |
  1459079.5 |
  1459810.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=3046.4% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.5% of algo (FFI overhead may distort results)
