# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_tight_rec32 shows alternating (throttle bounce) (autocorr -0.64)

carrier_lay_tight_rec32's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_lay_tight_rec24)

The baseline carrier_lay_tight_rec24 is the fastest (2.14 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_lay_tight_rec24 vs stability leader carrier_lay_tight_rec32 (+3% speed for 1.9x steadier)

carrier_lay_tight_rec24 is fastest (2.14 us, CV 3.3%); carrier_lay_tight_rec32 gives up 2.7% median for 1.9x lower variance (CV 1.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.3% of the fastest

All 5 variants sit between 2.14 us and 2.21 us - a 3.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_tight_rec12's edge over baseline is significant but tiny (4 ns, 0.16%)

carrier_lay_tight_rec12 differs from baseline carrier_lay_tight_rec24 by 4 ns (0.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (carrier_lay_tight_rec24) is the fastest** at 2135.2 ns median
- Spread: 1.03x (fastest 2135.2 ns, slowest 2206.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 4505ns | 4573ns | 4190ns | 4504ns | 4665ns | +1.26% |
| carrier_lay_tight_rec16 | 4483ns | 4538ns | 4256ns | 4473ns | 4610ns | +0.75% |
| carrier_lay_tight_rec20 | 4486ns | 4549ns | 4320ns | 4474ns | 4586ns | +0.83% |
| carrier_lay_tight_rec24 | 4449ns | 4434ns | 4297ns | 4392ns | 4611ns | base |
| carrier_lay_tight_rec32 | 4592ns | 4576ns | 4400ns | 4566ns | 4726ns | +3.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 2170ns | 2014ns | 2249ns | +1.19% | 0.029 |
| carrier_lay_tight_rec16 | 2159ns | 2058ns | 2217ns | +0.67% | 0.030 |
| carrier_lay_tight_rec20 | 2164ns | 2084ns | 2212ns | +0.91% | 0.030 |
| carrier_lay_tight_rec24 | 2144ns | 2071ns | 2224ns | base | 0.030 |
| carrier_lay_tight_rec32 | 2189ns | 2124ns | 2228ns | +2.08% | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 264423 | 1500568 | 0.176 | 1.00× |
| carrier_lay_tight_rec16 | 263630 | 1504970 | 0.175 | 0.99× |
| carrier_lay_tight_rec20 | 263608 | 1500473 | 0.176 | 0.99× |
| carrier_lay_tight_rec24 | 265337 | 1515885 | 0.175 | 1.00× |
| carrier_lay_tight_rec32 | 262471 | 1489113 | 0.176 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_lay_tight_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.029 | 91.3% |
| carrier_lay_tight_rec16 | 0.029 | 92.2% |
| carrier_lay_tight_rec20 | 0.029 | 91.8% |
| carrier_lay_tight_rec24 | 0.030 | 94.3% |
| carrier_lay_tight_rec32 | 0.029 | 91.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 4505ns | 4505ns | +1.26% |
| carrier_lay_tight_rec16 | 4483ns | 4483ns | +0.75% |
| carrier_lay_tight_rec20 | 4486ns | 4486ns | +0.83% |
| carrier_lay_tight_rec24 | 4449ns | 4449ns | base |
| carrier_lay_tight_rec32 | 4592ns | 4592ns | +3.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 2135ns | base | --- | [2074, 2224] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 2207ns | no significant difference | [-33, +106]ns | [2054, 2249] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec16 | 2184ns | no significant difference | [-86, +114]ns | [2075, 2217] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_tight_rec20 | 2194ns | no significant difference | [-78, +126]ns | [2085, 2212] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 2194ns | no significant difference | [-54, +132]ns | [2145, 2228] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 2071ns | -2.7% | +5.0% | +7.0% | +6.0% |
| 2 | 2077ns | +9.5% | +0.7% | +0.4% | +6.8% |
| 3 | 2079ns | +0.7% | +6.0% | +5.1% | +4.1% |
| 4 | 2215ns | +0.3% | +0.7% | -5.9% | +1.1% |
| 5 | 2233ns | -0.4% | -7.8% | -1.1% | -4.9% |
| 6 | 2192ns | +0.1% | +0.1% | +0.5% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | -0.513 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec16 | -0.499 | moderate- |
| carrier_lay_tight_rec20 | -0.490 | moderate- |
| carrier_lay_tight_rec24 | 0.519 | HIGH+ (drift/warm-up) |
| carrier_lay_tight_rec32 | -0.642 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 2/6, lost 3/6
- **carrier_lay_tight_rec16**: won 1/6, lost 4/6
- **carrier_lay_tight_rec20**: won 2/6, lost 4/6
- **carrier_lay_tight_rec32**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 86102.3ns | 2169.7ns | 3968.3% | HIGH |
| carrier_lay_tight_rec16 | 85924.4ns | 2158.7ns | 3980.4% | HIGH |
| carrier_lay_tight_rec20 | 85765.6ns | 2163.7ns | 3963.9% | HIGH |
| carrier_lay_tight_rec24 | 85986.2ns | 2144.2ns | 4010.1% | HIGH |
| carrier_lay_tight_rec32 | 86170.3ns | 2188.9ns | 3936.7% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 2014.2-2248.6 ns)
   2014.2 |####################
   2025.9 |
   2037.6 |
   2049.4 |
   2061.1 |
   2072.8 |
   2084.5 |####################
   2096.2 |
   2107.9 |
   2119.7 |
   2131.4 |
   2143.1 |
   2154.8 |
   2166.5 |
   2178.2 |
   2190.0 |####################
   2201.7 |
   2213.4 |########################################
   2225.1 |
   2236.8 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 2057.9-2217.1 ns)
   2057.9 |########################################
   2065.9 |
   2073.8 |
   2081.8 |
   2089.7 |########################################
   2097.7 |
   2105.6 |
   2113.6 |
   2121.6 |
   2129.5 |
   2137.5 |
   2145.4 |
   2153.4 |
   2161.3 |
   2169.3 |########################################
   2177.3 |
   2185.2 |
   2193.2 |########################################
   2201.1 |########################################
   2209.1 |
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 2084.2-2211.9 ns)
   2084.2 |########################################
   2090.6 |
   2097.0 |
   2103.3 |
   2109.7 |
   2116.1 |
   2122.5 |
   2128.9 |
   2135.3 |
   2141.6 |
   2148.0 |
   2154.4 |
   2160.8 |
   2167.2 |
   2173.6 |
   2179.9 |####################
   2186.3 |
   2192.7 |
   2199.1 |####################
   2205.5 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 2070.8-2223.8 ns)
   2070.8 |########################################
   2078.4 |####################
   2086.1 |
   2093.7 |
   2101.4 |
   2109.0 |
   2116.7 |
   2124.3 |
   2132.0 |
   2139.6 |
   2147.3 |
   2154.9 |
   2162.6 |
   2170.2 |
   2177.9 |
   2185.5 |####################
   2193.2 |
   2200.8 |
   2208.5 |####################
   2216.1 |
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 2124.2-2228.3 ns)
   2124.2 |####################
   2129.4 |
   2134.6 |
   2139.8 |
   2145.0 |
   2150.2 |
   2155.4 |
   2160.7 |####################
   2165.9 |
   2171.1 |
   2176.3 |
   2181.5 |
   2186.7 |
   2191.9 |########################################
   2197.1 |
   2202.3 |
   2207.5 |
   2212.7 |####################
   2217.9 |
   2223.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=3898.5% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=3933.0% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=3914.5% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **carrier_lay_tight_rec24**: bridge=4029.5% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=3927.8% of algo (FFI overhead may distort results)
