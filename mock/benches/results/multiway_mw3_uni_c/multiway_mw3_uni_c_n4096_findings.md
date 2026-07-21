# Multiway branch strategies, cheap-arm, mw3_uni: 3-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_c_mw3_uni** at 7859.0 ns median (-7.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.75x (fastest 7859.0 ns, slowest 13750.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 11064ns | 11212ns | 9081ns | 11070ns | 12046ns | base |
| mw_chain_c_mw3_uni | 10607ns | 10851ns | 9085ns | 10826ns | 11038ns | -4.13% |
| mw_chain_rev_c_mw3_uni | 10189ns | 10375ns | 8946ns | 10004ns | 11087ns | -7.91% |
| mw_jumptable_c_mw3_uni | 10505ns | 10630ns | 9142ns | 10461ns | 11253ns | -5.05% |
| mw_predicate_all_c_mw3_uni | 16183ns | 16171ns | 14449ns | 15623ns | 17890ns | +46.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 8421ns | 6901ns | 9213ns | base | 0.486 |
| mw_chain_c_mw3_uni | 8061ns | 6916ns | 8388ns | -4.27% | 0.508 |
| mw_chain_rev_c_mw3_uni | 7713ns | 6771ns | 8391ns | -8.40% | 0.531 |
| mw_jumptable_c_mw3_uni | 7980ns | 6947ns | 8547ns | -5.23% | 0.513 |
| mw_predicate_all_c_mw3_uni | 13739ns | 12288ns | 15147ns | +63.16% | 0.298 |

## Performance model

- Peak throughput: **0.605 Gops/s** (mw_chain_rev_c_mw3_uni; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.482 | 79.6% |
| mw_chain_c_mw3_uni | 0.497 | 82.1% |
| mw_chain_rev_c_mw3_uni | 0.521 | 86.2% |
| mw_jumptable_c_mw3_uni | 0.507 | 83.9% |
| mw_predicate_all_c_mw3_uni | 0.298 | 49.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_uni | 11064ns | 11064ns | base |
| mw_chain_c_mw3_uni | 10607ns | 10607ns | -4.13% |
| mw_chain_rev_c_mw3_uni | 10189ns | 10189ns | -7.91% |
| mw_jumptable_c_mw3_uni | 10505ns | 10505ns | -5.05% |
| mw_predicate_all_c_mw3_uni | 16183ns | 16183ns | +46.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 8506ns | base | --- | [7544, 9213] | --- | --- | --- | --- |
| mw_chain_c_mw3_uni | 8244ns | no significant difference | [-940, +41]ns | [7552, 8388] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_c_mw3_uni | 7859ns | -467.3ns (-5.5%) | [-1556, -99]ns | [6890, 8391] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| mw_jumptable_c_mw3_uni | 8074ns | no significant difference | [-1228, +26]ns | [7321, 8547] | no | 0.9167 | 0.6875 | 0 |
| mw_predicate_all_c_mw3_uni | 13751ns | +5895.2ns (+69.3%) | [+3460, +6600]ns | [12320, 15147] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_uni | mw_chain_c_mw3_uni | mw_chain_rev_c_mw3_uni | mw_jumptable_c_mw3_uni | mw_predicate_all_c_mw3_uni |
|---|---|---|---|---|---|
| 1 | 9533ns | -13.8% | -20.3% | -19.3% | +28.9% |
| 2 | 6901ns | +0.2% | -1.9% | +0.7% | +85.2% |
| 3 | 8187ns | +0.0% | -14.4% | -2.8% | +50.9% |
| 4 | 8202ns | +0.8% | -0.8% | -0.1% | +85.5% |
| 5 | 8810ns | -4.1% | -7.8% | -7.0% | +67.1% |
| 6 | 8892ns | -6.4% | -2.7% | +0.1% | +69.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_uni | -0.295 | moderate- |
| mw_chain_c_mw3_uni | -0.071 | ok |
| mw_chain_rev_c_mw3_uni | 0.393 | moderate+ |
| mw_jumptable_c_mw3_uni | 0.267 | moderate+ |
| mw_predicate_all_c_mw3_uni | 0.347 | moderate+ |

**Consistency summary:**

- **mw_chain_c_mw3_uni**: won 3/6, lost 2/6
- **mw_chain_rev_c_mw3_uni**: won 6/6, lost 0/6
- **mw_jumptable_c_mw3_uni**: won 3/6, lost 1/6
- **mw_predicate_all_c_mw3_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 4.6ns | 8420.8ns | 0.1% |  |
| mw_chain_c_mw3_uni | 3.5ns | 8061.4ns | 0.0% |  |
| mw_chain_rev_c_mw3_uni | 4.6ns | 7713.4ns | 0.1% |  |
| mw_jumptable_c_mw3_uni | 2.9ns | 7980.4ns | 0.0% |  |
| mw_predicate_all_c_mw3_uni | 3.8ns | 13739.2ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_uni (n=6, range 6901.2-9212.7 ns)
   6901.2 |####################
   7016.8 |
   7132.4 |
   7247.9 |
   7363.5 |
   7479.1 |
   7594.6 |
   7710.2 |
   7825.8 |
   7941.4 |
   8057.0 |
   8172.5 |########################################
   8288.1 |
   8403.7 |
   8519.2 |
   8634.8 |
   8750.4 |####################
   8866.0 |####################
   8981.6 |
   9097.1 |
  (0 below, 1 above range)

mw_chain_c_mw3_uni (n=6, range 6915.8-8388.4 ns)
   6915.8 |####################
   6989.4 |
   7063.1 |
   7136.7 |
   7210.3 |
   7283.9 |
   7357.6 |
   7431.2 |
   7504.8 |
   7578.4 |
   7652.1 |
   7725.7 |
   7799.3 |
   7873.0 |
   7946.6 |
   8020.2 |
   8093.8 |
   8167.5 |########################################
   8241.1 |####################
   8314.7 |####################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_uni (n=6, range 6771.2-8390.8 ns)
   6771.2 |####################
   6852.2 |
   6933.2 |####################
   7014.1 |
   7095.1 |
   7176.1 |
   7257.1 |
   7338.1 |
   7419.0 |
   7500.0 |
   7581.0 |####################
   7662.0 |
   7743.0 |
   7823.9 |
   7904.9 |
   7985.9 |
   8066.9 |########################################
   8147.9 |
   8228.8 |
   8309.8 |
  (0 below, 1 above range)

mw_jumptable_c_mw3_uni (n=6, range 6947.1-8546.6 ns)
   6947.1 |####################
   7027.1 |
   7107.1 |
   7187.0 |
   7267.0 |
   7347.0 |
   7427.0 |
   7506.9 |
   7586.9 |
   7666.9 |####################
   7746.9 |
   7826.9 |
   7906.8 |####################
   7986.8 |
   8066.8 |
   8146.8 |########################################
   8226.7 |
   8306.7 |
   8386.7 |
   8466.7 |
  (0 below, 1 above range)

mw_predicate_all_c_mw3_uni (n=6, range 12287.9-15146.8 ns)
  12287.9 |########################################
  12430.8 |
  12573.8 |
  12716.7 |####################
  12859.7 |
  13002.6 |
  13145.6 |
  13288.5 |
  13431.5 |
  13574.4 |
  13717.4 |
  13860.3 |
  14003.3 |
  14146.2 |
  14289.2 |
  14432.1 |
  14575.1 |
  14718.0 |####################
  14861.0 |
  15003.9 |####################
  (0 below, 1 above range)

```
