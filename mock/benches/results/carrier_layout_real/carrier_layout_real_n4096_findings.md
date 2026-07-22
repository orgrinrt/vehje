# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_lay_real_rec20, carrier_lay_real_rec32) are a dead heat (<1%)

carrier_lay_real_rec20 (585.32 us) and carrier_lay_real_rec32 (590.61 us) differ by 0.90%, inside the noise, even though the wider field spreads 5.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_lay_real_rec20 shows alternating (throttle bounce) (autocorr -0.66)

carrier_lay_real_rec20's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_lay_real_rec20 vs stability leader carrier_lay_real_rec16 (+6% speed for 2.2x steadier)

carrier_lay_real_rec20 is fastest (585.32 us, CV 3.8%); carrier_lay_real_rec16 gives up 5.7% median for 2.2x lower variance (CV 1.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_lay_real_rec20** at 585322.3 ns median (-1.3% vs baseline)
- Spread: 1.06x (fastest 585322.3 ns, slowest 618582.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 600889ns | 604407ns | 581065ns | 598993ns | 613645ns | +1.45% |
| carrier_lay_real_rec16 | 620251ns | 622129ns | 599296ns | 620779ns | 629936ns | +4.72% |
| carrier_lay_real_rec20 | 587846ns | 588898ns | 551145ns | 583538ns | 612657ns | -0.75% |
| carrier_lay_real_rec24 | 592311ns | 595876ns | 552250ns | 591382ns | 613736ns | base |
| carrier_lay_real_rec32 | 592106ns | 593966ns | 561398ns | 585587ns | 617238ns | -0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 597433ns | 577555ns | 610156ns | +1.41% | 0.007 |
| carrier_lay_real_rec16 | 616887ns | 595734ns | 626990ns | +4.71% | 0.007 |
| carrier_lay_real_rec20 | 584451ns | 548740ns | 608975ns | -0.79% | 0.007 |
| carrier_lay_real_rec24 | 589116ns | 548877ns | 610644ns | base | 0.007 |
| carrier_lay_real_rec32 | 588735ns | 557867ns | 614041ns | -0.06% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_lay_real_rec20; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.007 | 91.3% |
| carrier_lay_real_rec16 | 0.007 | 88.7% |
| carrier_lay_real_rec20 | 0.007 | 93.7% |
| carrier_lay_real_rec24 | 0.007 | 92.5% |
| carrier_lay_real_rec32 | 0.007 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 600889ns | 600889ns | +1.45% |
| carrier_lay_real_rec16 | 620251ns | 620251ns | +4.72% |
| carrier_lay_real_rec20 | 587846ns | 587846ns | -0.75% |
| carrier_lay_real_rec24 | 592311ns | 592311ns | base |
| carrier_lay_real_rec32 | 592106ns | 592106ns | -0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 592971ns | base | --- | [563734, 610644] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 600748ns | no significant difference | [-25533, +46423]ns | [581393, 610156] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec16 | 618583ns | no significant difference | [-2901, +56785]ns | [605089, 626990] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_real_rec20 | 585322ns | no significant difference | [-37663, +24553]ns | [559056, 608975] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec32 | 590610ns | no significant difference | [-36008, +45097]ns | [561552, 614041] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 580087ns | +3.4% | +6.8% | +4.2% | +3.3% |
| 2 | 578590ns | +5.8% | +6.2% | -1.6% | -3.6% |
| 3 | 613290ns | -1.9% | +0.7% | -2.6% | -5.1% |
| 4 | 548877ns | +10.8% | +13.5% | +4.5% | +12.9% |
| 5 | 605854ns | -4.7% | -1.7% | +1.2% | -6.7% |
| 6 | 607999ns | -3.7% | +3.8% | -9.7% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | 0.196 | ok |
| carrier_lay_real_rec16 | -0.618 | HIGH- (thermal bounce) |
| carrier_lay_real_rec20 | -0.656 | HIGH- (thermal bounce) |
| carrier_lay_real_rec24 | -0.491 | moderate- |
| carrier_lay_real_rec32 | -0.504 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 3/6, lost 3/6
- **carrier_lay_real_rec16**: won 1/6, lost 5/6
- **carrier_lay_real_rec20**: won 3/6, lost 3/6
- **carrier_lay_real_rec32**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 597851.2ns | 597432.5ns | 100.1% | HIGH |
| carrier_lay_real_rec16 | 617434.5ns | 616887.2ns | 100.1% | HIGH |
| carrier_lay_real_rec20 | 585543.3ns | 584451.2ns | 100.2% | HIGH |
| carrier_lay_real_rec24 | 589701.9ns | 589116.1ns | 100.1% | HIGH |
| carrier_lay_real_rec32 | 589670.3ns | 588734.7ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 577555.0-610156.5 ns)
  577555.0 |########################################
  579185.1 |
  580815.2 |
  582445.2 |
  584075.3 |########################################
  585705.4 |
  587335.4 |
  588965.5 |
  590595.6 |
  592225.7 |
  593855.8 |
  595485.8 |
  597115.9 |
  598746.0 |########################################
  600376.1 |########################################
  602006.1 |
  603636.2 |
  605266.3 |
  606896.3 |########################################
  608526.4 |
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 595734.2-626989.8 ns)
  595734.2 |########################################
  597297.0 |
  598859.8 |
  600422.5 |
  601985.3 |
  603548.1 |
  605110.9 |
  606673.7 |
  608236.4 |
  609799.2 |
  611362.0 |
  612924.8 |########################################
  614487.6 |
  616050.3 |########################################
  617613.1 |
  619175.9 |########################################
  620738.7 |
  622301.5 |########################################
  623864.2 |
  625427.0 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 548739.6-608975.4 ns)
  548739.6 |########################################
  551751.4 |
  554763.2 |
  557775.0 |
  560786.8 |
  563798.6 |
  566810.3 |########################################
  569822.1 |
  572833.9 |########################################
  575845.7 |
  578857.5 |
  581869.3 |
  584881.1 |
  587892.9 |
  590904.7 |
  593916.4 |
  596928.2 |########################################
  599940.0 |
  602951.8 |########################################
  605963.6 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 548877.1-610644.1 ns)
  548877.1 |########################################
  551965.5 |
  555053.8 |
  558142.2 |
  561230.5 |
  564318.9 |
  567407.2 |
  570495.6 |
  573583.9 |
  576672.3 |########################################
  579760.6 |########################################
  582849.0 |
  585937.3 |
  589025.7 |
  592114.0 |
  595202.4 |
  598290.7 |
  601379.1 |
  604467.4 |########################################
  607555.8 |########################################
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 557867.1-614041.4 ns)
  557867.1 |########################################
  560675.8 |
  563484.5 |########################################
  566293.3 |
  569102.0 |
  571910.7 |
  574719.4 |
  577528.1 |
  580336.8 |########################################
  583145.6 |
  585954.3 |
  588763.0 |
  591571.7 |
  594380.4 |
  597189.1 |########################################
  599997.9 |
  602806.6 |
  605615.3 |########################################
  608424.0 |
  611232.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=100.1% of algo (FFI overhead may distort results)
