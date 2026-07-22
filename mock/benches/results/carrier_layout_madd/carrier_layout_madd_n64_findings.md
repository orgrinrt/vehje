# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_lay_madd_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_lay_madd_rec24 has the worst median (2.73 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_lay_madd_rec12 at 2.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_lay_madd_rec12 is fastest but the noisiest (CV 5.3%)

carrier_lay_madd_rec12 wins on median (2.66 us) yet has the highest variance (CV 5.3%), while carrier_lay_madd_rec20 is the steadiest (CV 1.1%, 2.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_lay_madd_rec16 shows alternating (throttle bounce) (autocorr -0.65)

carrier_lay_madd_rec16's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (66 ns) is smaller than the fastest variant's own run-to-run std-dev (142 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_madd_rec12 vs stability leader carrier_lay_madd_rec20 (+1% speed for 5.0x steadier)

carrier_lay_madd_rec12 is fastest (2.66 us, CV 5.3%); carrier_lay_madd_rec20 gives up 0.6% median for 5.0x lower variance (CV 1.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.5% of the fastest

All 5 variants sit between 2.66 us and 2.73 us - a 2.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_madd_rec16's edge over baseline is significant but tiny (-46 ns, 1.69%)

carrier_lay_madd_rec16 differs from baseline carrier_lay_madd_rec24 by -46 ns (1.69%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_madd_rec12** at 2663.9 ns median (-2.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.02x (fastest 2663.9 ns, slowest 2730.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 5061ns | 5150ns | 4526ns | 5088ns | 5288ns | -4.39% |
| carrier_lay_madd_rec16 | 5333ns | 5353ns | 5155ns | 5339ns | 5414ns | +0.75% |
| carrier_lay_madd_rec20 | 5297ns | 5302ns | 5237ns | 5282ns | 5350ns | +0.07% |
| carrier_lay_madd_rec24 | 5293ns | 5292ns | 5136ns | 5283ns | 5388ns | base |
| carrier_lay_madd_rec32 | 5282ns | 5260ns | 5204ns | 5243ns | 5380ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 2593ns | 2294ns | 2690ns | -5.14% | 0.025 |
| carrier_lay_madd_rec16 | 2706ns | 2630ns | 2756ns | -0.99% | 0.024 |
| carrier_lay_madd_rec20 | 2686ns | 2654ns | 2717ns | -1.74% | 0.024 |
| carrier_lay_madd_rec24 | 2733ns | 2678ns | 2789ns | base | 0.023 |
| carrier_lay_madd_rec32 | 2700ns | 2655ns | 2739ns | -1.22% | 0.024 |

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_lay_madd_rec12; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.024 | 86.1% |
| carrier_lay_madd_rec16 | 0.024 | 84.4% |
| carrier_lay_madd_rec20 | 0.024 | 85.6% |
| carrier_lay_madd_rec24 | 0.023 | 84.0% |
| carrier_lay_madd_rec32 | 0.024 | 85.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 5061ns | 5061ns | -4.39% |
| carrier_lay_madd_rec16 | 5333ns | 5333ns | +0.75% |
| carrier_lay_madd_rec20 | 5297ns | 5297ns | +0.07% |
| carrier_lay_madd_rec24 | 5293ns | 5293ns | base |
| carrier_lay_madd_rec32 | 5282ns | 5282ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 2730ns | base | --- | [2680, 2789] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 2664ns | -102.1ns (-3.7%) | [-257, -62]ns | [2425, 2690] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_madd_rec16 | 2719ns | no significant difference | [-83, +47]ns | [2643, 2756] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 2679ns | no significant difference | [-113, +19]ns | [2661, 2717] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_madd_rec32 | 2694ns | no significant difference | [-78, +12]ns | [2666, 2739] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 2683ns | -14.5% | -2.0% | -1.1% | -0.2% |
| 2 | 2678ns | -4.6% | +2.7% | +2.5% | +0.5% |
| 3 | 2719ns | -2.1% | -2.3% | -1.7% | -2.3% |
| 4 | 2741ns | -2.5% | +0.8% | -1.8% | +0.4% |
| 5 | 2790ns | -2.9% | -3.6% | -4.4% | -2.2% |
| 6 | 2789ns | -4.5% | -1.4% | -3.7% | -3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | 0.262 | moderate+ |
| carrier_lay_madd_rec16 | -0.652 | HIGH- (thermal bounce) |
| carrier_lay_madd_rec20 | -0.561 | HIGH- (thermal bounce) |
| carrier_lay_madd_rec24 | 0.579 | HIGH+ (drift/warm-up) |
| carrier_lay_madd_rec32 | -0.062 | ok |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 6/6, lost 0/6
- **carrier_lay_madd_rec16**: won 4/6, lost 2/6
- **carrier_lay_madd_rec20**: won 5/6, lost 1/6
- **carrier_lay_madd_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 86403.9ns | 2592.8ns | 3332.5% | HIGH |
| carrier_lay_madd_rec16 | 86188.2ns | 2706.0ns | 3185.0% | HIGH |
| carrier_lay_madd_rec20 | 86571.8ns | 2685.8ns | 3223.4% | HIGH |
| carrier_lay_madd_rec24 | 86605.0ns | 2733.2ns | 3168.6% | HIGH |
| carrier_lay_madd_rec32 | 86526.9ns | 2699.8ns | 3204.9% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 2294.2-2689.8 ns)
   2294.2 |####################
   2314.0 |
   2333.8 |
   2353.5 |
   2373.3 |
   2393.1 |
   2412.9 |
   2432.7 |
   2452.4 |
   2472.2 |
   2492.0 |
   2511.8 |
   2531.6 |
   2551.3 |####################
   2571.1 |
   2590.9 |
   2610.7 |
   2630.5 |
   2650.2 |########################################
   2670.0 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 2630.0-2756.2 ns)
   2630.0 |####################
   2636.3 |
   2642.6 |
   2648.9 |
   2655.2 |####################
   2661.6 |
   2667.9 |
   2674.2 |
   2680.5 |
   2686.8 |####################
   2693.1 |
   2699.4 |
   2705.8 |
   2712.1 |
   2718.4 |
   2724.7 |
   2731.0 |
   2737.3 |
   2743.6 |
   2749.9 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 2654.2-2716.9 ns)
   2654.2 |########################################
   2657.3 |
   2660.5 |
   2663.6 |
   2666.7 |########################################
   2669.9 |########################################
   2673.0 |
   2676.1 |
   2679.3 |
   2682.4 |
   2685.5 |########################################
   2688.7 |########################################
   2691.8 |
   2694.9 |
   2698.1 |
   2701.2 |
   2704.3 |
   2707.5 |
   2710.6 |
   2713.7 |
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 2677.5-2789.4 ns)
   2677.5 |########################################
   2683.1 |
   2688.7 |
   2694.3 |
   2699.9 |
   2705.5 |
   2711.1 |
   2716.7 |####################
   2722.3 |
   2727.9 |
   2733.4 |
   2739.0 |####################
   2744.6 |
   2750.2 |
   2755.8 |
   2761.4 |
   2767.0 |
   2772.6 |
   2778.2 |
   2783.8 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 2655.4-2739.2 ns)
   2655.4 |########################################
   2659.6 |
   2663.8 |
   2668.0 |
   2672.2 |
   2676.3 |########################################
   2680.5 |
   2684.7 |
   2688.9 |########################################
   2693.1 |########################################
   2697.3 |
   2701.5 |
   2705.7 |
   2709.8 |
   2714.0 |
   2718.2 |
   2722.4 |
   2726.6 |########################################
   2730.8 |
   2735.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=3246.5% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=3170.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=3231.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **carrier_lay_madd_rec24**: bridge=3175.1% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=3207.5% of algo (FFI overhead may distort results)
