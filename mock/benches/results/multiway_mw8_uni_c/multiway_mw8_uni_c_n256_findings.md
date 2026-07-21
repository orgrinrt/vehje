# Multiway branch strategies, cheap-arm, mw8_uni: 8-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_uni**

## Key findings

- **Fastest: mw_chain_c_mw8_uni** at 348.1 ns median (-41.3% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.21x (fastest 348.1 ns, slowest 1812.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 3065ns | 3052ns | 2648ns | 2965ns | 3422ns | base |
| mw_chain_c_mw8_uni | 2864ns | 2835ns | 2601ns | 2809ns | 3079ns | -6.53% |
| mw_chain_rev_c_mw8_uni | 2863ns | 2857ns | 2433ns | 2729ns | 3278ns | -6.58% |
| mw_jumptable_c_mw8_uni | 2877ns | 2858ns | 2579ns | 2824ns | 3106ns | -6.11% |
| mw_predicate_all_c_mw8_uni | 4346ns | 3982ns | 3914ns | 3979ns | 5114ns | +41.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 574ns | 483ns | 621ns | base | 0.446 |
| mw_chain_c_mw8_uni | 357ns | 324ns | 388ns | -37.83% | 0.717 |
| mw_chain_rev_c_mw8_uni | 357ns | 305ns | 404ns | -37.78% | 0.717 |
| mw_jumptable_c_mw8_uni | 352ns | 311ns | 387ns | -38.78% | 0.728 |
| mw_predicate_all_c_mw8_uni | 1966ns | 1783ns | 2290ns | +242.34% | 0.130 |

## Performance model

- Peak throughput: **0.838 Gops/s** (mw_chain_rev_c_mw8_uni; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.432 | 51.5% |
| mw_chain_c_mw8_uni | 0.735 | 87.7% |
| mw_chain_rev_c_mw8_uni | 0.712 | 84.9% |
| mw_jumptable_c_mw8_uni | 0.735 | 87.7% |
| mw_predicate_all_c_mw8_uni | 0.141 | 16.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_uni | 3065ns | 3065ns | base |
| mw_chain_c_mw8_uni | 2864ns | 2864ns | -6.53% |
| mw_chain_rev_c_mw8_uni | 2863ns | 2863ns | -6.58% |
| mw_jumptable_c_mw8_uni | 2877ns | 2877ns | -6.11% |
| mw_predicate_all_c_mw8_uni | 4346ns | 4346ns | +41.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 593ns | base | --- | [509, 621] | --- | --- | --- | --- |
| mw_chain_c_mw8_uni | 348ns | -221.0ns (-37.3%) | [-258, -173]ns | [335, 388] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_uni | 360ns | -217.4ns (-36.7%) | [-266, -167]ns | [309, 404] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_uni | 348ns | -208.7ns (-35.2%) | [-280, -180]ns | [319, 387] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_uni | 1812ns | +1284.2ns (+216.5%) | [+1178, +1713]ns | [1795, 2290] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_uni | mw_chain_c_mw8_uni | mw_chain_rev_c_mw8_uni | mw_jumptable_c_mw8_uni | mw_predicate_all_c_mw8_uni |
|---|---|---|---|---|---|
| 1 | 621ns | -40.1% | -35.3% | -43.7% | +187.0% |
| 2 | 621ns | -35.0% | -34.8% | -35.2% | +277.8% |
| 3 | 571ns | -39.3% | -39.0% | -34.8% | +216.4% |
| 4 | 534ns | -34.9% | -30.6% | -35.0% | +318.4% |
| 5 | 483ns | -33.0% | -35.5% | -35.6% | +275.9% |
| 6 | 615ns | -43.3% | -50.3% | -46.8% | +194.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_uni | 0.131 | ok |
| mw_chain_c_mw8_uni | 0.244 | moderate+ |
| mw_chain_rev_c_mw8_uni | 0.361 | moderate+ |
| mw_jumptable_c_mw8_uni | 0.388 | moderate+ |
| mw_predicate_all_c_mw8_uni | -0.587 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mw_chain_c_mw8_uni**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_uni**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_uni**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_uni | 5.8ns | 574.2ns | 1.0% |  |
| mw_chain_c_mw8_uni | 5.5ns | 357.0ns | 1.5% |  |
| mw_chain_rev_c_mw8_uni | 4.2ns | 357.3ns | 1.2% |  |
| mw_jumptable_c_mw8_uni | 4.9ns | 351.5ns | 1.4% |  |
| mw_predicate_all_c_mw8_uni | 5.3ns | 1965.8ns | 0.3% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_uni (n=6, range 482.9-621.0 ns)
    482.9 |####################
    489.8 |
    496.7 |
    503.6 |
    510.5 |
    517.4 |
    524.3 |
    531.2 |####################
    538.1 |
    545.0 |
    552.0 |
    558.9 |
    565.8 |####################
    572.7 |
    579.6 |
    586.5 |
    593.4 |
    600.3 |
    607.2 |
    614.1 |########################################
  (0 below, 1 above range)

mw_chain_c_mw8_uni (n=6, range 323.7-387.7 ns)
    323.7 |#############
    326.9 |
    330.1 |
    333.3 |
    336.5 |
    339.7 |
    342.9 |
    346.1 |########################################
    349.3 |
    352.5 |
    355.7 |
    358.9 |
    362.1 |
    365.3 |
    368.5 |
    371.7 |#############
    374.9 |
    378.1 |
    381.3 |
    384.5 |
  (0 below, 1 above range)

mw_chain_rev_c_mw8_uni (n=6, range 305.4-403.6 ns)
    305.4 |########################################
    310.3 |########################################
    315.2 |
    320.1 |
    325.0 |
    329.9 |
    334.8 |
    339.8 |
    344.7 |########################################
    349.6 |
    354.5 |
    359.4 |
    364.3 |
    369.2 |########################################
    374.1 |
    379.0 |
    383.9 |
    388.8 |
    393.7 |
    398.6 |########################################
  (0 below, 1 above range)

mw_jumptable_c_mw8_uni (n=6, range 310.8-387.3 ns)
    310.8 |########################################
    314.6 |
    318.4 |
    322.3 |
    326.1 |########################################
    329.9 |
    333.8 |
    337.6 |
    341.4 |
    345.2 |########################################
    349.1 |########################################
    352.9 |
    356.7 |
    360.5 |
    364.4 |
    368.2 |
    372.0 |########################################
    375.8 |
    379.7 |
    383.5 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_uni (n=6, range 1782.9-2290.2 ns)
   1782.9 |########################################
   1808.3 |########################################
   1833.6 |
   1859.0 |
   1884.4 |
   1909.7 |
   1935.1 |
   1960.5 |
   1985.8 |
   2011.2 |
   2036.5 |
   2061.9 |
   2087.3 |
   2112.6 |
   2138.0 |
   2163.4 |
   2188.7 |
   2214.1 |####################
   2239.5 |
   2264.8 |
  (0 below, 1 above range)

```
