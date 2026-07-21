# Multiway branch strategies, cheap-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_skew**

## Key findings

- **Fastest: mw_chain_c_mw8_skew** at 420.9 ns median (-37.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.75x (fastest 420.9 ns, slowest 2419.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 3355ns | 3368ns | 2810ns | 3322ns | 3677ns | base |
| mw_chain_c_mw8_skew | 3180ns | 3172ns | 2489ns | 3067ns | 3694ns | -5.23% |
| mw_chain_rev_c_mw8_skew | 3237ns | 3375ns | 2612ns | 3141ns | 3694ns | -3.53% |
| mw_jumptable_c_mw8_skew | 3046ns | 3081ns | 2556ns | 2930ns | 3464ns | -9.22% |
| mw_predicate_all_c_mw8_skew | 5055ns | 5251ns | 3963ns | 4855ns | 5901ns | +50.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 673ns | 556ns | 739ns | base | 0.380 |
| mw_chain_c_mw8_skew | 421ns | 321ns | 488ns | -37.46% | 0.608 |
| mw_chain_rev_c_mw8_skew | 538ns | 438ns | 604ns | -20.09% | 0.476 |
| mw_jumptable_c_mw8_skew | 468ns | 393ns | 533ns | -30.49% | 0.547 |
| mw_predicate_all_c_mw8_skew | 2304ns | 1827ns | 2644ns | +242.29% | 0.111 |

## Performance model

- Peak throughput: **0.797 Gops/s** (mw_chain_c_mw8_skew; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.378 | 47.4% |
| mw_chain_c_mw8_skew | 0.608 | 76.3% |
| mw_chain_rev_c_mw8_skew | 0.455 | 57.0% |
| mw_jumptable_c_mw8_skew | 0.541 | 67.9% |
| mw_predicate_all_c_mw8_skew | 0.106 | 13.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_skew | 3355ns | 3355ns | base |
| mw_chain_c_mw8_skew | 3180ns | 3180ns | -5.23% |
| mw_chain_rev_c_mw8_skew | 3237ns | 3237ns | -3.53% |
| mw_jumptable_c_mw8_skew | 3046ns | 3046ns | -9.22% |
| mw_predicate_all_c_mw8_skew | 5055ns | 5055ns | +50.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 678ns | base | --- | [603, 739] | --- | --- | --- | --- |
| mw_chain_c_mw8_skew | 421ns | -250.8ns (-37.0%) | [-271, -235]ns | [354, 488] | YES | 0.0313 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_skew | 563ns | -131.9ns (-19.5%) | [-167, -107]ns | [446, 604] | YES | 0.0313 | 0.0313 | 0 |
| mw_jumptable_c_mw8_skew | 473ns | -205.1ns (-30.3%) | [-231, -179]ns | [398, 533] | YES | 0.0313 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_skew | 2419ns | +1741.4ns (+257.0%) | [+1246, +1905]ns | [1849, 2644] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_skew | mw_chain_c_mw8_skew | mw_chain_rev_c_mw8_skew | mw_jumptable_c_mw8_skew | mw_predicate_all_c_mw8_skew |
|---|---|---|---|---|---|
| 1 | 649ns | -36.2% | -14.8% | -32.5% | +181.4% |
| 2 | 707ns | -39.5% | -18.5% | -28.2% | +239.7% |
| 3 | 706ns | -35.6% | -18.8% | -27.7% | +260.5% |
| 4 | 556ns | -42.3% | -21.2% | -29.3% | +236.4% |
| 5 | 649ns | -40.4% | -30.0% | -38.1% | +275.6% |
| 6 | 771ns | -32.5% | -17.9% | -28.0% | +255.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_skew | -0.117 | ok |
| mw_chain_c_mw8_skew | -0.144 | ok |
| mw_chain_rev_c_mw8_skew | -0.043 | ok |
| mw_jumptable_c_mw8_skew | -0.161 | ok |
| mw_predicate_all_c_mw8_skew | -0.183 | ok |

**Consistency summary:**

- **mw_chain_c_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_skew**: won 6/6, lost 0/6
- **mw_jumptable_c_mw8_skew**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 5.7ns | 673.1ns | 0.8% |  |
| mw_chain_c_mw8_skew | 4.4ns | 421.0ns | 1.1% |  |
| mw_chain_rev_c_mw8_skew | 6.6ns | 537.9ns | 1.2% |  |
| mw_jumptable_c_mw8_skew | 5.6ns | 467.9ns | 1.2% |  |
| mw_predicate_all_c_mw8_skew | 5.3ns | 2304.0ns | 0.2% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_skew (n=6, range 556.2-739.0 ns)
    556.2 |####################
    565.3 |
    574.5 |
    583.6 |
    592.8 |
    601.9 |
    611.0 |
    620.2 |
    629.3 |
    638.4 |
    647.6 |########################################
    656.7 |
    665.9 |
    675.0 |
    684.1 |
    693.3 |
    702.4 |########################################
    711.5 |
    720.7 |
    729.8 |
  (0 below, 1 above range)

mw_chain_c_mw8_skew (n=6, range 321.2-487.9 ns)
    321.2 |########################################
    329.5 |
    337.9 |
    346.2 |
    354.5 |
    362.9 |
    371.2 |
    379.5 |########################################
    387.9 |
    396.2 |
    404.5 |
    412.9 |########################################
    421.2 |########################################
    429.6 |
    437.9 |
    446.2 |
    454.6 |########################################
    462.9 |
    471.2 |
    479.6 |
  (0 below, 1 above range)

mw_chain_rev_c_mw8_skew (n=6, range 438.3-604.3 ns)
    438.3 |####################
    446.6 |####################
    454.9 |
    463.2 |
    471.5 |
    479.8 |
    488.1 |
    496.4 |
    504.7 |
    513.0 |
    521.3 |
    529.6 |
    537.9 |
    546.2 |####################
    554.5 |
    562.8 |
    571.1 |########################################
    579.4 |
    587.7 |
    596.0 |
  (0 below, 1 above range)

mw_jumptable_c_mw8_skew (n=6, range 393.3-533.1 ns)
    393.3 |####################
    400.3 |####################
    407.3 |
    414.3 |
    421.3 |
    428.2 |
    435.2 |####################
    442.2 |
    449.2 |
    456.2 |
    463.2 |
    470.2 |
    477.2 |
    484.2 |
    491.2 |
    498.2 |
    505.1 |########################################
    512.1 |
    519.1 |
    526.1 |
  (0 below, 1 above range)

mw_predicate_all_c_mw8_skew (n=6, range 1826.7-2643.8 ns)
   1826.7 |####################
   1867.6 |####################
   1908.4 |
   1949.3 |
   1990.1 |
   2031.0 |
   2071.8 |
   2112.7 |
   2153.5 |
   2194.4 |
   2235.2 |
   2276.1 |
   2316.9 |
   2357.8 |
   2398.6 |########################################
   2439.5 |
   2480.3 |
   2521.2 |####################
   2562.0 |
   2602.9 |
  (0 below, 1 above range)

```
