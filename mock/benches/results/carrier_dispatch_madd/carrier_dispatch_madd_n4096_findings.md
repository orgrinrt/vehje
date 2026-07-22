# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 162050.5 ns median (-18.7% vs baseline)
- 5 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.32x (fastest 162050.5 ns, slowest 214322.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 181468ns | 179921ns | 178846ns | 179730ns | 185386ns | -10.25% |
| carrier_disp_madd_fntable | 216353ns | 216680ns | 214867ns | 216182ns | 217353ns | +7.00% |
| carrier_disp_madd_ifchain | 198455ns | 197314ns | 196066ns | 197084ns | 201705ns | -1.85% |
| carrier_disp_madd_ifchainasc | 196496ns | 196221ns | 193465ns | 195717ns | 199179ns | -2.82% |
| carrier_disp_madd_ifchainlin | 174703ns | 174833ns | 170481ns | 174087ns | 177739ns | -13.60% |
| carrier_disp_madd_nullfloor | 165030ns | 164535ns | 161250ns | 163788ns | 168782ns | -18.38% |
| carrier_disp_madd_switch | 202194ns | 202181ns | 200052ns | 201704ns | 204002ns | base |
| carrier_disp_madd_threaded | 216741ns | 216921ns | 214536ns | 216386ns | 218376ns | +7.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 178715ns | 175928ns | 182777ns | -10.35% | 0.023 |
| carrier_disp_madd_fntable | 213522ns | 212023ns | 214740ns | +7.11% | 0.019 |
| carrier_disp_madd_ifchain | 195613ns | 193519ns | 198742ns | -1.88% | 0.021 |
| carrier_disp_madd_ifchainasc | 193672ns | 190641ns | 196236ns | -2.85% | 0.021 |
| carrier_disp_madd_ifchainlin | 171992ns | 167715ns | 175171ns | -13.73% | 0.024 |
| carrier_disp_madd_nullfloor | 162460ns | 158625ns | 166169ns | -18.51% | 0.025 |
| carrier_disp_madd_switch | 199355ns | 197286ns | 201110ns | base | 0.021 |
| carrier_disp_madd_threaded | 214143ns | 211740ns | 215857ns | +7.42% | 0.019 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.023 | 89.6% |
| carrier_disp_madd_fntable | 0.019 | 74.2% |
| carrier_disp_madd_ifchain | 0.021 | 81.6% |
| carrier_disp_madd_ifchainasc | 0.021 | 82.0% |
| carrier_disp_madd_ifchainlin | 0.024 | 92.2% |
| carrier_disp_madd_nullfloor | 0.025 | 97.9% |
| carrier_disp_madd_switch | 0.021 | 79.6% |
| carrier_disp_madd_threaded | 0.019 | 74.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 181468ns | 181468ns | -10.25% |
| carrier_disp_madd_fntable | 216353ns | 216353ns | +7.00% |
| carrier_disp_madd_ifchain | 198455ns | 198455ns | -1.85% |
| carrier_disp_madd_ifchainasc | 196496ns | 196496ns | -2.82% |
| carrier_disp_madd_ifchainlin | 174703ns | 174703ns | -13.60% |
| carrier_disp_madd_nullfloor | 165030ns | 165030ns | -18.38% |
| carrier_disp_madd_switch | 202194ns | 202194ns | base |
| carrier_disp_madd_threaded | 216741ns | 216741ns | +7.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 199325ns | base | --- | [197630, 201110] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 177110ns | -20876.7ns (-10.5%) | [-23598, -17444]ns | [176260, 182777] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 213748ns | +14081.7ns (+7.1%) | [+13096, +15323]ns | [212078, 214740] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 194444ns | -3978.1ns (-2.0%) | [-5776, -1472]ns | [193652, 198742] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_madd_ifchainasc | 193441ns | -6265.1ns (-3.1%) | [-9171, -1614]ns | [191338, 196236] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_madd_ifchainlin | 172119ns | -27587.8ns (-13.8%) | [-31525, -22976]ns | [168686, 175171] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 162050ns | -36992.0ns (-18.6%) | [-39330, -34363]ns | [159161, 166169] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 214322ns | +13573.9ns (+6.8%) | [+12564, +18227]ns | [212250, 215857] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 199642ns | -9.3% | +7.1% | -2.3% | -3.8% | -13.5% | -17.0% | +6.6% |
| 2 | 197286ns | -10.0% | +7.5% | -1.8% | -0.0% | -10.9% | -18.6% | +9.0% |
| 3 | 197974ns | -11.1% | +8.0% | -2.3% | -1.6% | -14.3% | -19.3% | +9.5% |
| 4 | 201421ns | -12.3% | +6.5% | -1.1% | -5.4% | -14.9% | -17.3% | +6.2% |
| 5 | 200799ns | -8.2% | +7.0% | -3.4% | -2.8% | -16.5% | -18.5% | +7.0% |
| 6 | 199008ns | -11.2% | +6.5% | -0.3% | -3.5% | -12.2% | -20.3% | +6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.313 | moderate- |
| carrier_disp_madd_fntable | -0.128 | ok |
| carrier_disp_madd_ifchain | -0.428 | moderate- |
| carrier_disp_madd_ifchainasc | -0.402 | moderate- |
| carrier_disp_madd_ifchainlin | -0.297 | moderate- |
| carrier_disp_madd_nullfloor | -0.220 | moderate- |
| carrier_disp_madd_switch | 0.148 | ok |
| carrier_disp_madd_threaded | -0.109 | ok |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 6/6, lost 0/6
- **carrier_disp_madd_ifchainasc**: won 5/6, lost 0/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 179367.0ns | 178715.4ns | 100.4% | HIGH |
| carrier_disp_madd_fntable | 218147.6ns | 213521.9ns | 102.2% | HIGH |
| carrier_disp_madd_ifchain | 196369.8ns | 195613.0ns | 100.4% | HIGH |
| carrier_disp_madd_ifchainasc | 194160.4ns | 193671.7ns | 100.3% | HIGH |
| carrier_disp_madd_ifchainlin | 172664.2ns | 171992.0ns | 100.4% | HIGH |
| carrier_disp_madd_nullfloor | 163154.1ns | 162459.9ns | 100.4% | HIGH |
| carrier_disp_madd_switch | 200411.9ns | 199355.0ns | 100.5% | HIGH |
| carrier_disp_madd_threaded | 215147.9ns | 214143.3ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 175927.9-182776.8 ns)
  175927.9 |########################################
  176270.3 |########################################
  176612.8 |########################################
  176955.2 |
  177297.7 |########################################
  177640.1 |
  177982.6 |
  178325.0 |
  178667.5 |
  179009.9 |
  179352.4 |
  179694.8 |
  180037.3 |
  180379.7 |
  180722.2 |
  181064.6 |########################################
  181407.1 |
  181749.5 |
  182092.0 |
  182434.4 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 212023.3-214739.8 ns)
  212023.3 |########################################
  212159.1 |
  212294.9 |
  212430.8 |
  212566.6 |
  212702.4 |
  212838.2 |
  212974.1 |
  213109.9 |
  213245.7 |
  213381.5 |
  213517.4 |
  213653.2 |########################################
  213789.0 |
  213924.8 |
  214060.7 |
  214196.5 |
  214332.3 |
  214468.1 |####################
  214604.0 |
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 193519.2-198742.5 ns)
  193519.2 |####################
  193780.4 |########################################
  194041.5 |
  194302.7 |
  194563.9 |
  194825.0 |####################
  195086.2 |
  195347.4 |
  195608.5 |
  195869.7 |
  196130.9 |
  196392.0 |
  196653.2 |
  196914.3 |
  197175.5 |
  197436.7 |
  197697.8 |
  197959.0 |
  198220.2 |####################
  198481.3 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 190640.8-196236.0 ns)
  190640.8 |########################################
  190920.6 |
  191200.3 |
  191480.1 |
  191759.8 |########################################
  192039.6 |########################################
  192319.4 |
  192599.1 |
  192878.9 |
  193158.6 |
  193438.4 |
  193718.2 |
  193997.9 |
  194277.7 |
  194557.4 |########################################
  194837.2 |
  195117.0 |########################################
  195396.7 |
  195676.5 |
  195956.2 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 167715.4-175171.0 ns)
  167715.4 |########################################
  168088.2 |
  168461.0 |
  168833.7 |
  169206.5 |
  169579.3 |########################################
  169952.1 |
  170324.9 |
  170697.7 |
  171070.4 |
  171443.2 |########################################
  171816.0 |
  172188.8 |
  172561.6 |########################################
  172934.4 |
  173307.1 |
  173679.9 |
  174052.7 |
  174425.5 |########################################
  174798.3 |
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 158625.0-166168.5 ns)
  158625.0 |########################################
  159002.2 |
  159379.4 |########################################
  159756.5 |
  160133.7 |
  160510.9 |########################################
  160888.1 |
  161265.2 |
  161642.4 |
  162019.6 |
  162396.8 |
  162774.0 |
  163151.1 |
  163528.3 |########################################
  163905.5 |
  164282.7 |
  164659.8 |
  165037.0 |
  165414.2 |########################################
  165791.4 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 197286.2-201109.8 ns)
  197286.2 |########################################
  197477.4 |
  197668.6 |
  197859.7 |########################################
  198050.9 |
  198242.1 |
  198433.3 |
  198624.5 |
  198815.6 |
  199006.8 |########################################
  199198.0 |
  199389.2 |
  199580.4 |########################################
  199771.5 |
  199962.7 |
  200153.9 |
  200345.1 |
  200536.3 |
  200727.4 |########################################
  200918.6 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 211740.4-215857.1 ns)
  211740.4 |####################
  211946.2 |
  212152.1 |
  212357.9 |
  212563.7 |####################
  212769.6 |
  212975.4 |
  213181.2 |
  213387.1 |
  213592.9 |
  213798.8 |####################
  214004.6 |
  214210.4 |
  214416.3 |
  214622.1 |
  214827.9 |########################################
  215033.8 |
  215239.6 |
  215445.4 |
  215651.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=100.5% of algo (FFI overhead may distort results)
