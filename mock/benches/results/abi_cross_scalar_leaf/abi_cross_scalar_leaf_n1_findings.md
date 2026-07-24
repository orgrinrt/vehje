# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 4.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 29925% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (4.93 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.48 ms) by 29925%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.50 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 305.3x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.50 ms) is 305.3x the fastest (4.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_leaf_ffi_batched_scalar shows alternating (throttle bounce) (autocorr -0.57)

abi_cross_scalar_leaf_ffi_batched_scalar's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (29925% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 29925% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 305.3x the fastest

Fastest abi_cross_scalar_leaf_null_entry (4.93 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.50 ms): 305.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 4928.0 ns median (-99.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 305.26x (fastest 4928.0 ns, slowest 1504316.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1493214ns | 1495090ns | 1481516ns | 1493979ns | 1497916ns | -1.00% |
| abi_cross_scalar_leaf_inproc_direct | 1508333ns | 1507329ns | 1502533ns | 1506555ns | 1513900ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1481948ns | 1482705ns | 1471482ns | 1480028ns | 1490060ns | -1.75% |
| abi_cross_scalar_leaf_null_entry | 7300ns | 7222ns | 7058ns | 7215ns | 7548ns | -99.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1489789ns | 1478350ns | 1494132ns | -1.02% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1505131ns | 1498921ns | 1510663ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1478906ns | 1468402ns | 1487044ns | -1.74% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 4971ns | 4790ns | 5135ns | -99.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 63397.3 | 1490044.0 | 1489788.9 | 4 |
| abi_cross_scalar_leaf_inproc_direct | 8334.6 | 1505980.0 | 1505131.3 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 8164.1 | 1479177.1 | 1478906.0 | n/a |
| abi_cross_scalar_leaf_null_entry | 29591.2 | 5108.8 | 4970.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.3% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.3% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.3% |
| abi_cross_scalar_leaf_null_entry | 0.000 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1493214ns | 1493214ns | -1.00% |
| abi_cross_scalar_leaf_inproc_direct | 1508333ns | 1508333ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1481948ns | 1481948ns | -1.75% |
| abi_cross_scalar_leaf_null_entry | 7300ns | 7300ns | -99.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1504316ns | base | --- | [1500415, 1510663] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1491754ns | -13215.0ns (-0.9%) | [-26284, -6528]ns | [1483480, 1494132] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1479635ns | -20954.6ns (-1.4%) | [-40624, -17097]ns | [1470039, 1487044] | YES | 0.0313 | 0.0313 | 0 |
| abi_cross_scalar_leaf_null_entry | 4928ns | -1499180.6ns (-99.7%) | [-1505807, -1495494]ns | [4849, 5135] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1502259ns | -0.6% | -1.4% | -99.7% |
| 2 | 1506373ns | -1.9% | -1.4% | -99.7% |
| 3 | 1501910ns | -0.4% | -0.9% | -99.7% |
| 4 | 1513156ns | -1.6% | -2.7% | -99.7% |
| 5 | 1508170ns | -1.1% | -2.6% | -99.7% |
| 6 | 1498921ns | -0.4% | -1.4% | -99.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | -0.573 | HIGH- (thermal bounce) |
| abi_cross_scalar_leaf_inproc_direct | -0.211 | moderate- |
| abi_cross_scalar_leaf_inproc_fnptr | 0.323 | moderate+ |
| abi_cross_scalar_leaf_null_entry | 0.417 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4536217.2ns | 1489788.9ns | 304.5% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4527982.8ns | 1505131.3ns | 300.8% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4445573.0ns | 1478906.0ns | 300.6% | HIGH |
| abi_cross_scalar_leaf_null_entry | 126556.5ns | 4970.8ns | 2546.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1478350.4-1494132.1 ns)
  1478350.4 |########################################
  1479139.5 |
  1479928.6 |
  1480717.7 |
  1481506.7 |
  1482295.8 |
  1483084.9 |
  1483874.0 |
  1484663.1 |
  1485452.2 |
  1486241.2 |
  1487030.3 |
  1487819.4 |
  1488608.5 |########################################
  1489397.6 |
  1490186.7 |
  1490975.8 |########################################
  1491764.8 |########################################
  1492553.9 |########################################
  1493343.0 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1498920.8-1510662.7 ns)
  1498920.8 |####################
  1499507.9 |
  1500095.0 |
  1500682.1 |
  1501269.2 |
  1501856.3 |########################################
  1502443.4 |
  1503030.5 |
  1503617.6 |
  1504204.7 |
  1504791.8 |
  1505378.8 |
  1505965.9 |####################
  1506553.0 |
  1507140.1 |
  1507727.2 |####################
  1508314.3 |
  1508901.4 |
  1509488.5 |
  1510075.6 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1468401.7-1487044.1 ns)
  1468401.7 |########################################
  1469333.8 |
  1470265.9 |
  1471198.1 |########################################
  1472130.2 |
  1473062.3 |
  1473994.4 |
  1474926.6 |
  1475858.7 |
  1476790.8 |
  1477722.9 |########################################
  1478655.0 |
  1479587.2 |
  1480519.3 |########################################
  1481451.4 |
  1482383.5 |
  1483315.7 |
  1484247.8 |
  1485179.9 |########################################
  1486112.0 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 4790.0-5135.4 ns)
   4790.0 |########################################
   4807.3 |
   4824.5 |
   4841.8 |
   4859.1 |
   4876.4 |
   4893.6 |########################################
   4910.9 |########################################
   4928.2 |########################################
   4945.4 |
   4962.7 |
   4980.0 |
   4997.2 |
   5014.5 |
   5031.8 |
   5049.0 |
   5066.3 |
   5083.6 |
   5100.9 |
   5118.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=300.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=2570.4% of algo (FFI overhead may distort results)
