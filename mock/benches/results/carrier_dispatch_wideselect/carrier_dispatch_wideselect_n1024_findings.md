# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 37% faster than the next best (carrier_disp_wideselect_switch)

carrier_disp_wideselect_nullfloor (29.33 us) leads carrier_disp_wideselect_switch (40.15 us) by 37%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 27% (significant)

carrier_disp_wideselect_nullfloor is -10.91 us (27%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 3.3x slower than the field

carrier_disp_wideselect_ifchainlin (97.69 us) is 3.3x the fastest (29.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_wideselect_ifchain shows alternating (throttle bounce) (autocorr -0.58)

carrier_disp_wideselect_ifchain's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} vs {carrier_disp_wideselect_ifchainlin} (93% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_threaded, carrier_disp_wideselect_bittree, carrier_disp_wideselect_fntable} and a slow tier {carrier_disp_wideselect_ifchainlin} with a 93% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest carrier_disp_wideselect_nullfloor (29.33 us) to slowest carrier_disp_wideselect_ifchainlin (97.69 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 29328.1 ns median (-27.0% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 3.33x (fastest 29328.1 ns, slowest 97693.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 50986ns | 51081ns | 50333ns | 50971ns | 51335ns | +17.99% |
| carrier_disp_wideselect_fntable | 53611ns | 52717ns | 52320ns | 52602ns | 55769ns | +24.06% |
| carrier_disp_wideselect_ifchain | 45223ns | 44918ns | 43641ns | 44596ns | 46954ns | +4.65% |
| carrier_disp_wideselect_ifchainasc | 43211ns | 42995ns | 41815ns | 42685ns | 44699ns | -0.00% |
| carrier_disp_wideselect_ifchainlin | 102317ns | 100056ns | 97306ns | 99318ns | 109319ns | +136.78% |
| carrier_disp_wideselect_nullfloor | 31886ns | 31562ns | 30723ns | 31348ns | 33274ns | -26.21% |
| carrier_disp_wideselect_switch | 43212ns | 42354ns | 41611ns | 42273ns | 45421ns | base |
| carrier_disp_wideselect_threaded | 47624ns | 46788ns | 45187ns | 46484ns | 50552ns | +10.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 48648ns | 47981ns | 48985ns | +18.79% | 0.021 |
| carrier_disp_wideselect_fntable | 51369ns | 50190ns | 53398ns | +25.43% | 0.020 |
| carrier_disp_wideselect_ifchain | 42968ns | 41381ns | 44663ns | +4.92% | 0.024 |
| carrier_disp_wideselect_ifchainasc | 40882ns | 39449ns | 42198ns | -0.18% | 0.025 |
| carrier_disp_wideselect_ifchainlin | 99943ns | 95090ns | 106809ns | +144.04% | 0.010 |
| carrier_disp_wideselect_nullfloor | 29646ns | 28590ns | 30962ns | -27.61% | 0.035 |
| carrier_disp_wideselect_switch | 40954ns | 39466ns | 43050ns | base | 0.025 |
| carrier_disp_wideselect_threaded | 45301ns | 43072ns | 48129ns | +10.61% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 434029 | 1902596 | 0.228 | 0.89× |
| carrier_disp_wideselect_fntable | 483864 | 2491444 | 0.194 | 1.00× |
| carrier_disp_wideselect_ifchain | 466925 | 2174644 | 0.215 | 0.96× |
| carrier_disp_wideselect_ifchainasc | 486068 | 2380364 | 0.204 | 1.00× |
| carrier_disp_wideselect_ifchainlin | 610677 | 3280019 | 0.186 | 1.26× |
| carrier_disp_wideselect_nullfloor | 372962 | 2101244 | 0.177 | 0.77× |
| carrier_disp_wideselect_switch | 485031 | 2311348 | 0.210 | 1.00× |
| carrier_disp_wideselect_threaded | 417782 | 2478947 | 0.169 | 0.86× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.021 | 58.7% |
| carrier_disp_wideselect_fntable | 0.020 | 56.6% |
| carrier_disp_wideselect_ifchain | 0.024 | 67.0% |
| carrier_disp_wideselect_ifchainasc | 0.025 | 70.1% |
| carrier_disp_wideselect_ifchainlin | 0.010 | 29.3% |
| carrier_disp_wideselect_nullfloor | 0.035 | 97.5% |
| carrier_disp_wideselect_switch | 0.026 | 71.2% |
| carrier_disp_wideselect_threaded | 0.023 | 64.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 50986ns | 50986ns | +17.99% |
| carrier_disp_wideselect_fntable | 53611ns | 53611ns | +24.06% |
| carrier_disp_wideselect_ifchain | 45223ns | 45223ns | +4.65% |
| carrier_disp_wideselect_ifchainasc | 43211ns | 43211ns | -0.00% |
| carrier_disp_wideselect_ifchainlin | 102317ns | 102317ns | +136.78% |
| carrier_disp_wideselect_nullfloor | 31886ns | 31886ns | -26.21% |
| carrier_disp_wideselect_switch | 43212ns | 43212ns | base |
| carrier_disp_wideselect_threaded | 47624ns | 47624ns | +10.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 40149ns | base | --- | [39664, 43050] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 48731ns | +8564.0ns (+21.3%) | [+5234, +9282]ns | [48227, 48985] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 50500ns | +10272.3ns (+25.6%) | [+7239, +13734]ns | [50210, 53398] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchain | 42652ns | +1499.4ns (+3.7%) | [+883, +3659]ns | [41590, 44663] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_ifchainasc | 40772ns | no significant difference | [-1901, +2156]ns | [39677, 42198] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainlin | 97694ns | +55866.7ns (+139.1%) | [+54316, +66783]ns | [95326, 106809] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 29328ns | -10906.7ns (-27.2%) | [-14153, -8865]ns | [28647, 30962] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 44401ns | +4416.0ns (+11.0%) | [+2239, +6386]ns | [43374, 48129] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 40108ns | +22.1% | +26.3% | +3.2% | -1.6% | +138.3% | -28.7% | +11.7% |
| 2 | 42803ns | +12.1% | +17.4% | +1.2% | -6.8% | +124.2% | -32.0% | +2.0% |
| 3 | 40189ns | +20.6% | +24.9% | +4.0% | -0.7% | +176.6% | -21.5% | +17.9% |
| 4 | 43296ns | +12.2% | +16.3% | +3.2% | -2.1% | +129.6% | -33.7% | +12.9% |
| 5 | 39863ns | +22.6% | +34.1% | +5.4% | +5.4% | +157.0% | -25.8% | +10.3% |
| 6 | 39466ns | +24.2% | +35.2% | +13.1% | +5.5% | +140.9% | -23.1% | +9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.007 | ok |
| carrier_disp_wideselect_fntable | 0.430 | moderate+ |
| carrier_disp_wideselect_ifchain | -0.578 | HIGH- (thermal bounce) |
| carrier_disp_wideselect_ifchainasc | 0.424 | moderate+ |
| carrier_disp_wideselect_ifchainlin | -0.244 | moderate- |
| carrier_disp_wideselect_nullfloor | -0.349 | moderate- |
| carrier_disp_wideselect_switch | -0.419 | moderate- |
| carrier_disp_wideselect_threaded | 0.115 | ok |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 0/6, lost 6/6
- **carrier_disp_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchain**: won 0/6, lost 6/6
- **carrier_disp_wideselect_ifchainasc**: won 4/6, lost 2/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 97412.2ns | 48647.8ns | 200.2% | HIGH |
| carrier_disp_wideselect_fntable | 103728.7ns | 51369.2ns | 201.9% | HIGH |
| carrier_disp_wideselect_ifchain | 107430.1ns | 42968.1ns | 250.0% | HIGH |
| carrier_disp_wideselect_ifchainasc | 116189.6ns | 40882.2ns | 284.2% | HIGH |
| carrier_disp_wideselect_ifchainlin | 100008.9ns | 99943.0ns | 100.1% | HIGH |
| carrier_disp_wideselect_nullfloor | 89921.5ns | 29645.8ns | 303.3% | HIGH |
| carrier_disp_wideselect_switch | 115846.2ns | 40954.2ns | 282.9% | HIGH |
| carrier_disp_wideselect_threaded | 90671.7ns | 45301.3ns | 200.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 47981.2-48985.0 ns)
  47981.2 |########################################
  48031.4 |
  48081.6 |
  48131.8 |
  48182.0 |
  48232.1 |
  48282.3 |
  48332.5 |
  48382.7 |
  48432.9 |########################################
  48483.1 |
  48533.3 |
  48583.5 |########################################
  48633.7 |
  48683.9 |
  48734.1 |
  48784.2 |
  48834.4 |########################################
  48884.6 |
  48934.8 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 50189.6-53398.3 ns)
  50189.6 |########################################
  50350.0 |
  50510.5 |#############
  50670.9 |
  50831.3 |
  50991.8 |
  51152.2 |
  51312.6 |
  51473.1 |
  51633.5 |
  51793.9 |
  51954.4 |
  52114.8 |
  52275.3 |
  52435.7 |
  52596.1 |
  52756.6 |
  52917.0 |
  53077.4 |
  53237.9 |#############
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 41381.2-44662.7 ns)
  41381.2 |########################################
  41545.3 |
  41709.3 |########################################
  41873.4 |########################################
  42037.5 |
  42201.6 |
  42365.6 |
  42529.7 |
  42693.8 |
  42857.9 |
  43021.9 |
  43186.0 |########################################
  43350.1 |
  43514.2 |
  43678.2 |
  43842.3 |
  44006.4 |
  44170.5 |
  44334.5 |
  44498.6 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 39449.2-42197.8 ns)
  39449.2 |####################
  39586.6 |
  39724.1 |
  39861.5 |########################################
  39998.9 |
  40136.3 |
  40273.8 |
  40411.2 |
  40548.6 |
  40686.0 |
  40823.5 |
  40960.9 |
  41098.3 |
  41235.8 |
  41373.2 |
  41510.6 |####################
  41648.0 |
  41785.5 |
  41922.9 |####################
  42060.3 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 95090.0-106809.1 ns)
  95090.0 |########################################
  95676.0 |####################
  96261.9 |
  96847.9 |
  97433.8 |
  98019.8 |
  98605.7 |
  99191.7 |####################
  99777.7 |
  100363.6 |
  100949.6 |
  101535.5 |
  102121.5 |####################
  102707.4 |
  103293.4 |
  103879.4 |
  104465.3 |
  105051.3 |
  105637.2 |
  106223.2 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 28590.0-30962.1 ns)
  28590.0 |########################################
  28708.6 |
  28827.2 |
  28945.8 |
  29064.4 |####################
  29183.0 |
  29301.6 |
  29420.2 |
  29538.8 |####################
  29657.4 |
  29776.0 |
  29894.6 |
  30013.2 |
  30131.8 |
  30250.4 |####################
  30369.0 |
  30487.6 |
  30606.2 |
  30724.8 |
  30843.4 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 39465.8-43049.8 ns)
  39465.8 |########################################
  39645.0 |
  39824.2 |########################################
  40003.4 |########################################
  40182.6 |########################################
  40361.8 |
  40541.0 |
  40720.2 |
  40899.4 |
  41078.6 |
  41257.8 |
  41437.0 |
  41616.2 |
  41795.4 |
  41974.6 |
  42153.8 |
  42333.0 |
  42512.2 |
  42691.4 |########################################
  42870.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 43071.7-48128.9 ns)
  43071.7 |########################################
  43324.6 |
  43577.4 |########################################
  43830.3 |########################################
  44083.1 |
  44336.0 |
  44588.9 |########################################
  44841.7 |
  45094.6 |
  45347.5 |
  45600.3 |
  45853.2 |
  46106.0 |
  46358.9 |
  46611.8 |
  46864.6 |
  47117.5 |
  47370.4 |########################################
  47623.2 |
  47876.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=201.8% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=252.7% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=286.5% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=304.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=296.4% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=200.0% of algo (FFI overhead may distort results)
