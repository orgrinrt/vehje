# Entropy x locality surface: op_correlation {0,500,900} x locality_window {4,64,unbounded}, fixed predecoded switch dispatch

9 variants, 6 samples per variant.
Baseline: **carrier_ent_c0_w64**

## Highlights

Baseline for all deltas below: **carrier_ent_c0_w64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ent_c900_w64 beats baseline by 26% (significant)

carrier_ent_c900_w64 is -9.79 us (26%) faster than baseline carrier_ent_c0_w64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ent_c900_wmax shows alternating (throttle bounce) (autocorr -0.57)

carrier_ent_c900_wmax's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_ent_c0_wmax is inconsistent: worst-20% is 1.6x its best-20%

carrier_ent_c0_wmax's best 20% of batches run at 35.43 us but its worst 20% at 55.15 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_ent_c900_w64** at 26658.6 ns median (-28.1% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.72x (fastest 26658.6 ns, slowest 45975.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 40339ns | 40611ns | 38149ns | 40113ns | 41773ns | +2.17% |
| carrier_ent_c0_w64 | 39480ns | 39300ns | 37463ns | 39131ns | 41014ns | base |
| carrier_ent_c0_wmax | 44938ns | 39831ns | 37586ns | 39118ns | 57345ns | +13.82% |
| carrier_ent_c500_w4 | 37404ns | 37506ns | 36132ns | 37343ns | 38131ns | -5.26% |
| carrier_ent_c500_w64 | 37078ns | 37126ns | 34860ns | 36931ns | 38406ns | -6.09% |
| carrier_ent_c500_wmax | 35827ns | 35701ns | 34820ns | 35430ns | 36926ns | -9.25% |
| carrier_ent_c900_w4 | 47725ns | 48261ns | 45678ns | 47522ns | 49054ns | +20.88% |
| carrier_ent_c900_w64 | 29170ns | 28841ns | 28399ns | 28734ns | 30211ns | -26.11% |
| carrier_ent_c900_wmax | 29523ns | 29889ns | 28220ns | 29395ns | 30366ns | -25.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 38048ns | 35994ns | 39392ns | +2.16% | 0.027 |
| carrier_ent_c0_w64 | 37243ns | 35356ns | 38688ns | base | 0.027 |
| carrier_ent_c0_wmax | 42727ns | 35432ns | 55148ns | +14.72% | 0.024 |
| carrier_ent_c500_w4 | 35169ns | 33965ns | 35853ns | -5.57% | 0.029 |
| carrier_ent_c500_w64 | 34771ns | 32712ns | 35994ns | -6.64% | 0.029 |
| carrier_ent_c500_wmax | 33613ns | 32655ns | 34647ns | -9.75% | 0.030 |
| carrier_ent_c900_w4 | 45484ns | 43546ns | 46759ns | +22.13% | 0.023 |
| carrier_ent_c900_w64 | 26963ns | 26243ns | 27919ns | -27.60% | 0.038 |
| carrier_ent_c900_wmax | 27273ns | 26075ns | 28033ns | -26.77% | 0.038 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 460153 | 1372784 | 0.335 | 1.00× |
| carrier_ent_c0_w64 | 460935 | 1383766 | 0.333 | 1.00× |
| carrier_ent_c0_wmax | 515331 | 1383811 | 0.372 | 1.12× |
| carrier_ent_c500_w4 | 436987 | 1393542 | 0.314 | 0.95× |
| carrier_ent_c500_w64 | 422509 | 1384474 | 0.305 | 0.92× |
| carrier_ent_c500_wmax | 423112 | 1394857 | 0.303 | 0.92× |
| carrier_ent_c900_w4 | 425734 | 1082633 | 0.393 | 0.92× |
| carrier_ent_c900_w64 | 403159 | 1635772 | 0.246 | 0.87× |
| carrier_ent_c900_wmax | 402138 | 1634777 | 0.246 | 0.87× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_ent_c900_wmax; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ent_c0_w4 | 0.027 | 68.1% |
| carrier_ent_c0_w64 | 0.028 | 70.3% |
| carrier_ent_c0_wmax | 0.027 | 69.4% |
| carrier_ent_c500_w4 | 0.029 | 73.9% |
| carrier_ent_c500_w64 | 0.029 | 74.9% |
| carrier_ent_c500_wmax | 0.031 | 77.9% |
| carrier_ent_c900_w4 | 0.022 | 56.7% |
| carrier_ent_c900_w64 | 0.038 | 97.8% |
| carrier_ent_c900_wmax | 0.037 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ent_c0_w4 | 40339ns | 40339ns | +2.17% |
| carrier_ent_c0_w64 | 39480ns | 39480ns | base |
| carrier_ent_c0_wmax | 44938ns | 44938ns | +13.82% |
| carrier_ent_c500_w4 | 37404ns | 37404ns | -5.26% |
| carrier_ent_c500_w64 | 37078ns | 37078ns | -6.09% |
| carrier_ent_c500_wmax | 35827ns | 35827ns | -9.25% |
| carrier_ent_c900_w4 | 47725ns | 47725ns | +20.88% |
| carrier_ent_c900_w64 | 29170ns | 29170ns | -26.11% |
| carrier_ent_c900_wmax | 29523ns | 29523ns | -25.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ent_c0_w64 | 37071ns | base | --- | [35971, 38688] | --- | --- | --- | --- |
| carrier_ent_c0_w4 | 38300ns | no significant difference | [-1114, +1968]ns | [36452, 39392] | no | 0.2500 | 0.2188 | 0 |
| carrier_ent_c0_wmax | 37546ns | no significant difference | [-2500, +17723]ns | [35488, 55148] | no | 0.6875 | 0.6875 | 0 |
| carrier_ent_c500_w4 | 35265ns | -1981.5ns (-5.3%) | [-3461, -781]ns | [34388, 35853] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c500_w64 | 34825ns | -2249.9ns (-6.1%) | [-3821, -1346]ns | [33493, 35994] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c500_wmax | 33493ns | -3421.5ns (-9.2%) | [-5249, -2221]ns | [32698, 34647] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c900_w4 | 45975ns | +8578.6ns (+23.1%) | [+7239, +8904]ns | [43717, 46759] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c900_w64 | 26659ns | -9794.3ns (-26.4%) | [-11885, -9163]ns | [26311, 27919] | YES | 0.0417 | 0.0313 | 0 |
| carrier_ent_c900_wmax | 27628ns | -9513.4ns (-25.7%) | [-11656, -8740]ns | [26160, 28033] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ent_c0_w64 | carrier_ent_c0_w4 | carrier_ent_c0_wmax | carrier_ent_c500_w4 | carrier_ent_c500_w64 | carrier_ent_c500_wmax | carrier_ent_c900_w4 | carrier_ent_c900_w64 | carrier_ent_c900_wmax |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 36586ns | +5.7% | +53.3% | -7.2% | -3.9% | -4.8% | +19.0% | -26.4% | -24.2% |
| 2 | 35356ns | +4.4% | +3.7% | -0.9% | -7.5% | -7.6% | +24.1% | -25.3% | -26.3% |
| 3 | 36864ns | +4.3% | -3.6% | -3.4% | -3.5% | -8.2% | +23.8% | -25.4% | -23.4% |
| 4 | 37279ns | +2.4% | +3.1% | -4.8% | -7.5% | -11.1% | +24.3% | -29.6% | -26.1% |
| 5 | 39112ns | -8.0% | -9.4% | -11.0% | -12.4% | -16.3% | +19.2% | -32.6% | -32.9% |
| 6 | 38262ns | +4.9% | +41.6% | -5.7% | -4.8% | -10.0% | +22.5% | -25.9% | -27.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ent_c0_w4 | -0.540 | HIGH- (thermal bounce) |
| carrier_ent_c0_w64 | 0.451 | moderate+ |
| carrier_ent_c0_wmax | -0.126 | ok |
| carrier_ent_c500_w4 | -0.070 | ok |
| carrier_ent_c500_w64 | -0.422 | moderate- |
| carrier_ent_c500_wmax | -0.438 | moderate- |
| carrier_ent_c900_w4 | 0.540 | HIGH+ (drift/warm-up) |
| carrier_ent_c900_w64 | -0.307 | moderate- |
| carrier_ent_c900_wmax | -0.569 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_ent_c0_w4**: won 1/6, lost 5/6
- **carrier_ent_c0_wmax**: won 2/6, lost 4/6
- **carrier_ent_c500_w4**: won 6/6, lost 0/6
- **carrier_ent_c500_w64**: won 6/6, lost 0/6
- **carrier_ent_c500_wmax**: won 6/6, lost 0/6
- **carrier_ent_c900_w4**: won 0/6, lost 6/6
- **carrier_ent_c900_w64**: won 6/6, lost 0/6
- **carrier_ent_c900_wmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 115493.2ns | 38048.3ns | 303.5% | HIGH |
| carrier_ent_c0_w64 | 114181.5ns | 37243.3ns | 306.6% | HIGH |
| carrier_ent_c0_wmax | 124663.7ns | 42727.2ns | 291.8% | HIGH |
| carrier_ent_c500_w4 | 108003.1ns | 35168.6ns | 307.1% | HIGH |
| carrier_ent_c500_w64 | 107151.2ns | 34770.8ns | 308.2% | HIGH |
| carrier_ent_c500_wmax | 103801.6ns | 33612.8ns | 308.8% | HIGH |
| carrier_ent_c900_w4 | 93674.2ns | 45483.8ns | 206.0% | HIGH |
| carrier_ent_c900_w64 | 102838.8ns | 26962.6ns | 381.4% | HIGH |
| carrier_ent_c900_wmax | 103911.4ns | 27273.4ns | 381.0% | HIGH |

## Distribution (algo ns)

```
carrier_ent_c0_w4 (n=6, range 35994.2-39392.1 ns)
  35994.2 |########################################
  36164.1 |
  36334.0 |
  36503.9 |
  36673.8 |
  36843.7 |########################################
  37013.6 |
  37183.4 |
  37353.3 |
  37523.2 |
  37693.1 |
  37863.0 |
  38032.9 |########################################
  38202.8 |
  38372.7 |########################################
  38542.6 |########################################
  38712.5 |
  38882.4 |
  39052.3 |
  39222.2 |
  (0 below, 1 above range)

carrier_ent_c0_w64 (n=6, range 35356.2-38687.5 ns)
  35356.2 |########################################
  35522.8 |
  35689.3 |
  35855.9 |
  36022.5 |
  36189.0 |
  36355.6 |
  36522.2 |########################################
  36688.7 |
  36855.3 |########################################
  37021.8 |
  37188.4 |########################################
  37355.0 |
  37521.5 |
  37688.1 |
  37854.7 |
  38021.2 |
  38187.8 |########################################
  38354.4 |
  38520.9 |
  (0 below, 1 above range)

carrier_ent_c0_wmax (n=6, range 35431.7-55147.7 ns)
  35431.7 |########################################
  36417.5 |####################
  37403.3 |
  38389.1 |####################
  39374.9 |
  40360.7 |
  41346.5 |
  42332.3 |
  43318.1 |
  44303.9 |
  45289.7 |
  46275.5 |
  47261.3 |
  48247.1 |
  49232.9 |
  50218.7 |
  51204.5 |
  52190.3 |
  53176.1 |
  54161.9 |####################
  (0 below, 1 above range)

carrier_ent_c500_w4 (n=6, range 33964.6-35852.7 ns)
  33964.6 |########################################
  34059.0 |
  34153.4 |
  34247.8 |
  34342.2 |
  34436.6 |
  34531.0 |
  34625.4 |
  34719.8 |########################################
  34814.2 |
  34908.6 |
  35003.1 |########################################
  35097.5 |
  35191.9 |
  35286.3 |
  35380.7 |
  35475.1 |########################################
  35569.5 |########################################
  35663.9 |
  35758.3 |
  (0 below, 1 above range)

carrier_ent_c500_w64 (n=6, range 32711.7-35993.8 ns)
  32711.7 |########################################
  32875.8 |
  33039.9 |
  33204.0 |
  33368.1 |
  33532.2 |
  33696.3 |
  33860.4 |
  34024.5 |
  34188.6 |########################################
  34352.7 |########################################
  34516.8 |
  34680.9 |
  34845.0 |
  35009.1 |
  35173.2 |########################################
  35337.3 |
  35501.4 |########################################
  35665.5 |
  35829.6 |
  (0 below, 1 above range)

carrier_ent_c500_wmax (n=6, range 32655.4-34647.3 ns)
  32655.4 |########################################
  32755.0 |
  32854.6 |
  32954.2 |
  33053.8 |####################
  33153.4 |
  33253.0 |
  33352.6 |
  33452.2 |
  33551.8 |
  33651.4 |
  33750.9 |####################
  33850.5 |
  33950.1 |
  34049.7 |
  34149.3 |
  34248.9 |
  34348.5 |
  34448.1 |####################
  34547.7 |
  (0 below, 1 above range)

carrier_ent_c900_w4 (n=6, range 43545.8-46759.4 ns)
  43545.8 |########################################
  43706.5 |
  43867.2 |########################################
  44027.8 |
  44188.5 |
  44349.2 |
  44509.9 |
  44670.5 |
  44831.2 |
  44991.9 |
  45152.6 |
  45313.3 |
  45473.9 |########################################
  45634.6 |
  45795.3 |
  45956.0 |
  46116.6 |
  46277.3 |########################################
  46438.0 |
  46598.7 |########################################
  (0 below, 1 above range)

carrier_ent_c900_w64 (n=6, range 26242.9-27918.5 ns)
  26242.9 |####################
  26326.7 |########################################
  26410.5 |
  26494.2 |
  26578.0 |
  26661.8 |
  26745.6 |
  26829.4 |####################
  26913.2 |
  26996.9 |
  27080.7 |
  27164.5 |
  27248.3 |
  27332.1 |
  27415.9 |####################
  27499.6 |
  27583.4 |
  27667.2 |
  27751.0 |
  27834.8 |
  (0 below, 1 above range)

carrier_ent_c900_wmax (n=6, range 26075.0-28032.7 ns)
  26075.0 |########################################
  26172.9 |########################################
  26270.8 |
  26368.7 |
  26466.5 |
  26564.4 |
  26662.3 |
  26760.2 |
  26858.1 |
  26956.0 |
  27053.8 |
  27151.7 |
  27249.6 |
  27347.5 |
  27445.4 |########################################
  27543.3 |
  27641.2 |########################################
  27739.0 |########################################
  27836.9 |
  27934.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ent_c0_w4**: bridge=301.7% of algo (FFI overhead may distort results)
- **carrier_ent_c0_w64**: bridge=308.4% of algo (FFI overhead may distort results)
- **carrier_ent_c0_wmax**: CV=20.7% (high variance, measurements may be unstable)
- **carrier_ent_c0_wmax**: bridge=306.0% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w4**: bridge=307.6% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w64**: bridge=308.3% of algo (FFI overhead may distort results)
- **carrier_ent_c500_wmax**: bridge=309.0% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w4**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **carrier_ent_c900_w4**: bridge=206.1% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w64**: bridge=405.5% of algo (FFI overhead may distort results)
- **carrier_ent_c900_wmax**: bridge=388.9% of algo (FFI overhead may distort results)
