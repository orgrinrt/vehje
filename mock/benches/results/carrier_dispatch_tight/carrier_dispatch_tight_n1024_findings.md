# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 36% faster than the next best (carrier_disp_tight_ifchainasc)

carrier_disp_tight_nullfloor (32.93 us) leads carrier_disp_tight_ifchainasc (44.69 us) by 36%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 27% (significant)

carrier_disp_tight_nullfloor is -12.50 us (27%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_switch, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} (36% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_switch, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 32931.1 ns median (-27.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 32931.1 ns, slowest 60246.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 47983ns | 48058ns | 47285ns | 47837ns | 48551ns | +1.26% |
| carrier_disp_tight_fntable | 56489ns | 56339ns | 55632ns | 56233ns | 57301ns | +19.21% |
| carrier_disp_tight_ifchain | 47582ns | 48042ns | 45907ns | 47666ns | 48294ns | +0.42% |
| carrier_disp_tight_ifchainasc | 47284ns | 47212ns | 45229ns | 46926ns | 48848ns | -0.21% |
| carrier_disp_tight_ifchainlin | 62372ns | 62798ns | 59530ns | 62642ns | 63388ns | +31.63% |
| carrier_disp_tight_nullfloor | 35378ns | 35391ns | 34645ns | 35224ns | 35975ns | -25.34% |
| carrier_disp_tight_switch | 47384ns | 47979ns | 45105ns | 47419ns | 48472ns | base |
| carrier_disp_tight_threaded | 49416ns | 49244ns | 47487ns | 48865ns | 51208ns | +4.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 45443ns | 44795ns | 45952ns | +1.18% | 0.023 |
| carrier_disp_tight_fntable | 53994ns | 53313ns | 54795ns | +20.22% | 0.019 |
| carrier_disp_tight_ifchain | 45134ns | 43665ns | 45802ns | +0.49% | 0.023 |
| carrier_disp_tight_ifchainasc | 44874ns | 42989ns | 46418ns | -0.09% | 0.023 |
| carrier_disp_tight_ifchainlin | 59776ns | 56997ns | 60730ns | +33.09% | 0.017 |
| carrier_disp_tight_nullfloor | 32900ns | 32171ns | 33499ns | -26.75% | 0.031 |
| carrier_disp_tight_switch | 44912ns | 42549ns | 46031ns | base | 0.023 |
| carrier_disp_tight_threaded | 46951ns | 45193ns | 48493ns | +4.54% | 0.022 |

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.022 | 70.6% |
| carrier_disp_tight_fntable | 0.019 | 59.8% |
| carrier_disp_tight_ifchain | 0.023 | 70.8% |
| carrier_disp_tight_ifchainasc | 0.023 | 72.0% |
| carrier_disp_tight_ifchainlin | 0.017 | 53.4% |
| carrier_disp_tight_nullfloor | 0.031 | 97.7% |
| carrier_disp_tight_switch | 0.022 | 70.7% |
| carrier_disp_tight_threaded | 0.022 | 68.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 47983ns | 47983ns | +1.26% |
| carrier_disp_tight_fntable | 56489ns | 56489ns | +19.21% |
| carrier_disp_tight_ifchain | 47582ns | 47582ns | +0.42% |
| carrier_disp_tight_ifchainasc | 47284ns | 47284ns | -0.21% |
| carrier_disp_tight_ifchainlin | 62372ns | 62372ns | +31.63% |
| carrier_disp_tight_nullfloor | 35378ns | 35378ns | -25.34% |
| carrier_disp_tight_switch | 47384ns | 47384ns | base |
| carrier_disp_tight_threaded | 49416ns | 49416ns | +4.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 45514ns | base | --- | [43193, 46031] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 45539ns | no significant difference | [-802, +2605]ns | [44837, 45952] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_tight_fntable | 53806ns | +9156.2ns (+20.1%) | [+7555, +10533]ns | [53381, 54795] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 45458ns | no significant difference | [-1496, +2609]ns | [44142, 45802] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_tight_ifchainasc | 44694ns | no significant difference | [-2285, +2294]ns | [43509, 46418] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_tight_ifchainlin | 60246ns | +14552.5ns (+32.0%) | [+13479, +16558]ns | [58351, 60730] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 32931ns | -12497.9ns (-27.5%) | [-13018, -10522]ns | [32270, 33499] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 46902ns | no significant difference | [-182, +5101]ns | [45457, 48493] | no | 0.8021 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 46253ns | -0.6% | +15.9% | -1.9% | -7.1% | +29.8% | -26.8% | +4.0% |
| 2 | 45469ns | -1.3% | +22.1% | -4.0% | -0.7% | +31.3% | -27.7% | -0.6% |
| 3 | 45808ns | -2.2% | +18.1% | -2.6% | +0.2% | +32.5% | -27.6% | -0.2% |
| 4 | 45558ns | -0.4% | +17.0% | -0.1% | -2.9% | +32.7% | -29.4% | +1.2% |
| 5 | 43838ns | +4.2% | +23.2% | +3.9% | +7.1% | +30.0% | -24.8% | +8.8% |
| 6 | 42549ns | +7.9% | +25.6% | +8.3% | +3.5% | +42.9% | -23.9% | +14.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | 0.143 | ok |
| carrier_disp_tight_fntable | -0.162 | ok |
| carrier_disp_tight_ifchain | 0.190 | ok |
| carrier_disp_tight_ifchainasc | -0.388 | moderate- |
| carrier_disp_tight_ifchainlin | -0.410 | moderate- |
| carrier_disp_tight_nullfloor | -0.168 | ok |
| carrier_disp_tight_switch | 0.364 | moderate+ |
| carrier_disp_tight_threaded | 0.183 | ok |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 4/6, lost 2/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 3/6, lost 2/6
- **carrier_disp_tight_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 91090.2ns | 45442.5ns | 200.5% | HIGH |
| carrier_disp_tight_fntable | 108931.1ns | 53994.0ns | 201.7% | HIGH |
| carrier_disp_tight_ifchain | 91278.4ns | 45134.4ns | 202.2% | HIGH |
| carrier_disp_tight_ifchainasc | 92797.1ns | 44873.7ns | 206.8% | HIGH |
| carrier_disp_tight_ifchainlin | 119805.3ns | 59775.8ns | 200.4% | HIGH |
| carrier_disp_tight_nullfloor | 98853.2ns | 32900.1ns | 300.5% | HIGH |
| carrier_disp_tight_switch | 95604.1ns | 44912.5ns | 212.9% | HIGH |
| carrier_disp_tight_threaded | 93932.0ns | 46950.7ns | 200.1% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 44794.6-45951.5 ns)
  44794.6 |########################################
  44852.4 |########################################
  44910.3 |
  44968.1 |
  45026.0 |
  45083.8 |
  45141.7 |
  45199.5 |
  45257.4 |
  45315.2 |
  45373.1 |########################################
  45430.9 |
  45488.7 |
  45546.6 |
  45604.4 |
  45662.3 |########################################
  45720.1 |
  45778.0 |
  45835.8 |
  45893.7 |########################################
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 53312.9-54794.8 ns)
  53312.9 |########################################
  53387.0 |########################################
  53461.1 |
  53535.2 |########################################
  53609.3 |
  53683.4 |
  53757.5 |
  53831.6 |
  53905.7 |
  53979.8 |########################################
  54053.9 |########################################
  54127.9 |
  54202.0 |
  54276.1 |
  54350.2 |
  54424.3 |
  54498.4 |
  54572.5 |
  54646.6 |
  54720.7 |
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 43665.0-45802.3 ns)
  43665.0 |####################
  43771.9 |
  43878.7 |
  43985.6 |
  44092.5 |
  44199.3 |
  44306.2 |
  44413.1 |
  44519.9 |####################
  44626.8 |
  44733.7 |
  44840.5 |
  44947.4 |
  45054.2 |
  45161.1 |
  45268.0 |
  45374.8 |####################
  45481.7 |########################################
  45588.6 |
  45695.4 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 42989.2-46418.3 ns)
  42989.2 |########################################
  43160.7 |
  43332.1 |
  43503.6 |
  43675.0 |
  43846.5 |
  44017.9 |########################################
  44189.4 |########################################
  44360.8 |
  44532.3 |
  44703.8 |
  44875.2 |
  45046.7 |########################################
  45218.1 |
  45389.6 |
  45561.0 |
  45732.5 |########################################
  45903.9 |
  46075.4 |
  46246.8 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 56996.7-60730.4 ns)
  56996.7 |########################################
  57183.4 |
  57370.1 |
  57556.8 |
  57743.4 |
  57930.1 |
  58116.8 |
  58303.5 |
  58490.2 |
  58676.9 |
  58863.5 |
  59050.2 |
  59236.9 |
  59423.6 |
  59610.3 |########################################
  59797.0 |
  59983.7 |########################################
  60170.3 |
  60357.0 |########################################
  60543.7 |########################################
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 32170.8-33499.2 ns)
  32170.8 |########################################
  32237.2 |
  32303.6 |########################################
  32370.1 |
  32436.5 |
  32502.9 |
  32569.3 |
  32635.7 |
  32702.1 |
  32768.6 |
  32835.0 |########################################
  32901.4 |
  32967.8 |########################################
  33034.2 |
  33100.6 |########################################
  33167.1 |
  33233.5 |
  33299.9 |
  33366.3 |
  33432.7 |
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 42548.7-46030.6 ns)
  42548.7 |########################################
  42722.8 |
  42896.9 |
  43071.0 |
  43245.1 |
  43419.2 |
  43593.3 |
  43767.4 |########################################
  43941.5 |
  44115.6 |
  44289.7 |
  44463.7 |
  44637.8 |
  44811.9 |
  44986.0 |
  45160.1 |
  45334.2 |########################################
  45508.3 |########################################
  45682.4 |########################################
  45856.5 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 45192.9-48492.7 ns)
  45192.9 |########################################
  45357.9 |
  45522.9 |
  45687.9 |########################################
  45852.9 |
  46017.8 |########################################
  46182.8 |
  46347.8 |
  46512.8 |
  46677.8 |
  46842.8 |
  47007.8 |
  47172.8 |
  47337.8 |
  47502.8 |
  47667.8 |########################################
  47832.7 |
  47997.7 |########################################
  48162.7 |
  48327.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=200.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=202.0% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=200.6% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=204.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=300.7% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=201.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=199.9% of algo (FFI overhead may distort results)
