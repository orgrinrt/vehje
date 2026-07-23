# Dispatch shape over the wire form, real profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_real_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_real_nullfloor dominates: 91% faster than the next best (carrier_disp_real_bittree)

carrier_disp_real_nullfloor (115.30 us) leads carrier_disp_real_bittree (220.17 us) by 91%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_real_nullfloor beats baseline by 79% (significant)

carrier_disp_real_nullfloor is -442.43 us (79%) faster than baseline carrier_disp_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_real_threaded is an outlier: 5.5x slower than the field

carrier_disp_real_threaded (639.56 us) is 5.5x the fastest (115.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_disp_real_switch shows alternating (throttle bounce) (autocorr -0.87)

carrier_disp_real_switch's per-pass series has lag-1 autocorrelation -0.87, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_disp_real_nullfloor, carrier_disp_real_bittree} vs {carrier_disp_real_ifchainlin, carrier_disp_real_fntable, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_ifchain, carrier_disp_real_threaded} (118% apart)

The field splits into a fast tier {carrier_disp_real_nullfloor, carrier_disp_real_bittree} and a slow tier {carrier_disp_real_ifchainlin, carrier_disp_real_fntable, carrier_disp_real_switch, carrier_disp_real_ifchainasc, carrier_disp_real_ifchain, carrier_disp_real_threaded} with a 118% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.5x the fastest

Fastest carrier_disp_real_nullfloor (115.30 us) to slowest carrier_disp_real_threaded (639.56 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_real_nullfloor** at 115297.7 ns median (-79.3% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.55x (fastest 115297.7 ns, slowest 639562.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_real_bittree | 244167ns | 222732ns | 204135ns | 217428ns | 304293ns | -55.93% |
| carrier_disp_real_fntable | 556764ns | 556071ns | 542723ns | 555304ns | 565973ns | +0.48% |
| carrier_disp_real_ifchain | 574483ns | 580194ns | 525057ns | 578083ns | 593796ns | +3.68% |
| carrier_disp_real_ifchainasc | 560449ns | 566617ns | 528894ns | 564018ns | 570874ns | +1.15% |
| carrier_disp_real_ifchainlin | 482938ns | 482965ns | 449635ns | 479881ns | 504174ns | -12.84% |
| carrier_disp_real_nullfloor | 117791ns | 117492ns | 117405ns | 117469ns | 118468ns | -78.74% |
| carrier_disp_real_switch | 554096ns | 559995ns | 522025ns | 548914ns | 577905ns | base |
| carrier_disp_real_threaded | 634523ns | 642693ns | 577678ns | 638680ns | 656710ns | +14.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_real_bittree | 241554ns | 201776ns | 301297ns | -56.19% | 0.017 |
| carrier_disp_real_fntable | 553982ns | 540497ns | 562618ns | +0.48% | 0.007 |
| carrier_disp_real_ifchain | 571634ns | 522608ns | 590516ns | +3.69% | 0.007 |
| carrier_disp_real_ifchainasc | 557647ns | 526638ns | 568106ns | +1.15% | 0.007 |
| carrier_disp_real_ifchainlin | 480183ns | 447103ns | 501169ns | -12.90% | 0.009 |
| carrier_disp_real_nullfloor | 115592ns | 115239ns | 116239ns | -79.03% | 0.035 |
| carrier_disp_real_switch | 551317ns | 519208ns | 574597ns | base | 0.007 |
| carrier_disp_real_threaded | 631405ns | 575110ns | 653428ns | +14.53% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_real_bittree | 1521924 | 4868504 | 0.313 | 0.44× |
| carrier_disp_real_fntable | 3453391 | 6387391 | 0.541 | 1.00× |
| carrier_disp_real_ifchain | 3588596 | 4720238 | 0.760 | 1.04× |
| carrier_disp_real_ifchainasc | 3499976 | 4720411 | 0.741 | 1.01× |
| carrier_disp_real_ifchainlin | 3017600 | 12693973 | 0.238 | 0.87× |
| carrier_disp_real_nullfloor | 726118 | 4060738 | 0.179 | 0.21× |
| carrier_disp_real_switch | 3449175 | 4596656 | 0.750 | 1.00× |
| carrier_disp_real_threaded | 3972808 | 6373592 | 0.623 | 1.15× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_real_nullfloor; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_real_bittree | 0.019 | 52.3% |
| carrier_disp_real_fntable | 0.007 | 20.8% |
| carrier_disp_real_ifchain | 0.007 | 20.0% |
| carrier_disp_real_ifchainasc | 0.007 | 20.4% |
| carrier_disp_real_ifchainlin | 0.009 | 24.0% |
| carrier_disp_real_nullfloor | 0.036 | 99.9% |
| carrier_disp_real_switch | 0.007 | 20.7% |
| carrier_disp_real_threaded | 0.006 | 18.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_real_bittree | 244167ns | 244167ns | -55.93% |
| carrier_disp_real_fntable | 556764ns | 556764ns | +0.48% |
| carrier_disp_real_ifchain | 574483ns | 574483ns | +3.68% |
| carrier_disp_real_ifchainasc | 560449ns | 560449ns | +1.15% |
| carrier_disp_real_ifchainlin | 482938ns | 482938ns | -12.84% |
| carrier_disp_real_nullfloor | 117791ns | 117791ns | -78.74% |
| carrier_disp_real_switch | 554096ns | 554096ns | base |
| carrier_disp_real_threaded | 634523ns | 634523ns | +14.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_real_switch | 557683ns | base | --- | [521672, 574597] | --- | --- | --- | --- |
| carrier_disp_real_bittree | 220170ns | -308413.8ns (-55.3%) | [-351338, -269538]ns | [203196, 301297] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_fntable | 553571ns | no significant difference | [-25868, +33585]ns | [545756, 562618] | no | 1.0000 | 1.0000 | 0 |
| carrier_disp_real_ifchain | 577601ns | +15602.9ns (+2.8%) | [+2050, +43297]ns | [546785, 590516] | YES (adj: no) | 0.3063 | 0.2188 | 0 |
| carrier_disp_real_ifchainasc | 563609ns | no significant difference | [-33372, +42772]ns | [541225, 568106] | no | 0.8021 | 0.6875 | 0 |
| carrier_disp_real_ifchainlin | 480338ns | -74279.2ns (-13.3%) | [-89648, -49475]ns | [459043, 501169] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_nullfloor | 115298ns | -442429.8ns (-79.3%) | [-458913, -405834]ns | [115239, 116239] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_real_threaded | 639562ns | +77619.5ns (+13.9%) | [+58983, +103661]ns | [601225, 653428] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_real_switch | carrier_disp_real_bittree | carrier_disp_real_fntable | carrier_disp_real_ifchain | carrier_disp_real_ifchainasc | carrier_disp_real_ifchainlin | carrier_disp_real_nullfloor | carrier_disp_real_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 547437ns | -62.0% | +3.3% | +6.4% | +2.9% | -14.0% | -78.9% | +19.0% |
| 2 | 569810ns | -51.7% | -3.1% | +1.0% | -2.5% | -9.9% | -79.6% | +15.0% |
| 3 | 519208ns | -55.3% | +6.1% | +10.0% | +8.6% | -13.9% | -77.6% | +10.8% |
| 4 | 579385ns | -43.5% | -4.2% | +3.3% | -9.1% | -15.6% | -80.1% | +12.0% |
| 5 | 524135ns | -61.5% | +6.7% | -0.3% | +7.8% | -8.1% | -78.0% | +19.7% |
| 6 | 567930ns | -64.0% | -4.8% | +2.1% | +0.6% | -15.6% | -79.7% | +10.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_real_bittree | -0.331 | moderate- |
| carrier_disp_real_fntable | -0.243 | moderate- |
| carrier_disp_real_ifchain | -0.513 | HIGH- (thermal bounce) |
| carrier_disp_real_ifchainasc | -0.269 | moderate- |
| carrier_disp_real_ifchainlin | -0.712 | HIGH- (thermal bounce) |
| carrier_disp_real_nullfloor | 0.100 | ok |
| carrier_disp_real_switch | -0.872 | HIGH- (thermal bounce) |
| carrier_disp_real_threaded | -0.430 | moderate- |

**Consistency summary:**

- **carrier_disp_real_bittree**: won 6/6, lost 0/6
- **carrier_disp_real_fntable**: won 3/6, lost 3/6
- **carrier_disp_real_ifchain**: won 1/6, lost 5/6
- **carrier_disp_real_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_real_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_real_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_real_bittree | 242624.0ns | 241554.3ns | 100.4% | HIGH |
| carrier_disp_real_fntable | 549239.0ns | 553981.7ns | 99.1% | HIGH |
| carrier_disp_real_ifchain | 571949.9ns | 571633.9ns | 100.1% | HIGH |
| carrier_disp_real_ifchainasc | 557955.8ns | 557646.5ns | 100.1% | HIGH |
| carrier_disp_real_ifchainlin | 482215.7ns | 480183.2ns | 100.4% | HIGH |
| carrier_disp_real_nullfloor | 116114.5ns | 115591.9ns | 100.5% | HIGH |
| carrier_disp_real_switch | 552418.5ns | 551317.4ns | 100.2% | HIGH |
| carrier_disp_real_threaded | 632556.7ns | 631405.0ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_disp_real_bittree (n=6, range 201775.8-301296.9 ns)
  201775.8 |########################################
  206751.9 |####################
  211727.9 |
  216704.0 |
  221680.0 |
  226656.1 |
  231632.1 |####################
  236608.2 |
  241584.2 |
  246560.3 |
  251536.4 |
  256512.4 |
  261488.5 |
  266464.5 |
  271440.6 |####################
  276416.6 |
  281392.7 |
  286368.7 |
  291344.8 |
  296320.8 |
  (0 below, 1 above range)

carrier_disp_real_fntable (n=6, range 540497.1-562617.7 ns)
  540497.1 |########################################
  541603.1 |
  542709.2 |
  543815.2 |
  544921.2 |
  546027.2 |
  547133.3 |
  548239.3 |
  549345.3 |
  550451.4 |########################################
  551557.4 |########################################
  552663.4 |
  553769.5 |
  554875.5 |########################################
  555981.5 |
  557087.5 |
  558193.6 |
  559299.6 |########################################
  560405.6 |
  561511.7 |
  (0 below, 1 above range)

carrier_disp_real_ifchain (n=6, range 522607.9-590515.8 ns)
  522607.9 |########################################
  526003.3 |
  529398.7 |
  532794.1 |
  536189.5 |
  539584.9 |
  542980.3 |
  546375.7 |
  549771.1 |
  553166.5 |
  556561.9 |
  559957.3 |
  563352.7 |
  566748.1 |
  570143.5 |########################################
  573538.9 |########################################
  576934.3 |########################################
  580329.7 |########################################
  583725.1 |
  587120.5 |
  (0 below, 1 above range)

carrier_disp_real_ifchainasc (n=6, range 526637.5-568106.1 ns)
  526637.5 |####################
  528710.9 |
  530784.4 |
  532857.8 |
  534931.2 |
  537004.6 |
  539078.1 |
  541151.5 |
  543224.9 |
  545298.3 |
  547371.8 |
  549445.2 |
  551518.6 |
  553592.1 |
  555665.5 |####################
  557738.9 |
  559812.3 |
  561885.8 |########################################
  563959.2 |####################
  566032.6 |
  (0 below, 1 above range)

carrier_disp_real_ifchainlin (n=6, range 447102.9-501169.0 ns)
  447102.9 |########################################
  449806.2 |
  452509.5 |
  455212.8 |
  457916.1 |
  460619.4 |
  463322.7 |
  466026.0 |
  468729.3 |########################################
  471432.6 |
  474135.9 |
  476839.2 |########################################
  479542.5 |########################################
  482245.8 |
  484949.1 |
  487652.4 |########################################
  490355.7 |
  493059.0 |
  495762.3 |
  498465.6 |
  (0 below, 1 above range)

carrier_disp_real_nullfloor (n=6, range 115239.2-116238.8 ns)
  115239.2 |########################################
  115289.2 |#############
  115339.2 |
  115389.1 |
  115439.1 |
  115489.1 |
  115539.1 |
  115589.0 |
  115639.0 |
  115689.0 |
  115739.0 |
  115789.0 |
  115838.9 |
  115888.9 |
  115938.9 |
  115988.9 |
  116038.8 |
  116088.8 |#############
  116138.8 |
  116188.8 |
  (0 below, 1 above range)

carrier_disp_real_switch (n=6, range 519207.9-574597.3 ns)
  519207.9 |########################################
  521977.4 |########################################
  524746.8 |
  527516.3 |
  530285.8 |
  533055.2 |
  535824.7 |
  538594.2 |
  541363.7 |
  544133.1 |
  546902.6 |########################################
  549672.1 |
  552441.5 |
  555211.0 |
  557980.5 |
  560750.0 |
  563519.4 |
  566288.9 |########################################
  569058.4 |########################################
  571827.8 |
  (0 below, 1 above range)

carrier_disp_real_threaded (n=6, range 575110.4-653427.9 ns)
  575110.4 |########################################
  579026.3 |
  582942.2 |
  586858.0 |
  590773.9 |
  594689.8 |
  598605.7 |
  602521.5 |
  606437.4 |
  610353.3 |
  614269.1 |
  618185.0 |
  622100.9 |
  626016.8 |########################################
  629932.6 |########################################
  633848.5 |
  637764.4 |
  641680.3 |
  645596.1 |########################################
  649512.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_real_bittree**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_fntable**: bridge=99.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchain**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainasc**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_real_ifchainlin**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_real_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_real_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_disp_real_threaded**: bridge=100.2% of algo (FFI overhead may distort results)
