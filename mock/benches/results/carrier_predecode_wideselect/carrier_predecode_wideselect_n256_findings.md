# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null beats baseline by 31% (significant)

carrier_pre_wideselect_null is -2.69 us (31%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_switch shows alternating (throttle bounce) (autocorr -0.58)

carrier_pre_wideselect_switch's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 6029.1 ns median (-31.0% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.88x (fastest 6029.1 ns, slowest 11312.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 8755ns | 8818ns | 8285ns | 8789ns | 8939ns | -21.47% |
| carrier_pre_wideselect_fntable | 13557ns | 13740ns | 12469ns | 13685ns | 13909ns | +21.60% |
| carrier_pre_wideselect_null | 8452ns | 8491ns | 8055ns | 8480ns | 8608ns | -24.19% |
| carrier_pre_wideselect_regcache | 12220ns | 12346ns | 11428ns | 12263ns | 12550ns | +9.60% |
| carrier_pre_wideselect_switch | 11149ns | 11221ns | 10759ns | 11161ns | 11326ns | base |
| carrier_pre_wideselect_threaded | 10309ns | 10420ns | 9779ns | 10298ns | 10591ns | -7.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 6421ns | 6072ns | 6549ns | -26.27% | 0.040 |
| carrier_pre_wideselect_fntable | 11167ns | 10345ns | 11438ns | +28.23% | 0.023 |
| carrier_pre_wideselect_null | 5975ns | 5631ns | 6081ns | -31.39% | 0.043 |
| carrier_pre_wideselect_regcache | 9702ns | 9045ns | 9935ns | +11.40% | 0.026 |
| carrier_pre_wideselect_switch | 8709ns | 8488ns | 8815ns | base | 0.029 |
| carrier_pre_wideselect_threaded | 7812ns | 7400ns | 8029ns | -10.31% | 0.033 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 282237 | 1030904 | 0.274 | 0.95× |
| carrier_pre_wideselect_fntable | 311071 | 1002704 | 0.310 | 1.04× |
| carrier_pre_wideselect_null | 283108 | 1378498 | 0.205 | 0.95× |
| carrier_pre_wideselect_regcache | 307548 | 1262424 | 0.244 | 1.03× |
| carrier_pre_wideselect_switch | 298524 | 976711 | 0.306 | 1.00× |
| carrier_pre_wideselect_threaded | 291975 | 1187668 | 0.246 | 0.98× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.040 | 87.0% |
| carrier_pre_wideselect_fntable | 0.023 | 49.8% |
| carrier_pre_wideselect_null | 0.042 | 93.4% |
| carrier_pre_wideselect_regcache | 0.026 | 57.6% |
| carrier_pre_wideselect_switch | 0.029 | 64.5% |
| carrier_pre_wideselect_threaded | 0.032 | 71.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 8755ns | 8755ns | -21.47% |
| carrier_pre_wideselect_fntable | 13557ns | 13557ns | +21.60% |
| carrier_pre_wideselect_null | 8452ns | 8452ns | -24.19% |
| carrier_pre_wideselect_regcache | 12220ns | 12220ns | +9.60% |
| carrier_pre_wideselect_switch | 11149ns | 11149ns | base |
| carrier_pre_wideselect_threaded | 10309ns | 10309ns | -7.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 8736ns | base | --- | [8576, 8815] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 6469ns | -2266.1ns (-25.9%) | [-2570, -2028]ns | [6245, 6549] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 11312ns | +2624.8ns (+30.0%) | [+2020, +2730]ns | [10752, 11438] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 6029ns | -2691.1ns (-30.8%) | [-2962, -2549]ns | [5815, 6081] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 9776ns | +1068.3ns (+12.2%) | [+578, +1331]ns | [9393, 9935] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 7884ns | -859.0ns (-9.8%) | [-1215, -619]ns | [7521, 8029] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 8805ns | -27.1% | +17.5% | -36.0% | +10.6% | -11.0% |
| 2 | 8665ns | -25.3% | +31.9% | -30.1% | +13.0% | -8.5% |
| 3 | 8825ns | -31.2% | +28.3% | -31.0% | +2.5% | -8.5% |
| 4 | 8488ns | -21.9% | +31.5% | -29.3% | +16.3% | -5.9% |
| 5 | 8751ns | -26.0% | +30.8% | -31.4% | +11.6% | -15.4% |
| 6 | 8720ns | -25.9% | +29.6% | -30.3% | +14.6% | -12.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.447 | moderate- |
| carrier_pre_wideselect_fntable | -0.160 | ok |
| carrier_pre_wideselect_null | -0.092 | ok |
| carrier_pre_wideselect_regcache | -0.246 | moderate- |
| carrier_pre_wideselect_switch | -0.581 | HIGH- (thermal bounce) |
| carrier_pre_wideselect_threaded | 0.250 | moderate+ |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 88132.0ns | 6421.0ns | 1372.6% | HIGH |
| carrier_pre_wideselect_fntable | 91890.1ns | 11167.4ns | 822.8% | HIGH |
| carrier_pre_wideselect_null | 88211.2ns | 5975.1ns | 1476.3% | HIGH |
| carrier_pre_wideselect_regcache | 92117.6ns | 9701.6ns | 949.5% | HIGH |
| carrier_pre_wideselect_switch | 91776.7ns | 8709.0ns | 1053.8% | HIGH |
| carrier_pre_wideselect_threaded | 90253.8ns | 7811.5ns | 1155.4% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 6072.1-6549.1 ns)
   6072.1 |#############
   6096.0 |
   6119.8 |
   6143.7 |
   6167.5 |
   6191.4 |
   6215.2 |
   6239.1 |
   6262.9 |
   6286.8 |
   6310.6 |
   6334.5 |
   6358.3 |
   6382.2 |
   6406.0 |#############
   6429.9 |
   6453.7 |########################################
   6477.6 |
   6501.4 |
   6525.3 |
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 10345.4-11438.3 ns)
  10345.4 |####################
  10400.0 |
  10454.7 |
  10509.3 |
  10564.0 |
  10618.6 |
  10673.3 |
  10727.9 |
  10782.6 |
  10837.2 |
  10891.8 |
  10946.5 |
  11001.1 |
  11055.8 |
  11110.4 |####################
  11165.1 |
  11219.7 |
  11274.4 |########################################
  11329.0 |
  11383.7 |####################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 5631.2-6081.4 ns)
   5631.2 |####################
   5653.7 |
   5676.2 |
   5698.7 |
   5721.2 |
   5743.8 |
   5766.3 |
   5788.8 |
   5811.3 |
   5833.8 |
   5856.3 |
   5878.8 |
   5901.3 |
   5923.9 |
   5946.4 |
   5968.9 |
   5991.4 |########################################
   6013.9 |
   6036.4 |####################
   6058.9 |####################
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 9045.4-9935.4 ns)
   9045.4 |####################
   9089.9 |
   9134.4 |
   9178.9 |
   9223.4 |
   9267.9 |
   9312.4 |
   9356.9 |
   9401.4 |
   9445.9 |
   9490.4 |
   9534.9 |
   9579.4 |
   9623.9 |
   9668.4 |
   9712.9 |####################
   9757.4 |########################################
   9801.9 |
   9846.4 |####################
   9890.9 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 8487.9-8815.0 ns)
   8487.9 |########################################
   8504.3 |
   8520.6 |
   8537.0 |
   8553.3 |
   8569.7 |
   8586.0 |
   8602.4 |
   8618.7 |
   8635.1 |
   8651.5 |########################################
   8667.8 |
   8684.2 |
   8700.5 |
   8716.9 |########################################
   8733.2 |
   8749.6 |########################################
   8765.9 |
   8782.3 |
   8798.6 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 7400.4-8029.4 ns)
   7400.4 |########################################
   7431.8 |
   7463.3 |
   7494.7 |
   7526.2 |
   7557.6 |
   7589.1 |
   7620.5 |########################################
   7652.0 |
   7683.4 |
   7714.9 |
   7746.3 |
   7777.8 |
   7809.2 |########################################
   7840.7 |
   7872.1 |
   7903.6 |########################################
   7935.0 |
   7966.5 |########################################
   7997.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=1358.5% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=812.6% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=1459.3% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=950.0% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=1050.2% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=1142.8% of algo (FFI overhead may distort results)
