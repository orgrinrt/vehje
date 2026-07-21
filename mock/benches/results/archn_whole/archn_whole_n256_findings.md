# Whole-program single strategy (NATIVE tier)

5 variants, 6 samples per variant.
Baseline: **an_whole_table**

## Highlights

Baseline for all deltas below: **an_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_whole_pred is an outlier: 2.0x slower than the field

an_whole_pred (3.96 us) is 2.0x the fastest (1.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### an_whole_prof is fastest but the noisiest (CV 15.1%)

an_whole_prof wins on median (1.95 us) yet has the highest variance (CV 15.1%), while an_whole_tree is the steadiest (CV 8.8%, 2.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### an_whole_table shows warm-up / thermal drift (autocorr +0.50)

an_whole_table's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {an_whole_prof, an_whole_table, an_whole_seq, an_whole_tree} vs {an_whole_pred} (74% apart)

The field splits into a fast tier {an_whole_prof, an_whole_table, an_whole_seq, an_whole_tree} and a slow tier {an_whole_pred} with a 74% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: an_whole_prof** at 1946.9 ns median (-7.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.03x (fastest 1946.9 ns, slowest 3961.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_whole_pred | 6512ns | 6540ns | 5479ns | 6193ns | 7506ns | +39.12% |
| an_whole_prof | 4512ns | 4489ns | 3785ns | 4270ns | 5239ns | -3.60% |
| an_whole_seq | 4758ns | 4856ns | 3825ns | 4761ns | 5220ns | +1.65% |
| an_whole_table | 4681ns | 4585ns | 3987ns | 4431ns | 5403ns | base |
| an_whole_tree | 4802ns | 4973ns | 3985ns | 4825ns | 5177ns | +2.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_whole_pred | 3942ns | 3317ns | 4546ns | +83.29% | 0.065 |
| an_whole_prof | 1976ns | 1650ns | 2317ns | -8.13% | 0.130 |
| an_whole_seq | 2104ns | 1681ns | 2294ns | -2.19% | 0.122 |
| an_whole_table | 2151ns | 1835ns | 2477ns | base | 0.119 |
| an_whole_tree | 2199ns | 1841ns | 2374ns | +2.25% | 0.116 |

## Performance model

- Peak throughput: **0.155 Gops/s** (an_whole_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_whole_pred | 0.065 | 41.7% |
| an_whole_prof | 0.131 | 84.8% |
| an_whole_seq | 0.117 | 75.7% |
| an_whole_table | 0.121 | 78.3% |
| an_whole_tree | 0.112 | 72.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_whole_pred | 6512ns | 6512ns | +39.12% |
| an_whole_prof | 4512ns | 4512ns | -3.60% |
| an_whole_seq | 4758ns | 4758ns | +1.65% |
| an_whole_table | 4681ns | 4681ns | base |
| an_whole_tree | 4802ns | 4802ns | +2.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_whole_table | 2108ns | base | --- | [1868, 2477] | --- | --- | --- | --- |
| an_whole_pred | 3961ns | +1827.9ns (+86.7%) | [+1377, +2170]ns | [3320, 4546] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_whole_prof | 1947ns | -160.6ns (-7.6%) | [-311, -53]ns | [1665, 2317] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_whole_seq | 2179ns | no significant difference | [-187, +128]ns | [1839, 2294] | no | 0.6875 | 0.6875 | 0 |
| an_whole_tree | 2282ns | no significant difference | [-103, +175]ns | [1941, 2374] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_whole_table | an_whole_pred | an_whole_prof | an_whole_seq | an_whole_tree |
|---|---|---|---|---|---|
| 1 | 1901ns | +108.0% | -13.2% | +5.1% | +7.4% |
| 2 | 1835ns | +80.7% | -5.3% | -8.4% | +0.3% |
| 3 | 2050ns | +62.1% | -18.1% | +7.8% | +7.3% |
| 4 | 2165ns | +102.8% | -0.4% | -0.5% | +9.2% |
| 5 | 2366ns | +67.7% | -4.1% | -6.9% | +0.2% |
| 6 | 2588ns | +81.6% | -8.6% | -8.1% | -8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_whole_pred | 0.086 | ok |
| an_whole_prof | 0.503 | HIGH+ (drift/warm-up) |
| an_whole_seq | 0.130 | ok |
| an_whole_table | 0.503 | HIGH+ (drift/warm-up) |
| an_whole_tree | 0.476 | moderate+ |

**Consistency summary:**

- **an_whole_pred**: won 0/6, lost 6/6
- **an_whole_prof**: won 6/6, lost 0/6
- **an_whole_seq**: won 4/6, lost 2/6
- **an_whole_tree**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_whole_pred | 5.0ns | 3942.5ns | 0.1% |  |
| an_whole_prof | 6.0ns | 1976.2ns | 0.3% |  |
| an_whole_seq | 6.5ns | 2103.9ns | 0.3% |  |
| an_whole_table | 7.4ns | 2150.9ns | 0.3% |  |
| an_whole_tree | 7.0ns | 2199.3ns | 0.3% |  |

## Distribution (algo ns)

```
an_whole_pred (n=6, range 3317.1-4546.2 ns)
   3317.1 |########################################
   3378.6 |
   3440.0 |
   3501.5 |
   3562.9 |
   3624.4 |
   3685.8 |
   3747.3 |
   3808.8 |
   3870.2 |
   3931.7 |########################################
   3993.1 |
   4054.6 |
   4116.0 |
   4177.5 |
   4239.0 |
   4300.4 |
   4361.9 |####################
   4423.3 |
   4484.8 |
  (0 below, 1 above range)

an_whole_prof (n=6, range 1650.4-2316.7 ns)
   1650.4 |########################################
   1683.7 |
   1717.0 |####################
   1750.3 |
   1783.7 |
   1817.0 |
   1850.3 |
   1883.6 |
   1916.9 |
   1950.2 |
   1983.5 |
   2016.9 |
   2050.2 |
   2083.5 |
   2116.8 |
   2150.1 |####################
   2183.4 |
   2216.8 |
   2250.1 |####################
   2283.4 |
  (0 below, 1 above range)

an_whole_seq (n=6, range 1680.8-2293.6 ns)
   1680.8 |####################
   1711.4 |
   1742.1 |
   1772.7 |
   1803.3 |
   1834.0 |
   1864.6 |
   1895.3 |
   1925.9 |
   1956.5 |
   1987.2 |####################
   2017.8 |
   2048.4 |
   2079.1 |
   2109.7 |
   2140.4 |####################
   2171.0 |
   2201.6 |########################################
   2232.3 |
   2262.9 |
  (0 below, 1 above range)

an_whole_table (n=6, range 1835.4-2476.9 ns)
   1835.4 |########################################
   1867.5 |
   1899.5 |########################################
   1931.6 |
   1963.7 |
   1995.8 |
   2027.8 |########################################
   2059.9 |
   2092.0 |
   2124.1 |
   2156.1 |########################################
   2188.2 |
   2220.3 |
   2252.3 |
   2284.4 |
   2316.5 |
   2348.6 |########################################
   2380.6 |
   2412.7 |
   2444.8 |
  (0 below, 1 above range)

an_whole_tree (n=6, range 1840.8-2373.9 ns)
   1840.8 |####################
   1867.5 |
   1894.1 |
   1920.8 |
   1947.4 |
   1974.1 |
   2000.7 |
   2027.4 |####################
   2054.1 |
   2080.7 |
   2107.4 |
   2134.0 |
   2160.7 |
   2187.3 |####################
   2214.0 |
   2240.7 |
   2267.3 |
   2294.0 |
   2320.6 |
   2347.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **an_whole_prof**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **an_whole_table**: autocorrelation=0.50 (measurement drift or warm-up artifact)
