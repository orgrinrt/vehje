# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_ifchainlin shows alternating (throttle bounce) (autocorr -0.73)

carrier_disp_madd_ifchainlin's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_disp_madd_ifchainasc's edge over baseline is significant but tiny (-41 ns, 0.33%)

carrier_disp_madd_ifchainasc differs from baseline carrier_disp_madd_switch by -41 ns (0.33%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 10663.8 ns median (-14.8% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.29x (fastest 10663.8 ns, slowest 13808.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 14234ns | 14235ns | 14065ns | 14182ns | 14396ns | -5.49% |
| carrier_disp_madd_fntable | 16554ns | 16520ns | 16405ns | 16498ns | 16712ns | +9.91% |
| carrier_disp_madd_ifchain | 15324ns | 15267ns | 15045ns | 15255ns | 15567ns | +1.75% |
| carrier_disp_madd_ifchainasc | 14798ns | 14944ns | 13332ns | 14886ns | 15401ns | -1.74% |
| carrier_disp_madd_ifchainlin | 14252ns | 14224ns | 14122ns | 14212ns | 14378ns | -5.37% |
| carrier_disp_madd_nullfloor | 13375ns | 13290ns | 13045ns | 13247ns | 13731ns | -11.19% |
| carrier_disp_madd_switch | 15061ns | 15131ns | 14756ns | 15078ns | 15188ns | base |
| carrier_disp_madd_threaded | 15846ns | 15773ns | 15585ns | 15732ns | 16148ns | +5.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 11641ns | 11399ns | 11780ns | -6.64% | 0.022 |
| carrier_disp_madd_fntable | 13811ns | 13701ns | 13877ns | +10.76% | 0.019 |
| carrier_disp_madd_ifchain | 12698ns | 12488ns | 12923ns | +1.84% | 0.020 |
| carrier_disp_madd_ifchainasc | 12209ns | 10985ns | 12621ns | -2.09% | 0.021 |
| carrier_disp_madd_ifchainlin | 11598ns | 11470ns | 11722ns | -6.98% | 0.022 |
| carrier_disp_madd_nullfloor | 10731ns | 10577ns | 10925ns | -13.94% | 0.024 |
| carrier_disp_madd_switch | 12469ns | 12228ns | 12629ns | base | 0.021 |
| carrier_disp_madd_threaded | 13174ns | 13001ns | 13366ns | +5.65% | 0.019 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.022 | 90.5% |
| carrier_disp_madd_fntable | 0.019 | 76.6% |
| carrier_disp_madd_ifchain | 0.020 | 83.6% |
| carrier_disp_madd_ifchainasc | 0.021 | 85.3% |
| carrier_disp_madd_ifchainlin | 0.022 | 91.4% |
| carrier_disp_madd_nullfloor | 0.024 | 99.2% |
| carrier_disp_madd_switch | 0.020 | 84.6% |
| carrier_disp_madd_threaded | 0.019 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 14234ns | 14234ns | -5.49% |
| carrier_disp_madd_fntable | 16554ns | 16554ns | +9.91% |
| carrier_disp_madd_ifchain | 15324ns | 15324ns | +1.75% |
| carrier_disp_madd_ifchainasc | 14798ns | 14798ns | -1.74% |
| carrier_disp_madd_ifchainlin | 14252ns | 14252ns | -5.37% |
| carrier_disp_madd_nullfloor | 13375ns | 13375ns | -11.19% |
| carrier_disp_madd_switch | 15061ns | 15061ns | base |
| carrier_disp_madd_threaded | 15846ns | 15846ns | +5.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 12509ns | base | --- | [12269, 12629] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 11690ns | -821.2ns (-6.6%) | [-1052, -609]ns | [11454, 11780] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_fntable | 13809ns | +1336.1ns (+10.7%) | [+1149, +1540]ns | [13746, 13877] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 12651ns | no significant difference | [-79, +655]ns | [12519, 12923] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_madd_ifchainasc | 12404ns | no significant difference | [-924, +185]ns | [11601, 12621] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_madd_ifchainlin | 11577ns | -818.5ns (-6.5%) | [-1032, -761]ns | [11496, 11722] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 10664ns | -1815.2ns (-14.5%) | [-1920, -1478]ns | [10604, 10925] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 13138ns | +678.1ns (+5.4%) | [+419, +1018]ns | [13018, 13366] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 12550ns | -6.5% | +9.2% | +1.1% | -12.5% | -8.2% | -15.3% | +5.3% |
| 2 | 12613ns | -7.7% | +9.7% | -0.0% | -2.2% | -6.6% | -15.2% | +3.6% |
| 3 | 12228ns | -6.8% | +12.8% | +5.5% | +2.2% | -6.2% | -13.0% | +8.1% |
| 4 | 12468ns | -5.8% | +11.6% | +0.7% | +0.1% | -6.4% | -15.2% | +8.4% |
| 5 | 12644ns | -9.0% | +9.1% | -1.2% | +0.8% | -8.2% | -13.8% | +3.1% |
| 6 | 12309ns | -4.1% | +12.3% | +5.2% | -0.7% | -6.2% | -11.1% | +5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.505 | HIGH- (thermal bounce) |
| carrier_disp_madd_fntable | -0.306 | moderate- |
| carrier_disp_madd_ifchain | -0.385 | moderate- |
| carrier_disp_madd_ifchainasc | 0.058 | ok |
| carrier_disp_madd_ifchainlin | -0.729 | HIGH- (thermal bounce) |
| carrier_disp_madd_nullfloor | 0.273 | moderate+ |
| carrier_disp_madd_switch | -0.359 | moderate- |
| carrier_disp_madd_threaded | -0.106 | ok |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 6/6, lost 0/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 1/6, lost 4/6
- **carrier_disp_madd_ifchainasc**: won 3/6, lost 2/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 90977.3ns | 11641.4ns | 781.5% | HIGH |
| carrier_disp_madd_fntable | 94431.7ns | 13810.6ns | 683.8% | HIGH |
| carrier_disp_madd_ifchain | 89056.4ns | 12697.8ns | 701.4% | HIGH |
| carrier_disp_madd_ifchainasc | 91629.3ns | 12208.8ns | 750.5% | HIGH |
| carrier_disp_madd_ifchainlin | 92037.0ns | 11598.4ns | 793.5% | HIGH |
| carrier_disp_madd_nullfloor | 93476.1ns | 10731.0ns | 871.1% | HIGH |
| carrier_disp_madd_switch | 90285.7ns | 12468.8ns | 724.1% | HIGH |
| carrier_disp_madd_threaded | 92334.0ns | 13173.8ns | 700.9% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 11399.2-11780.0 ns)
  11399.2 |########################################
  11418.2 |
  11437.3 |
  11456.3 |
  11475.4 |
  11494.4 |########################################
  11513.4 |
  11532.5 |
  11551.5 |
  11570.6 |
  11589.6 |
  11608.6 |
  11627.7 |########################################
  11646.7 |
  11665.8 |
  11684.8 |
  11703.8 |
  11722.9 |########################################
  11741.9 |########################################
  11761.0 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 13700.8-13876.6 ns)
  13700.8 |####################
  13709.6 |
  13718.4 |
  13727.2 |
  13736.0 |
  13744.8 |
  13753.6 |
  13762.3 |
  13771.1 |
  13779.9 |
  13788.7 |########################################
  13797.5 |
  13806.3 |
  13815.1 |####################
  13823.9 |
  13832.7 |####################
  13841.5 |
  13850.3 |
  13859.1 |
  13867.9 |
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 12487.5-12923.3 ns)
  12487.5 |########################################
  12509.3 |
  12531.1 |########################################
  12552.9 |
  12574.7 |
  12596.5 |########################################
  12618.2 |
  12640.0 |
  12661.8 |
  12683.6 |########################################
  12705.4 |
  12727.2 |
  12749.0 |
  12770.8 |
  12792.6 |
  12814.3 |
  12836.1 |
  12857.9 |
  12879.7 |########################################
  12901.5 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 10985.0-12621.3 ns)
  10985.0 |####################
  11066.8 |
  11148.6 |
  11230.4 |
  11312.3 |
  11394.1 |
  11475.9 |
  11557.7 |
  11639.5 |
  11721.3 |
  11803.1 |
  11885.0 |
  11966.8 |
  12048.6 |
  12130.4 |
  12212.2 |####################
  12294.0 |####################
  12375.9 |
  12457.7 |########################################
  12539.5 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 11470.4-11722.1 ns)
  11470.4 |########################################
  11483.0 |
  11495.6 |
  11508.2 |
  11520.7 |########################################
  11533.3 |########################################
  11545.9 |
  11558.5 |
  11571.1 |
  11583.7 |
  11596.2 |########################################
  11608.8 |
  11621.4 |
  11634.0 |
  11646.6 |
  11659.2 |########################################
  11671.8 |
  11684.3 |
  11696.9 |
  11709.5 |
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 10577.1-10925.4 ns)
  10577.1 |####################
  10594.5 |
  10611.9 |
  10629.3 |########################################
  10646.8 |
  10664.2 |
  10681.6 |####################
  10699.0 |
  10716.4 |
  10733.8 |
  10751.2 |
  10768.7 |
  10786.1 |
  10803.5 |
  10820.9 |
  10838.3 |
  10855.7 |
  10873.2 |
  10890.6 |####################
  10908.0 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 12227.9-12628.5 ns)
  12227.9 |########################################
  12247.9 |
  12268.0 |
  12288.0 |
  12308.0 |########################################
  12328.1 |
  12348.1 |
  12368.1 |
  12388.2 |
  12408.2 |
  12428.2 |
  12448.3 |
  12468.3 |########################################
  12488.3 |
  12508.4 |
  12528.4 |
  12548.4 |########################################
  12568.5 |
  12588.5 |
  12608.5 |########################################
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 13001.2-13366.2 ns)
  13001.2 |####################
  13019.5 |####################
  13037.7 |
  13056.0 |####################
  13074.2 |
  13092.5 |
  13110.7 |
  13129.0 |
  13147.2 |
  13165.5 |
  13183.7 |
  13202.0 |########################################
  13220.2 |
  13238.5 |
  13256.7 |
  13275.0 |
  13293.2 |
  13311.5 |
  13329.7 |
  13348.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=778.7% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=684.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=701.2% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=744.9% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=794.6% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=875.0% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=717.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=701.4% of algo (FFI overhead may distort results)
