# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 160063.4 ns median (-20.1% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.34x (fastest 160063.4 ns, slowest 215149.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 180734ns | 180452ns | 178162ns | 179834ns | 183368ns | -10.86% |
| carrier_disp_madd_fntable | 218306ns | 217562ns | 214805ns | 217006ns | 222005ns | +7.67% |
| carrier_disp_madd_ifchain | 200430ns | 199886ns | 198457ns | 199472ns | 202854ns | -1.15% |
| carrier_disp_madd_ifchainasc | 197762ns | 198279ns | 194623ns | 198131ns | 198778ns | -2.47% |
| carrier_disp_madd_ifchainlin | 174210ns | 173348ns | 171821ns | 172998ns | 177222ns | -14.08% |
| carrier_disp_madd_nullfloor | 163025ns | 162428ns | 160617ns | 161977ns | 165800ns | -19.60% |
| carrier_disp_madd_switch | 202764ns | 203075ns | 201096ns | 202511ns | 203977ns | base |
| carrier_disp_madd_threaded | 216949ns | 216950ns | 214818ns | 216420ns | 218810ns | +7.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 178317ns | 176016ns | 180943ns | -10.98% | 0.023 |
| carrier_disp_madd_fntable | 215817ns | 212495ns | 219473ns | +7.74% | 0.019 |
| carrier_disp_madd_ifchain | 197958ns | 195632ns | 200370ns | -1.17% | 0.021 |
| carrier_disp_madd_ifchainasc | 195327ns | 191995ns | 196557ns | -2.49% | 0.021 |
| carrier_disp_madd_ifchainlin | 171880ns | 169671ns | 174889ns | -14.19% | 0.024 |
| carrier_disp_madd_nullfloor | 160676ns | 158318ns | 163371ns | -19.79% | 0.025 |
| carrier_disp_madd_switch | 200311ns | 198772ns | 201535ns | base | 0.020 |
| carrier_disp_madd_threaded | 214567ns | 212670ns | 216629ns | +7.12% | 0.019 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 1120684 | 4996307 | 0.224 | 0.90× |
| carrier_disp_madd_fntable | 1364610 | 6638839 | 0.206 | 1.09× |
| carrier_disp_madd_ifchain | 1235210 | 4932994 | 0.250 | 0.99× |
| carrier_disp_madd_ifchainasc | 1224878 | 4933182 | 0.248 | 0.98× |
| carrier_disp_madd_ifchainlin | 1081776 | 5225285 | 0.207 | 0.87× |
| carrier_disp_madd_nullfloor | 1008273 | 4273179 | 0.236 | 0.81× |
| carrier_disp_madd_switch | 1248197 | 4801812 | 0.260 | 1.00× |
| carrier_disp_madd_threaded | 1349752 | 6636284 | 0.203 | 1.08× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.023 | 89.0% |
| carrier_disp_madd_fntable | 0.019 | 73.6% |
| carrier_disp_madd_ifchain | 0.021 | 80.2% |
| carrier_disp_madd_ifchainasc | 0.021 | 80.8% |
| carrier_disp_madd_ifchainlin | 0.024 | 92.6% |
| carrier_disp_madd_nullfloor | 0.026 | 98.9% |
| carrier_disp_madd_switch | 0.020 | 79.0% |
| carrier_disp_madd_threaded | 0.019 | 73.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 180734ns | 180734ns | -10.86% |
| carrier_disp_madd_fntable | 218306ns | 218306ns | +7.67% |
| carrier_disp_madd_ifchain | 200430ns | 200430ns | -1.15% |
| carrier_disp_madd_ifchainasc | 197762ns | 197762ns | -2.47% |
| carrier_disp_madd_ifchainlin | 174210ns | 174210ns | -14.08% |
| carrier_disp_madd_nullfloor | 163025ns | 163025ns | -19.60% |
| carrier_disp_madd_switch | 202764ns | 202764ns | base |
| carrier_disp_madd_threaded | 216949ns | 216949ns | +7.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 200429ns | base | --- | [198968, 201535] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 177957ns | -21679.7ns (-10.8%) | [-25341, -18960]ns | [176050, 180943] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 215149ns | +15317.1ns (+7.6%) | [+12329, +18874]ns | [212830, 219473] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 197455ns | no significant difference | [-4638, +1403]ns | [196050, 200370] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_madd_ifchainasc | 195878ns | -4048.9ns (-2.0%) | [-7181, -3721]ns | [193546, 196557] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_madd_ifchainlin | 170973ns | -29966.6ns (-15.0%) | [-30506, -24820]ns | [169778, 174889] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 160063ns | -39446.9ns (-19.7%) | [-41429, -38029]ns | [158592, 163371] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 214339ns | +13837.3ns (+6.9%) | [+11272, +17661]ns | [212734, 216629] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 200645ns | -9.8% | +6.2% | -2.1% | -4.3% | -12.3% | -19.2% | +6.3% |
| 2 | 198772ns | -10.0% | +9.3% | +0.2% | -1.8% | -12.6% | -19.3% | +9.0% |
| 3 | 199163ns | -9.2% | +8.7% | +1.3% | -1.9% | -14.8% | -20.2% | +8.8% |
| 4 | 202425ns | -13.0% | +9.5% | -2.2% | -2.8% | -15.1% | -18.6% | +5.1% |
| 5 | 200501ns | -11.8% | +6.6% | -1.7% | -2.0% | -15.2% | -20.4% | +6.1% |
| 6 | 200358ns | -12.1% | +6.1% | -2.4% | -2.0% | -15.2% | -21.0% | +7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | 0.148 | ok |
| carrier_disp_madd_fntable | -0.066 | ok |
| carrier_disp_madd_ifchain | 0.187 | ok |
| carrier_disp_madd_ifchainasc | 0.225 | moderate+ |
| carrier_disp_madd_ifchainlin | 0.220 | moderate+ |
| carrier_disp_madd_nullfloor | -0.316 | moderate- |
| carrier_disp_madd_switch | -0.092 | ok |
| carrier_disp_madd_threaded | -0.034 | ok |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 4/6, lost 2/6
- **carrier_disp_madd_ifchainasc**: won 6/6, lost 0/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 178826.9ns | 178316.7ns | 100.3% | HIGH |
| carrier_disp_madd_fntable | 220942.6ns | 215817.1ns | 102.4% | HIGH |
| carrier_disp_madd_ifchain | 198818.8ns | 197958.3ns | 100.4% | HIGH |
| carrier_disp_madd_ifchainasc | 195903.8ns | 195327.0ns | 100.3% | HIGH |
| carrier_disp_madd_ifchainlin | 172523.5ns | 171879.7ns | 100.4% | HIGH |
| carrier_disp_madd_nullfloor | 161298.7ns | 160675.6ns | 100.4% | HIGH |
| carrier_disp_madd_switch | 200887.1ns | 200310.6ns | 100.3% | HIGH |
| carrier_disp_madd_threaded | 215727.6ns | 214567.2ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 176015.8-180943.3 ns)
  176015.8 |########################################
  176262.2 |
  176508.5 |
  176754.9 |####################
  177001.3 |
  177247.7 |
  177494.0 |
  177740.4 |
  177986.8 |
  178233.2 |
  178479.5 |
  178725.9 |
  178972.3 |####################
  179218.7 |
  179465.0 |
  179711.4 |
  179957.8 |
  180204.2 |
  180450.5 |
  180696.9 |####################
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 212495.4-219472.7 ns)
  212495.4 |########################################
  212844.3 |########################################
  213193.1 |
  213542.0 |########################################
  213890.9 |
  214239.7 |
  214588.6 |
  214937.5 |
  215286.3 |
  215635.2 |
  215984.0 |
  216332.9 |########################################
  216681.8 |
  217030.6 |########################################
  217379.5 |
  217728.4 |
  218077.2 |
  218426.1 |
  218775.0 |
  219123.8 |
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 195632.1-200370.2 ns)
  195632.1 |########################################
  195869.0 |
  196105.9 |
  196342.8 |########################################
  196579.7 |
  196816.6 |########################################
  197053.5 |
  197290.4 |
  197527.3 |
  197764.2 |########################################
  198001.2 |
  198238.1 |
  198475.0 |
  198711.9 |
  198948.8 |########################################
  199185.7 |
  199422.6 |
  199659.5 |
  199896.4 |
  200133.3 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 191995.4-196556.9 ns)
  191995.4 |####################
  192223.5 |
  192451.5 |
  192679.6 |
  192907.7 |
  193135.8 |
  193363.9 |
  193591.9 |
  193820.0 |
  194048.1 |
  194276.2 |
  194504.2 |
  194732.3 |
  194960.4 |####################
  195188.5 |####################
  195416.5 |
  195644.6 |
  195872.7 |
  196100.8 |
  196328.8 |########################################
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 169671.2-174888.5 ns)
  169671.2 |########################################
  169932.1 |####################
  170192.9 |
  170453.8 |
  170714.7 |
  170975.5 |
  171236.4 |
  171497.3 |
  171758.1 |####################
  172019.0 |
  172279.9 |
  172540.7 |
  172801.6 |
  173062.5 |
  173323.3 |
  173584.2 |####################
  173845.1 |
  174105.9 |
  174366.8 |
  174627.7 |
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 158317.9-163371.5 ns)
  158317.9 |########################################
  158570.6 |
  158823.3 |########################################
  159075.9 |
  159328.6 |
  159581.3 |########################################
  159834.0 |
  160086.6 |
  160339.3 |########################################
  160592.0 |
  160844.7 |
  161097.4 |
  161350.0 |
  161602.7 |
  161855.4 |########################################
  162108.1 |
  162360.7 |
  162613.4 |
  162866.1 |
  163118.8 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 198772.5-201534.6 ns)
  198772.5 |########################################
  198910.6 |
  199048.7 |########################################
  199186.8 |
  199324.9 |
  199463.0 |
  199601.1 |
  199739.2 |
  199877.3 |
  200015.4 |
  200153.5 |
  200291.7 |########################################
  200429.8 |########################################
  200567.9 |########################################
  200706.0 |
  200844.1 |
  200982.2 |
  201120.3 |
  201258.4 |
  201396.5 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 212670.4-216628.8 ns)
  212670.4 |########################################
  212868.3 |
  213066.2 |####################
  213264.2 |
  213462.1 |
  213660.0 |
  213857.9 |
  214055.8 |
  214253.7 |
  214451.7 |
  214649.6 |
  214847.5 |
  215045.4 |
  215243.3 |####################
  215441.2 |
  215639.2 |
  215837.1 |
  216035.0 |
  216232.9 |
  216430.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=102.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=100.6% of algo (FFI overhead may distort results)
