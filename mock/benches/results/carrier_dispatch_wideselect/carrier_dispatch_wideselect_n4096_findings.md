# Dispatch shape over the wire form, wideselect profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_wideselect_nullfloor dominates: 67% faster than the next best (carrier_disp_wideselect_bittree)

carrier_disp_wideselect_nullfloor (119.28 us) leads carrier_disp_wideselect_bittree (199.68 us) by 67%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_wideselect_nullfloor beats baseline by 75% (significant)

carrier_disp_wideselect_nullfloor is -358.49 us (75%) faster than baseline carrier_disp_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_wideselect_ifchainlin is an outlier: 4.3x slower than the field

carrier_disp_wideselect_ifchainlin (516.96 us) is 4.3x the fastest (119.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_bittree} vs {carrier_disp_wideselect_threaded, carrier_disp_wideselect_fntable, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainlin} (127% apart)

The field splits into a fast tier {carrier_disp_wideselect_nullfloor, carrier_disp_wideselect_bittree} and a slow tier {carrier_disp_wideselect_threaded, carrier_disp_wideselect_fntable, carrier_disp_wideselect_ifchain, carrier_disp_wideselect_ifchainasc, carrier_disp_wideselect_switch, carrier_disp_wideselect_ifchainlin} with a 127% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest carrier_disp_wideselect_nullfloor (119.28 us) to slowest carrier_disp_wideselect_ifchainlin (516.96 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_disp_wideselect_bittree is inconsistent: worst-20% is 1.6x its best-20%

carrier_disp_wideselect_bittree's best 20% of batches run at 186.30 us but its worst 20% at 304.07 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_disp_wideselect_nullfloor** at 119276.2 ns median (-75.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.33x (fastest 119276.2 ns, slowest 516959.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 232974ns | 202306ns | 189078ns | 198243ns | 307020ns | -50.52% |
| carrier_disp_wideselect_fntable | 459583ns | 459519ns | 443675ns | 458173ns | 469653ns | -2.40% |
| carrier_disp_wideselect_ifchain | 463939ns | 465983ns | 388196ns | 460789ns | 506537ns | -1.48% |
| carrier_disp_wideselect_ifchainasc | 470938ns | 479519ns | 401954ns | 473121ns | 502154ns | +0.01% |
| carrier_disp_wideselect_ifchainlin | 526222ns | 519734ns | 514217ns | 518707ns | 543497ns | +11.75% |
| carrier_disp_wideselect_nullfloor | 121759ns | 121748ns | 118159ns | 121312ns | 124230ns | -74.14% |
| carrier_disp_wideselect_switch | 470890ns | 481537ns | 420579ns | 474504ns | 490624ns | base |
| carrier_disp_wideselect_threaded | 458471ns | 455870ns | 405150ns | 440861ns | 511546ns | -2.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 230174ns | 186300ns | 304073ns | -50.81% | 0.018 |
| carrier_disp_wideselect_fntable | 456486ns | 441377ns | 466715ns | -2.45% | 0.009 |
| carrier_disp_wideselect_ifchain | 460881ns | 384903ns | 503069ns | -1.51% | 0.009 |
| carrier_disp_wideselect_ifchainasc | 467995ns | 399713ns | 498853ns | +0.01% | 0.009 |
| carrier_disp_wideselect_ifchainlin | 523214ns | 510513ns | 540652ns | +11.81% | 0.008 |
| carrier_disp_wideselect_nullfloor | 119180ns | 115527ns | 121559ns | -74.53% | 0.034 |
| carrier_disp_wideselect_switch | 467936ns | 418322ns | 487164ns | base | 0.009 |
| carrier_disp_wideselect_threaded | 455458ns | 401772ns | 508371ns | -2.67% | 0.009 |

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_disp_wideselect_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_wideselect_bittree | 0.021 | 57.9% |
| carrier_disp_wideselect_fntable | 0.009 | 25.3% |
| carrier_disp_wideselect_ifchain | 0.009 | 24.9% |
| carrier_disp_wideselect_ifchainasc | 0.009 | 24.3% |
| carrier_disp_wideselect_ifchainlin | 0.008 | 22.3% |
| carrier_disp_wideselect_nullfloor | 0.034 | 96.9% |
| carrier_disp_wideselect_switch | 0.009 | 24.1% |
| carrier_disp_wideselect_threaded | 0.009 | 25.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_wideselect_bittree | 232974ns | 232974ns | -50.52% |
| carrier_disp_wideselect_fntable | 459583ns | 459583ns | -2.40% |
| carrier_disp_wideselect_ifchain | 463939ns | 463939ns | -1.48% |
| carrier_disp_wideselect_ifchainasc | 470938ns | 470938ns | +0.01% |
| carrier_disp_wideselect_ifchainlin | 526222ns | 526222ns | +11.75% |
| carrier_disp_wideselect_nullfloor | 121759ns | 121759ns | -74.14% |
| carrier_disp_wideselect_switch | 470890ns | 470890ns | base |
| carrier_disp_wideselect_threaded | 458471ns | 458471ns | -2.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_wideselect_switch | 478680ns | base | --- | [437963, 487164] | --- | --- | --- | --- |
| carrier_disp_wideselect_bittree | 199677ns | -253652.2ns (-53.0%) | [-297852, -161781]ns | [186772, 304073] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_wideselect_fntable | 456092ns | no significant difference | [-32031, +21224]ns | [446649, 466715] | no | 0.3828 | 0.2188 | 0 |
| carrier_disp_wideselect_ifchain | 463691ns | no significant difference | [-57086, +25708]ns | [415884, 503069] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_wideselect_ifchainasc | 476263ns | no significant difference | [-17986, +20593]ns | [428870, 498853] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_wideselect_ifchainlin | 516960ns | +43320.6ns (+9.1%) | [+26336, +96177]ns | [512030, 540652] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_wideselect_nullfloor | 119276ns | -358488.3ns (-74.9%) | [-367587, -320192]ns | [116704, 121559] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_wideselect_threaded | 452805ns | no significant difference | [-50405, +21207]ns | [405198, 508371] | no | 0.8021 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_wideselect_switch | carrier_disp_wideselect_bittree | carrier_disp_wideselect_fntable | carrier_disp_wideselect_ifchain | carrier_disp_wideselect_ifchainasc | carrier_disp_wideselect_ifchainlin | carrier_disp_wideselect_nullfloor | carrier_disp_wideselect_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 457605ns | -31.8% | -0.6% | -15.9% | +5.7% | +12.2% | -74.8% | -4.2% |
| 2 | 483257ns | -61.3% | -6.5% | +4.2% | +3.1% | +6.3% | -75.6% | -16.9% |
| 3 | 485992ns | -61.7% | -3.3% | +3.4% | -3.6% | +8.3% | -74.8% | +2.1% |
| 4 | 488336ns | -58.4% | -6.3% | -8.5% | +2.3% | +4.5% | -75.7% | +6.6% |
| 5 | 474103ns | -37.5% | -6.9% | +0.8% | -3.4% | +9.7% | -74.5% | -1.4% |
| 6 | 418322ns | -53.1% | +10.8% | +7.4% | -4.4% | +32.6% | -71.3% | -2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_wideselect_bittree | -0.268 | moderate- |
| carrier_disp_wideselect_fntable | -0.338 | moderate- |
| carrier_disp_wideselect_ifchain | -0.249 | moderate- |
| carrier_disp_wideselect_ifchainasc | 0.128 | ok |
| carrier_disp_wideselect_ifchainlin | -0.030 | ok |
| carrier_disp_wideselect_nullfloor | -0.032 | ok |
| carrier_disp_wideselect_switch | 0.086 | ok |
| carrier_disp_wideselect_threaded | 0.140 | ok |

**Consistency summary:**

- **carrier_disp_wideselect_bittree**: won 6/6, lost 0/6
- **carrier_disp_wideselect_fntable**: won 5/6, lost 1/6
- **carrier_disp_wideselect_ifchain**: won 2/6, lost 4/6
- **carrier_disp_wideselect_ifchainasc**: won 3/6, lost 3/6
- **carrier_disp_wideselect_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_wideselect_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_wideselect_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_wideselect_bittree | 231272.4ns | 230174.0ns | 100.5% | HIGH |
| carrier_disp_wideselect_fntable | 448345.1ns | 456485.6ns | 98.2% | HIGH |
| carrier_disp_wideselect_ifchain | 461883.5ns | 460881.4ns | 100.2% | HIGH |
| carrier_disp_wideselect_ifchainasc | 469263.0ns | 467995.4ns | 100.3% | HIGH |
| carrier_disp_wideselect_ifchainlin | 524340.5ns | 523213.7ns | 100.2% | HIGH |
| carrier_disp_wideselect_nullfloor | 119826.6ns | 119179.9ns | 100.5% | HIGH |
| carrier_disp_wideselect_switch | 469155.5ns | 467935.8ns | 100.3% | HIGH |
| carrier_disp_wideselect_threaded | 456704.2ns | 455458.0ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_disp_wideselect_bittree (n=6, range 186300.4-304073.1 ns)
  186300.4 |########################################
  192189.0 |####################
  198077.7 |####################
  203966.3 |
  209854.9 |
  215743.6 |
  221632.2 |
  227520.8 |
  233409.5 |
  239298.1 |
  245186.8 |
  251075.4 |
  256964.0 |
  262852.7 |
  268741.3 |
  274629.9 |
  280518.6 |
  286407.2 |
  292295.8 |####################
  298184.5 |
  (0 below, 1 above range)

carrier_disp_wideselect_fntable (n=6, range 441377.1-466715.5 ns)
  441377.1 |########################################
  442644.0 |
  443910.9 |
  445177.9 |
  446444.8 |
  447711.7 |
  448978.6 |
  450245.5 |
  451512.4 |########################################
  452779.4 |
  454046.3 |########################################
  455313.2 |
  456580.1 |########################################
  457847.0 |
  459113.9 |
  460380.9 |
  461647.8 |
  462914.7 |########################################
  464181.6 |
  465448.5 |
  (0 below, 1 above range)

carrier_disp_wideselect_ifchain (n=6, range 384903.3-503069.2 ns)
  384903.3 |####################
  390811.6 |
  396719.9 |
  402628.2 |
  408536.5 |
  414444.8 |
  420353.1 |
  426261.3 |
  432169.6 |
  438077.9 |
  443986.2 |########################################
  449894.5 |
  455802.8 |
  461711.1 |
  467619.4 |
  473527.7 |####################
  479436.0 |
  485344.3 |
  491252.6 |
  497160.9 |####################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainasc (n=6, range 399712.9-498853.3 ns)
  399712.9 |########################################
  404669.9 |
  409626.9 |
  414584.0 |
  419541.0 |
  424498.0 |
  429455.0 |
  434412.1 |
  439369.1 |
  444326.1 |
  449283.1 |
  454240.1 |########################################
  459197.2 |
  464154.2 |########################################
  469111.2 |
  474068.2 |
  479025.3 |########################################
  483982.3 |
  488939.3 |
  493896.3 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_ifchainlin (n=6, range 510512.9-540651.5 ns)
  510512.9 |####################
  512019.8 |
  513526.8 |########################################
  515033.7 |
  516540.6 |
  518047.6 |
  519554.5 |####################
  521061.4 |
  522568.3 |
  524075.3 |
  525582.2 |####################
  527089.1 |
  528596.1 |
  530103.0 |
  531609.9 |
  533116.8 |
  534623.8 |
  536130.7 |
  537637.6 |
  539144.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_nullfloor (n=6, range 115527.1-121559.2 ns)
  115527.1 |########################################
  115828.7 |
  116130.3 |
  116431.9 |
  116733.5 |
  117035.1 |
  117336.7 |
  117638.3 |########################################
  117939.9 |
  118241.5 |########################################
  118543.1 |
  118844.8 |
  119146.4 |
  119448.0 |
  119749.6 |########################################
  120051.2 |
  120352.8 |
  120654.4 |########################################
  120956.0 |
  121257.6 |
  (0 below, 1 above range)

carrier_disp_wideselect_switch (n=6, range 418322.1-487163.8 ns)
  418322.1 |########################################
  421764.2 |
  425206.3 |
  428648.3 |
  432090.4 |
  435532.5 |
  438974.6 |
  442416.7 |
  445858.8 |
  449300.8 |
  452742.9 |
  456185.0 |########################################
  459627.1 |
  463069.2 |
  466511.3 |
  469953.3 |
  473395.4 |########################################
  476837.5 |
  480279.6 |########################################
  483721.7 |########################################
  (0 below, 1 above range)

carrier_disp_wideselect_threaded (n=6, range 401772.5-508370.8 ns)
  401772.5 |########################################
  407102.4 |########################################
  412432.3 |
  417762.2 |
  423092.2 |
  428422.1 |
  433752.0 |########################################
  439081.9 |
  444411.8 |
  449741.7 |
  455071.7 |
  460401.6 |
  465731.5 |########################################
  471061.4 |
  476391.3 |
  481721.2 |
  487051.1 |
  492381.1 |########################################
  497711.0 |
  503040.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_wideselect_bittree**: CV=22.9% (high variance, measurements may be unstable)
- **carrier_disp_wideselect_bittree**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_fntable**: bridge=99.0% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchain**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainasc**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_ifchainlin**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_nullfloor**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_wideselect_threaded**: bridge=100.2% of algo (FFI overhead may distort results)
