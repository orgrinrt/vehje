# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_regcache dominates: 18% faster than the next best (carrier_pre_madd_null)

carrier_pre_madd_regcache (145.59 us) leads carrier_pre_madd_null (172.06 us) by 18%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_madd_regcache beats baseline by 23% (significant)

carrier_pre_madd_regcache is -44.15 us (23%) faster than baseline carrier_pre_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_madd_null shows alternating (throttle bounce) (autocorr -0.53)

carrier_pre_madd_null's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 145585.0 ns median (-23.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.32x (fastest 145585.0 ns, slowest 191690.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 192272ns | 192470ns | 189467ns | 192261ns | 193690ns | -0.37% |
| carrier_pre_madd_fntable | 193433ns | 193852ns | 191677ns | 193190ns | 194676ns | +0.24% |
| carrier_pre_madd_null | 174419ns | 174242ns | 171796ns | 174160ns | 176120ns | -9.62% |
| carrier_pre_madd_regcache | 148048ns | 147767ns | 147196ns | 147689ns | 149013ns | -23.28% |
| carrier_pre_madd_switch | 192977ns | 191938ns | 191536ns | 191917ns | 195289ns | base |
| carrier_pre_madd_threaded | 190978ns | 191259ns | 188500ns | 190550ns | 192859ns | -1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 190109ns | 187328ns | 191512ns | -0.37% | 0.022 |
| carrier_pre_madd_fntable | 191254ns | 189542ns | 192431ns | +0.23% | 0.021 |
| carrier_pre_madd_null | 172230ns | 169644ns | 173905ns | -9.74% | 0.024 |
| carrier_pre_madd_regcache | 145865ns | 145031ns | 146814ns | -23.55% | 0.028 |
| carrier_pre_madd_switch | 190805ns | 189395ns | 193104ns | base | 0.021 |
| carrier_pre_madd_threaded | 188812ns | 186368ns | 190657ns | -1.04% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_madd_direct | 1230286 | 2254771 | 0.546 | 1.01× |
| carrier_pre_madd_fntable | 1234295 | 3370237 | 0.366 | 1.01× |
| carrier_pre_madd_null | 1104586 | 3095379 | 0.357 | 0.90× |
| carrier_pre_madd_regcache | 939597 | 3976041 | 0.236 | 0.77× |
| carrier_pre_madd_switch | 1222836 | 2905467 | 0.421 | 1.00× |
| carrier_pre_madd_threaded | 1215412 | 2974986 | 0.409 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.022 | 76.2% |
| carrier_pre_madd_fntable | 0.021 | 75.7% |
| carrier_pre_madd_null | 0.024 | 84.3% |
| carrier_pre_madd_regcache | 0.028 | 99.6% |
| carrier_pre_madd_switch | 0.022 | 76.4% |
| carrier_pre_madd_threaded | 0.022 | 76.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 192272ns | 192272ns | -0.37% |
| carrier_pre_madd_fntable | 193433ns | 193433ns | +0.24% |
| carrier_pre_madd_null | 174419ns | 174419ns | -9.62% |
| carrier_pre_madd_regcache | 148048ns | 148048ns | -23.28% |
| carrier_pre_madd_switch | 192977ns | 192977ns | base |
| carrier_pre_madd_threaded | 190978ns | 190978ns | -1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 189780ns | base | --- | [189532, 193104] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 190306ns | no significant difference | [-4596, +1917]ns | [188508, 191512] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_madd_fntable | 191690ns | no significant difference | [-3464, +2899]ns | [189640, 192431] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_madd_null | 172059ns | -18050.8ns (-9.5%) | [-21581, -16095]ns | [170725, 173905] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 145585ns | -44147.5ns (-23.3%) | [-47771, -42903]ns | [145195, 146814] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 189096ns | no significant difference | [-6422, +1104]ns | [186682, 190657] | no | 0.3646 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 190915ns | -0.6% | -0.7% | -9.5% | -24.0% | -2.1% |
| 2 | 189796ns | +0.7% | +0.9% | -9.5% | -23.4% | -0.5% |
| 3 | 189765ns | +0.2% | +1.1% | -7.8% | -23.0% | -0.2% |
| 4 | 189669ns | +0.5% | +1.6% | -10.6% | -22.3% | +1.2% |
| 5 | 189395ns | +1.4% | +1.4% | -9.2% | -23.2% | -0.0% |
| 6 | 195292ns | -4.1% | -2.8% | -11.8% | -25.4% | -4.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.389 | moderate- |
| carrier_pre_madd_fntable | 0.091 | ok |
| carrier_pre_madd_null | -0.527 | HIGH- (thermal bounce) |
| carrier_pre_madd_regcache | 0.079 | ok |
| carrier_pre_madd_switch | -0.102 | ok |
| carrier_pre_madd_threaded | 0.098 | ok |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 2/6, lost 4/6
- **carrier_pre_madd_fntable**: won 2/6, lost 4/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 203031.1ns | 190108.8ns | 106.8% | HIGH |
| carrier_pre_madd_fntable | 201229.0ns | 191253.6ns | 105.2% | HIGH |
| carrier_pre_madd_null | 181911.9ns | 172229.6ns | 105.6% | HIGH |
| carrier_pre_madd_regcache | 155566.4ns | 145864.6ns | 106.7% | HIGH |
| carrier_pre_madd_switch | 200405.3ns | 190805.4ns | 105.0% | HIGH |
| carrier_pre_madd_threaded | 198723.6ns | 188811.6ns | 105.2% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 187328.3-191512.5 ns)
  187328.3 |########################################
  187537.5 |
  187746.7 |
  187955.9 |
  188165.1 |
  188374.3 |
  188583.6 |
  188792.8 |
  189002.0 |
  189211.2 |
  189420.4 |
  189629.6 |########################################
  189838.8 |
  190048.0 |########################################
  190257.2 |
  190466.5 |########################################
  190675.7 |
  190884.9 |########################################
  191094.1 |
  191303.3 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 189542.1-192430.6 ns)
  189542.1 |########################################
  189686.5 |########################################
  189831.0 |
  189975.4 |
  190119.8 |
  190264.2 |
  190408.7 |
  190553.1 |
  190697.5 |
  190841.9 |
  190986.4 |
  191130.8 |
  191275.2 |
  191419.7 |########################################
  191564.1 |
  191708.5 |
  191852.9 |########################################
  191997.4 |########################################
  192141.8 |
  192286.2 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 169644.2-173904.6 ns)
  169644.2 |####################
  169857.2 |
  170070.2 |
  170283.3 |
  170496.3 |
  170709.3 |
  170922.3 |
  171135.3 |
  171348.4 |
  171561.4 |
  171774.4 |########################################
  171987.4 |####################
  172200.4 |
  172413.5 |
  172626.5 |####################
  172839.5 |
  173052.5 |
  173265.5 |
  173478.6 |
  173691.6 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 145030.8-146813.5 ns)
  145030.8 |########################################
  145119.9 |
  145209.1 |
  145298.2 |########################################
  145387.3 |
  145476.5 |########################################
  145565.6 |########################################
  145654.8 |
  145743.9 |
  145833.0 |
  145922.2 |
  146011.3 |
  146100.4 |########################################
  146189.6 |
  146278.7 |
  146367.9 |
  146457.0 |
  146546.1 |
  146635.3 |
  146724.4 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 189394.6-193103.8 ns)
  189394.6 |####################
  189580.1 |########################################
  189765.5 |####################
  189951.0 |
  190136.4 |
  190321.9 |
  190507.3 |
  190692.8 |
  190878.3 |####################
  191063.7 |
  191249.2 |
  191434.6 |
  191620.1 |
  191805.5 |
  191991.0 |
  192176.5 |
  192361.9 |
  192547.4 |
  192732.8 |
  192918.3 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 186368.3-190656.6 ns)
  186368.3 |####################
  186582.7 |
  186797.1 |####################
  187011.6 |
  187226.0 |
  187440.4 |
  187654.8 |
  187869.2 |
  188083.6 |
  188298.1 |
  188512.5 |
  188726.9 |####################
  188941.3 |
  189155.7 |########################################
  189370.1 |
  189584.6 |
  189799.0 |
  190013.4 |
  190227.8 |
  190442.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=106.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=105.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=105.6% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=106.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=105.1% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=105.2% of algo (FFI overhead may distort results)
