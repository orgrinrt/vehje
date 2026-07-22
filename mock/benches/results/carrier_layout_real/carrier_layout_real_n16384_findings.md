# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.5% of the fastest

All 5 variants sit between 2.59 ms and 2.60 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_real_rec12** at 2591637.1 ns median (-0.2% vs baseline)
- Spread: 1.01x (fastest 2591637.1 ns, slowest 2604599.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 2593583ns | 2594731ns | 2580513ns | 2592345ns | 2601976ns | -0.29% |
| carrier_lay_real_rec16 | 2607342ns | 2607746ns | 2584350ns | 2607257ns | 2618967ns | +0.24% |
| carrier_lay_real_rec20 | 2595905ns | 2603089ns | 2557075ns | 2597192ns | 2613388ns | -0.20% |
| carrier_lay_real_rec24 | 2601226ns | 2601501ns | 2578612ns | 2597004ns | 2618865ns | base |
| carrier_lay_real_rec32 | 2604050ns | 2606093ns | 2581411ns | 2603434ns | 2616293ns | +0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 2590449ns | 2577585ns | 2598726ns | -0.29% | 0.006 |
| carrier_lay_real_rec16 | 2604000ns | 2581250ns | 2615377ns | +0.24% | 0.006 |
| carrier_lay_real_rec20 | 2592232ns | 2553688ns | 2609620ns | -0.22% | 0.006 |
| carrier_lay_real_rec24 | 2597859ns | 2575725ns | 2615490ns | base | 0.006 |
| carrier_lay_real_rec32 | 2600859ns | 2578728ns | 2612867ns | +0.12% | 0.006 |

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_lay_real_rec20; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.006 | 98.5% |
| carrier_lay_real_rec16 | 0.006 | 98.0% |
| carrier_lay_real_rec20 | 0.006 | 98.2% |
| carrier_lay_real_rec24 | 0.006 | 98.3% |
| carrier_lay_real_rec32 | 0.006 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 2593583ns | 2593583ns | -0.29% |
| carrier_lay_real_rec16 | 2607342ns | 2607342ns | +0.24% |
| carrier_lay_real_rec20 | 2595905ns | 2595905ns | -0.20% |
| carrier_lay_real_rec24 | 2601226ns | 2601226ns | base |
| carrier_lay_real_rec32 | 2604050ns | 2604050ns | +0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 2598010ns | base | --- | [2580076, 2615490] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 2591637ns | no significant difference | [-22496, +6876]ns | [2580983, 2598726] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec16 | 2604599ns | no significant difference | [-14362, +25455]ns | [2592023, 2615377] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_real_rec20 | 2599354ns | no significant difference | [-37913, +22174]ns | [2567722, 2609620] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec32 | 2602889ns | no significant difference | [-21576, +24729]ns | [2586820, 2612867] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 2590463ns | +0.2% | +0.5% | +1.0% | +0.4% |
| 2 | 2605557ns | -0.6% | +0.0% | -0.3% | -0.0% |
| 3 | 2610257ns | -1.0% | -1.1% | -2.2% | +0.0% |
| 4 | 2575725ns | +0.1% | +1.1% | +0.2% | +0.7% |
| 5 | 2584428ns | +0.3% | +0.9% | +0.7% | +1.2% |
| 6 | 2620722ns | -0.7% | +0.1% | -0.7% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | 0.205 | moderate+ |
| carrier_lay_real_rec16 | 0.066 | ok |
| carrier_lay_real_rec20 | 0.133 | ok |
| carrier_lay_real_rec24 | -0.168 | ok |
| carrier_lay_real_rec32 | -0.494 | moderate- |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 3/6, lost 2/6
- **carrier_lay_real_rec16**: won 1/6, lost 3/6
- **carrier_lay_real_rec20**: won 3/6, lost 3/6
- **carrier_lay_real_rec32**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 2593303.7ns | 2590448.6ns | 100.1% | HIGH |
| carrier_lay_real_rec16 | 2607901.9ns | 2603999.6ns | 100.1% | HIGH |
| carrier_lay_real_rec20 | 2596116.5ns | 2592232.0ns | 100.1% | HIGH |
| carrier_lay_real_rec24 | 2600308.1ns | 2597858.7ns | 100.1% | HIGH |
| carrier_lay_real_rec32 | 2601064.9ns | 2600858.5ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 2577585.4-2598725.9 ns)
  2577585.4 |########################################
  2578642.4 |
  2579699.4 |
  2580756.5 |
  2581813.5 |
  2582870.5 |
  2583927.5 |########################################
  2584984.6 |
  2586041.6 |
  2587098.6 |
  2588155.6 |
  2589212.6 |
  2590269.7 |########################################
  2591326.7 |
  2592383.7 |########################################
  2593440.7 |
  2594497.8 |
  2595554.8 |########################################
  2596611.8 |
  2597668.8 |
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 2581250.0-2615376.9 ns)
  2581250.0 |####################
  2582956.3 |
  2584662.7 |
  2586369.0 |
  2588075.4 |
  2589781.7 |
  2591488.1 |
  2593194.4 |
  2594900.8 |
  2596607.1 |
  2598313.5 |
  2600019.8 |
  2601726.1 |########################################
  2603432.5 |
  2605138.8 |####################
  2606845.2 |####################
  2608551.5 |
  2610257.9 |
  2611964.2 |
  2613670.6 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 2553688.3-2609619.5 ns)
  2553688.3 |####################
  2556484.9 |
  2559281.4 |
  2562078.0 |
  2564874.5 |
  2567671.1 |
  2570467.7 |
  2573264.2 |
  2576060.8 |
  2578857.4 |
  2581653.9 |####################
  2584450.5 |
  2587247.0 |
  2590043.6 |
  2592840.2 |
  2595636.7 |####################
  2598433.3 |
  2601229.9 |########################################
  2604026.4 |
  2606823.0 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 2575724.6-2615489.8 ns)
  2575724.6 |########################################
  2577712.9 |
  2579701.1 |
  2581689.4 |
  2583677.6 |########################################
  2585665.9 |
  2587654.2 |
  2589642.4 |########################################
  2591630.7 |
  2593618.9 |
  2595607.2 |
  2597595.5 |
  2599583.7 |
  2601572.0 |
  2603560.2 |
  2605548.5 |########################################
  2607536.8 |
  2609525.0 |########################################
  2611513.3 |
  2613501.5 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 2578728.3-2612866.7 ns)
  2578728.3 |########################################
  2580435.2 |
  2582142.1 |
  2583849.1 |
  2585556.0 |
  2587262.9 |
  2588969.8 |
  2590676.7 |
  2592383.6 |
  2594090.6 |########################################
  2595797.5 |
  2597504.4 |
  2599211.3 |
  2600918.2 |########################################
  2602625.1 |
  2604332.1 |########################################
  2606039.0 |
  2607745.9 |
  2609452.8 |########################################
  2611159.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=100.0% of algo (FFI overhead may distort results)
