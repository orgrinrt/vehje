# Per-branch strategy (NATIVE tier): archetype 3

5 variants, 6 samples per variant.
Baseline: **an_b3_table**

## Highlights

Baseline for all deltas below: **an_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (an_b3_table)

The baseline an_b3_table is the fastest (2.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b3_table, an_b3_prof, an_b3_tree, an_b3_seq} vs {an_b3_pred} (59% apart)

The field splits into a fast tier {an_b3_table, an_b3_prof, an_b3_tree, an_b3_seq} and a slow tier {an_b3_pred} with a 59% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_b3_prof's edge over baseline is significant but tiny (10 ns, 0.45%)

an_b3_prof differs from baseline an_b3_table by 10 ns (0.45%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b3_table) is the fastest** at 2152.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.62x (fastest 2152.1 ns, slowest 3484.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b3_pred | 5792ns | 5972ns | 5296ns | 5752ns | 6100ns | +27.00% |
| an_b3_prof | 4557ns | 4735ns | 3943ns | 4484ns | 4974ns | -0.07% |
| an_b3_seq | 4647ns | 4750ns | 4000ns | 4653ns | 4963ns | +1.90% |
| an_b3_table | 4561ns | 4683ns | 3987ns | 4602ns | 4785ns | base |
| an_b3_tree | 4700ns | 4752ns | 3979ns | 4751ns | 4984ns | +3.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b3_pred | 3367ns | 3077ns | 3538ns | +60.92% | 0.076 |
| an_b3_prof | 2094ns | 1802ns | 2284ns | +0.05% | 0.122 |
| an_b3_seq | 2130ns | 1832ns | 2262ns | +1.80% | 0.120 |
| an_b3_table | 2093ns | 1831ns | 2191ns | base | 0.122 |
| an_b3_tree | 2154ns | 1825ns | 2272ns | +2.92% | 0.119 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_b3_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b3_pred | 0.073 | 51.7% |
| an_b3_prof | 0.118 | 82.8% |
| an_b3_seq | 0.117 | 82.2% |
| an_b3_table | 0.119 | 83.7% |
| an_b3_tree | 0.117 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b3_pred | 5792ns | 5792ns | +27.00% |
| an_b3_prof | 4557ns | 4557ns | -0.07% |
| an_b3_seq | 4647ns | 4647ns | +1.90% |
| an_b3_table | 4561ns | 4561ns | base |
| an_b3_tree | 4700ns | 4700ns | +3.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b3_table | 2152ns | base | --- | [1935, 2191] | --- | --- | --- | --- |
| an_b3_pred | 3485ns | +1341.1ns (+62.3%) | [+1071, +1412]ns | [3080, 3538] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b3_prof | 2176ns | no significant difference | [-206, +200]ns | [1821, 2284] | no | 0.6875 | 0.6875 | 0 |
| an_b3_seq | 2192ns | no significant difference | [-40, +139]ns | [1936, 2262] | no | 0.4375 | 0.2188 | 0 |
| an_b3_tree | 2185ns | no significant difference | [-3, +153]ns | [2004, 2272] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b3_table | an_b3_pred | an_b3_prof | an_b3_seq | an_b3_tree |
|---|---|---|---|---|---|
| 1 | 1831ns | +68.3% | +0.5% | +0.0% | -0.3% |
| 2 | 2184ns | +61.9% | +0.5% | +0.7% | -0.0% |
| 3 | 2185ns | +40.8% | -17.6% | +0.5% | +0.2% |
| 4 | 2197ns | +60.6% | -1.3% | +5.9% | +7.2% |
| 5 | 2120ns | +67.0% | +11.9% | -3.8% | +3.0% |
| 6 | 2038ns | +68.9% | +7.2% | +7.3% | +7.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b3_pred | -0.400 | moderate- |
| an_b3_prof | -0.121 | ok |
| an_b3_seq | -0.172 | ok |
| an_b3_table | -0.043 | ok |
| an_b3_tree | 0.036 | ok |

**Consistency summary:**

- **an_b3_pred**: won 0/6, lost 6/6
- **an_b3_prof**: won 2/6, lost 4/6
- **an_b3_seq**: won 1/6, lost 4/6
- **an_b3_tree**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b3_pred | 23.0ns | 3367.4ns | 0.7% |  |
| an_b3_prof | 7.0ns | 2093.7ns | 0.3% |  |
| an_b3_seq | 5.9ns | 2130.2ns | 0.3% |  |
| an_b3_table | 5.4ns | 2092.5ns | 0.3% |  |
| an_b3_tree | 6.7ns | 2153.8ns | 0.3% |  |

## Distribution (algo ns)

```
an_b3_pred (n=6, range 3077.1-3537.7 ns)
   3077.1 |########################################
   3100.1 |
   3123.2 |
   3146.2 |
   3169.2 |
   3192.2 |
   3215.3 |
   3238.3 |
   3261.3 |
   3284.4 |
   3307.4 |
   3330.4 |
   3353.5 |
   3376.5 |
   3399.5 |
   3422.5 |####################
   3445.6 |
   3468.6 |
   3491.6 |
   3514.7 |########################################
  (0 below, 1 above range)

an_b3_prof (n=6, range 1801.7-2283.9 ns)
   1801.7 |####################
   1825.8 |####################
   1849.9 |
   1874.0 |
   1898.2 |
   1922.3 |
   1946.4 |
   1970.5 |
   1994.6 |
   2018.7 |
   2042.8 |
   2066.9 |
   2091.1 |
   2115.2 |
   2139.3 |
   2163.4 |########################################
   2187.5 |####################
   2211.6 |
   2235.7 |
   2259.8 |
  (0 below, 1 above range)

an_b3_seq (n=6, range 1832.1-2262.1 ns)
   1832.1 |####################
   1853.6 |
   1875.1 |
   1896.6 |
   1918.1 |
   1939.6 |
   1961.1 |
   1982.6 |
   2004.1 |
   2025.6 |####################
   2047.1 |
   2068.6 |
   2090.1 |
   2111.6 |
   2133.1 |
   2154.6 |
   2176.1 |########################################
   2197.6 |####################
   2219.1 |
   2240.6 |
  (0 below, 1 above range)

an_b3_table (n=6, range 1831.2-2191.1 ns)
   1831.2 |####################
   1849.2 |
   1867.2 |
   1885.2 |
   1903.2 |
   1921.2 |
   1939.2 |
   1957.1 |
   1975.1 |
   1993.1 |
   2011.1 |
   2029.1 |####################
   2047.1 |
   2065.1 |
   2083.1 |
   2101.1 |
   2119.1 |####################
   2137.1 |
   2155.1 |
   2173.1 |########################################
  (0 below, 1 above range)

an_b3_tree (n=6, range 1825.4-2271.7 ns)
   1825.4 |##########
   1847.7 |
   1870.0 |
   1892.3 |
   1914.7 |
   1937.0 |
   1959.3 |
   1981.6 |
   2003.9 |
   2026.2 |
   2048.6 |
   2070.9 |
   2093.2 |
   2115.5 |
   2137.8 |
   2160.1 |
   2182.4 |########################################
   2204.8 |
   2227.1 |
   2249.4 |
  (0 below, 1 above range)

```
