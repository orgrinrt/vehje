# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_nullfloor shows alternating (throttle bounce) (autocorr -0.84)

carrier_disp_madd_nullfloor's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_disp_madd_ifchainasc's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_disp_madd_ifchainasc are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### carrier_disp_madd_ifchainasc's edge over baseline is significant but tiny (30 ns, 0.26%)

carrier_disp_madd_ifchainasc differs from baseline carrier_disp_madd_switch by 30 ns (0.26%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 9966.0 ns median (-14.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.27x (fastest 9966.0 ns, slowest 12666.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 13773ns | 13422ns | 12575ns | 13144ns | 15316ns | -1.99% |
| carrier_disp_madd_fntable | 15259ns | 15038ns | 14210ns | 14765ns | 16524ns | +8.58% |
| carrier_disp_madd_ifchain | 14266ns | 14188ns | 13388ns | 13929ns | 15211ns | +1.52% |
| carrier_disp_madd_ifchainasc | 13983ns | 13943ns | 13146ns | 13697ns | 14832ns | -0.49% |
| carrier_disp_madd_ifchainlin | 12980ns | 12568ns | 12429ns | 12554ns | 13896ns | -7.63% |
| carrier_disp_madd_nullfloor | 12399ns | 12387ns | 11694ns | 12177ns | 13085ns | -11.77% |
| carrier_disp_madd_switch | 14053ns | 13959ns | 13055ns | 13671ns | 15125ns | base |
| carrier_disp_madd_threaded | 14584ns | 14391ns | 13912ns | 14312ns | 15330ns | +3.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 11300ns | 10392ns | 12458ns | -3.22% | 0.023 |
| carrier_disp_madd_fntable | 12861ns | 12024ns | 13884ns | +10.15% | 0.020 |
| carrier_disp_madd_ifchain | 11907ns | 11172ns | 12651ns | +1.97% | 0.022 |
| carrier_disp_madd_ifchainasc | 11629ns | 10967ns | 12348ns | -0.41% | 0.022 |
| carrier_disp_madd_ifchainlin | 10727ns | 10285ns | 11470ns | -8.13% | 0.024 |
| carrier_disp_madd_nullfloor | 9994ns | 9498ns | 10515ns | -14.41% | 0.026 |
| carrier_disp_madd_switch | 11676ns | 10900ns | 12499ns | base | 0.022 |
| carrier_disp_madd_threaded | 12229ns | 11741ns | 12849ns | +4.73% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 315853 | 1446729 | 0.218 | 1.02× |
| carrier_disp_madd_fntable | 308177 | 1607132 | 0.192 | 1.00× |
| carrier_disp_madd_ifchain | 308020 | 1313263 | 0.235 | 1.00× |
| carrier_disp_madd_ifchainasc | 305839 | 1334699 | 0.229 | 0.99× |
| carrier_disp_madd_ifchainlin | 320150 | 1583860 | 0.202 | 1.04× |
| carrier_disp_madd_nullfloor | 313006 | 1363943 | 0.229 | 1.01× |
| carrier_disp_madd_switch | 308601 | 1312615 | 0.235 | 1.00× |
| carrier_disp_madd_threaded | 327270 | 1802236 | 0.182 | 1.06× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.023 | 86.0% |
| carrier_disp_madd_fntable | 0.020 | 75.0% |
| carrier_disp_madd_ifchain | 0.022 | 80.0% |
| carrier_disp_madd_ifchainasc | 0.022 | 82.1% |
| carrier_disp_madd_ifchainlin | 0.025 | 91.4% |
| carrier_disp_madd_nullfloor | 0.026 | 95.3% |
| carrier_disp_madd_switch | 0.022 | 81.7% |
| carrier_disp_madd_threaded | 0.021 | 79.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 13773ns | 13773ns | -1.99% |
| carrier_disp_madd_fntable | 15259ns | 15259ns | +8.58% |
| carrier_disp_madd_ifchain | 14266ns | 14266ns | +1.52% |
| carrier_disp_madd_ifchainasc | 13983ns | 13983ns | -0.49% |
| carrier_disp_madd_ifchainlin | 12980ns | 12980ns | -7.63% |
| carrier_disp_madd_nullfloor | 12399ns | 12399ns | -11.77% |
| carrier_disp_madd_switch | 14053ns | 14053ns | base |
| carrier_disp_madd_threaded | 14584ns | 14584ns | +3.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 11620ns | base | --- | [10911, 12499] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 11040ns | no significant difference | [-716, +110]ns | [10402, 12458] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_madd_fntable | 12667ns | +1123.1ns (+9.7%) | [+901, +1530]ns | [12032, 13884] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 11868ns | +273.5ns (+2.4%) | [+81, +336]ns | [11200, 12651] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_madd_ifchainasc | 11565ns | no significant difference | [-241, +69]ns | [10975, 12348] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| carrier_disp_madd_ifchainlin | 10395ns | -721.0ns (-6.2%) | [-1607, -521]ns | [10316, 11470] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_nullfloor | 9966ns | -1665.9ns (-14.3%) | [-1984, -1398]ns | [9500, 10515] | YES (adj: no) | 0.0729 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 12014ns | no significant difference | [-124, +1014]ns | [11822, 12849] | no | 0.2552 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 10922ns | -4.8% | +10.2% | +2.8% | +0.6% | -5.3% | -13.0% | +9.4% |
| 2 | 12598ns | +4.3% | +5.4% | -0.2% | -3.7% | -18.4% | -15.8% | -4.1% |
| 3 | 10900ns | -2.9% | +10.3% | +3.2% | +0.6% | -4.3% | -12.9% | +9.2% |
| 4 | 12308ns | -4.3% | +13.5% | +1.5% | -0.1% | -7.0% | -15.5% | +5.9% |
| 5 | 10932ns | -4.8% | +10.3% | +2.2% | +0.6% | -5.2% | -12.8% | +7.4% |
| 6 | 12400ns | -7.3% | +11.3% | +2.6% | +0.0% | -7.3% | -16.0% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.681 | HIGH- (thermal bounce) |
| carrier_disp_madd_fntable | -0.760 | HIGH- (thermal bounce) |
| carrier_disp_madd_ifchain | -0.799 | HIGH- (thermal bounce) |
| carrier_disp_madd_ifchainasc | -0.786 | HIGH- (thermal bounce) |
| carrier_disp_madd_ifchainlin | -0.273 | moderate- |
| carrier_disp_madd_nullfloor | -0.836 | HIGH- (thermal bounce) |
| carrier_disp_madd_switch | -0.832 | HIGH- (thermal bounce) |
| carrier_disp_madd_threaded | -0.607 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 5/6, lost 1/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 1/6, lost 5/6
- **carrier_disp_madd_ifchainasc**: won 2/6, lost 3/6
- **carrier_disp_madd_ifchainlin**: won 6/6, lost 0/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 93379.6ns | 11300.0ns | 826.4% | HIGH |
| carrier_disp_madd_fntable | 88617.8ns | 12861.1ns | 689.0% | HIGH |
| carrier_disp_madd_ifchain | 89384.0ns | 11906.7ns | 750.7% | HIGH |
| carrier_disp_madd_ifchainasc | 89035.5ns | 11629.0ns | 765.6% | HIGH |
| carrier_disp_madd_ifchainlin | 92626.3ns | 10727.0ns | 863.5% | HIGH |
| carrier_disp_madd_nullfloor | 92656.9ns | 9993.9ns | 927.1% | HIGH |
| carrier_disp_madd_switch | 90489.8ns | 11676.5ns | 775.0% | HIGH |
| carrier_disp_madd_threaded | 94213.9ns | 12228.6ns | 770.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 10392.5-12458.0 ns)
  10392.5 |########################################
  10495.8 |####################
  10599.0 |
  10702.3 |
  10805.6 |
  10908.9 |
  11012.1 |
  11115.4 |
  11218.7 |
  11322.0 |
  11425.2 |####################
  11528.5 |
  11631.8 |
  11735.0 |####################
  11838.3 |
  11941.6 |
  12044.9 |
  12148.1 |
  12251.4 |
  12354.7 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 12024.2-13884.0 ns)
  12024.2 |########################################
  12117.2 |
  12210.2 |
  12303.2 |
  12396.2 |
  12489.1 |
  12582.1 |
  12675.1 |
  12768.1 |
  12861.1 |
  12954.1 |
  13047.1 |
  13140.1 |
  13233.0 |#############
  13326.0 |
  13419.0 |
  13512.0 |
  13605.0 |
  13698.0 |
  13791.0 |#############
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 11172.1-12651.2 ns)
  11172.1 |########################################
  11246.1 |
  11320.0 |
  11394.0 |
  11467.9 |
  11541.9 |
  11615.8 |
  11689.8 |
  11763.8 |
  11837.7 |
  11911.7 |
  11985.6 |
  12059.6 |
  12133.5 |
  12207.5 |
  12281.5 |
  12355.4 |
  12429.4 |#############
  12503.3 |#############
  12577.3 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 10966.7-12347.5 ns)
  10966.7 |########################################
  11035.7 |
  11104.8 |
  11173.8 |
  11242.9 |
  11311.9 |
  11380.9 |
  11450.0 |
  11519.0 |
  11588.1 |
  11657.1 |
  11726.1 |
  11795.2 |
  11864.2 |
  11933.3 |
  12002.3 |
  12071.3 |#############
  12140.4 |
  12209.4 |
  12278.5 |#############
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 10284.6-11470.4 ns)
  10284.6 |####################
  10343.9 |########################################
  10403.2 |####################
  10462.5 |
  10521.8 |
  10581.1 |
  10640.3 |
  10699.6 |
  10758.9 |
  10818.2 |
  10877.5 |
  10936.8 |
  10996.1 |
  11055.4 |
  11114.7 |
  11174.0 |
  11233.2 |
  11292.5 |
  11351.8 |
  11411.1 |####################
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 9498.3-10515.2 ns)
   9498.3 |########################################
   9549.1 |
   9600.0 |
   9650.8 |
   9701.7 |
   9752.5 |
   9803.4 |
   9854.2 |
   9905.1 |
   9955.9 |
  10006.8 |
  10057.6 |
  10108.4 |
  10159.3 |
  10210.1 |
  10261.0 |
  10311.8 |
  10362.7 |#############
  10413.5 |#############
  10464.4 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 10899.6-12498.8 ns)
  10899.6 |########################################
  10979.6 |
  11059.5 |
  11139.5 |
  11219.4 |
  11299.4 |
  11379.3 |
  11459.3 |
  11539.3 |
  11619.2 |
  11699.2 |
  11779.1 |
  11859.1 |
  11939.0 |
  12019.0 |
  12099.0 |
  12178.9 |
  12258.9 |#############
  12338.8 |#############
  12418.8 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 11741.2-12849.0 ns)
  11741.2 |########################################
  11796.6 |
  11852.0 |########################################
  11907.4 |########################################
  11962.8 |
  12018.1 |
  12073.5 |########################################
  12128.9 |
  12184.3 |
  12239.7 |
  12295.1 |
  12350.5 |
  12405.9 |
  12461.2 |
  12516.6 |
  12572.0 |
  12627.4 |########################################
  12682.8 |
  12738.2 |
  12793.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: bridge=839.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=696.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=753.0% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=761.7% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=890.9% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=931.0% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=772.9% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=781.7% of algo (FFI overhead may distort results)
