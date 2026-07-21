# Multiway branch strategies, cheap-arm, mw3_uni: 3-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_uni**

## Key findings

- **Fastest: mw_chain_rev_c_mw3_uni** at 2145.0 ns median (-1.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.75x (fastest 2145.0 ns, slowest 3746.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 4840ns | 4882ns | 3930ns | 4670ns | 5549ns | base |
| mw_chain_c_mw3_uni | 4898ns | 5096ns | 3922ns | 4758ns | 5595ns | +1.20% |
| mw_chain_rev_c_mw3_uni | 4759ns | 4848ns | 3909ns | 4546ns | 5504ns | -1.67% |
| mw_jumptable_c_mw3_uni | 4844ns | 5084ns | 3925ns | 4808ns | 5359ns | +0.10% |
| mw_predicate_all_c_mw3_uni | 6326ns | 6363ns | 5277ns | 6025ns | 7302ns | +30.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 2193ns | 1758ns | 2483ns | base | 0.467 |
| mw_chain_c_mw3_uni | 2182ns | 1756ns | 2483ns | -0.52% | 0.469 |
| mw_chain_rev_c_mw3_uni | 2106ns | 1725ns | 2437ns | -3.99% | 0.486 |
| mw_jumptable_c_mw3_uni | 2187ns | 1760ns | 2379ns | -0.28% | 0.468 |
| mw_predicate_all_c_mw3_uni | 3718ns | 3104ns | 4278ns | +69.54% | 0.275 |

## Performance model

- Peak throughput: **0.593 Gops/s** (mw_chain_rev_c_mw3_uni; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.469 | 79.1% |
| mw_chain_c_mw3_uni | 0.450 | 75.9% |
| mw_chain_rev_c_mw3_uni | 0.477 | 80.4% |
| mw_jumptable_c_mw3_uni | 0.451 | 76.0% |
| mw_predicate_all_c_mw3_uni | 0.273 | 46.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_uni | 4840ns | 4840ns | base |
| mw_chain_c_mw3_uni | 4898ns | 4898ns | +1.20% |
| mw_chain_rev_c_mw3_uni | 4759ns | 4759ns | -1.67% |
| mw_jumptable_c_mw3_uni | 4844ns | 4844ns | +0.10% |
| mw_predicate_all_c_mw3_uni | 6326ns | 6326ns | +30.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 2182ns | base | --- | [1914, 2483] | --- | --- | --- | --- |
| mw_chain_c_mw3_uni | 2274ns | no significant difference | [-159, +120]ns | [1788, 2483] | no | 0.6875 | 0.6875 | 0 |
| mw_chain_rev_c_mw3_uni | 2145ns | no significant difference | [-279, +63]ns | [1735, 2437] | no | 0.4375 | 0.2188 | 0 |
| mw_jumptable_c_mw3_uni | 2270ns | no significant difference | [-262, +240]ns | [1912, 2379] | no | 0.6875 | 0.6875 | 0 |
| mw_predicate_all_c_mw3_uni | 3746ns | +1507.1ns (+69.1%) | [+1215, +1852]ns | [3129, 4278] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_uni | mw_chain_c_mw3_uni | mw_chain_rev_c_mw3_uni | mw_jumptable_c_mw3_uni | mw_predicate_all_c_mw3_uni |
|---|---|---|---|---|---|
| 1 | 1758ns | +3.5% | -0.8% | +17.4% | +76.6% |
| 2 | 2070ns | -15.2% | -16.6% | -15.0% | +52.4% |
| 3 | 2267ns | +0.3% | -9.4% | +0.1% | +63.8% |
| 4 | 2486ns | -0.1% | -2.1% | -8.6% | +81.4% |
| 5 | 2480ns | +0.1% | -1.7% | +0.2% | +63.2% |
| 6 | 2097ns | +8.5% | +6.7% | +8.3% | +80.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.317 | moderate+ |
| mw_chain_c_mw3_uni | 0.512 | HIGH+ (drift/warm-up) |
| mw_chain_rev_c_mw3_uni | 0.570 | HIGH+ (drift/warm-up) |
| mw_jumptable_c_mw3_uni | 0.245 | moderate+ |
| mw_predicate_all_c_mw3_uni | 0.437 | moderate+ |

**Consistency summary:**

- **mw_chain_c_mw3_uni**: won 2/6, lost 4/6
- **mw_chain_rev_c_mw3_uni**: won 5/6, lost 1/6
- **mw_jumptable_c_mw3_uni**: won 2/6, lost 3/6
- **mw_predicate_all_c_mw3_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 3.3ns | 2193.0ns | 0.1% |  |
| mw_chain_c_mw3_uni | 3.1ns | 2181.6ns | 0.1% |  |
| mw_chain_rev_c_mw3_uni | 3.6ns | 2105.6ns | 0.2% |  |
| mw_jumptable_c_mw3_uni | 3.6ns | 2186.8ns | 0.2% |  |
| mw_predicate_all_c_mw3_uni | 2.4ns | 3718.0ns | 0.1% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_uni (n=6, range 1757.5-2483.3 ns)
   1757.5 |########################################
   1793.8 |
   1830.1 |
   1866.4 |
   1902.7 |
   1939.0 |
   1975.2 |
   2011.5 |
   2047.8 |########################################
   2084.1 |########################################
   2120.4 |
   2156.7 |
   2193.0 |
   2229.3 |
   2265.6 |########################################
   2301.9 |
   2338.1 |
   2374.4 |
   2410.7 |
   2447.0 |########################################
  (0 below, 1 above range)

mw_chain_c_mw3_uni (n=6, range 1756.2-2482.9 ns)
   1756.2 |####################
   1792.5 |####################
   1828.9 |
   1865.2 |
   1901.5 |
   1937.9 |
   1974.2 |
   2010.5 |
   2046.9 |
   2083.2 |
   2119.6 |
   2155.9 |
   2192.2 |
   2228.6 |
   2264.9 |########################################
   2301.2 |
   2337.6 |
   2373.9 |
   2410.2 |
   2446.6 |####################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_uni (n=6, range 1725.4-2436.9 ns)
   1725.4 |########################################
   1761.0 |
   1796.6 |
   1832.1 |
   1867.7 |
   1903.3 |
   1938.9 |
   1974.4 |
   2010.0 |
   2045.6 |####################
   2081.2 |
   2116.7 |
   2152.3 |
   2187.9 |
   2223.5 |####################
   2259.0 |
   2294.6 |
   2330.2 |
   2365.8 |
   2401.3 |####################
  (0 below, 1 above range)

mw_jumptable_c_mw3_uni (n=6, range 1760.4-2378.6 ns)
   1760.4 |#############
   1791.3 |
   1822.2 |
   1853.1 |
   1884.0 |
   1914.9 |
   1945.8 |
   1976.8 |
   2007.7 |
   2038.6 |#############
   2069.5 |
   2100.4 |
   2131.3 |
   2162.2 |
   2193.1 |
   2224.0 |
   2254.9 |########################################
   2285.8 |
   2316.7 |
   2347.6 |
  (0 below, 1 above range)

mw_predicate_all_c_mw3_uni (n=6, range 3104.2-4278.4 ns)
   3104.2 |########################################
   3162.9 |
   3221.6 |
   3280.3 |
   3339.0 |
   3397.7 |
   3456.4 |
   3515.2 |
   3573.9 |
   3632.6 |
   3691.3 |####################
   3750.0 |####################
   3808.7 |
   3867.4 |
   3926.1 |
   3984.8 |
   4043.5 |####################
   4102.2 |
   4160.9 |
   4219.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **mw_chain_c_mw3_uni**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **mw_chain_rev_c_mw3_uni**: autocorrelation=0.57 (measurement drift or warm-up artifact)
