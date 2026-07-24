# abi_residency (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_residency_leaf_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_leaf_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_leaf_null_entry dominates: 45224% faster than the next best (abi_residency_leaf_reused_buffer)

abi_residency_leaf_null_entry (3.20 us) leads abi_residency_leaf_reused_buffer (1.45 ms) by 45224%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_leaf_null_entry beats baseline by 100% (significant)

abi_residency_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_residency_leaf_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_leaf_fresh_alloc is an outlier: 454.2x slower than the field

abi_residency_leaf_fresh_alloc (1.45 ms) is 454.2x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 454.2x the fastest

Fastest abi_residency_leaf_null_entry (3.20 us) to slowest abi_residency_leaf_fresh_alloc (1.45 ms): 454.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_leaf_null_entry** at 3195.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 454.21x (fastest 3195.6 ns, slowest 1451507.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454663ns | 1453999ns | 1449565ns | 1453309ns | 1459242ns | +0.26% |
| abi_residency_leaf_null_entry | 5487ns | 5510ns | 5147ns | 5454ns | 5708ns | -99.62% |
| abi_residency_leaf_reused_buffer | 1450832ns | 1450830ns | 1447221ns | 1450246ns | 1453516ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1452160ns | 1447195ns | 1456627ns | +0.26% | 0.000 |
| abi_residency_leaf_null_entry | 3194ns | 3020ns | 3320ns | -99.78% | 0.080 |
| abi_residency_leaf_reused_buffer | 1448376ns | 1444930ns | 1450946ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 37403.5 | 1451874.2 | 1452160.1 | n/a |
| abi_residency_leaf_null_entry | 27337.8 | 3212.2 | 3194.5 | n/a |
| abi_residency_leaf_reused_buffer | 34772.3 | 1447181.4 | 1448375.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.085 Gops/s** (abi_residency_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.000 | 0.2% |
| abi_residency_leaf_null_entry | 0.080 | 94.5% |
| abi_residency_leaf_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 1454663ns | 1454663ns | +0.26% |
| abi_residency_leaf_null_entry | 5487ns | 5487ns | -99.62% |
| abi_residency_leaf_reused_buffer | 1450832ns | 1450832ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_leaf_reused_buffer | 1448406ns | base | --- | [1445775, 1450946] | --- | --- | --- | --- |
| abi_residency_leaf_fresh_alloc | 1451508ns | +2570.8ns (+0.2%) | [+1434, +7348]ns | [1448346, 1456627] | YES | 0.0313 | 0.0313 | 0 |
| abi_residency_leaf_null_entry | 3196ns | -1445209.9ns (-99.8%) | [-1447726, -1442607]ns | [3068, 3320] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_leaf_reused_buffer | abi_residency_leaf_fresh_alloc | abi_residency_leaf_null_entry |
|---|---|---|---|
| 1 | 1444930ns | +0.2% | -99.8% |
| 2 | 1452371ns | +0.0% | -99.8% |
| 3 | 1449521ns | +0.5% | -99.8% |
| 4 | 1449036ns | +0.5% | -99.8% |
| 5 | 1447775ns | +0.2% | -99.8% |
| 6 | 1446620ns | +0.2% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_leaf_fresh_alloc | 0.188 | ok |
| abi_residency_leaf_null_entry | 0.049 | ok |
| abi_residency_leaf_reused_buffer | -0.235 | moderate- |

**Consistency summary:**

- **abi_residency_leaf_fresh_alloc**: won 0/6, lost 5/6
- **abi_residency_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_leaf_fresh_alloc | 4394389.2ns | 1452160.1ns | 302.6% | HIGH |
| abi_residency_leaf_null_entry | 120953.7ns | 3194.5ns | 3786.4% | HIGH |
| abi_residency_leaf_reused_buffer | 4379886.8ns | 1448375.6ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_leaf_fresh_alloc (n=6, range 1447195.4-1456626.7 ns)
  1447195.4 |########################################
  1447667.0 |
  1448138.5 |
  1448610.1 |
  1449081.7 |########################################
  1449553.2 |########################################
  1450024.8 |
  1450496.4 |
  1450967.9 |
  1451439.5 |
  1451911.1 |
  1452382.6 |
  1452854.2 |########################################
  1453325.7 |
  1453797.3 |
  1454268.9 |
  1454740.4 |
  1455212.0 |
  1455683.6 |
  1456155.1 |########################################
  (0 below, 1 above range)

abi_residency_leaf_null_entry (n=6, range 3020.0-3320.2 ns)
   3020.0 |########################################
   3035.0 |
   3050.0 |
   3065.0 |
   3080.0 |
   3095.1 |
   3110.1 |########################################
   3125.1 |
   3140.1 |
   3155.1 |
   3170.1 |########################################
   3185.1 |
   3200.1 |########################################
   3215.1 |
   3230.1 |
   3245.1 |
   3260.2 |
   3275.2 |
   3290.2 |
   3305.2 |########################################
  (0 below, 1 above range)

abi_residency_leaf_reused_buffer (n=6, range 1444930.4-1450946.2 ns)
  1444930.4 |########################################
  1445231.2 |
  1445532.0 |
  1445832.8 |
  1446133.6 |
  1446434.3 |########################################
  1446735.1 |
  1447035.9 |
  1447336.7 |
  1447637.5 |########################################
  1447938.3 |
  1448239.1 |
  1448539.9 |
  1448840.7 |########################################
  1449141.5 |
  1449442.2 |########################################
  1449743.0 |
  1450043.8 |
  1450344.6 |
  1450645.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_leaf_fresh_alloc**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_residency_leaf_null_entry**: bridge=3777.8% of algo (FFI overhead may distort results)
- **abi_residency_leaf_reused_buffer**: bridge=302.5% of algo (FFI overhead may distort results)
