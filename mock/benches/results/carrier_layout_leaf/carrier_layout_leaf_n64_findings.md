# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec12 is fastest but the noisiest (CV 5.1%)

carrier_lay_leaf_rec12 wins on median (2.13 us) yet has the highest variance (CV 5.1%), while carrier_lay_leaf_rec32 is the steadiest (CV 0.6%, 2.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (24 ns) is smaller than the fastest variant's own run-to-run std-dev (109 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_leaf_rec12 vs stability leader carrier_lay_leaf_rec32 (+1% speed for 9.0x steadier)

carrier_lay_leaf_rec12 is fastest (2.13 us, CV 5.1%); carrier_lay_leaf_rec32 gives up 0.9% median for 9.0x lower variance (CV 0.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.1% of the fastest

All 5 variants sit between 2.13 us and 2.15 us - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_leaf_rec12's edge over baseline is significant but tiny (-28 ns, 1.31%)

carrier_lay_leaf_rec12 differs from baseline carrier_lay_leaf_rec24 by -28 ns (1.31%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_leaf_rec12** at 2128.8 ns median (-0.8% vs baseline)
- Spread: 1.01x (fastest 2128.8 ns, slowest 2152.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 4596ns | 4638ns | 4180ns | 4620ns | 4768ns | -2.76% |
| carrier_lay_leaf_rec16 | 4806ns | 4810ns | 4644ns | 4782ns | 4924ns | +1.68% |
| carrier_lay_leaf_rec20 | 4748ns | 4754ns | 4713ns | 4742ns | 4772ns | +0.45% |
| carrier_lay_leaf_rec24 | 4727ns | 4743ns | 4648ns | 4719ns | 4777ns | base |
| carrier_lay_leaf_rec32 | 4700ns | 4699ns | 4645ns | 4685ns | 4749ns | -0.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 2082ns | 1839ns | 2140ns | -2.74% | 0.031 |
| carrier_lay_leaf_rec16 | 2150ns | 2125ns | 2170ns | +0.43% | 0.030 |
| carrier_lay_leaf_rec20 | 2154ns | 2121ns | 2185ns | +0.60% | 0.030 |
| carrier_lay_leaf_rec24 | 2141ns | 2098ns | 2172ns | base | 0.030 |
| carrier_lay_leaf_rec32 | 2148ns | 2124ns | 2159ns | +0.30% | 0.030 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_lay_leaf_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.030 | 86.4% |
| carrier_lay_leaf_rec16 | 0.030 | 85.4% |
| carrier_lay_leaf_rec20 | 0.030 | 85.6% |
| carrier_lay_leaf_rec24 | 0.030 | 85.7% |
| carrier_lay_leaf_rec32 | 0.030 | 85.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 4596ns | 4596ns | -2.76% |
| carrier_lay_leaf_rec16 | 4806ns | 4806ns | +1.68% |
| carrier_lay_leaf_rec20 | 4748ns | 4748ns | +0.45% |
| carrier_lay_leaf_rec24 | 4727ns | 4727ns | base |
| carrier_lay_leaf_rec32 | 4700ns | 4700ns | -0.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 2146ns | base | --- | [2105, 2172] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 2129ns | no significant difference | [-155, +7]ns | [1979, 2140] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_leaf_rec16 | 2152ns | no significant difference | [-29, +49]ns | [2128, 2170] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_leaf_rec20 | 2149ns | no significant difference | [-26, +48]ns | [2128, 2185] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec32 | 2149ns | no significant difference | [-23, +35]ns | [2135, 2159] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 2112ns | -12.9% | +0.6% | +2.2% | +0.6% |
| 2 | 2149ns | -0.9% | +0.4% | -1.3% | +0.2% |
| 3 | 2165ns | -1.8% | -1.5% | -1.1% | -0.8% |
| 4 | 2144ns | -1.2% | +0.3% | +2.3% | +1.0% |
| 5 | 2098ns | +1.6% | +4.0% | +1.7% | +2.3% |
| 6 | 2179ns | -1.4% | -1.2% | -0.2% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | -0.043 | ok |
| carrier_lay_leaf_rec16 | -0.112 | ok |
| carrier_lay_leaf_rec20 | -0.388 | moderate- |
| carrier_lay_leaf_rec24 | -0.364 | moderate- |
| carrier_lay_leaf_rec32 | -0.171 | ok |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 5/6, lost 1/6
- **carrier_lay_leaf_rec16**: won 2/6, lost 4/6
- **carrier_lay_leaf_rec20**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 86888.3ns | 2082.3ns | 4172.6% | HIGH |
| carrier_lay_leaf_rec16 | 86981.6ns | 2150.3ns | 4045.1% | HIGH |
| carrier_lay_leaf_rec20 | 87145.5ns | 2153.9ns | 4046.0% | HIGH |
| carrier_lay_leaf_rec24 | 86986.3ns | 2141.0ns | 4062.9% | HIGH |
| carrier_lay_leaf_rec32 | 86719.3ns | 2147.5ns | 4038.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 1839.2-2139.6 ns)
   1839.2 |#############
   1854.2 |
   1869.2 |
   1884.3 |
   1899.3 |
   1914.3 |
   1929.3 |
   1944.3 |
   1959.3 |
   1974.4 |
   1989.4 |
   2004.4 |
   2019.4 |
   2034.4 |
   2049.4 |
   2064.5 |
   2079.5 |
   2094.5 |
   2109.5 |#############
   2124.5 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 2125.0-2170.0 ns)
   2125.0 |########################################
   2127.2 |
   2129.5 |########################################
   2131.8 |
   2134.0 |
   2136.2 |
   2138.5 |
   2140.8 |
   2143.0 |
   2145.2 |
   2147.5 |
   2149.8 |########################################
   2152.0 |########################################
   2154.2 |
   2156.5 |########################################
   2158.8 |
   2161.0 |
   2163.2 |
   2165.5 |
   2167.8 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 2121.2-2184.6 ns)
   2121.2 |########################################
   2124.4 |
   2127.5 |
   2130.7 |
   2133.9 |########################################
   2137.0 |
   2140.2 |########################################
   2143.4 |
   2146.5 |
   2149.7 |
   2152.9 |
   2156.0 |########################################
   2159.2 |
   2162.4 |
   2165.5 |
   2168.7 |
   2171.9 |
   2175.0 |########################################
   2178.2 |
   2181.4 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 2097.5-2171.9 ns)
   2097.5 |########################################
   2101.2 |
   2104.9 |
   2108.7 |########################################
   2112.4 |
   2116.1 |
   2119.8 |
   2123.5 |
   2127.3 |
   2131.0 |
   2134.7 |
   2138.4 |
   2142.1 |########################################
   2145.9 |########################################
   2149.6 |
   2153.3 |
   2157.0 |
   2160.7 |
   2164.5 |########################################
   2168.2 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 2123.8-2158.8 ns)
   2123.8 |########################################
   2125.5 |
   2127.3 |
   2129.0 |
   2130.8 |
   2132.5 |
   2134.3 |
   2136.0 |
   2137.8 |
   2139.5 |
   2141.3 |
   2143.0 |
   2144.8 |########################################
   2146.5 |########################################
   2148.3 |########################################
   2150.0 |
   2151.8 |########################################
   2153.5 |
   2155.3 |
   2157.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=4082.7% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=4039.5% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=4045.9% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=4050.7% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=4034.9% of algo (FFI overhead may distort results)
