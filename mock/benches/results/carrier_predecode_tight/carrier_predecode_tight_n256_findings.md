# Predecoded dispatch shape, tight profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_tight_null shows alternating (throttle bounce) (autocorr -0.72)

carrier_pre_tight_null's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_tight_null** at 7770.4 ns median (-10.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.23x (fastest 7770.4 ns, slowest 9543.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_tight_direct | 11831ns | 11894ns | 11451ns | 11866ns | 11968ns | +5.77% |
| carrier_pre_tight_fntable | 11618ns | 11771ns | 10715ns | 11752ns | 11870ns | +3.87% |
| carrier_pre_tight_null | 10238ns | 10203ns | 9970ns | 10172ns | 10471ns | -8.46% |
| carrier_pre_tight_regcache | 10889ns | 10991ns | 10101ns | 10953ns | 11185ns | -2.65% |
| carrier_pre_tight_switch | 11185ns | 11186ns | 10933ns | 11169ns | 11335ns | base |
| carrier_pre_tight_threaded | 11319ns | 11590ns | 10486ns | 11320ns | 11734ns | +1.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_tight_direct | 9499ns | 9195ns | 9611ns | +9.26% | 0.027 |
| carrier_pre_tight_fntable | 9121ns | 8431ns | 9329ns | +4.91% | 0.028 |
| carrier_pre_tight_null | 7726ns | 7512ns | 7875ns | -11.13% | 0.033 |
| carrier_pre_tight_regcache | 8416ns | 7858ns | 8561ns | -3.19% | 0.030 |
| carrier_pre_tight_switch | 8694ns | 8569ns | 8778ns | base | 0.029 |
| carrier_pre_tight_threaded | 8786ns | 8183ns | 9075ns | +1.06% | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_tight_direct | 298576 | 680683 | 0.439 | 1.03× |
| carrier_pre_tight_fntable | 295852 | 1085546 | 0.273 | 1.02× |
| carrier_pre_tight_null | 296057 | 1154522 | 0.256 | 1.02× |
| carrier_pre_tight_regcache | 295501 | 1385666 | 0.213 | 1.02× |
| carrier_pre_tight_switch | 290846 | 959545 | 0.303 | 1.00× |
| carrier_pre_tight_threaded | 302420 | 1007422 | 0.300 | 1.04× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_pre_tight_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_tight_direct | 0.027 | 78.7% |
| carrier_pre_tight_fntable | 0.028 | 81.4% |
| carrier_pre_tight_null | 0.033 | 96.7% |
| carrier_pre_tight_regcache | 0.030 | 88.0% |
| carrier_pre_tight_switch | 0.029 | 86.3% |
| carrier_pre_tight_threaded | 0.029 | 83.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_tight_direct | 11831ns | 11831ns | +5.77% |
| carrier_pre_tight_fntable | 11618ns | 11618ns | +3.87% |
| carrier_pre_tight_null | 10238ns | 10238ns | -8.46% |
| carrier_pre_tight_regcache | 10889ns | 10889ns | -2.65% |
| carrier_pre_tight_switch | 11185ns | 11185ns | base |
| carrier_pre_tight_threaded | 11319ns | 11319ns | +1.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_tight_switch | 8709ns | base | --- | [8595, 8778] | --- | --- | --- | --- |
| carrier_pre_tight_direct | 9544ns | +808.8ns (+9.3%) | [+633, +974]ns | [9343, 9611] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_fntable | 9232ns | +542.3ns (+6.2%) | [+86, +653]ns | [8802, 9329] | YES (adj: no) | 0.2734 | 0.2188 | 0 |
| carrier_pre_tight_null | 7770ns | -911.3ns (-10.5%) | [-1176, -816]ns | [7534, 7875] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_regcache | 8535ns | -174.3ns (-2.0%) | [-625, -34]ns | [8152, 8561] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_tight_threaded | 8965ns | no significant difference | [-406, +388]ns | [8319, 9075] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_tight_switch | carrier_pre_tight_direct | carrier_pre_tight_fntable | carrier_pre_tight_null | carrier_pre_tight_regcache | carrier_pre_tight_threaded |
|---|---|---|---|---|---|---|
| 1 | 8730ns | +9.7% | -3.4% | -13.5% | -2.3% | +2.9% |
| 2 | 8621ns | +11.9% | +8.1% | -8.6% | -0.8% | +3.9% |
| 3 | 8569ns | +10.8% | +7.1% | -10.5% | -0.0% | +4.7% |
| 4 | 8795ns | +8.6% | +6.2% | -10.5% | -4.0% | +4.3% |
| 5 | 8688ns | +5.8% | +6.2% | -13.5% | -1.8% | -5.8% |
| 6 | 8761ns | +8.8% | +5.4% | -10.1% | -10.3% | -3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_tight_direct | -0.141 | ok |
| carrier_pre_tight_fntable | -0.135 | ok |
| carrier_pre_tight_null | -0.724 | HIGH- (thermal bounce) |
| carrier_pre_tight_regcache | -0.056 | ok |
| carrier_pre_tight_switch | -0.190 | ok |
| carrier_pre_tight_threaded | 0.145 | ok |

**Consistency summary:**

- **carrier_pre_tight_direct**: won 0/6, lost 6/6
- **carrier_pre_tight_fntable**: won 1/6, lost 5/6
- **carrier_pre_tight_null**: won 6/6, lost 0/6
- **carrier_pre_tight_regcache**: won 5/6, lost 0/6
- **carrier_pre_tight_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_tight_direct | 91008.9ns | 9499.2ns | 958.1% | HIGH |
| carrier_pre_tight_fntable | 89018.5ns | 9121.1ns | 976.0% | HIGH |
| carrier_pre_tight_null | 90428.8ns | 7726.2ns | 1170.4% | HIGH |
| carrier_pre_tight_regcache | 89653.1ns | 8416.2ns | 1065.2% | HIGH |
| carrier_pre_tight_switch | 90122.8ns | 8694.0ns | 1036.6% | HIGH |
| carrier_pre_tight_threaded | 91535.4ns | 8786.4ns | 1041.8% | HIGH |

## Distribution (algo ns)

```
carrier_pre_tight_direct (n=6, range 9194.6-9611.0 ns)
   9194.6 |########################################
   9215.4 |
   9236.2 |
   9257.1 |
   9277.9 |
   9298.7 |
   9319.5 |
   9340.3 |
   9361.2 |
   9382.0 |
   9402.8 |
   9423.6 |
   9444.4 |
   9465.3 |
   9486.1 |########################################
   9506.9 |
   9527.7 |########################################
   9548.5 |########################################
   9569.4 |########################################
   9590.2 |
  (0 below, 1 above range)

carrier_pre_tight_fntable (n=6, range 8430.8-9329.0 ns)
   8430.8 |####################
   8475.7 |
   8520.6 |
   8565.5 |
   8610.4 |
   8655.3 |
   8700.2 |
   8745.2 |
   8790.1 |
   8835.0 |
   8879.9 |
   8924.8 |
   8969.7 |
   9014.6 |
   9059.5 |
   9104.4 |
   9149.3 |####################
   9194.2 |########################################
   9239.1 |
   9284.0 |####################
  (0 below, 1 above range)

carrier_pre_tight_null (n=6, range 7512.5-7874.6 ns)
   7512.5 |####################
   7530.6 |
   7548.7 |####################
   7566.8 |
   7584.9 |
   7603.0 |
   7621.1 |
   7639.2 |
   7657.3 |####################
   7675.4 |
   7693.6 |
   7711.7 |
   7729.8 |
   7747.9 |
   7766.0 |
   7784.1 |
   7802.2 |
   7820.3 |
   7838.4 |
   7856.5 |########################################
  (0 below, 1 above range)

carrier_pre_tight_regcache (n=6, range 7857.9-8561.2 ns)
   7857.9 |#############
   7893.1 |
   7928.2 |
   7963.4 |
   7998.6 |
   8033.7 |
   8068.9 |
   8104.1 |
   8139.2 |
   8174.4 |
   8209.6 |
   8244.7 |
   8279.9 |
   8315.1 |
   8350.2 |
   8385.4 |
   8420.6 |#############
   8455.7 |
   8490.9 |
   8526.1 |########################################
  (0 below, 1 above range)

carrier_pre_tight_switch (n=6, range 8568.8-8777.9 ns)
   8568.8 |########################################
   8579.3 |
   8589.7 |
   8600.2 |
   8610.6 |########################################
   8621.1 |
   8631.5 |
   8642.0 |
   8652.4 |
   8662.9 |
   8673.4 |
   8683.8 |########################################
   8694.3 |
   8704.7 |
   8715.2 |
   8725.6 |########################################
   8736.1 |
   8746.5 |
   8757.0 |########################################
   8767.4 |
  (0 below, 1 above range)

carrier_pre_tight_threaded (n=6, range 8182.9-9074.8 ns)
   8182.9 |#############
   8227.5 |
   8272.1 |
   8316.7 |
   8361.3 |
   8405.9 |
   8450.5 |#############
   8495.1 |
   8539.7 |
   8584.3 |
   8628.8 |
   8673.4 |
   8718.0 |
   8762.6 |
   8807.2 |
   8851.8 |
   8896.4 |
   8941.0 |########################################
   8985.6 |
   9030.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_tight_direct**: bridge=951.5% of algo (FFI overhead may distort results)
- **carrier_pre_tight_fntable**: bridge=959.6% of algo (FFI overhead may distort results)
- **carrier_pre_tight_null**: bridge=1161.6% of algo (FFI overhead may distort results)
- **carrier_pre_tight_regcache**: bridge=1053.4% of algo (FFI overhead may distort results)
- **carrier_pre_tight_switch**: bridge=1033.9% of algo (FFI overhead may distort results)
- **carrier_pre_tight_threaded**: bridge=1026.6% of algo (FFI overhead may distort results)
