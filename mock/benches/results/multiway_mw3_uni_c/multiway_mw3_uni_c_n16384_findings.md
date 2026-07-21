# Multiway branch strategies, cheap-arm, mw3_uni: 3-way, uniform key

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_uni**

## Key findings

- **Fastest: mw_chain_c_mw3_uni** at 29893.5 ns median (-3.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.78x (fastest 29893.5 ns, slowest 53234.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 32915ns | 33514ns | 29878ns | 33408ns | 33694ns | base |
| mw_chain_c_mw3_uni | 32262ns | 32295ns | 29341ns | 31556ns | 34781ns | -1.98% |
| mw_chain_rev_c_mw3_uni | 32310ns | 33164ns | 29620ns | 32289ns | 33686ns | -1.84% |
| mw_jumptable_c_mw3_uni | 34658ns | 33038ns | 29843ns | 32268ns | 40651ns | +5.30% |
| mw_predicate_all_c_mw3_uni | 55418ns | 55617ns | 50723ns | 54269ns | 59489ns | +68.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 30433ns | 27646ns | 31160ns | base | 0.538 |
| mw_chain_c_mw3_uni | 29873ns | 27134ns | 32212ns | -1.84% | 0.548 |
| mw_chain_rev_c_mw3_uni | 29854ns | 27357ns | 31139ns | -1.90% | 0.549 |
| mw_jumptable_c_mw3_uni | 32149ns | 27666ns | 37825ns | +5.64% | 0.510 |
| mw_predicate_all_c_mw3_uni | 53049ns | 48577ns | 56926ns | +74.31% | 0.309 |

## Performance model

- Peak throughput: **0.604 Gops/s** (mw_chain_c_mw3_uni; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_uni | 0.529 | 87.7% |
| mw_chain_c_mw3_uni | 0.548 | 90.8% |
| mw_chain_rev_c_mw3_uni | 0.535 | 88.6% |
| mw_jumptable_c_mw3_uni | 0.536 | 88.7% |
| mw_predicate_all_c_mw3_uni | 0.308 | 51.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_uni | 32915ns | 32915ns | base |
| mw_chain_c_mw3_uni | 32262ns | 32262ns | -1.98% |
| mw_chain_rev_c_mw3_uni | 32310ns | 32310ns | -1.84% |
| mw_jumptable_c_mw3_uni | 34658ns | 34658ns | +5.30% |
| mw_predicate_all_c_mw3_uni | 55418ns | 55418ns | +68.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 30944ns | base | --- | [29196, 31160] | --- | --- | --- | --- |
| mw_chain_c_mw3_uni | 29894ns | no significant difference | [-2981, +1077]ns | [27514, 32212] | no | 0.9167 | 0.6875 | 0 |
| mw_chain_rev_c_mw3_uni | 30638ns | no significant difference | [-1698, +135]ns | [27786, 31139] | no | 0.4375 | 0.2188 | 0 |
| mw_jumptable_c_mw3_uni | 30595ns | no significant difference | [-2122, +7166]ns | [28027, 37825] | no | 1.0000 | 1.0000 | 0 |
| mw_predicate_all_c_mw3_uni | 53234ns | +23780.8ns (+76.9%) | [+18016, +26049]ns | [48985, 56926] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_uni | mw_chain_c_mw3_uni | mw_chain_rev_c_mw3_uni | mw_jumptable_c_mw3_uni | mw_predicate_all_c_mw3_uni |
|---|---|---|---|---|---|
| 1 | 31060ns | -7.1% | -0.2% | +43.8% | +56.4% |
| 2 | 27646ns | +0.9% | -1.0% | +2.7% | +86.4% |
| 3 | 30880ns | -12.1% | -8.6% | -10.4% | +60.0% |
| 4 | 30745ns | +0.6% | +1.0% | +0.8% | +84.0% |
| 5 | 31009ns | +3.0% | -2.4% | -0.2% | +84.7% |
| 6 | 31261ns | +3.9% | -0.1% | -3.3% | +75.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_uni | -0.232 | moderate- |
| mw_chain_c_mw3_uni | 0.492 | moderate+ |
| mw_chain_rev_c_mw3_uni | 0.028 | ok |
| mw_jumptable_c_mw3_uni | -0.108 | ok |
| mw_predicate_all_c_mw3_uni | 0.319 | moderate+ |

**Consistency summary:**

- **mw_chain_c_mw3_uni**: won 2/6, lost 4/6
- **mw_chain_rev_c_mw3_uni**: won 4/6, lost 1/6
- **mw_jumptable_c_mw3_uni**: won 3/6, lost 3/6
- **mw_predicate_all_c_mw3_uni**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_uni | 3.6ns | 30433.4ns | 0.0% |  |
| mw_chain_c_mw3_uni | 4.2ns | 29873.3ns | 0.0% |  |
| mw_chain_rev_c_mw3_uni | 4.0ns | 29854.2ns | 0.0% |  |
| mw_jumptable_c_mw3_uni | 7.5ns | 32148.8ns | 0.0% |  |
| mw_predicate_all_c_mw3_uni | 3.2ns | 53048.6ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_uni (n=6, range 27645.8-31160.4 ns)
  27645.8 |####################
  27821.5 |
  27997.3 |
  28173.0 |
  28348.7 |
  28524.5 |
  28700.2 |
  28875.9 |
  29051.6 |
  29227.4 |
  29403.1 |
  29578.8 |
  29754.6 |
  29930.3 |
  30106.0 |
  30281.8 |
  30457.5 |
  30633.2 |####################
  30808.9 |####################
  30984.7 |########################################
  (0 below, 1 above range)

mw_chain_c_mw3_uni (n=6, range 27133.8-32212.3 ns)
  27133.8 |########################################
  27387.7 |
  27641.6 |########################################
  27895.6 |
  28149.5 |
  28403.4 |
  28657.3 |########################################
  28911.3 |
  29165.2 |
  29419.1 |
  29673.0 |
  29927.0 |
  30180.9 |
  30434.8 |
  30688.8 |
  30942.7 |########################################
  31196.6 |
  31450.5 |
  31704.5 |########################################
  31958.4 |
  (0 below, 1 above range)

mw_chain_rev_c_mw3_uni (n=6, range 27357.1-31138.5 ns)
  27357.1 |####################
  27546.2 |
  27735.2 |
  27924.3 |
  28113.4 |####################
  28302.5 |
  28491.5 |
  28680.6 |
  28869.7 |
  29058.8 |
  29247.8 |
  29436.9 |
  29626.0 |
  29815.0 |
  30004.1 |
  30193.2 |####################
  30382.3 |
  30571.3 |
  30760.4 |
  30949.5 |########################################
  (0 below, 1 above range)

mw_jumptable_c_mw3_uni (n=6, range 27665.8-37824.8 ns)
  27665.8 |####################
  28173.8 |####################
  28681.7 |
  29189.7 |
  29697.6 |
  30205.5 |####################
  30713.5 |########################################
  31221.5 |
  31729.4 |
  32237.3 |
  32745.3 |
  33253.2 |
  33761.2 |
  34269.2 |
  34777.1 |
  35285.1 |
  35793.0 |
  36301.0 |
  36808.9 |
  37316.9 |
  (0 below, 1 above range)

mw_predicate_all_c_mw3_uni (n=6, range 48577.1-56926.1 ns)
  48577.1 |########################################
  48994.5 |########################################
  49412.0 |
  49829.4 |
  50246.9 |
  50664.3 |
  51081.8 |
  51499.2 |########################################
  51916.7 |
  52334.1 |
  52751.6 |
  53169.0 |
  53586.5 |
  54003.9 |
  54421.4 |
  54838.8 |########################################
  55256.3 |
  55673.7 |
  56091.2 |
  56508.6 |########################################
  (0 below, 1 above range)

```
