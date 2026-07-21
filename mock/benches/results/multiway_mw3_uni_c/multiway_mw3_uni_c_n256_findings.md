# Multiway branch strategies, cheap-arm, mw3_uni: 3-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_c_mw3_uni** at 581.0 ns median (-5.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.78x (fastest 581.0 ns, slowest 1032.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 3313ns | 3458ns | 2613ns | 3209ns | 3819ns | base |
| mw_chain_c_mw3_uni | 3363ns | 3545ns | 2618ns | 3241ns | 3919ns | +1.52% |
| mw_chain_rev_c_mw3_uni | 3263ns | 3375ns | 2609ns | 3154ns | 3754ns | -1.50% |
| mw_jumptable_c_mw3_uni | 3181ns | 3382ns | 2617ns | 3127ns | 3544ns | -3.98% |
| mw_predicate_all_c_mw3_uni | 3737ns | 3833ns | 2996ns | 3577ns | 4347ns | +12.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 585ns | 454ns | 682ns | base | 0.438 |
| mw_chain_c_mw3_uni | 583ns | 453ns | 679ns | -0.28% | 0.439 |
| mw_chain_rev_c_mw3_uni | 561ns | 448ns | 645ns | -4.15% | 0.457 |
| mw_jumptable_c_mw3_uni | 553ns | 455ns | 614ns | -5.54% | 0.463 |
| mw_predicate_all_c_mw3_uni | 1000ns | 811ns | 1150ns | +70.93% | 0.256 |

## Performance model

- Peak throughput: **0.571 Gops/s** (mw_chain_rev_c_mw3_uni; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.418 | 73.3% |
| mw_chain_c_mw3_uni | 0.415 | 72.7% |
| mw_chain_rev_c_mw3_uni | 0.441 | 77.2% |
| mw_jumptable_c_mw3_uni | 0.436 | 76.4% |
| mw_predicate_all_c_mw3_uni | 0.248 | 43.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_uni | 3313ns | 3313ns | base |
| mw_chain_c_mw3_uni | 3363ns | 3363ns | +1.52% |
| mw_chain_rev_c_mw3_uni | 3263ns | 3263ns | -1.50% |
| mw_jumptable_c_mw3_uni | 3181ns | 3181ns | -3.98% |
| mw_predicate_all_c_mw3_uni | 3737ns | 3737ns | +12.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 612ns | base | --- | [461, 682] | --- | --- | --- | --- |
| mw_chain_c_mw3_uni | 617ns | no significant difference | [-31, +24]ns | [454, 679] | no | 1.0000 | 1.0000 | 0 |
| mw_chain_rev_c_mw3_uni | 581ns | no significant difference | [-84, +16]ns | [456, 645] | no | 0.4375 | 0.2188 | 0 |
| mw_jumptable_c_mw3_uni | 587ns | no significant difference | [-78, +3]ns | [456, 614] | no | 0.9167 | 0.6875 | 0 |
| mw_predicate_all_c_mw3_uni | 1032ns | +376.3ns (+61.5%) | [+350, +518]ns | [818, 1150] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_uni | mw_chain_c_mw3_uni | mw_chain_rev_c_mw3_uni | mw_jumptable_c_mw3_uni | mw_predicate_all_c_mw3_uni |
|---|---|---|---|---|---|
| 1 | 467ns | -2.6% | -0.8% | -2.6% | +76.6% |
| 2 | 454ns | -0.2% | -1.3% | +0.6% | +78.5% |
| 3 | 638ns | +0.7% | -8.9% | -8.1% | +61.8% |
| 4 | 672ns | +6.3% | +5.4% | -4.7% | +87.9% |
| 5 | 585ns | +1.0% | -0.8% | +0.5% | +76.2% |
| 6 | 693ns | -7.1% | -15.9% | -15.2% | +49.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.248 | moderate+ |
| mw_chain_c_mw3_uni | 0.316 | moderate+ |
| mw_chain_rev_c_mw3_uni | 0.334 | moderate+ |
| mw_jumptable_c_mw3_uni | 0.448 | moderate+ |
| mw_predicate_all_c_mw3_uni | 0.324 | moderate+ |

**Consistency summary:**

- **mw_chain_c_mw3_uni**: won 3/6, lost 3/6
- **mw_chain_rev_c_mw3_uni**: won 5/6, lost 1/6
- **mw_jumptable_c_mw3_uni**: won 4/6, lost 2/6
- **mw_predicate_all_c_mw3_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 4.7ns | 584.9ns | 0.8% |  |
| mw_chain_c_mw3_uni | 5.9ns | 583.3ns | 1.0% |  |
| mw_chain_rev_c_mw3_uni | 4.6ns | 560.7ns | 0.8% |  |
| mw_jumptable_c_mw3_uni | 5.8ns | 552.5ns | 1.0% |  |
| mw_predicate_all_c_mw3_uni | 6.6ns | 999.9ns | 0.7% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_uni (n=6, range 454.2-682.3 ns)
    454.2 |########################################
    465.6 |########################################
    477.0 |
    488.4 |
    499.8 |
    511.2 |
    522.6 |
    534.0 |
    545.4 |
    556.8 |
    568.2 |
    579.7 |########################################
    591.1 |
    602.5 |
    613.9 |
    625.3 |
    636.7 |########################################
    648.1 |
    659.5 |
    670.9 |########################################
  (0 below, 1 above range)

mw_chain_c_mw3_uni (n=6, range 453.3-679.0 ns)
    453.3 |########################################
    464.6 |
    475.9 |
    487.1 |
    498.4 |
    509.7 |
    521.0 |
    532.3 |
    543.6 |
    554.8 |
    566.1 |
    577.4 |
    588.7 |####################
    600.0 |
    611.3 |
    622.5 |
    633.8 |########################################
    645.1 |
    656.4 |
    667.7 |
  (0 below, 1 above range)

mw_chain_rev_c_mw3_uni (n=6, range 448.3-645.2 ns)
    448.3 |#############
    458.1 |#############
    468.0 |
    477.8 |
    487.7 |
    497.5 |
    507.4 |
    517.2 |
    527.1 |
    536.9 |
    546.8 |
    556.6 |
    566.4 |
    576.3 |########################################
    586.1 |
    596.0 |
    605.8 |
    615.7 |
    625.5 |
    635.4 |
  (0 below, 1 above range)

mw_jumptable_c_mw3_uni (n=6, range 455.0-614.3 ns)
    455.0 |##########################
    463.0 |
    470.9 |
    478.9 |
    486.9 |
    494.8 |
    502.8 |
    510.8 |
    518.7 |
    526.7 |
    534.7 |
    542.6 |
    550.6 |
    558.6 |
    566.5 |
    574.5 |
    582.5 |########################################
    590.4 |
    598.4 |
    606.4 |
  (0 below, 1 above range)

mw_predicate_all_c_mw3_uni (n=6, range 810.8-1149.6 ns)
    810.8 |##########################
    827.7 |
    844.7 |
    861.6 |
    878.6 |
    895.5 |
    912.4 |
    929.4 |
    946.3 |
    963.3 |
    980.2 |
    997.1 |
   1014.1 |
   1031.0 |########################################
   1048.0 |
   1064.9 |
   1081.8 |
   1098.8 |
   1115.7 |
   1132.7 |
  (0 below, 1 above range)

```
