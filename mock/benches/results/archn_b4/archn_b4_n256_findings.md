# Per-branch strategy (NATIVE tier): archetype 4

5 variants, 6 samples per variant.
Baseline: **an_b4_table**

## Highlights

Baseline for all deltas below: **an_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b4_tree shows alternating (throttle bounce) (autocorr -0.73)

an_b4_tree's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### an_b4_seq's comparison is tie-heavy (17% tied pairs)

17% of paired samples for an_b4_seq are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### an_b4_prof's edge over baseline is significant but tiny (-26 ns, 1.20%)

an_b4_prof differs from baseline an_b4_table by -26 ns (1.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b4_tree** at 2120.0 ns median (-4.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.20x (fastest 2120.0 ns, slowest 2534.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b4_pred | 5037ns | 5018ns | 4336ns | 4963ns | 5498ns | +6.25% |
| an_b4_prof | 4551ns | 4535ns | 4006ns | 4385ns | 5072ns | -4.00% |
| an_b4_seq | 4893ns | 4948ns | 4144ns | 4922ns | 5224ns | +3.22% |
| an_b4_table | 4741ns | 4922ns | 4123ns | 4661ns | 5168ns | base |
| an_b4_tree | 4686ns | 4613ns | 4137ns | 4555ns | 5158ns | -1.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b4_pred | 2538ns | 2184ns | 2770ns | +17.19% | 0.101 |
| an_b4_prof | 2151ns | 1894ns | 2394ns | -0.68% | 0.119 |
| an_b4_seq | 2319ns | 1960ns | 2494ns | +7.10% | 0.110 |
| an_b4_table | 2166ns | 1897ns | 2381ns | base | 0.118 |
| an_b4_tree | 2151ns | 1900ns | 2362ns | -0.68% | 0.119 |

## Performance model

- Peak throughput: **0.135 Gops/s** (an_b4_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b4_pred | 0.101 | 74.7% |
| an_b4_prof | 0.120 | 88.4% |
| an_b4_seq | 0.109 | 80.8% |
| an_b4_table | 0.116 | 85.5% |
| an_b4_tree | 0.121 | 89.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b4_pred | 5037ns | 5037ns | +6.25% |
| an_b4_prof | 4551ns | 4551ns | -4.00% |
| an_b4_seq | 4893ns | 4893ns | +3.22% |
| an_b4_table | 4741ns | 4741ns | base |
| an_b4_tree | 4686ns | 4686ns | -1.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b4_table | 2216ns | base | --- | [1900, 2381] | --- | --- | --- | --- |
| an_b4_pred | 2535ns | +346.2ns (+15.6%) | [+272, +499]ns | [2309, 2770] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b4_prof | 2142ns | no significant difference | [-275, +257]ns | [1916, 2394] | no | 1.0000 | 1.0000 | 0 |
| an_b4_seq | 2344ns | +82.5ns (+3.7%) | [+27, +352]ns | [2120, 2494] | YES (adj: no) | 0.1250 | 0.0625 | **1** (17%, HIGH) |
| an_b4_tree | 2120ns | no significant difference | [-204, +161]ns | [1970, 2362] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b4_table | an_b4_pred | an_b4_prof | an_b4_seq | an_b4_tree |
|---|---|---|---|---|---|
| 1 | 2260ns | +15.8% | -16.2% | +2.4% | -9.2% |
| 2 | 2374ns | +14.2% | +4.9% | +0.0% | -0.0% |
| 3 | 1897ns | +29.3% | +21.0% | +29.9% | +7.5% |
| 4 | 2172ns | +12.0% | -8.4% | +4.9% | +8.2% |
| 5 | 1902ns | +14.8% | +1.9% | +3.0% | -0.1% |
| 6 | 2388ns | +18.5% | -3.7% | +5.7% | -8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b4_pred | -0.221 | moderate- |
| an_b4_prof | -0.198 | ok |
| an_b4_seq | -0.293 | moderate- |
| an_b4_table | -0.404 | moderate- |
| an_b4_tree | -0.728 | HIGH- (thermal bounce) |

**Consistency summary:**

- **an_b4_pred**: won 0/6, lost 6/6
- **an_b4_prof**: won 3/6, lost 3/6
- **an_b4_seq**: won 0/6, lost 5/6
- **an_b4_tree**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b4_pred | 5.2ns | 2537.8ns | 0.2% |  |
| an_b4_prof | 6.3ns | 2150.8ns | 0.3% |  |
| an_b4_seq | 6.2ns | 2319.3ns | 0.3% |  |
| an_b4_table | 6.6ns | 2165.5ns | 0.3% |  |
| an_b4_tree | 6.6ns | 2150.8ns | 0.3% |  |

## Distribution (algo ns)

```
an_b4_pred (n=6, range 2184.2-2770.0 ns)
   2184.2 |########################################
   2213.5 |
   2242.8 |
   2272.1 |
   2301.4 |
   2330.6 |
   2359.9 |
   2389.2 |
   2418.5 |########################################
   2447.8 |########################################
   2477.1 |
   2506.4 |
   2535.7 |
   2565.0 |
   2594.3 |########################################
   2623.6 |
   2652.8 |
   2682.1 |########################################
   2711.4 |
   2740.7 |
  (0 below, 1 above range)

an_b4_prof (n=6, range 1893.8-2393.9 ns)
   1893.8 |####################
   1918.8 |####################
   1943.8 |
   1968.8 |####################
   1993.8 |
   2018.8 |
   2043.8 |
   2068.9 |
   2093.9 |
   2118.9 |
   2143.9 |
   2168.9 |
   2193.9 |
   2218.9 |
   2243.9 |
   2268.9 |
   2293.9 |########################################
   2318.9 |
   2343.9 |
   2368.9 |
  (0 below, 1 above range)

an_b4_seq (n=6, range 1959.6-2494.2 ns)
   1959.6 |########################################
   1986.3 |
   2013.1 |
   2039.8 |
   2066.5 |
   2093.2 |
   2120.0 |
   2146.7 |
   2173.4 |
   2200.2 |
   2226.9 |
   2253.6 |########################################
   2280.4 |
   2307.1 |########################################
   2333.8 |
   2360.5 |########################################
   2387.3 |
   2414.0 |
   2440.7 |########################################
   2467.5 |
  (0 below, 1 above range)

an_b4_table (n=6, range 1897.1-2380.7 ns)
   1897.1 |########################################
   1921.3 |
   1945.5 |
   1969.6 |
   1993.8 |
   2018.0 |
   2042.2 |
   2066.3 |
   2090.5 |
   2114.7 |
   2138.9 |
   2163.1 |####################
   2187.2 |
   2211.4 |
   2235.6 |
   2259.8 |####################
   2283.9 |
   2308.1 |
   2332.3 |
   2356.5 |####################
  (0 below, 1 above range)

an_b4_tree (n=6, range 1900.4-2362.1 ns)
   1900.4 |####################
   1923.5 |
   1946.6 |
   1969.6 |
   1992.7 |
   2015.8 |
   2038.9 |########################################
   2062.0 |
   2085.1 |
   2108.1 |
   2131.2 |
   2154.3 |
   2177.4 |####################
   2200.5 |
   2223.6 |
   2246.6 |
   2269.7 |
   2292.8 |
   2315.9 |
   2339.0 |####################
  (0 below, 1 above range)

```
