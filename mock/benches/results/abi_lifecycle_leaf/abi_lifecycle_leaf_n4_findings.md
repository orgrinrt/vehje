# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 35554% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (4.10 us) leads abi_lifecycle_leaf_held_handle (1.46 ms) by 35554%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.46 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 535.8x slower than the field

abi_lifecycle_leaf_fresh_per_batch (2.20 ms) is 535.8x the fastest (4.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (35554% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 35554% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 535.8x the fastest

Fastest abi_lifecycle_leaf_null_entry (4.10 us) to slowest abi_lifecycle_leaf_fresh_per_batch (2.20 ms): 535.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 4099.0 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 535.77x (fastest 4099.0 ns, slowest 2196095.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 2199271ns | 2199109ns | 2191652ns | 2197547ns | 2205666ns | +50.31% |
| abi_lifecycle_leaf_fresh_per_column | 1476909ns | 1477658ns | 1467292ns | 1476209ns | 1482768ns | +0.94% |
| abi_lifecycle_leaf_held_handle | 1463117ns | 1464179ns | 1457440ns | 1462163ns | 1467385ns | base |
| abi_lifecycle_leaf_null_entry | 6393ns | 6395ns | 6163ns | 6389ns | 6515ns | -99.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 2196338ns | 2188558ns | 2202972ns | +50.40% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1474084ns | 1464911ns | 1479680ns | +0.94% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1460371ns | 1454875ns | 1464459ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 4090ns | 3940ns | 4164ns | -99.72% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 50162.9 | 2193281.2 | 2196337.5 | n/a |
| abi_lifecycle_leaf_fresh_per_column | 46438.8 | 1478126.4 | 1474083.9 | n/a |
| abi_lifecycle_leaf_held_handle | 45037.6 | 1459530.9 | 1460371.4 | n/a |
| abi_lifecycle_leaf_null_entry | 27210.3 | 4189.9 | 4090.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.2% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.3% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.3% |
| abi_lifecycle_leaf_null_entry | 0.001 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 2199271ns | 2199271ns | +50.31% |
| abi_lifecycle_leaf_fresh_per_column | 1476909ns | 1476909ns | +0.94% |
| abi_lifecycle_leaf_held_handle | 1463117ns | 1463117ns | base |
| abi_lifecycle_leaf_null_entry | 6393ns | 6393ns | -99.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1461451ns | base | --- | [1455204, 1464459] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 2196096ns | +736900.4ns (+50.4%) | [+727390, +743608]ns | [2189945, 2202972] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1474904ns | +14416.6ns (+1.0%) | [+10271, +16450]ns | [1467669, 1479680] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_leaf_null_entry | 4099ns | -1457333.1ns (-99.7%) | [-1460315, -1451195]ns | [4008, 4164] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1463821ns | +50.4% | +0.8% | -99.7% |
| 2 | 1461542ns | +50.0% | +0.9% | -99.7% |
| 3 | 1455532ns | +51.5% | +0.6% | -99.7% |
| 4 | 1454875ns | +50.6% | +1.1% | -99.7% |
| 5 | 1465098ns | +49.4% | +1.1% | -99.7% |
| 6 | 1461361ns | +50.5% | +1.1% | -99.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.361 | moderate- |
| abi_lifecycle_leaf_fresh_per_column | 0.164 | ok |
| abi_lifecycle_leaf_held_handle | 0.041 | ok |
| abi_lifecycle_leaf_null_entry | -0.355 | moderate- |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 6628189.4ns | 2196337.5ns | 301.8% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4477841.2ns | 1474083.9ns | 303.8% | HIGH |
| abi_lifecycle_leaf_held_handle | 4427153.5ns | 1460371.4ns | 303.2% | HIGH |
| abi_lifecycle_leaf_null_entry | 122401.4ns | 4090.4ns | 2992.4% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 2188558.3-2202972.0 ns)
  2188558.3 |########################################
  2189279.0 |
  2189999.7 |
  2190720.4 |########################################
  2191441.0 |
  2192161.7 |########################################
  2192882.4 |
  2193603.1 |
  2194323.8 |
  2195044.5 |
  2195765.2 |
  2196485.9 |
  2197206.5 |
  2197927.2 |
  2198647.9 |########################################
  2199368.6 |
  2200089.3 |
  2200810.0 |########################################
  2201530.7 |
  2202251.4 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1464911.2-1479679.6 ns)
  1464911.2 |####################
  1465649.6 |
  1466388.0 |
  1467126.5 |
  1467864.9 |
  1468603.3 |
  1469341.7 |
  1470080.1 |####################
  1470818.6 |
  1471557.0 |
  1472295.4 |
  1473033.8 |
  1473772.2 |
  1474510.7 |########################################
  1475249.1 |
  1475987.5 |
  1476725.9 |
  1477464.3 |####################
  1478202.8 |
  1478941.2 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1454875.0-1464459.4 ns)
  1454875.0 |####################
  1455354.2 |####################
  1455833.4 |
  1456312.7 |
  1456791.9 |
  1457271.1 |
  1457750.3 |
  1458229.5 |
  1458708.7 |
  1459188.0 |
  1459667.2 |
  1460146.4 |
  1460625.6 |
  1461104.8 |########################################
  1461584.0 |
  1462063.3 |
  1462542.5 |
  1463021.7 |
  1463500.9 |####################
  1463980.1 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 3939.6-4164.1 ns)
   3939.6 |########################################
   3950.8 |
   3962.1 |
   3973.3 |
   3984.5 |
   3995.7 |
   4007.0 |
   4018.2 |
   4029.4 |
   4040.6 |
   4051.9 |
   4063.1 |
   4074.3 |########################################
   4085.6 |########################################
   4096.8 |
   4108.0 |########################################
   4119.2 |########################################
   4130.5 |
   4141.7 |
   4152.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=2986.9% of algo (FFI overhead may distort results)
