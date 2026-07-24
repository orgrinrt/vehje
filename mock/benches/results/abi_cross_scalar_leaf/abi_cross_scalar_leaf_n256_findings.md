# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 3.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 48117% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (3.35 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.61 ms) by 48117%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.70 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 507.7x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.70 ms) is 507.7x the fastest (3.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (48117% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 48117% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 507.7x the fastest

Fastest abi_cross_scalar_leaf_null_entry (3.35 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.70 ms): 507.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 3348.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 507.66x (fastest 3348.8 ns, slowest 1700031.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1635741ns | 1651957ns | 1530878ns | 1613774ns | 1721122ns | -3.58% |
| abi_cross_scalar_leaf_inproc_direct | 1696409ns | 1704017ns | 1586105ns | 1680754ns | 1775043ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1667765ns | 1618857ns | 1577928ns | 1607933ns | 1802431ns | -1.69% |
| abi_cross_scalar_leaf_null_entry | 5846ns | 5745ns | 5532ns | 5674ns | 6259ns | -99.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1630927ns | 1526632ns | 1716229ns | -3.61% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1691986ns | 1582476ns | 1769518ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1663266ns | 1574272ns | 1796966ns | -1.70% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 3370ns | 3190ns | 3549ns | -99.80% | 0.076 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 98714.6 | 1640237.6 | 1630927.2 | 1 |
| abi_cross_scalar_leaf_inproc_direct | 10555.8 | 1688910.3 | 1691986.1 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 10472.1 | 1675854.7 | 1663265.8 | n/a |
| abi_cross_scalar_leaf_null_entry | 32603.3 | 3409.4 | 3370.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.080 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.076 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1635741ns | 1635741ns | -3.58% |
| abi_cross_scalar_leaf_inproc_direct | 1696409ns | 1696409ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1667765ns | 1667765ns | -1.69% |
| abi_cross_scalar_leaf_null_entry | 5846ns | 5846ns | -99.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1700031ns | base | --- | [1606410, 1769518] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1646396ns | no significant difference | [-148914, +31838]ns | [1530157, 1716229] | no | 0.3281 | 0.2188 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1614654ns | no significant difference | [-121854, +27449]ns | [1578177, 1796966] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_leaf_null_entry | 3349ns | -1696637.5ns (-99.8%) | [-1766080, -1603131]ns | [3212, 3549] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1674167ns | -8.8% | -6.0% | -99.8% |
| 2 | 1582476ns | -3.1% | +0.9% | -99.8% |
| 3 | 1725896ns | -4.8% | -8.3% | -99.8% |
| 4 | 1630343ns | +5.3% | +0.1% | -99.8% |
| 5 | 1800597ns | -8.3% | +2.1% | -99.8% |
| 6 | 1738438ns | -1.3% | +1.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.376 | moderate+ |
| abi_cross_scalar_leaf_inproc_direct | -0.176 | ok |
| abi_cross_scalar_leaf_inproc_fnptr | 0.417 | moderate+ |
| abi_cross_scalar_leaf_null_entry | -0.138 | ok |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 5/6, lost 1/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4990568.3ns | 1630927.2ns | 306.0% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 5078711.4ns | 1691986.1ns | 300.2% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 5040047.6ns | 1663265.8ns | 303.0% | HIGH |
| abi_cross_scalar_leaf_null_entry | 127022.1ns | 3370.0ns | 3769.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1526632.1-1716228.7 ns)
  1526632.1 |########################################
  1536111.9 |
  1545591.8 |
  1555071.6 |
  1564551.4 |
  1574031.2 |
  1583511.1 |
  1592990.9 |
  1602470.7 |
  1611950.6 |
  1621430.4 |
  1630910.2 |
  1640390.1 |####################
  1649869.9 |####################
  1659349.7 |
  1668829.6 |
  1678309.4 |
  1687789.2 |
  1697269.0 |
  1706748.9 |####################
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1582476.2-1769517.5 ns)
  1582476.2 |########################################
  1591828.3 |
  1601180.3 |
  1610532.4 |
  1619884.5 |
  1629236.5 |########################################
  1638588.6 |
  1647940.7 |
  1657292.7 |
  1666644.8 |########################################
  1675996.9 |
  1685348.9 |
  1694701.0 |
  1704053.0 |
  1713405.1 |
  1722757.2 |########################################
  1732109.2 |########################################
  1741461.3 |
  1750813.4 |
  1760165.4 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1574272.5-1796966.4 ns)
  1574272.5 |########################################
  1585407.2 |
  1596541.9 |####################
  1607676.6 |
  1618811.3 |
  1629946.0 |####################
  1641080.7 |
  1652215.4 |
  1663350.1 |
  1674484.8 |
  1685619.5 |
  1696754.2 |
  1707888.9 |
  1719023.6 |
  1730158.3 |
  1741293.0 |
  1752427.7 |####################
  1763562.4 |
  1774697.1 |
  1785831.8 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 3189.6-3549.2 ns)
   3189.6 |########################################
   3207.6 |
   3225.6 |########################################
   3243.5 |
   3261.5 |
   3279.5 |
   3297.5 |
   3315.4 |########################################
   3333.4 |
   3351.4 |
   3369.4 |########################################
   3387.4 |
   3405.3 |
   3423.3 |
   3441.3 |
   3459.3 |
   3477.2 |
   3495.2 |########################################
   3513.2 |
   3531.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=3757.7% of algo (FFI overhead may distort results)
