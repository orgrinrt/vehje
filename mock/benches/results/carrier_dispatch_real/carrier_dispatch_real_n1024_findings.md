# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_real**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_real**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_nullfloor_real dominates: 41% faster than the next best (carrier_disp_switch_real)

carrier_disp_nullfloor_real (29.59 us) leads carrier_disp_switch_real (41.64 us) by 41%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_nullfloor_real beats baseline by 30% (significant)

carrier_disp_nullfloor_real is -12.61 us (30%) faster than baseline carrier_disp_switch_real, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_ifchainlin_real is an outlier: 2.9x slower than the field

carrier_disp_ifchainlin_real (85.45 us) is 2.9x the fastest (29.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_nullfloor_real, carrier_disp_switch_real, carrier_disp_ifchainasc_real, carrier_disp_ifchain_real, carrier_disp_threaded_real, carrier_disp_bittree_real, carrier_disp_fntable_real} vs {carrier_disp_ifchainlin_real} (54% apart)

The field splits into a fast tier {carrier_disp_nullfloor_real, carrier_disp_switch_real, carrier_disp_ifchainasc_real, carrier_disp_ifchain_real, carrier_disp_threaded_real, carrier_disp_bittree_real, carrier_disp_fntable_real} and a slow tier {carrier_disp_ifchainlin_real} with a 54% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_disp_ifchainasc_real is inconsistent: worst-20% is 1.6x its best-20%

carrier_disp_ifchainasc_real's best 20% of batches run at 40.28 us but its worst 20% at 65.53 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### carrier_disp_ifchainasc_real's edge over baseline is significant but tiny (46 ns, 0.11%)

carrier_disp_ifchainasc_real differs from baseline carrier_disp_switch_real by 46 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_nullfloor_real** at 29592.1 ns median (-28.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.89x (fastest 29592.1 ns, slowest 85447.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_bittree_real | 55012ns | 55866ns | 51129ns | 54971ns | 57016ns | +22.26% |
| carrier_disp_fntable_real | 58189ns | 57846ns | 53865ns | 56536ns | 62831ns | +29.32% |
| carrier_disp_ifchain_real | 50957ns | 45496ns | 42771ns | 44646ns | 64515ns | +13.25% |
| carrier_disp_ifchainasc_real | 51895ns | 44861ns | 42449ns | 44324ns | 67973ns | +15.34% |
| carrier_disp_ifchainlin_real | 90309ns | 87679ns | 86865ns | 87436ns | 96341ns | +100.71% |
| carrier_disp_nullfloor_real | 32486ns | 31951ns | 30727ns | 31780ns | 34425ns | -27.80% |
| carrier_disp_switch_real | 44995ns | 43914ns | 42885ns | 43599ns | 48143ns | base |
| carrier_disp_threaded_real | 50332ns | 48850ns | 47814ns | 48508ns | 54329ns | +11.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_bittree_real | 52598ns | 48912ns | 54520ns | +23.25% | 0.019 |
| carrier_disp_fntable_real | 55845ns | 51662ns | 60356ns | +30.86% | 0.018 |
| carrier_disp_ifchain_real | 48620ns | 40591ns | 61995ns | +13.93% | 0.021 |
| carrier_disp_ifchainasc_real | 49562ns | 40284ns | 65529ns | +16.13% | 0.021 |
| carrier_disp_ifchainlin_real | 87968ns | 84621ns | 93801ns | +106.13% | 0.012 |
| carrier_disp_nullfloor_real | 30134ns | 28532ns | 31972ns | -29.39% | 0.034 |
| carrier_disp_switch_real | 42676ns | 40673ns | 45653ns | base | 0.024 |
| carrier_disp_threaded_real | 48011ns | 45590ns | 51841ns | +12.50% | 0.021 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_nullfloor_real; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_bittree_real | 0.019 | 53.4% |
| carrier_disp_fntable_real | 0.018 | 51.4% |
| carrier_disp_ifchain_real | 0.024 | 66.1% |
| carrier_disp_ifchainasc_real | 0.024 | 67.1% |
| carrier_disp_ifchainlin_real | 0.012 | 33.4% |
| carrier_disp_nullfloor_real | 0.035 | 96.4% |
| carrier_disp_switch_real | 0.025 | 68.5% |
| carrier_disp_threaded_real | 0.022 | 61.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_bittree_real | 55012ns | 55012ns | +22.26% |
| carrier_disp_fntable_real | 58189ns | 58189ns | +29.32% |
| carrier_disp_ifchain_real | 50957ns | 50957ns | +13.25% |
| carrier_disp_ifchainasc_real | 51895ns | 51895ns | +15.34% |
| carrier_disp_ifchainlin_real | 90309ns | 90309ns | +100.71% |
| carrier_disp_nullfloor_real | 32486ns | 32486ns | -27.80% |
| carrier_disp_switch_real | 44995ns | 44995ns | base |
| carrier_disp_threaded_real | 50332ns | 50332ns | +11.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_real | 41645ns | base | --- | [40731, 45653] | --- | --- | --- | --- |
| carrier_disp_bittree_real | 53380ns | +9250.2ns (+22.2%) | [+6927, +13590]ns | [49895, 54520] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_fntable_real | 55496ns | +11986.8ns (+28.8%) | [+10238, +17283]ns | [51684, 60356] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_ifchain_real | 43195ns | no significant difference | [-777, +18522]ns | [40668, 61995] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_ifchainasc_real | 42521ns | no significant difference | [-3161, +23771]ns | [40634, 65529] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_ifchainlin_real | 85447ns | +44259.6ns (+106.3%) | [+41289, +50328]ns | [84656, 93801] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_nullfloor_real | 29592ns | -12605.8ns (-30.3%) | [-14934, -10087]ns | [28838, 31972] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_threaded_real | 46584ns | +4891.7ns (+11.7%) | [+2746, +8367]ns | [45609, 51841] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_real | carrier_disp_bittree_real | carrier_disp_fntable_real | carrier_disp_ifchain_real | carrier_disp_ifchainasc_real | carrier_disp_ifchainlin_real | carrier_disp_nullfloor_real | carrier_disp_threaded_real |
|---|---|---|---|---|---|---|---|---|
| 1 | 45546ns | +15.5% | +26.0% | -0.1% | -11.6% | +87.8% | -35.8% | +4.3% |
| 2 | 42102ns | +16.2% | +22.7% | -3.2% | -2.5% | +101.2% | -32.2% | +8.4% |
| 3 | 41187ns | +32.4% | +52.5% | +83.8% | +107.4% | +123.2% | -27.3% | +28.1% |
| 4 | 45760ns | +18.4% | +26.5% | +5.5% | -0.2% | +109.1% | -29.6% | +11.3% |
| 5 | 40673ns | +34.1% | +31.8% | +0.6% | +8.2% | +109.9% | -21.9% | +12.2% |
| 6 | 40789ns | +24.7% | +26.8% | -0.5% | +0.5% | +107.5% | -28.6% | +11.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_bittree_real | -0.163 | ok |
| carrier_disp_fntable_real | -0.176 | ok |
| carrier_disp_ifchain_real | -0.143 | ok |
| carrier_disp_ifchainasc_real | -0.190 | ok |
| carrier_disp_ifchainlin_real | 0.129 | ok |
| carrier_disp_nullfloor_real | 0.272 | moderate+ |
| carrier_disp_switch_real | -0.279 | moderate- |
| carrier_disp_threaded_real | 0.052 | ok |

**Consistency summary:**

- **carrier_disp_bittree_real**: won 0/6, lost 6/6
- **carrier_disp_fntable_real**: won 0/6, lost 6/6
- **carrier_disp_ifchain_real**: won 3/6, lost 3/6
- **carrier_disp_ifchainasc_real**: won 3/6, lost 3/6
- **carrier_disp_ifchainlin_real**: won 0/6, lost 6/6
- **carrier_disp_nullfloor_real**: won 6/6, lost 0/6
- **carrier_disp_threaded_real**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_bittree_real | 169.5ns | 52598.5ns | 0.3% |  |
| carrier_disp_fntable_real | 166.0ns | 55845.3ns | 0.3% |  |
| carrier_disp_ifchain_real | 164.7ns | 48619.7ns | 0.3% |  |
| carrier_disp_ifchainasc_real | 159.4ns | 49561.5ns | 0.3% |  |
| carrier_disp_ifchainlin_real | 160.0ns | 87968.2ns | 0.2% |  |
| carrier_disp_nullfloor_real | 161.6ns | 30134.0ns | 0.5% |  |
| carrier_disp_switch_real | 162.0ns | 42676.2ns | 0.4% |  |
| carrier_disp_threaded_real | 167.8ns | 48011.2ns | 0.3% |  |

## Distribution (algo ns)

```
carrier_disp_bittree_real (n=6, range 48912.5-54519.6 ns)
  48912.5 |########################################
  49192.9 |
  49473.2 |
  49753.6 |
  50033.9 |
  50314.3 |
  50594.6 |
  50875.0 |########################################
  51155.3 |
  51435.7 |
  51716.0 |
  51996.4 |
  52276.7 |
  52557.1 |########################################
  52837.4 |
  53117.8 |
  53398.1 |
  53678.5 |
  53958.8 |########################################
  54239.2 |########################################
  (0 below, 1 above range)

carrier_disp_fntable_real (n=6, range 51661.7-60356.2 ns)
  51661.7 |########################################
  52096.4 |
  52531.1 |
  52965.9 |
  53400.6 |####################
  53835.3 |
  54270.0 |
  54704.8 |
  55139.5 |
  55574.2 |
  56008.9 |
  56443.7 |
  56878.4 |
  57313.1 |####################
  57747.8 |####################
  58182.6 |
  58617.3 |
  59052.0 |
  59486.8 |
  59921.5 |
  (0 below, 1 above range)

carrier_disp_ifchain_real (n=6, range 40590.8-61995.4 ns)
  40590.8 |########################################
  41661.0 |
  42731.3 |
  43801.5 |
  44871.7 |#############
  45942.0 |
  47012.2 |
  48082.4 |#############
  49152.6 |
  50222.9 |
  51293.1 |
  52363.3 |
  53433.6 |
  54503.8 |
  55574.0 |
  56644.2 |
  57714.5 |
  58784.7 |
  59854.9 |
  60925.2 |
  (0 below, 1 above range)

carrier_disp_ifchainasc_real (n=6, range 40284.2-65529.2 ns)
  40284.2 |########################################
  41546.4 |
  42808.7 |#############
  44070.9 |
  45333.2 |#############
  46595.4 |
  47857.7 |
  49119.9 |
  50382.2 |
  51644.4 |
  52906.7 |
  54168.9 |
  55431.2 |
  56693.4 |
  57955.7 |
  59217.9 |
  60480.2 |
  61742.4 |
  63004.7 |
  64266.9 |
  (0 below, 1 above range)

carrier_disp_ifchainlin_real (n=6, range 84621.2-93801.1 ns)
  84621.2 |########################################
  85080.2 |########################################
  85539.2 |
  85998.2 |
  86457.2 |
  86916.2 |
  87375.2 |
  87834.1 |
  88293.1 |
  88752.1 |
  89211.1 |
  89670.1 |
  90129.1 |
  90588.1 |
  91047.1 |
  91506.1 |####################
  91965.1 |
  92424.1 |
  92883.1 |
  93342.1 |
  (0 below, 1 above range)

carrier_disp_nullfloor_real (n=6, range 28531.7-31972.5 ns)
  28531.7 |########################################
  28703.7 |
  28875.8 |
  29047.8 |########################################
  29219.9 |########################################
  29391.9 |
  29563.9 |
  29736.0 |
  29908.0 |########################################
  30080.0 |
  30252.1 |
  30424.1 |
  30596.2 |
  30768.2 |
  30940.2 |
  31112.3 |
  31284.3 |
  31456.3 |
  31628.4 |########################################
  31800.4 |
  (0 below, 1 above range)

carrier_disp_switch_real (n=6, range 40673.3-45652.9 ns)
  40673.3 |########################################
  40922.3 |
  41171.3 |####################
  41420.2 |
  41669.2 |
  41918.2 |####################
  42167.2 |
  42416.2 |
  42665.1 |
  42914.1 |
  43163.1 |
  43412.1 |
  43661.1 |
  43910.0 |
  44159.0 |
  44408.0 |
  44657.0 |
  44906.0 |
  45154.9 |
  45403.9 |####################
  (0 below, 1 above range)

carrier_disp_threaded_real (n=6, range 45590.4-51840.6 ns)
  45590.4 |########################################
  45902.9 |
  46215.4 |
  46527.9 |
  46840.4 |
  47153.0 |
  47465.5 |#############
  47778.0 |
  48090.5 |
  48403.0 |
  48715.5 |
  49028.0 |
  49340.5 |
  49653.0 |
  49965.5 |
  50278.1 |
  50590.6 |
  50903.1 |#############
  51215.6 |
  51528.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_ifchain_real**: CV=25.6% (high variance, measurements may be unstable)
- **carrier_disp_ifchainasc_real**: CV=32.6% (high variance, measurements may be unstable)
