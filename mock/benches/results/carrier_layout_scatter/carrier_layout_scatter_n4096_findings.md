# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_scatter_rec16 shows alternating (throttle bounce) (autocorr -0.80)

carrier_lay_scatter_rec16's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (10.07 us) is smaller than the fastest variant's own run-to-run std-dev (13.02 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (carrier_lay_scatter_rec24)

The baseline carrier_lay_scatter_rec24 is the fastest (576.66 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.7% of the fastest

All 5 variants sit between 576.66 us and 586.73 us - a 1.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_scatter_rec24) is the fastest** at 576662.3 ns median
- Spread: 1.02x (fastest 576662.3 ns, slowest 586730.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 580828ns | 582163ns | 571780ns | 578767ns | 588443ns | +0.53% |
| carrier_lay_scatter_rec16 | 579528ns | 580459ns | 556550ns | 576885ns | 594982ns | +0.30% |
| carrier_lay_scatter_rec20 | 589948ns | 589859ns | 578319ns | 586924ns | 600299ns | +2.11% |
| carrier_lay_scatter_rec24 | 577776ns | 579954ns | 551861ns | 578689ns | 589364ns | base |
| carrier_lay_scatter_rec32 | 580471ns | 581801ns | 563769ns | 579817ns | 589803ns | +0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 577721ns | 568665ns | 585482ns | +0.53% | 0.007 |
| carrier_lay_scatter_rec16 | 576664ns | 554109ns | 592256ns | +0.34% | 0.007 |
| carrier_lay_scatter_rec20 | 586812ns | 575262ns | 597109ns | +2.11% | 0.007 |
| carrier_lay_scatter_rec24 | 574698ns | 549389ns | 586261ns | base | 0.007 |
| carrier_lay_scatter_rec32 | 577351ns | 560502ns | 586786ns | +0.46% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 3619244 | 4762682 | 0.760 | 1.00× |
| carrier_lay_scatter_rec16 | 3631158 | 4705615 | 0.772 | 1.01× |
| carrier_lay_scatter_rec20 | 3696557 | 4704883 | 0.786 | 1.02× |
| carrier_lay_scatter_rec24 | 3609568 | 4704966 | 0.767 | 1.00× |
| carrier_lay_scatter_rec32 | 3637210 | 4704770 | 0.773 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_lay_scatter_rec24; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.007 | 94.9% |
| carrier_lay_scatter_rec16 | 0.007 | 95.1% |
| carrier_lay_scatter_rec20 | 0.007 | 93.6% |
| carrier_lay_scatter_rec24 | 0.007 | 95.3% |
| carrier_lay_scatter_rec32 | 0.007 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 580828ns | 580828ns | +0.53% |
| carrier_lay_scatter_rec16 | 579528ns | 579528ns | +0.30% |
| carrier_lay_scatter_rec20 | 589948ns | 589948ns | +2.11% |
| carrier_lay_scatter_rec24 | 577776ns | 577776ns | base |
| carrier_lay_scatter_rec32 | 580471ns | 580471ns | +0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 576662ns | base | --- | [561170, 586261] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 578861ns | no significant difference | [-12234, +16647]ns | [568820, 585482] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 577400ns | no significant difference | [-25925, +21832]ns | [560336, 592256] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_scatter_rec20 | 586731ns | no significant difference | [-1393, +34202]ns | [576596, 597109] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_scatter_rec32 | 578660ns | no significant difference | [-16016, +23880]ns | [566608, 586786] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 549389ns | +3.5% | +4.1% | +8.9% | +7.3% |
| 2 | 576424ns | +0.4% | +3.7% | +3.4% | +1.3% |
| 3 | 579268ns | -1.8% | -4.3% | -0.2% | -1.1% |
| 4 | 576900ns | +2.4% | +1.7% | -0.3% | -2.8% |
| 5 | 593255ns | -2.4% | -4.5% | +0.4% | -2.6% |
| 6 | 572952ns | +1.2% | +1.8% | +0.9% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.325 | moderate- |
| carrier_lay_scatter_rec16 | -0.797 | HIGH- (thermal bounce) |
| carrier_lay_scatter_rec20 | -0.082 | ok |
| carrier_lay_scatter_rec24 | -0.017 | ok |
| carrier_lay_scatter_rec32 | 0.246 | moderate+ |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec16**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec20**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 578148.0ns | 577721.0ns | 100.1% | HIGH |
| carrier_lay_scatter_rec16 | 577037.0ns | 576664.2ns | 100.1% | HIGH |
| carrier_lay_scatter_rec20 | 587416.8ns | 586812.0ns | 100.1% | HIGH |
| carrier_lay_scatter_rec24 | 575169.4ns | 574698.1ns | 100.1% | HIGH |
| carrier_lay_scatter_rec32 | 577482.9ns | 577351.2ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 568664.6-585481.6 ns)
  568664.6 |########################################
  569505.5 |
  570346.3 |
  571187.2 |
  572028.0 |
  572868.9 |
  573709.7 |
  574550.6 |
  575391.4 |
  576232.3 |
  577073.1 |
  577914.0 |####################
  578754.8 |####################
  579595.7 |####################
  580436.5 |
  581277.4 |
  582118.2 |
  582959.1 |
  583799.9 |
  584640.8 |
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 554109.2-592256.1 ns)
  554109.2 |########################################
  556016.5 |
  557923.9 |
  559831.2 |
  561738.6 |
  563645.9 |
  565553.3 |########################################
  567460.6 |
  569367.9 |
  571275.3 |########################################
  573182.6 |
  575090.0 |
  576997.3 |
  578904.7 |
  580812.0 |
  582719.3 |########################################
  584626.7 |
  586534.0 |########################################
  588441.4 |
  590348.7 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 575262.5-597108.9 ns)
  575262.5 |####################
  576354.8 |
  577447.1 |########################################
  578539.5 |
  579631.8 |
  580724.1 |
  581816.4 |
  582908.8 |
  584001.1 |
  585093.4 |
  586185.7 |
  587278.0 |
  588370.4 |
  589462.7 |
  590555.0 |
  591647.3 |
  592739.7 |
  593832.0 |
  594924.3 |########################################
  596016.6 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 549388.8-586261.4 ns)
  549388.8 |####################
  551232.4 |
  553076.1 |
  554919.7 |
  556763.3 |
  558607.0 |
  560450.6 |
  562294.2 |
  564137.9 |
  565981.5 |
  567825.1 |
  569668.8 |
  571512.4 |####################
  573356.0 |
  575199.7 |########################################
  577043.3 |
  578886.9 |####################
  580730.6 |
  582574.2 |
  584417.8 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 560502.5-586786.2 ns)
  560502.5 |########################################
  561816.7 |
  563130.9 |
  564445.1 |
  565759.2 |
  567073.4 |
  568387.6 |
  569701.8 |
  571016.0 |
  572330.2 |########################################
  573644.4 |
  574958.6 |
  576272.8 |
  577586.9 |########################################
  578901.1 |########################################
  580215.3 |
  581529.5 |
  582843.7 |########################################
  584157.9 |
  585472.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=99.9% of algo (FFI overhead may distort results)
