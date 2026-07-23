# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_threaded shows alternating (throttle bounce) (autocorr -0.67)

carrier_disp_madd_threaded's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 654385.7 ns median (-17.4% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.39x (fastest 654385.7 ns, slowest 912864.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 722238ns | 720544ns | 718954ns | 720068ns | 727135ns | -9.19% |
| carrier_disp_madd_fntable | 878545ns | 878255ns | 870320ns | 875654ns | 886994ns | +10.46% |
| carrier_disp_madd_ifchain | 790618ns | 790836ns | 786825ns | 789576ns | 794077ns | -0.60% |
| carrier_disp_madd_ifchainasc | 786910ns | 787544ns | 780929ns | 785472ns | 792058ns | -1.06% |
| carrier_disp_madd_ifchainlin | 701030ns | 700535ns | 694330ns | 699142ns | 707211ns | -11.86% |
| carrier_disp_madd_nullfloor | 658522ns | 657934ns | 654538ns | 657563ns | 661954ns | -17.21% |
| carrier_disp_madd_switch | 795372ns | 794930ns | 789420ns | 793246ns | 801536ns | base |
| carrier_disp_madd_threaded | 917273ns | 916243ns | 910080ns | 914822ns | 924547ns | +15.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 718896ns | 715365ns | 724193ns | -9.27% | 0.023 |
| carrier_disp_madd_fntable | 875399ns | 866465ns | 884435ns | +10.48% | 0.019 |
| carrier_disp_madd_ifchain | 787402ns | 783370ns | 791241ns | -0.62% | 0.021 |
| carrier_disp_madd_ifchainasc | 783550ns | 777345ns | 788966ns | -1.11% | 0.021 |
| carrier_disp_madd_ifchainlin | 697891ns | 691110ns | 704732ns | -11.92% | 0.023 |
| carrier_disp_madd_nullfloor | 655209ns | 651017ns | 658863ns | -17.31% | 0.025 |
| carrier_disp_madd_switch | 792351ns | 786075ns | 799028ns | base | 0.021 |
| carrier_disp_madd_threaded | 914109ns | 906284ns | 922063ns | +15.37% | 0.018 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 4518472 | 19922584 | 0.227 | 0.91× |
| carrier_disp_madd_fntable | 5505622 | 27018655 | 0.204 | 1.11× |
| carrier_disp_madd_ifchain | 4930345 | 19672878 | 0.251 | 0.99× |
| carrier_disp_madd_ifchainasc | 4910235 | 19672876 | 0.250 | 0.99× |
| carrier_disp_madd_ifchainlin | 4374738 | 20786489 | 0.210 | 0.88× |
| carrier_disp_madd_nullfloor | 4118065 | 17037531 | 0.242 | 0.83× |
| carrier_disp_madd_switch | 4958142 | 19149418 | 0.259 | 1.00× |
| carrier_disp_madd_threaded | 5713793 | 26487983 | 0.216 | 1.15× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.023 | 90.8% |
| carrier_disp_madd_fntable | 0.019 | 74.4% |
| carrier_disp_madd_ifchain | 0.021 | 82.7% |
| carrier_disp_madd_ifchainasc | 0.021 | 83.0% |
| carrier_disp_madd_ifchainlin | 0.024 | 93.4% |
| carrier_disp_madd_nullfloor | 0.025 | 99.5% |
| carrier_disp_madd_switch | 0.021 | 82.2% |
| carrier_disp_madd_threaded | 0.018 | 71.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 722238ns | 722238ns | -9.19% |
| carrier_disp_madd_fntable | 878545ns | 878545ns | +10.46% |
| carrier_disp_madd_ifchain | 790618ns | 790618ns | -0.60% |
| carrier_disp_madd_ifchainasc | 786910ns | 786910ns | -1.06% |
| carrier_disp_madd_ifchainlin | 701030ns | 701030ns | -11.86% |
| carrier_disp_madd_nullfloor | 658522ns | 658522ns | -17.21% |
| carrier_disp_madd_switch | 795372ns | 795372ns | base |
| carrier_disp_madd_threaded | 917273ns | 917273ns | +15.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 791771ns | base | --- | [786254, 799028] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 717021ns | -75224.8ns (-9.5%) | [-82376, -62765]ns | [715474, 724193] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 875164ns | +83844.6ns (+10.6%) | [+67727, +97571]ns | [866598, 884435] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 787545ns | no significant difference | [-12863, +1596]ns | [783419, 791241] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_madd_ifchainasc | 784145ns | -9187.7ns (-1.2%) | [-17122, -93]ns | [777539, 788966] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_disp_madd_ifchainlin | 696944ns | -97940.0ns (-12.4%) | [-103468, -81973]ns | [691996, 704732] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 654386ns | -137853.3ns (-17.4%) | [-145551, -128022]ns | [652378, 658863] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 912865ns | +122030.4ns (+15.4%) | [+112138, +131104]ns | [907398, 922063] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 787336ns | -8.9% | +10.1% | -0.1% | +0.2% | -10.5% | -16.7% | +17.4% |
| 2 | 799652ns | -10.2% | +8.4% | -1.3% | -2.0% | -13.0% | -18.2% | +13.3% |
| 3 | 798403ns | -10.4% | +8.6% | -1.9% | -1.2% | -12.6% | -18.0% | +15.2% |
| 4 | 786075ns | -8.8% | +12.4% | +0.5% | -1.1% | -10.3% | -16.8% | +15.6% |
| 5 | 796207ns | -10.1% | +11.1% | -0.5% | -2.3% | -13.0% | -18.2% | +14.8% |
| 6 | 786434ns | -7.1% | +12.5% | -0.4% | -0.3% | -12.1% | -15.8% | +16.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.105 | ok |
| carrier_disp_madd_fntable | 0.532 | HIGH+ (drift/warm-up) |
| carrier_disp_madd_ifchain | -0.364 | moderate- |
| carrier_disp_madd_ifchainasc | 0.018 | ok |
| carrier_disp_madd_ifchainlin | -0.079 | ok |
| carrier_disp_madd_nullfloor | -0.336 | moderate- |
| carrier_disp_madd_switch | -0.379 | moderate- |
| carrier_disp_madd_threaded | -0.669 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 5/6, lost 1/6
- **carrier_disp_madd_ifchainasc**: won 5/6, lost 1/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 720603.5ns | 718895.8ns | 100.2% | HIGH |
| carrier_disp_madd_fntable | 889673.3ns | 875398.6ns | 101.6% | HIGH |
| carrier_disp_madd_ifchain | 788345.1ns | 787401.9ns | 100.1% | HIGH |
| carrier_disp_madd_ifchainasc | 785703.8ns | 783550.0ns | 100.3% | HIGH |
| carrier_disp_madd_ifchainlin | 699288.8ns | 697890.8ns | 100.2% | HIGH |
| carrier_disp_madd_nullfloor | 657182.9ns | 655208.9ns | 100.3% | HIGH |
| carrier_disp_madd_switch | 793863.8ns | 792351.1ns | 100.2% | HIGH |
| carrier_disp_madd_threaded | 915972.8ns | 914108.5ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 715365.0-724193.2 ns)
  715365.0 |########################################
  715806.4 |
  716247.8 |####################
  716689.2 |
  717130.6 |####################
  717572.0 |####################
  718013.4 |
  718454.9 |
  718896.3 |
  719337.7 |
  719779.1 |
  720220.5 |
  720661.9 |
  721103.3 |
  721544.7 |
  721986.1 |
  722427.5 |
  722868.9 |
  723310.3 |
  723751.7 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 866465.0-884434.6 ns)
  866465.0 |########################################
  867363.5 |
  868262.0 |
  869160.4 |
  870058.9 |
  870957.4 |
  871855.9 |
  872754.4 |
  873652.8 |
  874551.3 |
  875449.8 |
  876348.3 |
  877246.8 |
  878145.2 |
  879043.7 |
  879942.2 |
  880840.7 |
  881739.2 |
  882637.6 |#############
  883536.1 |#############
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 783369.6-791241.4 ns)
  783369.6 |########################################
  783763.2 |
  784156.8 |
  784550.4 |
  784944.0 |
  785337.6 |
  785731.2 |
  786124.7 |####################
  786518.3 |
  786911.9 |
  787305.5 |
  787699.1 |
  788092.7 |
  788486.3 |####################
  788879.9 |
  789273.5 |
  789667.1 |
  790060.7 |####################
  790454.3 |
  790847.9 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 777345.0-788966.1 ns)
  777345.0 |########################################
  777926.1 |
  778507.1 |
  779088.2 |
  779669.2 |
  780250.3 |
  780831.3 |
  781412.4 |
  781993.4 |
  782574.5 |
  783155.5 |
  783736.6 |####################
  784317.6 |####################
  784898.7 |
  785479.7 |
  786060.8 |
  786641.8 |
  787222.9 |
  787803.9 |
  788385.0 |####################
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 691110.0-704731.8 ns)
  691110.0 |########################################
  691791.1 |
  692472.2 |########################################
  693153.3 |
  693834.4 |
  694515.5 |
  695196.6 |
  695877.6 |########################################
  696558.7 |
  697239.8 |########################################
  697920.9 |
  698602.0 |
  699283.1 |
  699964.2 |
  700645.3 |
  701326.4 |
  702007.5 |
  702688.6 |
  703369.7 |
  704050.8 |########################################
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 651017.1-658862.9 ns)
  651017.1 |########################################
  651409.4 |
  651801.7 |
  652194.0 |
  652586.3 |
  652978.6 |
  653370.9 |########################################
  653763.1 |########################################
  654155.4 |
  654547.7 |########################################
  654940.0 |
  655332.3 |
  655724.6 |########################################
  656116.9 |
  656509.2 |
  656901.5 |
  657293.8 |
  657686.1 |
  658078.4 |
  658470.7 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 786074.6-799027.5 ns)
  786074.6 |########################################
  786722.2 |####################
  787369.9 |
  788017.5 |
  788665.2 |
  789312.8 |
  789960.5 |
  790608.1 |
  791255.8 |
  791903.4 |
  792551.1 |
  793198.7 |
  793846.3 |
  794494.0 |
  795141.6 |
  795789.3 |####################
  796436.9 |
  797084.6 |
  797732.2 |
  798379.9 |####################
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 906283.8-922062.9 ns)
  906283.8 |########################################
  907072.8 |
  907861.7 |########################################
  908650.7 |
  909439.6 |
  910228.6 |
  911017.5 |
  911806.5 |########################################
  912595.4 |
  913384.4 |########################################
  914173.3 |
  914962.3 |
  915751.3 |
  916540.2 |
  917329.2 |
  918118.1 |
  918907.1 |
  919696.0 |########################################
  920485.0 |
  921273.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **carrier_disp_madd_fntable**: bridge=101.7% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=100.2% of algo (FFI overhead may distort results)
