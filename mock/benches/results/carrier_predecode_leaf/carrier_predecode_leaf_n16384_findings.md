# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_null dominates: 17% faster than the next best (carrier_pre_leaf_direct)

carrier_pre_leaf_null (552.78 us) leads carrier_pre_leaf_direct (648.17 us) by 17%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_leaf_null beats baseline by 31% (significant)

carrier_pre_leaf_null is -247.55 us (31%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_null is fastest but the noisiest (CV 5.2%)

carrier_pre_leaf_null wins on median (552.78 us) yet has the highest variance (CV 5.2%), while carrier_pre_leaf_regcache is the steadiest (CV 0.1%, 836.53 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_pre_leaf_regcache shows alternating (throttle bounce) (autocorr -0.82)

carrier_pre_leaf_regcache's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_pre_leaf_null** at 552782.1 ns median (-31.1% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.83x (fastest 552782.1 ns, slowest 1010814.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 650979ns | 650332ns | 644745ns | 648600ns | 657665ns | -19.12% |
| carrier_pre_leaf_fntable | 1013567ns | 1012996ns | 1010677ns | 1012279ns | 1016945ns | +25.93% |
| carrier_pre_leaf_null | 536758ns | 554969ns | 486072ns | 539730ns | 557641ns | -33.31% |
| carrier_pre_leaf_regcache | 838743ns | 838721ns | 836796ns | 838497ns | 840086ns | +4.21% |
| carrier_pre_leaf_switch | 804868ns | 804875ns | 793315ns | 803188ns | 813165ns | base |
| carrier_pre_leaf_threaded | 818989ns | 825908ns | 790411ns | 820716ns | 830688ns | +1.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 648757ns | 642570ns | 655315ns | -19.18% | 0.025 |
| carrier_pre_leaf_fntable | 1011344ns | 1008522ns | 1014626ns | +26.00% | 0.016 |
| carrier_pre_leaf_null | 534569ns | 483912ns | 555464ns | -33.40% | 0.031 |
| carrier_pre_leaf_regcache | 836516ns | 834608ns | 837817ns | +4.22% | 0.020 |
| carrier_pre_leaf_switch | 802677ns | 791109ns | 810976ns | base | 0.020 |
| carrier_pre_leaf_threaded | 816750ns | 788202ns | 828532ns | +1.75% | 0.020 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 4269992 | 7943362 | 0.538 | 0.81× |
| carrier_pre_leaf_fntable | 6513492 | 12818748 | 0.508 | 1.23× |
| carrier_pre_leaf_null | 3240238 | 9248182 | 0.350 | 0.61× |
| carrier_pre_leaf_regcache | 5412088 | 12482166 | 0.434 | 1.02× |
| carrier_pre_leaf_switch | 5290914 | 10546468 | 0.502 | 1.00× |
| carrier_pre_leaf_threaded | 5283095 | 10824599 | 0.488 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_pre_leaf_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.025 | 74.7% |
| carrier_pre_leaf_fntable | 0.016 | 47.9% |
| carrier_pre_leaf_null | 0.030 | 87.5% |
| carrier_pre_leaf_regcache | 0.020 | 57.8% |
| carrier_pre_leaf_switch | 0.020 | 60.3% |
| carrier_pre_leaf_threaded | 0.020 | 58.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 650979ns | 650979ns | -19.12% |
| carrier_pre_leaf_fntable | 1013567ns | 1013567ns | +25.93% |
| carrier_pre_leaf_null | 536758ns | 536758ns | -33.31% |
| carrier_pre_leaf_regcache | 838743ns | 838743ns | +4.21% |
| carrier_pre_leaf_switch | 804868ns | 804868ns | base |
| carrier_pre_leaf_threaded | 818989ns | 818989ns | +1.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 802667ns | base | --- | [794388, 810976] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 648174ns | -155671.2ns (-19.4%) | [-166701, -139389]ns | [642781, 655315] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 1010815ns | +207630.0ns (+25.9%) | [+198648, +219721]ns | [1008590, 1014626] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 552782ns | -247549.2ns (-30.8%) | [-315514, -241260]ns | [495462, 555464] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 836527ns | +34679.6ns (+4.3%) | [+24959, +41878]ns | [835204, 837817] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_threaded | 823742ns | no significant difference | [-6185, +29354]ns | [797976, 828532] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 798300ns | -18.6% | +26.4% | -30.7% | +4.9% | +1.2% |
| 2 | 791109ns | -16.5% | +27.8% | -30.2% | +5.5% | +4.1% |
| 3 | 797668ns | -18.7% | +27.5% | -30.5% | +5.0% | +3.3% |
| 4 | 811930ns | -20.8% | +24.5% | -37.6% | +2.9% | +2.2% |
| 5 | 807035ns | -20.4% | +25.4% | -31.0% | +3.7% | +2.5% |
| 6 | 810022ns | -20.0% | +24.5% | -40.3% | +3.2% | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | 0.248 | moderate+ |
| carrier_pre_leaf_fntable | -0.138 | ok |
| carrier_pre_leaf_null | -0.328 | moderate- |
| carrier_pre_leaf_regcache | -0.817 | HIGH- (thermal bounce) |
| carrier_pre_leaf_switch | 0.400 | moderate+ |
| carrier_pre_leaf_threaded | -0.051 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 0/6, lost 6/6
- **carrier_pre_leaf_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 709456.2ns | 648756.7ns | 109.4% | HIGH |
| carrier_pre_leaf_fntable | 1061554.9ns | 1011343.8ns | 105.0% | HIGH |
| carrier_pre_leaf_null | 493604.1ns | 534569.5ns | 92.3% | HIGH |
| carrier_pre_leaf_regcache | 886653.1ns | 836516.2ns | 106.0% | HIGH |
| carrier_pre_leaf_switch | 880969.2ns | 802677.2ns | 109.8% | HIGH |
| carrier_pre_leaf_threaded | 866551.8ns | 816750.0ns | 106.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 642570.4-655315.0 ns)
  642570.4 |########################################
  643207.6 |
  643844.9 |
  644482.1 |
  645119.3 |
  645756.6 |
  646393.8 |
  647031.0 |
  647668.2 |####################
  648305.5 |####################
  648942.7 |
  649579.9 |####################
  650217.2 |
  650854.4 |
  651491.6 |
  652128.8 |
  652766.1 |
  653403.3 |
  654040.5 |
  654677.8 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 1008522.1-1014626.4 ns)
  1008522.1 |########################################
  1008827.3 |
  1009132.5 |
  1009437.8 |
  1009743.0 |
  1010048.2 |
  1010353.4 |
  1010658.6 |########################################
  1010963.8 |
  1011269.1 |
  1011574.3 |
  1011879.5 |####################
  1012184.7 |
  1012489.9 |
  1012795.1 |
  1013100.4 |
  1013405.6 |
  1013710.8 |
  1014016.0 |
  1014321.2 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 483911.7-555463.9 ns)
  483911.7 |#############
  487489.3 |
  491066.9 |
  494644.5 |
  498222.2 |
  501799.8 |
  505377.4 |#############
  508955.0 |
  512532.6 |
  516110.2 |
  519687.8 |
  523265.4 |
  526843.0 |
  530420.7 |
  533998.3 |
  537575.9 |
  541153.5 |
  544731.1 |
  548308.7 |
  551886.3 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 834607.5-837817.3 ns)
  834607.5 |########################################
  834768.0 |
  834928.5 |
  835089.0 |
  835249.5 |
  835409.9 |
  835570.4 |
  835730.9 |########################################
  835891.4 |
  836051.9 |########################################
  836212.4 |
  836372.9 |
  836533.4 |
  836693.9 |
  836854.4 |########################################
  837014.9 |
  837175.3 |
  837335.8 |
  837496.3 |
  837656.8 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 791109.2-810976.2 ns)
  791109.2 |########################################
  792102.6 |
  793095.9 |
  794089.3 |
  795082.6 |
  796076.0 |
  797069.3 |########################################
  798062.7 |########################################
  799056.0 |
  800049.4 |
  801042.7 |
  802036.1 |
  803029.4 |
  804022.8 |
  805016.1 |
  806009.5 |
  807002.8 |########################################
  807996.2 |
  808989.5 |
  809982.9 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 788202.1-828532.3 ns)
  788202.1 |####################
  790218.6 |
  792235.1 |
  794251.6 |
  796268.1 |
  798284.7 |
  800301.2 |
  802317.7 |
  804334.2 |
  806350.7 |####################
  808367.2 |
  810383.7 |
  812400.2 |
  814416.7 |
  816433.2 |
  818449.8 |
  820466.3 |
  822482.8 |########################################
  824499.3 |
  826515.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=109.5% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=105.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=86.4% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=106.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=109.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=106.0% of algo (FFI overhead may distort results)
