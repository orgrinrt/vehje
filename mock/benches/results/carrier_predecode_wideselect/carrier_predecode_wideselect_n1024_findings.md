# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null dominates: 11% faster than the next best (carrier_pre_wideselect_direct)

carrier_pre_wideselect_null (25.35 us) leads carrier_pre_wideselect_direct (28.07 us) by 11%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_wideselect_null beats baseline by 32% (significant)

carrier_pre_wideselect_null is -11.76 us (32%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_switch shows alternating (throttle bounce) (autocorr -0.54)

carrier_pre_wideselect_switch's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 25345.2 ns median (-31.7% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.95x (fastest 25345.2 ns, slowest 49370.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 30525ns | 30407ns | 30035ns | 30284ns | 31131ns | -22.30% |
| carrier_pre_wideselect_fntable | 50926ns | 51696ns | 48060ns | 51121ns | 52067ns | +29.63% |
| carrier_pre_wideselect_null | 27698ns | 27651ns | 27195ns | 27530ns | 28201ns | -29.50% |
| carrier_pre_wideselect_regcache | 42488ns | 42436ns | 41975ns | 42309ns | 43014ns | +8.15% |
| carrier_pre_wideselect_switch | 39287ns | 39507ns | 38321ns | 39303ns | 39745ns | base |
| carrier_pre_wideselect_threaded | 34977ns | 35158ns | 34179ns | 34865ns | 35544ns | -10.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 28184ns | 27722ns | 28748ns | -23.63% | 0.036 |
| carrier_pre_wideselect_fntable | 48657ns | 45949ns | 49748ns | +31.85% | 0.021 |
| carrier_pre_wideselect_null | 25370ns | 24907ns | 25821ns | -31.26% | 0.040 |
| carrier_pre_wideselect_regcache | 40178ns | 39711ns | 40691ns | +8.87% | 0.025 |
| carrier_pre_wideselect_switch | 36904ns | 36051ns | 37366ns | base | 0.028 |
| carrier_pre_wideselect_threaded | 32671ns | 31937ns | 33210ns | -11.47% | 0.031 |

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.036 | 88.7% |
| carrier_pre_wideselect_fntable | 0.021 | 50.4% |
| carrier_pre_wideselect_null | 0.040 | 98.3% |
| carrier_pre_wideselect_regcache | 0.026 | 62.1% |
| carrier_pre_wideselect_switch | 0.028 | 67.1% |
| carrier_pre_wideselect_threaded | 0.031 | 75.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 30525ns | 30525ns | -22.30% |
| carrier_pre_wideselect_fntable | 50926ns | 50926ns | +29.63% |
| carrier_pre_wideselect_null | 27698ns | 27698ns | -29.50% |
| carrier_pre_wideselect_regcache | 42488ns | 42488ns | +8.15% |
| carrier_pre_wideselect_switch | 39287ns | 39287ns | base |
| carrier_pre_wideselect_threaded | 34977ns | 34977ns | -10.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 37101ns | base | --- | [36246, 37366] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 28075ns | -8952.1ns (-24.1%) | [-9354, -7855]ns | [27730, 28748] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 49371ns | +12285.4ns (+33.1%) | [+9648, +13324]ns | [46852, 49748] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 25345ns | -11762.3ns (-31.7%) | [-12028, -10813]ns | [24944, 25821] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 40124ns | +3233.5ns (+8.7%) | [+2534, +4052]ns | [39718, 40691] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 32822ns | -4277.2ns (-11.5%) | [-5223, -3200]ns | [31981, 33210] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 37302ns | -25.6% | +23.2% | -31.6% | +8.6% | -14.1% |
| 2 | 36441ns | -20.3% | +36.3% | -28.8% | +12.2% | -9.8% |
| 3 | 37097ns | -24.6% | +32.8% | -32.7% | +7.1% | -10.5% |
| 4 | 36051ns | -23.1% | +37.2% | -30.9% | +10.2% | -7.8% |
| 5 | 37430ns | -24.0% | +33.1% | -31.4% | +6.5% | -12.4% |
| 6 | 37105ns | -24.0% | +28.7% | -32.2% | +8.8% | -13.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.462 | moderate- |
| carrier_pre_wideselect_fntable | -0.147 | ok |
| carrier_pre_wideselect_null | -0.190 | ok |
| carrier_pre_wideselect_regcache | 0.172 | ok |
| carrier_pre_wideselect_switch | -0.537 | HIGH- (thermal bounce) |
| carrier_pre_wideselect_threaded | 0.154 | ok |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 100933.8ns | 28184.2ns | 358.1% | HIGH |
| carrier_pre_wideselect_fntable | 101704.3ns | 48657.0ns | 209.0% | HIGH |
| carrier_pre_wideselect_null | 104212.8ns | 25369.9ns | 410.8% | HIGH |
| carrier_pre_wideselect_regcache | 122391.9ns | 40177.5ns | 304.6% | HIGH |
| carrier_pre_wideselect_switch | 113253.7ns | 36904.4ns | 306.9% | HIGH |
| carrier_pre_wideselect_threaded | 100830.8ns | 32671.1ns | 308.6% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 27721.7-28748.1 ns)
  27721.7 |########################################
  27773.0 |
  27824.3 |
  27875.7 |
  27927.0 |####################
  27978.3 |
  28029.6 |
  28080.9 |
  28132.3 |
  28183.6 |####################
  28234.9 |
  28286.2 |
  28337.5 |
  28388.9 |####################
  28440.2 |
  28491.5 |
  28542.8 |
  28594.1 |
  28645.5 |
  28696.8 |
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 45948.8-49748.5 ns)
  45948.8 |########################################
  46138.8 |
  46328.8 |
  46518.8 |
  46708.7 |
  46898.7 |
  47088.7 |
  47278.7 |
  47468.7 |
  47658.7 |########################################
  47848.7 |
  48038.6 |
  48228.6 |
  48418.6 |
  48608.6 |
  48798.6 |
  48988.6 |
  49178.5 |########################################
  49368.5 |########################################
  49558.5 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 24907.1-25820.6 ns)
  24907.1 |########################################
  24952.8 |########################################
  24998.4 |
  25044.1 |
  25089.8 |
  25135.5 |########################################
  25181.1 |
  25226.8 |
  25272.5 |
  25318.2 |
  25363.8 |
  25409.5 |
  25455.2 |
  25500.9 |########################################
  25546.5 |
  25592.2 |
  25637.9 |########################################
  25683.6 |
  25729.2 |
  25774.9 |
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 39711.2-40690.6 ns)
  39711.2 |########################################
  39760.2 |
  39809.1 |
  39858.1 |####################
  39907.1 |
  39956.1 |
  40005.0 |
  40054.0 |
  40103.0 |
  40152.0 |
  40200.9 |
  40249.9 |
  40298.9 |
  40347.8 |####################
  40396.8 |
  40445.8 |
  40494.8 |####################
  40543.7 |
  40592.7 |
  40641.7 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 36050.8-37366.4 ns)
  36050.8 |########################################
  36116.6 |
  36182.4 |
  36248.1 |
  36313.9 |
  36379.7 |########################################
  36445.5 |
  36511.3 |
  36577.1 |
  36642.8 |
  36708.6 |
  36774.4 |
  36840.2 |
  36906.0 |
  36971.8 |
  37037.5 |########################################
  37103.3 |########################################
  37169.1 |
  37234.9 |
  37300.7 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 31936.7-33209.6 ns)
  31936.7 |########################################
  32000.3 |########################################
  32064.0 |
  32127.6 |
  32191.3 |
  32254.9 |
  32318.6 |
  32382.2 |
  32445.9 |
  32509.5 |
  32573.2 |
  32636.8 |
  32700.4 |
  32764.1 |########################################
  32827.7 |########################################
  32891.4 |
  32955.0 |
  33018.7 |
  33082.3 |
  33146.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=363.2% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=209.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=410.7% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=305.1% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=305.7% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=308.6% of algo (FFI overhead may distort results)
