# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 34% faster than the next best (carrier_disp_tight_switch)

carrier_disp_tight_nullfloor (8.32 us) leads carrier_disp_tight_switch (11.13 us) by 34%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 25% (significant)

carrier_disp_tight_nullfloor is -2.83 us (25%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_ifchainlin is an outlier: 2.1x slower than the field

carrier_disp_tight_ifchainlin (17.18 us) is 2.1x the fastest (8.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_tight_threaded shows alternating (throttle bounce) (autocorr -0.83)

carrier_disp_tight_threaded's per-pass series has lag-1 autocorrelation -0.83, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_switch, carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} (34% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_switch, carrier_disp_tight_ifchainasc, carrier_disp_tight_ifchain, carrier_disp_tight_bittree, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} with a 34% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 8315.6 ns median (-25.3% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 2.07x (fastest 8315.6 ns, slowest 17183.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 14235ns | 14315ns | 13744ns | 14283ns | 14408ns | +3.61% |
| carrier_disp_tight_fntable | 15696ns | 15641ns | 15431ns | 15614ns | 15952ns | +14.25% |
| carrier_disp_tight_ifchain | 14095ns | 14112ns | 13881ns | 14096ns | 14203ns | +2.60% |
| carrier_disp_tight_ifchainasc | 13551ns | 13875ns | 12213ns | 13415ns | 14425ns | -1.36% |
| carrier_disp_tight_ifchainlin | 19814ns | 19785ns | 19601ns | 19765ns | 19994ns | +44.23% |
| carrier_disp_tight_nullfloor | 10931ns | 10942ns | 10705ns | 10892ns | 11102ns | -20.43% |
| carrier_disp_tight_switch | 13738ns | 13748ns | 13425ns | 13736ns | 13897ns | base |
| carrier_disp_tight_threaded | 15190ns | 15234ns | 14878ns | 15191ns | 15346ns | +10.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 11579ns | 11187ns | 11718ns | +4.16% | 0.022 |
| carrier_disp_tight_fntable | 13071ns | 12955ns | 13198ns | +17.58% | 0.020 |
| carrier_disp_tight_ifchain | 11421ns | 11348ns | 11528ns | +2.74% | 0.022 |
| carrier_disp_tight_ifchainasc | 11074ns | 9960ns | 11807ns | -0.38% | 0.023 |
| carrier_disp_tight_ifchainlin | 17207ns | 17129ns | 17296ns | +54.78% | 0.015 |
| carrier_disp_tight_nullfloor | 8294ns | 8221ns | 8342ns | -25.39% | 0.031 |
| carrier_disp_tight_switch | 11117ns | 10918ns | 11217ns | base | 0.023 |
| carrier_disp_tight_threaded | 12548ns | 12426ns | 12657ns | +12.87% | 0.020 |

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.022 | 70.7% |
| carrier_disp_tight_fntable | 0.020 | 63.0% |
| carrier_disp_tight_ifchain | 0.022 | 72.2% |
| carrier_disp_tight_ifchainasc | 0.023 | 72.6% |
| carrier_disp_tight_ifchainlin | 0.015 | 47.8% |
| carrier_disp_tight_nullfloor | 0.031 | 98.9% |
| carrier_disp_tight_switch | 0.023 | 73.9% |
| carrier_disp_tight_threaded | 0.020 | 65.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 14235ns | 14235ns | +3.61% |
| carrier_disp_tight_fntable | 15696ns | 15696ns | +14.25% |
| carrier_disp_tight_ifchain | 14095ns | 14095ns | +2.60% |
| carrier_disp_tight_ifchainasc | 13551ns | 13551ns | -1.36% |
| carrier_disp_tight_ifchainlin | 19814ns | 19814ns | +44.23% |
| carrier_disp_tight_nullfloor | 10931ns | 10931ns | -20.43% |
| carrier_disp_tight_switch | 13738ns | 13738ns | base |
| carrier_disp_tight_threaded | 15190ns | 15190ns | +10.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 11130ns | base | --- | [11004, 11217] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 11632ns | +462.5ns (+4.2%) | [+324, +602]ns | [11387, 11718] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 13051ns | +1921.8ns (+17.3%) | [+1794, +2146]ns | [12963, 13198] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 11384ns | +276.1ns (+2.5%) | [+183, +454]ns | [11352, 11528] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_ifchainasc | 11325ns | no significant difference | [-1091, +755]ns | [10091, 11807] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_tight_ifchainlin | 17184ns | +6081.8ns (+54.6%) | [+5983, +6205]ns | [17140, 17296] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 8316ns | -2830.9ns (-25.4%) | [-2909, -2728]ns | [8225, 8342] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 12548ns | +1474.8ns (+13.3%) | [+1270, +1549]ns | [12439, 12657] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 11115ns | +4.3% | +17.5% | +2.5% | -8.0% | +54.6% | -24.9% | +13.6% |
| 2 | 11249ns | +3.4% | +15.2% | +0.9% | -11.5% | +53.2% | -26.2% | +10.5% |
| 3 | 11185ns | +4.0% | +18.6% | +4.0% | +5.5% | +55.2% | -25.5% | +12.9% |
| 4 | 11089ns | +5.0% | +17.0% | +2.4% | +3.4% | +55.0% | -25.9% | +12.3% |
| 5 | 11145ns | +5.8% | +17.0% | +2.5% | +0.4% | +53.7% | -25.2% | +13.8% |
| 6 | 10918ns | +2.5% | +20.3% | +4.2% | +8.2% | +57.1% | -24.6% | +14.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.304 | moderate- |
| carrier_disp_tight_fntable | -0.601 | HIGH- (thermal bounce) |
| carrier_disp_tight_ifchain | -0.474 | moderate- |
| carrier_disp_tight_ifchainasc | 0.172 | ok |
| carrier_disp_tight_ifchainlin | 0.167 | ok |
| carrier_disp_tight_nullfloor | -0.519 | HIGH- (thermal bounce) |
| carrier_disp_tight_switch | 0.009 | ok |
| carrier_disp_tight_threaded | -0.825 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 0/6, lost 6/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 91332.5ns | 11579.4ns | 788.8% | HIGH |
| carrier_disp_tight_fntable | 90796.8ns | 13070.9ns | 694.6% | HIGH |
| carrier_disp_tight_ifchain | 91517.2ns | 11421.2ns | 801.3% | HIGH |
| carrier_disp_tight_ifchainasc | 91232.1ns | 11074.3ns | 823.8% | HIGH |
| carrier_disp_tight_ifchainlin | 98413.8ns | 17206.6ns | 572.0% | HIGH |
| carrier_disp_tight_nullfloor | 90245.3ns | 8294.0ns | 1088.1% | HIGH |
| carrier_disp_tight_switch | 89644.8ns | 11116.8ns | 806.4% | HIGH |
| carrier_disp_tight_threaded | 88008.5ns | 12548.0ns | 701.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 11186.7-11718.4 ns)
  11186.7 |####################
  11213.3 |
  11239.9 |
  11266.4 |
  11293.0 |
  11319.6 |
  11346.2 |
  11372.8 |
  11399.4 |
  11425.9 |
  11452.5 |
  11479.1 |
  11505.7 |
  11532.3 |
  11558.9 |
  11585.4 |####################
  11612.0 |########################################
  11638.6 |####################
  11665.2 |
  11691.8 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 12955.4-13198.1 ns)
  12955.4 |########################################
  12967.5 |########################################
  12979.7 |
  12991.8 |
  13003.9 |
  13016.1 |
  13028.2 |
  13040.3 |########################################
  13052.5 |########################################
  13064.6 |
  13076.8 |
  13088.9 |
  13101.0 |
  13113.2 |
  13125.3 |########################################
  13137.4 |
  13149.6 |
  13161.7 |
  13173.8 |
  13186.0 |
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 11347.5-11527.5 ns)
  11347.5 |########################################
  11356.5 |########################################
  11365.5 |
  11374.5 |########################################
  11383.5 |########################################
  11392.5 |
  11401.5 |
  11410.5 |
  11419.5 |########################################
  11428.5 |
  11437.5 |
  11446.5 |
  11455.5 |
  11464.5 |
  11473.5 |
  11482.5 |
  11491.5 |
  11500.5 |
  11509.5 |
  11518.5 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 9960.4-11807.3 ns)
   9960.4 |########################################
  10052.7 |
  10145.1 |########################################
  10237.4 |
  10329.8 |
  10422.1 |
  10514.5 |
  10606.8 |
  10699.2 |
  10791.5 |
  10883.8 |
  10976.2 |
  11068.5 |
  11160.9 |########################################
  11253.2 |
  11345.6 |
  11437.9 |########################################
  11530.3 |
  11622.6 |
  11715.0 |########################################
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 17128.7-17296.4 ns)
  17128.7 |####################
  17137.1 |
  17145.5 |####################
  17153.9 |
  17162.2 |
  17170.6 |
  17179.0 |########################################
  17187.4 |
  17195.8 |
  17204.2 |
  17212.6 |
  17221.0 |
  17229.3 |####################
  17237.7 |
  17246.1 |
  17254.5 |
  17262.9 |
  17271.3 |
  17279.7 |
  17288.1 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 8220.8-8341.6 ns)
   8220.8 |####################
   8226.8 |####################
   8232.9 |
   8238.9 |
   8245.0 |
   8251.0 |
   8257.1 |
   8263.1 |
   8269.1 |
   8275.2 |
   8281.2 |
   8287.3 |
   8293.3 |####################
   8299.4 |
   8305.4 |
   8311.4 |
   8317.5 |
   8323.5 |
   8329.6 |########################################
   8335.6 |
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 10918.3-11217.3 ns)
  10918.3 |########################################
  10933.2 |
  10948.2 |
  10963.1 |
  10978.1 |
  10993.0 |
  11008.0 |
  11022.9 |
  11037.9 |
  11052.8 |
  11067.8 |
  11082.8 |########################################
  11097.7 |
  11112.6 |########################################
  11127.6 |
  11142.5 |########################################
  11157.5 |
  11172.4 |########################################
  11187.4 |
  11202.3 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 12425.8-12656.8 ns)
  12425.8 |########################################
  12437.4 |
  12448.9 |########################################
  12460.5 |
  12472.0 |########################################
  12483.6 |
  12495.1 |
  12506.7 |
  12518.2 |
  12529.8 |
  12541.3 |
  12552.9 |
  12564.4 |
  12576.0 |
  12587.5 |
  12599.1 |
  12610.6 |########################################
  12622.2 |########################################
  12633.7 |
  12645.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=787.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=698.8% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=801.0% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=809.8% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=570.8% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=1084.7% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=806.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=701.9% of algo (FFI overhead may distort results)
