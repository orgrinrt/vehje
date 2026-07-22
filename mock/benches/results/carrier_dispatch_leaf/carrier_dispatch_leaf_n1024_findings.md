# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 44% faster than the next best (carrier_disp_leaf_bittree)

carrier_disp_leaf_nullfloor (28.70 us) leads carrier_disp_leaf_bittree (41.19 us) by 44%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 39% (significant)

carrier_disp_leaf_nullfloor is -18.00 us (39%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_threaded shows alternating (throttle bounce) (autocorr -0.70)

carrier_disp_leaf_threaded's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_fntable} (44% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_bittree, carrier_disp_leaf_threaded, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_fntable} with a 44% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 28695.0 ns median (-38.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.95x (fastest 28695.0 ns, slowest 55998.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 43790ns | 43643ns | 42680ns | 43422ns | 44896ns | -11.62% |
| carrier_disp_leaf_fntable | 58168ns | 58664ns | 53831ns | 58507ns | 59828ns | +17.40% |
| carrier_disp_leaf_ifchain | 52260ns | 51995ns | 51305ns | 51897ns | 53282ns | +5.48% |
| carrier_disp_leaf_ifchainasc | 50605ns | 50292ns | 47895ns | 49755ns | 53233ns | +2.14% |
| carrier_disp_leaf_ifchainlin | 53464ns | 53468ns | 52980ns | 53363ns | 53858ns | +7.91% |
| carrier_disp_leaf_nullfloor | 31148ns | 31184ns | 30207ns | 31057ns | 31756ns | -37.13% |
| carrier_disp_leaf_switch | 49547ns | 48895ns | 43925ns | 48381ns | 54106ns | base |
| carrier_disp_leaf_threaded | 47333ns | 47230ns | 46573ns | 47156ns | 47978ns | -4.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 41303ns | 40219ns | 42325ns | -12.25% | 0.025 |
| carrier_disp_leaf_fntable | 55622ns | 51385ns | 57263ns | +18.17% | 0.018 |
| carrier_disp_leaf_ifchain | 49744ns | 48948ns | 50698ns | +5.68% | 0.021 |
| carrier_disp_leaf_ifchainasc | 48212ns | 45527ns | 50671ns | +2.43% | 0.021 |
| carrier_disp_leaf_ifchainlin | 50990ns | 50465ns | 51262ns | +8.33% | 0.020 |
| carrier_disp_leaf_nullfloor | 28692ns | 27848ns | 29298ns | -39.04% | 0.036 |
| carrier_disp_leaf_switch | 47070ns | 41530ns | 51610ns | base | 0.022 |
| carrier_disp_leaf_threaded | 44903ns | 44068ns | 45441ns | -4.60% | 0.023 |

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.025 | 67.6% |
| carrier_disp_leaf_fntable | 0.018 | 49.7% |
| carrier_disp_leaf_ifchain | 0.021 | 56.3% |
| carrier_disp_leaf_ifchainasc | 0.021 | 58.0% |
| carrier_disp_leaf_ifchainlin | 0.020 | 54.5% |
| carrier_disp_leaf_nullfloor | 0.036 | 97.0% |
| carrier_disp_leaf_switch | 0.022 | 60.0% |
| carrier_disp_leaf_threaded | 0.023 | 62.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 43790ns | 43790ns | -11.62% |
| carrier_disp_leaf_fntable | 58168ns | 58168ns | +17.40% |
| carrier_disp_leaf_ifchain | 52260ns | 52260ns | +5.48% |
| carrier_disp_leaf_ifchainasc | 50605ns | 50605ns | +2.14% |
| carrier_disp_leaf_ifchainlin | 53464ns | 53464ns | +7.91% |
| carrier_disp_leaf_nullfloor | 31148ns | 31148ns | -37.13% |
| carrier_disp_leaf_switch | 49547ns | 49547ns | base |
| carrier_disp_leaf_threaded | 47333ns | 47333ns | -4.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 46390ns | base | --- | [43210, 51610] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 41188ns | -5582.7ns (-12.0%) | [-10417, -1300]ns | [40397, 42325] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 55999ns | +8755.7ns (+18.9%) | [+4167, +12734]ns | [53605, 57263] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 49484ns | no significant difference | [-1928, +6258]ns | [49050, 50698] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainasc | 48034ns | no significant difference | [-1073, +4824]ns | [45931, 50671] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainlin | 51061ns | no significant difference | [-690, +7779]ns | [50646, 51262] | no | 0.3828 | 0.2188 | 0 |
| carrier_disp_leaf_nullfloor | 28695ns | -18003.1ns (-38.8%) | [-22695, -14435]ns | [28084, 29298] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 44874ns | no significant difference | [-7098, +1948]ns | [44394, 45441] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 41530ns | -0.5% | +34.4% | +17.9% | +14.8% | +22.4% | -31.5% | +7.7% |
| 2 | 53319ns | -24.6% | +6.8% | -5.8% | -3.0% | -5.4% | -46.9% | -15.7% |
| 3 | 46693ns | -13.1% | +10.0% | +5.6% | -0.8% | +9.4% | -38.1% | -4.1% |
| 4 | 46086ns | -10.9% | +21.4% | +11.1% | -1.2% | +10.7% | -39.6% | -1.7% |
| 5 | 49901ns | -15.5% | +15.4% | -1.5% | -0.6% | +3.0% | -40.9% | -11.7% |
| 6 | 44890ns | -5.4% | +24.9% | +10.6% | +7.8% | +13.9% | -35.2% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | 0.444 | moderate+ |
| carrier_disp_leaf_fntable | -0.221 | moderate- |
| carrier_disp_leaf_ifchain | -0.580 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchainasc | -0.273 | moderate- |
| carrier_disp_leaf_ifchainlin | 0.215 | moderate+ |
| carrier_disp_leaf_nullfloor | -0.319 | moderate- |
| carrier_disp_leaf_switch | -0.545 | HIGH- (thermal bounce) |
| carrier_disp_leaf_threaded | -0.697 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 6/6, lost 0/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 2/6, lost 4/6
- **carrier_disp_leaf_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_leaf_ifchainlin**: won 1/6, lost 5/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 109498.2ns | 41303.2ns | 265.1% | HIGH |
| carrier_disp_leaf_fntable | 113789.6ns | 55622.1ns | 204.6% | HIGH |
| carrier_disp_leaf_ifchain | 99553.7ns | 49744.2ns | 200.1% | HIGH |
| carrier_disp_leaf_ifchainasc | 96744.5ns | 48212.0ns | 200.7% | HIGH |
| carrier_disp_leaf_ifchainlin | 102107.8ns | 50990.1ns | 200.3% | HIGH |
| carrier_disp_leaf_nullfloor | 103634.5ns | 28692.2ns | 361.2% | HIGH |
| carrier_disp_leaf_switch | 98241.6ns | 47069.8ns | 208.7% | HIGH |
| carrier_disp_leaf_threaded | 90819.2ns | 44903.1ns | 202.3% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 40219.2-42325.0 ns)
  40219.2 |########################################
  40324.5 |
  40429.8 |
  40535.1 |########################################
  40640.4 |
  40745.6 |
  40850.9 |
  40956.2 |########################################
  41061.5 |
  41166.8 |
  41272.1 |########################################
  41377.4 |
  41482.7 |
  41588.0 |
  41693.3 |
  41798.6 |
  41903.8 |
  42009.1 |
  42114.4 |########################################
  42219.7 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 51385.4-57262.7 ns)
  51385.4 |#############
  51679.3 |
  51973.1 |
  52267.0 |
  52560.9 |
  52854.7 |
  53148.6 |
  53442.5 |
  53736.3 |
  54030.2 |
  54324.1 |
  54617.9 |
  54911.8 |
  55205.6 |
  55499.5 |
  55793.4 |########################################
  56087.2 |
  56381.1 |
  56675.0 |#############
  56968.8 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 48947.9-50697.9 ns)
  48947.9 |########################################
  49035.4 |
  49122.9 |########################################
  49210.4 |
  49297.9 |########################################
  49385.4 |
  49472.9 |
  49560.4 |
  49647.9 |########################################
  49735.4 |
  49822.9 |
  49910.4 |
  49997.9 |
  50085.4 |
  50172.9 |########################################
  50260.4 |
  50347.9 |
  50435.4 |
  50522.9 |
  50610.4 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 45526.7-50670.6 ns)
  45526.7 |########################################
  45783.9 |
  46041.1 |
  46298.3 |########################################
  46555.5 |
  46812.7 |
  47069.9 |
  47327.1 |
  47584.3 |########################################
  47841.5 |
  48098.7 |
  48355.8 |########################################
  48613.0 |
  48870.2 |
  49127.4 |
  49384.6 |########################################
  49641.8 |
  49899.0 |
  50156.2 |
  50413.4 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 50464.6-51262.5 ns)
  50464.6 |########################################
  50504.5 |
  50544.4 |
  50584.3 |
  50624.2 |
  50664.1 |
  50704.0 |
  50743.9 |
  50783.8 |
  50823.7 |########################################
  50863.6 |
  50903.4 |
  50943.3 |
  50983.2 |########################################
  51023.1 |
  51063.0 |########################################
  51102.9 |
  51142.8 |########################################
  51182.7 |
  51222.6 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 27847.5-29297.5 ns)
  27847.5 |########################################
  27920.0 |
  27992.5 |
  28065.0 |
  28137.5 |
  28210.0 |
  28282.5 |########################################
  28355.0 |
  28427.5 |########################################
  28500.0 |
  28572.5 |
  28645.0 |
  28717.5 |
  28790.0 |
  28862.5 |########################################
  28935.0 |
  29007.5 |
  29080.0 |########################################
  29152.5 |
  29225.0 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 41529.6-51610.0 ns)
  41529.6 |########################################
  42033.6 |
  42537.6 |
  43041.7 |
  43545.7 |
  44049.7 |
  44553.7 |########################################
  45057.7 |
  45561.8 |
  46065.8 |########################################
  46569.8 |########################################
  47073.8 |
  47577.8 |
  48081.9 |
  48585.9 |
  49089.9 |
  49593.9 |########################################
  50097.9 |
  50602.0 |
  51106.0 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 44068.3-45440.6 ns)
  44068.3 |########################################
  44136.9 |
  44205.5 |
  44274.2 |
  44342.8 |
  44411.4 |
  44480.0 |
  44548.6 |
  44617.2 |
  44685.9 |########################################
  44754.5 |########################################
  44823.1 |
  44891.7 |########################################
  44960.3 |
  45028.9 |
  45097.6 |
  45166.2 |
  45234.8 |########################################
  45303.4 |
  45372.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=268.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=205.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=358.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=208.6% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=201.0% of algo (FFI overhead may distort results)
