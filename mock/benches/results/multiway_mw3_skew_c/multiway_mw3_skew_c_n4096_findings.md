# Multiway branch strategies, cheap-arm, mw3_skew: 3-way, skewed to arm 0 (~80%)

5 variants, 6 samples per variant.
Baseline: **mw_bintree_c_mw3_skew**

## Key findings

- **Fastest: mw_jumptable_c_mw3_skew** at 5999.8 ns median (-2.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.32x (fastest 5999.8 ns, slowest 13947.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 8954ns | 8759ns | 7344ns | 8306ns | 10731ns | base |
| mw_chain_c_mw3_skew | 8728ns | 8927ns | 7371ns | 8484ns | 9772ns | -2.52% |
| mw_chain_rev_c_mw3_skew | 8629ns | 8770ns | 7503ns | 8577ns | 9270ns | -3.63% |
| mw_jumptable_c_mw3_skew | 8600ns | 8502ns | 7340ns | 8411ns | 9512ns | -3.96% |
| mw_predicate_all_c_mw3_skew | 16046ns | 16549ns | 14107ns | 15832ns | 17338ns | +79.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 6274ns | 5165ns | 7460ns | base | 0.653 |
| mw_chain_c_mw3_skew | 6105ns | 5192ns | 6772ns | -2.71% | 0.671 |
| mw_chain_rev_c_mw3_skew | 6046ns | 5282ns | 6437ns | -3.64% | 0.677 |
| mw_jumptable_c_mw3_skew | 6062ns | 5171ns | 6700ns | -3.39% | 0.676 |
| mw_predicate_all_c_mw3_skew | 13532ns | 11860ns | 14630ns | +115.67% | 0.303 |

## Performance model

- Peak throughput: **0.793 Gops/s** (mw_bintree_c_mw3_skew; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.663 | 83.6% |
| mw_chain_c_mw3_skew | 0.654 | 82.4% |
| mw_chain_rev_c_mw3_skew | 0.664 | 83.7% |
| mw_jumptable_c_mw3_skew | 0.683 | 86.1% |
| mw_predicate_all_c_mw3_skew | 0.294 | 37.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mw_bintree_c_mw3_skew | 8954ns | 8954ns | base |
| mw_chain_c_mw3_skew | 8728ns | 8728ns | -2.52% |
| mw_chain_rev_c_mw3_skew | 8629ns | 8629ns | -3.63% |
| mw_jumptable_c_mw3_skew | 8600ns | 8600ns | -3.96% |
| mw_predicate_all_c_mw3_skew | 16046ns | 16046ns | +79.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 6174ns | base | --- | [5188, 7460] | --- | --- | --- | --- |
| mw_chain_c_mw3_skew | 6268ns | no significant difference | [-1008, +413]ns | [5274, 6772] | no | 0.4375 | 0.2188 | 0 |
| mw_chain_rev_c_mw3_skew | 6173ns | no significant difference | [-1225, +679]ns | [5528, 6437] | no | 1.0000 | 1.0000 | 0 |
| mw_jumptable_c_mw3_skew | 6000ns | no significant difference | [-1198, +556]ns | [5485, 6700] | no | 0.9167 | 0.6875 | 0 |
| mw_predicate_all_c_mw3_skew | 13948ns | +7903.5ns (+128.0%) | [+5325, +8544]ns | [12018, 14630] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | mw_bintree_c_mw3_skew | mw_chain_c_mw3_skew | mw_chain_rev_c_mw3_skew | mw_jumptable_c_mw3_skew | mw_predicate_all_c_mw3_skew |
|---|---|---|---|---|---|
| 1 | 8221ns | -24.6% | -24.9% | -29.1% | +44.3% |
| 2 | 6174ns | +10.8% | +6.4% | +8.5% | +129.3% |
| 3 | 5212ns | +2.7% | +18.4% | +11.3% | +166.6% |
| 4 | 5165ns | +0.5% | +2.3% | +0.1% | +135.8% |
| 5 | 6174ns | +2.6% | -6.5% | -0.1% | +126.8% |
| 6 | 6700ns | +0.0% | -5.9% | +0.0% | +125.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mw_bintree_c_mw3_skew | 0.182 | ok |
| mw_chain_c_mw3_skew | 0.056 | ok |
| mw_chain_rev_c_mw3_skew | 0.169 | ok |
| mw_jumptable_c_mw3_skew | -0.062 | ok |
| mw_predicate_all_c_mw3_skew | -0.154 | ok |

**Consistency summary:**

- **mw_chain_c_mw3_skew**: won 1/6, lost 4/6
- **mw_chain_rev_c_mw3_skew**: won 3/6, lost 3/6
- **mw_jumptable_c_mw3_skew**: won 1/6, lost 3/6
- **mw_predicate_all_c_mw3_skew**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mw_bintree_c_mw3_skew | 4.6ns | 6274.3ns | 0.1% |  |
| mw_chain_c_mw3_skew | 3.8ns | 6104.5ns | 0.1% |  |
| mw_chain_rev_c_mw3_skew | 3.8ns | 6045.9ns | 0.1% |  |
| mw_jumptable_c_mw3_skew | 5.2ns | 6061.7ns | 0.1% |  |
| mw_predicate_all_c_mw3_skew | 3.7ns | 13531.8ns | 0.0% |  |

## Distribution (algo ns)

```
mw_bintree_c_mw3_skew (n=6, range 5164.6-7460.4 ns)
   5164.6 |########################################
   5279.4 |
   5394.2 |
   5509.0 |
   5623.8 |
   5738.6 |
   5853.3 |
   5968.1 |
   6082.9 |########################################
   6197.7 |
   6312.5 |
   6427.3 |
   6542.1 |
   6656.9 |####################
   6771.7 |
   6886.4 |
   7001.2 |
   7116.0 |
   7230.8 |
   7345.6 |
  (0 below, 1 above range)

mw_chain_c_mw3_skew (n=6, range 5192.5-6772.5 ns)
   5192.5 |########################################
   5271.5 |
   5350.5 |########################################
   5429.5 |
   5508.5 |
   5587.5 |
   5666.5 |
   5745.5 |
   5824.5 |
   5903.5 |
   5982.5 |
   6061.5 |
   6140.5 |########################################
   6219.5 |
   6298.5 |########################################
   6377.5 |
   6456.5 |
   6535.5 |
   6614.5 |
   6693.5 |########################################
  (0 below, 1 above range)

mw_chain_rev_c_mw3_skew (n=6, range 5282.5-6436.9 ns)
   5282.5 |####################
   5340.2 |
   5397.9 |
   5455.7 |
   5513.4 |
   5571.1 |
   5628.8 |
   5686.5 |
   5744.3 |####################
   5802.0 |
   5859.7 |
   5917.4 |
   5975.1 |
   6032.9 |
   6090.6 |
   6148.3 |########################################
   6206.0 |
   6263.7 |####################
   6321.5 |
   6379.2 |
  (0 below, 1 above range)

mw_jumptable_c_mw3_skew (n=6, range 5170.8-6700.4 ns)
   5170.8 |####################
   5247.3 |
   5323.8 |
   5400.2 |
   5476.7 |
   5553.2 |
   5629.7 |
   5706.2 |
   5782.6 |########################################
   5859.1 |
   5935.6 |
   6012.1 |
   6088.6 |
   6165.0 |####################
   6241.5 |
   6318.0 |
   6394.5 |
   6471.0 |
   6547.4 |
   6623.9 |####################
  (0 below, 1 above range)

mw_predicate_all_c_mw3_skew (n=6, range 11859.6-14629.8 ns)
  11859.6 |########################################
  11998.1 |
  12136.6 |########################################
  12275.1 |
  12413.6 |
  12552.1 |
  12690.7 |
  12829.2 |
  12967.7 |
  13106.2 |
  13244.7 |
  13383.2 |
  13521.7 |
  13660.2 |
  13798.7 |########################################
  13937.2 |########################################
  14075.8 |########################################
  14214.3 |
  14352.8 |
  14491.3 |
  (0 below, 1 above range)

```
