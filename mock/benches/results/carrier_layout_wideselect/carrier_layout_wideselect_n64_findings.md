# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_lay_wideselect_rec32, carrier_lay_wideselect_rec20) are a dead heat (<1%)

carrier_lay_wideselect_rec32 (2.37 us) and carrier_lay_wideselect_rec20 (2.39 us) differ by 0.69%, inside the noise, even though the wider field spreads 4.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_lay_wideselect_rec16 shows alternating (throttle bounce) (autocorr -0.57)

carrier_lay_wideselect_rec16's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 4.6% of the fastest

All 5 variants sit between 2.37 us and 2.48 us - a 4.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_wideselect_rec16's edge over baseline is significant but tiny (10 ns, 0.40%)

carrier_lay_wideselect_rec16 differs from baseline carrier_lay_wideselect_rec24 by 10 ns (0.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_wideselect_rec32** at 2369.3 ns median (-1.0% vs baseline)
- Spread: 1.05x (fastest 2369.3 ns, slowest 2479.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 4925ns | 4977ns | 4515ns | 4941ns | 5106ns | -1.28% |
| carrier_lay_wideselect_rec16 | 4953ns | 4946ns | 4904ns | 4936ns | 5001ns | -0.72% |
| carrier_lay_wideselect_rec20 | 4980ns | 4946ns | 4848ns | 4932ns | 5117ns | -0.17% |
| carrier_lay_wideselect_rec24 | 4988ns | 4995ns | 4910ns | 4975ns | 5047ns | base |
| carrier_lay_wideselect_rec32 | 4928ns | 4918ns | 4818ns | 4914ns | 5004ns | -1.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2440ns | 2178ns | 2522ns | +1.99% | 0.026 |
| carrier_lay_wideselect_rec16 | 2405ns | 2362ns | 2432ns | +0.53% | 0.027 |
| carrier_lay_wideselect_rec20 | 2395ns | 2365ns | 2427ns | +0.13% | 0.027 |
| carrier_lay_wideselect_rec24 | 2392ns | 2372ns | 2410ns | base | 0.027 |
| carrier_lay_wideselect_rec32 | 2377ns | 2361ns | 2400ns | -0.64% | 0.027 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_lay_wideselect_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.026 | 87.8% |
| carrier_lay_wideselect_rec16 | 0.027 | 90.6% |
| carrier_lay_wideselect_rec20 | 0.027 | 91.3% |
| carrier_lay_wideselect_rec24 | 0.027 | 91.0% |
| carrier_lay_wideselect_rec32 | 0.027 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 4925ns | 4925ns | -1.28% |
| carrier_lay_wideselect_rec16 | 4953ns | 4953ns | -0.72% |
| carrier_lay_wideselect_rec20 | 4980ns | 4980ns | -0.17% |
| carrier_lay_wideselect_rec24 | 4988ns | 4988ns | base |
| carrier_lay_wideselect_rec32 | 4928ns | 4928ns | -1.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 2394ns | base | --- | [2373, 2410] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 2479ns | no significant difference | [-76, +119]ns | [2318, 2522] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 2403ns | no significant difference | [-11, +39]ns | [2379, 2432] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_wideselect_rec20 | 2386ns | no significant difference | [-24, +36]ns | [2374, 2427] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_wideselect_rec32 | 2369ns | no significant difference | [-34, +3]ns | [2361, 2400] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 2381ns | -8.5% | +0.9% | +0.1% | -0.8% |
| 2 | 2407ns | +4.4% | -0.1% | -0.9% | +0.5% |
| 3 | 2372ns | +5.0% | -0.4% | -0.3% | -0.4% |
| 4 | 2412ns | +4.9% | +1.3% | -1.1% | -1.3% |
| 5 | 2408ns | +2.1% | -0.5% | +0.1% | -1.5% |
| 6 | 2374ns | +3.9% | +2.0% | +2.9% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | -0.094 | ok |
| carrier_lay_wideselect_rec16 | -0.565 | HIGH- (thermal bounce) |
| carrier_lay_wideselect_rec20 | 0.325 | moderate+ |
| carrier_lay_wideselect_rec24 | -0.486 | moderate- |
| carrier_lay_wideselect_rec32 | -0.557 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec16**: won 3/6, lost 3/6
- **carrier_lay_wideselect_rec20**: won 3/6, lost 1/6
- **carrier_lay_wideselect_rec32**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 86435.1ns | 2439.8ns | 3542.7% | HIGH |
| carrier_lay_wideselect_rec16 | 86732.3ns | 2405.0ns | 3606.4% | HIGH |
| carrier_lay_wideselect_rec20 | 86246.2ns | 2395.4ns | 3600.5% | HIGH |
| carrier_lay_wideselect_rec24 | 86206.3ns | 2392.3ns | 3603.5% | HIGH |
| carrier_lay_wideselect_rec32 | 86739.7ns | 2377.1ns | 3649.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 2177.5-2521.9 ns)
   2177.5 |####################
   2194.7 |
   2211.9 |
   2229.2 |
   2246.4 |
   2263.6 |
   2280.8 |
   2298.0 |
   2315.3 |
   2332.5 |
   2349.7 |
   2366.9 |
   2384.1 |
   2401.4 |
   2418.6 |
   2435.8 |
   2453.0 |########################################
   2470.2 |
   2487.5 |####################
   2504.7 |####################
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 2362.5-2432.5 ns)
   2362.5 |####################
   2366.0 |
   2369.5 |
   2373.0 |
   2376.5 |
   2380.0 |
   2383.5 |
   2387.0 |
   2390.5 |
   2394.0 |####################
   2397.5 |
   2401.0 |########################################
   2404.5 |
   2408.0 |
   2411.5 |
   2415.0 |
   2418.5 |####################
   2422.0 |
   2425.5 |
   2429.0 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 2365.0-2426.7 ns)
   2365.0 |####################
   2368.1 |
   2371.2 |
   2374.3 |
   2377.3 |
   2380.4 |####################
   2383.5 |########################################
   2386.6 |
   2389.7 |
   2392.8 |
   2395.8 |
   2398.9 |
   2402.0 |
   2405.1 |
   2408.2 |####################
   2411.3 |
   2414.4 |
   2417.4 |
   2420.5 |
   2423.6 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 2372.1-2410.0 ns)
   2372.1 |####################
   2374.0 |####################
   2375.9 |
   2377.8 |
   2379.7 |####################
   2381.6 |
   2383.5 |
   2385.4 |
   2387.3 |
   2389.2 |
   2391.1 |
   2392.9 |
   2394.8 |
   2396.7 |
   2398.6 |
   2400.5 |
   2402.4 |
   2404.3 |
   2406.2 |########################################
   2408.1 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 2361.2-2400.4 ns)
   2361.2 |########################################
   2363.2 |
   2365.1 |
   2367.1 |####################
   2369.0 |
   2371.0 |####################
   2373.0 |
   2374.9 |
   2376.9 |
   2378.8 |
   2380.8 |####################
   2382.8 |
   2384.7 |
   2386.7 |
   2388.6 |
   2390.6 |
   2392.6 |
   2394.5 |
   2396.5 |
   2398.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=3487.8% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=3602.7% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=3614.8% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=3603.7% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=3659.3% of algo (FFI overhead may distort results)
