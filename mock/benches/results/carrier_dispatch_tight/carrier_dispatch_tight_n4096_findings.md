# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 36% faster than the next best (carrier_disp_tight_bittree)

carrier_disp_tight_nullfloor (128.31 us) leads carrier_disp_tight_bittree (173.94 us) by 36%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 32% (significant)

carrier_disp_tight_nullfloor is -59.15 us (32%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_switch shows alternating (throttle bounce) (autocorr -0.58)

carrier_disp_tight_switch's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_bittree, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} (36% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_bittree, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_threaded, carrier_disp_tight_fntable, carrier_disp_tight_ifchainlin} with a 36% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 128306.2 ns median (-31.2% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.84x (fastest 128306.2 ns, slowest 236339.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 176958ns | 176712ns | 175593ns | 176567ns | 178227ns | -6.80% |
| carrier_disp_tight_fntable | 220032ns | 220241ns | 218880ns | 219860ns | 220866ns | +15.89% |
| carrier_disp_tight_ifchain | 192176ns | 192071ns | 187772ns | 191339ns | 195632ns | +1.22% |
| carrier_disp_tight_ifchainasc | 188337ns | 188323ns | 187452ns | 188139ns | 189077ns | -0.80% |
| carrier_disp_tight_ifchainlin | 239783ns | 239157ns | 238454ns | 239021ns | 241589ns | +26.29% |
| carrier_disp_tight_nullfloor | 130204ns | 130727ns | 125175ns | 130506ns | 132266ns | -31.42% |
| carrier_disp_tight_switch | 189859ns | 189527ns | 188420ns | 189208ns | 191557ns | base |
| carrier_disp_tight_threaded | 198215ns | 198252ns | 196296ns | 198012ns | 199478ns | +4.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 174290ns | 173089ns | 175568ns | -6.88% | 0.024 |
| carrier_disp_tight_fntable | 217315ns | 215951ns | 218272ns | +16.11% | 0.019 |
| carrier_disp_tight_ifchain | 189352ns | 184908ns | 192784ns | +1.17% | 0.022 |
| carrier_disp_tight_ifchainasc | 185530ns | 184772ns | 186222ns | -0.87% | 0.022 |
| carrier_disp_tight_ifchainlin | 236871ns | 235427ns | 238655ns | +26.56% | 0.017 |
| carrier_disp_tight_nullfloor | 127692ns | 123018ns | 129518ns | -31.77% | 0.032 |
| carrier_disp_tight_switch | 187157ns | 185684ns | 189016ns | base | 0.022 |
| carrier_disp_tight_threaded | 195524ns | 193622ns | 196625ns | +4.47% | 0.021 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.024 | 70.7% |
| carrier_disp_tight_fntable | 0.019 | 56.6% |
| carrier_disp_tight_ifchain | 0.022 | 65.0% |
| carrier_disp_tight_ifchainasc | 0.022 | 66.3% |
| carrier_disp_tight_ifchainlin | 0.017 | 52.1% |
| carrier_disp_tight_nullfloor | 0.032 | 95.9% |
| carrier_disp_tight_switch | 0.022 | 65.9% |
| carrier_disp_tight_threaded | 0.021 | 62.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 176958ns | 176958ns | -6.80% |
| carrier_disp_tight_fntable | 220032ns | 220032ns | +15.89% |
| carrier_disp_tight_ifchain | 192176ns | 192176ns | +1.22% |
| carrier_disp_tight_ifchainasc | 188337ns | 188337ns | -0.80% |
| carrier_disp_tight_ifchainlin | 239783ns | 239783ns | +26.29% |
| carrier_disp_tight_nullfloor | 130204ns | 130204ns | -31.42% |
| carrier_disp_tight_switch | 189859ns | 189859ns | base |
| carrier_disp_tight_threaded | 198215ns | 198215ns | +4.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 186618ns | base | --- | [185838, 189016] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 173938ns | -12604.2ns (-6.8%) | [-14577, -11422]ns | [173362, 175568] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 217285ns | +30227.5ns (+16.2%) | [+27811, +32434]ns | [216388, 218272] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 189231ns | no significant difference | [-2169, +6166]ns | [186042, 192784] | no | 0.2188 | 0.2188 | 0 |
| carrier_disp_tight_ifchainasc | 185466ns | -1446.1ns (-0.8%) | [-3189, -248]ns | [184901, 186222] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_ifchainlin | 236339ns | +49680.9ns (+26.6%) | [+47794, +51666]ns | [235618, 238655] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 128306ns | -59147.5ns (-31.7%) | [-62678, -56569]ns | [125254, 129518] | YES | 0.0365 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 195626ns | +9048.8ns (+4.8%) | [+6390, +9661]ns | [194321, 196625] | YES | 0.0365 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 186182ns | -7.0% | +16.5% | +3.2% | -0.8% | +26.8% | -30.8% | +4.9% |
| 2 | 189736ns | -8.5% | +13.8% | -2.5% | -2.3% | +24.7% | -32.3% | +2.0% |
| 3 | 185991ns | -6.6% | +17.5% | +0.6% | -0.2% | +26.6% | -30.0% | +5.4% |
| 4 | 187054ns | -6.9% | +15.9% | +3.4% | -1.1% | +26.1% | -34.2% | +4.8% |
| 5 | 188295ns | -6.5% | +15.6% | +0.3% | -0.8% | +26.7% | -31.9% | +4.8% |
| 6 | 185684ns | -5.7% | +17.4% | +2.2% | -0.0% | +28.6% | -31.3% | +5.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | 0.391 | moderate+ |
| carrier_disp_tight_fntable | -0.314 | moderate- |
| carrier_disp_tight_ifchain | -0.290 | moderate- |
| carrier_disp_tight_ifchainasc | -0.165 | ok |
| carrier_disp_tight_ifchainlin | 0.348 | moderate+ |
| carrier_disp_tight_nullfloor | -0.385 | moderate- |
| carrier_disp_tight_switch | -0.578 | HIGH- (thermal bounce) |
| carrier_disp_tight_threaded | -0.053 | ok |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 6/6, lost 0/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 1/6, lost 5/6
- **carrier_disp_tight_ifchainasc**: won 5/6, lost 0/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 175150.8ns | 174289.6ns | 100.5% | HIGH |
| carrier_disp_tight_fntable | 222074.9ns | 217314.9ns | 102.2% | HIGH |
| carrier_disp_tight_ifchain | 189985.2ns | 189352.2ns | 100.3% | HIGH |
| carrier_disp_tight_ifchainasc | 186038.9ns | 185529.6ns | 100.3% | HIGH |
| carrier_disp_tight_ifchainlin | 237695.3ns | 236870.9ns | 100.3% | HIGH |
| carrier_disp_tight_nullfloor | 128367.1ns | 127692.5ns | 100.5% | HIGH |
| carrier_disp_tight_switch | 187927.1ns | 187157.2ns | 100.4% | HIGH |
| carrier_disp_tight_threaded | 196357.9ns | 195523.8ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 173089.2-175568.1 ns)
  173089.2 |####################
  173213.1 |
  173337.1 |
  173461.0 |
  173585.0 |########################################
  173708.9 |
  173832.9 |
  173956.8 |
  174080.8 |####################
  174204.7 |
  174328.7 |
  174452.6 |
  174576.5 |
  174700.5 |
  174824.4 |
  174948.4 |####################
  175072.3 |
  175196.3 |
  175320.2 |
  175444.2 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 215951.2-218271.9 ns)
  215951.2 |####################
  216067.2 |
  216183.3 |
  216299.3 |
  216415.3 |
  216531.4 |
  216647.4 |
  216763.4 |########################################
  216879.5 |
  216995.5 |
  217111.5 |
  217227.6 |
  217343.6 |
  217459.7 |
  217575.7 |
  217691.7 |####################
  217807.8 |
  217923.8 |####################
  218039.8 |
  218155.9 |
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 184908.3-192784.4 ns)
  184908.3 |########################################
  185302.1 |
  185695.9 |
  186089.7 |
  186483.5 |
  186877.3 |########################################
  187271.1 |
  187664.9 |
  188058.7 |
  188452.5 |########################################
  188846.3 |
  189240.2 |
  189634.0 |########################################
  190027.8 |
  190421.6 |
  190815.4 |
  191209.2 |
  191603.0 |
  191996.8 |########################################
  192390.6 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 184772.1-186221.6 ns)
  184772.1 |########################################
  184844.6 |
  184917.1 |
  184989.5 |########################################
  185062.0 |
  185134.5 |
  185207.0 |
  185279.4 |
  185351.9 |########################################
  185424.4 |
  185496.9 |########################################
  185569.4 |########################################
  185641.8 |
  185714.3 |
  185786.8 |
  185859.3 |
  185931.7 |
  186004.2 |
  186076.7 |
  186149.2 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 235426.7-238655.4 ns)
  235426.7 |########################################
  235588.1 |
  235749.6 |########################################
  235911.0 |
  236072.4 |########################################
  236233.9 |
  236395.3 |
  236556.7 |########################################
  236718.2 |
  236879.6 |
  237041.0 |
  237202.5 |
  237363.9 |
  237525.4 |
  237686.8 |
  237848.2 |
  238009.7 |
  238171.1 |
  238332.5 |
  238494.0 |########################################
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 123018.3-129517.7 ns)
  123018.3 |########################################
  123343.3 |
  123668.2 |
  123993.2 |
  124318.2 |
  124643.1 |
  124968.1 |
  125293.1 |
  125618.1 |
  125943.0 |
  126268.0 |
  126593.0 |
  126917.9 |
  127242.9 |########################################
  127567.9 |
  127892.9 |########################################
  128217.8 |########################################
  128542.8 |########################################
  128867.8 |
  129192.7 |
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 185684.2-189015.6 ns)
  185684.2 |########################################
  185850.8 |########################################
  186017.3 |########################################
  186183.9 |
  186350.5 |
  186517.1 |
  186683.6 |
  186850.2 |
  187016.8 |########################################
  187183.3 |
  187349.9 |
  187516.5 |
  187683.0 |
  187849.6 |
  188016.2 |
  188182.8 |########################################
  188349.3 |
  188515.9 |
  188682.5 |
  188849.0 |
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 193622.1-196624.5 ns)
  193622.1 |####################
  193772.2 |
  193922.3 |
  194072.5 |
  194222.6 |
  194372.7 |
  194522.8 |
  194673.0 |
  194823.1 |
  194973.2 |####################
  195123.3 |
  195273.4 |####################
  195423.6 |
  195573.7 |
  195723.8 |
  195873.9 |########################################
  196024.1 |
  196174.2 |
  196324.3 |
  196474.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=102.1% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=100.4% of algo (FFI overhead may distort results)
