# Match lowering: if-chain vs jump-table vs decision-tree, K=8 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u8**

## Highlights

Baseline for all deltas below: **ml_jumptable_u8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u8 is an outlier: 3.2x slower than the field

ml_tree_u8 (3.32 ms) is 3.2x the fastest (1.02 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ml_jumptable_u8, ml_ifchain_u8) are a dead heat (<1%)

ml_jumptable_u8 (1.02 ms) and ml_ifchain_u8 (1.02 ms) differ by 0.16%, inside the noise, even though the wider field spreads 225.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ml_jumptable_u8)

The baseline ml_jumptable_u8 is the fastest (1.02 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest ml_jumptable_u8 (1.02 ms) to slowest ml_tree_u8 (3.32 ms): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ml_jumptable_u8) is the fastest** at 1021627.1 ns median
- 1 variant significantly slower than baseline
- Spread: 3.25x (fastest 1021627.1 ns, slowest 3320264.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u8 | 1026885ns | 1025638ns | 1000953ns | 1023818ns | 1044451ns | +0.26% |
| ml_jumptable_u8 | 1024264ns | 1024135ns | 1009462ns | 1020299ns | 1037613ns | base |
| ml_tree_u8 | 3305260ns | 3323479ns | 3238449ns | 3300579ns | 3345686ns | +222.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u8 | 1024267ns | 997682ns | 1041796ns | +0.25% | 0.016 |
| ml_jumptable_u8 | 1021717ns | 1007098ns | 1035069ns | base | 0.016 |
| ml_tree_u8 | 3302410ns | 3235967ns | 3342901ns | +223.22% | 0.005 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u8 | 0.016 | 97.5% |
| ml_jumptable_u8 | 0.016 | 97.7% |
| ml_tree_u8 | 0.005 | 30.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u8 | 1026885ns | 1026885ns | +0.26% |
| ml_jumptable_u8 | 1024264ns | 1024264ns | base |
| ml_tree_u8 | 3305260ns | 3305260ns | +222.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u8 | 1021627ns | base | --- | [1008453, 1035069] | --- | --- | --- | --- |
| ml_ifchain_u8 | 1023267ns | no significant difference | [-10238, +13905]ns | [1007738, 1041796] | no | 0.6875 | 0.6875 | 0 |
| ml_tree_u8 | 3320265ns | +2303599.4ns (+225.5%) | [+2222436, +2316044]ns | [3244064, 3342901] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u8 | ml_ifchain_u8 | ml_tree_u8 |
|---|---|---|---|
| 1 | 1017111ns | +0.4% | +219.7% |
| 2 | 1031878ns | +1.0% | +224.8% |
| 3 | 1007098ns | +1.8% | +229.4% |
| 4 | 1026143ns | -0.8% | +215.4% |
| 5 | 1038260ns | +0.3% | +221.2% |
| 6 | 1009809ns | -1.2% | +229.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u8 | -0.449 | moderate- |
| ml_jumptable_u8 | -0.496 | moderate- |
| ml_tree_u8 | -0.383 | moderate- |

**Consistency summary:**

- **ml_ifchain_u8**: won 2/6, lost 4/6
- **ml_tree_u8**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u8 | 25.8ns | 1024267.2ns | 0.0% |  |
| ml_jumptable_u8 | 31.1ns | 1021716.5ns | 0.0% |  |
| ml_tree_u8 | 72.0ns | 3302409.7ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u8 (n=6, range 997682.1-1041796.2 ns)
  997682.1 |########################################
  999887.8 |
  1002093.5 |
  1004299.2 |
  1006504.9 |
  1008710.6 |
  1010916.3 |
  1013122.1 |
  1015327.8 |
  1017533.5 |########################################
  1019739.2 |########################################
  1021944.9 |
  1024150.6 |########################################
  1026356.3 |
  1028562.0 |
  1030767.7 |
  1032973.4 |
  1035179.1 |
  1037384.8 |
  1039590.5 |########################################
  (0 below, 1 above range)

ml_jumptable_u8 (n=6, range 1007097.5-1035069.2 ns)
  1007097.5 |########################################
  1008496.1 |########################################
  1009894.7 |
  1011293.2 |
  1012691.8 |
  1014090.4 |
  1015489.0 |
  1016887.6 |########################################
  1018286.2 |
  1019684.7 |
  1021083.3 |
  1022481.9 |
  1023880.5 |
  1025279.1 |########################################
  1026677.7 |
  1028076.2 |
  1029474.8 |
  1030873.4 |########################################
  1032272.0 |
  1033670.6 |
  (0 below, 1 above range)

ml_tree_u8 (n=6, range 3235966.7-3342900.9 ns)
  3235966.7 |########################################
  3241313.4 |
  3246660.1 |
  3252006.8 |########################################
  3257353.5 |
  3262700.2 |
  3268046.9 |
  3273393.7 |
  3278740.4 |
  3284087.1 |
  3289433.8 |
  3294780.5 |
  3300127.2 |
  3305473.9 |
  3310820.6 |
  3316167.3 |########################################
  3321514.0 |########################################
  3326860.7 |
  3332207.4 |########################################
  3337554.1 |
  (0 below, 1 above range)

```
