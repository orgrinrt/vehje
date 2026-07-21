# Per-type strategy (NATIVE tier): all ifchain

5 variants, 6 samples per variant.
Baseline: **an_ifchain_table**

## Highlights

Baseline for all deltas below: **an_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_ifchain_prof dominates: 13% faster than the next best (an_ifchain_seq)

an_ifchain_prof (1.76 us) leads an_ifchain_seq (1.97 us) by 13%, a clear separation rather than a photo finish. CV 16.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### an_ifchain_pred is an outlier: 2.4x slower than the field

an_ifchain_pred (4.17 us) is 2.4x the fastest (1.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### an_ifchain_prof is fastest but the noisiest (CV 16.9%)

an_ifchain_prof wins on median (1.76 us) yet has the highest variance (CV 16.9%), while an_ifchain_pred is the steadiest (CV 7.0%, 4.17 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {an_ifchain_prof, an_ifchain_seq, an_ifchain_tree, an_ifchain_table} vs {an_ifchain_pred} (97% apart)

The field splits into a fast tier {an_ifchain_prof, an_ifchain_seq, an_ifchain_tree, an_ifchain_table} and a slow tier {an_ifchain_pred} with a 97% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_ifchain_tree's edge over baseline is significant but tiny (5 ns, 0.22%)

an_ifchain_tree differs from baseline an_ifchain_table by 5 ns (0.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_ifchain_prof** at 1755.2 ns median (-16.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.37x (fastest 1755.2 ns, slowest 4167.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_ifchain_pred | 6452ns | 6741ns | 5785ns | 6433ns | 6814ns | +42.82% |
| an_ifchain_prof | 4255ns | 4025ns | 3745ns | 3942ns | 4979ns | -5.82% |
| an_ifchain_seq | 4321ns | 4529ns | 3793ns | 4336ns | 4562ns | -4.36% |
| an_ifchain_table | 4518ns | 4606ns | 4059ns | 4437ns | 4868ns | base |
| an_ifchain_tree | 4650ns | 4600ns | 3970ns | 4440ns | 5304ns | +2.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_ifchain_pred | 3980ns | 3525ns | 4200ns | +91.61% | 0.064 |
| an_ifchain_prof | 1876ns | 1638ns | 2233ns | -9.67% | 0.136 |
| an_ifchain_seq | 1884ns | 1651ns | 1989ns | -9.31% | 0.136 |
| an_ifchain_table | 2077ns | 1880ns | 2231ns | base | 0.123 |
| an_ifchain_tree | 2144ns | 1820ns | 2463ns | +3.21% | 0.119 |

## Performance model

- Peak throughput: **0.156 Gops/s** (an_ifchain_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_ifchain_pred | 0.061 | 39.3% |
| an_ifchain_prof | 0.146 | 93.3% |
| an_ifchain_seq | 0.130 | 83.0% |
| an_ifchain_table | 0.121 | 77.6% |
| an_ifchain_tree | 0.121 | 77.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_ifchain_pred | 6452ns | 6452ns | +42.82% |
| an_ifchain_prof | 4255ns | 4255ns | -5.82% |
| an_ifchain_seq | 4321ns | 4321ns | -4.36% |
| an_ifchain_table | 4518ns | 4518ns | base |
| an_ifchain_tree | 4650ns | 4650ns | +2.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_ifchain_table | 2112ns | base | --- | [1888, 2231] | --- | --- | --- | --- |
| an_ifchain_pred | 4168ns | +1936.7ns (+91.7%) | [+1684, +2088]ns | [3571, 4200] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_ifchain_prof | 1755ns | no significant difference | [-403, +42]ns | [1640, 2233] | no | 0.2917 | 0.2188 | 0 |
| an_ifchain_seq | 1975ns | -202.1ns (-9.6%) | [-264, -114]ns | [1687, 1989] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_ifchain_tree | 2112ns | no significant difference | [-149, +345]ns | [1856, 2463] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_ifchain_table | an_ifchain_pred | an_ifchain_prof | an_ifchain_seq | an_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 2039ns | +107.1% | -19.5% | -3.5% | +7.0% |
| 2 | 2185ns | +91.1% | +13.0% | -8.8% | -0.1% |
| 3 | 2198ns | +89.8% | -9.1% | -9.6% | +24.8% |
| 4 | 1895ns | +86.0% | -12.8% | -12.9% | -3.9% |
| 5 | 1880ns | +92.4% | -12.9% | -8.3% | +0.6% |
| 6 | 2265ns | +83.8% | -18.0% | -12.5% | -9.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_ifchain_pred | 0.189 | ok |
| an_ifchain_prof | -0.070 | ok |
| an_ifchain_seq | 0.151 | ok |
| an_ifchain_table | -0.105 | ok |
| an_ifchain_tree | -0.113 | ok |

**Consistency summary:**

- **an_ifchain_pred**: won 0/6, lost 6/6
- **an_ifchain_prof**: won 5/6, lost 1/6
- **an_ifchain_seq**: won 6/6, lost 0/6
- **an_ifchain_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_ifchain_pred | 5.4ns | 3979.7ns | 0.1% |  |
| an_ifchain_prof | 5.5ns | 1876.0ns | 0.3% |  |
| an_ifchain_seq | 5.9ns | 1883.7ns | 0.3% |  |
| an_ifchain_table | 6.0ns | 2077.0ns | 0.3% |  |
| an_ifchain_tree | 6.1ns | 2143.6ns | 0.3% |  |

## Distribution (algo ns)

```
an_ifchain_pred (n=6, range 3525.0-4199.8 ns)
   3525.0 |####################
   3558.7 |
   3592.5 |####################
   3626.2 |
   3660.0 |
   3693.7 |
   3727.4 |
   3761.2 |
   3794.9 |
   3828.7 |
   3862.4 |
   3896.1 |
   3929.9 |
   3963.6 |
   3997.4 |
   4031.1 |
   4064.8 |
   4098.6 |
   4132.3 |####################
   4166.1 |########################################
  (0 below, 1 above range)

an_ifchain_prof (n=6, range 1638.3-2232.9 ns)
   1638.3 |########################################
   1668.0 |
   1697.8 |
   1727.5 |
   1757.2 |
   1787.0 |
   1816.7 |
   1846.4 |#############
   1876.2 |
   1905.9 |
   1935.6 |
   1965.4 |
   1995.1 |#############
   2024.8 |
   2054.6 |
   2084.3 |
   2114.0 |
   2143.8 |
   2173.5 |
   2203.2 |
  (0 below, 1 above range)

an_ifchain_seq (n=6, range 1651.2-1989.2 ns)
   1651.2 |####################
   1668.1 |
   1685.0 |
   1701.9 |
   1718.8 |####################
   1735.7 |
   1752.6 |
   1769.5 |
   1786.4 |
   1803.3 |
   1820.2 |
   1837.1 |
   1854.0 |
   1870.9 |
   1887.8 |
   1904.7 |
   1921.6 |
   1938.5 |
   1955.4 |####################
   1972.3 |########################################
  (0 below, 1 above range)

an_ifchain_table (n=6, range 1880.0-2231.2 ns)
   1880.0 |########################################
   1897.6 |
   1915.1 |
   1932.7 |
   1950.2 |
   1967.8 |
   1985.4 |
   2002.9 |
   2020.5 |
   2038.1 |####################
   2055.6 |
   2073.2 |
   2090.8 |
   2108.3 |
   2125.9 |
   2143.4 |
   2161.0 |
   2178.6 |####################
   2196.1 |####################
   2213.7 |
  (0 below, 1 above range)

an_ifchain_tree (n=6, range 1820.4-2463.3 ns)
   1820.4 |####################
   1852.5 |
   1884.7 |####################
   1916.8 |
   1949.0 |
   1981.1 |
   2013.3 |####################
   2045.4 |
   2077.6 |
   2109.7 |
   2141.9 |
   2174.0 |########################################
   2206.1 |
   2238.3 |
   2270.4 |
   2302.6 |
   2334.7 |
   2366.9 |
   2399.0 |
   2431.2 |
  (0 below, 1 above range)

```
