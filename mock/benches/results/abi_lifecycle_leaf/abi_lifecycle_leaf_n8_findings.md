# abi_lifecycle (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_leaf_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_leaf_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_leaf_null_entry dominates: 46994% faster than the next best (abi_lifecycle_leaf_held_handle)

abi_lifecycle_leaf_null_entry (3.09 us) leads abi_lifecycle_leaf_held_handle (1.45 ms) by 46994%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_leaf_null_entry beats baseline by 100% (significant)

abi_lifecycle_leaf_null_entry is -1.45 ms (100%) faster than baseline abi_lifecycle_leaf_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_leaf_fresh_per_batch is an outlier: 590.4x slower than the field

abi_lifecycle_leaf_fresh_per_batch (1.82 ms) is 590.4x the fastest (3.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_leaf_fresh_per_batch shows alternating (throttle bounce) (autocorr -0.63)

abi_lifecycle_leaf_fresh_per_batch's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_leaf_null_entry} vs {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} (46994% apart)

The field splits into a fast tier {abi_lifecycle_leaf_null_entry} and a slow tier {abi_lifecycle_leaf_held_handle, abi_lifecycle_leaf_fresh_per_column, abi_lifecycle_leaf_fresh_per_batch} with a 46994% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 590.4x the fastest

Fastest abi_lifecycle_leaf_null_entry (3.09 us) to slowest abi_lifecycle_leaf_fresh_per_batch (1.82 ms): 590.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_leaf_null_entry** at 3088.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 590.39x (fastest 3088.1 ns, slowest 1823210.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1827246ns | 1826012ns | 1818406ns | 1824331ns | 1836038ns | +25.28% |
| abi_lifecycle_leaf_fresh_per_column | 1475273ns | 1474305ns | 1468717ns | 1472746ns | 1482343ns | +1.15% |
| abi_lifecycle_leaf_held_handle | 1458495ns | 1456870ns | 1454432ns | 1456295ns | 1463826ns | base |
| abi_lifecycle_leaf_null_entry | 5409ns | 5382ns | 5095ns | 5342ns | 5667ns | -99.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1824444ns | 1815800ns | 1832991ns | +25.33% | 0.000 |
| abi_lifecycle_leaf_fresh_per_column | 1472476ns | 1466022ns | 1479396ns | +1.15% | 0.000 |
| abi_lifecycle_leaf_held_handle | 1455739ns | 1451778ns | 1460809ns | base | 0.000 |
| abi_lifecycle_leaf_null_entry | 3093ns | 2929ns | 3245ns | -99.79% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 49904.5 | 1824563.6 | 1824444.2 | n/a |
| abi_lifecycle_leaf_fresh_per_column | 45751.8 | 1472790.6 | 1472475.8 | n/a |
| abi_lifecycle_leaf_held_handle | 45120.1 | 1456766.5 | 1455738.5 | n/a |
| abi_lifecycle_leaf_null_entry | 27783.1 | 3187.4 | 3092.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_lifecycle_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 0.000 | 0.2% |
| abi_lifecycle_leaf_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_leaf_held_handle | 0.000 | 0.2% |
| abi_lifecycle_leaf_null_entry | 0.003 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 1827246ns | 1827246ns | +25.28% |
| abi_lifecycle_leaf_fresh_per_column | 1475273ns | 1475273ns | +1.15% |
| abi_lifecycle_leaf_held_handle | 1458495ns | 1458495ns | base |
| abi_lifecycle_leaf_null_entry | 5409ns | 5409ns | -99.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_leaf_held_handle | 1454331ns | base | --- | [1452075, 1460809] | --- | --- | --- | --- |
| abi_lifecycle_leaf_fresh_per_batch | 1823210ns | +366625.0ns (+25.2%) | [+359703, +379789]ns | [1817131, 1832991] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_leaf_fresh_per_column | 1471545ns | +15212.5ns (+1.0%) | [+11167, +23833]ns | [1466486, 1479396] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_leaf_null_entry | 3088ns | -1451316.6ns (-99.8%) | [-1457732, -1448889]ns | [2945, 3245] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_leaf_held_handle | abi_lifecycle_leaf_fresh_per_batch | abi_lifecycle_leaf_fresh_per_column | abi_lifecycle_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1462864ns | +24.4% | +0.7% | -99.8% |
| 2 | 1451778ns | +26.1% | +1.0% | -99.8% |
| 3 | 1452372ns | +25.2% | +1.6% | -99.8% |
| 4 | 1458755ns | +25.2% | +1.7% | -99.8% |
| 5 | 1454036ns | +24.9% | +0.8% | -99.8% |
| 6 | 1454626ns | +26.2% | +1.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | -0.628 | HIGH- (thermal bounce) |
| abi_lifecycle_leaf_fresh_per_column | -0.196 | ok |
| abi_lifecycle_leaf_held_handle | -0.311 | moderate- |
| abi_lifecycle_leaf_null_entry | 0.332 | moderate+ |

**Consistency summary:**

- **abi_lifecycle_leaf_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_leaf_fresh_per_batch | 5527327.0ns | 1824444.2ns | 303.0% | HIGH |
| abi_lifecycle_leaf_fresh_per_column | 4465915.5ns | 1472475.8ns | 303.3% | HIGH |
| abi_lifecycle_leaf_held_handle | 4418223.4ns | 1455738.5ns | 303.5% | HIGH |
| abi_lifecycle_leaf_null_entry | 119976.7ns | 3092.7ns | 3879.4% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_leaf_fresh_per_batch (n=6, range 1815799.6-1832991.0 ns)
  1815799.6 |########################################
  1816659.2 |
  1817518.7 |
  1818378.3 |########################################
  1819237.9 |
  1820097.5 |########################################
  1820957.0 |
  1821816.6 |
  1822676.2 |
  1823535.7 |
  1824395.3 |
  1825254.9 |########################################
  1826114.4 |
  1826974.0 |
  1827833.6 |
  1828693.1 |
  1829552.7 |
  1830412.3 |########################################
  1831271.9 |
  1832131.4 |
  (0 below, 1 above range)

abi_lifecycle_leaf_fresh_per_column (n=6, range 1466022.5-1479396.4 ns)
  1466022.5 |########################################
  1466691.2 |########################################
  1467359.9 |
  1468028.6 |
  1468697.3 |
  1469366.0 |########################################
  1470034.7 |
  1470703.4 |
  1471372.1 |
  1472040.8 |
  1472709.5 |########################################
  1473378.2 |
  1474046.9 |
  1474715.6 |
  1475384.3 |########################################
  1476053.0 |
  1476721.7 |
  1477390.4 |
  1478059.1 |
  1478727.8 |
  (0 below, 1 above range)

abi_lifecycle_leaf_held_handle (n=6, range 1451778.3-1460809.4 ns)
  1451778.3 |########################################
  1452229.9 |########################################
  1452681.4 |
  1453133.0 |
  1453584.5 |########################################
  1454036.1 |
  1454487.6 |########################################
  1454939.2 |
  1455390.7 |
  1455842.3 |
  1456293.9 |
  1456745.4 |
  1457197.0 |
  1457648.5 |
  1458100.1 |
  1458551.6 |########################################
  1459003.2 |
  1459454.7 |
  1459906.3 |
  1460357.8 |
  (0 below, 1 above range)

abi_lifecycle_leaf_null_entry (n=6, range 2928.8-3244.6 ns)
   2928.8 |########################################
   2944.6 |
   2960.4 |########################################
   2976.2 |
   2992.0 |
   3007.7 |
   3023.5 |
   3039.3 |
   3055.1 |########################################
   3070.9 |
   3086.7 |
   3102.5 |########################################
   3118.2 |
   3134.0 |
   3149.8 |
   3165.6 |
   3181.4 |
   3197.2 |
   3213.0 |########################################
   3228.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_leaf_fresh_per_batch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_fresh_per_column**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_held_handle**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_leaf_null_entry**: bridge=3881.5% of algo (FFI overhead may distort results)
