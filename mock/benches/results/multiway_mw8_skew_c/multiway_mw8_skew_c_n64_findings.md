# Multiway branch strategies, cheap-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_skew**

## Key findings

- **Fastest: mw_chain_c_mw8_skew** at 130.4 ns median (-40.0% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.64x (fastest 130.4 ns, slowest 605.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 3096ns | 3146ns | 2332ns | 2901ns | 3770ns | base |
| mw_chain_c_mw8_skew | 2865ns | 2930ns | 2264ns | 2712ns | 3396ns | -7.44% |
| mw_chain_rev_c_mw8_skew | 3096ns | 3296ns | 2302ns | 2996ns | 3644ns | +0.01% |
| mw_jumptable_c_mw8_skew | 3294ns | 2970ns | 2285ns | 2754ns | 4609ns | +6.42% |
| mw_predicate_all_c_mw8_skew | 3399ns | 3294ns | 2643ns | 3193ns | 4086ns | +9.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 211ns | 155ns | 255ns | base | 0.304 |
| mw_chain_c_mw8_skew | 129ns | 101ns | 155ns | -38.83% | 0.497 |
| mw_chain_rev_c_mw8_skew | 176ns | 132ns | 211ns | -16.43% | 0.364 |
| mw_jumptable_c_mw8_skew | 173ns | 119ns | 242ns | -17.78% | 0.370 |
| mw_predicate_all_c_mw8_skew | 611ns | 483ns | 712ns | +190.38% | 0.105 |

## Performance model

- Peak throughput: **0.635 Gops/s** (mw_chain_c_mw8_skew; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.294 | 46.3% |
| mw_chain_c_mw8_skew | 0.491 | 77.3% |
| mw_chain_rev_c_mw8_skew | 0.355 | 55.9% |
| mw_jumptable_c_mw8_skew | 0.407 | 64.2% |
| mw_predicate_all_c_mw8_skew | 0.106 | 16.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_skew | 3096ns | 3096ns | base |
| mw_chain_c_mw8_skew | 2865ns | 2865ns | -7.44% |
| mw_chain_rev_c_mw8_skew | 3096ns | 3096ns | +0.01% |
| mw_jumptable_c_mw8_skew | 3294ns | 3294ns | +6.42% |
| mw_predicate_all_c_mw8_skew | 3399ns | 3399ns | +9.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 218ns | base | --- | [160, 255] | --- | --- | --- | --- |
| mw_chain_c_mw8_skew | 130ns | -70.2ns (-32.3%) | [-117, -58]ns | [101, 155] | YES | 0.0417 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_skew | 180ns | -34.4ns (-15.8%) | [-55, -15]ns | [136, 211] | YES | 0.0417 | 0.0313 | 0 |
| mw_jumptable_c_mw8_skew | 157ns | no significant difference | [-74, +1]ns | [120, 242] | no | 0.2188 | 0.2188 | 0 |
| mw_predicate_all_c_mw8_skew | 606ns | +408.8ns (+187.9%) | [+322, +472]ns | [516, 712] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_skew | mw_chain_c_mw8_skew | mw_chain_rev_c_mw8_skew | mw_jumptable_c_mw8_skew | mw_predicate_all_c_mw8_skew |
|---|---|---|---|---|---|
| 1 | 255ns | -48.9% | -17.5% | +14.7% | +127.8% |
| 2 | 254ns | -43.3% | -25.4% | -33.1% | +171.6% |
| 3 | 155ns | -34.8% | -9.2% | -22.9% | +255.2% |
| 4 | 165ns | -38.5% | -19.7% | -26.1% | +193.6% |
| 5 | 208ns | -37.2% | -17.5% | -30.5% | +203.8% |
| 6 | 228ns | -27.1% | -6.8% | -16.0% | +222.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.229 | moderate+ |
| mw_chain_c_mw8_skew | 0.121 | ok |
| mw_chain_rev_c_mw8_skew | 0.269 | moderate+ |
| mw_jumptable_c_mw8_skew | 0.168 | ok |
| mw_predicate_all_c_mw8_skew | 0.013 | ok |

**Consistency summary:**

- **mw_chain_c_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_skew**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_skew**: won 5/6, lost 1/6
- **mw_predicate_all_c_mw8_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 3.8ns | 210.6ns | 1.8% |  |
| mw_chain_c_mw8_skew | 4.4ns | 128.8ns | 3.4% |  |
| mw_chain_rev_c_mw8_skew | 3.1ns | 176.0ns | 1.8% |  |
| mw_jumptable_c_mw8_skew | 5.1ns | 173.1ns | 2.9% |  |
| mw_predicate_all_c_mw8_skew | 4.5ns | 611.4ns | 0.7% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_skew (n=6, range 154.6-254.6 ns)
    154.6 |########################################
    159.6 |
    164.6 |########################################
    169.6 |
    174.6 |
    179.6 |
    184.6 |
    189.6 |
    194.6 |
    199.6 |
    204.6 |########################################
    209.6 |
    214.6 |
    219.6 |
    224.6 |########################################
    229.6 |
    234.6 |
    239.6 |
    244.6 |
    249.6 |########################################
  (0 below, 1 above range)

mw_chain_c_mw8_skew (n=6, range 100.8-155.0 ns)
    100.8 |########################################
    103.5 |
    106.2 |
    108.9 |
    111.6 |
    114.3 |
    117.1 |
    119.8 |
    122.5 |
    125.2 |
    127.9 |########################################
    130.6 |
    133.3 |
    136.0 |
    138.7 |
    141.4 |
    144.2 |####################
    146.9 |
    149.6 |
    152.3 |
  (0 below, 1 above range)

mw_chain_rev_c_mw8_skew (n=6, range 132.1-211.2 ns)
    132.1 |########################################
    136.1 |
    140.0 |########################################
    144.0 |
    147.9 |
    151.9 |
    155.8 |
    159.8 |
    163.8 |
    167.7 |########################################
    171.7 |
    175.6 |
    179.6 |
    183.5 |
    187.5 |########################################
    191.5 |
    195.4 |
    199.4 |
    203.3 |
    207.3 |########################################
  (0 below, 1 above range)

mw_jumptable_c_mw8_skew (n=6, range 119.2-241.8 ns)
    119.2 |########################################
    125.3 |
    131.5 |
    137.6 |
    143.7 |####################
    149.9 |
    156.0 |
    162.1 |
    168.3 |####################
    174.4 |
    180.5 |
    186.7 |####################
    192.8 |
    198.9 |
    205.1 |
    211.2 |
    217.3 |
    223.5 |
    229.6 |
    235.7 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_skew (n=6, range 483.3-712.5 ns)
    483.3 |########################################
    494.8 |
    506.2 |
    517.7 |
    529.1 |
    540.6 |########################################
    552.1 |
    563.5 |
    575.0 |########################################
    586.4 |
    597.9 |
    609.4 |
    620.8 |########################################
    632.3 |
    643.7 |
    655.2 |
    666.7 |
    678.1 |
    689.6 |########################################
    701.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_jumptable_c_mw8_skew**: CV=34.2% (high variance, measurements may be unstable)
