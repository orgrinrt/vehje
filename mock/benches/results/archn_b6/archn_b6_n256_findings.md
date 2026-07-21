# Per-branch strategy (NATIVE tier): archetype 6

5 variants, 6 samples per variant.
Baseline: **an_b6_table**

## Highlights

Baseline for all deltas below: **an_b6_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b6_tree, an_b6_table) are a dead heat (<1%)

an_b6_tree (2.11 us) and an_b6_table (2.12 us) differ by 0.27%, inside the noise, even though the wider field spreads 84.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {an_b6_tree, an_b6_table, an_b6_seq, an_b6_prof} vs {an_b6_pred} (69% apart)

The field splits into a fast tier {an_b6_tree, an_b6_table, an_b6_seq, an_b6_prof} and a slow tier {an_b6_pred} with a 69% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b6_tree vs stability leader an_b6_seq (+8% speed for 1.1x steadier)

an_b6_tree is fastest (2.11 us, CV 10.4%); an_b6_seq gives up 8.4% median for 1.1x lower variance (CV 9.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b6_tree's edge over baseline is significant but tiny (0 ns, 0.01%)

an_b6_tree differs from baseline an_b6_table by 0 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b6_tree** at 2112.5 ns median (-0.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.84x (fastest 2112.5 ns, slowest 3894.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b6_pred | 6325ns | 6429ns | 5472ns | 6113ns | 7069ns | +38.15% |
| an_b6_prof | 4803ns | 4875ns | 4017ns | 4612ns | 5482ns | +4.90% |
| an_b6_seq | 4710ns | 4878ns | 4089ns | 4652ns | 5107ns | +2.87% |
| an_b6_table | 4578ns | 4606ns | 3985ns | 4442ns | 5079ns | base |
| an_b6_tree | 4564ns | 4715ns | 3921ns | 4468ns | 5030ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b6_pred | 3831ns | 3308ns | 4288ns | +82.60% | 0.067 |
| an_b6_prof | 2242ns | 1890ns | 2516ns | +6.86% | 0.114 |
| an_b6_seq | 2210ns | 1925ns | 2408ns | +5.32% | 0.116 |
| an_b6_table | 2098ns | 1830ns | 2319ns | base | 0.122 |
| an_b6_tree | 2084ns | 1803ns | 2327ns | -0.69% | 0.123 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_b6_tree; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b6_pred | 0.066 | 46.3% |
| an_b6_prof | 0.111 | 78.2% |
| an_b6_seq | 0.112 | 78.8% |
| an_b6_table | 0.121 | 85.1% |
| an_b6_tree | 0.121 | 85.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b6_pred | 6325ns | 6325ns | +38.15% |
| an_b6_prof | 4803ns | 4803ns | +4.90% |
| an_b6_seq | 4710ns | 4710ns | +2.87% |
| an_b6_table | 4578ns | 4578ns | base |
| an_b6_tree | 4564ns | 4564ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b6_table | 2118ns | base | --- | [1857, 2319] | --- | --- | --- | --- |
| an_b6_pred | 3894ns | +1662.9ns (+78.5%) | [+1454, +2082]ns | [3311, 4288] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b6_prof | 2307ns | no significant difference | [-61, +357]ns | [1902, 2516] | no | 0.2917 | 0.2188 | 0 |
| an_b6_seq | 2290ns | +109.2ns (+5.2%) | [+32, +194]ns | [1931, 2408] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b6_tree | 2112ns | no significant difference | [-145, +101]ns | [1811, 2327] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b6_table | an_b6_pred | an_b6_prof | an_b6_seq | an_b6_tree |
|---|---|---|---|---|---|
| 1 | 2044ns | +109.4% | -7.5% | +12.0% | +6.8% |
| 2 | 2270ns | +69.5% | +10.2% | +1.0% | -10.0% |
| 3 | 2193ns | +79.7% | +5.0% | +5.1% | +2.9% |
| 4 | 1883ns | +75.6% | +1.7% | +2.2% | -3.4% |
| 5 | 1830ns | +81.1% | +26.3% | +5.9% | -1.5% |
| 6 | 2369ns | +81.4% | +6.8% | +6.0% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b6_pred | -0.019 | ok |
| an_b6_prof | -0.252 | moderate- |
| an_b6_seq | -0.065 | ok |
| an_b6_table | -0.122 | ok |
| an_b6_tree | -0.245 | moderate- |

**Consistency summary:**

- **an_b6_pred**: won 0/6, lost 6/6
- **an_b6_prof**: won 1/6, lost 5/6
- **an_b6_seq**: won 0/6, lost 6/6
- **an_b6_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b6_pred | 6.2ns | 3831.1ns | 0.2% |  |
| an_b6_prof | 6.3ns | 2242.0ns | 0.3% |  |
| an_b6_seq | 6.1ns | 2209.7ns | 0.3% |  |
| an_b6_table | 6.5ns | 2098.1ns | 0.3% |  |
| an_b6_tree | 6.5ns | 2083.6ns | 0.3% |  |

## Distribution (algo ns)

```
an_b6_pred (n=6, range 3307.5-4288.0 ns)
   3307.5 |########################################
   3356.5 |
   3405.5 |
   3454.6 |
   3503.6 |
   3552.6 |
   3601.6 |
   3650.7 |
   3699.7 |
   3748.7 |
   3797.7 |
   3846.7 |####################
   3895.8 |####################
   3944.8 |
   3993.8 |
   4042.8 |
   4091.9 |
   4140.9 |
   4189.9 |
   4238.9 |####################
  (0 below, 1 above range)

an_b6_prof (n=6, range 1889.6-2516.4 ns)
   1889.6 |########################################
   1920.9 |
   1952.3 |
   1983.6 |
   2015.0 |
   2046.3 |
   2077.7 |
   2109.0 |
   2140.3 |
   2171.7 |
   2203.0 |
   2234.4 |
   2265.7 |
   2297.1 |########################################
   2328.4 |
   2359.7 |
   2391.1 |
   2422.4 |
   2453.8 |
   2485.1 |####################
  (0 below, 1 above range)

an_b6_seq (n=6, range 1925.0-2407.8 ns)
   1925.0 |##########################
   1949.1 |
   1973.3 |
   1997.4 |
   2021.5 |
   2045.7 |
   2069.8 |
   2094.0 |
   2118.1 |
   2142.2 |
   2166.4 |
   2190.5 |
   2214.7 |
   2238.8 |
   2262.9 |
   2287.1 |########################################
   2311.2 |
   2335.3 |
   2359.5 |
   2383.6 |
  (0 below, 1 above range)

an_b6_table (n=6, range 1830.4-2319.2 ns)
   1830.4 |########################################
   1854.8 |
   1879.3 |########################################
   1903.7 |
   1928.2 |
   1952.6 |
   1977.0 |
   2001.5 |
   2025.9 |########################################
   2050.4 |
   2074.8 |
   2099.2 |
   2123.7 |
   2148.1 |
   2172.6 |########################################
   2197.0 |
   2221.4 |
   2245.9 |########################################
   2270.3 |
   2294.8 |
  (0 below, 1 above range)

an_b6_tree (n=6, range 1803.3-2326.8 ns)
   1803.3 |########################################
   1829.5 |
   1855.7 |
   1881.8 |
   1908.0 |
   1934.2 |
   1960.4 |
   1986.5 |
   2012.7 |
   2038.9 |####################
   2065.1 |
   2091.3 |
   2117.4 |
   2143.6 |
   2169.8 |####################
   2196.0 |
   2222.1 |
   2248.3 |####################
   2274.5 |
   2300.7 |
  (0 below, 1 above range)

```
