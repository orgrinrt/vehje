# Multiway branch strategies, heavy-arm, mw3_uni: 3-way, uniform key

4 variants, 6 samples per variant.
Baseline: **mw_bintree_h_mw3_uni**

## Key findings

- **Baseline (mw_bintree_h_mw3_uni) is the fastest** at 72441.9 ns median
- Spread: 1.01x (fastest 72441.9 ns, slowest 73486.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 77875ns | 74738ns | 74127ns | 74548ns | 84739ns | base |
| mw_chain_h_mw3_uni | 77683ns | 75008ns | 73163ns | 74816ns | 84243ns | -0.25% |
| mw_chain_rev_h_mw3_uni | 78452ns | 75751ns | 73273ns | 75319ns | 85742ns | +0.74% |
| mw_jumptable_h_mw3_uni | 78424ns | 75638ns | 73662ns | 75161ns | 85699ns | +0.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 75493ns | 71875ns | 82144ns | base | 0.054 |
| mw_chain_h_mw3_uni | 75323ns | 71001ns | 81696ns | -0.23% | 0.054 |
| mw_chain_rev_h_mw3_uni | 76060ns | 71122ns | 83016ns | +0.75% | 0.054 |
| mw_jumptable_h_mw3_uni | 75987ns | 71472ns | 82994ns | +0.65% | 0.054 |

## Performance model

- Peak throughput: **0.058 Gops/s** (mw_chain_h_mw3_uni; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.057 | 98.0% |
| mw_chain_h_mw3_uni | 0.056 | 97.6% |
| mw_chain_rev_h_mw3_uni | 0.056 | 96.6% |
| mw_jumptable_h_mw3_uni | 0.056 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_h_mw3_uni | 77875ns | 77875ns | base |
| mw_chain_h_mw3_uni | 77683ns | 77683ns | -0.25% |
| mw_chain_rev_h_mw3_uni | 78452ns | 78452ns | +0.74% |
| mw_jumptable_h_mw3_uni | 78424ns | 78424ns | +0.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 72442ns | base | --- | [71895, 82144] | --- | --- | --- | --- |
| mw_chain_h_mw3_uni | 72775ns | no significant difference | [-1175, +687]ns | [71497, 81696] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_h_mw3_uni | 73486ns | no significant difference | [-764, +1592]ns | [71678, 83016] | no | 1.0000 | 0.6875 | 0 |
| mw_jumptable_h_mw3_uni | 73269ns | no significant difference | [-754, +2391]ns | [71699, 82994] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_h_mw3_uni | mw_chain_h_mw3_uni | mw_chain_rev_h_mw3_uni | mw_jumptable_h_mw3_uni |
|---|---|---|---|---|
| 1 | 71914ns | +0.1% | +2.6% | +3.7% |
| 2 | 72581ns | -2.2% | -2.0% | -1.5% |
| 3 | 82510ns | -0.9% | +0.8% | -0.5% |
| 4 | 81778ns | -0.2% | +1.3% | +2.6% |
| 5 | 71875ns | +1.3% | +1.8% | +0.1% |
| 6 | 72302ns | +0.6% | -0.1% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_h_mw3_uni | 0.172 | ok |
| mw_chain_h_mw3_uni | 0.145 | ok |
| mw_chain_rev_h_mw3_uni | 0.107 | ok |
| mw_jumptable_h_mw3_uni | 0.073 | ok |

**Consistency summary:**

- **mw_chain_h_mw3_uni**: won 3/6, lost 3/6
- **mw_chain_rev_h_mw3_uni**: won 1/6, lost 4/6
- **mw_jumptable_h_mw3_uni**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_h_mw3_uni | 4.0ns | 75493.5ns | 0.0% |  |
| mw_chain_h_mw3_uni | 3.9ns | 75322.8ns | 0.0% |  |
| mw_chain_rev_h_mw3_uni | 3.2ns | 76060.1ns | 0.0% |  |
| mw_jumptable_h_mw3_uni | 2.9ns | 75987.2ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_h_mw3_uni (n=6, range 71875.0-82144.0 ns)
  71875.0 |########################################
  72388.4 |#############
  72901.9 |
  73415.3 |
  73928.8 |
  74442.2 |
  74955.7 |
  75469.1 |
  75982.6 |
  76496.0 |
  77009.5 |
  77522.9 |
  78036.4 |
  78549.8 |
  79063.3 |
  79576.7 |
  80090.2 |
  80603.6 |
  81117.1 |
  81630.5 |#############
  (0 below, 1 above range)

mw_chain_h_mw3_uni (n=6, range 71000.8-81696.5 ns)
  71000.8 |####################
  71535.6 |####################
  72070.4 |
  72605.1 |########################################
  73139.9 |
  73674.7 |
  74209.5 |
  74744.3 |
  75279.1 |
  75813.8 |
  76348.6 |
  76883.4 |
  77418.2 |
  77953.0 |
  78487.8 |
  79022.5 |
  79557.3 |
  80092.1 |
  80626.9 |
  81161.7 |####################
  (0 below, 1 above range)

mw_chain_rev_h_mw3_uni (n=6, range 71121.7-83016.0 ns)
  71121.7 |########################################
  71716.4 |########################################
  72311.1 |
  72905.9 |########################################
  73500.6 |########################################
  74095.3 |
  74690.0 |
  75284.7 |
  75879.4 |
  76474.2 |
  77068.9 |
  77663.6 |
  78258.3 |
  78853.0 |
  79447.7 |
  80042.5 |
  80637.2 |
  81231.9 |
  81826.6 |
  82421.3 |########################################
  (0 below, 1 above range)

mw_jumptable_h_mw3_uni (n=6, range 71471.7-82994.2 ns)
  71471.7 |########################################
  72047.8 |
  72623.9 |
  73200.1 |
  73776.2 |
  74352.3 |#############
  74928.4 |
  75504.6 |
  76080.7 |
  76656.8 |
  77232.9 |
  77809.1 |
  78385.2 |
  78961.3 |
  79537.4 |
  80113.6 |
  80689.7 |
  81265.8 |
  81841.9 |#############
  82418.1 |
  (0 below, 1 above range)

```
