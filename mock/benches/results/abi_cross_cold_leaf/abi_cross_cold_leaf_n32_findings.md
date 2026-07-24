# abi_cross_cold (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_leaf_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_leaf_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_leaf_warm_scalar is an outlier: 664.4x slower than the field

abi_cross_cold_leaf_warm_scalar (1.53 ms) is 664.4x the fastest (2.31 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_leaf_warm_null is fastest but the noisiest (CV 5.7%)

abi_cross_cold_leaf_warm_null wins on median (2.31 us) yet has the highest variance (CV 5.7%), while abi_cross_cold_leaf_cold_null is the steadiest (CV 0.9%, 2.35 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (abi_cross_cold_leaf_warm_null)

The baseline abi_cross_cold_leaf_warm_null is the fastest (2.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} vs {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} (64922% apart)

The field splits into a fast tier {abi_cross_cold_leaf_warm_null, abi_cross_cold_leaf_cold_null} and a slow tier {abi_cross_cold_leaf_cold_scalar, abi_cross_cold_leaf_warm_scalar} with a 64922% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 664.4x the fastest

Fastest abi_cross_cold_leaf_warm_null (2.31 us) to slowest abi_cross_cold_leaf_warm_scalar (1.53 ms): 664.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader abi_cross_cold_leaf_warm_null vs stability leader abi_cross_cold_leaf_cold_null (+2% speed for 6.3x steadier)

abi_cross_cold_leaf_warm_null is fastest (2.31 us, CV 5.7%); abi_cross_cold_leaf_cold_null gives up 1.7% median for 6.3x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### abi_cross_cold_leaf_cold_null's edge over baseline is significant but tiny (30 ns, 1.32%)

abi_cross_cold_leaf_cold_null differs from baseline abi_cross_cold_leaf_warm_null by 30 ns (1.32%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_leaf_warm_null) is the fastest** at 2306.6 ns median
- 2 variants significantly slower than baseline
- Spread: 664.36x (fastest 2306.6 ns, slowest 1532456.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 4638ns | 4632ns | 4523ns | 4613ns | 4733ns | -2.60% |
| abi_cross_cold_leaf_cold_scalar | 1522889ns | 1528707ns | 1495918ns | 1519845ns | 1540941ns | +31882.27% |
| abi_cross_cold_leaf_warm_null | 4762ns | 4619ns | 4534ns | 4601ns | 5116ns | base |
| abi_cross_cold_leaf_warm_scalar | 1538620ns | 1535948ns | 1511723ns | 1531418ns | 1562872ns | +32212.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 2348ns | 2321ns | 2375ns | -1.15% | 0.014 |
| abi_cross_cold_leaf_cold_scalar | 1519443ns | 1492574ns | 1537454ns | +63863.51% | 0.000 |
| abi_cross_cold_leaf_warm_null | 2375ns | 2257ns | 2558ns | base | 0.013 |
| abi_cross_cold_leaf_warm_scalar | 1535028ns | 1508434ns | 1559074ns | +64519.62% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 33143.4 | 2477.2 | 2348.1 | 82 |
| abi_cross_cold_leaf_cold_scalar | 73842.8 | 1519989.5 | 1519442.6 | n/a |
| abi_cross_cold_leaf_warm_null | 30889.2 | 2519.0 | 2375.5 | n/a |
| abi_cross_cold_leaf_warm_scalar | 67895.4 | 1537526.5 | 1535028.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_cold_leaf_warm_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_leaf_cold_null | 0.014 | 96.2% |
| abi_cross_cold_leaf_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_leaf_warm_null | 0.014 | 97.9% |
| abi_cross_cold_leaf_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 4638ns | 4638ns | -2.60% |
| abi_cross_cold_leaf_cold_scalar | 1522889ns | 1522889ns | +31882.27% |
| abi_cross_cold_leaf_warm_null | 4762ns | 4762ns | base |
| abi_cross_cold_leaf_warm_scalar | 1538620ns | 1538620ns | +32212.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_leaf_warm_null | 2307ns | base | --- | [2262, 2558] | --- | --- | --- | --- |
| abi_cross_cold_leaf_cold_null | 2346ns | no significant difference | [-212, +99]ns | [2324, 2375] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_leaf_cold_scalar | 1525317ns | +1522904.4ns (+66022.3%) | [+1493149, +1535148]ns | [1495557, 1537454] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_leaf_warm_scalar | 1532457ns | +1530150.0ns (+66336.5%) | [+1511146, +1556662]ns | [1513554, 1559074] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_leaf_warm_null | abi_cross_cold_leaf_cold_null | abi_cross_cold_leaf_cold_scalar | abi_cross_cold_leaf_warm_scalar |
|---|---|---|---|---|
| 1 | 2267ns | +2.4% | +67421.7% | +68902.6% |
| 2 | 2558ns | -8.2% | +58493.9% | +60654.3% |
| 3 | 2294ns | +3.5% | +66883.7% | +66584.8% |
| 4 | 2320ns | +0.3% | +66226.2% | +66090.8% |
| 5 | 2257ns | +5.3% | +66028.0% | +67184.3% |
| 6 | 2558ns | -8.4% | +59317.8% | +58871.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_leaf_cold_null | -0.486 | moderate- |
| abi_cross_cold_leaf_cold_scalar | -0.408 | moderate- |
| abi_cross_cold_leaf_warm_null | -0.441 | moderate- |
| abi_cross_cold_leaf_warm_scalar | 0.395 | moderate+ |

**Consistency summary:**

- **abi_cross_cold_leaf_cold_null**: won 2/6, lost 4/6
- **abi_cross_cold_leaf_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_leaf_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_leaf_cold_null | 122401.3ns | 2348.1ns | 5212.7% | HIGH |
| abi_cross_cold_leaf_cold_scalar | 4639532.6ns | 1519442.6ns | 305.3% | HIGH |
| abi_cross_cold_leaf_warm_null | 120212.3ns | 2375.5ns | 5060.5% | HIGH |
| abi_cross_cold_leaf_warm_scalar | 4681548.3ns | 1535028.3ns | 305.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_leaf_cold_null (n=6, range 2321.2-2374.8 ns)
   2321.2 |########################################
   2323.9 |########################################
   2326.6 |
   2329.2 |
   2331.9 |
   2334.6 |
   2337.3 |
   2340.0 |
   2342.6 |########################################
   2345.3 |########################################
   2348.0 |
   2350.7 |
   2353.4 |
   2356.0 |
   2358.7 |
   2361.4 |
   2364.1 |
   2366.8 |
   2369.4 |
   2372.1 |########################################
  (0 below, 1 above range)

abi_cross_cold_leaf_cold_scalar (n=6, range 1492574.2-1537454.4 ns)
  1492574.2 |########################################
  1494818.2 |
  1497062.2 |########################################
  1499306.2 |
  1501550.2 |
  1503794.2 |
  1506038.2 |
  1508282.3 |
  1510526.3 |
  1512770.3 |
  1515014.3 |
  1517258.3 |
  1519502.3 |########################################
  1521746.3 |
  1523990.3 |
  1526234.3 |
  1528478.3 |
  1530722.3 |########################################
  1532966.3 |
  1535210.3 |########################################
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_null (n=6, range 2257.1-2557.7 ns)
   2257.1 |########################################
   2272.1 |
   2287.2 |####################
   2302.2 |
   2317.2 |####################
   2332.2 |
   2347.3 |
   2362.3 |
   2377.3 |
   2392.4 |
   2407.4 |
   2422.4 |
   2437.5 |
   2452.5 |
   2467.5 |
   2482.5 |
   2497.6 |
   2512.6 |
   2527.6 |
   2542.7 |####################
  (0 below, 1 above range)

abi_cross_cold_leaf_warm_scalar (n=6, range 1508433.8-1559074.4 ns)
  1508433.8 |########################################
  1510965.8 |
  1513497.9 |
  1516029.9 |
  1518561.9 |########################################
  1521093.9 |
  1523626.0 |
  1526158.0 |
  1528690.0 |########################################
  1531222.1 |
  1533754.1 |########################################
  1536286.1 |
  1538818.2 |
  1541350.2 |
  1543882.2 |
  1546414.2 |
  1548946.3 |
  1551478.3 |########################################
  1554010.3 |
  1556542.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_leaf_cold_null**: bridge=5241.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_cold_scalar**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_null**: bridge=5211.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_leaf_warm_scalar**: bridge=305.3% of algo (FFI overhead may distort results)
