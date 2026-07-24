# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 65449% faster than the next best (abi_entry_form_leaf_scalar_anchor)

abi_entry_form_leaf_null_entry (2.27 us) leads abi_entry_form_leaf_scalar_anchor (1.49 ms) by 65449%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.49 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_dispatch_table is an outlier: 662.1x slower than the field

abi_entry_form_leaf_dispatch_table (1.50 ms) is 662.1x the fastest (2.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table} (65449% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table} with a 65449% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 662.1x the fastest

Fastest abi_entry_form_leaf_null_entry (2.27 us) to slowest abi_entry_form_leaf_dispatch_table (1.50 ms): 662.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 2266.4 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 662.10x (fastest 2266.4 ns, slowest 1500619.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1523284ns | 1504195ns | 1470523ns | 1496682ns | 1589569ns | -1.55% |
| abi_entry_form_leaf_null_entry | 4588ns | 4543ns | 4455ns | 4536ns | 4732ns | -99.70% |
| abi_entry_form_leaf_per_w_set | 1552729ns | 1501322ns | 1465635ns | 1495048ns | 1682798ns | +0.36% |
| abi_entry_form_leaf_runtime_w | 1547208ns | 1493553ns | 1481521ns | 1489694ns | 1666323ns | base |
| abi_entry_form_leaf_scalar_anchor | 1484491ns | 1488711ns | 1473251ns | 1483617ns | 1491423ns | -4.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1519762ns | 1467748ns | 1585583ns | -1.55% | 0.000 |
| abi_entry_form_leaf_null_entry | 2292ns | 2223ns | 2369ns | -99.85% | 0.014 |
| abi_entry_form_leaf_per_w_set | 1549018ns | 1462750ns | 1678545ns | +0.34% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1543703ns | 1478276ns | 1662311ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1481380ns | 1470225ns | 1488211ns | -4.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 71837.8 | 1518893.2 | 1519762.2 | 0 |
| abi_entry_form_leaf_null_entry | 29208.9 | 2391.1 | 2292.1 | n/a |
| abi_entry_form_leaf_per_w_set | 68704.8 | 1565280.2 | 1549018.3 | n/a |
| abi_entry_form_leaf_runtime_w | 62134.5 | 1544862.4 | 1543702.8 | n/a |
| abi_entry_form_leaf_scalar_anchor | 60014.9 | 1481474.4 | 1481380.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_leaf_null_entry | 0.014 | 98.1% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.1% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.1% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1523284ns | 1523284ns | -1.55% |
| abi_entry_form_leaf_null_entry | 4588ns | 4588ns | -99.70% |
| abi_entry_form_leaf_per_w_set | 1552729ns | 1552729ns | +0.36% |
| abi_entry_form_leaf_runtime_w | 1547208ns | 1547208ns | base |
| abi_entry_form_leaf_scalar_anchor | 1484491ns | 1484491ns | -4.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1490154ns | base | --- | [1478643, 1662311] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1500619ns | no significant difference | [-164427, +82393]ns | [1473084, 1585583] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_leaf_null_entry | 2266ns | -1487907.7ns (-99.8%) | [-1660032, -1476293]ns | [2241, 2369] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1497595ns | no significant difference | [-169760, +190260]ns | [1470915, 1678545] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1485624ns | -11955.8ns (-0.8%) | [-174342, -671]ns | [1470305, 1488211] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1478276ns | +0.1% | -99.8% | +0.7% | -0.5% |
| 2 | 1479010ns | -0.8% | -99.8% | +16.0% | +0.5% |
| 3 | 1482750ns | +9.5% | -99.8% | -1.3% | -0.8% |
| 4 | 1796012ns | -17.7% | -99.9% | -16.1% | -17.3% |
| 5 | 1528610ns | +1.2% | -99.9% | -3.2% | -2.5% |
| 6 | 1497558ns | +1.6% | -99.9% | +9.6% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | -0.499 | moderate- |
| abi_entry_form_leaf_null_entry | -0.061 | ok |
| abi_entry_form_leaf_per_w_set | -0.446 | moderate- |
| abi_entry_form_leaf_runtime_w | -0.132 | ok |
| abi_entry_form_leaf_scalar_anchor | -0.162 | ok |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 2/6, lost 4/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_leaf_scalar_anchor**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4621344.6ns | 1519762.2ns | 304.1% | HIGH |
| abi_entry_form_leaf_null_entry | 118059.6ns | 2292.1ns | 5150.8% | HIGH |
| abi_entry_form_leaf_per_w_set | 4759779.1ns | 1549018.3ns | 307.3% | HIGH |
| abi_entry_form_leaf_runtime_w | 4689026.3ns | 1543702.8ns | 303.8% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4508250.9ns | 1481380.1ns | 304.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1467747.9-1585583.4 ns)
  1467747.9 |########################################
  1473639.7 |########################################
  1479531.4 |########################################
  1485423.2 |
  1491315.0 |
  1497206.8 |
  1503098.5 |
  1508990.3 |
  1514882.1 |
  1520773.9 |########################################
  1526665.6 |
  1532557.4 |
  1538449.2 |
  1544340.9 |########################################
  1550232.7 |
  1556124.5 |
  1562016.3 |
  1567908.0 |
  1573799.8 |
  1579691.6 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 2223.3-2369.2 ns)
   2223.3 |########################################
   2230.6 |
   2237.9 |
   2245.2 |
   2252.5 |########################################
   2259.8 |########################################
   2267.1 |########################################
   2274.3 |
   2281.6 |
   2288.9 |
   2296.2 |########################################
   2303.5 |
   2310.8 |
   2318.1 |
   2325.4 |
   2332.7 |
   2340.0 |
   2347.3 |
   2354.6 |
   2361.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1462750.4-1678544.6 ns)
  1462750.4 |########################################
  1473540.1 |########################################
  1484329.8 |########################################
  1495119.5 |
  1505909.2 |########################################
  1516698.9 |
  1527488.7 |
  1538278.4 |
  1549068.1 |
  1559857.8 |
  1570647.5 |
  1581437.2 |
  1592226.9 |
  1603016.6 |
  1613806.3 |
  1624596.1 |
  1635385.8 |########################################
  1646175.5 |
  1656965.2 |
  1667754.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1478276.2-1662311.0 ns)
  1478276.2 |########################################
  1487477.9 |
  1496679.7 |#############
  1505881.4 |
  1515083.2 |
  1524284.9 |#############
  1533486.7 |
  1542688.4 |
  1551890.1 |
  1561091.9 |
  1570293.6 |
  1579495.4 |
  1588697.1 |
  1597898.9 |
  1607100.6 |
  1616302.3 |
  1625504.1 |
  1634705.8 |
  1643907.6 |
  1653109.3 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1470224.6-1488210.9 ns)
  1470224.6 |##########################
  1471123.9 |
  1472023.2 |
  1472922.5 |
  1473821.9 |
  1474721.2 |
  1475620.5 |
  1476519.8 |
  1477419.1 |
  1478318.4 |
  1479217.7 |
  1480117.0 |
  1481016.4 |
  1481915.7 |
  1482815.0 |
  1483714.3 |
  1484613.6 |
  1485512.9 |########################################
  1486412.2 |
  1487311.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=5156.1% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=308.8% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=304.1% of algo (FFI overhead may distort results)
