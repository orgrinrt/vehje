# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 20% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (6.58 us) leads carrier_pre_real_direct (7.93 us) by 20%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 30% (significant)

carrier_pre_real_null is -2.83 us (30%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 2.1x slower than the field

carrier_pre_real_fntable (13.61 us) is 2.1x the fastest (6.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} vs {carrier_pre_real_fntable} (25% apart)

The field splits into a fast tier {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} and a slow tier {carrier_pre_real_fntable} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_real_null** at 6584.8 ns median (-30.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.07x (fastest 6584.8 ns, slowest 13611.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 10369ns | 10371ns | 10224ns | 10339ns | 10485ns | -13.93% |
| carrier_pre_real_fntable | 15904ns | 16193ns | 14058ns | 16165ns | 16435ns | +32.02% |
| carrier_pre_real_null | 9180ns | 9180ns | 9013ns | 9172ns | 9275ns | -23.80% |
| carrier_pre_real_regcache | 13408ns | 13427ns | 13195ns | 13385ns | 13550ns | +11.30% |
| carrier_pre_real_switch | 12047ns | 12014ns | 11919ns | 11996ns | 12187ns | base |
| carrier_pre_real_threaded | 12066ns | 11864ns | 11681ns | 11816ns | 12634ns | +0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 7924ns | 7808ns | 8005ns | -16.13% | 0.032 |
| carrier_pre_real_fntable | 13374ns | 11929ns | 13762ns | +41.55% | 0.019 |
| carrier_pre_real_null | 6595ns | 6558ns | 6638ns | -30.19% | 0.039 |
| carrier_pre_real_regcache | 10883ns | 10795ns | 10938ns | +15.19% | 0.024 |
| carrier_pre_real_switch | 9448ns | 9306ns | 9569ns | base | 0.027 |
| carrier_pre_real_threaded | 9267ns | 9063ns | 9452ns | -1.91% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_real_direct | 276320 | 788768 | 0.350 | 0.98× |
| carrier_pre_real_fntable | 314322 | 831454 | 0.378 | 1.12× |
| carrier_pre_real_null | 268122 | 1221956 | 0.219 | 0.95× |
| carrier_pre_real_regcache | 282228 | 1067206 | 0.264 | 1.00× |
| carrier_pre_real_switch | 281216 | 875782 | 0.321 | 1.00× |
| carrier_pre_real_threaded | 285392 | 963302 | 0.296 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.032 | 82.7% |
| carrier_pre_real_fntable | 0.019 | 48.2% |
| carrier_pre_real_null | 0.039 | 99.6% |
| carrier_pre_real_regcache | 0.024 | 60.3% |
| carrier_pre_real_switch | 0.027 | 69.3% |
| carrier_pre_real_threaded | 0.028 | 70.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 10369ns | 10369ns | -13.93% |
| carrier_pre_real_fntable | 15904ns | 15904ns | +32.02% |
| carrier_pre_real_null | 9180ns | 9180ns | -23.80% |
| carrier_pre_real_regcache | 13408ns | 13408ns | +11.30% |
| carrier_pre_real_switch | 12047ns | 12047ns | base |
| carrier_pre_real_threaded | 12066ns | 12066ns | +0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 9462ns | base | --- | [9313, 9569] | --- | --- | --- | --- |
| carrier_pre_real_direct | 7934ns | -1515.3ns (-16.0%) | [-1645, -1411]ns | [7833, 8005] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_fntable | 13611ns | +4158.1ns (+43.9%) | [+3225, +4394]ns | [12748, 13762] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_null | 6585ns | -2833.6ns (-29.9%) | [-3005, -2719]ns | [6563, 6638] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_regcache | 10881ns | +1418.5ns (+15.0%) | [+1297, +1589]ns | [10829, 10938] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_threaded | 9259ns | -188.9ns (-2.0%) | [-326, -28]ns | [9091, 9452] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 9601ns | -17.0% | +24.3% | -31.7% | +12.4% | -1.0% |
| 2 | 9320ns | -15.7% | +48.6% | -29.2% | +17.4% | +0.5% |
| 3 | 9537ns | -16.5% | +43.4% | -31.1% | +14.7% | -1.4% |
| 4 | 9466ns | -17.5% | +44.1% | -30.5% | +15.0% | -3.3% |
| 5 | 9306ns | -15.1% | +45.8% | -29.2% | +16.7% | -2.6% |
| 6 | 9459ns | -15.0% | +43.6% | -29.4% | +15.0% | -3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | -0.302 | moderate- |
| carrier_pre_real_fntable | -0.144 | ok |
| carrier_pre_real_null | 0.042 | ok |
| carrier_pre_real_regcache | -0.128 | ok |
| carrier_pre_real_switch | -0.490 | moderate- |
| carrier_pre_real_threaded | 0.463 | moderate+ |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 89690.3ns | 7924.1ns | 1131.9% | HIGH |
| carrier_pre_real_fntable | 94611.8ns | 13373.8ns | 707.4% | HIGH |
| carrier_pre_real_null | 86948.3ns | 6595.3ns | 1318.3% | HIGH |
| carrier_pre_real_regcache | 87909.8ns | 10882.8ns | 807.8% | HIGH |
| carrier_pre_real_switch | 88627.7ns | 9448.1ns | 938.1% | HIGH |
| carrier_pre_real_threaded | 91422.9ns | 9267.2ns | 986.5% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 7807.5-8005.5 ns)
   7807.5 |####################
   7817.4 |
   7827.3 |
   7837.2 |
   7847.1 |
   7857.0 |####################
   7866.9 |
   7876.8 |
   7886.7 |
   7896.6 |####################
   7906.5 |
   7916.4 |
   7926.3 |
   7936.2 |
   7946.1 |
   7956.0 |
   7965.9 |########################################
   7975.8 |
   7985.7 |
   7995.6 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 11929.2-13762.1 ns)
  11929.2 |####################
  12020.8 |
  12112.5 |
  12204.1 |
  12295.8 |
  12387.4 |
  12479.1 |
  12570.7 |
  12662.4 |
  12754.0 |
  12845.7 |
  12937.3 |
  13028.9 |
  13120.6 |
  13212.2 |
  13303.9 |
  13395.5 |
  13487.2 |####################
  13578.8 |########################################
  13670.5 |####################
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 6558.3-6637.7 ns)
   6558.3 |########################################
   6562.3 |
   6566.2 |########################################
   6570.2 |
   6574.2 |########################################
   6578.1 |
   6582.1 |
   6586.1 |
   6590.1 |########################################
   6594.0 |########################################
   6598.0 |
   6602.0 |
   6605.9 |
   6609.9 |
   6613.9 |
   6617.9 |
   6621.8 |
   6625.8 |
   6629.8 |
   6633.7 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 10795.4-10938.1 ns)
  10795.4 |########################################
  10802.5 |
  10809.7 |
  10816.8 |
  10823.9 |
  10831.1 |
  10838.2 |
  10845.3 |
  10852.5 |
  10859.6 |########################################
  10866.8 |
  10873.9 |########################################
  10881.0 |########################################
  10888.2 |
  10895.3 |
  10902.4 |
  10909.6 |
  10916.7 |
  10923.8 |
  10931.0 |########################################
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 9305.8-9568.8 ns)
   9305.8 |########################################
   9318.9 |########################################
   9332.1 |
   9345.2 |
   9358.4 |
   9371.5 |
   9384.7 |
   9397.8 |
   9411.0 |
   9424.1 |
   9437.3 |
   9450.4 |########################################
   9463.6 |########################################
   9476.7 |
   9489.9 |
   9503.0 |
   9516.2 |
   9529.3 |########################################
   9542.5 |
   9555.6 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 9062.9-9451.9 ns)
   9062.9 |########################################
   9082.4 |
   9101.8 |########################################
   9121.2 |
   9140.7 |########################################
   9160.1 |
   9179.6 |
   9199.1 |
   9218.5 |
   9238.0 |
   9257.4 |
   9276.9 |
   9296.3 |
   9315.8 |
   9335.2 |
   9354.7 |########################################
   9374.1 |
   9393.6 |########################################
   9413.0 |
   9432.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=1135.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=693.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=1319.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=808.2% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=945.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=986.6% of algo (FFI overhead may distort results)
