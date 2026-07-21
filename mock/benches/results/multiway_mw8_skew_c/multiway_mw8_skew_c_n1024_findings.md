# Multiway branch strategies, cheap-arm, mw8_skew: 8-way, skewed to arm 0 (~70%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw8_skew**

## Key findings

- **Fastest: mw_chain_c_mw8_skew** at 1604.8 ns median (-36.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.12x (fastest 1604.8 ns, slowest 8223.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 5002ns | 5140ns | 4274ns | 4901ns | 5517ns | base |
| mw_chain_c_mw8_skew | 4115ns | 4310ns | 3491ns | 4079ns | 4483ns | -17.72% |
| mw_chain_rev_c_mw8_skew | 4653ns | 5004ns | 3865ns | 4672ns | 5018ns | -6.98% |
| mw_jumptable_c_mw8_skew | 4411ns | 4543ns | 3810ns | 4390ns | 4744ns | -11.81% |
| mw_predicate_all_c_mw8_skew | 10815ns | 10683ns | 9358ns | 10296ns | 12323ns | +116.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 2448ns | 2096ns | 2701ns | base | 0.418 |
| mw_chain_c_mw8_skew | 1535ns | 1291ns | 1674ns | -37.32% | 0.667 |
| mw_chain_rev_c_mw8_skew | 2080ns | 1706ns | 2307ns | -15.03% | 0.492 |
| mw_jumptable_c_mw8_skew | 1798ns | 1549ns | 1933ns | -26.59% | 0.570 |
| mw_predicate_all_c_mw8_skew | 8342ns | 7229ns | 9517ns | +240.71% | 0.123 |

## Performance model

- Peak throughput: **0.793 Gops/s** (mw_chain_c_mw8_skew; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw8_skew | 0.408 | 51.4% |
| mw_chain_c_mw8_skew | 0.638 | 80.5% |
| mw_chain_rev_c_mw8_skew | 0.465 | 58.6% |
| mw_jumptable_c_mw8_skew | 0.553 | 69.7% |
| mw_predicate_all_c_mw8_skew | 0.125 | 15.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw8_skew | 5002ns | 5002ns | base |
| mw_chain_c_mw8_skew | 4115ns | 4115ns | -17.72% |
| mw_chain_rev_c_mw8_skew | 4653ns | 4653ns | -6.98% |
| mw_jumptable_c_mw8_skew | 4411ns | 4411ns | -11.81% |
| mw_predicate_all_c_mw8_skew | 10815ns | 10815ns | +116.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 2513ns | base | --- | [2131, 2701] | --- | --- | --- | --- |
| mw_chain_c_mw8_skew | 1605ns | -889.6ns (-35.4%) | [-1055, -797]ns | [1326, 1674] | YES | 0.0417 | 0.0313 | 0 |
| mw_chain_rev_c_mw8_skew | 2203ns | -479.0ns (-19.1%) | [-615, -10]ns | [1731, 2307] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| mw_jumptable_c_mw8_skew | 1853ns | -659.8ns (-26.3%) | [-768, -525]ns | [1606, 1933] | YES | 0.0417 | 0.0313 | 0 |
| mw_predicate_all_c_mw8_skew | 8223ns | +5907.0ns (+235.1%) | [+4959, +6815]ns | [7287, 9517] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw8_skew | mw_chain_c_mw8_skew | mw_chain_rev_c_mw8_skew | mw_jumptable_c_mw8_skew | mw_predicate_all_c_mw8_skew |
|---|---|---|---|---|---|
| 1 | 2489ns | -36.3% | -29.4% | -28.4% | +190.4% |
| 2 | 2536ns | -33.8% | -13.1% | -24.2% | +254.0% |
| 3 | 2700ns | -39.8% | -18.4% | -28.4% | +251.7% |
| 4 | 2167ns | -40.4% | -21.3% | -23.2% | +239.0% |
| 5 | 2096ns | -35.1% | +14.9% | -26.1% | +256.2% |
| 6 | 2703ns | -38.3% | -18.4% | -28.5% | +252.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw8_skew | -0.104 | ok |
| mw_chain_c_mw8_skew | 0.125 | ok |
| mw_chain_rev_c_mw8_skew | -0.383 | moderate- |
| mw_jumptable_c_mw8_skew | -0.024 | ok |
| mw_predicate_all_c_mw8_skew | -0.211 | moderate- |

**Consistency summary:**

- **mw_chain_c_mw8_skew**: won 6/6, lost 0/6
- **mw_chain_rev_c_mw8_skew**: won 5/6, lost 1/6
- **mw_jumptable_c_mw8_skew**: won 6/6, lost 0/6
- **mw_predicate_all_c_mw8_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw8_skew | 3.4ns | 2448.5ns | 0.1% |  |
| mw_chain_c_mw8_skew | 3.3ns | 1534.8ns | 0.2% |  |
| mw_chain_rev_c_mw8_skew | 4.1ns | 2080.4ns | 0.2% |  |
| mw_jumptable_c_mw8_skew | 4.0ns | 1797.5ns | 0.2% |  |
| mw_predicate_all_c_mw8_skew | 4.1ns | 8342.2ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw8_skew (n=6, range 2096.2-2701.2 ns)
   2096.2 |########################################
   2126.5 |
   2156.7 |########################################
   2187.0 |
   2217.2 |
   2247.5 |
   2277.7 |
   2308.0 |
   2338.2 |
   2368.5 |
   2398.7 |
   2429.0 |
   2459.2 |########################################
   2489.5 |
   2519.7 |########################################
   2550.0 |
   2580.2 |
   2610.5 |
   2640.7 |
   2671.0 |########################################
  (0 below, 1 above range)

mw_chain_c_mw8_skew (n=6, range 1291.2-1673.5 ns)
   1291.2 |########################################
   1310.3 |
   1329.4 |
   1348.5 |########################################
   1367.7 |
   1386.8 |
   1405.9 |
   1425.0 |
   1444.1 |
   1463.2 |
   1482.3 |
   1501.5 |
   1520.6 |
   1539.7 |
   1558.8 |
   1577.9 |########################################
   1597.0 |
   1616.2 |########################################
   1635.3 |
   1654.4 |########################################
  (0 below, 1 above range)

mw_chain_rev_c_mw8_skew (n=6, range 1705.8-2306.7 ns)
   1705.8 |#############
   1735.8 |#############
   1765.9 |
   1795.9 |
   1826.0 |
   1856.0 |
   1886.1 |
   1916.1 |
   1946.1 |
   1976.2 |
   2006.2 |
   2036.3 |
   2066.3 |
   2096.4 |
   2126.4 |
   2156.4 |
   2186.5 |########################################
   2216.5 |
   2246.6 |
   2276.6 |
  (0 below, 1 above range)

mw_jumptable_c_mw8_skew (n=6, range 1549.2-1933.3 ns)
   1549.2 |####################
   1568.4 |
   1587.6 |
   1606.8 |
   1626.0 |
   1645.2 |####################
   1664.4 |
   1683.7 |
   1702.9 |
   1722.1 |
   1741.3 |
   1760.5 |
   1779.7 |####################
   1798.9 |
   1818.1 |
   1837.3 |
   1856.5 |
   1875.7 |
   1894.9 |
   1914.1 |########################################
  (0 below, 1 above range)

mw_predicate_all_c_mw8_skew (n=6, range 7229.2-9516.6 ns)
   7229.2 |########################################
   7343.6 |########################################
   7457.9 |########################################
   7572.3 |
   7686.7 |
   7801.1 |
   7915.4 |
   8029.8 |
   8144.2 |
   8258.6 |
   8372.9 |
   8487.3 |
   8601.7 |
   8716.0 |
   8830.4 |
   8944.8 |########################################
   9059.2 |
   9173.5 |
   9287.9 |
   9402.3 |########################################
  (0 below, 1 above range)

```
