# Entropy x locality surface: op_correlation {0,500,900} x locality_window {4,64,unbounded}, fixed predecoded switch dispatch

9 variants, 6 samples per variant.
Baseline: **carrier_ent_c0_w64**

## Highlights

Baseline for all deltas below: **carrier_ent_c0_w64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ent_c900_wmax beats baseline by 72% (significant)

carrier_ent_c900_wmax is -336.32 us (72%) faster than baseline carrier_ent_c0_w64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ent_c0_wmax is an outlier: 3.8x slower than the field

carrier_ent_c0_wmax (500.48 us) is 3.8x the fastest (131.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_ent_c900_w64 shows alternating (throttle bounce) (autocorr -0.59)

carrier_ent_c900_w64's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_ent_c900_wmax, carrier_ent_c900_w64, carrier_ent_c900_w4} vs {carrier_ent_c500_w4, carrier_ent_c500_wmax, carrier_ent_c500_w64, carrier_ent_c0_w64, carrier_ent_c0_w4, carrier_ent_c0_wmax} (146% apart)

The field splits into a fast tier {carrier_ent_c900_wmax, carrier_ent_c900_w64, carrier_ent_c900_w4} and a slow tier {carrier_ent_c500_w4, carrier_ent_c500_wmax, carrier_ent_c500_w64, carrier_ent_c0_w64, carrier_ent_c0_w4, carrier_ent_c0_wmax} with a 146% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.8x the fastest

Fastest carrier_ent_c900_wmax (131.58 us) to slowest carrier_ent_c0_wmax (500.48 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_ent_c900_wmax** at 131584.8 ns median (-71.8% vs baseline)
- 6 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.80x (fastest 131584.8 ns, slowest 500478.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 479305ns | 477831ns | 461550ns | 473867ns | 496340ns | +1.19% |
| carrier_ent_c0_w64 | 473669ns | 469380ns | 461072ns | 468348ns | 487948ns | base |
| carrier_ent_c0_wmax | 503194ns | 502650ns | 495165ns | 501304ns | 510043ns | +6.23% |
| carrier_ent_c500_w4 | 432386ns | 434341ns | 410246ns | 429619ns | 447606ns | -8.72% |
| carrier_ent_c500_w64 | 443490ns | 447362ns | 421003ns | 440160ns | 459729ns | -6.37% |
| carrier_ent_c500_wmax | 435975ns | 437566ns | 414452ns | 432960ns | 451258ns | -7.96% |
| carrier_ent_c900_w4 | 178481ns | 178166ns | 176160ns | 177569ns | 181009ns | -62.32% |
| carrier_ent_c900_w64 | 136117ns | 135877ns | 135154ns | 135794ns | 137084ns | -71.26% |
| carrier_ent_c900_wmax | 134397ns | 133757ns | 132645ns | 133600ns | 136470ns | -71.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 477093ns | 459326ns | 494129ns | +1.19% | 0.009 |
| carrier_ent_c0_w64 | 471464ns | 458868ns | 485759ns | base | 0.009 |
| carrier_ent_c0_wmax | 500992ns | 492963ns | 507831ns | +6.26% | 0.008 |
| carrier_ent_c500_w4 | 430192ns | 408052ns | 445420ns | -8.75% | 0.010 |
| carrier_ent_c500_w64 | 441303ns | 418792ns | 457588ns | -6.40% | 0.009 |
| carrier_ent_c500_wmax | 433741ns | 412301ns | 449103ns | -8.00% | 0.009 |
| carrier_ent_c900_w4 | 176291ns | 174015ns | 178780ns | -62.61% | 0.023 |
| carrier_ent_c900_w64 | 133930ns | 132976ns | 134891ns | -71.59% | 0.031 |
| carrier_ent_c900_wmax | 132222ns | 130499ns | 134271ns | -71.95% | 0.031 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 3081817 | 2881146 | 1.070 | 1.00× |
| carrier_ent_c0_w64 | 3077619 | 2880492 | 1.068 | 1.00× |
| carrier_ent_c0_wmax | 3135564 | 2880409 | 1.089 | 1.02× |
| carrier_ent_c500_w4 | 2764990 | 2899550 | 0.954 | 0.90× |
| carrier_ent_c500_w64 | 2765957 | 2899649 | 0.954 | 0.90× |
| carrier_ent_c500_wmax | 2807740 | 2899462 | 0.968 | 0.91× |
| carrier_ent_c900_w4 | 1141853 | 2920432 | 0.391 | 0.37× |
| carrier_ent_c900_w64 | 878511 | 2917322 | 0.301 | 0.29× |
| carrier_ent_c900_wmax | 872287 | 2917475 | 0.299 | 0.28× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.031 Gops/s** (carrier_ent_c900_wmax; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ent_c0_w4 | 0.009 | 27.4% |
| carrier_ent_c0_w64 | 0.009 | 27.9% |
| carrier_ent_c0_wmax | 0.008 | 26.1% |
| carrier_ent_c500_w4 | 0.009 | 30.2% |
| carrier_ent_c500_w64 | 0.009 | 29.3% |
| carrier_ent_c500_wmax | 0.009 | 30.0% |
| carrier_ent_c900_w4 | 0.023 | 74.2% |
| carrier_ent_c900_w64 | 0.031 | 97.6% |
| carrier_ent_c900_wmax | 0.031 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ent_c0_w4 | 479305ns | 479305ns | +1.19% |
| carrier_ent_c0_w64 | 473669ns | 473669ns | base |
| carrier_ent_c0_wmax | 503194ns | 503194ns | +6.23% |
| carrier_ent_c500_w4 | 432386ns | 432386ns | -8.72% |
| carrier_ent_c500_w64 | 443490ns | 443490ns | -6.37% |
| carrier_ent_c500_wmax | 435975ns | 435975ns | -7.96% |
| carrier_ent_c900_w4 | 178481ns | 178481ns | -62.32% |
| carrier_ent_c900_w64 | 136117ns | 136117ns | -71.26% |
| carrier_ent_c900_wmax | 134397ns | 134397ns | -71.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ent_c0_w64 | 467162ns | base | --- | [461470, 485759] | --- | --- | --- | --- |
| carrier_ent_c0_w4 | 475615ns | no significant difference | [-7443, +20955]ns | [461535, 494129] | no | 0.6875 | 0.6875 | 0 |
| carrier_ent_c0_wmax | 500479ns | +38973.8ns (+8.3%) | [+8907, +40704]ns | [494666, 507831] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c500_w4 | 432143ns | -48030.0ns (-10.3%) | [-59735, -16051]ns | [413013, 445420] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c500_w64 | 445136ns | -27992.3ns (-6.0%) | [-58573, -3916]ns | [421185, 457588] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c500_wmax | 435288ns | -33000.2ns (-7.1%) | [-59870, -20296]ns | [416833, 449103] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c900_w4 | 175986ns | -290137.7ns (-62.1%) | [-310225, -285156]ns | [174107, 178780] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c900_w64 | 133695ns | -332576.7ns (-71.2%) | [-352063, -327960]ns | [133205, 134891] | YES | 0.0357 | 0.0313 | 0 |
| carrier_ent_c900_wmax | 131585ns | -336317.1ns (-72.0%) | [-352179, -329229]ns | [130811, 134271] | YES | 0.0357 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ent_c0_w64 | carrier_ent_c0_w4 | carrier_ent_c0_wmax | carrier_ent_c500_w4 | carrier_ent_c500_w64 | carrier_ent_c500_wmax | carrier_ent_c900_w4 | carrier_ent_c900_w64 | carrier_ent_c900_wmax |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 482183ns | -3.0% | +2.9% | -9.1% | -8.2% | -6.2% | -63.3% | -72.3% | -71.8% |
| 2 | 489334ns | +1.3% | +0.7% | -13.0% | -14.4% | -13.9% | -64.4% | -72.7% | -73.2% |
| 3 | 464073ns | +4.2% | +8.6% | -3.4% | -3.5% | -11.2% | -62.5% | -71.1% | -71.7% |
| 4 | 458868ns | +0.1% | +8.6% | -3.6% | -0.9% | -3.6% | -61.9% | -71.0% | -71.2% |
| 5 | 470182ns | +4.8% | +8.9% | -11.1% | -9.9% | -5.1% | -61.8% | -71.1% | -72.2% |
| 6 | 464142ns | -0.1% | +8.3% | -12.1% | -0.7% | -7.7% | -61.7% | -71.3% | -71.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ent_c0_w4 | -0.540 | HIGH- (thermal bounce) |
| carrier_ent_c0_w64 | 0.253 | moderate+ |
| carrier_ent_c0_wmax | -0.019 | ok |
| carrier_ent_c500_w4 | 0.193 | ok |
| carrier_ent_c500_w64 | -0.471 | moderate- |
| carrier_ent_c500_wmax | -0.091 | ok |
| carrier_ent_c900_w4 | 0.239 | moderate+ |
| carrier_ent_c900_w64 | -0.592 | HIGH- (thermal bounce) |
| carrier_ent_c900_wmax | -0.145 | ok |

**Consistency summary:**

- **carrier_ent_c0_w4**: won 1/6, lost 3/6
- **carrier_ent_c0_wmax**: won 0/6, lost 6/6
- **carrier_ent_c500_w4**: won 6/6, lost 0/6
- **carrier_ent_c500_w64**: won 6/6, lost 0/6
- **carrier_ent_c500_wmax**: won 6/6, lost 0/6
- **carrier_ent_c900_w4**: won 6/6, lost 0/6
- **carrier_ent_c900_w64**: won 6/6, lost 0/6
- **carrier_ent_c900_wmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 508562.2ns | 477093.2ns | 106.6% | HIGH |
| carrier_ent_c0_w64 | 511731.4ns | 471463.8ns | 108.5% | HIGH |
| carrier_ent_c0_wmax | 499776.9ns | 500991.9ns | 99.8% | HIGH |
| carrier_ent_c500_w4 | 456120.7ns | 430191.8ns | 106.0% | HIGH |
| carrier_ent_c500_w64 | 443840.2ns | 441303.2ns | 100.6% | HIGH |
| carrier_ent_c500_wmax | 466755.4ns | 433741.4ns | 107.6% | HIGH |
| carrier_ent_c900_w4 | 188018.8ns | 176291.0ns | 106.7% | HIGH |
| carrier_ent_c900_w64 | 146586.2ns | 133930.5ns | 109.4% | HIGH |
| carrier_ent_c900_wmax | 145555.8ns | 132222.1ns | 110.1% | HIGH |

## Distribution (algo ns)

```
carrier_ent_c0_w4 (n=6, range 459326.2-494129.4 ns)
  459326.2 |########################################
  461066.4 |
  462806.5 |########################################
  464546.7 |
  466286.8 |########################################
  468027.0 |
  469767.2 |
  471507.3 |
  473247.5 |
  474987.6 |
  476727.8 |
  478468.0 |
  480208.1 |
  481948.3 |########################################
  483688.4 |
  485428.6 |
  487168.8 |
  488908.9 |
  490649.1 |
  492389.2 |########################################
  (0 below, 1 above range)

carrier_ent_c0_w64 (n=6, range 458867.5-485758.8 ns)
  458867.5 |####################
  460212.1 |
  461556.6 |
  462901.2 |########################################
  464245.8 |
  465590.3 |
  466934.9 |
  468279.4 |
  469624.0 |####################
  470968.6 |
  472313.1 |
  473657.7 |
  475002.2 |
  476346.8 |
  477691.4 |
  479035.9 |
  480380.5 |
  481725.1 |####################
  483069.6 |
  484414.2 |
  (0 below, 1 above range)

carrier_ent_c0_wmax (n=6, range 492962.9-507831.5 ns)
  492962.9 |########################################
  493706.3 |
  494449.8 |
  495193.2 |
  495936.6 |########################################
  496680.0 |
  497423.5 |
  498166.9 |########################################
  498910.3 |
  499653.7 |
  500397.2 |
  501140.6 |
  501884.0 |########################################
  502627.5 |
  503370.9 |########################################
  504114.3 |
  504857.7 |
  505601.2 |
  506344.6 |
  507088.0 |
  (0 below, 1 above range)

carrier_ent_c500_w4 (n=6, range 408052.1-445419.6 ns)
  408052.1 |########################################
  409920.5 |
  411788.8 |
  413657.2 |
  415525.6 |
  417394.0 |########################################
  419262.3 |
  421130.7 |
  422999.1 |
  424867.5 |########################################
  426735.8 |
  428604.2 |
  430472.6 |
  432341.0 |
  434209.3 |
  436077.7 |
  437946.1 |########################################
  439814.5 |
  441682.8 |########################################
  443551.2 |
  (0 below, 1 above range)

carrier_ent_c500_w64 (n=6, range 418792.5-457588.5 ns)
  418792.5 |########################################
  420732.3 |
  422672.1 |########################################
  424611.9 |
  426551.7 |
  428491.5 |
  430431.3 |
  432371.1 |
  434310.9 |
  436250.7 |
  438190.5 |
  440130.3 |
  442070.1 |########################################
  444009.9 |
  445949.7 |########################################
  447889.5 |
  449829.3 |
  451769.1 |
  453708.9 |########################################
  455648.7 |
  (0 below, 1 above range)

carrier_ent_c500_wmax (n=6, range 412301.2-449103.3 ns)
  412301.2 |########################################
  414141.3 |
  415981.4 |
  417821.5 |
  419661.6 |########################################
  421501.7 |
  423341.8 |
  425181.9 |
  427022.0 |########################################
  428862.1 |
  430702.2 |
  432542.4 |
  434382.5 |
  436222.6 |
  438062.7 |
  439902.8 |
  441742.9 |########################################
  443583.0 |
  445423.1 |########################################
  447263.2 |
  (0 below, 1 above range)

carrier_ent_c900_w4 (n=6, range 174015.0-178780.0 ns)
  174015.0 |########################################
  174253.2 |
  174491.5 |
  174729.8 |####################
  174968.0 |
  175206.2 |
  175444.5 |
  175682.8 |
  175921.0 |
  176159.2 |
  176397.5 |
  176635.8 |
  176874.0 |####################
  177112.2 |
  177350.5 |
  177588.8 |####################
  177827.0 |
  178065.2 |
  178303.5 |
  178541.8 |
  (0 below, 1 above range)

carrier_ent_c900_w64 (n=6, range 132976.2-134890.7 ns)
  132976.2 |########################################
  133071.9 |
  133167.6 |
  133263.4 |
  133359.1 |########################################
  133454.8 |
  133550.5 |########################################
  133646.3 |
  133742.0 |########################################
  133837.7 |
  133933.4 |
  134029.1 |########################################
  134124.9 |
  134220.6 |
  134316.3 |
  134412.0 |
  134507.8 |
  134603.5 |
  134699.2 |
  134794.9 |
  (0 below, 1 above range)

carrier_ent_c900_wmax (n=6, range 130498.7-134270.8 ns)
  130498.7 |####################
  130687.3 |
  130875.9 |
  131064.5 |########################################
  131253.1 |
  131441.7 |
  131630.3 |
  131818.9 |
  132007.5 |####################
  132196.1 |
  132384.8 |####################
  132573.4 |
  132762.0 |
  132950.6 |
  133139.2 |
  133327.8 |
  133516.4 |
  133705.0 |
  133893.6 |
  134082.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ent_c0_w4**: bridge=107.1% of algo (FFI overhead may distort results)
- **carrier_ent_c0_w64**: bridge=110.1% of algo (FFI overhead may distort results)
- **carrier_ent_c0_wmax**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w4**: bridge=105.3% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w64**: bridge=99.4% of algo (FFI overhead may distort results)
- **carrier_ent_c500_wmax**: bridge=106.3% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w4**: bridge=106.5% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w64**: bridge=109.3% of algo (FFI overhead may distort results)
- **carrier_ent_c900_wmax**: bridge=110.2% of algo (FFI overhead may distort results)
