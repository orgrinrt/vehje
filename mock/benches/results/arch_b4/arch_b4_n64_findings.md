# Per-branch strategy: archetype 4 (ifchain4_linear), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_b4_table**

## Highlights

Baseline for all deltas below: **ab_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: ab_b4_prof** at 22444.6 ns median (-4.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 22444.6 ns, slowest 27518.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_b4_pred | 29855ns | 30051ns | 27392ns | 29764ns | 31224ns | +18.89% |
| ab_b4_prof | 24805ns | 24793ns | 22990ns | 24244ns | 26553ns | -1.22% |
| ab_b4_seq | 26194ns | 26398ns | 23418ns | 26222ns | 27542ns | +4.31% |
| ab_b4_table | 25111ns | 25947ns | 22304ns | 24860ns | 26891ns | base |
| ab_b4_tree | 25717ns | 26060ns | 23156ns | 25830ns | 26827ns | +2.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_b4_pred | 27343ns | 25133ns | 28581ns | +20.67% | 0.002 |
| ab_b4_prof | 22448ns | 20833ns | 24005ns | -0.93% | 0.003 |
| ab_b4_seq | 23699ns | 21223ns | 24907ns | +4.59% | 0.003 |
| ab_b4_table | 22660ns | 20124ns | 24274ns | base | 0.003 |
| ab_b4_tree | 23192ns | 20938ns | 24205ns | +2.35% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_b4_table; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_b4_pred | 0.002 | 73.1% |
| ab_b4_prof | 0.003 | 89.7% |
| ab_b4_seq | 0.003 | 84.3% |
| ab_b4_table | 0.003 | 85.9% |
| ab_b4_tree | 0.003 | 85.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_b4_pred | 29855ns | 29855ns | +18.89% |
| ab_b4_prof | 24805ns | 24805ns | -1.22% |
| ab_b4_seq | 26194ns | 26194ns | +4.31% |
| ab_b4_table | 25111ns | 25111ns | base |
| ab_b4_tree | 25717ns | 25717ns | +2.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_b4_table | 23416ns | base | --- | [20290, 24274] | --- | --- | --- | --- |
| ab_b4_pred | 27518ns | +4307.3ns (+18.4%) | [+3730, +6014]ns | [25930, 28581] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| ab_b4_prof | 22445ns | no significant difference | [-1758, +1041]ns | [20896, 24005] | no | 1.0000 | 1.0000 | 0 |
| ab_b4_seq | 23877ns | no significant difference | [-2, +2358]ns | [22313, 24907] | no | 0.4375 | 0.2188 | 0 |
| ab_b4_tree | 23452ns | no significant difference | [-1250, +2453]ns | [21918, 24205] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_b4_table | ab_b4_pred | ab_b4_prof | ab_b4_seq | ab_b4_tree |
|---|---|---|---|---|---|
| 1 | 20124ns | +36.5% | +3.5% | +18.4% | +19.5% |
| 2 | 22914ns | +16.6% | +6.0% | +4.5% | +4.3% |
| 3 | 24487ns | +16.4% | -5.2% | +2.1% | -6.0% |
| 4 | 24061ns | +19.2% | -1.4% | +3.1% | +1.3% |
| 5 | 23917ns | +15.2% | -9.3% | -2.1% | -4.3% |
| 6 | 20455ns | +22.9% | +2.5% | +3.8% | +2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_b4_pred | 0.065 | ok |
| ab_b4_prof | -0.041 | ok |
| ab_b4_seq | 0.237 | moderate+ |
| ab_b4_table | 0.075 | ok |
| ab_b4_tree | 0.074 | ok |

**Consistency summary:**

- **ab_b4_pred**: won 0/6, lost 6/6
- **ab_b4_prof**: won 3/6, lost 3/6
- **ab_b4_seq**: won 1/6, lost 5/6
- **ab_b4_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_b4_pred | 3.3ns | 27343.3ns | 0.0% |  |
| ab_b4_prof | 3.8ns | 22448.5ns | 0.0% |  |
| ab_b4_seq | 3.4ns | 23699.3ns | 0.0% |  |
| ab_b4_table | 3.7ns | 22659.8ns | 0.0% |  |
| ab_b4_tree | 4.4ns | 23191.9ns | 0.0% |  |

## Distribution (algo ns)

```
ab_b4_pred (n=6, range 25132.9-28581.5 ns)
  25132.9 |########################################
  25305.3 |
  25477.8 |
  25650.2 |
  25822.6 |
  25995.0 |
  26167.5 |
  26339.9 |
  26512.3 |
  26684.7 |########################################
  26857.2 |
  27029.6 |
  27202.0 |
  27374.5 |########################################
  27546.9 |########################################
  27719.3 |
  27891.7 |
  28064.2 |
  28236.6 |
  28409.0 |########################################
  (0 below, 1 above range)

ab_b4_prof (n=6, range 20832.9-24005.2 ns)
  20832.9 |########################################
  20991.5 |
  21150.1 |
  21308.7 |
  21467.4 |
  21626.0 |####################
  21784.6 |
  21943.2 |
  22101.8 |
  22260.4 |
  22419.0 |
  22577.7 |
  22736.3 |
  22894.9 |
  23053.5 |####################
  23212.1 |
  23370.7 |
  23529.4 |
  23688.0 |####################
  23846.6 |
  (0 below, 1 above range)

ab_b4_seq (n=6, range 21222.9-24907.1 ns)
  21222.9 |####################
  21407.1 |
  21591.3 |
  21775.5 |
  21959.7 |
  22144.0 |
  22328.2 |
  22512.4 |
  22696.6 |
  22880.8 |
  23065.0 |
  23249.2 |####################
  23433.4 |
  23617.6 |
  23801.8 |########################################
  23986.0 |
  24170.3 |
  24354.5 |
  24538.7 |
  24722.9 |####################
  (0 below, 1 above range)

ab_b4_table (n=6, range 20124.2-24274.2 ns)
  20124.2 |####################
  20331.7 |####################
  20539.2 |
  20746.7 |
  20954.2 |
  21161.7 |
  21369.2 |
  21576.7 |
  21784.2 |
  21991.7 |
  22199.2 |
  22406.7 |
  22614.2 |
  22821.7 |####################
  23029.2 |
  23236.7 |
  23444.2 |
  23651.7 |
  23859.2 |########################################
  24066.7 |
  (0 below, 1 above range)

ab_b4_tree (n=6, range 20937.9-24205.4 ns)
  20937.9 |########################################
  21101.3 |
  21264.7 |
  21428.0 |
  21591.4 |
  21754.8 |
  21918.2 |
  22081.5 |
  22244.9 |
  22408.3 |
  22571.7 |
  22735.0 |########################################
  22898.4 |########################################
  23061.8 |
  23225.2 |
  23388.5 |
  23551.9 |
  23715.3 |
  23878.7 |########################################
  24042.0 |########################################
  (0 below, 1 above range)

```
