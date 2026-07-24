# abi_zig_entry (leaf)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_leaf_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_leaf_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_leaf_zig_null dominates: 27973% faster than the next best (abi_zig_entry_leaf_zig_per_w_set)

abi_zig_entry_leaf_zig_null (5.12 us) leads abi_zig_entry_leaf_zig_per_w_set (1.44 ms) by 27973%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_leaf_zig_null beats baseline by 100% (significant)

abi_zig_entry_leaf_zig_null is -1.43 ms (100%) faster than baseline abi_zig_entry_leaf_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_leaf_zig_tail_runtime_w is an outlier: 652.1x slower than the field

abi_zig_entry_leaf_zig_tail_runtime_w (3.34 ms) is 652.1x the fastest (5.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_leaf_zig_null} vs {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} (27973% apart)

The field splits into a fast tier {abi_zig_entry_leaf_zig_null} and a slow tier {abi_zig_entry_leaf_zig_per_w_set, abi_zig_entry_leaf_zig_dispatch, abi_zig_entry_leaf_zig_anchor, abi_zig_entry_leaf_zig_runtime_w, abi_zig_entry_leaf_zig_tail_dispatch, abi_zig_entry_leaf_zig_tail_runtime_w} with a 27973% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 652.1x the fastest

Fastest abi_zig_entry_leaf_zig_null (5.12 us) to slowest abi_zig_entry_leaf_zig_tail_runtime_w (3.34 ms): 652.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_leaf_zig_null** at 5115.6 ns median (-99.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 652.07x (fastest 5115.6 ns, slowest 3335706.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1442213ns | 1439221ns | 1435189ns | 1438373ns | 1451485ns | -1.00% |
| abi_zig_entry_leaf_zig_dispatch | 1439690ns | 1438663ns | 1437795ns | 1438444ns | 1442505ns | -1.17% |
| abi_zig_entry_leaf_zig_null | 7446ns | 7434ns | 7357ns | 7432ns | 7510ns | -99.49% |
| abi_zig_entry_leaf_zig_per_w_set | 1443091ns | 1438684ns | 1435262ns | 1437660ns | 1455153ns | -0.94% |
| abi_zig_entry_leaf_zig_runtime_w | 1456784ns | 1441870ns | 1439529ns | 1441540ns | 1488278ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3092568ns | 3086382ns | 3078254ns | 3086125ns | 3109389ns | +112.29% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3338446ns | 3338613ns | 3334784ns | 3337364ns | 3341901ns | +129.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1439611ns | 1432668ns | 1448940ns | -1.00% | 0.000 |
| abi_zig_entry_leaf_zig_dispatch | 1437128ns | 1435288ns | 1439843ns | -1.17% | 0.000 |
| abi_zig_entry_leaf_zig_null | 5112ns | 5074ns | 5147ns | -99.65% | 0.000 |
| abi_zig_entry_leaf_zig_per_w_set | 1440321ns | 1432629ns | 1452128ns | -0.95% | 0.000 |
| abi_zig_entry_leaf_zig_runtime_w | 1454097ns | 1437014ns | 1485342ns | base | 0.000 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3089725ns | 3075562ns | 3106505ns | +112.48% | 0.000 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3335541ns | 3331908ns | 3339007ns | +129.39% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 175869.2 | 1438652.3 | 1439610.9 | n/a |
| abi_zig_entry_leaf_zig_dispatch | 171915.0 | 1437177.6 | 1437128.3 | n/a |
| abi_zig_entry_leaf_zig_null | 155384.2 | 5151.9 | 5112.5 | n/a |
| abi_zig_entry_leaf_zig_per_w_set | 183415.6 | 1440136.8 | 1440320.6 | n/a |
| abi_zig_entry_leaf_zig_runtime_w | 192898.7 | 1453828.8 | 1454096.5 | n/a |
| abi_zig_entry_leaf_zig_tail_dispatch | 193001.7 | 3089295.3 | 3089724.7 | n/a |
| abi_zig_entry_leaf_zig_tail_runtime_w | 191158.6 | 3279849.8 | 3335541.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_zig_entry_leaf_zig_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 0.000 | 0.4% |
| abi_zig_entry_leaf_zig_dispatch | 0.000 | 0.4% |
| abi_zig_entry_leaf_zig_null | 0.000 | 99.2% |
| abi_zig_entry_leaf_zig_per_w_set | 0.000 | 0.4% |
| abi_zig_entry_leaf_zig_runtime_w | 0.000 | 0.4% |
| abi_zig_entry_leaf_zig_tail_dispatch | 0.000 | 0.2% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 1442213ns | 1442213ns | -1.00% |
| abi_zig_entry_leaf_zig_dispatch | 1439690ns | 1439690ns | -1.17% |
| abi_zig_entry_leaf_zig_null | 7446ns | 7446ns | -99.49% |
| abi_zig_entry_leaf_zig_per_w_set | 1443091ns | 1443091ns | -0.94% |
| abi_zig_entry_leaf_zig_runtime_w | 1456784ns | 1456784ns | base |
| abi_zig_entry_leaf_zig_tail_dispatch | 3092568ns | 3092568ns | +112.29% |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3338446ns | 3338446ns | +129.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_leaf_zig_runtime_w | 1439286ns | base | --- | [1437661, 1485342] | --- | --- | --- | --- |
| abi_zig_entry_leaf_zig_anchor | 1436502ns | no significant difference | [-38576, +410]ns | [1433390, 1448940] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_dispatch | 1436154ns | no significant difference | [-48705, +190]ns | [1435388, 1439843] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_leaf_zig_null | 5116ns | -1434140.7ns (-99.6%) | [-1480268, -1432544]ns | [5075, 5147] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_per_w_set | 1436084ns | -4413.6ns (-0.3%) | [-35647, -1267]ns | [1432750, 1452128] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_dispatch | 3083526ns | +1643777.2ns (+114.2%) | [+1598944, +1664163]ns | [3079143, 3106505] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_leaf_zig_tail_runtime_w | 3335707ns | +1896692.0ns (+131.8%) | [+1848754, +1898888]ns | [3331910, 3339007] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_leaf_zig_runtime_w | abi_zig_entry_leaf_zig_anchor | abi_zig_entry_leaf_zig_dispatch | abi_zig_entry_leaf_zig_null | abi_zig_entry_leaf_zig_per_w_set | abi_zig_entry_leaf_zig_tail_dispatch | abi_zig_entry_leaf_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1439055ns | -0.3% | -0.2% | -99.6% | -0.1% | +114.2% | +131.9% |
| 2 | 1437014ns | +0.1% | -0.1% | -99.6% | -0.3% | +114.0% | +131.9% |
| 3 | 1529686ns | -4.6% | -6.1% | -99.7% | -4.1% | +101.9% | +117.9% |
| 4 | 1440998ns | -0.4% | -0.0% | -99.6% | -0.6% | +116.8% | +131.7% |
| 5 | 1439518ns | -0.0% | -0.2% | -99.6% | -0.3% | +114.2% | +131.9% |
| 6 | 1438308ns | -0.4% | +0.0% | -99.6% | -0.1% | +114.4% | +131.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_leaf_zig_anchor | -0.220 | moderate- |
| abi_zig_entry_leaf_zig_dispatch | -0.119 | ok |
| abi_zig_entry_leaf_zig_null | -0.045 | ok |
| abi_zig_entry_leaf_zig_per_w_set | -0.373 | moderate- |
| abi_zig_entry_leaf_zig_runtime_w | -0.234 | moderate- |
| abi_zig_entry_leaf_zig_tail_dispatch | -0.062 | ok |
| abi_zig_entry_leaf_zig_tail_runtime_w | -0.133 | ok |

**Consistency summary:**

- **abi_zig_entry_leaf_zig_anchor**: won 4/6, lost 0/6
- **abi_zig_entry_leaf_zig_dispatch**: won 4/6, lost 0/6
- **abi_zig_entry_leaf_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_leaf_zig_per_w_set**: won 5/6, lost 0/6
- **abi_zig_entry_leaf_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_leaf_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_leaf_zig_anchor | 4557258.4ns | 1439610.9ns | 316.6% | HIGH |
| abi_zig_entry_leaf_zig_dispatch | 4549370.6ns | 1437128.3ns | 316.6% | HIGH |
| abi_zig_entry_leaf_zig_null | 312797.9ns | 5112.5ns | 6118.3% | HIGH |
| abi_zig_entry_leaf_zig_per_w_set | 4575732.8ns | 1440320.6ns | 317.7% | HIGH |
| abi_zig_entry_leaf_zig_runtime_w | 4648336.1ns | 1454096.5ns | 319.7% | HIGH |
| abi_zig_entry_leaf_zig_tail_dispatch | 9533105.9ns | 3089724.7ns | 308.5% | HIGH |
| abi_zig_entry_leaf_zig_tail_runtime_w | 10087428.7ns | 3335541.2ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_leaf_zig_anchor (n=6, range 1432667.5-1448940.4 ns)
  1432667.5 |####################
  1433481.1 |####################
  1434294.8 |####################
  1435108.4 |
  1435922.1 |
  1436735.7 |
  1437549.4 |
  1438363.0 |########################################
  1439176.7 |
  1439990.3 |
  1440803.9 |
  1441617.6 |
  1442431.2 |
  1443244.9 |
  1444058.5 |
  1444872.2 |
  1445685.8 |
  1446499.5 |
  1447313.1 |
  1448126.8 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_dispatch (n=6, range 1435288.3-1439842.9 ns)
  1435288.3 |########################################
  1435516.0 |
  1435743.8 |####################
  1435971.5 |
  1436199.2 |
  1436427.0 |####################
  1436654.7 |
  1436882.4 |
  1437110.2 |
  1437337.9 |
  1437565.6 |
  1437793.4 |
  1438021.1 |
  1438248.8 |
  1438476.6 |
  1438704.3 |####################
  1438932.0 |
  1439159.8 |
  1439387.5 |
  1439615.2 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_null (n=6, range 5074.2-5147.3 ns)
   5074.2 |########################################
   5077.9 |
   5081.5 |
   5085.2 |
   5088.8 |
   5092.5 |
   5096.1 |
   5099.8 |
   5103.4 |
   5107.1 |
   5110.8 |####################
   5114.4 |####################
   5118.1 |####################
   5121.7 |
   5125.4 |
   5129.0 |
   5132.7 |
   5136.3 |
   5140.0 |
   5143.6 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_per_w_set (n=6, range 1432629.2-1452127.9 ns)
  1432629.2 |########################################
  1433604.1 |
  1434579.1 |####################
  1435554.0 |
  1436528.9 |########################################
  1437503.9 |
  1438478.8 |
  1439453.7 |
  1440428.7 |
  1441403.6 |
  1442378.5 |
  1443353.5 |
  1444328.4 |
  1445303.4 |
  1446278.3 |
  1447253.2 |
  1448228.2 |
  1449203.1 |
  1450178.0 |
  1451153.0 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_runtime_w (n=6, range 1437014.2-1485342.2 ns)
  1437014.2 |########################################
  1439430.6 |##########################
  1441847.0 |
  1444263.4 |
  1446679.8 |
  1449096.2 |
  1451512.6 |
  1453929.0 |
  1456345.4 |
  1458761.8 |
  1461178.2 |
  1463594.6 |
  1466011.0 |
  1468427.4 |
  1470843.8 |
  1473260.2 |
  1475676.6 |
  1478093.0 |
  1480509.4 |
  1482925.8 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_dispatch (n=6, range 3075562.1-3106505.2 ns)
  3075562.1 |####################
  3077109.3 |
  3078656.4 |
  3080203.6 |
  3081750.7 |####################
  3083297.9 |########################################
  3084845.0 |
  3086392.2 |
  3087939.4 |####################
  3089486.5 |
  3091033.7 |
  3092580.8 |
  3094128.0 |
  3095675.1 |
  3097222.3 |
  3098769.5 |
  3100316.6 |
  3101863.8 |
  3103410.9 |
  3104958.1 |
  (0 below, 1 above range)

abi_zig_entry_leaf_zig_tail_runtime_w (n=6, range 3331908.3-3339007.2 ns)
  3331908.3 |########################################
  3332263.2 |
  3332618.2 |
  3332973.1 |
  3333328.1 |####################
  3333683.0 |
  3334038.0 |
  3334392.9 |
  3334747.9 |
  3335102.8 |
  3335457.8 |
  3335812.7 |
  3336167.7 |
  3336522.6 |
  3336877.6 |
  3337232.5 |
  3337587.5 |####################
  3337942.4 |
  3338297.4 |####################
  3338652.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_leaf_zig_anchor**: bridge=316.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_dispatch**: bridge=316.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_null**: bridge=6122.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_per_w_set**: bridge=316.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_runtime_w**: bridge=316.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_dispatch**: bridge=308.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_leaf_zig_tail_runtime_w**: bridge=302.3% of algo (FFI overhead may distort results)
