# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 125% faster than the next best (carrier_disp_real_bittree)

carrier_disp_real_nullfloor (115.98 us) leads carrier_disp_real_bittree (260.44 us) by 125%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 80% (significant)

carrier_disp_real_nullfloor is -487.87 us (80%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_threaded is an outlier: 5.6x slower than the field

carrier_disp_real_threaded (652.39 us) is 5.6x the fastest (115.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_ifchainlin shows alternating (throttle bounce) (autocorr -0.61)

carrier_disp_real_ifchainlin's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor} vs {carrier_disp_real_bittree, carrier_disp_real_ifchainlin, carrier_disp_real_fntable, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_threaded} (125% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor} and a slow tier {carrier_disp_real_bittree, carrier_disp_real_ifchainlin, carrier_disp_real_fntable, carrier_disp_real_ifchain, carrier_disp_real_ifchainasc, carrier_disp_real_switch, carrier_disp_real_threaded} with a 125% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest carrier_disp_real_nullfloor (115.98 us) to slowest carrier_disp_real_threaded (652.39 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 115975.2 ns median (-80.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.63x (fastest 115975.2 ns, slowest 652392.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 264885ns | 263411ns | 222272ns | 252617ns | 304595ns | -55.28% |
| carrier_disp_real_fntable | 565300ns | 567421ns | 545106ns | 566957ns | 572913ns | -4.57% |
| carrier_disp_real_ifchain | 591626ns | 600654ns | 528150ns | 586172ns | 631544ns | -0.13% |
| carrier_disp_real_ifchainasc | 601495ns | 607385ns | 576415ns | 600148ns | 616057ns | +1.54% |
| carrier_disp_real_ifchainlin | 548652ns | 542411ns | 497935ns | 527915ns | 605116ns | -7.38% |
| carrier_disp_real_nullfloor | 119476ns | 118496ns | 117423ns | 118196ns | 122424ns | -79.83% |
| carrier_disp_real_switch | 592376ns | 611329ns | 533228ns | 590531ns | 624717ns | base |
| carrier_disp_real_threaded | 642694ns | 655667ns | 597855ns | 636700ns | 674104ns | +8.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 261891ns | 219251ns | 301372ns | -55.53% | 0.016 |
| carrier_disp_real_fntable | 562281ns | 542809ns | 569611ns | -4.52% | 0.007 |
| carrier_disp_real_ifchain | 588545ns | 525555ns | 628311ns | -0.06% | 0.007 |
| carrier_disp_real_ifchainasc | 598435ns | 573758ns | 613074ns | +1.62% | 0.007 |
| carrier_disp_real_ifchainlin | 545474ns | 495113ns | 601480ns | -7.37% | 0.008 |
| carrier_disp_real_nullfloor | 116982ns | 115005ns | 119948ns | -80.14% | 0.035 |
| carrier_disp_real_switch | 588903ns | 529746ns | 621398ns | base | 0.007 |
| carrier_disp_real_threaded | 639492ns | 594397ns | 670736ns | +8.59% | 0.006 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.016 | 44.2% |
| carrier_disp_real_fntable | 0.007 | 20.4% |
| carrier_disp_real_ifchain | 0.007 | 19.3% |
| carrier_disp_real_ifchainasc | 0.007 | 19.0% |
| carrier_disp_real_ifchainlin | 0.008 | 21.3% |
| carrier_disp_real_nullfloor | 0.035 | 99.2% |
| carrier_disp_real_switch | 0.007 | 18.9% |
| carrier_disp_real_threaded | 0.006 | 17.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 264885ns | 264885ns | -55.28% |
| carrier_disp_real_fntable | 565300ns | 565300ns | -4.57% |
| carrier_disp_real_ifchain | 591626ns | 591626ns | -0.13% |
| carrier_disp_real_ifchainasc | 601495ns | 601495ns | +1.54% |
| carrier_disp_real_ifchainlin | 548652ns | 548652ns | -7.38% |
| carrier_disp_real_nullfloor | 119476ns | 119476ns | -79.83% |
| carrier_disp_real_switch | 592376ns | 592376ns | base |
| carrier_disp_real_threaded | 642694ns | 642694ns | +8.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 607815ns | base | --- | [537497, 621398] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 260445ns | -311372.9ns (-51.2%) | [-374690, -294974]ns | [223855, 301372] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_real_fntable | 564521ns | no significant difference | [-57411, +19272]ns | [552712, 569611] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_real_ifchain | 597226ns | no significant difference | [-46336, +32516]ns | [540098, 628311] | no | 0.3063 | 0.2188 | 0 |
| carrier_disp_real_ifchainasc | 603844ns | no significant difference | [-26489, +57839]ns | [578389, 613074] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchainlin | 539457ns | no significant difference | [-101455, +22343]ns | [495485, 601480] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_real_nullfloor | 115975ns | -487866.7ns (-80.3%) | [-506065, -421832]ns | [115023, 119948] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_real_threaded | 652393ns | +49408.5ns (+8.1%) | [+7160, +95197]ns | [595346, 670736] | YES (adj: no) | 0.3063 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 545248ns | -51.9% | -0.4% | +1.7% | +5.2% | +0.5% | -78.7% | +9.4% |
| 2 | 602605ns | -51.8% | -6.5% | +4.1% | +1.1% | -11.2% | -79.7% | +8.3% |
| 3 | 529746ns | -58.6% | +7.7% | +7.6% | +16.5% | -6.5% | -78.3% | +26.3% |
| 4 | 613025ns | -57.8% | -7.3% | +2.6% | -2.0% | +6.9% | -80.9% | -3.0% |
| 5 | 619312ns | -49.6% | -9.2% | +0.8% | -2.0% | -19.9% | -81.4% | +5.3% |
| 6 | 623483ns | -63.4% | -9.3% | -15.7% | -6.5% | -12.7% | -81.5% | +7.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.463 | moderate- |
| carrier_disp_real_fntable | 0.080 | ok |
| carrier_disp_real_ifchain | -0.364 | moderate- |
| carrier_disp_real_ifchainasc | -0.105 | ok |
| carrier_disp_real_ifchainlin | -0.605 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | -0.326 | moderate- |
| carrier_disp_real_switch | -0.127 | ok |
| carrier_disp_real_threaded | -0.269 | moderate- |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 6/6, lost 0/6
- **carrier_disp_real_fntable**: won 5/6, lost 1/6
- **carrier_disp_real_ifchain**: won 1/6, lost 5/6
- **carrier_disp_real_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_real_ifchainlin**: won 4/6, lost 2/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 263038.7ns | 261890.9ns | 100.4% | HIGH |
| carrier_disp_real_fntable | 561297.5ns | 562281.3ns | 99.8% | HIGH |
| carrier_disp_real_ifchain | 589285.6ns | 588544.9ns | 100.1% | HIGH |
| carrier_disp_real_ifchainasc | 598697.5ns | 598435.3ns | 100.0% | HIGH |
| carrier_disp_real_ifchainlin | 547125.9ns | 545474.0ns | 100.3% | HIGH |
| carrier_disp_real_nullfloor | 117250.5ns | 116982.1ns | 100.2% | HIGH |
| carrier_disp_real_switch | 589564.2ns | 588903.1ns | 100.1% | HIGH |
| carrier_disp_real_threaded | 640104.9ns | 639491.6ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 219250.8-301372.5 ns)
  219250.8 |########################################
  223356.9 |
  227463.0 |########################################
  231569.1 |
  235675.1 |
  239781.2 |
  243887.3 |
  247993.4 |
  252099.5 |
  256205.6 |########################################
  260311.6 |########################################
  264417.7 |
  268523.8 |
  272629.9 |
  276736.0 |
  280842.1 |
  284948.2 |
  289054.2 |########################################
  293160.3 |
  297266.4 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 542809.2-569611.1 ns)
  542809.2 |########################################
  544149.3 |
  545489.4 |
  546829.5 |
  548169.6 |
  549509.7 |
  550849.8 |
  552189.8 |
  553529.9 |
  554870.0 |
  556210.1 |
  557550.2 |
  558890.3 |
  560230.4 |
  561570.5 |########################################
  562910.6 |########################################
  564250.7 |########################################
  565590.8 |
  566930.9 |
  568271.0 |########################################
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 525555.0-628310.8 ns)
  525555.0 |####################
  530692.8 |
  535830.6 |
  540968.4 |
  546106.2 |
  551243.9 |####################
  556381.7 |
  561519.5 |
  566657.3 |####################
  571795.1 |
  576932.9 |
  582070.7 |
  587208.5 |
  592346.3 |
  597484.1 |
  602621.9 |
  607759.6 |
  612897.4 |
  618035.2 |
  623173.0 |########################################
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 573757.5-613073.8 ns)
  573757.5 |########################################
  575723.3 |
  577689.1 |
  579654.9 |
  581620.8 |########################################
  583586.6 |
  585552.4 |
  587518.2 |
  589484.0 |
  591449.8 |
  593415.6 |
  595381.4 |
  597347.2 |
  599313.1 |########################################
  601278.9 |
  603244.7 |
  605210.5 |
  607176.3 |########################################
  609142.1 |########################################
  611107.9 |
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 495112.9-601479.6 ns)
  495112.9 |########################################
  500431.2 |
  505749.6 |
  511067.9 |
  516386.2 |
  521704.6 |
  527022.9 |
  532341.2 |####################
  537659.6 |
  542977.9 |########################################
  548296.2 |
  553614.6 |
  558932.9 |
  564251.2 |
  569569.6 |
  574887.9 |
  580206.2 |
  585524.6 |
  590842.9 |
  596161.2 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 115004.6-119948.4 ns)
  115004.6 |########################################
  115251.8 |
  115499.0 |####################
  115746.2 |
  115993.4 |
  116240.5 |####################
  116487.7 |
  116734.9 |
  116982.1 |
  117229.3 |####################
  117476.5 |
  117723.7 |
  117970.9 |
  118218.0 |
  118465.2 |
  118712.4 |
  118959.6 |
  119206.8 |
  119454.0 |
  119701.2 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 529746.2-621397.5 ns)
  529746.2 |########################################
  534328.8 |
  538911.3 |
  543493.9 |########################################
  548076.5 |
  552659.0 |
  557241.6 |
  561824.2 |
  566406.7 |
  570989.3 |
  575571.8 |
  580154.4 |
  584737.0 |
  589319.5 |
  593902.1 |
  598484.7 |########################################
  603067.2 |
  607649.8 |
  612232.4 |########################################
  616814.9 |########################################
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 594397.1-670736.2 ns)
  594397.1 |########################################
  598214.1 |
  602031.0 |
  605848.0 |
  609664.9 |
  613481.9 |
  617298.8 |
  621115.8 |
  624932.8 |
  628749.7 |
  632566.7 |
  636383.6 |
  640200.6 |
  644017.5 |
  647834.5 |
  651651.5 |########################################
  655468.4 |
  659285.4 |
  663102.3 |
  666919.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=99.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
