# Predecoded dispatch shape, scatter profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_scatter_null dominates: 250% faster than the next best (carrier_pre_scatter_direct)

carrier_pre_scatter_null (100.15 us) leads carrier_pre_scatter_direct (350.73 us) by 250%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_scatter_null beats baseline by 80% (significant)

carrier_pre_scatter_null is -397.19 us (80%) faster than baseline carrier_pre_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_scatter_fntable is an outlier: 5.5x slower than the field

carrier_pre_scatter_fntable (553.16 us) is 5.5x the fastest (100.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_scatter_null} vs {carrier_pre_scatter_direct, carrier_pre_scatter_regcache, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_fntable} (250% apart)

The field splits into a fast tier {carrier_pre_scatter_null} and a slow tier {carrier_pre_scatter_direct, carrier_pre_scatter_regcache, carrier_pre_scatter_threaded, carrier_pre_scatter_switch, carrier_pre_scatter_fntable} with a 250% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.5x the fastest

Fastest carrier_pre_scatter_null (100.15 us) to slowest carrier_pre_scatter_fntable (553.16 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_scatter_null** at 100149.2 ns median (-79.9% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.52x (fastest 100149.2 ns, slowest 553157.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 355817ns | 352938ns | 330856ns | 349203ns | 378217ns | -28.23% |
| carrier_pre_scatter_fntable | 551272ns | 555551ns | 523863ns | 548187ns | 569606ns | +11.19% |
| carrier_pre_scatter_null | 103139ns | 102350ns | 100158ns | 102315ns | 105865ns | -79.20% |
| carrier_pre_scatter_regcache | 473124ns | 471778ns | 461305ns | 470526ns | 482930ns | -4.57% |
| carrier_pre_scatter_switch | 495782ns | 499538ns | 471325ns | 493861ns | 510893ns | base |
| carrier_pre_scatter_threaded | 483041ns | 479793ns | 461632ns | 477522ns | 502025ns | -2.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_scatter_direct | 353522ns | 328525ns | 376008ns | -28.36% | 0.012 |
| carrier_pre_scatter_fntable | 548951ns | 521676ns | 567340ns | +11.24% | 0.007 |
| carrier_pre_scatter_null | 100902ns | 98006ns | 103580ns | -79.55% | 0.041 |
| carrier_pre_scatter_regcache | 470802ns | 459023ns | 480614ns | -4.60% | 0.009 |
| carrier_pre_scatter_switch | 493493ns | 469064ns | 508598ns | base | 0.008 |
| carrier_pre_scatter_threaded | 480673ns | 459298ns | 499651ns | -2.60% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 2251848 | 2283201 | 0.986 | 0.72× |
| carrier_pre_scatter_fntable | 3503408 | 3406256 | 1.029 | 1.12× |
| carrier_pre_scatter_null | 658936 | 2922282 | 0.225 | 0.21× |
| carrier_pre_scatter_regcache | 2987986 | 3925290 | 0.761 | 0.96× |
| carrier_pre_scatter_switch | 3123207 | 2904179 | 1.075 | 1.00× |
| carrier_pre_scatter_threaded | 3051099 | 3003902 | 1.016 | 0.98× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.042 Gops/s** (carrier_pre_scatter_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_scatter_direct | 0.012 | 27.9% |
| carrier_pre_scatter_fntable | 0.007 | 17.7% |
| carrier_pre_scatter_null | 0.041 | 97.9% |
| carrier_pre_scatter_regcache | 0.009 | 20.9% |
| carrier_pre_scatter_switch | 0.008 | 19.7% |
| carrier_pre_scatter_threaded | 0.009 | 20.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_scatter_direct | 355817ns | 355817ns | -28.23% |
| carrier_pre_scatter_fntable | 551272ns | 551272ns | +11.19% |
| carrier_pre_scatter_null | 103139ns | 103139ns | -79.20% |
| carrier_pre_scatter_regcache | 473124ns | 473124ns | -4.57% |
| carrier_pre_scatter_switch | 495782ns | 495782ns | base |
| carrier_pre_scatter_threaded | 483041ns | 483041ns | -2.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_scatter_switch | 497275ns | base | --- | [474606, 508598] | --- | --- | --- | --- |
| carrier_pre_scatter_direct | 350732ns | -140664.8ns (-28.3%) | [-172662, -106586]ns | [333826, 376008] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_fntable | 553158ns | +62822.3ns (+12.6%) | [+25000, +78552]ns | [526354, 567340] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_null | 100149ns | -397190.6ns (-79.9%) | [-409556, -371026]ns | [98976, 103580] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_scatter_regcache | 469509ns | -16661.0ns (-3.4%) | [-45327, -6085]ns | [462283, 480614] | YES (adj: no) | 0.2734 | 0.2188 | 0 |
| carrier_pre_scatter_threaded | 477389ns | no significant difference | [-30122, +7992]ns | [464978, 499651] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_scatter_switch | carrier_pre_scatter_direct | carrier_pre_scatter_fntable | carrier_pre_scatter_null | carrier_pre_scatter_regcache | carrier_pre_scatter_threaded |
|---|---|---|---|---|---|---|
| 1 | 499245ns | -32.1% | +6.4% | -79.9% | -3.1% | -5.5% |
| 2 | 469064ns | -26.6% | +18.8% | -77.9% | +0.5% | +3.0% |
| 3 | 495305ns | -27.3% | +13.7% | -79.8% | -3.6% | +0.4% |
| 4 | 513731ns | -36.1% | +11.3% | -80.9% | -10.6% | -2.3% |
| 5 | 503464ns | -29.0% | +3.6% | -80.1% | -7.1% | -6.5% |
| 6 | 480148ns | -18.4% | +14.3% | -78.4% | -3.0% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_scatter_direct | -0.019 | ok |
| carrier_pre_scatter_fntable | -0.180 | ok |
| carrier_pre_scatter_null | -0.054 | ok |
| carrier_pre_scatter_regcache | -0.024 | ok |
| carrier_pre_scatter_switch | -0.060 | ok |
| carrier_pre_scatter_threaded | 0.273 | moderate+ |

**Consistency summary:**

- **carrier_pre_scatter_direct**: won 6/6, lost 0/6
- **carrier_pre_scatter_fntable**: won 0/6, lost 6/6
- **carrier_pre_scatter_null**: won 6/6, lost 0/6
- **carrier_pre_scatter_regcache**: won 5/6, lost 1/6
- **carrier_pre_scatter_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_scatter_direct | 368072.3ns | 353522.0ns | 104.1% | HIGH |
| carrier_pre_scatter_fntable | 568335.1ns | 548950.7ns | 103.5% | HIGH |
| carrier_pre_scatter_null | 111503.0ns | 100901.9ns | 110.5% | HIGH |
| carrier_pre_scatter_regcache | 481922.8ns | 470801.9ns | 102.4% | HIGH |
| carrier_pre_scatter_switch | 503223.7ns | 493492.9ns | 102.0% | HIGH |
| carrier_pre_scatter_threaded | 493503.6ns | 480672.7ns | 102.7% | HIGH |

## Distribution (algo ns)

```
carrier_pre_scatter_direct (n=6, range 328524.6-376007.5 ns)
  328524.6 |########################################
  330898.7 |
  333272.9 |
  335647.0 |
  338021.2 |########################################
  340395.3 |
  342769.5 |########################################
  345143.6 |
  347517.8 |
  349891.9 |
  352266.0 |
  354640.2 |
  357014.3 |########################################
  359388.5 |########################################
  361762.6 |
  364136.8 |
  366510.9 |
  368885.1 |
  371259.2 |
  373633.4 |
  (0 below, 1 above range)

carrier_pre_scatter_fntable (n=6, range 521676.2-567340.2 ns)
  521676.2 |########################################
  523959.4 |
  526242.6 |
  528525.8 |
  530809.0 |########################################
  533092.2 |
  535375.4 |
  537658.6 |
  539941.8 |
  542225.0 |
  544508.2 |
  546791.4 |########################################
  549074.6 |
  551357.8 |
  553641.0 |
  555924.2 |########################################
  558207.4 |
  560490.6 |
  562773.8 |########################################
  565057.0 |
  (0 below, 1 above range)

carrier_pre_scatter_null (n=6, range 98006.2-103580.4 ns)
  98006.2 |####################
  98284.9 |
  98563.6 |
  98842.3 |
  99121.0 |
  99399.8 |
  99678.5 |####################
  99957.2 |########################################
  100235.9 |
  100514.6 |
  100793.3 |
  101072.0 |
  101350.7 |
  101629.4 |
  101908.1 |
  102186.8 |
  102465.6 |
  102744.3 |
  103023.0 |
  103301.7 |####################
  (0 below, 1 above range)

carrier_pre_scatter_regcache (n=6, range 459023.3-480613.6 ns)
  459023.3 |########################################
  460102.8 |
  461182.3 |
  462261.8 |
  463341.3 |
  464420.9 |
  465500.4 |########################################
  466579.9 |########################################
  467659.4 |
  468738.9 |
  469818.4 |
  470897.9 |########################################
  471977.5 |
  473057.0 |
  474136.5 |
  475216.0 |
  476295.5 |
  477375.0 |########################################
  478454.5 |
  479534.0 |
  (0 below, 1 above range)

carrier_pre_scatter_switch (n=6, range 469064.2-508597.7 ns)
  469064.2 |########################################
  471040.9 |
  473017.5 |
  474994.2 |
  476970.9 |
  478947.6 |########################################
  480924.2 |
  482900.9 |
  484877.6 |
  486854.3 |
  488831.0 |
  490807.6 |
  492784.3 |
  494761.0 |########################################
  496737.7 |
  498714.3 |########################################
  500691.0 |
  502667.7 |########################################
  504644.4 |
  506621.0 |
  (0 below, 1 above range)

carrier_pre_scatter_threaded (n=6, range 459297.5-499651.5 ns)
  459297.5 |########################################
  461315.2 |
  463332.9 |
  465350.6 |
  467368.3 |
  469386.0 |########################################
  471403.7 |########################################
  473421.4 |
  475439.1 |
  477456.8 |
  479474.5 |
  481492.2 |########################################
  483509.9 |
  485527.6 |
  487545.3 |
  489563.0 |
  491580.7 |
  493598.4 |
  495616.1 |########################################
  497633.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_scatter_direct**: bridge=103.9% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_fntable**: bridge=103.3% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_null**: bridge=110.2% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_regcache**: bridge=102.6% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_switch**: bridge=102.0% of algo (FFI overhead may distort results)
- **carrier_pre_scatter_threaded**: bridge=102.7% of algo (FFI overhead may distort results)
