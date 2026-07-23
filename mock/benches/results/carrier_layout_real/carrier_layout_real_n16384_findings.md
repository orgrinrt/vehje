# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.7% of the fastest

All 5 variants sit between 2.62 ms and 2.64 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_real_rec20** at 2615992.5 ns median (-0.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 2615992.5 ns, slowest 2635034.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 2625888ns | 2622962ns | 2619165ns | 2621811ns | 2635364ns | -0.33% |
| carrier_lay_real_rec16 | 2631895ns | 2638892ns | 2599631ns | 2633934ns | 2644970ns | -0.10% |
| carrier_lay_real_rec20 | 2617838ns | 2619204ns | 2595455ns | 2612813ns | 2636567ns | -0.64% |
| carrier_lay_real_rec24 | 2634585ns | 2634348ns | 2625514ns | 2632194ns | 2642706ns | base |
| carrier_lay_real_rec32 | 2633387ns | 2630070ns | 2613886ns | 2629258ns | 2649331ns | -0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 2622771ns | 2615472ns | 2632646ns | -0.32% | 0.006 |
| carrier_lay_real_rec16 | 2628388ns | 2597058ns | 2641511ns | -0.10% | 0.006 |
| carrier_lay_real_rec20 | 2614630ns | 2593032ns | 2632492ns | -0.63% | 0.006 |
| carrier_lay_real_rec24 | 2631096ns | 2621484ns | 2639143ns | base | 0.006 |
| carrier_lay_real_rec32 | 2629951ns | 2610310ns | 2646225ns | -0.04% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 16428351 | 18505651 | 0.888 | 1.00× |
| carrier_lay_real_rec16 | 16463242 | 18291792 | 0.900 | 1.00× |
| carrier_lay_real_rec20 | 16401079 | 18289390 | 0.897 | 1.00× |
| carrier_lay_real_rec24 | 16474287 | 18293265 | 0.901 | 1.00× |
| carrier_lay_real_rec32 | 16472664 | 18292434 | 0.901 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_lay_real_rec20; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.006 | 99.0% |
| carrier_lay_real_rec16 | 0.006 | 98.4% |
| carrier_lay_real_rec20 | 0.006 | 99.1% |
| carrier_lay_real_rec24 | 0.006 | 98.6% |
| carrier_lay_real_rec32 | 0.006 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 2625888ns | 2625888ns | -0.33% |
| carrier_lay_real_rec16 | 2631895ns | 2631895ns | -0.10% |
| carrier_lay_real_rec20 | 2617838ns | 2617838ns | -0.64% |
| carrier_lay_real_rec24 | 2634585ns | 2634585ns | base |
| carrier_lay_real_rec32 | 2633387ns | 2633387ns | -0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 2631116ns | base | --- | [2623029, 2639143] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 2619633ns | no significant difference | [-19923, +3929]ns | [2616032, 2632646] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_real_rec16 | 2635035ns | no significant difference | [-21285, +12427]ns | [2608617, 2641511] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec20 | 2615992ns | -11179.4ns (-0.4%) | [-33598, -4620]ns | [2595406, 2632492] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_real_rec32 | 2626302ns | no significant difference | [-15841, +13079]ns | [2617327, 2646225] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 2629372ns | -0.5% | +0.1% | -0.3% | -0.2% |
| 2 | 2621484ns | -0.2% | -0.0% | -0.3% | -0.4% |
| 3 | 2633434ns | -0.6% | -1.4% | -1.4% | +0.3% |
| 4 | 2632860ns | +0.2% | +0.4% | -0.5% | +0.7% |
| 5 | 2624574ns | +0.1% | +0.5% | -1.2% | +0.1% |
| 6 | 2644852ns | -0.9% | -0.2% | -0.1% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | 0.086 | ok |
| carrier_lay_real_rec16 | -0.016 | ok |
| carrier_lay_real_rec20 | -0.471 | moderate- |
| carrier_lay_real_rec24 | -0.307 | moderate- |
| carrier_lay_real_rec32 | 0.084 | ok |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 4/6, lost 1/6
- **carrier_lay_real_rec16**: won 2/6, lost 3/6
- **carrier_lay_real_rec20**: won 5/6, lost 0/6
- **carrier_lay_real_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 2624583.8ns | 2622770.5ns | 100.1% | HIGH |
| carrier_lay_real_rec16 | 2630739.7ns | 2628387.6ns | 100.1% | HIGH |
| carrier_lay_real_rec20 | 2615622.7ns | 2614630.4ns | 100.0% | HIGH |
| carrier_lay_real_rec24 | 2633105.9ns | 2631095.9ns | 100.1% | HIGH |
| carrier_lay_real_rec32 | 2631360.0ns | 2629951.3ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 2615472.1-2632646.0 ns)
  2615472.1 |########################################
  2616330.8 |########################################
  2617189.5 |########################################
  2618048.2 |
  2618906.9 |
  2619765.6 |
  2620624.3 |
  2621483.0 |########################################
  2622341.7 |
  2623200.4 |
  2624059.1 |
  2624917.8 |
  2625776.5 |########################################
  2626635.2 |
  2627493.9 |
  2628352.6 |
  2629211.3 |
  2630070.0 |
  2630928.7 |
  2631787.4 |
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 2597057.9-2641511.0 ns)
  2597057.9 |####################
  2599280.6 |
  2601503.2 |
  2603725.9 |
  2605948.5 |
  2608171.2 |
  2610393.8 |
  2612616.5 |
  2614839.2 |
  2617061.8 |
  2619284.5 |####################
  2621507.1 |
  2623729.8 |
  2625952.4 |
  2628175.1 |
  2630397.8 |####################
  2632620.4 |
  2634843.1 |
  2637065.7 |########################################
  2639288.4 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 2593032.1-2632492.1 ns)
  2593032.1 |########################################
  2595005.1 |
  2596978.1 |########################################
  2598951.1 |
  2600924.1 |
  2602897.1 |
  2604870.1 |
  2606843.1 |
  2608816.1 |
  2610789.1 |
  2612762.1 |########################################
  2614735.1 |
  2616708.1 |
  2618681.1 |########################################
  2620654.1 |########################################
  2622627.1 |
  2624600.1 |
  2626573.1 |
  2628546.1 |
  2630519.1 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 2621484.2-2639143.2 ns)
  2621484.2 |########################################
  2622367.1 |
  2623250.1 |
  2624133.0 |########################################
  2625016.0 |
  2625898.9 |
  2626781.9 |
  2627664.8 |
  2628547.8 |########################################
  2629430.7 |
  2630313.7 |
  2631196.6 |
  2632079.6 |########################################
  2632962.5 |########################################
  2633845.5 |
  2634728.4 |
  2635611.4 |
  2636494.3 |
  2637377.3 |
  2638260.2 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 2610309.6-2646225.5 ns)
  2610309.6 |########################################
  2612105.4 |
  2613901.2 |
  2615697.0 |
  2617492.8 |
  2619288.6 |
  2621084.4 |
  2622880.1 |########################################
  2624675.9 |########################################
  2626471.7 |########################################
  2628267.5 |
  2630063.3 |
  2631859.1 |
  2633654.9 |
  2635450.7 |
  2637246.5 |
  2639042.3 |
  2640838.1 |########################################
  2642633.9 |
  2644429.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=100.1% of algo (FFI overhead may distort results)
