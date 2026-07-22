# Dispatch shape over the wire form, leaf profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_leaf_nullfloor dominates: 36% faster than the next best (carrier_disp_leaf_bittree)

carrier_disp_leaf_nullfloor (568.45 us) leads carrier_disp_leaf_bittree (774.38 us) by 36%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_leaf_nullfloor beats baseline by 46% (significant)

carrier_disp_leaf_nullfloor is -493.59 us (46%) faster than baseline carrier_disp_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_leaf_threaded is an outlier: 2.2x slower than the field

carrier_disp_leaf_threaded (1.24 ms) is 2.2x the fastest (568.45 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_leaf_ifchain shows alternating (throttle bounce) (autocorr -0.88)

carrier_disp_leaf_ifchain's per-pass series has lag-1 autocorrelation -0.88, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_leaf_nullfloor} vs {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded} (36% apart)

The field splits into a fast tier {carrier_disp_leaf_nullfloor} and a slow tier {carrier_disp_leaf_bittree, carrier_disp_leaf_ifchainlin, carrier_disp_leaf_switch, carrier_disp_leaf_ifchainasc, carrier_disp_leaf_ifchain, carrier_disp_leaf_fntable, carrier_disp_leaf_threaded} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_leaf_nullfloor** at 568452.3 ns median (-46.5% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.19x (fastest 568452.3 ns, slowest 1243971.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 782374ns | 778283ns | 762396ns | 774541ns | 804114ns | -26.71% |
| carrier_disp_leaf_fntable | 1142224ns | 1142006ns | 1140175ns | 1141675ns | 1144072ns | +7.00% |
| carrier_disp_leaf_ifchain | 1080990ns | 1081484ns | 1071965ns | 1079348ns | 1087966ns | +1.27% |
| carrier_disp_leaf_ifchainasc | 1066906ns | 1069012ns | 1055214ns | 1067029ns | 1072568ns | -0.05% |
| carrier_disp_leaf_ifchainlin | 979968ns | 980856ns | 974945ns | 979352ns | 983404ns | -8.20% |
| carrier_disp_leaf_nullfloor | 576588ns | 571805ns | 569010ns | 571115ns | 588586ns | -45.99% |
| carrier_disp_leaf_switch | 1067475ns | 1065869ns | 1064639ns | 1065572ns | 1071748ns | base |
| carrier_disp_leaf_threaded | 1247921ns | 1248304ns | 1243737ns | 1246807ns | 1251684ns | +16.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_leaf_bittree | 778655ns | 758945ns | 800288ns | -26.78% | 0.021 |
| carrier_disp_leaf_fntable | 1138339ns | 1136540ns | 1140211ns | +7.04% | 0.014 |
| carrier_disp_leaf_ifchain | 1076938ns | 1068189ns | 1083940ns | +1.27% | 0.015 |
| carrier_disp_leaf_ifchainasc | 1063109ns | 1051558ns | 1068787ns | -0.03% | 0.015 |
| carrier_disp_leaf_ifchainlin | 976607ns | 971674ns | 979916ns | -8.17% | 0.017 |
| carrier_disp_leaf_nullfloor | 573040ns | 565442ns | 584927ns | -46.12% | 0.029 |
| carrier_disp_leaf_switch | 1063475ns | 1060572ns | 1067998ns | base | 0.015 |
| carrier_disp_leaf_threaded | 1243900ns | 1239638ns | 1247700ns | +16.97% | 0.013 |

## Performance model

- Peak throughput: **0.029 Gops/s** (carrier_disp_leaf_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_leaf_bittree | 0.021 | 73.0% |
| carrier_disp_leaf_fntable | 0.014 | 49.7% |
| carrier_disp_leaf_ifchain | 0.015 | 52.5% |
| carrier_disp_leaf_ifchainasc | 0.015 | 53.1% |
| carrier_disp_leaf_ifchainlin | 0.017 | 57.8% |
| carrier_disp_leaf_nullfloor | 0.029 | 99.5% |
| carrier_disp_leaf_switch | 0.015 | 53.3% |
| carrier_disp_leaf_threaded | 0.013 | 45.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_leaf_bittree | 782374ns | 782374ns | -26.71% |
| carrier_disp_leaf_fntable | 1142224ns | 1142224ns | +7.00% |
| carrier_disp_leaf_ifchain | 1080990ns | 1080990ns | +1.27% |
| carrier_disp_leaf_ifchainasc | 1066906ns | 1066906ns | -0.05% |
| carrier_disp_leaf_ifchainlin | 979968ns | 979968ns | -8.20% |
| carrier_disp_leaf_nullfloor | 576588ns | 576588ns | -45.99% |
| carrier_disp_leaf_switch | 1067475ns | 1067475ns | base |
| carrier_disp_leaf_threaded | 1247921ns | 1247921ns | +16.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_leaf_switch | 1061763ns | base | --- | [1060666, 1067998] | --- | --- | --- | --- |
| carrier_disp_leaf_bittree | 774377ns | -286289.0ns (-27.0%) | [-300462, -267710]ns | [761301, 800288] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_leaf_fntable | 1137901ns | +76344.9ns (+7.2%) | [+70886, +77359]ns | [1136904, 1140211] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_leaf_ifchain | 1077321ns | +14269.8ns (+1.3%) | [+4673, +21446]ns | [1069555, 1083940] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_leaf_ifchainasc | 1065067ns | no significant difference | [-10699, +5684]ns | [1055472, 1068787] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_leaf_ifchainlin | 977494ns | -85346.3ns (-8.0%) | [-93685, -81575]ns | [972410, 979916] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_leaf_nullfloor | 568452ns | -493588.1ns (-46.5%) | [-499760, -477960]ns | [565739, 584927] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_leaf_threaded | 1243971ns | +181075.2ns (+17.1%) | [+173557, +186640]ns | [1240028, 1247700] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_leaf_switch | carrier_disp_leaf_bittree | carrier_disp_leaf_fntable | carrier_disp_leaf_ifchain | carrier_disp_leaf_ifchainasc | carrier_disp_leaf_ifchainlin | carrier_disp_leaf_nullfloor | carrier_disp_leaf_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 1060572ns | -26.7% | +7.3% | +1.8% | -0.9% | -8.2% | -46.4% | +17.6% |
| 2 | 1061978ns | -28.1% | +7.1% | +0.8% | +0.6% | -7.8% | -46.8% | +17.2% |
| 3 | 1060759ns | -27.3% | +7.1% | +2.1% | +0.4% | -7.8% | -46.6% | +16.9% |
| 4 | 1061547ns | -28.5% | +7.2% | +0.6% | +0.3% | -7.6% | -46.2% | +17.6% |
| 5 | 1064227ns | -25.7% | +7.3% | +1.9% | +0.4% | -8.7% | -43.8% | +16.8% |
| 6 | 1071768ns | -24.4% | +6.2% | +0.3% | -1.2% | -8.8% | -46.9% | +15.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_leaf_bittree | 0.232 | moderate+ |
| carrier_disp_leaf_fntable | 0.146 | ok |
| carrier_disp_leaf_ifchain | -0.878 | HIGH- (thermal bounce) |
| carrier_disp_leaf_ifchainasc | -0.277 | moderate- |
| carrier_disp_leaf_ifchainlin | -0.344 | moderate- |
| carrier_disp_leaf_nullfloor | -0.056 | ok |
| carrier_disp_leaf_switch | 0.202 | moderate+ |
| carrier_disp_leaf_threaded | -0.250 | moderate- |

**Consistency summary:**

- **carrier_disp_leaf_bittree**: won 6/6, lost 0/6
- **carrier_disp_leaf_fntable**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchain**: won 0/6, lost 6/6
- **carrier_disp_leaf_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_leaf_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_leaf_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_leaf_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_leaf_bittree | 778914.0ns | 778655.0ns | 100.0% | HIGH |
| carrier_disp_leaf_fntable | 1145705.1ns | 1138338.6ns | 100.6% | HIGH |
| carrier_disp_leaf_ifchain | 1078875.7ns | 1076938.5ns | 100.2% | HIGH |
| carrier_disp_leaf_ifchainasc | 1064601.7ns | 1063108.5ns | 100.1% | HIGH |
| carrier_disp_leaf_ifchainlin | 977829.5ns | 976606.6ns | 100.1% | HIGH |
| carrier_disp_leaf_nullfloor | 575391.0ns | 573039.5ns | 100.4% | HIGH |
| carrier_disp_leaf_switch | 1065526.6ns | 1063475.4ns | 100.2% | HIGH |
| carrier_disp_leaf_threaded | 1244713.3ns | 1243899.6ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_disp_leaf_bittree (n=6, range 758945.4-800287.5 ns)
  758945.4 |########################################
  761012.5 |
  763079.6 |########################################
  765146.7 |
  767213.8 |
  769280.9 |########################################
  771348.0 |
  773415.1 |
  775482.2 |
  777549.3 |########################################
  779616.4 |
  781683.6 |
  783750.7 |
  785817.8 |
  787884.9 |
  789952.0 |########################################
  792019.1 |
  794086.2 |
  796153.3 |
  798220.4 |
  (0 below, 1 above range)

carrier_disp_leaf_fntable (n=6, range 1136540.0-1140210.6 ns)
  1136540.0 |########################################
  1136723.5 |
  1136907.1 |
  1137090.6 |########################################
  1137274.1 |
  1137457.6 |########################################
  1137641.2 |
  1137824.7 |
  1138008.2 |
  1138191.8 |########################################
  1138375.3 |########################################
  1138558.8 |
  1138742.4 |
  1138925.9 |
  1139109.4 |
  1139293.0 |
  1139476.5 |
  1139660.0 |
  1139843.5 |
  1140027.1 |
  (0 below, 1 above range)

carrier_disp_leaf_ifchain (n=6, range 1068188.8-1083939.6 ns)
  1068188.8 |########################################
  1068976.3 |
  1069763.9 |
  1070551.4 |########################################
  1071339.0 |
  1072126.5 |
  1072914.0 |
  1073701.6 |########################################
  1074489.1 |
  1075276.7 |
  1076064.2 |
  1076851.7 |
  1077639.3 |
  1078426.8 |
  1079214.4 |
  1080001.9 |########################################
  1080789.4 |
  1081577.0 |
  1082364.5 |
  1083152.1 |########################################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainasc (n=6, range 1051557.5-1068786.7 ns)
  1051557.5 |####################
  1052419.0 |
  1053280.4 |
  1054141.9 |
  1055003.3 |
  1055864.8 |
  1056726.3 |
  1057587.7 |
  1058449.2 |
  1059310.6 |####################
  1060172.1 |
  1061033.6 |
  1061895.0 |
  1062756.5 |
  1063617.9 |
  1064479.4 |########################################
  1065340.9 |
  1066202.3 |
  1067063.8 |
  1067925.2 |####################
  (0 below, 1 above range)

carrier_disp_leaf_ifchainlin (n=6, range 971673.7-979915.6 ns)
  971673.7 |########################################
  972085.8 |
  972497.9 |
  972910.0 |########################################
  973322.1 |
  973734.2 |
  974146.3 |
  974558.4 |
  974970.5 |
  975382.6 |
  975794.7 |
  976206.8 |
  976618.9 |########################################
  977031.0 |
  977443.1 |
  977855.2 |########################################
  978267.3 |
  978679.4 |########################################
  979091.5 |
  979503.6 |
  (0 below, 1 above range)

carrier_disp_leaf_nullfloor (n=6, range 565442.5-584927.2 ns)
  565442.5 |########################################
  566416.7 |
  567391.0 |####################
  568365.2 |####################
  569339.4 |
  570313.7 |
  571287.9 |####################
  572262.2 |
  573236.4 |
  574210.6 |
  575184.9 |
  576159.1 |
  577133.3 |
  578107.6 |
  579081.8 |
  580056.1 |
  581030.3 |
  582004.5 |
  582978.8 |
  583953.0 |
  (0 below, 1 above range)

carrier_disp_leaf_switch (n=6, range 1060572.5-1067997.7 ns)
  1060572.5 |########################################
  1060943.8 |
  1061315.0 |####################
  1061686.3 |####################
  1062057.5 |
  1062428.8 |
  1062800.1 |
  1063171.3 |
  1063542.6 |
  1063913.8 |####################
  1064285.1 |
  1064656.4 |
  1065027.6 |
  1065398.9 |
  1065770.1 |
  1066141.4 |
  1066512.7 |
  1066883.9 |
  1067255.2 |
  1067626.4 |
  (0 below, 1 above range)

carrier_disp_leaf_threaded (n=6, range 1239637.5-1247699.6 ns)
  1239637.5 |########################################
  1240040.6 |########################################
  1240443.7 |
  1240846.8 |
  1241249.9 |
  1241653.0 |
  1242056.1 |
  1242459.2 |
  1242862.3 |
  1243265.4 |########################################
  1243668.6 |
  1244071.7 |########################################
  1244474.8 |
  1244877.9 |
  1245281.0 |
  1245684.1 |
  1246087.2 |
  1246490.3 |
  1246893.4 |########################################
  1247296.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_leaf_bittree**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_fntable**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchain**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_ifchainlin**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_leaf_threaded**: bridge=100.1% of algo (FFI overhead may distort results)
