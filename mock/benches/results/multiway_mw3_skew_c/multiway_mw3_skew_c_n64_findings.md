# Multiway branch strategies, cheap-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_skew**

## Key findings

- **Fastest: mw_jumptable_c_mw3_skew** at 138.9 ns median (-7.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.99x (fastest 138.9 ns, slowest 276.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 3106ns | 3076ns | 2272ns | 3030ns | 3638ns | base |
| mw_chain_c_mw3_skew | 2999ns | 3085ns | 2268ns | 2814ns | 3642ns | -3.45% |
| mw_chain_rev_c_mw3_skew | 3183ns | 3223ns | 2275ns | 3051ns | 3834ns | +2.46% |
| mw_jumptable_c_mw3_skew | 2787ns | 2835ns | 2275ns | 2674ns | 3213ns | -10.27% |
| mw_predicate_all_c_mw3_skew | 2963ns | 3095ns | 2401ns | 2883ns | 3364ns | -4.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 150ns | 108ns | 176ns | base | 0.426 |
| mw_chain_c_mw3_skew | 144ns | 107ns | 178ns | -4.01% | 0.444 |
| mw_chain_rev_c_mw3_skew | 151ns | 111ns | 181ns | +0.74% | 0.423 |
| mw_jumptable_c_mw3_skew | 135ns | 106ns | 155ns | -10.46% | 0.476 |
| mw_predicate_all_c_mw3_skew | 264ns | 214ns | 300ns | +75.91% | 0.242 |

## Performance model

- Peak throughput: **0.603 Gops/s** (mw_jumptable_c_mw3_skew; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.428 | 71.0% |
| mw_chain_c_mw3_skew | 0.436 | 72.3% |
| mw_chain_rev_c_mw3_skew | 0.408 | 67.8% |
| mw_jumptable_c_mw3_skew | 0.461 | 76.4% |
| mw_predicate_all_c_mw3_skew | 0.232 | 38.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_skew | 3106ns | 3106ns | base |
| mw_chain_c_mw3_skew | 2999ns | 2999ns | -3.45% |
| mw_chain_rev_c_mw3_skew | 3183ns | 3183ns | +2.46% |
| mw_jumptable_c_mw3_skew | 2787ns | 2787ns | -10.27% |
| mw_predicate_all_c_mw3_skew | 2963ns | 2963ns | -4.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 150ns | base | --- | [125, 176] | --- | --- | --- | --- |
| mw_chain_c_mw3_skew | 147ns | no significant difference | [-20, +3]ns | [108, 178] | no | 0.6875 | 0.6875 | 0 |
| mw_chain_rev_c_mw3_skew | 157ns | no significant difference | [-11, +11]ns | [116, 181] | no | 0.6875 | 0.6875 | 0 |
| mw_jumptable_c_mw3_skew | 139ns | -6.5ns (-4.3%) | [-40, -1]ns | [110, 155] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| mw_predicate_all_c_mw3_skew | 276ns | +124.8ns (+83.4%) | [+67, +150]ns | [217, 300] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_skew | mw_chain_c_mw3_skew | mw_chain_rev_c_mw3_skew | mw_jumptable_c_mw3_skew | mw_predicate_all_c_mw3_skew |
|---|---|---|---|---|---|
| 1 | 195ns | +1.7% | -0.8% | -26.0% | +29.2% |
| 2 | 143ns | -23.3% | -14.8% | -20.4% | +54.0% |
| 3 | 108ns | -0.7% | +3.1% | -1.2% | +98.8% |
| 4 | 145ns | -4.0% | +6.6% | -8.1% | +106.9% |
| 5 | 157ns | -1.3% | +7.7% | -0.6% | +91.4% |
| 6 | 154ns | +1.6% | +3.0% | -0.3% | +94.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.050 | ok |
| mw_chain_c_mw3_skew | -0.054 | ok |
| mw_chain_rev_c_mw3_skew | -0.000 | ok |
| mw_jumptable_c_mw3_skew | 0.370 | moderate+ |
| mw_predicate_all_c_mw3_skew | 0.413 | moderate+ |

**Consistency summary:**

- **mw_chain_c_mw3_skew**: won 4/6, lost 2/6
- **mw_chain_rev_c_mw3_skew**: won 2/6, lost 4/6
- **mw_jumptable_c_mw3_skew**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw3_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 4.2ns | 150.3ns | 2.8% |  |
| mw_chain_c_mw3_skew | 4.0ns | 144.2ns | 2.7% |  |
| mw_chain_rev_c_mw3_skew | 4.2ns | 151.4ns | 2.8% |  |
| mw_jumptable_c_mw3_skew | 3.7ns | 134.6ns | 2.7% |  |
| mw_predicate_all_c_mw3_skew | 4.4ns | 264.4ns | 1.7% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_skew (n=6, range 107.5-176.1 ns)
    107.5 |####################
    110.9 |
    114.4 |
    117.8 |
    121.2 |
    124.6 |
    128.1 |
    131.5 |
    134.9 |
    138.3 |
    141.8 |########################################
    145.2 |
    148.6 |
    152.1 |####################
    155.5 |####################
    158.9 |
    162.3 |
    165.8 |
    169.2 |
    172.6 |
  (0 below, 1 above range)

mw_chain_c_mw3_skew (n=6, range 106.7-177.7 ns)
    106.7 |########################################
    110.2 |
    113.8 |
    117.3 |
    120.9 |
    124.5 |
    128.0 |
    131.6 |
    135.1 |
    138.7 |####################
    142.2 |
    145.8 |
    149.3 |
    152.8 |####################
    156.4 |####################
    159.9 |
    163.5 |
    167.0 |
    170.6 |
    174.1 |
  (0 below, 1 above range)

mw_chain_rev_c_mw3_skew (n=6, range 110.8-181.2 ns)
    110.8 |########################################
    114.3 |
    117.8 |
    121.4 |########################################
    124.9 |
    128.4 |
    131.9 |
    135.5 |
    139.0 |
    142.5 |
    146.0 |
    149.5 |
    153.1 |########################################
    156.6 |########################################
    160.1 |
    163.6 |
    167.2 |########################################
    170.7 |
    174.2 |
    177.7 |
  (0 below, 1 above range)

mw_jumptable_c_mw3_skew (n=6, range 106.2-154.8 ns)
    106.2 |########################################
    108.6 |
    111.1 |
    113.5 |########################################
    115.9 |
    118.4 |
    120.8 |
    123.2 |
    125.6 |
    128.1 |
    130.5 |
    132.9 |########################################
    135.4 |
    137.8 |
    140.2 |
    142.7 |########################################
    145.1 |
    147.5 |
    149.9 |
    152.4 |########################################
  (0 below, 1 above range)

mw_predicate_all_c_mw3_skew (n=6, range 213.7-300.0 ns)
    213.7 |########################################
    218.0 |########################################
    222.3 |
    226.6 |
    231.0 |
    235.3 |
    239.6 |
    243.9 |
    248.2 |########################################
    252.5 |
    256.9 |
    261.2 |
    265.5 |
    269.8 |
    274.1 |
    278.4 |
    282.7 |
    287.1 |
    291.4 |
    295.7 |
  (0 below, 3 above range)

```

## Diagnostics

- **mw_chain_c_mw3_skew**: CV=21.7% (high variance, measurements may be unstable)
