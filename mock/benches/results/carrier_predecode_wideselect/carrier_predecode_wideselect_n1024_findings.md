# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null dominates: 11% faster than the next best (carrier_pre_wideselect_direct)

carrier_pre_wideselect_null (23.61 us) leads carrier_pre_wideselect_direct (26.14 us) by 11%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_wideselect_null beats baseline by 32% (significant)

carrier_pre_wideselect_null is -11.15 us (32%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_direct shows alternating (throttle bounce) (autocorr -0.68)

carrier_pre_wideselect_direct's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 23614.0 ns median (-32.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.92x (fastest 23614.0 ns, slowest 45284.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 28294ns | 28300ns | 28049ns | 28257ns | 28473ns | -23.80% |
| carrier_pre_wideselect_fntable | 47635ns | 47424ns | 46848ns | 47306ns | 48523ns | +28.29% |
| carrier_pre_wideselect_null | 25895ns | 25748ns | 25614ns | 25704ns | 26324ns | -30.26% |
| carrier_pre_wideselect_regcache | 40584ns | 40142ns | 39538ns | 40088ns | 41850ns | +9.30% |
| carrier_pre_wideselect_switch | 37132ns | 37070ns | 36735ns | 36964ns | 37581ns | base |
| carrier_pre_wideselect_threaded | 34013ns | 33810ns | 32855ns | 33604ns | 35206ns | -8.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 26145ns | 25910ns | 26324ns | -25.25% | 0.039 |
| carrier_pre_wideselect_fntable | 45472ns | 44732ns | 46300ns | +30.01% | 0.023 |
| carrier_pre_wideselect_null | 23751ns | 23484ns | 24143ns | -32.10% | 0.043 |
| carrier_pre_wideselect_regcache | 38379ns | 37425ns | 39531ns | +9.73% | 0.027 |
| carrier_pre_wideselect_switch | 34977ns | 34602ns | 35403ns | base | 0.029 |
| carrier_pre_wideselect_threaded | 31796ns | 30716ns | 32893ns | -9.09% | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 424933 | 1441374 | 0.295 | 0.95× |
| carrier_pre_wideselect_fntable | 443678 | 1337018 | 0.332 | 0.99× |
| carrier_pre_wideselect_null | 387628 | 1785443 | 0.217 | 0.86× |
| carrier_pre_wideselect_regcache | 489747 | 1944448 | 0.252 | 1.09× |
| carrier_pre_wideselect_switch | 448425 | 1396776 | 0.321 | 1.00× |
| carrier_pre_wideselect_threaded | 401052 | 1550596 | 0.259 | 0.89× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.044 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.039 | 89.8% |
| carrier_pre_wideselect_fntable | 0.023 | 51.9% |
| carrier_pre_wideselect_null | 0.043 | 99.4% |
| carrier_pre_wideselect_regcache | 0.027 | 61.8% |
| carrier_pre_wideselect_switch | 0.029 | 67.2% |
| carrier_pre_wideselect_threaded | 0.032 | 74.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 28294ns | 28294ns | -23.80% |
| carrier_pre_wideselect_fntable | 47635ns | 47635ns | +28.29% |
| carrier_pre_wideselect_null | 25895ns | 25895ns | -30.26% |
| carrier_pre_wideselect_regcache | 40584ns | 40584ns | +9.30% |
| carrier_pre_wideselect_switch | 37132ns | 37132ns | base |
| carrier_pre_wideselect_threaded | 34013ns | 34013ns | -8.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 34923ns | base | --- | [34605, 35403] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 26142ns | -8784.0ns (-25.2%) | [-9348, -8362]ns | [25970, 26324] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 45285ns | +10370.6ns (+29.7%) | [+10185, +10931]ns | [44833, 46300] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 23614ns | -11149.4ns (-31.9%) | [-11770, -10759]ns | [23495, 24143] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 37978ns | +3189.8ns (+9.1%) | [+2590, +4426]ns | [37627, 39531] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 31611ns | -3773.0ns (-10.8%) | [-4018, -1751]ns | [30884, 32893] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 34602ns | -23.8% | +29.3% | -32.1% | +13.1% | -1.8% |
| 2 | 34607ns | -25.1% | +30.5% | -30.3% | +9.3% | -11.2% |
| 3 | 35160ns | -25.2% | +29.1% | -32.5% | +8.4% | -10.4% |
| 4 | 35609ns | -26.9% | +31.7% | -34.0% | +12.2% | -10.9% |
| 5 | 35198ns | -25.9% | +29.8% | -31.3% | +6.3% | -11.8% |
| 6 | 34686ns | -24.5% | +29.5% | -32.3% | +9.1% | -8.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.678 | HIGH- (thermal bounce) |
| carrier_pre_wideselect_fntable | 0.111 | ok |
| carrier_pre_wideselect_null | -0.613 | HIGH- (thermal bounce) |
| carrier_pre_wideselect_regcache | -0.363 | moderate- |
| carrier_pre_wideselect_switch | 0.310 | moderate+ |
| carrier_pre_wideselect_threaded | -0.299 | moderate- |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 107750.8ns | 26145.4ns | 412.1% | HIGH |
| carrier_pre_wideselect_fntable | 94693.1ns | 45472.5ns | 208.2% | HIGH |
| carrier_pre_wideselect_null | 97548.2ns | 23750.6ns | 410.7% | HIGH |
| carrier_pre_wideselect_regcache | 117889.5ns | 38379.0ns | 307.2% | HIGH |
| carrier_pre_wideselect_switch | 107557.2ns | 34976.9ns | 307.5% | HIGH |
| carrier_pre_wideselect_threaded | 98197.1ns | 31796.4ns | 308.8% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 25910.4-26324.2 ns)
  25910.4 |########################################
  25931.1 |
  25951.8 |
  25972.5 |
  25993.2 |
  26013.8 |########################################
  26034.5 |
  26055.2 |
  26075.9 |########################################
  26096.6 |
  26117.3 |
  26138.0 |
  26158.7 |
  26179.3 |
  26200.0 |########################################
  26220.7 |
  26241.4 |
  26262.1 |
  26282.8 |########################################
  26303.5 |
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 44732.1-46300.2 ns)
  44732.1 |########################################
  44810.5 |
  44888.9 |########################################
  44967.3 |
  45045.7 |
  45124.1 |########################################
  45202.5 |
  45280.9 |
  45359.3 |########################################
  45437.7 |
  45516.1 |
  45594.6 |
  45673.0 |########################################
  45751.4 |
  45829.8 |
  45908.2 |
  45986.6 |
  46065.0 |
  46143.4 |
  46221.8 |
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 23483.8-24143.3 ns)
  23483.8 |########################################
  23516.8 |
  23549.8 |
  23582.7 |
  23615.7 |
  23648.7 |
  23681.6 |
  23714.6 |#############
  23747.6 |
  23780.6 |
  23813.5 |
  23846.5 |
  23879.5 |
  23912.5 |
  23945.5 |
  23978.4 |
  24011.4 |
  24044.4 |
  24077.3 |
  24110.3 |#############
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 37425.4-39531.4 ns)
  37425.4 |####################
  37530.7 |
  37636.0 |
  37741.3 |########################################
  37846.6 |
  37951.9 |
  38057.2 |####################
  38162.5 |
  38267.8 |
  38373.1 |
  38478.4 |
  38583.7 |
  38689.0 |
  38794.3 |
  38899.6 |
  39004.9 |
  39110.2 |####################
  39215.5 |
  39320.8 |
  39426.1 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 34602.1-35403.2 ns)
  34602.1 |########################################
  34642.2 |
  34682.2 |####################
  34722.3 |
  34762.3 |
  34802.4 |
  34842.4 |
  34882.5 |
  34922.5 |
  34962.6 |
  35002.6 |
  35042.7 |
  35082.7 |
  35122.8 |####################
  35162.8 |####################
  35202.9 |
  35242.9 |
  35283.0 |
  35323.0 |
  35363.1 |
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 30715.8-32893.3 ns)
  30715.8 |####################
  30824.7 |
  30933.6 |
  31042.4 |####################
  31151.3 |
  31260.2 |
  31369.1 |
  31477.9 |####################
  31586.8 |
  31695.7 |########################################
  31804.6 |
  31913.5 |
  32022.3 |
  32131.2 |
  32240.1 |
  32349.0 |
  32457.8 |
  32566.7 |
  32675.6 |
  32784.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=412.1% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=208.4% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=411.3% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=306.9% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=307.3% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=308.6% of algo (FFI overhead may distort results)
