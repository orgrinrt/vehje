# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_lay_scatter_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_lay_scatter_rec24 has the worst median (2.61 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_lay_scatter_rec16 at 2.59 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.8% of the fastest

All 5 variants sit between 2.59 ms and 2.61 ms - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_scatter_rec16** at 2585844.8 ns median (-0.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 2585844.8 ns, slowest 2606013.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2591017ns | 2591550ns | 2577028ns | 2588828ns | 2601295ns | -2.82% |
| carrier_lay_scatter_rec16 | 2593346ns | 2589066ns | 2579112ns | 2586908ns | 2610120ns | -2.73% |
| carrier_lay_scatter_rec20 | 2605063ns | 2606253ns | 2585162ns | 2600009ns | 2622594ns | -2.29% |
| carrier_lay_scatter_rec24 | 2666196ns | 2609462ns | 2588445ns | 2603180ns | 2799596ns | base |
| carrier_lay_scatter_rec32 | 2598448ns | 2590168ns | 2582526ns | 2587935ns | 2622177ns | -2.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2587726ns | 2574243ns | 2597801ns | -2.83% | 0.006 |
| carrier_lay_scatter_rec16 | 2590268ns | 2575925ns | 2607446ns | -2.73% | 0.006 |
| carrier_lay_scatter_rec20 | 2601619ns | 2581870ns | 2619107ns | -2.30% | 0.006 |
| carrier_lay_scatter_rec24 | 2662980ns | 2585877ns | 2796162ns | base | 0.006 |
| carrier_lay_scatter_rec32 | 2595172ns | 2579741ns | 2619190ns | -2.55% | 0.006 |

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_lay_scatter_rec12; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.006 | 99.5% |
| carrier_lay_scatter_rec16 | 0.006 | 99.6% |
| carrier_lay_scatter_rec20 | 0.006 | 98.9% |
| carrier_lay_scatter_rec24 | 0.006 | 98.8% |
| carrier_lay_scatter_rec32 | 0.006 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 2591017ns | 2591017ns | -2.82% |
| carrier_lay_scatter_rec16 | 2593346ns | 2593346ns | -2.73% |
| carrier_lay_scatter_rec20 | 2605063ns | 2605063ns | -2.29% |
| carrier_lay_scatter_rec24 | 2666196ns | 2666196ns | base |
| carrier_lay_scatter_rec32 | 2598448ns | 2598448ns | -2.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 2606014ns | base | --- | [2586763, 2796162] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 2588029ns | -14436.0ns (-0.6%) | [-201910, -9415]ns | [2577348, 2597801] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_scatter_rec16 | 2585845ns | -17257.8ns (-0.7%) | [-198980, -1896]ns | [2577515, 2607446] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_lay_scatter_rec20 | 2602747ns | no significant difference | [-188843, +8520]ns | [2583004, 2619107] | no | 0.2188 | 0.2188 | 0 |
| carrier_lay_scatter_rec32 | 2586388ns | no significant difference | [-185010, +4418]ns | [2579937, 2619190] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 2587649ns | -0.3% | -0.5% | -0.2% | -0.3% |
| 2 | 2614083ns | -0.6% | -1.3% | -0.8% | -1.1% |
| 3 | 2598173ns | -0.5% | +0.1% | +0.7% | -0.7% |
| 4 | 2613854ns | -0.6% | -0.9% | -0.0% | -1.0% |
| 5 | 2978242ns | -13.0% | -12.2% | -12.0% | -11.5% |
| 6 | 2585877ns | -0.4% | -0.3% | -0.1% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.294 | moderate- |
| carrier_lay_scatter_rec16 | -0.184 | ok |
| carrier_lay_scatter_rec20 | 0.053 | ok |
| carrier_lay_scatter_rec24 | -0.248 | moderate- |
| carrier_lay_scatter_rec32 | 0.147 | ok |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 6/6, lost 0/6
- **carrier_lay_scatter_rec16**: won 5/6, lost 1/6
- **carrier_lay_scatter_rec20**: won 3/6, lost 1/6
- **carrier_lay_scatter_rec32**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 2591971.9ns | 2587726.2ns | 100.2% | HIGH |
| carrier_lay_scatter_rec16 | 2594561.9ns | 2590268.4ns | 100.2% | HIGH |
| carrier_lay_scatter_rec20 | 2605173.4ns | 2601619.0ns | 100.1% | HIGH |
| carrier_lay_scatter_rec24 | 2654909.6ns | 2662979.7ns | 99.7% | HIGH |
| carrier_lay_scatter_rec32 | 2598512.1ns | 2595171.5ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 2574242.9-2597801.5 ns)
  2574242.9 |########################################
  2575420.8 |
  2576598.8 |
  2577776.7 |
  2578954.6 |
  2580132.5 |########################################
  2581310.5 |
  2582488.4 |
  2583666.3 |
  2584844.2 |########################################
  2586022.2 |
  2587200.1 |
  2588378.0 |
  2589556.0 |########################################
  2590733.9 |
  2591911.8 |
  2593089.7 |
  2594267.7 |
  2595445.6 |
  2596623.5 |########################################
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 2575925.0-2607445.6 ns)
  2575925.0 |####################
  2577501.0 |
  2579077.1 |########################################
  2580653.1 |
  2582229.1 |
  2583805.2 |
  2585381.2 |
  2586957.2 |
  2588533.3 |
  2590109.3 |####################
  2591685.3 |
  2593261.4 |
  2594837.4 |
  2596413.4 |
  2597989.5 |
  2599565.5 |
  2601141.5 |####################
  2602717.6 |
  2604293.6 |
  2605869.6 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 2581870.0-2619106.7 ns)
  2581870.0 |########################################
  2583731.8 |########################################
  2585593.7 |
  2587455.5 |
  2589317.3 |
  2591179.2 |########################################
  2593041.0 |
  2594902.8 |
  2596764.7 |
  2598626.5 |
  2600488.3 |
  2602350.2 |
  2604212.0 |
  2606073.8 |
  2607935.7 |
  2609797.5 |
  2611659.3 |########################################
  2613521.2 |
  2615383.0 |########################################
  2617244.8 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 2585876.7-2796162.5 ns)
  2585876.7 |########################################
  2596391.0 |####################
  2606905.3 |########################################
  2617419.6 |
  2627933.9 |
  2638448.2 |
  2648962.4 |
  2659476.7 |
  2669991.0 |
  2680505.3 |
  2691019.6 |
  2701533.9 |
  2712048.2 |
  2722562.5 |
  2733076.8 |
  2743591.0 |
  2754105.3 |
  2764619.6 |
  2775133.9 |
  2785648.2 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 2579741.2-2619190.4 ns)
  2579741.2 |########################################
  2581713.7 |
  2583686.1 |
  2585658.6 |########################################
  2587631.0 |
  2589603.5 |
  2591576.0 |
  2593548.4 |
  2595520.9 |
  2597493.3 |
  2599465.8 |
  2601438.3 |####################
  2603410.7 |
  2605383.2 |
  2607355.6 |
  2609328.1 |
  2611300.6 |
  2613273.0 |
  2615245.5 |
  2617217.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=100.1% of algo (FFI overhead may distort results)
