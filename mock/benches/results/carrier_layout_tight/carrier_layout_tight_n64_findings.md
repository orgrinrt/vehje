# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_tight_rec12 is fastest but the noisiest (CV 6.4%)

carrier_lay_tight_rec12 wins on median (2.26 us) yet has the highest variance (CV 6.4%), while carrier_lay_tight_rec16 is the steadiest (CV 0.8%, 2.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_lay_tight_rec24 shows alternating (throttle bounce) (autocorr -0.56)

carrier_lay_tight_rec24's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (32 ns) is smaller than the fastest variant's own run-to-run std-dev (144 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_tight_rec12 vs stability leader carrier_lay_tight_rec16 (+1% speed for 7.8x steadier)

carrier_lay_tight_rec12 is fastest (2.26 us, CV 6.4%); carrier_lay_tight_rec16 gives up 0.8% median for 7.8x lower variance (CV 0.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.4% of the fastest

All 5 variants sit between 2.26 us and 2.30 us - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_tight_rec12's edge over baseline is significant but tiny (1 ns, 0.05%)

carrier_lay_tight_rec12 differs from baseline carrier_lay_tight_rec24 by 1 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_tight_rec12** at 2264.8 ns median (-0.7% vs baseline)
- Spread: 1.01x (fastest 2264.8 ns, slowest 2297.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 4611ns | 4755ns | 4123ns | 4581ns | 4901ns | -4.19% |
| carrier_lay_tight_rec16 | 4831ns | 4812ns | 4736ns | 4792ns | 4938ns | +0.39% |
| carrier_lay_tight_rec20 | 4856ns | 4864ns | 4756ns | 4846ns | 4922ns | +0.90% |
| carrier_lay_tight_rec24 | 4813ns | 4797ns | 4707ns | 4779ns | 4916ns | base |
| carrier_lay_tight_rec32 | 4850ns | 4831ns | 4803ns | 4822ns | 4915ns | +0.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 2188ns | 1961ns | 2311ns | -3.87% | 0.029 |
| carrier_lay_tight_rec16 | 2286ns | 2254ns | 2308ns | +0.45% | 0.028 |
| carrier_lay_tight_rec20 | 2293ns | 2246ns | 2325ns | +0.74% | 0.028 |
| carrier_lay_tight_rec24 | 2276ns | 2232ns | 2306ns | base | 0.028 |
| carrier_lay_tight_rec32 | 2295ns | 2270ns | 2319ns | +0.85% | 0.028 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_lay_tight_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.028 | 86.6% |
| carrier_lay_tight_rec16 | 0.028 | 85.9% |
| carrier_lay_tight_rec20 | 0.028 | 85.4% |
| carrier_lay_tight_rec24 | 0.028 | 86.0% |
| carrier_lay_tight_rec32 | 0.028 | 85.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 4611ns | 4611ns | -4.19% |
| carrier_lay_tight_rec16 | 4831ns | 4831ns | +0.39% |
| carrier_lay_tight_rec20 | 4856ns | 4856ns | +0.90% |
| carrier_lay_tight_rec24 | 4813ns | 4813ns | base |
| carrier_lay_tight_rec32 | 4850ns | 4850ns | +0.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 2280ns | base | --- | [2242, 2306] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 2265ns | no significant difference | [-292, +27]ns | [1988, 2311] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec16 | 2282ns | no significant difference | [-11, +45]ns | [2267, 2308] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec20 | 2297ns | no significant difference | [-20, +55]ns | [2256, 2325] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 2296ns | no significant difference | [-30, +66]ns | [2271, 2319] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 2291ns | -14.4% | -0.5% | -0.5% | -0.9% |
| 2 | 2295ns | -0.6% | -0.5% | -1.3% | +1.1% |
| 3 | 2251ns | +1.6% | +0.1% | +2.9% | +0.9% |
| 4 | 2316ns | +0.8% | -0.5% | +0.7% | -1.7% |
| 5 | 2232ns | +0.7% | +2.2% | +0.6% | +3.7% |
| 6 | 2270ns | -11.2% | +1.9% | +2.1% | +2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | 0.008 | ok |
| carrier_lay_tight_rec16 | -0.368 | moderate- |
| carrier_lay_tight_rec20 | -0.401 | moderate- |
| carrier_lay_tight_rec24 | -0.563 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec32 | -0.219 | moderate- |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 3/6, lost 3/6
- **carrier_lay_tight_rec16**: won 3/6, lost 3/6
- **carrier_lay_tight_rec20**: won 2/6, lost 4/6
- **carrier_lay_tight_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 86086.3ns | 2187.9ns | 3934.6% | HIGH |
| carrier_lay_tight_rec16 | 86389.4ns | 2286.0ns | 3779.0% | HIGH |
| carrier_lay_tight_rec20 | 86226.7ns | 2292.8ns | 3760.8% | HIGH |
| carrier_lay_tight_rec24 | 86050.9ns | 2275.9ns | 3781.0% | HIGH |
| carrier_lay_tight_rec32 | 86615.6ns | 2295.1ns | 3773.9% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 1961.2-2310.6 ns)
   1961.2 |####################
   1978.7 |
   1996.1 |
   2013.6 |####################
   2031.1 |
   2048.6 |
   2066.0 |
   2083.5 |
   2101.0 |
   2118.5 |
   2135.9 |
   2153.4 |
   2170.9 |
   2188.3 |
   2205.8 |
   2223.3 |
   2240.8 |####################
   2258.2 |
   2275.7 |########################################
   2293.2 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 2254.2-2308.3 ns)
   2254.2 |####################
   2256.9 |
   2259.6 |
   2262.3 |
   2265.0 |
   2267.7 |
   2270.4 |
   2273.2 |
   2275.9 |
   2278.6 |########################################
   2281.3 |
   2284.0 |####################
   2286.7 |
   2289.4 |
   2292.1 |
   2294.8 |
   2297.5 |
   2300.2 |
   2302.9 |####################
   2305.6 |
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 2246.2-2325.0 ns)
   2246.2 |####################
   2250.1 |
   2254.1 |
   2258.0 |
   2262.0 |####################
   2265.9 |
   2269.8 |
   2273.8 |
   2277.7 |####################
   2281.7 |
   2285.6 |
   2289.5 |
   2293.5 |
   2297.4 |
   2301.4 |
   2305.3 |
   2309.2 |
   2313.2 |########################################
   2317.1 |
   2321.1 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 2232.1-2305.6 ns)
   2232.1 |########################################
   2235.8 |
   2239.4 |
   2243.1 |
   2246.8 |
   2250.5 |########################################
   2254.2 |
   2257.8 |
   2261.5 |
   2265.2 |
   2268.8 |########################################
   2272.5 |
   2276.2 |
   2279.9 |
   2283.5 |
   2287.2 |########################################
   2290.9 |
   2294.6 |########################################
   2298.2 |
   2301.9 |
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 2270.4-2318.9 ns)
   2270.4 |########################################
   2272.8 |
   2275.3 |####################
   2277.7 |
   2280.1 |
   2282.5 |
   2285.0 |
   2287.4 |
   2289.8 |
   2292.2 |
   2294.7 |
   2297.1 |
   2299.5 |
   2302.0 |
   2304.4 |
   2306.8 |
   2309.2 |
   2311.7 |
   2314.1 |####################
   2316.5 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=3801.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=3787.1% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=3745.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=3773.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=3781.6% of algo (FFI overhead may distort results)
