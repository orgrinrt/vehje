# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.54 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 2.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 54077% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (2.78 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.50 ms) by 54077%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.54 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 554.6x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.54 ms) is 554.6x the fastest (2.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (54077% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 54077% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 554.6x the fastest

Fastest abi_cross_scalar_leaf_null_entry (2.78 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.54 ms): 554.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 2775.8 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 554.61x (fastest 2775.8 ns, slowest 1539479.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1539190ns | 1536823ns | 1500051ns | 1531395ns | 1570452ns | -0.75% |
| abi_cross_scalar_leaf_inproc_direct | 1550780ns | 1542944ns | 1537368ns | 1542044ns | 1570591ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1507006ns | 1507261ns | 1492710ns | 1503835ns | 1518911ns | -2.82% |
| abi_cross_scalar_leaf_null_entry | 5088ns | 5095ns | 5001ns | 5071ns | 5157ns | -99.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1535242ns | 1496521ns | 1566198ns | -0.78% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1547325ns | 1533785ns | 1567219ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1503438ns | 1489319ns | 1515013ns | -2.84% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 2770ns | 2718ns | 2808ns | -99.82% | 0.046 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 80829.4 | 1528687.3 | 1535241.9 | 6 |
| abi_cross_scalar_leaf_inproc_direct | 8975.6 | 1548028.1 | 1547325.2 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 9143.9 | 1507026.5 | 1503437.9 | 0 |
| abi_cross_scalar_leaf_null_entry | 30058.6 | 2850.0 | 2770.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.047 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.046 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1539190ns | 1539190ns | -0.75% |
| abi_cross_scalar_leaf_inproc_direct | 1550780ns | 1550780ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1507006ns | 1507006ns | -2.82% |
| abi_cross_scalar_leaf_null_entry | 5088ns | 5088ns | -99.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1539480ns | base | --- | [1535276, 1567219] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1532938ns | no significant difference | [-42593, +24802]ns | [1506590, 1566198] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1503842ns | -45849.6ns (-3.0%) | [-63378, -22435]ns | [1491459, 1515013] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_leaf_null_entry | 2776ns | -1536679.0ns (-99.8%) | [-1564493, -1532494]ns | [2727, 2808] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1537848ns | +0.2% | -2.9% | -99.8% |
| 2 | 1579985ns | -1.7% | -4.6% | -99.8% |
| 3 | 1533785ns | +3.0% | -0.8% | -99.8% |
| 4 | 1541111ns | -1.6% | -2.1% | -99.8% |
| 5 | 1536768ns | -0.8% | -3.1% | -99.8% |
| 6 | 1554454ns | -3.7% | -3.5% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.161 | ok |
| abi_cross_scalar_leaf_inproc_direct | -0.440 | moderate- |
| abi_cross_scalar_leaf_inproc_fnptr | 0.131 | ok |
| abi_cross_scalar_leaf_null_entry | -0.308 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 4/6, lost 2/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 6/6, lost 0/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4697619.5ns | 1535241.9ns | 306.0% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4659184.1ns | 1547325.2ns | 301.1% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4530597.1ns | 1503437.9ns | 301.3% | HIGH |
| abi_cross_scalar_leaf_null_entry | 122921.0ns | 2770.1ns | 4437.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1496520.8-1566197.9 ns)
  1496520.8 |########################################
  1500004.7 |
  1503488.5 |
  1506972.4 |
  1510456.2 |
  1513940.1 |########################################
  1517423.9 |
  1520907.8 |########################################
  1524391.6 |
  1527875.5 |
  1531359.4 |
  1534843.2 |
  1538327.1 |########################################
  1541810.9 |
  1545294.8 |
  1548778.6 |
  1552262.5 |########################################
  1555746.3 |
  1559230.2 |
  1562714.0 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1533785.0-1567219.4 ns)
  1533785.0 |########################################
  1535456.7 |########################################
  1537128.4 |########################################
  1538800.2 |
  1540471.9 |########################################
  1542143.6 |
  1543815.3 |
  1545487.0 |
  1547158.8 |
  1548830.5 |
  1550502.2 |
  1552173.9 |
  1553845.6 |########################################
  1555517.4 |
  1557189.1 |
  1558860.8 |
  1560532.5 |
  1562204.2 |
  1563876.0 |
  1565547.7 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1489319.2-1515013.3 ns)
  1489319.2 |########################################
  1490603.9 |
  1491888.6 |
  1493173.3 |########################################
  1494458.0 |
  1495742.7 |
  1497027.4 |
  1498312.1 |
  1499596.8 |########################################
  1500881.5 |
  1502166.2 |
  1503451.0 |
  1504735.7 |
  1506020.4 |########################################
  1507305.1 |
  1508589.8 |########################################
  1509874.5 |
  1511159.2 |
  1512443.9 |
  1513728.6 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 2717.9-2807.9 ns)
   2717.9 |########################################
   2722.4 |
   2726.9 |
   2731.4 |########################################
   2735.9 |
   2740.4 |
   2744.9 |
   2749.4 |
   2753.9 |
   2758.4 |########################################
   2762.9 |
   2767.4 |
   2771.9 |
   2776.4 |
   2780.9 |
   2785.4 |
   2789.9 |########################################
   2794.4 |
   2798.9 |
   2803.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=301.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=4411.8% of algo (FFI overhead may distort results)
