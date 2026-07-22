# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 18% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (6.66 us) leads carrier_pre_real_direct (7.86 us) by 18%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 31% (significant)

carrier_pre_real_null is -2.94 us (31%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 2.0x slower than the field

carrier_pre_real_fntable (13.60 us) is 2.0x the fastest (6.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_pre_real_null** at 6658.4 ns median (-30.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.04x (fastest 6658.4 ns, slowest 13601.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 10440ns | 10447ns | 10372ns | 10440ns | 10475ns | -14.81% |
| carrier_pre_real_fntable | 15831ns | 16195ns | 14195ns | 16073ns | 16286ns | +29.18% |
| carrier_pre_real_null | 9226ns | 9180ns | 9132ns | 9173ns | 9352ns | -24.72% |
| carrier_pre_real_regcache | 13438ns | 13455ns | 13205ns | 13448ns | 13538ns | +9.65% |
| carrier_pre_real_switch | 12256ns | 12168ns | 12055ns | 12136ns | 12535ns | base |
| carrier_pre_real_threaded | 11960ns | 11922ns | 11824ns | 11905ns | 12111ns | -2.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 7866ns | 7829ns | 7897ns | -18.53% | 0.033 |
| carrier_pre_real_fntable | 13321ns | 11926ns | 13734ns | +37.96% | 0.019 |
| carrier_pre_real_null | 6667ns | 6590ns | 6724ns | -30.95% | 0.038 |
| carrier_pre_real_regcache | 10870ns | 10658ns | 10960ns | +12.59% | 0.024 |
| carrier_pre_real_switch | 9655ns | 9538ns | 9856ns | base | 0.027 |
| carrier_pre_real_threaded | 9285ns | 9182ns | 9406ns | -3.83% | 0.028 |

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.033 | 83.8% |
| carrier_pre_real_fntable | 0.019 | 48.4% |
| carrier_pre_real_null | 0.038 | 99.0% |
| carrier_pre_real_regcache | 0.024 | 60.5% |
| carrier_pre_real_switch | 0.027 | 68.9% |
| carrier_pre_real_threaded | 0.028 | 71.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 10440ns | 10440ns | -14.81% |
| carrier_pre_real_fntable | 15831ns | 15831ns | +29.18% |
| carrier_pre_real_null | 9226ns | 9226ns | -24.72% |
| carrier_pre_real_regcache | 13438ns | 13438ns | +9.65% |
| carrier_pre_real_switch | 12256ns | 12256ns | base |
| carrier_pre_real_threaded | 11960ns | 11960ns | -2.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 9568ns | base | --- | [9541, 9856] | --- | --- | --- | --- |
| carrier_pre_real_direct | 7862ns | -1695.6ns (-17.7%) | [-2015, -1655]ns | [7840, 7897] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_fntable | 13602ns | +3843.9ns (+40.2%) | [+3034, +4119]ns | [12626, 13734] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_null | 6658ns | -2935.2ns (-30.7%) | [-3132, -2898]ns | [6618, 6724] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_regcache | 10889ns | +1261.7ns (+13.2%) | [+1003, +1381]ns | [10762, 10960] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_real_threaded | 9262ns | -324.4ns (-3.4%) | [-585, -200]ns | [9188, 9406] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 9545ns | -17.2% | +25.0% | -31.0% | +14.0% | -3.1% |
| 2 | 9734ns | -19.6% | +40.1% | -30.6% | +12.6% | -3.6% |
| 3 | 9977ns | -21.3% | +36.9% | -32.9% | +9.2% | -8.0% |
| 4 | 9568ns | -17.9% | +41.8% | -30.4% | +13.6% | -3.9% |
| 5 | 9568ns | -17.5% | +44.3% | -30.5% | +11.4% | -1.4% |
| 6 | 9538ns | -17.5% | +39.7% | -30.3% | +14.9% | -2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | -0.241 | moderate- |
| carrier_pre_real_fntable | -0.050 | ok |
| carrier_pre_real_null | -0.282 | moderate- |
| carrier_pre_real_regcache | -0.243 | moderate- |
| carrier_pre_real_switch | 0.043 | ok |
| carrier_pre_real_threaded | -0.368 | moderate- |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 90349.4ns | 7866.5ns | 1148.5% | HIGH |
| carrier_pre_real_fntable | 94360.8ns | 13320.6ns | 708.4% | HIGH |
| carrier_pre_real_null | 87823.2ns | 6666.7ns | 1317.3% | HIGH |
| carrier_pre_real_regcache | 88136.0ns | 10870.4ns | 810.8% | HIGH |
| carrier_pre_real_switch | 91224.2ns | 9655.1ns | 944.8% | HIGH |
| carrier_pre_real_threaded | 90874.5ns | 9285.1ns | 978.7% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 7828.7-7896.9 ns)
   7828.7 |########################################
   7832.1 |
   7835.5 |
   7838.9 |
   7842.3 |
   7845.7 |
   7849.1 |########################################
   7852.6 |########################################
   7856.0 |
   7859.4 |
   7862.8 |
   7866.2 |########################################
   7869.6 |
   7873.0 |
   7876.4 |
   7879.8 |
   7883.2 |
   7886.6 |
   7890.0 |########################################
   7893.4 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 11926.2-13734.0 ns)
  11926.2 |####################
  12016.6 |
  12107.0 |
  12197.4 |
  12287.8 |
  12378.1 |
  12468.5 |
  12558.9 |
  12649.3 |
  12739.7 |
  12830.1 |
  12920.5 |
  13010.9 |
  13101.2 |
  13191.6 |
  13282.0 |####################
  13372.4 |
  13462.8 |
  13553.2 |########################################
  13643.6 |####################
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 6589.6-6723.8 ns)
   6589.6 |########################################
   6596.3 |
   6603.0 |
   6609.7 |
   6616.4 |
   6623.1 |
   6629.8 |
   6636.6 |
   6643.3 |########################################
   6650.0 |########################################
   6656.7 |
   6663.4 |########################################
   6670.1 |
   6676.8 |
   6683.5 |
   6690.2 |########################################
   6696.9 |
   6703.6 |
   6710.3 |
   6717.0 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 10657.5-10959.8 ns)
  10657.5 |####################
  10672.6 |
  10687.7 |
  10702.8 |
  10718.0 |
  10733.1 |
  10748.2 |
  10763.3 |
  10778.4 |
  10793.5 |
  10808.6 |
  10823.8 |
  10838.9 |
  10854.0 |####################
  10869.1 |
  10884.2 |########################################
  10899.3 |
  10914.5 |
  10929.6 |
  10944.7 |####################
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 9538.3-9855.7 ns)
   9538.3 |########################################
   9554.2 |########################################
   9570.0 |
   9585.9 |
   9601.8 |
   9617.6 |
   9633.5 |
   9649.4 |
   9665.2 |
   9681.1 |
   9697.0 |
   9712.8 |
   9728.7 |####################
   9744.6 |
   9760.4 |
   9776.3 |
   9792.2 |
   9808.0 |
   9823.9 |
   9839.8 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 9182.5-9405.9 ns)
   9182.5 |########################################
   9193.7 |
   9204.8 |
   9216.0 |
   9227.2 |
   9238.3 |
   9249.5 |####################
   9260.7 |
   9271.8 |####################
   9283.0 |
   9294.2 |
   9305.3 |
   9316.5 |
   9327.7 |
   9338.8 |
   9350.0 |
   9361.2 |
   9372.3 |####################
   9383.5 |
   9394.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=1152.2% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=693.1% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=1315.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=807.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=953.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=982.6% of algo (FFI overhead may distort results)
