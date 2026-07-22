# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_lay_scatter_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_lay_scatter_rec24 has the worst median (608.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_lay_scatter_rec32 at 576.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_lay_scatter_rec32 is fastest but the noisiest (CV 6.4%)

carrier_lay_scatter_rec32 wins on median (576.15 us) yet has the highest variance (CV 6.4%), while carrier_lay_scatter_rec20 is the steadiest (CV 3.1%, 592.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_lay_scatter_rec16 shows alternating (throttle bounce) (autocorr -0.60)

carrier_lay_scatter_rec16's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (32.05 us) is smaller than the fastest variant's own run-to-run std-dev (36.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_scatter_rec32 vs stability leader carrier_lay_scatter_rec20 (+3% speed for 2.0x steadier)

carrier_lay_scatter_rec32 is fastest (576.15 us, CV 6.4%); carrier_lay_scatter_rec20 gives up 2.9% median for 2.0x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_lay_scatter_rec32** at 576148.3 ns median (-5.3% vs baseline)
- Spread: 1.06x (fastest 576148.3 ns, slowest 608198.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 573350ns | 585723ns | 519685ns | 576326ns | 595719ns | -3.99% |
| carrier_lay_scatter_rec16 | 583852ns | 585959ns | 540422ns | 576458ns | 616658ns | -2.23% |
| carrier_lay_scatter_rec20 | 592291ns | 596142ns | 566480ns | 587929ns | 611741ns | -0.82% |
| carrier_lay_scatter_rec24 | 597161ns | 610884ns | 553003ns | 596845ns | 619715ns | base |
| carrier_lay_scatter_rec32 | 580883ns | 578949ns | 519610ns | 574323ns | 621360ns | -2.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 570344ns | 517065ns | 592461ns | -4.00% | 0.007 |
| carrier_lay_scatter_rec16 | 580947ns | 537850ns | 613738ns | -2.21% | 0.007 |
| carrier_lay_scatter_rec20 | 589482ns | 563837ns | 609070ns | -0.77% | 0.007 |
| carrier_lay_scatter_rec24 | 594079ns | 549603ns | 616369ns | base | 0.007 |
| carrier_lay_scatter_rec32 | 578044ns | 517266ns | 618074ns | -2.70% | 0.007 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_lay_scatter_rec12; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.007 | 88.8% |
| carrier_lay_scatter_rec16 | 0.007 | 88.7% |
| carrier_lay_scatter_rec20 | 0.007 | 87.2% |
| carrier_lay_scatter_rec24 | 0.007 | 85.0% |
| carrier_lay_scatter_rec32 | 0.007 | 89.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 573350ns | 573350ns | -3.99% |
| carrier_lay_scatter_rec16 | 583852ns | 583852ns | -2.23% |
| carrier_lay_scatter_rec20 | 592291ns | 592291ns | -0.82% |
| carrier_lay_scatter_rec24 | 597161ns | 597161ns | base |
| carrier_lay_scatter_rec32 | 580883ns | 580883ns | -2.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 608199ns | base | --- | [557668, 616369] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 582540ns | no significant difference | [-74350, +30936]ns | [536032, 592461] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 582926ns | no significant difference | [-56652, +29455]ns | [546176, 613738] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_scatter_rec20 | 592914ns | no significant difference | [-43920, +45195]ns | [566461, 609070] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_scatter_rec32 | 576148ns | no significant difference | [-47594, +16801]ns | [539909, 618074] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 549603ns | +8.1% | +8.3% | +12.5% | +2.4% |
| 2 | 613777ns | -9.6% | -12.4% | -7.3% | +3.4% |
| 3 | 618962ns | -4.6% | +2.2% | -3.3% | -2.8% |
| 4 | 609410ns | -4.5% | -6.1% | -1.6% | -7.7% |
| 5 | 565733ns | +3.0% | -2.0% | +3.8% | -8.6% |
| 6 | 606987ns | -14.8% | -2.2% | -7.1% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.219 | moderate- |
| carrier_lay_scatter_rec16 | -0.600 | HIGH- (thermal bounce) |
| carrier_lay_scatter_rec20 | -0.308 | moderate- |
| carrier_lay_scatter_rec24 | -0.192 | ok |
| carrier_lay_scatter_rec32 | 0.041 | ok |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 4/6, lost 2/6
- **carrier_lay_scatter_rec16**: won 4/6, lost 2/6
- **carrier_lay_scatter_rec20**: won 4/6, lost 2/6
- **carrier_lay_scatter_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 571601.9ns | 570344.3ns | 100.2% | HIGH |
| carrier_lay_scatter_rec16 | 581394.4ns | 580946.6ns | 100.1% | HIGH |
| carrier_lay_scatter_rec20 | 590010.1ns | 589481.7ns | 100.1% | HIGH |
| carrier_lay_scatter_rec24 | 595098.5ns | 594078.6ns | 100.2% | HIGH |
| carrier_lay_scatter_rec32 | 579186.5ns | 578043.9ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 517064.6-592460.8 ns)
  517064.6 |####################
  520834.4 |
  524604.2 |
  528374.0 |
  532143.8 |
  535913.7 |
  539683.5 |
  543453.3 |
  547223.1 |
  550992.9 |
  554762.7 |####################
  558532.5 |
  562302.3 |
  566072.2 |
  569842.0 |
  573611.8 |
  577381.6 |
  581151.4 |########################################
  584921.2 |
  588691.0 |####################
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 537849.6-613737.7 ns)
  537849.6 |########################################
  541644.0 |
  545438.4 |
  549232.8 |
  553027.2 |########################################
  556821.6 |
  560616.0 |
  564410.4 |
  568204.8 |
  571999.2 |########################################
  575793.6 |
  579588.1 |
  583382.5 |
  587176.9 |
  590971.3 |########################################
  594765.7 |########################################
  598560.1 |
  602354.5 |
  606148.9 |
  609943.3 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 563836.7-609070.2 ns)
  563836.7 |####################
  566098.4 |
  568360.0 |####################
  570621.7 |
  572883.4 |
  575145.1 |
  577406.8 |
  579668.4 |
  581930.1 |
  584191.8 |
  586453.4 |####################
  588715.1 |
  590976.8 |
  593238.5 |
  595500.1 |
  597761.8 |########################################
  600023.5 |
  602285.2 |
  604546.8 |
  606808.5 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 549603.3-616369.2 ns)
  549603.3 |####################
  552941.6 |
  556279.9 |
  559618.2 |
  562956.5 |####################
  566294.8 |
  569633.1 |
  572971.4 |
  576309.7 |
  579648.0 |
  582986.2 |
  586324.5 |
  589662.8 |
  593001.1 |
  596339.4 |
  599677.7 |
  603016.0 |
  606354.3 |########################################
  609692.6 |
  613030.9 |####################
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 517265.8-618074.4 ns)
  517265.8 |########################################
  522306.2 |
  527346.7 |
  532387.1 |
  537427.5 |
  542467.9 |
  547508.4 |
  552548.8 |
  557589.2 |########################################
  562629.6 |########################################
  567670.1 |
  572710.5 |
  577750.9 |
  582791.4 |
  587831.8 |########################################
  592872.2 |
  597912.6 |########################################
  602953.1 |
  607993.5 |
  613033.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=100.4% of algo (FFI overhead may distort results)
