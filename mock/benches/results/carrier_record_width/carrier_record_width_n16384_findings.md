# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_rec32 shows alternating (throttle bounce) (autocorr -0.61)

carrier_rec32's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7.57 us) is smaller than the fastest variant's own run-to-run std-dev (13.38 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 5 variants sit between 2.51 ms and 2.52 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_rec20's edge over baseline is significant but tiny (37 ns, 0.00%)

carrier_rec20 differs from baseline carrier_rec24 by 37 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_rec20** at 2512377.0 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 2512377.0 ns, slowest 2519946.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 2530282ns | 2523497ns | 2511026ns | 2520405ns | 2554725ns | +0.49% |
| carrier_rec16 | 2524758ns | 2519321ns | 2489622ns | 2514715ns | 2557391ns | +0.27% |
| carrier_rec20 | 2520595ns | 2515887ns | 2506944ns | 2513122ns | 2538629ns | +0.11% |
| carrier_rec24 | 2517928ns | 2520756ns | 2491150ns | 2519590ns | 2528823ns | base |
| carrier_rec32 | 2526436ns | 2517876ns | 2504103ns | 2513626ns | 2556817ns | +0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 2526584ns | 2507340ns | 2550871ns | +0.49% | 0.006 |
| carrier_rec16 | 2521337ns | 2485789ns | 2554290ns | +0.28% | 0.006 |
| carrier_rec20 | 2517030ns | 2503441ns | 2534997ns | +0.11% | 0.007 |
| carrier_rec24 | 2514337ns | 2487416ns | 2525246ns | base | 0.007 |
| carrier_rec32 | 2522904ns | 2500365ns | 2553662ns | +0.34% | 0.006 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_rec16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.007 | 98.6% |
| carrier_rec16 | 0.007 | 98.8% |
| carrier_rec20 | 0.007 | 98.9% |
| carrier_rec24 | 0.007 | 98.8% |
| carrier_rec32 | 0.007 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 2530282ns | 2530282ns | +0.49% |
| carrier_rec16 | 2524758ns | 2524758ns | +0.27% |
| carrier_rec20 | 2520595ns | 2520595ns | +0.11% |
| carrier_rec24 | 2517928ns | 2517928ns | base |
| carrier_rec32 | 2526436ns | 2526436ns | +0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 2516969ns | base | --- | [2500795, 2525246] | --- | --- | --- | --- |
| carrier_rec12 | 2519946ns | no significant difference | [-5420, +25941]ns | [2508934, 2550871] | no | 1.0000 | 0.6875 | 0 |
| carrier_rec16 | 2515900ns | no significant difference | [-22212, +46223]ns | [2493822, 2554290] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec20 | 2512377ns | no significant difference | [-13251, +21296]ns | [2503718, 2534997] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec32 | 2514192ns | no significant difference | [-24387, +36693]ns | [2500859, 2553662] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 2487416ns | +0.8% | +3.6% | +0.8% | +0.8% |
| 2 | 2516043ns | -0.2% | -0.3% | -0.5% | +2.0% |
| 3 | 2519541ns | +1.3% | +0.1% | -0.1% | -0.8% |
| 4 | 2517895ns | -0.2% | -1.3% | -0.6% | +0.9% |
| 5 | 2514174ns | +0.5% | -0.5% | +0.9% | +0.3% |
| 6 | 2530951ns | +0.8% | +0.1% | +0.1% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | -0.216 | moderate- |
| carrier_rec16 | -0.064 | ok |
| carrier_rec20 | 0.201 | moderate+ |
| carrier_rec24 | -0.021 | ok |
| carrier_rec32 | -0.606 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_rec12**: won 2/6, lost 4/6
- **carrier_rec16**: won 3/6, lost 2/6
- **carrier_rec20**: won 3/6, lost 3/6
- **carrier_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 2243.9ns | 2526584.0ns | 0.1% |  |
| carrier_rec16 | 2299.2ns | 2521337.4ns | 0.1% |  |
| carrier_rec20 | 1953.4ns | 2517030.5ns | 0.1% |  |
| carrier_rec24 | 2250.6ns | 2514336.6ns | 0.1% |  |
| carrier_rec32 | 2134.3ns | 2522904.3ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 2507340.4-2550871.3 ns)
  2507340.4 |########################################
  2509516.9 |########################################
  2511693.5 |########################################
  2513870.0 |
  2516046.6 |
  2518223.1 |
  2520399.7 |
  2522576.2 |
  2524752.8 |
  2526929.3 |########################################
  2529105.8 |
  2531282.4 |
  2533458.9 |
  2535635.5 |
  2537812.0 |
  2539988.6 |
  2542165.1 |
  2544341.7 |
  2546518.2 |
  2548694.8 |########################################
  (0 below, 1 above range)

carrier_rec16 (n=6, range 2485788.8-2554289.5 ns)
  2485788.8 |########################################
  2489213.8 |
  2492638.9 |
  2496063.9 |
  2499488.9 |########################################
  2502914.0 |
  2506339.0 |########################################
  2509764.1 |
  2513189.1 |
  2516614.1 |
  2520039.2 |########################################
  2523464.2 |
  2526889.2 |
  2530314.3 |########################################
  2533739.3 |
  2537164.4 |
  2540589.4 |
  2544014.4 |
  2547439.5 |
  2550864.5 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 2503441.2-2534996.7 ns)
  2503441.2 |########################################
  2505019.0 |
  2506596.7 |
  2508174.5 |####################
  2509752.3 |
  2511330.1 |
  2512907.8 |
  2514485.6 |
  2516063.4 |####################
  2517641.2 |
  2519218.9 |
  2520796.7 |
  2522374.5 |
  2523952.2 |
  2525530.0 |
  2527107.8 |
  2528685.6 |
  2530263.3 |
  2531841.1 |
  2533418.9 |####################
  (0 below, 1 above range)

carrier_rec24 (n=6, range 2487415.8-2525246.0 ns)
  2487415.8 |####################
  2489307.3 |
  2491198.8 |
  2493090.3 |
  2494981.8 |
  2496873.3 |
  2498764.9 |
  2500656.4 |
  2502547.9 |
  2504439.4 |
  2506330.9 |
  2508222.4 |
  2510113.9 |
  2512005.4 |
  2513896.9 |####################
  2515788.5 |####################
  2517680.0 |########################################
  2519571.5 |
  2521463.0 |
  2523354.5 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 2500365.4-2553661.9 ns)
  2500365.4 |########################################
  2503030.2 |
  2505695.0 |####################
  2508359.9 |
  2511024.7 |
  2513689.5 |
  2516354.3 |
  2519019.2 |
  2521684.0 |####################
  2524348.8 |
  2527013.6 |
  2529678.4 |
  2532343.3 |
  2535008.1 |
  2537672.9 |
  2540337.7 |####################
  2543002.6 |
  2545667.4 |
  2548332.2 |
  2550997.0 |
  (0 below, 1 above range)

```
