# Dispatch shape over the wire form, tight profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_tight_nullfloor dominates: 46% faster than the next best (carrier_disp_tight_bittree)

carrier_disp_tight_nullfloor (499.46 us) leads carrier_disp_tight_bittree (729.08 us) by 46%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_tight_nullfloor beats baseline by 40% (significant)

carrier_disp_tight_nullfloor is -333.04 us (40%) faster than baseline carrier_disp_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_tight_switch shows alternating (throttle bounce) (autocorr -0.59)

carrier_disp_tight_switch's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_tight_nullfloor} vs {carrier_disp_tight_bittree, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_fntable, carrier_disp_tight_threaded, carrier_disp_tight_ifchainlin} (46% apart)

The field splits into a fast tier {carrier_disp_tight_nullfloor} and a slow tier {carrier_disp_tight_bittree, carrier_disp_tight_ifchainasc, carrier_disp_tight_switch, carrier_disp_tight_ifchain, carrier_disp_tight_fntable, carrier_disp_tight_threaded, carrier_disp_tight_ifchainlin} with a 46% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_disp_tight_nullfloor** at 499456.1 ns median (-40.0% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.96x (fastest 499456.1 ns, slowest 977700.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 733607ns | 732852ns | 730410ns | 732081ns | 737495ns | -12.22% |
| carrier_disp_tight_fntable | 943792ns | 943509ns | 943080ns | 943472ns | 944627ns | +12.93% |
| carrier_disp_tight_ifchain | 842215ns | 844296ns | 833155ns | 843455ns | 844884ns | +0.78% |
| carrier_disp_tight_ifchainasc | 835545ns | 835332ns | 832099ns | 835116ns | 837911ns | -0.02% |
| carrier_disp_tight_ifchainlin | 982576ns | 981567ns | 977599ns | 980365ns | 988380ns | +17.57% |
| carrier_disp_tight_nullfloor | 502084ns | 502727ns | 497591ns | 502685ns | 503430ns | -39.92% |
| carrier_disp_tight_switch | 835704ns | 836393ns | 831270ns | 834964ns | 839030ns | base |
| carrier_disp_tight_threaded | 959541ns | 959011ns | 954250ns | 958211ns | 964181ns | +14.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_tight_bittree | 729925ns | 726512ns | 734146ns | -12.26% | 0.022 |
| carrier_disp_tight_fntable | 939860ns | 939028ns | 940748ns | +12.97% | 0.017 |
| carrier_disp_tight_ifchain | 838505ns | 829498ns | 841320ns | +0.79% | 0.020 |
| carrier_disp_tight_ifchainasc | 831677ns | 828988ns | 833950ns | -0.03% | 0.020 |
| carrier_disp_tight_ifchainlin | 978690ns | 973508ns | 984528ns | +17.64% | 0.017 |
| carrier_disp_tight_nullfloor | 498683ns | 494323ns | 499830ns | -40.06% | 0.033 |
| carrier_disp_tight_switch | 831927ns | 827165ns | 835596ns | base | 0.020 |
| carrier_disp_tight_threaded | 955737ns | 950484ns | 960291ns | +14.88% | 0.017 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_disp_tight_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_tight_bittree | 0.022 | 67.8% |
| carrier_disp_tight_fntable | 0.017 | 52.6% |
| carrier_disp_tight_ifchain | 0.019 | 58.8% |
| carrier_disp_tight_ifchainasc | 0.020 | 59.5% |
| carrier_disp_tight_ifchainlin | 0.017 | 50.6% |
| carrier_disp_tight_nullfloor | 0.033 | 99.0% |
| carrier_disp_tight_switch | 0.020 | 59.4% |
| carrier_disp_tight_threaded | 0.017 | 51.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_tight_bittree | 733607ns | 733607ns | -12.22% |
| carrier_disp_tight_fntable | 943792ns | 943792ns | +12.93% |
| carrier_disp_tight_ifchain | 842215ns | 842215ns | +0.78% |
| carrier_disp_tight_ifchainasc | 835545ns | 835545ns | -0.02% |
| carrier_disp_tight_ifchainlin | 982576ns | 982576ns | +17.57% |
| carrier_disp_tight_nullfloor | 502084ns | 502084ns | -39.92% |
| carrier_disp_tight_switch | 835704ns | 835704ns | base |
| carrier_disp_tight_threaded | 959541ns | 959541ns | +14.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_tight_switch | 832493ns | base | --- | [827693, 835596] | --- | --- | --- | --- |
| carrier_disp_tight_bittree | 729083ns | -103034.5ns (-12.4%) | [-107141, -95832]ns | [726545, 734146] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_fntable | 939699ns | +106639.0ns (+12.8%) | [+105152, +112006]ns | [939132, 940748] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_ifchain | 840560ns | no significant difference | [-507, +11412]ns | [833635, 841320] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_tight_ifchainasc | 831259ns | no significant difference | [-4729, +5290]ns | [829822, 833950] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_tight_ifchainlin | 977701ns | +144723.1ns (+17.4%) | [+141243, +154320]ns | [973840, 984528] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_nullfloor | 499456ns | -333035.8ns (-40.0%) | [-335959, -330738]ns | [496763, 499830] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_tight_threaded | 955200ns | +122868.0ns (+14.8%) | [+121073, +127487]ns | [951720, 960291] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_tight_switch | carrier_disp_tight_bittree | carrier_disp_tight_fntable | carrier_disp_tight_ifchain | carrier_disp_tight_ifchainasc | carrier_disp_tight_ifchainlin | carrier_disp_tight_nullfloor | carrier_disp_tight_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 831736ns | -11.4% | +12.9% | +1.1% | -0.0% | +17.0% | -40.0% | +14.8% |
| 2 | 833250ns | -12.8% | +12.7% | +1.0% | -0.3% | +18.3% | -40.0% | +14.6% |
| 3 | 835852ns | -12.9% | +12.6% | +0.6% | -0.8% | +16.8% | -40.2% | +15.5% |
| 4 | 828221ns | -11.7% | +13.5% | +1.2% | +0.3% | +17.6% | -39.7% | +14.8% |
| 5 | 835341ns | -12.6% | +12.6% | -0.7% | -0.3% | +17.2% | -40.2% | +14.4% |
| 6 | 827165ns | -12.2% | +13.6% | +1.6% | +0.9% | +18.9% | -40.2% | +15.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_tight_bittree | -0.264 | moderate- |
| carrier_disp_tight_fntable | -0.154 | ok |
| carrier_disp_tight_ifchain | 0.020 | ok |
| carrier_disp_tight_ifchainasc | 0.393 | moderate+ |
| carrier_disp_tight_ifchainlin | -0.347 | moderate- |
| carrier_disp_tight_nullfloor | -0.046 | ok |
| carrier_disp_tight_switch | -0.590 | HIGH- (thermal bounce) |
| carrier_disp_tight_threaded | -0.426 | moderate- |

**Consistency summary:**

- **carrier_disp_tight_bittree**: won 6/6, lost 0/6
- **carrier_disp_tight_fntable**: won 0/6, lost 6/6
- **carrier_disp_tight_ifchain**: won 1/6, lost 5/6
- **carrier_disp_tight_ifchainasc**: won 3/6, lost 2/6
- **carrier_disp_tight_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_tight_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_tight_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_tight_bittree | 731491.0ns | 729924.8ns | 100.2% | HIGH |
| carrier_disp_tight_fntable | 951876.5ns | 939859.7ns | 101.3% | HIGH |
| carrier_disp_tight_ifchain | 841102.6ns | 838504.8ns | 100.3% | HIGH |
| carrier_disp_tight_ifchainasc | 834403.6ns | 831677.0ns | 100.3% | HIGH |
| carrier_disp_tight_ifchainlin | 981444.7ns | 978689.5ns | 100.3% | HIGH |
| carrier_disp_tight_nullfloor | 501208.7ns | 498683.1ns | 100.5% | HIGH |
| carrier_disp_tight_switch | 834921.0ns | 831927.4ns | 100.4% | HIGH |
| carrier_disp_tight_threaded | 957801.9ns | 955736.8ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_tight_bittree (n=6, range 726512.5-734146.2 ns)
  726512.5 |########################################
  726894.2 |
  727275.9 |
  727657.6 |
  728039.2 |####################
  728420.9 |
  728802.6 |
  729184.3 |
  729566.0 |####################
  729947.7 |
  730329.4 |
  730711.1 |
  731092.8 |####################
  731474.4 |
  731856.1 |
  732237.8 |
  732619.5 |
  733001.2 |
  733382.9 |
  733764.6 |
  (0 below, 1 above range)

carrier_disp_tight_fntable (n=6, range 939028.3-940748.3 ns)
  939028.3 |########################################
  939114.3 |
  939200.3 |########################################
  939286.3 |
  939372.3 |
  939458.3 |
  939544.3 |########################################
  939630.3 |
  939716.3 |
  939802.3 |########################################
  939888.3 |
  939974.3 |
  940060.3 |
  940146.3 |
  940232.3 |########################################
  940318.3 |
  940404.3 |
  940490.3 |
  940576.3 |
  940662.3 |
  (0 below, 1 above range)

carrier_disp_tight_ifchain (n=6, range 829497.5-841320.4 ns)
  829497.5 |####################
  830088.6 |
  830679.8 |
  831270.9 |
  831862.1 |
  832453.2 |
  833044.4 |
  833635.5 |
  834226.7 |
  834817.8 |
  835408.9 |
  836000.1 |
  836591.2 |
  837182.4 |####################
  837773.5 |
  838364.7 |
  838955.8 |
  839547.0 |
  840138.1 |########################################
  840729.3 |####################
  (0 below, 1 above range)

carrier_disp_tight_ifchainasc (n=6, range 828988.3-833950.2 ns)
  828988.3 |########################################
  829236.4 |
  829484.5 |
  829732.6 |
  829980.7 |
  830228.8 |
  830476.9 |########################################
  830725.0 |
  830973.1 |########################################
  831221.2 |########################################
  831469.2 |
  831717.3 |
  831965.4 |
  832213.5 |
  832461.6 |
  832709.7 |
  832957.8 |########################################
  833205.9 |
  833454.0 |
  833702.1 |
  (0 below, 1 above range)

carrier_disp_tight_ifchainlin (n=6, range 973507.9-984527.7 ns)
  973507.9 |########################################
  974058.9 |########################################
  974609.9 |
  975160.9 |
  975711.9 |
  976262.8 |########################################
  976813.8 |
  977364.8 |
  977915.8 |
  978466.8 |########################################
  979017.8 |
  979568.8 |
  980119.8 |
  980670.8 |
  981221.8 |
  981772.8 |
  982323.7 |
  982874.7 |########################################
  983425.7 |
  983976.7 |
  (0 below, 1 above range)

carrier_disp_tight_nullfloor (n=6, range 494322.9-499830.0 ns)
  494322.9 |####################
  494598.3 |
  494873.6 |
  495149.0 |
  495424.3 |
  495699.7 |
  495975.0 |
  496250.4 |
  496525.7 |
  496801.1 |
  497076.5 |
  497351.8 |
  497627.2 |
  497902.5 |
  498177.9 |
  498453.2 |
  498728.6 |
  499003.9 |########################################
  499279.3 |
  499554.6 |########################################
  (0 below, 1 above range)

carrier_disp_tight_switch (n=6, range 827164.6-835596.4 ns)
  827164.6 |########################################
  827586.2 |
  828007.8 |########################################
  828429.4 |
  828851.0 |
  829272.6 |
  829694.2 |
  830115.7 |
  830537.3 |
  830958.9 |
  831380.5 |########################################
  831802.1 |
  832223.7 |
  832645.3 |
  833066.9 |########################################
  833488.5 |
  833910.1 |
  834331.7 |
  834753.3 |
  835174.9 |########################################
  (0 below, 1 above range)

carrier_disp_tight_threaded (n=6, range 950483.8-960291.1 ns)
  950483.8 |####################
  950974.2 |
  951464.5 |
  951954.9 |
  952445.2 |
  952935.6 |####################
  953426.0 |
  953916.3 |
  954406.7 |
  954897.1 |########################################
  955387.4 |####################
  955877.8 |
  956368.2 |
  956858.5 |
  957348.9 |
  957839.2 |
  958329.6 |
  958820.0 |
  959310.3 |
  959800.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_tight_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_tight_fntable**: bridge=101.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchain**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainasc**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_ifchainlin**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_nullfloor**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_tight_switch**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_tight_threaded**: bridge=100.1% of algo (FFI overhead may distort results)
