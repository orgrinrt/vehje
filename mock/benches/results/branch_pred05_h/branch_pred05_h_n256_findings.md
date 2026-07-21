# Branch strategies, heavy-arm, pred05: ~5% taken, predictable (b<13)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_pred05**

## Key findings

- **Fastest: br_profiled_hot_h_pred05** at 1995.6 ns median (-1.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.46x (fastest 1995.6 ns, slowest 4913.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 4710ns | 4715ns | 4243ns | 4563ns | 5163ns | base |
| br_predicate_h_pred05 | 7893ns | 7507ns | 7000ns | 7348ns | 9157ns | +67.58% |
| br_profiled_hot_h_pred05 | 4501ns | 4604ns | 3863ns | 4502ns | 4819ns | -4.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_pred05 | 2012ns | 1808ns | 2201ns | base | 0.127 |
| br_predicate_h_pred05 | 5143ns | 4583ns | 5932ns | +155.65% | 0.050 |
| br_profiled_hot_h_pred05 | 1968ns | 1728ns | 2105ns | -2.17% | 0.130 |

## Performance model

- Peak throughput: **0.148 Gops/s** (br_profiled_hot_h_pred05; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_pred05 | 0.127 | 85.7% |
| br_predicate_h_pred05 | 0.052 | 35.2% |
| br_profiled_hot_h_pred05 | 0.128 | 86.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_pred05 | 4710ns | 4710ns | base |
| br_predicate_h_pred05 | 7893ns | 7893ns | +67.58% |
| br_profiled_hot_h_pred05 | 4501ns | 4501ns | -4.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_pred05 | 2015ns | base | --- | [1819, 2201] | --- | --- | --- | --- |
| br_predicate_h_pred05 | 4914ns | +3041.2ns (+150.9%) | [+2622, +3731]ns | [4584, 5932] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_pred05 | 1996ns | no significant difference | [-215, +90]ns | [1804, 2105] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_pred05 | br_predicate_h_pred05 | br_profiled_hot_h_pred05 |
|---|---|---|---|
| 1 | 1831ns | +150.3% | -5.6% |
| 2 | 2308ns | +183.6% | -14.2% |
| 3 | 2095ns | +118.9% | -4.0% |
| 4 | 2093ns | +154.1% | +3.7% |
| 5 | 1937ns | +154.3% | +5.3% |
| 6 | 1808ns | +171.1% | +4.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_pred05 | -0.073 | ok |
| br_predicate_h_pred05 | -0.605 | HIGH- (thermal bounce) |
| br_profiled_hot_h_pred05 | 0.126 | ok |

**Consistency summary:**

- **br_predicate_h_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_h_pred05**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_pred05 | 5.3ns | 2011.9ns | 0.3% |  |
| br_predicate_h_pred05 | 6.0ns | 5143.3ns | 0.1% |  |
| br_profiled_hot_h_pred05 | 4.4ns | 1968.2ns | 0.2% |  |

## Distribution (algo ns)

```
br_branch_h_pred05 (n=6, range 1807.9-2201.2 ns)
   1807.9 |####################
   1827.6 |####################
   1847.2 |
   1866.9 |
   1886.6 |
   1906.2 |
   1925.9 |####################
   1945.6 |
   1965.2 |
   1984.9 |
   2004.6 |
   2024.2 |
   2043.9 |
   2063.6 |
   2083.2 |########################################
   2102.9 |
   2122.6 |
   2142.2 |
   2161.9 |
   2181.6 |
  (0 below, 1 above range)

br_predicate_h_pred05 (n=6, range 4583.3-5931.9 ns)
   4583.3 |########################################
   4650.7 |
   4718.2 |
   4785.6 |
   4853.0 |####################
   4920.4 |####################
   4987.9 |
   5055.3 |
   5122.7 |
   5190.1 |
   5257.6 |####################
   5325.0 |
   5392.4 |
   5459.9 |
   5527.3 |
   5594.7 |
   5662.1 |
   5729.6 |
   5797.0 |
   5864.4 |
  (0 below, 1 above range)

br_profiled_hot_h_pred05 (n=6, range 1727.5-2105.4 ns)
   1727.5 |########################################
   1746.4 |
   1765.3 |
   1784.2 |
   1803.1 |
   1822.0 |
   1840.9 |
   1859.8 |
   1878.7 |########################################
   1897.6 |
   1916.4 |
   1935.3 |
   1954.2 |
   1973.1 |########################################
   1992.0 |########################################
   2010.9 |
   2029.8 |########################################
   2048.7 |
   2067.6 |
   2086.5 |
  (0 below, 1 above range)

```
