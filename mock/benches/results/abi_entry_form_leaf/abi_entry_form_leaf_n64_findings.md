# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_leaf_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_leaf_runtime_w has the worst median (1.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_leaf_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_leaf_null_entry dominates: 59398% faster than the next best (abi_entry_form_leaf_per_w_set)

abi_entry_form_leaf_null_entry (2.49 us) leads abi_entry_form_leaf_per_w_set (1.48 ms) by 59398%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.49 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_runtime_w is an outlier: 601.2x slower than the field

abi_entry_form_leaf_runtime_w (1.50 ms) is 601.2x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_runtime_w} (59398% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_runtime_w} with a 59398% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 601.2x the fastest

Fastest abi_entry_form_leaf_null_entry (2.49 us) to slowest abi_entry_form_leaf_runtime_w (1.50 ms): 601.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 2486.9 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 601.23x (fastest 2486.9 ns, slowest 1495170.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1513645ns | 1489525ns | 1475304ns | 1489009ns | 1569768ns | -0.41% |
| abi_entry_form_leaf_null_entry | 4794ns | 4828ns | 4595ns | 4806ns | 4875ns | -99.68% |
| abi_entry_form_leaf_per_w_set | 1492396ns | 1482787ns | 1470720ns | 1480223ns | 1521494ns | -1.80% |
| abi_entry_form_leaf_runtime_w | 1519809ns | 1498694ns | 1488762ns | 1495894ns | 1571204ns | base |
| abi_entry_form_leaf_scalar_anchor | 1503370ns | 1487133ns | 1464449ns | 1483796ns | 1552192ns | -1.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1510201ns | 1471925ns | 1566018ns | -0.40% | 0.000 |
| abi_entry_form_leaf_null_entry | 2474ns | 2414ns | 2511ns | -99.84% | 0.026 |
| abi_entry_form_leaf_per_w_set | 1489103ns | 1467503ns | 1517946ns | -1.79% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1516300ns | 1485571ns | 1567425ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1500075ns | 1461321ns | 1548529ns | -1.07% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 62501.5 | 1503746.1 | 1510201.4 | n/a |
| abi_entry_form_leaf_null_entry | 27836.7 | 2721.2 | 2474.5 | n/a |
| abi_entry_form_leaf_per_w_set | 63913.7 | 1489454.1 | 1489103.3 | 0 |
| abi_entry_form_leaf_runtime_w | 63858.5 | 1509954.4 | 1516300.5 | n/a |
| abi_entry_form_leaf_scalar_anchor | 61046.5 | 1499742.0 | 1500074.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_leaf_null_entry | 0.026 | 97.1% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.2% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.2% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1513645ns | 1513645ns | -0.41% |
| abi_entry_form_leaf_null_entry | 4794ns | 4794ns | -99.68% |
| abi_entry_form_leaf_per_w_set | 1492396ns | 1492396ns | -1.80% |
| abi_entry_form_leaf_runtime_w | 1519809ns | 1519809ns | base |
| abi_entry_form_leaf_scalar_anchor | 1503370ns | 1503370ns | -1.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1495171ns | base | --- | [1486306, 1567425] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1486163ns | no significant difference | [-34978, +25688]ns | [1478423, 1566018] | no | 0.2188 | 0.2188 | 0 |
| abi_entry_form_leaf_null_entry | 2487ns | -1492705.9ns (-99.8%) | [-1564938, -1483834]ns | [2426, 2511] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1479622ns | -18374.2ns (-1.2%) | [-55499, -7719]ns | [1469741, 1517946] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1483940ns | no significant difference | [-35816, +3970]ns | [1467756, 1548529] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1598305ns | -3.4% | -99.8% | -5.4% | +0.5% |
| 2 | 1536545ns | +3.4% | -99.8% | -0.8% | -3.0% |
| 3 | 1487040ns | -1.0% | -99.8% | -0.2% | -0.6% |
| 4 | 1499682ns | -0.8% | -99.8% | -1.6% | -1.7% |
| 5 | 1485571ns | -0.0% | -99.8% | -0.9% | -1.6% |
| 6 | 1490660ns | -0.4% | -99.8% | -1.6% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.163 | ok |
| abi_entry_form_leaf_null_entry | -0.049 | ok |
| abi_entry_form_leaf_per_w_set | 0.478 | moderate+ |
| abi_entry_form_leaf_runtime_w | 0.289 | moderate+ |
| abi_entry_form_leaf_scalar_anchor | 0.081 | ok |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 4/6, lost 1/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 6/6, lost 0/6
- **abi_entry_form_leaf_scalar_anchor**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4588235.3ns | 1510201.4ns | 303.8% | HIGH |
| abi_entry_form_leaf_null_entry | 113031.7ns | 2474.5ns | 4567.9% | HIGH |
| abi_entry_form_leaf_per_w_set | 4533412.7ns | 1489103.3ns | 304.4% | HIGH |
| abi_entry_form_leaf_runtime_w | 4598152.7ns | 1516300.5ns | 303.2% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4563057.6ns | 1500074.7ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1471925.0-1566017.5 ns)
  1471925.0 |####################
  1476629.6 |
  1481334.2 |########################################
  1486038.9 |####################
  1490743.5 |
  1495448.1 |
  1500152.8 |
  1504857.4 |
  1509562.0 |
  1514266.6 |
  1518971.2 |
  1523675.9 |
  1528380.5 |
  1533085.1 |
  1537789.8 |
  1542494.4 |####################
  1547199.0 |
  1551903.6 |
  1556608.2 |
  1561312.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 2413.8-2510.6 ns)
   2413.8 |########################################
   2418.6 |
   2423.5 |
   2428.3 |
   2433.2 |
   2438.0 |########################################
   2442.8 |
   2447.7 |
   2452.5 |
   2457.4 |
   2462.2 |
   2467.0 |
   2471.9 |########################################
   2476.7 |
   2481.6 |
   2486.4 |
   2491.2 |
   2496.1 |########################################
   2500.9 |########################################
   2505.8 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1467503.3-1517946.0 ns)
  1467503.3 |########################################
  1470025.4 |########################################
  1472547.6 |
  1475069.7 |########################################
  1477591.9 |
  1480114.0 |
  1482636.1 |########################################
  1485158.3 |
  1487680.4 |
  1490202.5 |
  1492724.7 |
  1495246.8 |
  1497768.9 |
  1500291.1 |
  1502813.2 |
  1505335.4 |
  1507857.5 |
  1510379.6 |########################################
  1512901.8 |
  1515423.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1485571.2-1567424.8 ns)
  1485571.2 |########################################
  1489663.9 |####################
  1493756.6 |
  1497849.2 |####################
  1501941.9 |
  1506034.6 |
  1510127.3 |
  1514220.0 |
  1518312.6 |
  1522405.3 |
  1526498.0 |
  1530590.7 |
  1534683.4 |####################
  1538776.0 |
  1542868.7 |
  1546961.4 |
  1551054.1 |
  1555146.8 |
  1559239.4 |
  1563332.1 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1461321.2-1548528.5 ns)
  1461321.2 |####################
  1465681.6 |
  1470041.9 |####################
  1474402.3 |####################
  1478762.7 |
  1483123.0 |
  1487483.4 |########################################
  1491843.8 |
  1496204.1 |
  1500564.5 |
  1504924.9 |
  1509285.2 |
  1513645.6 |
  1518006.0 |
  1522366.3 |
  1526726.7 |
  1531087.1 |
  1535447.4 |
  1539807.8 |
  1544168.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=4549.7% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=303.6% of algo (FFI overhead may distort results)
