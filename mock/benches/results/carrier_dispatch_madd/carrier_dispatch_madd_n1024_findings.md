# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 42295.2 ns median (-16.0% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.28x (fastest 42295.2 ns, slowest 54343.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 48582ns | 48566ns | 47908ns | 48529ns | 49000ns | -7.04% |
| carrier_disp_madd_fntable | 56945ns | 56850ns | 56110ns | 56753ns | 57652ns | +8.96% |
| carrier_disp_madd_ifchain | 51293ns | 51039ns | 50298ns | 50847ns | 52460ns | -1.85% |
| carrier_disp_madd_ifchainasc | 51233ns | 51240ns | 48694ns | 51072ns | 52745ns | -1.97% |
| carrier_disp_madd_ifchainlin | 47518ns | 47371ns | 46842ns | 47230ns | 48289ns | -9.08% |
| carrier_disp_madd_nullfloor | 44679ns | 44868ns | 43725ns | 44641ns | 45213ns | -14.51% |
| carrier_disp_madd_switch | 52262ns | 52867ns | 49753ns | 52494ns | 53167ns | base |
| carrier_disp_madd_threaded | 57179ns | 56956ns | 55320ns | 56768ns | 58725ns | +9.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 46116ns | 45595ns | 46456ns | -7.25% | 0.022 |
| carrier_disp_madd_fntable | 54434ns | 53615ns | 55184ns | +9.49% | 0.019 |
| carrier_disp_madd_ifchain | 48793ns | 47829ns | 49917ns | -1.86% | 0.021 |
| carrier_disp_madd_ifchainasc | 48688ns | 46290ns | 50011ns | -2.07% | 0.021 |
| carrier_disp_madd_ifchainlin | 44972ns | 44189ns | 45749ns | -9.55% | 0.023 |
| carrier_disp_madd_nullfloor | 42110ns | 41131ns | 42680ns | -15.30% | 0.024 |
| carrier_disp_madd_switch | 49718ns | 47245ns | 50576ns | base | 0.021 |
| carrier_disp_madd_threaded | 54485ns | 52888ns | 55638ns | +9.59% | 0.019 |

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.022 | 89.2% |
| carrier_disp_madd_fntable | 0.019 | 75.7% |
| carrier_disp_madd_ifchain | 0.021 | 84.8% |
| carrier_disp_madd_ifchainasc | 0.021 | 84.3% |
| carrier_disp_madd_ifchainlin | 0.023 | 91.6% |
| carrier_disp_madd_nullfloor | 0.024 | 97.2% |
| carrier_disp_madd_switch | 0.020 | 81.7% |
| carrier_disp_madd_threaded | 0.019 | 75.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 48582ns | 48582ns | -7.04% |
| carrier_disp_madd_fntable | 56945ns | 56945ns | +8.96% |
| carrier_disp_madd_ifchain | 51293ns | 51293ns | -1.85% |
| carrier_disp_madd_ifchainasc | 51233ns | 51233ns | -1.97% |
| carrier_disp_madd_ifchainlin | 47518ns | 47518ns | -9.08% |
| carrier_disp_madd_nullfloor | 44679ns | 44679ns | -14.51% |
| carrier_disp_madd_switch | 52262ns | 52262ns | base |
| carrier_disp_madd_threaded | 57179ns | 57179ns | +9.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 50335ns | base | --- | [48244, 50576] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 46089ns | -4245.6ns (-8.4%) | [-4367, -2196]ns | [45802, 46456] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 54344ns | +4248.1ns (+8.4%) | [+3199, +6701]ns | [53775, 55184] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 48531ns | no significant difference | [-2417, +1169]ns | [47931, 49917] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_madd_ifchainasc | 48791ns | no significant difference | [-3056, +677]ns | [47262, 50011] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_madd_ifchainlin | 44908ns | -4813.8ns (-9.6%) | [-5995, -3431]ns | [44258, 45749] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 42295ns | -7981.3ns (-15.9%) | [-8097, -6747]ns | [41354, 42680] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 54343ns | +4061.5ns (+8.1%) | [+3133, +7106]ns | [53474, 55638] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 50780ns | -8.6% | +6.2% | -2.6% | -8.8% | -9.9% | -15.6% | +7.5% |
| 2 | 50346ns | -8.5% | +9.0% | -3.4% | -3.0% | -9.1% | -15.5% | +8.5% |
| 3 | 47245ns | -3.5% | +17.5% | +2.5% | +2.1% | -6.2% | -12.0% | +14.4% |
| 4 | 49243ns | -5.6% | +10.4% | +2.3% | +0.7% | -8.0% | -16.5% | +15.0% |
| 5 | 50324ns | -8.3% | +7.9% | -5.0% | +0.2% | -11.5% | -15.9% | +5.1% |
| 6 | 50372ns | -8.7% | +6.4% | -4.6% | -3.2% | -12.3% | -16.0% | +7.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.354 | moderate- |
| carrier_disp_madd_fntable | 0.131 | ok |
| carrier_disp_madd_ifchain | -0.311 | moderate- |
| carrier_disp_madd_ifchainasc | 0.089 | ok |
| carrier_disp_madd_ifchainlin | 0.036 | ok |
| carrier_disp_madd_nullfloor | 0.224 | moderate+ |
| carrier_disp_madd_switch | 0.046 | ok |
| carrier_disp_madd_threaded | -0.494 | moderate- |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 4/6, lost 2/6
- **carrier_disp_madd_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 92427.5ns | 46115.6ns | 200.4% | HIGH |
| carrier_disp_madd_fntable | 109902.8ns | 54434.2ns | 201.9% | HIGH |
| carrier_disp_madd_ifchain | 97711.3ns | 48793.1ns | 200.3% | HIGH |
| carrier_disp_madd_ifchainasc | 97564.7ns | 48688.1ns | 200.4% | HIGH |
| carrier_disp_madd_ifchainlin | 90314.8ns | 44971.6ns | 200.8% | HIGH |
| carrier_disp_madd_nullfloor | 116412.1ns | 42109.8ns | 276.4% | HIGH |
| carrier_disp_madd_switch | 99616.5ns | 49718.3ns | 200.4% | HIGH |
| carrier_disp_madd_threaded | 108789.3ns | 54484.9ns | 199.7% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 45595.0-46455.6 ns)
  45595.0 |########################################
  45638.0 |
  45681.1 |
  45724.1 |
  45767.1 |
  45810.2 |
  45853.2 |
  45896.2 |
  45939.2 |
  45982.3 |########################################
  46025.3 |########################################
  46068.3 |
  46111.4 |########################################
  46154.4 |
  46197.4 |
  46240.4 |
  46283.5 |
  46326.5 |
  46369.5 |########################################
  46412.6 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 53615.0-55184.2 ns)
  53615.0 |########################################
  53693.5 |
  53771.9 |
  53850.4 |
  53928.8 |########################################
  54007.3 |
  54085.8 |
  54164.2 |
  54242.7 |########################################
  54321.1 |########################################
  54399.6 |
  54478.1 |
  54556.5 |
  54635.0 |
  54713.4 |
  54791.9 |########################################
  54870.4 |
  54948.8 |
  55027.3 |
  55105.7 |
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 47828.8-49917.1 ns)
  47828.8 |########################################
  47933.2 |########################################
  48037.6 |
  48142.0 |
  48246.5 |
  48350.9 |########################################
  48455.3 |
  48559.7 |########################################
  48664.1 |
  48768.5 |
  48872.9 |
  48977.4 |
  49081.8 |
  49186.2 |
  49290.6 |
  49395.0 |########################################
  49499.4 |
  49603.9 |
  49708.3 |
  49812.7 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 46290.0-50011.1 ns)
  46290.0 |####################
  46476.1 |
  46662.1 |
  46848.2 |
  47034.2 |
  47220.3 |
  47406.3 |
  47592.4 |
  47778.4 |
  47964.5 |
  48150.5 |####################
  48336.6 |
  48522.6 |
  48708.7 |########################################
  48894.7 |
  49080.8 |
  49266.8 |
  49452.9 |####################
  49638.9 |
  49825.0 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 44189.2-45748.9 ns)
  44189.2 |########################################
  44267.2 |########################################
  44345.2 |
  44423.2 |
  44501.1 |########################################
  44579.1 |
  44657.1 |
  44735.1 |
  44813.1 |
  44891.1 |
  44969.1 |
  45047.1 |
  45125.0 |
  45203.0 |
  45281.0 |########################################
  45359.0 |
  45437.0 |
  45515.0 |
  45593.0 |
  45671.0 |########################################
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 41130.8-42679.8 ns)
  41130.8 |########################################
  41208.2 |
  41285.7 |
  41363.1 |
  41440.6 |
  41518.0 |########################################
  41595.5 |
  41672.9 |
  41750.4 |
  41827.8 |
  41905.3 |
  41982.7 |
  42060.2 |
  42137.6 |
  42215.1 |########################################
  42292.5 |########################################
  42370.0 |
  42447.4 |########################################
  42524.9 |
  42602.3 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 47244.6-50576.1 ns)
  47244.6 |#############
  47411.2 |
  47577.7 |
  47744.3 |
  47910.9 |
  48077.5 |
  48244.0 |
  48410.6 |
  48577.2 |
  48743.8 |
  48910.3 |
  49076.9 |#############
  49243.5 |
  49410.0 |
  49576.6 |
  49743.2 |
  49909.8 |
  50076.3 |
  50242.9 |########################################
  50409.5 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 52888.3-55637.9 ns)
  52888.3 |####################
  53025.8 |
  53163.3 |
  53300.7 |
  53438.2 |
  53575.7 |
  53713.2 |
  53850.7 |
  53988.1 |########################################
  54125.6 |
  54263.1 |
  54400.6 |
  54538.1 |########################################
  54675.5 |
  54813.0 |
  54950.5 |
  55088.0 |
  55225.5 |
  55362.9 |
  55500.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=202.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=201.0% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=274.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=200.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=199.9% of algo (FFI overhead may distort results)
