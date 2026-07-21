# Per-branch strategy (NATIVE tier): archetype 2

5 variants, 6 samples per variant.
Baseline: **an_b2_table**

## Highlights

Baseline for all deltas below: **an_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b2_prof is fastest but the noisiest (CV 13.1%)

an_b2_prof wins on median (2.10 us) yet has the highest variance (CV 13.1%), while an_b2_seq is the steadiest (CV 8.5%, 2.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (an_b2_prof, an_b2_table) are a dead heat (<1%)

an_b2_prof (2.10 us) and an_b2_table (2.11 us) differ by 0.22%, inside the noise, even though the wider field spreads 36.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b2_pred shows warm-up / thermal drift (autocorr +0.59)

an_b2_pred's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {an_b2_prof, an_b2_table, an_b2_seq, an_b2_tree} vs {an_b2_pred} (26% apart)

The field splits into a fast tier {an_b2_prof, an_b2_table, an_b2_seq, an_b2_tree} and a slow tier {an_b2_pred} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b2_prof vs stability leader an_b2_seq (+8% speed for 1.5x steadier)

an_b2_prof is fastest (2.10 us, CV 13.1%); an_b2_seq gives up 8.0% median for 1.5x lower variance (CV 8.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b2_prof's edge over baseline is significant but tiny (2 ns, 0.10%)

an_b2_prof differs from baseline an_b2_table by 2 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b2_prof** at 2101.8 ns median (-0.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.37x (fastest 2101.8 ns, slowest 2871.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b2_pred | 5340ns | 5532ns | 4550ns | 5264ns | 5850ns | +14.79% |
| an_b2_prof | 4574ns | 4590ns | 3952ns | 4391ns | 5161ns | -1.67% |
| an_b2_seq | 4786ns | 4942ns | 4018ns | 4779ns | 5179ns | +2.87% |
| an_b2_table | 4652ns | 4601ns | 3982ns | 4546ns | 5147ns | base |
| an_b2_tree | 4800ns | 4972ns | 3990ns | 4805ns | 5196ns | +3.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b2_pred | 2781ns | 2377ns | 3055ns | +30.09% | 0.092 |
| an_b2_prof | 2096ns | 1801ns | 2374ns | -1.97% | 0.122 |
| an_b2_seq | 2200ns | 1867ns | 2374ns | +2.89% | 0.116 |
| an_b2_table | 2138ns | 1837ns | 2368ns | base | 0.120 |
| an_b2_tree | 2210ns | 1838ns | 2406ns | +3.39% | 0.116 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_b2_prof; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b2_pred | 0.089 | 62.7% |
| an_b2_prof | 0.122 | 85.7% |
| an_b2_seq | 0.113 | 79.3% |
| an_b2_table | 0.122 | 85.5% |
| an_b2_tree | 0.112 | 78.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b2_pred | 5340ns | 5340ns | +14.79% |
| an_b2_prof | 4574ns | 4574ns | -1.67% |
| an_b2_seq | 4786ns | 4786ns | +2.87% |
| an_b2_table | 4652ns | 4652ns | base |
| an_b2_tree | 4800ns | 4800ns | +3.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b2_table | 2106ns | base | --- | [1940, 2368] | --- | --- | --- | --- |
| an_b2_pred | 2871ns | +669.4ns (+31.8%) | [+477, +784]ns | [2418, 3055] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b2_prof | 2102ns | no significant difference | [-232, +104]ns | [1811, 2374] | no | 0.6875 | 0.6875 | 0 |
| an_b2_seq | 2270ns | no significant difference | [-7, +180]ns | [1955, 2374] | no | 0.6875 | 0.6875 | 0 |
| an_b2_tree | 2284ns | +21.0ns (+1.0%) | [+1, +195]ns | [1941, 2406] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b2_table | an_b2_pred | an_b2_prof | an_b2_seq | an_b2_tree |
|---|---|---|---|---|---|
| 1 | 2044ns | +20.3% | -10.9% | -0.1% | +0.0% |
| 2 | 1837ns | +29.4% | +0.2% | +1.7% | +0.0% |
| 3 | 2043ns | +32.3% | -11.8% | +16.2% | +16.5% |
| 4 | 2169ns | +40.1% | +9.1% | +0.5% | +0.8% |
| 5 | 2363ns | +29.5% | +0.0% | +0.5% | +1.0% |
| 6 | 2372ns | +28.6% | +0.5% | -0.5% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b2_pred | 0.589 | HIGH+ (drift/warm-up) |
| an_b2_prof | 0.474 | moderate+ |
| an_b2_seq | 0.070 | ok |
| an_b2_table | 0.528 | HIGH+ (drift/warm-up) |
| an_b2_tree | 0.101 | ok |

**Consistency summary:**

- **an_b2_pred**: won 0/6, lost 6/6
- **an_b2_prof**: won 2/6, lost 3/6
- **an_b2_seq**: won 2/6, lost 4/6
- **an_b2_tree**: won 0/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b2_pred | 5.0ns | 2781.3ns | 0.2% |  |
| an_b2_prof | 5.5ns | 2095.8ns | 0.3% |  |
| an_b2_seq | 6.6ns | 2199.7ns | 0.3% |  |
| an_b2_table | 6.4ns | 2138.0ns | 0.3% |  |
| an_b2_tree | 6.3ns | 2210.4ns | 0.3% |  |

## Distribution (algo ns)

```
an_b2_pred (n=6, range 2376.7-3055.4 ns)
   2376.7 |####################
   2410.6 |
   2444.6 |####################
   2478.5 |
   2512.4 |
   2546.4 |
   2580.3 |
   2614.2 |
   2648.2 |
   2682.1 |####################
   2716.0 |
   2750.0 |
   2783.9 |
   2817.9 |
   2851.8 |
   2885.7 |
   2919.7 |
   2953.6 |
   2987.5 |
   3021.5 |########################################
  (0 below, 1 above range)

an_b2_prof (n=6, range 1801.2-2374.2 ns)
   1801.2 |########################################
   1829.8 |####################
   1858.5 |
   1887.1 |
   1915.8 |
   1944.4 |
   1973.1 |
   2001.7 |
   2030.4 |
   2059.0 |
   2087.7 |
   2116.3 |
   2145.0 |
   2173.6 |
   2202.3 |
   2230.9 |
   2259.6 |
   2288.2 |
   2316.9 |
   2345.5 |########################################
  (0 below, 1 above range)

an_b2_seq (n=6, range 1867.1-2374.0 ns)
   1867.1 |####################
   1892.4 |
   1917.8 |
   1943.1 |
   1968.5 |
   1993.8 |
   2019.2 |####################
   2044.5 |
   2069.9 |
   2095.2 |
   2120.6 |
   2145.9 |
   2171.2 |####################
   2196.6 |
   2221.9 |
   2247.3 |
   2272.6 |
   2298.0 |
   2323.3 |
   2348.7 |########################################
  (0 below, 1 above range)

an_b2_table (n=6, range 1836.7-2367.5 ns)
   1836.7 |####################
   1863.2 |
   1889.8 |
   1916.3 |
   1942.9 |
   1969.4 |
   1995.9 |
   2022.5 |########################################
   2049.0 |
   2075.6 |
   2102.1 |
   2128.6 |
   2155.2 |####################
   2181.7 |
   2208.3 |
   2234.8 |
   2261.3 |
   2287.9 |
   2314.4 |
   2341.0 |####################
  (0 below, 1 above range)

an_b2_tree (n=6, range 1837.5-2406.2 ns)
   1837.5 |####################
   1865.9 |
   1894.4 |
   1922.8 |
   1951.2 |
   1979.7 |
   2008.1 |
   2036.6 |####################
   2065.0 |
   2093.4 |
   2121.9 |
   2150.3 |
   2178.8 |####################
   2207.2 |
   2235.6 |
   2264.1 |
   2292.5 |
   2320.9 |
   2349.4 |
   2377.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **an_b2_pred**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **an_b2_table**: autocorrelation=0.53 (measurement drift or warm-up artifact)
