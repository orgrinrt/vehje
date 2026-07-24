# abi_cross_scalar (leaf)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_leaf_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_leaf_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_leaf_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_leaf_inproc_direct has the worst median (1.51 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_leaf_null_entry at 3.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_leaf_null_entry dominates: 47393% faster than the next best (abi_cross_scalar_leaf_inproc_fnptr)

abi_cross_scalar_leaf_null_entry (3.11 us) leads abi_cross_scalar_leaf_inproc_fnptr (1.48 ms) by 47393%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_leaf_null_entry beats baseline by 100% (significant)

abi_cross_scalar_leaf_null_entry is -1.51 ms (100%) faster than baseline abi_cross_scalar_leaf_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_leaf_inproc_direct is an outlier: 486.6x slower than the field

abi_cross_scalar_leaf_inproc_direct (1.51 ms) is 486.6x the fastest (3.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_leaf_null_entry} vs {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} (47393% apart)

The field splits into a fast tier {abi_cross_scalar_leaf_null_entry} and a slow tier {abi_cross_scalar_leaf_inproc_fnptr, abi_cross_scalar_leaf_ffi_batched_scalar, abi_cross_scalar_leaf_inproc_direct} with a 47393% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 486.6x the fastest

Fastest abi_cross_scalar_leaf_null_entry (3.11 us) to slowest abi_cross_scalar_leaf_inproc_direct (1.51 ms): 486.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_leaf_null_entry** at 3107.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 486.60x (fastest 3107.3 ns, slowest 1512015.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1519692ns | 1486746ns | 1472331ns | 1485253ns | 1595032ns | -2.18% |
| abi_cross_scalar_leaf_inproc_direct | 1553593ns | 1515302ns | 1492889ns | 1512306ns | 1645876ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1512058ns | 1478646ns | 1465203ns | 1475533ns | 1590273ns | -2.67% |
| abi_cross_scalar_leaf_null_entry | 5425ns | 5399ns | 5278ns | 5367ns | 5586ns | -99.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1516036ns | 1469150ns | 1590525ns | -2.19% | 0.000 |
| abi_cross_scalar_leaf_inproc_direct | 1550045ns | 1489925ns | 1641402ns | base | 0.000 |
| abi_cross_scalar_leaf_inproc_fnptr | 1508755ns | 1462183ns | 1586320ns | -2.66% | 0.000 |
| abi_cross_scalar_leaf_null_entry | 3115ns | 3008ns | 3205ns | -99.80% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 68757.2 | 1515729.9 | 1516036.2 | 2 |
| abi_cross_scalar_leaf_inproc_direct | 8919.6 | 1581168.9 | 1550045.0 | n/a |
| abi_cross_scalar_leaf_inproc_fnptr | 8874.7 | 1497838.7 | 1508755.3 | n/a |
| abi_cross_scalar_leaf_null_entry | 30029.9 | 3178.7 | 3114.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_leaf_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_leaf_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_leaf_null_entry | 0.003 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 1519692ns | 1519692ns | -2.18% |
| abi_cross_scalar_leaf_inproc_direct | 1553593ns | 1553593ns | base |
| abi_cross_scalar_leaf_inproc_fnptr | 1512058ns | 1512058ns | -2.67% |
| abi_cross_scalar_leaf_null_entry | 5425ns | 5425ns | -99.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_leaf_inproc_direct | 1512015ns | base | --- | [1496718, 1641402] | --- | --- | --- | --- |
| abi_cross_scalar_leaf_ffi_batched_scalar | 1483376ns | no significant difference | [-88843, +15329]ns | [1474208, 1590525] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_leaf_inproc_fnptr | 1475761ns | no significant difference | [-111348, +18217]ns | [1464185, 1586320] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_leaf_null_entry | 3107ns | -1508877.5ns (-99.8%) | [-1638227, -1493686]ns | [3031, 3205] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_leaf_inproc_direct | abi_cross_scalar_leaf_ffi_batched_scalar | abi_cross_scalar_leaf_inproc_fnptr | abi_cross_scalar_leaf_null_entry |
|---|---|---|---|---|
| 1 | 1503920ns | -2.3% | -2.5% | -99.8% |
| 2 | 1520111ns | -2.2% | -3.8% | -99.8% |
| 3 | 1690872ns | -8.5% | -9.7% | -99.8% |
| 4 | 1591932ns | +2.6% | +3.4% | -99.8% |
| 5 | 1503510ns | -1.6% | -1.6% | -99.8% |
| 6 | 1489925ns | -0.7% | -1.2% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 0.059 | ok |
| abi_cross_scalar_leaf_inproc_direct | 0.129 | ok |
| abi_cross_scalar_leaf_inproc_fnptr | 0.025 | ok |
| abi_cross_scalar_leaf_null_entry | 0.201 | moderate+ |

**Consistency summary:**

- **abi_cross_scalar_leaf_ffi_batched_scalar**: won 5/6, lost 1/6
- **abi_cross_scalar_leaf_inproc_fnptr**: won 5/6, lost 1/6
- **abi_cross_scalar_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_leaf_ffi_batched_scalar | 4620396.8ns | 1516036.2ns | 304.8% | HIGH |
| abi_cross_scalar_leaf_inproc_direct | 4689416.6ns | 1550045.0ns | 302.5% | HIGH |
| abi_cross_scalar_leaf_inproc_fnptr | 4507206.6ns | 1508755.3ns | 298.7% | HIGH |
| abi_cross_scalar_leaf_null_entry | 122513.2ns | 3114.6ns | 3933.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_leaf_ffi_batched_scalar (n=6, range 1469150.0-1590525.2 ns)
  1469150.0 |####################
  1475218.8 |########################################
  1481287.5 |####################
  1487356.3 |
  1493425.1 |
  1499493.8 |
  1505562.6 |
  1511631.3 |
  1517700.1 |
  1523768.9 |
  1529837.6 |
  1535906.4 |
  1541975.1 |####################
  1548043.9 |
  1554112.7 |
  1560181.4 |
  1566250.2 |
  1572319.0 |
  1578387.7 |
  1584456.5 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_direct (n=6, range 1489924.6-1641402.1 ns)
  1489924.6 |####################
  1497498.5 |########################################
  1505072.4 |
  1512646.2 |####################
  1520220.1 |
  1527794.0 |
  1535367.9 |
  1542941.7 |
  1550515.6 |
  1558089.5 |
  1565663.4 |
  1573237.2 |
  1580811.1 |
  1588385.0 |####################
  1595958.9 |
  1603532.7 |
  1611106.6 |
  1618680.5 |
  1626254.4 |
  1633828.2 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_inproc_fnptr (n=6, range 1462182.9-1586320.2 ns)
  1462182.9 |########################################
  1468389.8 |####################
  1474596.6 |####################
  1480803.5 |
  1487010.4 |
  1493217.2 |
  1499424.1 |
  1505631.0 |
  1511837.8 |
  1518044.7 |
  1524251.5 |####################
  1530458.4 |
  1536665.3 |
  1542872.1 |
  1549079.0 |
  1555285.9 |
  1561492.7 |
  1567699.6 |
  1573906.5 |
  1580113.3 |
  (0 below, 1 above range)

abi_cross_scalar_leaf_null_entry (n=6, range 3007.5-3205.4 ns)
   3007.5 |########################################
   3017.4 |
   3027.3 |
   3037.2 |
   3047.1 |########################################
   3057.0 |########################################
   3066.9 |
   3076.8 |
   3086.7 |
   3096.6 |
   3106.4 |
   3116.3 |
   3126.2 |
   3136.1 |
   3146.0 |
   3155.9 |########################################
   3165.8 |
   3175.7 |
   3185.6 |########################################
   3195.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_leaf_ffi_batched_scalar**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_direct**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_inproc_fnptr**: bridge=300.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_leaf_null_entry**: bridge=3934.6% of algo (FFI overhead may distort results)
