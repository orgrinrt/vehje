# Match lowering: if-chain vs jump-table vs decision-tree, K=2 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u2**

## Highlights

Baseline for all deltas below: **ml_jumptable_u2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: ml_ifchain_u2** at 1006614.4 ns median (-1.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.32x (fastest 1006614.4 ns, slowest 1324668.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u2 | 1011529ns | 1009839ns | 1003805ns | 1009186ns | 1018906ns | -1.05% |
| ml_jumptable_u2 | 1022213ns | 1023682ns | 1004262ns | 1019967ns | 1034558ns | base |
| ml_tree_u2 | 1329790ns | 1328106ns | 1323052ns | 1326722ns | 1337760ns | +30.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u2 | 1008285ns | 1000411ns | 1015742ns | -1.09% | 0.016 |
| ml_jumptable_u2 | 1019348ns | 1000866ns | 1031931ns | base | 0.016 |
| ml_tree_u2 | 1326454ns | 1319666ns | 1334784ns | +30.13% | 0.012 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u2; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u2 | 0.016 | 99.4% |
| ml_jumptable_u2 | 0.016 | 98.0% |
| ml_tree_u2 | 0.012 | 75.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u2 | 1011529ns | 1011529ns | -1.05% |
| ml_jumptable_u2 | 1022213ns | 1022213ns | base |
| ml_tree_u2 | 1329790ns | 1329790ns | +30.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u2 | 1021074ns | base | --- | [1005040, 1031931] | --- | --- | --- | --- |
| ml_ifchain_u2 | 1006614ns | no significant difference | [-21937, +4130]ns | [1002499, 1015742] | no | 0.2188 | 0.2188 | 0 |
| ml_tree_u2 | 1324669ns | +308672.0ns (+30.2%) | [+292738, +319906]ns | [1319908, 1334784] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u2 | ml_ifchain_u2 | ml_tree_u2 |
|---|---|---|---|
| 1 | 1033772ns | -2.0% | +27.8% |
| 2 | 1027014ns | -2.1% | +29.9% |
| 3 | 1015134ns | -1.0% | +31.6% |
| 4 | 1009214ns | -0.9% | +30.8% |
| 5 | 1030090ns | -2.1% | +28.9% |
| 6 | 1000866ns | +1.7% | +31.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u2 | 0.115 | ok |
| ml_jumptable_u2 | -0.221 | moderate- |
| ml_tree_u2 | -0.215 | moderate- |

**Consistency summary:**

- **ml_ifchain_u2**: won 5/6, lost 1/6
- **ml_tree_u2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u2 | 21.0ns | 1008285.3ns | 0.0% |  |
| ml_jumptable_u2 | 30.6ns | 1019348.3ns | 0.0% |  |
| ml_tree_u2 | 22.6ns | 1326453.6ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u2 (n=6, range 1000410.8-1015742.5 ns)
  1000410.8 |########################################
  1001177.4 |
  1001944.0 |
  1002710.6 |
  1003477.1 |
  1004243.7 |########################################
  1005010.3 |########################################
  1005776.9 |
  1006543.5 |
  1007310.1 |
  1008076.7 |########################################
  1008843.2 |
  1009609.8 |
  1010376.4 |
  1011143.0 |
  1011909.6 |
  1012676.2 |
  1013442.7 |########################################
  1014209.3 |
  1014975.9 |
  (0 below, 1 above range)

ml_jumptable_u2 (n=6, range 1000865.8-1031931.1 ns)
  1000865.8 |########################################
  1002419.1 |
  1003972.3 |
  1005525.6 |
  1007078.9 |
  1008632.1 |########################################
  1010185.4 |
  1011738.6 |
  1013291.9 |
  1014845.2 |########################################
  1016398.4 |
  1017951.7 |
  1019505.0 |
  1021058.2 |
  1022611.5 |
  1024164.7 |
  1025718.0 |########################################
  1027271.3 |
  1028824.5 |########################################
  1030377.8 |
  (0 below, 1 above range)

ml_tree_u2 (n=6, range 1319666.2-1334784.0 ns)
  1319666.2 |########################################
  1320422.1 |
  1321178.0 |####################
  1321933.9 |
  1322689.8 |
  1323445.6 |
  1324201.5 |
  1324957.4 |
  1325713.3 |
  1326469.2 |
  1327225.1 |
  1327981.0 |####################
  1328736.9 |
  1329492.7 |
  1330248.6 |
  1331004.5 |
  1331760.4 |
  1332516.3 |
  1333272.2 |####################
  1334028.1 |
  (0 below, 1 above range)

```
