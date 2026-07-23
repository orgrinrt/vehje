# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), madd profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_madd_threaded shows alternating (throttle bounce) (autocorr -0.58)

carrier_cold_madd_threaded's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_cold_madd_null vs stability leader carrier_cold_madd_threaded (+10% speed for 1.1x steadier)

carrier_cold_madd_null is fastest (9.97 us, CV 3.0%); carrier_cold_madd_threaded gives up 9.7% median for 1.1x lower variance (CV 2.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_cold_madd_null** at 9967.5 ns median (-11.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.19x (fastest 9967.5 ns, slowest 11894.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_madd_fntable | 14183ns | 14337ns | 13549ns | 14080ns | 14655ns | +4.69% |
| carrier_cold_madd_null | 12281ns | 12438ns | 11683ns | 12245ns | 12634ns | -9.35% |
| carrier_cold_madd_switch | 13548ns | 13560ns | 12953ns | 13408ns | 14054ns | base |
| carrier_cold_madd_threaded | 13297ns | 13260ns | 12852ns | 13171ns | 13708ns | -1.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_madd_fntable | 11777ns | 11298ns | 12122ns | +4.89% | 0.022 |
| carrier_cold_madd_null | 9858ns | 9392ns | 10155ns | -12.20% | 0.026 |
| carrier_cold_madd_switch | 11228ns | 10722ns | 11604ns | base | 0.023 |
| carrier_cold_madd_threaded | 10994ns | 10610ns | 11353ns | -2.08% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_madd_fntable | 311316 | 886693 | 0.351 | 0.98× |
| carrier_cold_madd_null | 304308 | 931596 | 0.327 | 0.96× |
| carrier_cold_madd_switch | 316603 | 775637 | 0.408 | 1.00× |
| carrier_cold_madd_threaded | 313485 | 804334 | 0.390 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_cold_madd_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_madd_fntable | 0.022 | 79.0% |
| carrier_cold_madd_null | 0.026 | 94.2% |
| carrier_cold_madd_switch | 0.023 | 83.3% |
| carrier_cold_madd_threaded | 0.023 | 85.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_madd_fntable | 14183ns | 14183ns | +4.69% |
| carrier_cold_madd_null | 12281ns | 12281ns | -9.35% |
| carrier_cold_madd_switch | 13548ns | 13548ns | base |
| carrier_cold_madd_threaded | 13297ns | 13297ns | -1.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_madd_switch | 11277ns | base | --- | [10803, 11604] | --- | --- | --- | --- |
| carrier_cold_madd_fntable | 11895ns | +449.6ns (+4.0%) | [+270, +928]ns | [11314, 12122] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_madd_null | 9968ns | -1458.1ns (-12.9%) | [-1906, -746]ns | [9451, 10155] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_madd_threaded | 10937ns | no significant difference | [-525, +57]ns | [10694, 11353] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_madd_switch | carrier_cold_madd_fntable | carrier_cold_madd_null | carrier_cold_madd_threaded |
|---|---|---|---|---|
| 1 | 10883ns | +11.5% | -6.5% | -2.5% |
| 2 | 11614ns | +1.8% | -19.1% | -0.5% |
| 3 | 10968ns | +3.0% | -13.3% | -1.7% |
| 4 | 10722ns | +5.7% | -7.3% | +1.7% |
| 5 | 11593ns | +3.2% | -12.6% | -5.3% |
| 6 | 11585ns | +4.6% | -13.7% | -3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_madd_fntable | 0.266 | moderate+ |
| carrier_cold_madd_null | 0.086 | ok |
| carrier_cold_madd_switch | -0.183 | ok |
| carrier_cold_madd_threaded | -0.585 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_cold_madd_fntable**: won 0/6, lost 6/6
- **carrier_cold_madd_null**: won 6/6, lost 0/6
- **carrier_cold_madd_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_madd_fntable | 90431.0ns | 11777.1ns | 767.9% | HIGH |
| carrier_cold_madd_null | 90607.8ns | 9857.9ns | 919.1% | HIGH |
| carrier_cold_madd_switch | 91750.1ns | 11227.6ns | 817.2% | HIGH |
| carrier_cold_madd_threaded | 88865.5ns | 10994.4ns | 808.3% | HIGH |

## Distribution (algo ns)

```
carrier_cold_madd_fntable (n=6, range 11297.5-12122.5 ns)
  11297.5 |########################################
  11338.8 |
  11380.0 |
  11421.2 |
  11462.5 |
  11503.8 |
  11545.0 |
  11586.2 |
  11627.5 |
  11668.8 |
  11710.0 |
  11751.2 |
  11792.5 |####################
  11833.8 |
  11875.0 |
  11916.2 |
  11957.5 |####################
  11998.8 |
  12040.0 |
  12081.2 |####################
  (0 below, 1 above range)

carrier_cold_madd_null (n=6, range 9392.5-10154.8 ns)
   9392.5 |########################################
   9430.6 |
   9468.7 |
   9506.8 |########################################
   9545.0 |
   9583.1 |
   9621.2 |
   9659.3 |
   9697.4 |
   9735.5 |
   9773.6 |
   9811.8 |
   9849.9 |
   9888.0 |
   9926.1 |########################################
   9964.2 |########################################
  10002.3 |
  10040.5 |
  10078.6 |
  10116.7 |########################################
  (0 below, 1 above range)

carrier_cold_madd_switch (n=6, range 10722.5-11603.5 ns)
  10722.5 |####################
  10766.6 |
  10810.6 |
  10854.7 |####################
  10898.7 |
  10942.8 |####################
  10986.8 |
  11030.9 |
  11074.9 |
  11119.0 |
  11163.0 |
  11207.1 |
  11251.1 |
  11295.2 |
  11339.2 |
  11383.3 |
  11427.3 |
  11471.4 |
  11515.4 |
  11559.5 |########################################
  (0 below, 1 above range)

carrier_cold_madd_threaded (n=6, range 10610.4-11352.7 ns)
  10610.4 |########################################
  10647.5 |
  10684.6 |
  10721.7 |
  10758.9 |########################################
  10796.0 |
  10833.1 |
  10870.2 |########################################
  10907.3 |
  10944.4 |########################################
  10981.5 |
  11018.7 |
  11055.8 |
  11092.9 |
  11130.0 |########################################
  11167.1 |
  11204.2 |
  11241.4 |
  11278.5 |
  11315.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_madd_fntable**: bridge=761.1% of algo (FFI overhead may distort results)
- **carrier_cold_madd_null**: bridge=910.0% of algo (FFI overhead may distort results)
- **carrier_cold_madd_switch**: bridge=822.2% of algo (FFI overhead may distort results)
- **carrier_cold_madd_threaded**: bridge=809.5% of algo (FFI overhead may distort results)
