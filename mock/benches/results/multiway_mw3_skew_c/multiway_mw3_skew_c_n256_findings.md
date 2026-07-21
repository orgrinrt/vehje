# Multiway branch strategies, cheap-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_skew**

## Key findings

- **Fastest: mw_jumptable_c_mw3_skew** at 403.8 ns median (-11.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.18x (fastest 403.8 ns, slowest 881.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 3074ns | 3252ns | 2507ns | 3035ns | 3416ns | base |
| mw_chain_c_mw3_skew | 3003ns | 3004ns | 2521ns | 3000ns | 3247ns | -2.31% |
| mw_chain_rev_c_mw3_skew | 3193ns | 3251ns | 2518ns | 3250ns | 3446ns | +3.89% |
| mw_jumptable_c_mw3_skew | 2887ns | 2892ns | 2509ns | 2769ns | 3253ns | -6.07% |
| mw_predicate_all_c_mw3_skew | 3495ns | 3398ns | 2895ns | 3272ns | 4129ns | +13.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 428ns | 348ns | 476ns | base | 0.598 |
| mw_chain_c_mw3_skew | 418ns | 351ns | 454ns | -2.29% | 0.612 |
| mw_chain_rev_c_mw3_skew | 434ns | 347ns | 453ns | +1.43% | 0.590 |
| mw_jumptable_c_mw3_skew | 402ns | 349ns | 453ns | -6.07% | 0.637 |
| mw_predicate_all_c_mw3_skew | 906ns | 750ns | 1071ns | +111.69% | 0.283 |

## Performance model

- Peak throughput: **0.738 Gops/s** (mw_chain_rev_c_mw3_skew; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.564 | 76.5% |
| mw_chain_c_mw3_skew | 0.613 | 83.1% |
| mw_chain_rev_c_mw3_skew | 0.568 | 77.1% |
| mw_jumptable_c_mw3_skew | 0.634 | 86.0% |
| mw_predicate_all_c_mw3_skew | 0.291 | 39.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_skew | 3074ns | 3074ns | base |
| mw_chain_c_mw3_skew | 3003ns | 3003ns | -2.31% |
| mw_chain_rev_c_mw3_skew | 3193ns | 3193ns | +3.89% |
| mw_jumptable_c_mw3_skew | 2887ns | 2887ns | -6.07% |
| mw_predicate_all_c_mw3_skew | 3495ns | 3495ns | +13.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 454ns | base | --- | [354, 476] | --- | --- | --- | --- |
| mw_chain_c_mw3_skew | 418ns | no significant difference | [-41, +34]ns | [383, 454] | no | 0.6875 | 0.6875 | 0 |
| mw_chain_rev_c_mw3_skew | 450ns | no significant difference | [-27, +51]ns | [399, 453] | no | 0.5000 | 0.3750 | **1** (17%, HIGH) |
| mw_jumptable_c_mw3_skew | 404ns | no significant difference | [-54, +0]ns | [349, 453] | no | 0.5000 | 0.3750 | **1** (17%, HIGH) |
| mw_predicate_all_c_mw3_skew | 881ns | +497.9ns (+109.8%) | [+311, +624]ns | [765, 1071] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_skew | mw_chain_c_mw3_skew | mw_chain_rev_c_mw3_skew | mw_jumptable_c_mw3_skew | mw_predicate_all_c_mw3_skew |
|---|---|---|---|---|---|
| 1 | 457ns | -9.1% | -1.3% | -14.6% | +64.3% |
| 2 | 455ns | -7.7% | -1.2% | -8.3% | +72.3% |
| 3 | 359ns | -2.3% | -3.4% | -2.9% | +116.8% |
| 4 | 348ns | +19.0% | +29.2% | +0.3% | +180.6% |
| 5 | 452ns | +0.2% | +0.0% | +0.0% | +137.1% |
| 6 | 496ns | -8.2% | -8.4% | -8.4% | +116.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.229 | moderate+ |
| mw_chain_c_mw3_skew | 0.172 | ok |
| mw_chain_rev_c_mw3_skew | -0.207 | moderate- |
| mw_jumptable_c_mw3_skew | 0.158 | ok |
| mw_predicate_all_c_mw3_skew | 0.560 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **mw_chain_c_mw3_skew**: won 4/6, lost 2/6
- **mw_chain_rev_c_mw3_skew**: won 4/6, lost 1/6
- **mw_jumptable_c_mw3_skew**: won 4/6, lost 1/6
- **mw_predicate_all_c_mw3_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 5.8ns | 427.8ns | 1.4% |  |
| mw_chain_c_mw3_skew | 5.3ns | 418.1ns | 1.3% |  |
| mw_chain_rev_c_mw3_skew | 5.6ns | 434.0ns | 1.3% |  |
| mw_jumptable_c_mw3_skew | 4.8ns | 401.9ns | 1.2% |  |
| mw_predicate_all_c_mw3_skew | 4.9ns | 905.7ns | 0.5% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_skew (n=6, range 348.3-476.2 ns)
    348.3 |#############
    354.7 |#############
    361.1 |
    367.5 |
    373.9 |
    380.3 |
    386.7 |
    393.1 |
    399.5 |
    405.9 |
    412.3 |
    418.7 |
    425.1 |
    431.5 |
    437.9 |
    444.3 |
    450.7 |########################################
    457.1 |
    463.5 |
    469.9 |
  (0 below, 1 above range)

mw_chain_c_mw3_skew (n=6, range 350.8-453.8 ns)
    350.8 |####################
    355.9 |
    361.1 |
    366.2 |
    371.4 |
    376.5 |
    381.7 |
    386.8 |
    392.0 |
    397.1 |
    402.3 |
    407.4 |
    412.6 |########################################
    417.7 |####################
    422.9 |
    428.0 |
    433.2 |
    438.3 |
    443.5 |
    448.6 |####################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_skew (n=6, range 347.1-452.9 ns)
    347.1 |##########
    352.4 |
    357.7 |
    363.0 |
    368.3 |
    373.6 |
    378.9 |
    384.1 |
    389.4 |
    394.7 |
    400.0 |
    405.3 |
    410.6 |
    415.9 |
    421.2 |
    426.5 |
    431.8 |
    437.1 |
    442.4 |
    447.7 |########################################
  (0 below, 1 above range)

mw_jumptable_c_mw3_skew (n=6, range 348.7-452.9 ns)
    348.7 |########################################
    353.9 |
    359.1 |
    364.3 |
    369.6 |
    374.8 |
    380.0 |
    385.2 |####################
    390.4 |
    395.6 |
    400.8 |
    406.0 |
    411.2 |
    416.5 |####################
    421.7 |
    426.9 |
    432.1 |
    437.3 |
    442.5 |
    447.7 |####################
  (0 below, 1 above range)

mw_predicate_all_c_mw3_skew (n=6, range 750.4-1071.4 ns)
    750.4 |########################################
    766.5 |########################################
    782.5 |########################################
    798.6 |
    814.6 |
    830.7 |
    846.7 |
    862.8 |
    878.8 |
    894.9 |
    910.9 |
    927.0 |
    943.0 |
    959.1 |
    975.1 |########################################
    991.2 |
   1007.2 |
   1023.3 |
   1039.3 |
   1055.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **mw_predicate_all_c_mw3_skew**: autocorrelation=0.56 (measurement drift or warm-up artifact)
