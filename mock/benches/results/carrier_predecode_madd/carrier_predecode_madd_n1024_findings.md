# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_regcache dominates: 20% faster than the next best (carrier_pre_madd_null)

carrier_pre_madd_regcache (37.40 us) leads carrier_pre_madd_null (44.89 us) by 20%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_madd_regcache beats baseline by 24% (significant)

carrier_pre_madd_regcache is -11.97 us (24%) faster than baseline carrier_pre_madd_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_madd_null shows alternating (throttle bounce) (autocorr -0.69)

carrier_pre_madd_null's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 37402.1 ns median (-24.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.33x (fastest 37402.1 ns, slowest 49768.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 52224ns | 52023ns | 51021ns | 51817ns | 53435ns | +1.23% |
| carrier_pre_madd_fntable | 50384ns | 50680ns | 48283ns | 50609ns | 51098ns | -2.34% |
| carrier_pre_madd_null | 47228ns | 47146ns | 46964ns | 47109ns | 47536ns | -8.46% |
| carrier_pre_madd_regcache | 39793ns | 39625ns | 39380ns | 39552ns | 40360ns | -22.87% |
| carrier_pre_madd_switch | 51592ns | 51815ns | 49437ns | 51672ns | 52548ns | base |
| carrier_pre_madd_threaded | 51436ns | 51302ns | 50575ns | 51205ns | 52214ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 49966ns | 48815ns | 51135ns | +1.21% | 0.020 |
| carrier_pre_madd_fntable | 48155ns | 46122ns | 48841ns | -2.46% | 0.021 |
| carrier_pre_madd_null | 44963ns | 44710ns | 45250ns | -8.93% | 0.023 |
| carrier_pre_madd_regcache | 37555ns | 37159ns | 38088ns | -23.93% | 0.027 |
| carrier_pre_madd_switch | 49370ns | 47305ns | 50278ns | base | 0.021 |
| carrier_pre_madd_threaded | 49194ns | 48366ns | 49944ns | -0.36% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_madd_direct | 462952 | 826854 | 0.560 | 1.00× |
| carrier_pre_madd_fntable | 450458 | 1252769 | 0.360 | 0.98× |
| carrier_pre_madd_null | 417189 | 1149156 | 0.363 | 0.90× |
| carrier_pre_madd_regcache | 464112 | 1948188 | 0.238 | 1.01× |
| carrier_pre_madd_switch | 461292 | 1070660 | 0.431 | 1.00× |
| carrier_pre_madd_threaded | 456882 | 1104662 | 0.414 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.021 | 74.7% |
| carrier_pre_madd_fntable | 0.021 | 76.7% |
| carrier_pre_madd_null | 0.023 | 82.8% |
| carrier_pre_madd_regcache | 0.027 | 99.4% |
| carrier_pre_madd_switch | 0.021 | 74.9% |
| carrier_pre_madd_threaded | 0.021 | 75.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 52224ns | 52224ns | +1.23% |
| carrier_pre_madd_fntable | 50384ns | 50384ns | -2.34% |
| carrier_pre_madd_null | 47228ns | 47228ns | -8.46% |
| carrier_pre_madd_regcache | 39793ns | 39793ns | -22.87% |
| carrier_pre_madd_switch | 51592ns | 51592ns | base |
| carrier_pre_madd_threaded | 51436ns | 51436ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 49593ns | base | --- | [48239, 50278] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 49768ns | no significant difference | [-1283, +2520]ns | [48995, 51135] | no | 0.8594 | 0.6875 | 0 |
| carrier_pre_madd_fntable | 48441ns | -1087.5ns (-2.2%) | [-1841, -716]ns | [47183, 48841] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_null | 44892ns | -4554.0ns (-9.2%) | [-5494, -3174]ns | [44746, 45250] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 37402ns | -11975.0ns (-24.1%) | [-13034, -10437]ns | [37175, 38088] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 49059ns | no significant difference | [-1219, +993]ns | [48578, 49944] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 47305ns | +7.2% | -2.5% | -4.6% | -19.2% | +2.2% |
| 2 | 50090ns | -2.5% | -2.0% | -10.6% | -25.8% | -2.3% |
| 3 | 49174ns | +2.2% | -1.5% | -8.5% | -24.4% | +1.1% |
| 4 | 49261ns | +0.0% | -1.4% | -9.2% | -23.9% | +1.9% |
| 5 | 49925ns | +3.3% | -3.4% | -9.1% | -24.0% | -2.3% |
| 6 | 50465ns | -2.6% | -4.0% | -11.3% | -26.0% | -2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.676 | HIGH- (thermal bounce) |
| carrier_pre_madd_fntable | -0.277 | moderate- |
| carrier_pre_madd_null | -0.687 | HIGH- (thermal bounce) |
| carrier_pre_madd_regcache | -0.224 | moderate- |
| carrier_pre_madd_switch | -0.167 | ok |
| carrier_pre_madd_threaded | 0.088 | ok |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 2/6, lost 3/6
- **carrier_pre_madd_fntable**: won 6/6, lost 0/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 103603.3ns | 49966.1ns | 207.3% | HIGH |
| carrier_pre_madd_fntable | 99120.0ns | 48155.3ns | 205.8% | HIGH |
| carrier_pre_madd_null | 92716.7ns | 44962.7ns | 206.2% | HIGH |
| carrier_pre_madd_regcache | 115400.5ns | 37555.0ns | 307.3% | HIGH |
| carrier_pre_madd_switch | 101392.3ns | 49370.1ns | 205.4% | HIGH |
| carrier_pre_madd_threaded | 101364.0ns | 49193.8ns | 206.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 48815.4-51135.4 ns)
  48815.4 |####################
  48931.4 |
  49047.4 |
  49163.4 |########################################
  49279.4 |
  49395.4 |
  49511.4 |
  49627.4 |
  49743.4 |
  49859.4 |
  49975.4 |
  50091.4 |
  50207.4 |####################
  50323.4 |
  50439.4 |
  50555.4 |
  50671.4 |####################
  50787.4 |
  50903.4 |
  51019.4 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 46121.7-48841.4 ns)
  46121.7 |########################################
  46257.7 |
  46393.7 |
  46529.7 |
  46665.6 |
  46801.6 |
  46937.6 |
  47073.6 |
  47209.6 |
  47345.6 |
  47481.6 |
  47617.6 |
  47753.5 |
  47889.5 |
  48025.5 |
  48161.5 |########################################
  48297.5 |########################################
  48433.5 |########################################
  48569.5 |########################################
  48705.5 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 44710.0-45250.0 ns)
  44710.0 |####################
  44737.0 |
  44764.0 |########################################
  44791.0 |
  44818.0 |
  44845.0 |
  44872.0 |
  44899.0 |
  44926.0 |
  44953.0 |
  44980.0 |####################
  45007.0 |
  45034.0 |
  45061.0 |
  45088.0 |
  45115.0 |####################
  45142.0 |
  45169.0 |
  45196.0 |
  45223.0 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 37159.2-38088.2 ns)
  37159.2 |########################################
  37205.6 |
  37252.1 |
  37298.5 |####################
  37345.0 |
  37391.4 |
  37437.9 |####################
  37484.3 |
  37530.8 |
  37577.2 |
  37623.7 |
  37670.1 |
  37716.6 |
  37763.0 |
  37809.5 |
  37855.9 |
  37902.4 |
  37948.8 |####################
  37995.3 |
  38041.7 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 47305.0-50277.7 ns)
  47305.0 |########################################
  47453.6 |
  47602.3 |
  47750.9 |
  47899.5 |
  48048.2 |
  48196.8 |
  48345.4 |
  48494.1 |
  48642.7 |
  48791.3 |
  48940.0 |
  49088.6 |########################################
  49237.3 |########################################
  49385.9 |
  49534.5 |
  49683.2 |
  49831.8 |########################################
  49980.4 |########################################
  50129.1 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 48365.8-49943.9 ns)
  48365.8 |########################################
  48444.7 |
  48523.6 |
  48602.5 |
  48681.4 |
  48760.3 |########################################
  48839.2 |
  48918.2 |########################################
  48997.1 |
  49076.0 |
  49154.9 |########################################
  49233.8 |
  49312.7 |
  49391.6 |
  49470.5 |
  49549.4 |
  49628.3 |########################################
  49707.2 |
  49786.1 |
  49865.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=208.5% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=205.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=206.3% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=307.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=205.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=205.6% of algo (FFI overhead may distort results)
