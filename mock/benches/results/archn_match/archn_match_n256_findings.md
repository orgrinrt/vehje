# Per-type strategy (NATIVE tier): all match

5 variants, 6 samples per variant.
Baseline: **an_match_table**

## Highlights

Baseline for all deltas below: **an_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Two tiers: {an_match_tree, an_match_seq, an_match_table, an_match_prof} vs {an_match_pred} (66% apart)

The field splits into a fast tier {an_match_tree, an_match_seq, an_match_table, an_match_prof} and a slow tier {an_match_pred} with a 66% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_match_prof's edge over baseline is significant but tiny (19 ns, 0.83%)

an_match_prof differs from baseline an_match_table by 19 ns (0.83%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_match_tree** at 2190.7 ns median (-4.5% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.80x (fastest 2190.7 ns, slowest 3938.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_match_pred | 6617ns | 6501ns | 6107ns | 6499ns | 7049ns | +36.38% |
| an_match_prof | 4937ns | 5114ns | 4029ns | 4891ns | 5460ns | +1.75% |
| an_match_seq | 4749ns | 4978ns | 4022ns | 4694ns | 5195ns | -2.11% |
| an_match_table | 4852ns | 5005ns | 3998ns | 4819ns | 5329ns | base |
| an_match_tree | 4710ns | 4760ns | 3948ns | 4671ns | 5149ns | -2.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_match_pred | 4004ns | 3688ns | 4267ns | +80.57% | 0.064 |
| an_match_prof | 2269ns | 1850ns | 2493ns | +2.33% | 0.113 |
| an_match_seq | 2185ns | 1842ns | 2389ns | -1.45% | 0.117 |
| an_match_table | 2218ns | 1828ns | 2427ns | base | 0.115 |
| an_match_tree | 2163ns | 1807ns | 2372ns | -2.46% | 0.118 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_match_tree; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_match_pred | 0.065 | 45.9% |
| an_match_prof | 0.108 | 76.3% |
| an_match_seq | 0.112 | 78.8% |
| an_match_table | 0.112 | 78.8% |
| an_match_tree | 0.117 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_match_pred | 6617ns | 6617ns | +36.38% |
| an_match_prof | 4937ns | 4937ns | +1.75% |
| an_match_seq | 4749ns | 4749ns | -2.11% |
| an_match_table | 4852ns | 4852ns | base |
| an_match_tree | 4710ns | 4710ns | -2.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_match_table | 2293ns | base | --- | [1933, 2427] | --- | --- | --- | --- |
| an_match_pred | 3939ns | +1823.3ns (+79.5%) | [+1635, +1902]ns | [3807, 4267] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_match_prof | 2366ns | +19.0ns (+0.8%) | [+6, +129]ns | [1949, 2493] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_match_seq | 2292ns | no significant difference | [-112, +14]ns | [1876, 2389] | no | 1.0000 | 1.0000 | 0 |
| an_match_tree | 2191ns | no significant difference | [-149, +8]ns | [1926, 2372] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_match_table | an_match_pred | an_match_prof | an_match_seq | an_match_tree |
|---|---|---|---|---|---|
| 1 | 1828ns | +101.7% | +1.2% | +0.8% | -1.2% |
| 2 | 2378ns | +65.1% | +0.2% | +0.6% | -7.8% |
| 3 | 2372ns | +80.1% | +0.6% | +0.6% | +0.4% |
| 4 | 2475ns | +72.2% | +4.9% | -3.8% | -4.5% |
| 5 | 2214ns | +77.8% | +6.2% | -0.5% | -1.1% |
| 6 | 2039ns | +93.4% | +0.4% | -6.4% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_match_pred | 0.235 | moderate+ |
| an_match_prof | 0.036 | ok |
| an_match_seq | 0.028 | ok |
| an_match_table | 0.006 | ok |
| an_match_tree | 0.182 | ok |

**Consistency summary:**

- **an_match_pred**: won 0/6, lost 6/6
- **an_match_prof**: won 0/6, lost 6/6
- **an_match_seq**: won 3/6, lost 3/6
- **an_match_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_match_pred | 6.2ns | 4004.4ns | 0.2% |  |
| an_match_prof | 6.2ns | 2269.2ns | 0.3% |  |
| an_match_seq | 5.9ns | 2185.4ns | 0.3% |  |
| an_match_table | 6.0ns | 2217.6ns | 0.3% |  |
| an_match_tree | 6.5ns | 2163.0ns | 0.3% |  |

## Distribution (algo ns)

```
an_match_pred (n=6, range 3687.5-4267.1 ns)
   3687.5 |#############
   3716.5 |
   3745.5 |
   3774.4 |
   3803.4 |
   3832.4 |
   3861.4 |
   3890.4 |
   3919.3 |########################################
   3948.3 |
   3977.3 |
   4006.3 |
   4035.3 |
   4064.2 |
   4093.2 |
   4122.2 |
   4151.2 |
   4180.2 |
   4209.1 |
   4238.1 |#############
  (0 below, 1 above range)

an_match_prof (n=6, range 1850.4-2492.7 ns)
   1850.4 |####################
   1882.5 |
   1914.6 |
   1946.7 |
   1978.9 |
   2011.0 |
   2043.1 |####################
   2075.2 |
   2107.3 |
   2139.4 |
   2171.6 |
   2203.7 |
   2235.8 |
   2267.9 |
   2300.0 |
   2332.1 |####################
   2364.2 |########################################
   2396.4 |
   2428.5 |
   2460.6 |
  (0 below, 1 above range)

an_match_seq (n=6, range 1842.5-2388.7 ns)
   1842.5 |####################
   1869.8 |
   1897.1 |####################
   1924.4 |
   1951.7 |
   1979.0 |
   2006.4 |
   2033.7 |
   2061.0 |
   2088.3 |
   2115.6 |
   2142.9 |
   2170.2 |
   2197.5 |####################
   2224.8 |
   2252.1 |
   2279.5 |
   2306.8 |
   2334.1 |
   2361.4 |########################################
  (0 below, 1 above range)

an_match_table (n=6, range 1827.9-2426.7 ns)
   1827.9 |####################
   1857.8 |
   1887.8 |
   1917.7 |
   1947.7 |
   1977.6 |
   2007.5 |
   2037.5 |####################
   2067.4 |
   2097.3 |
   2127.3 |
   2157.2 |
   2187.2 |####################
   2217.1 |
   2247.0 |
   2277.0 |
   2306.9 |
   2336.8 |
   2366.8 |########################################
   2396.7 |
  (0 below, 1 above range)

an_match_tree (n=6, range 1806.7-2372.2 ns)
   1806.7 |####################
   1835.0 |
   1863.3 |
   1891.5 |
   1919.8 |
   1948.1 |
   1976.4 |
   2004.6 |
   2032.9 |####################
   2061.2 |
   2089.5 |
   2117.8 |
   2146.0 |
   2174.3 |########################################
   2202.6 |
   2230.9 |
   2259.1 |
   2287.4 |
   2315.7 |
   2344.0 |####################
  (0 below, 1 above range)

```
