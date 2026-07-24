# abi_entry_form (leaf)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_leaf_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_leaf_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_leaf_null_entry dominates: 46881% faster than the next best (abi_entry_form_leaf_scalar_anchor)

abi_entry_form_leaf_null_entry (3.14 us) leads abi_entry_form_leaf_scalar_anchor (1.48 ms) by 46881%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_leaf_null_entry beats baseline by 100% (significant)

abi_entry_form_leaf_null_entry is -1.48 ms (100%) faster than baseline abi_entry_form_leaf_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_leaf_per_w_set is an outlier: 471.3x slower than the field

abi_entry_form_leaf_per_w_set (1.48 ms) is 471.3x the fastest (3.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_leaf_null_entry} vs {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_per_w_set} (46881% apart)

The field splits into a fast tier {abi_entry_form_leaf_null_entry} and a slow tier {abi_entry_form_leaf_scalar_anchor, abi_entry_form_leaf_dispatch_table, abi_entry_form_leaf_runtime_w, abi_entry_form_leaf_per_w_set} with a 46881% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 471.3x the fastest

Fastest abi_entry_form_leaf_null_entry (3.14 us) to slowest abi_entry_form_leaf_per_w_set (1.48 ms): 471.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_leaf_null_entry** at 3144.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 471.35x (fastest 3144.6 ns, slowest 1482201.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1480374ns | 1481546ns | 1467984ns | 1480620ns | 1486199ns | -0.20% |
| abi_entry_form_leaf_null_entry | 5399ns | 5366ns | 5252ns | 5357ns | 5535ns | -99.64% |
| abi_entry_form_leaf_per_w_set | 1484560ns | 1485419ns | 1471591ns | 1483874ns | 1492073ns | +0.08% |
| abi_entry_form_leaf_runtime_w | 1483396ns | 1482210ns | 1468880ns | 1478999ns | 1497250ns | base |
| abi_entry_form_leaf_scalar_anchor | 1481000ns | 1480637ns | 1461934ns | 1478196ns | 1494739ns | -0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1477177ns | 1464735ns | 1482783ns | -0.20% | 0.000 |
| abi_entry_form_leaf_null_entry | 3140ns | 3074ns | 3191ns | -99.79% | 0.082 |
| abi_entry_form_leaf_per_w_set | 1481390ns | 1468526ns | 1488909ns | +0.09% | 0.000 |
| abi_entry_form_leaf_runtime_w | 1480122ns | 1466191ns | 1493680ns | base | 0.000 |
| abi_entry_form_leaf_scalar_anchor | 1477810ns | 1459148ns | 1491344ns | -0.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 56129.8 | 1477889.9 | 1477176.5 | n/a |
| abi_entry_form_leaf_null_entry | 27594.8 | 3165.8 | 3140.2 | n/a |
| abi_entry_form_leaf_per_w_set | 59546.2 | 1482009.4 | 1481390.4 | n/a |
| abi_entry_form_leaf_runtime_w | 58785.0 | 1480415.0 | 1480122.1 | n/a |
| abi_entry_form_leaf_scalar_anchor | 59588.2 | 1477574.4 | 1477810.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_entry_form_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_leaf_null_entry | 0.081 | 97.7% |
| abi_entry_form_leaf_per_w_set | 0.000 | 0.2% |
| abi_entry_form_leaf_runtime_w | 0.000 | 0.2% |
| abi_entry_form_leaf_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 1480374ns | 1480374ns | -0.20% |
| abi_entry_form_leaf_null_entry | 5399ns | 5399ns | -99.64% |
| abi_entry_form_leaf_per_w_set | 1484560ns | 1484560ns | +0.08% |
| abi_entry_form_leaf_runtime_w | 1483396ns | 1483396ns | base |
| abi_entry_form_leaf_scalar_anchor | 1481000ns | 1481000ns | -0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_leaf_runtime_w | 1478904ns | base | --- | [1467782, 1493680] | --- | --- | --- | --- |
| abi_entry_form_leaf_dispatch_table | 1478440ns | no significant difference | [-22642, +11345]ns | [1470306, 1482783] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_null_entry | 3145ns | -1475754.1ns (-99.8%) | [-1490535, -1464656]ns | [3085, 3191] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_leaf_per_w_set | 1482202ns | no significant difference | [-9628, +12440]ns | [1473060, 1488909] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_leaf_scalar_anchor | 1477356ns | no significant difference | [-18050, +18766]ns | [1464731, 1491344] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_leaf_runtime_w | abi_entry_form_leaf_dispatch_table | abi_entry_form_leaf_null_entry | abi_entry_form_leaf_per_w_set | abi_entry_form_leaf_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 1493693ns | -1.1% | -99.8% | -1.1% | -0.6% |
| 2 | 1469373ns | +0.7% | -99.8% | +1.0% | +0.7% |
| 3 | 1470150ns | +0.9% | -99.8% | +0.7% | +1.8% |
| 4 | 1466191ns | +0.7% | -99.8% | +0.2% | -0.5% |
| 5 | 1487658ns | -0.3% | -99.8% | -0.0% | -1.2% |
| 6 | 1493666ns | -1.9% | -99.8% | -0.2% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_leaf_dispatch_table | -0.321 | moderate- |
| abi_entry_form_leaf_null_entry | 0.125 | ok |
| abi_entry_form_leaf_per_w_set | -0.074 | ok |
| abi_entry_form_leaf_runtime_w | 0.117 | ok |
| abi_entry_form_leaf_scalar_anchor | -0.173 | ok |

**Consistency summary:**

- **abi_entry_form_leaf_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_leaf_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_leaf_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_leaf_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_leaf_dispatch_table | 4489816.5ns | 1477176.5ns | 303.9% | HIGH |
| abi_entry_form_leaf_null_entry | 120984.7ns | 3140.2ns | 3852.8% | HIGH |
| abi_entry_form_leaf_per_w_set | 4501128.5ns | 1481390.4ns | 303.8% | HIGH |
| abi_entry_form_leaf_runtime_w | 4501712.5ns | 1480122.1ns | 304.1% | HIGH |
| abi_entry_form_leaf_scalar_anchor | 4496028.3ns | 1477810.3ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_leaf_dispatch_table (n=6, range 1464735.0-1482783.1 ns)
  1464735.0 |########################################
  1465637.4 |
  1466539.8 |
  1467442.2 |
  1468344.6 |
  1469247.0 |
  1470149.4 |
  1471051.9 |
  1471954.3 |
  1472856.7 |
  1473759.1 |
  1474661.5 |
  1475563.9 |########################################
  1476466.3 |########################################
  1477368.7 |
  1478271.1 |
  1479173.5 |########################################
  1480075.9 |
  1480978.3 |
  1481880.7 |########################################
  (0 below, 1 above range)

abi_entry_form_leaf_null_entry (n=6, range 3073.7-3191.1 ns)
   3073.7 |########################################
   3079.6 |
   3085.4 |
   3091.3 |########################################
   3097.2 |
   3103.0 |
   3108.9 |
   3114.8 |
   3120.6 |
   3126.5 |
   3132.4 |
   3138.2 |########################################
   3144.1 |
   3150.0 |########################################
   3155.8 |
   3161.7 |
   3167.6 |
   3173.4 |########################################
   3179.3 |
   3185.2 |
  (0 below, 1 above range)

abi_entry_form_leaf_per_w_set (n=6, range 1468525.8-1488909.1 ns)
  1468525.8 |########################################
  1469545.0 |
  1470564.1 |
  1471583.3 |
  1472602.5 |
  1473621.6 |
  1474640.8 |
  1475660.0 |
  1476679.1 |########################################
  1477698.3 |
  1478717.5 |
  1479736.6 |########################################
  1480755.8 |
  1481775.0 |
  1482794.1 |
  1483813.3 |########################################
  1484832.5 |
  1485851.6 |
  1486870.8 |########################################
  1487890.0 |
  (0 below, 1 above range)

abi_entry_form_leaf_runtime_w (n=6, range 1466191.2-1493679.8 ns)
  1466191.2 |####################
  1467565.6 |
  1468940.1 |########################################
  1470314.5 |
  1471688.9 |
  1473063.3 |
  1474437.8 |
  1475812.2 |
  1477186.6 |
  1478561.0 |
  1479935.5 |
  1481309.9 |
  1482684.3 |
  1484058.8 |
  1485433.2 |
  1486807.6 |####################
  1488182.0 |
  1489556.5 |
  1490930.9 |
  1492305.3 |####################
  (0 below, 1 above range)

abi_entry_form_leaf_scalar_anchor (n=6, range 1459147.5-1491343.8 ns)
  1459147.5 |########################################
  1460757.3 |
  1462367.1 |
  1463976.9 |
  1465586.8 |
  1467196.6 |
  1468806.4 |########################################
  1470416.2 |
  1472026.0 |
  1473635.8 |########################################
  1475245.6 |
  1476855.4 |
  1478465.2 |########################################
  1480075.1 |
  1481684.9 |
  1483294.7 |
  1484904.5 |########################################
  1486514.3 |
  1488124.1 |
  1489733.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_leaf_dispatch_table**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_null_entry**: bridge=3852.7% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_per_w_set**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_runtime_w**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_entry_form_leaf_scalar_anchor**: bridge=304.7% of algo (FFI overhead may distort results)
