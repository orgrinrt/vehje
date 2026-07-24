# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 36440% faster than the next best (abi_entry_form_leaf_runtime_w)

abi_entry_form_leaf_null_entry (4.09 us) leads abi_entry_form_leaf_runtime_w (1.49 ms) by 36440%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.49 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_dispatch_table is an outlier: 376.3x slower than the field

abi_entry_form_leaf_dispatch_table (1.54 ms) is 376.3x the fastest (4.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_leaf_runtime_w shows warm-up / thermal drift (autocorr +0.52)

abi_entry_form_leaf_runtime_w's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table} (36440% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_per_w_set, abi_entry_form_leaf_dispatch_table} with a 36440% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 376.3x the fastest

Fastest abi_entry_form_leaf_null_entry (4.09 us) to slowest abi_entry_form_leaf_dispatch_table (1.54 ms): 376.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 4089.6 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 376.32x (fastest 4089.6 ns, slowest 1538989.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1536481ns | 1542961ns | 1493698ns | 1541183ns | 1550820ns | +0.66% |
| abi_entry_form_leaf_null_entry | 6464ns | 6447ns | 6238ns | 6422ns | 6641ns | -99.58% |
| abi_entry_form_leaf_per_w_set | 1542080ns | 1517975ns | 1470740ns | 1507634ns | 1629420ns | +1.03% |
| abi_entry_form_leaf_runtime_w | 1526343ns | 1497845ns | 1482120ns | 1492930ns | 1598573ns | base |
| abi_entry_form_leaf_scalar_anchor | 1528142ns | 1509866ns | 1477032ns | 1503684ns | 1590385ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1532604ns | 1490177ns | 1546949ns | +0.65% | 0.000 |
| abi_entry_form_leaf_null_entry | 4105ns | 3991ns | 4208ns | -99.73% | 0.001 |
| abi_entry_form_leaf_per_w_set | 1538405ns | 1467690ns | 1625357ns | +1.03% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1522717ns | 1478780ns | 1594426ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1524530ns | 1473772ns | 1586454ns | +0.12% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 78253.7 | 1536864.1 | 1532604.3 | n/a |
| abi_entry_form_leaf_null_entry | 30310.1 | 4269.4 | 4104.7 | n/a |
| abi_entry_form_leaf_per_w_set | 74546.1 | 1538145.8 | 1538404.6 | n/a |
| abi_entry_form_leaf_runtime_w | 70029.8 | 1519686.5 | 1522716.7 | n/a |
| abi_entry_form_leaf_scalar_anchor | 70689.3 | 1525128.1 | 1524530.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.3% |
| abi_entry_form_leaf_null_entry | 0.001 | 97.6% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.3% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.3% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1536481ns | 1536481ns | +0.66% |
| abi_entry_form_leaf_null_entry | 6464ns | 6464ns | -99.58% |
| abi_entry_form_leaf_per_w_set | 1542080ns | 1542080ns | +1.03% |
| abi_entry_form_leaf_runtime_w | 1526343ns | 1526343ns | base |
| abi_entry_form_leaf_scalar_anchor | 1528142ns | 1528142ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1494321ns | base | --- | [1479403, 1594426] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1538990ns | no significant difference | [-47815, +56669]ns | [1511874, 1546949] | no | 1.0000 | 0.6875 | 0 |
| abi_entry_form_leaf_null_entry | 4090ns | -1490113.0ns (-99.7%) | [-1590337, -1475386]ns | [4016, 4208] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1514206ns | no significant difference | [-40839, +86989]ns | [1475651, 1625357] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1506252ns | no significant difference | [-27077, +32154]ns | [1480884, 1586454] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1601821ns | -3.9% | -99.7% | -4.3% | -2.4% |
| 2 | 1587031ns | -2.1% | -99.7% | -0.0% | +1.4% |
| 3 | 1507628ns | +2.0% | -99.7% | +10.4% | -1.0% |
| 4 | 1478780ns | +0.8% | -99.7% | +1.2% | +2.8% |
| 5 | 1480026ns | +4.1% | -99.7% | -0.8% | +0.5% |
| 6 | 1481015ns | +3.5% | -99.7% | +0.2% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | -0.127 | ok |
| abi_entry_form_leaf_null_entry | -0.281 | moderate- |
| abi_entry_form_leaf_per_w_set | 0.260 | moderate+ |
| abi_entry_form_leaf_runtime_w | 0.524 | HIGH+ (drift/warm-up) |
| abi_entry_form_leaf_scalar_anchor | 0.195 | ok |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 2/6, lost 4/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_leaf_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4686792.6ns | 1532604.3ns | 305.8% | HIGH |
| abi_entry_form_leaf_null_entry | 125331.1ns | 4104.7ns | 3053.3% | HIGH |
| abi_entry_form_leaf_per_w_set | 4687162.3ns | 1538404.6ns | 304.7% | HIGH |
| abi_entry_form_leaf_runtime_w | 4637601.5ns | 1522716.7ns | 304.6% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4650438.7ns | 1524530.2ns | 305.0% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1490176.7-1546949.1 ns)
  1490176.7 |####################
  1493015.3 |
  1495853.9 |
  1498692.6 |
  1501531.2 |
  1504369.8 |
  1507208.4 |
  1510047.1 |
  1512885.7 |
  1515724.3 |
  1518562.9 |
  1521401.5 |
  1524240.2 |
  1527078.8 |
  1529917.4 |
  1532756.0 |####################
  1535594.7 |####################
  1538433.3 |########################################
  1541271.9 |
  1544110.5 |
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 3990.8-4208.1 ns)
   3990.8 |########################################
   4001.7 |
   4012.5 |
   4023.4 |
   4034.3 |########################################
   4045.1 |
   4056.0 |
   4066.9 |
   4077.7 |########################################
   4088.6 |########################################
   4099.5 |
   4110.3 |########################################
   4121.2 |
   4132.0 |
   4142.9 |
   4153.8 |
   4164.6 |
   4175.5 |
   4186.4 |
   4197.2 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1467689.6-1625357.3 ns)
  1467689.6 |########################################
  1475573.0 |
  1483456.4 |########################################
  1491339.8 |########################################
  1499223.1 |
  1507106.5 |
  1514989.9 |
  1522873.3 |
  1530756.7 |########################################
  1538640.1 |
  1546523.4 |
  1554406.8 |
  1562290.2 |
  1570173.6 |
  1578057.0 |
  1585940.4 |########################################
  1593823.8 |
  1601707.1 |
  1609590.5 |
  1617473.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1478780.0-1594426.2 ns)
  1478780.0 |########################################
  1484562.3 |
  1490344.6 |
  1496126.9 |
  1501909.2 |#############
  1507691.6 |
  1513473.9 |
  1519256.2 |
  1525038.5 |
  1530820.8 |
  1536603.1 |
  1542385.4 |
  1548167.7 |
  1553950.0 |
  1559732.3 |
  1565514.6 |
  1571297.0 |
  1577079.3 |
  1582861.6 |#############
  1588643.9 |
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1473771.7-1586453.8 ns)
  1473771.7 |########################################
  1479405.8 |
  1485039.9 |########################################
  1490674.0 |########################################
  1496308.1 |
  1501942.2 |
  1507576.3 |
  1513210.4 |
  1518844.5 |########################################
  1524478.6 |
  1530112.7 |
  1535746.8 |
  1541380.9 |
  1547015.0 |
  1552649.1 |
  1558283.2 |########################################
  1563917.3 |
  1569551.4 |
  1575185.5 |
  1580819.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=306.5% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=3065.3% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=305.4% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **abi_entry_form_leaf_runtime_w**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=304.4% of algo (FFI overhead may distort results)
