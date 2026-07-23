# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 162% faster than the next best (carrier_disp_scatter_bittree)

carrier_disp_scatter_nullfloor (116.91 us) leads carrier_disp_scatter_bittree (306.03 us) by 162%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 79% (significant)

carrier_disp_scatter_nullfloor is -449.40 us (79%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_threaded is an outlier: 5.0x slower than the field

carrier_disp_scatter_threaded (589.17 us) is 5.0x the fastest (116.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_scatter_ifchainasc shows alternating (throttle bounce) (autocorr -0.53)

carrier_disp_scatter_ifchainasc's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_scatter_nullfloor} vs {carrier_disp_scatter_bittree, carrier_disp_scatter_fntable, carrier_disp_scatter_ifchainlin, carrier_disp_scatter_ifchain, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_threaded} (162% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor} and a slow tier {carrier_disp_scatter_bittree, carrier_disp_scatter_fntable, carrier_disp_scatter_ifchainlin, carrier_disp_scatter_ifchain, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_threaded} with a 162% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.0x the fastest

Fastest carrier_disp_scatter_nullfloor (116.91 us) to slowest carrier_disp_scatter_threaded (589.17 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 116911.9 ns median (-79.4% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.04x (fastest 116911.9 ns, slowest 589170.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 301444ns | 308800ns | 231855ns | 300990ns | 336918ns | -46.61% |
| carrier_disp_scatter_fntable | 552777ns | 552212ns | 543972ns | 550865ns | 560049ns | -2.10% |
| carrier_disp_scatter_ifchain | 568380ns | 562079ns | 540955ns | 556376ns | 600097ns | +0.67% |
| carrier_disp_scatter_ifchainasc | 585719ns | 569740ns | 550126ns | 563333ns | 637095ns | +3.74% |
| carrier_disp_scatter_ifchainlin | 568997ns | 559503ns | 520457ns | 547294ns | 625820ns | +0.78% |
| carrier_disp_scatter_nullfloor | 119960ns | 119153ns | 118118ns | 119008ns | 122309ns | -78.75% |
| carrier_disp_scatter_switch | 564618ns | 570945ns | 530728ns | 562793ns | 584302ns | base |
| carrier_disp_scatter_threaded | 606289ns | 592050ns | 568407ns | 588636ns | 651711ns | +7.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 298762ns | 229551ns | 333993ns | -46.84% | 0.014 |
| carrier_disp_scatter_fntable | 550032ns | 540842ns | 557532ns | -2.13% | 0.007 |
| carrier_disp_scatter_ifchain | 565231ns | 537950ns | 597014ns | +0.57% | 0.007 |
| carrier_disp_scatter_ifchainasc | 582799ns | 547543ns | 633981ns | +3.70% | 0.007 |
| carrier_disp_scatter_ifchainlin | 566187ns | 518174ns | 622409ns | +0.74% | 0.007 |
| carrier_disp_scatter_nullfloor | 117624ns | 115632ns | 119875ns | -79.07% | 0.035 |
| carrier_disp_scatter_switch | 562003ns | 528180ns | 581783ns | base | 0.007 |
| carrier_disp_scatter_threaded | 603444ns | 565967ns | 648402ns | +7.37% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 1876408 | 4970794 | 0.377 | 0.54× |
| carrier_disp_scatter_fntable | 3460294 | 6564440 | 0.527 | 0.99× |
| carrier_disp_scatter_ifchain | 3557627 | 4835222 | 0.736 | 1.02× |
| carrier_disp_scatter_ifchainasc | 3633528 | 4836203 | 0.751 | 1.04× |
| carrier_disp_scatter_ifchainlin | 3533749 | 13237264 | 0.267 | 1.01× |
| carrier_disp_scatter_nullfloor | 733703 | 4154434 | 0.177 | 0.21× |
| carrier_disp_scatter_switch | 3496999 | 4704866 | 0.743 | 1.00× |
| carrier_disp_scatter_threaded | 3783025 | 6531161 | 0.579 | 1.08× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.013 | 37.8% |
| carrier_disp_scatter_fntable | 0.007 | 21.1% |
| carrier_disp_scatter_ifchain | 0.007 | 20.7% |
| carrier_disp_scatter_ifchainasc | 0.007 | 20.4% |
| carrier_disp_scatter_ifchainlin | 0.007 | 20.8% |
| carrier_disp_scatter_nullfloor | 0.035 | 98.9% |
| carrier_disp_scatter_switch | 0.007 | 20.4% |
| carrier_disp_scatter_threaded | 0.007 | 19.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 301444ns | 301444ns | -46.61% |
| carrier_disp_scatter_fntable | 552777ns | 552777ns | -2.10% |
| carrier_disp_scatter_ifchain | 568380ns | 568380ns | +0.67% |
| carrier_disp_scatter_ifchainasc | 585719ns | 585719ns | +3.74% |
| carrier_disp_scatter_ifchainlin | 568997ns | 568997ns | +0.78% |
| carrier_disp_scatter_nullfloor | 119960ns | 119960ns | -78.75% |
| carrier_disp_scatter_switch | 564618ns | 564618ns | base |
| carrier_disp_scatter_threaded | 606289ns | 606289ns | +7.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 568031ns | base | --- | [536194, 581783] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 306030ns | -255986.9ns (-45.1%) | [-313283, -220452]ns | [256262, 333993] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 549151ns | no significant difference | [-33946, +10292]ns | [543414, 557532] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_scatter_ifchain | 558888ns | no significant difference | [-39754, +48550]ns | [539791, 597014] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_scatter_ifchainasc | 566611ns | no significant difference | [-3515, +53712]ns | [547806, 633981] | no | 0.3828 | 0.2188 | 0 |
| carrier_disp_scatter_ifchainlin | 556789ns | no significant difference | [-18811, +40625]ns | [519364, 622409] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_scatter_nullfloor | 116912ns | -449399.8ns (-79.1%) | [-465033, -418702]ns | [116085, 119875] | YES (adj: no) | 0.1094 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 589170ns | +25072.1ns (+4.4%) | [+6069, +93182]ns | [572758, 648402] | YES (adj: no) | 0.3828 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 570886ns | -59.8% | -1.4% | +4.9% | +0.9% | +11.0% | -79.6% | +2.5% |
| 2 | 592680ns | -43.6% | -6.8% | -8.3% | +5.2% | +3.1% | -80.3% | +4.2% |
| 3 | 528180ns | -39.2% | +3.6% | +8.7% | +3.7% | -1.9% | -77.9% | +28.6% |
| 4 | 568204ns | -50.2% | -4.8% | -5.3% | -1.9% | -1.5% | -78.6% | -0.4% |
| 5 | 567858ns | -41.2% | -2.9% | -4.6% | +13.5% | -2.5% | -79.6% | +4.4% |
| 6 | 544207ns | -46.6% | +0.3% | +9.4% | +0.7% | -4.3% | -78.3% | +6.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.351 | moderate- |
| carrier_disp_scatter_fntable | 0.113 | ok |
| carrier_disp_scatter_ifchain | -0.320 | moderate- |
| carrier_disp_scatter_ifchainasc | -0.526 | HIGH- (thermal bounce) |
| carrier_disp_scatter_ifchainlin | 0.164 | ok |
| carrier_disp_scatter_nullfloor | -0.481 | moderate- |
| carrier_disp_scatter_switch | -0.408 | moderate- |
| carrier_disp_scatter_threaded | -0.166 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 6/6, lost 0/6
- **carrier_disp_scatter_fntable**: won 4/6, lost 2/6
- **carrier_disp_scatter_ifchain**: won 3/6, lost 3/6
- **carrier_disp_scatter_ifchainasc**: won 1/6, lost 5/6
- **carrier_disp_scatter_ifchainlin**: won 4/6, lost 2/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 299162.9ns | 298762.0ns | 100.1% | HIGH |
| carrier_disp_scatter_fntable | 555181.0ns | 550032.4ns | 100.9% | HIGH |
| carrier_disp_scatter_ifchain | 565228.5ns | 565231.1ns | 100.0% | HIGH |
| carrier_disp_scatter_ifchainasc | 582262.8ns | 582799.1ns | 99.9% | HIGH |
| carrier_disp_scatter_ifchainlin | 567770.6ns | 566187.0ns | 100.3% | HIGH |
| carrier_disp_scatter_nullfloor | 117904.0ns | 117624.2ns | 100.2% | HIGH |
| carrier_disp_scatter_switch | 562546.9ns | 562002.6ns | 100.1% | HIGH |
| carrier_disp_scatter_threaded | 604808.5ns | 603443.6ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 229551.2-333993.3 ns)
  229551.2 |########################################
  234773.3 |
  239995.4 |
  245217.5 |
  250439.6 |
  255661.7 |
  260883.8 |
  266106.0 |
  271328.1 |
  276550.2 |
  281772.3 |########################################
  286994.4 |########################################
  292216.5 |
  297438.6 |
  302660.7 |
  307882.8 |
  313104.9 |
  318327.0 |########################################
  323549.1 |
  328771.2 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 540841.7-557532.3 ns)
  540841.7 |########################################
  541676.2 |
  542510.8 |
  543345.3 |
  544179.8 |
  545014.3 |
  545848.9 |########################################
  546683.4 |########################################
  547517.9 |
  548352.5 |
  549187.0 |
  550021.5 |
  550856.1 |########################################
  551690.6 |########################################
  552525.1 |
  553359.7 |
  554194.2 |
  555028.7 |
  555863.2 |
  556697.8 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 537949.6-597013.9 ns)
  537949.6 |####################
  540902.8 |########################################
  543856.0 |
  546809.3 |
  549762.5 |
  552715.7 |
  555668.9 |
  558622.1 |
  561575.3 |
  564528.6 |
  567481.8 |
  570435.0 |
  573388.2 |####################
  576341.4 |
  579294.6 |
  582247.9 |
  585201.1 |
  588154.3 |
  591107.5 |
  594060.7 |####################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 547543.3-633980.8 ns)
  547543.3 |########################################
  551865.2 |
  556187.1 |####################
  560508.9 |
  564830.8 |
  569152.7 |
  573474.6 |####################
  577796.4 |
  582118.3 |
  586440.2 |
  590762.1 |
  595084.0 |
  599405.8 |
  603727.7 |
  608049.6 |
  612371.5 |
  616693.3 |
  621015.2 |####################
  625337.1 |
  629659.0 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 518174.2-622408.6 ns)
  518174.2 |########################################
  523385.9 |
  528597.6 |
  533809.4 |
  539021.1 |
  544232.8 |
  549444.5 |####################
  554656.2 |####################
  559867.9 |
  565079.7 |
  570291.4 |
  575503.1 |
  580714.8 |
  585926.5 |
  591138.2 |
  596350.0 |
  601561.7 |
  606773.4 |####################
  611985.1 |
  617196.8 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 115631.7-119875.4 ns)
  115631.7 |########################################
  115843.9 |
  116056.1 |
  116268.3 |
  116480.4 |########################################
  116692.6 |########################################
  116904.8 |########################################
  117117.0 |
  117329.2 |
  117541.4 |
  117753.5 |
  117965.7 |########################################
  118177.9 |
  118390.1 |
  118602.3 |
  118814.5 |
  119026.7 |
  119238.8 |
  119451.0 |
  119663.2 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 528180.4-581783.1 ns)
  528180.4 |####################
  530860.5 |
  533540.7 |
  536220.8 |
  538900.9 |
  541581.1 |####################
  544261.2 |
  546941.3 |
  549621.5 |
  552301.6 |
  554981.8 |
  557661.9 |
  560342.0 |
  563022.2 |
  565702.3 |########################################
  568382.4 |####################
  571062.6 |
  573742.7 |
  576422.8 |
  579103.0 |
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 565967.1-648402.5 ns)
  565967.1 |########################################
  570088.9 |
  574210.6 |
  578332.4 |########################################
  582454.2 |########################################
  586575.9 |
  590697.7 |########################################
  594819.5 |
  598941.3 |
  603063.0 |
  607184.8 |
  611306.6 |
  615428.3 |########################################
  619550.1 |
  623671.9 |
  627793.7 |
  631915.4 |
  636037.2 |
  640159.0 |
  644280.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=100.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=100.2% of algo (FFI overhead may distort results)
