# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_wideselect_rec24 shows alternating (throttle bounce) (autocorr -0.82)

carrier_lay_wideselect_rec24's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_lay_wideselect_rec12 vs stability leader carrier_lay_wideselect_rec32 (+4% speed for 2.0x steadier)

carrier_lay_wideselect_rec12 is fastest (448.01 us, CV 3.7%); carrier_lay_wideselect_rec32 gives up 4.5% median for 2.0x lower variance (CV 1.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_lay_wideselect_rec12** at 448011.7 ns median (-3.7% vs baseline)
- Spread: 1.07x (fastest 448011.7 ns, slowest 479550.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 453130ns | 451160ns | 431682ns | 446264ns | 474154ns | -2.91% |
| carrier_lay_wideselect_rec16 | 475672ns | 482366ns | 448922ns | 474244ns | 491189ns | +1.92% |
| carrier_lay_wideselect_rec20 | 472634ns | 475484ns | 453242ns | 471257ns | 484397ns | +1.27% |
| carrier_lay_wideselect_rec24 | 466726ns | 468524ns | 448002ns | 463896ns | 480332ns | base |
| carrier_lay_wideselect_rec32 | 473069ns | 471175ns | 463936ns | 469256ns | 483355ns | +1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 450158ns | 428650ns | 471220ns | -2.93% | 0.009 |
| carrier_lay_wideselect_rec16 | 472738ns | 445798ns | 488105ns | +1.94% | 0.009 |
| carrier_lay_wideselect_rec20 | 469540ns | 450131ns | 481251ns | +1.25% | 0.009 |
| carrier_lay_wideselect_rec24 | 463759ns | 444765ns | 477446ns | base | 0.009 |
| carrier_lay_wideselect_rec32 | 470021ns | 460825ns | 480496ns | +1.35% | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_lay_wideselect_rec12; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.009 | 95.7% |
| carrier_lay_wideselect_rec16 | 0.009 | 89.4% |
| carrier_lay_wideselect_rec20 | 0.009 | 90.7% |
| carrier_lay_wideselect_rec24 | 0.009 | 92.1% |
| carrier_lay_wideselect_rec32 | 0.009 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 453130ns | 453130ns | -2.91% |
| carrier_lay_wideselect_rec16 | 475672ns | 475672ns | +1.92% |
| carrier_lay_wideselect_rec20 | 472634ns | 472634ns | +1.27% |
| carrier_lay_wideselect_rec24 | 466726ns | 466726ns | base |
| carrier_lay_wideselect_rec32 | 473069ns | 473069ns | +1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 465400ns | base | --- | [448431, 477446] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 448012ns | no significant difference | [-39936, +10685]ns | [431242, 471220] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec16 | 479551ns | no significant difference | [-16133, +32724]ns | [450558, 488105] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec20 | 472464ns | no significant difference | [-10038, +18075]ns | [454903, 481251] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec32 | 467980ns | no significant difference | [-5258, +16351]ns | [461587, 480496] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 471260ns | -9.0% | +1.6% | +2.8% | -2.2% |
| 2 | 452097ns | -4.0% | +0.7% | +3.9% | +2.3% |
| 3 | 473605ns | -1.0% | +2.7% | +0.3% | -0.0% |
| 4 | 444765ns | +1.6% | +10.1% | +1.2% | +5.0% |
| 5 | 481286ns | -7.7% | -7.4% | -4.5% | +1.3% |
| 6 | 459539ns | +3.1% | +4.5% | +4.0% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | -0.044 | ok |
| carrier_lay_wideselect_rec16 | -0.481 | moderate- |
| carrier_lay_wideselect_rec20 | 0.005 | ok |
| carrier_lay_wideselect_rec24 | -0.817 | HIGH- (thermal bounce) |
| carrier_lay_wideselect_rec32 | -0.080 | ok |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 4/6, lost 2/6
- **carrier_lay_wideselect_rec16**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec20**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec32**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 451321.8ns | 450157.9ns | 100.3% | HIGH |
| carrier_lay_wideselect_rec16 | 473568.9ns | 472738.0ns | 100.2% | HIGH |
| carrier_lay_wideselect_rec20 | 470348.9ns | 469539.6ns | 100.2% | HIGH |
| carrier_lay_wideselect_rec24 | 464817.8ns | 463758.8ns | 100.2% | HIGH |
| carrier_lay_wideselect_rec32 | 469990.5ns | 470021.2ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 428650.4-471219.8 ns)
  428650.4 |########################################
  430778.9 |
  432907.3 |########################################
  435035.8 |
  437164.3 |
  439292.8 |
  441421.2 |
  443549.7 |########################################
  445678.2 |
  447806.6 |
  449935.1 |########################################
  452063.6 |
  454192.0 |
  456320.5 |
  458449.0 |
  460577.5 |
  462705.9 |
  464834.4 |
  466962.9 |########################################
  469091.3 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 445798.3-488105.2 ns)
  445798.3 |########################################
  447913.6 |
  450029.0 |
  452144.3 |
  454259.7 |########################################
  456375.0 |
  458490.4 |
  460605.7 |
  462721.1 |
  464836.4 |
  466951.8 |
  469067.1 |
  471182.5 |
  473297.8 |
  475413.2 |
  477528.5 |########################################
  479643.9 |########################################
  481759.2 |
  483874.6 |
  485989.9 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 450131.2-481251.4 ns)
  450131.2 |########################################
  451687.2 |
  453243.2 |
  454799.2 |
  456355.2 |
  457911.3 |
  459467.3 |########################################
  461023.3 |
  462579.3 |
  464135.3 |
  465691.3 |
  467247.3 |
  468803.3 |########################################
  470359.4 |
  471915.4 |
  473471.4 |
  475027.4 |########################################
  476583.4 |########################################
  478139.4 |
  479695.4 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 444765.4-477445.6 ns)
  444765.4 |########################################
  446399.4 |
  448033.4 |
  449667.4 |
  451301.4 |########################################
  452935.5 |
  454569.5 |
  456203.5 |
  457837.5 |
  459471.5 |########################################
  461105.5 |
  462739.5 |
  464373.5 |
  466007.5 |
  467641.5 |
  469275.5 |
  470909.6 |########################################
  472543.6 |########################################
  474177.6 |
  475811.6 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 460825.0-480496.2 ns)
  460825.0 |########################################
  461808.6 |########################################
  462792.1 |
  463775.7 |
  464759.2 |
  465742.8 |
  466726.4 |########################################
  467709.9 |
  468693.5 |########################################
  469677.1 |
  470660.6 |
  471644.2 |
  472627.8 |########################################
  473611.3 |
  474594.9 |
  475578.4 |
  476562.0 |
  477545.6 |
  478529.1 |
  479512.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=100.0% of algo (FFI overhead may distort results)
