# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_lay_real_rec24, carrier_lay_real_rec20) are a dead heat (<1%)

carrier_lay_real_rec24 (559.67 us) and carrier_lay_real_rec20 (562.03 us) differ by 0.42%, inside the noise, even though the wider field spreads 3.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_lay_real_rec32 shows alternating (throttle bounce) (autocorr -0.51)

carrier_lay_real_rec32's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_lay_real_rec24)

The baseline carrier_lay_real_rec24 is the fastest (559.67 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 3.6% of the fastest

All 5 variants sit between 559.67 us and 579.94 us - a 3.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_real_rec24) is the fastest** at 559671.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.04x (fastest 559671.2 ns, slowest 579936.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 584188ns | 583361ns | 567747ns | 581765ns | 596043ns | +3.28% |
| carrier_lay_real_rec16 | 575836ns | 576654ns | 559930ns | 574019ns | 586514ns | +1.80% |
| carrier_lay_real_rec20 | 569809ns | 565160ns | 554438ns | 563227ns | 587368ns | +0.74% |
| carrier_lay_real_rec24 | 565645ns | 562702ns | 551808ns | 559452ns | 581854ns | base |
| carrier_lay_real_rec32 | 574280ns | 570455ns | 559078ns | 569946ns | 588382ns | +1.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 580994ns | 564707ns | 592911ns | +3.27% | 0.007 |
| carrier_lay_real_rec16 | 572904ns | 557631ns | 583642ns | +1.84% | 0.007 |
| carrier_lay_real_rec20 | 566737ns | 551361ns | 584270ns | +0.74% | 0.007 |
| carrier_lay_real_rec24 | 562573ns | 548512ns | 578901ns | base | 0.007 |
| carrier_lay_real_rec32 | 571176ns | 555788ns | 585433ns | +1.53% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 3665228 | 4644854 | 0.789 | 1.03× |
| carrier_lay_real_rec16 | 3608933 | 4597143 | 0.785 | 1.02× |
| carrier_lay_real_rec20 | 3577055 | 4596846 | 0.778 | 1.01× |
| carrier_lay_real_rec24 | 3546268 | 4596790 | 0.771 | 1.00× |
| carrier_lay_real_rec32 | 3602589 | 4596825 | 0.784 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_lay_real_rec24; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.007 | 94.6% |
| carrier_lay_real_rec16 | 0.007 | 95.6% |
| carrier_lay_real_rec20 | 0.007 | 97.6% |
| carrier_lay_real_rec24 | 0.007 | 98.0% |
| carrier_lay_real_rec32 | 0.007 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 584188ns | 584188ns | +3.28% |
| carrier_lay_real_rec16 | 575836ns | 575836ns | +1.80% |
| carrier_lay_real_rec20 | 569809ns | 569809ns | +0.74% |
| carrier_lay_real_rec24 | 565645ns | 565645ns | base |
| carrier_lay_real_rec32 | 574280ns | 574280ns | +1.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 559671ns | base | --- | [549147, 578901] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 579937ns | +18614.1ns (+3.3%) | [+1036, +35613]ns | [570134, 592911] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_real_rec16 | 573571ns | +9907.3ns (+1.8%) | [+682, +20405]ns | [561499, 583642] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| carrier_lay_real_rec20 | 562034ns | no significant difference | [-16504, +22244]ns | [553905, 584270] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec32 | 567415ns | no significant difference | [-11760, +28974]ns | [560680, 585433] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 576837ns | +0.0% | +1.9% | +1.2% | -1.4% |
| 2 | 564815ns | +6.3% | +1.1% | -0.6% | +1.8% |
| 3 | 554528ns | +1.8% | +2.0% | +5.5% | +2.1% |
| 4 | 548512ns | +4.9% | +1.7% | +2.6% | +1.3% |
| 5 | 549782ns | +6.5% | +5.5% | +1.2% | +8.4% |
| 6 | 580965ns | +0.3% | -0.9% | -5.1% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.453 | moderate- |
| carrier_lay_real_rec16 | 0.034 | ok |
| carrier_lay_real_rec20 | -0.060 | ok |
| carrier_lay_real_rec24 | 0.074 | ok |
| carrier_lay_real_rec32 | -0.506 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 0/6, lost 5/6
- **carrier_lay_real_rec16**: won 1/6, lost 5/6
- **carrier_lay_real_rec20**: won 2/6, lost 4/6
- **carrier_lay_real_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 581354.5ns | 580994.0ns | 100.1% | HIGH |
| carrier_lay_real_rec16 | 573864.6ns | 572904.2ns | 100.2% | HIGH |
| carrier_lay_real_rec20 | 567434.0ns | 566736.5ns | 100.1% | HIGH |
| carrier_lay_real_rec24 | 563318.4ns | 562573.0ns | 100.1% | HIGH |
| carrier_lay_real_rec32 | 571245.2ns | 571175.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 564706.7-592911.2 ns)
  564706.7 |########################################
  566116.9 |
  567527.2 |
  568937.4 |
  570347.6 |
  571757.8 |
  573168.1 |
  574578.3 |########################################
  575988.5 |########################################
  577398.7 |
  578809.0 |
  580219.2 |
  581629.4 |########################################
  583039.7 |
  584449.9 |########################################
  585860.1 |
  587270.3 |
  588680.6 |
  590090.8 |
  591501.0 |
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 557630.8-583642.1 ns)
  557630.8 |########################################
  558931.4 |
  560231.9 |
  561532.5 |
  562833.1 |
  564133.6 |########################################
  565434.2 |
  566734.8 |
  568035.3 |
  569335.9 |
  570636.4 |########################################
  571937.0 |
  573237.6 |
  574538.1 |
  575838.7 |########################################
  577139.3 |
  578439.8 |
  579740.4 |########################################
  581041.0 |
  582341.5 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 551361.2-584270.0 ns)
  551361.2 |####################
  553006.6 |
  554652.1 |
  556297.5 |####################
  557943.0 |
  559588.4 |
  561233.8 |########################################
  562879.3 |
  564524.7 |
  566170.2 |
  567815.6 |
  569461.0 |
  571106.5 |
  572751.9 |
  574397.4 |
  576042.8 |
  577688.2 |
  579333.7 |
  580979.1 |
  582624.6 |####################
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 548512.1-578900.6 ns)
  548512.1 |########################################
  550031.5 |
  551551.0 |
  553070.4 |####################
  554589.8 |
  556109.2 |
  557628.7 |
  559148.1 |
  560667.5 |
  562186.9 |
  563706.4 |####################
  565225.8 |
  566745.2 |
  568264.7 |
  569784.1 |
  571303.5 |
  572822.9 |
  574342.4 |
  575861.8 |####################
  577381.2 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 555787.5-585432.9 ns)
  555787.5 |####################
  557269.8 |
  558752.0 |
  560234.3 |
  561716.6 |
  563198.8 |
  564681.1 |########################################
  566163.4 |
  567645.7 |####################
  569127.9 |
  570610.2 |
  572092.5 |
  573574.7 |####################
  575057.0 |
  576539.3 |
  578021.5 |
  579503.8 |
  580986.1 |
  582468.4 |
  583950.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=100.1% of algo (FFI overhead may distort results)
