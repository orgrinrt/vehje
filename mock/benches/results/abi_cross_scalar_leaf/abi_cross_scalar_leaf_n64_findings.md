# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.55 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 2.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 59945% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (2.53 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.52 ms) by 59945%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.55 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 611.6x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.55 ms) is 611.6x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (59945% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 59945% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 611.6x the fastest

Fastest abi_cross_scalar_leaf_null_entry (2.53 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.55 ms): 611.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 2533.1 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 611.58x (fastest 2533.1 ns, slowest 1549187.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1527709ns | 1526925ns | 1500151ns | 1518496ns | 1555306ns | -4.46% |
| abi_cross_scalar_leaf_inproc_direct | 1598945ns | 1552499ns | 1531716ns | 1549017ns | 1707452ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1541283ns | 1524655ns | 1499065ns | 1518419ns | 1596688ns | -3.61% |
| abi_cross_scalar_leaf_null_entry | 4829ns | 4838ns | 4672ns | 4797ns | 4954ns | -99.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1524096ns | 1496477ns | 1551654ns | -4.47% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1595401ns | 1528522ns | 1703500ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1537617ns | 1495620ns | 1592698ns | -3.62% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 2531ns | 2432ns | 2614ns | -99.84% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 69541.1 | 1524365.6 | 1524096.2 | 1 |
| abi_cross_scalar_leaf_inproc_direct | 9521.6 | 1600195.6 | 1595400.6 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 10033.4 | 1535972.9 | 1537616.9 | 0 |
| abi_cross_scalar_leaf_null_entry | 29665.2 | 2747.3 | 2530.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.025 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1527709ns | 1527709ns | -4.46% |
| abi_cross_scalar_leaf_inproc_direct | 1598945ns | 1598945ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1541283ns | 1541283ns | -3.61% |
| abi_cross_scalar_leaf_null_entry | 4829ns | 4829ns | -99.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1549187ns | base | --- | [1533515, 1703500] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1523142ns | -23261.5ns (-1.5%) | [-181441, -9211]ns | [1497492, 1551654] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1520997ns | no significant difference | [-191625, +53109]ns | [1499156, 1592698] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_leaf_null_entry | 2533ns | -1546654.2ns (-99.8%) | [-1700967, -1530989]ns | [2444, 2614] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1839438ns | -17.1% | -17.3% | -99.9% |
| 2 | 1554169ns | -0.4% | -1.7% | -99.8% |
| 3 | 1567561ns | -0.8% | -4.1% | -99.8% |
| 4 | 1544206ns | -3.1% | -1.5% | -99.8% |
| 5 | 1528522ns | -2.0% | +8.5% | -99.8% |
| 6 | 1538508ns | -1.1% | -2.8% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.224 | moderate+ |
| abi_cross_scalar_leaf_inproc_direct | -0.004 | ok |
| abi_cross_scalar_leaf_inproc_fnptr | -0.328 | moderate- |
| abi_cross_scalar_leaf_null_entry | -0.150 | ok |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 5/6, lost 1/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4634235.8ns | 1524096.2ns | 304.1% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4809838.8ns | 1595400.6ns | 301.5% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4611315.5ns | 1537616.9ns | 299.9% | HIGH |
| abi_cross_scalar_leaf_null_entry | 115563.3ns | 2530.6ns | 4566.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1496476.7-1551653.9 ns)
  1496476.7 |########################################
  1499235.6 |
  1501994.4 |
  1504753.3 |
  1507512.1 |
  1510271.0 |
  1513029.9 |
  1515788.7 |
  1518547.6 |
  1521306.5 |####################
  1524065.3 |####################
  1526824.2 |
  1529583.1 |
  1532341.9 |
  1535100.8 |
  1537859.6 |
  1540618.5 |
  1543377.4 |
  1546136.2 |####################
  1548895.1 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1528522.5-1703499.6 ns)
  1528522.5 |####################
  1537271.4 |########################################
  1546020.2 |####################
  1554769.1 |
  1563517.9 |####################
  1572266.8 |
  1581015.6 |
  1589764.5 |
  1598513.3 |
  1607262.2 |
  1616011.0 |
  1624759.9 |
  1633508.7 |
  1642257.6 |
  1651006.4 |
  1659755.3 |
  1668504.1 |
  1677253.0 |
  1686001.8 |
  1694750.7 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1495619.6-1592697.7 ns)
  1495619.6 |####################
  1500473.5 |####################
  1505327.4 |
  1510181.3 |
  1515035.2 |
  1519889.1 |########################################
  1524743.0 |####################
  1529596.9 |
  1534450.8 |
  1539304.7 |
  1544158.6 |
  1549012.6 |
  1553866.5 |
  1558720.4 |
  1563574.3 |
  1568428.2 |
  1573282.1 |
  1578136.0 |
  1582989.9 |
  1587843.8 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 2432.5-2614.4 ns)
   2432.5 |########################################
   2441.6 |
   2450.7 |########################################
   2459.8 |
   2468.9 |
   2478.0 |
   2487.1 |
   2496.2 |
   2505.3 |########################################
   2514.4 |
   2523.4 |
   2532.5 |
   2541.6 |
   2550.7 |########################################
   2559.8 |
   2568.9 |
   2578.0 |
   2587.1 |
   2596.2 |
   2605.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=299.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=4596.2% of algo (FFI overhead may distort results)
