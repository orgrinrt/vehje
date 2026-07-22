# Predecoded dispatch shape, leaf profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_leaf_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_leaf_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_leaf_null dominates: 34% faster than the next best (carrier_pre_leaf_direct)

carrier_pre_leaf_null (478.85 us) leads carrier_pre_leaf_direct (639.79 us) by 34%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_leaf_null beats baseline by 41% (significant)

carrier_pre_leaf_null is -330.30 us (41%) faster than baseline carrier_pre_leaf_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_leaf_fntable is an outlier: 2.1x slower than the field

carrier_pre_leaf_fntable (1.02 ms) is 2.1x the fastest (478.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_leaf_null is fastest but the noisiest (CV 5.7%)

carrier_pre_leaf_null wins on median (478.85 us) yet has the highest variance (CV 5.7%), while carrier_pre_leaf_fntable is the steadiest (CV 0.4%, 1.02 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {carrier_pre_leaf_null} vs {carrier_pre_leaf_direct, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache, carrier_pre_leaf_fntable} (34% apart)

The field splits into a fast tier {carrier_pre_leaf_null} and a slow tier {carrier_pre_leaf_direct, carrier_pre_leaf_threaded, carrier_pre_leaf_switch, carrier_pre_leaf_regcache, carrier_pre_leaf_fntable} with a 34% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_leaf_null** at 478852.9 ns median (-41.3% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.12x (fastest 478852.9 ns, slowest 1015534.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 642870ns | 642149ns | 638340ns | 641314ns | 647470ns | -21.30% |
| carrier_pre_leaf_fntable | 1019564ns | 1017832ns | 1015616ns | 1017254ns | 1025004ns | +24.82% |
| carrier_pre_leaf_null | 481692ns | 481124ns | 435098ns | 476415ns | 512904ns | -41.03% |
| carrier_pre_leaf_regcache | 830067ns | 836091ns | 806116ns | 828981ns | 843673ns | +1.62% |
| carrier_pre_leaf_switch | 816834ns | 817797ns | 811068ns | 815781ns | 821296ns | base |
| carrier_pre_leaf_threaded | 782743ns | 786230ns | 769927ns | 781900ns | 790415ns | -4.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_leaf_direct | 640559ns | 636088ns | 645184ns | -21.36% | 0.026 |
| carrier_pre_leaf_fntable | 1017229ns | 1013311ns | 1022605ns | +24.89% | 0.016 |
| carrier_pre_leaf_null | 479360ns | 432805ns | 510444ns | -41.15% | 0.034 |
| carrier_pre_leaf_regcache | 827719ns | 803751ns | 841388ns | +1.62% | 0.020 |
| carrier_pre_leaf_switch | 814522ns | 808804ns | 818939ns | base | 0.020 |
| carrier_pre_leaf_threaded | 780429ns | 767630ns | 788122ns | -4.19% | 0.021 |

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_pre_leaf_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_leaf_direct | 0.026 | 67.6% |
| carrier_pre_leaf_fntable | 0.016 | 42.6% |
| carrier_pre_leaf_null | 0.034 | 90.4% |
| carrier_pre_leaf_regcache | 0.020 | 51.9% |
| carrier_pre_leaf_switch | 0.020 | 53.1% |
| carrier_pre_leaf_threaded | 0.021 | 55.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_leaf_direct | 642870ns | 642870ns | -21.30% |
| carrier_pre_leaf_fntable | 1019564ns | 1019564ns | +24.82% |
| carrier_pre_leaf_null | 481692ns | 481692ns | -41.03% |
| carrier_pre_leaf_regcache | 830067ns | 830067ns | +1.62% |
| carrier_pre_leaf_switch | 816834ns | 816834ns | base |
| carrier_pre_leaf_threaded | 782743ns | 782743ns | -4.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_leaf_switch | 815476ns | base | --- | [809153, 818939] | --- | --- | --- | --- |
| carrier_pre_leaf_direct | 639794ns | -172667.7ns (-21.2%) | [-180751, -168471]ns | [636701, 645184] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_fntable | 1015534ns | +199565.5ns (+24.5%) | [+197038, +211515]ns | [1013547, 1022605] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_null | 478853ns | -330299.8ns (-40.5%) | [-368865, -306322]ns | [448785, 510444] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_leaf_regcache | 833687ns | no significant difference | [-7395, +30206]ns | [808081, 841388] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_leaf_threaded | 783919ns | -34246.7ns (-4.2%) | [-42629, -25405]ns | [769245, 788122] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_leaf_switch | carrier_pre_leaf_direct | carrier_pre_leaf_fntable | carrier_pre_leaf_null | carrier_pre_leaf_regcache | carrier_pre_leaf_threaded |
|---|---|---|---|---|---|---|
| 1 | 819285ns | -22.1% | +24.3% | -37.5% | +1.8% | -3.9% |
| 2 | 808804ns | -21.2% | +25.5% | -41.9% | +4.5% | -2.8% |
| 3 | 818592ns | -21.0% | +23.8% | -47.1% | +2.3% | -4.5% |
| 4 | 816706ns | -22.1% | +24.4% | -43.1% | -0.5% | -3.4% |
| 5 | 814246ns | -21.3% | +24.5% | -37.5% | -1.3% | -5.7% |
| 6 | 809502ns | -20.4% | +26.8% | -39.8% | +2.9% | -4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_leaf_direct | -0.470 | moderate- |
| carrier_pre_leaf_fntable | -0.156 | ok |
| carrier_pre_leaf_null | 0.138 | ok |
| carrier_pre_leaf_regcache | 0.289 | moderate+ |
| carrier_pre_leaf_switch | -0.400 | moderate- |
| carrier_pre_leaf_threaded | 0.185 | ok |

**Consistency summary:**

- **carrier_pre_leaf_direct**: won 6/6, lost 0/6
- **carrier_pre_leaf_fntable**: won 0/6, lost 6/6
- **carrier_pre_leaf_null**: won 6/6, lost 0/6
- **carrier_pre_leaf_regcache**: won 2/6, lost 4/6
- **carrier_pre_leaf_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_leaf_direct | 698264.5ns | 640559.4ns | 109.0% | HIGH |
| carrier_pre_leaf_fntable | 1066516.0ns | 1017228.8ns | 104.8% | HIGH |
| carrier_pre_leaf_null | 532397.9ns | 479360.3ns | 111.1% | HIGH |
| carrier_pre_leaf_regcache | 874392.9ns | 827718.7ns | 105.6% | HIGH |
| carrier_pre_leaf_switch | 887577.3ns | 814522.5ns | 109.0% | HIGH |
| carrier_pre_leaf_threaded | 827541.9ns | 780429.0ns | 106.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_leaf_direct (n=6, range 636087.9-645183.6 ns)
  636087.9 |########################################
  636542.7 |
  636997.5 |########################################
  637452.2 |
  637907.0 |
  638361.8 |########################################
  638816.6 |
  639271.4 |
  639726.2 |
  640180.9 |
  640635.7 |
  641090.5 |########################################
  641545.3 |
  642000.1 |
  642454.9 |
  642909.6 |
  643364.4 |
  643819.2 |########################################
  644274.0 |
  644728.8 |
  (0 below, 1 above range)

carrier_pre_leaf_fntable (n=6, range 1013310.8-1022605.4 ns)
  1013310.8 |########################################
  1013775.5 |########################################
  1014240.3 |
  1014705.0 |########################################
  1015169.7 |
  1015634.5 |
  1016099.2 |########################################
  1016563.9 |
  1017028.6 |
  1017493.4 |
  1017958.1 |
  1018422.8 |########################################
  1018887.6 |
  1019352.3 |
  1019817.0 |
  1020281.8 |
  1020746.5 |
  1021211.2 |
  1021675.9 |
  1022140.7 |
  (0 below, 1 above range)

carrier_pre_leaf_null (n=6, range 432805.4-510443.5 ns)
  432805.4 |########################################
  436687.3 |
  440569.2 |
  444451.1 |
  448333.0 |
  452214.9 |
  456096.8 |
  459978.8 |
  463860.7 |########################################
  467742.6 |########################################
  471624.5 |
  475506.4 |
  479388.3 |
  483270.2 |
  487152.1 |########################################
  491034.0 |
  494915.9 |
  498797.8 |
  502679.7 |
  506561.6 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_regcache (n=6, range 803751.2-841388.2 ns)
  803751.2 |########################################
  805633.0 |
  807514.9 |
  809396.7 |
  811278.6 |########################################
  813160.4 |
  815042.3 |
  816924.1 |
  818806.0 |
  820687.8 |
  822569.7 |
  824451.5 |
  826333.4 |
  828215.2 |
  830097.1 |
  831978.9 |########################################
  833860.8 |########################################
  835742.6 |########################################
  837624.5 |
  839506.3 |
  (0 below, 1 above range)

carrier_pre_leaf_switch (n=6, range 808803.8-818938.6 ns)
  808803.8 |########################################
  809310.5 |########################################
  809817.3 |
  810324.0 |
  810830.8 |
  811337.5 |
  811844.2 |
  812351.0 |
  812857.7 |
  813364.4 |
  813871.2 |########################################
  814377.9 |
  814884.7 |
  815391.4 |
  815898.1 |
  816404.9 |########################################
  816911.6 |
  817418.3 |
  817925.1 |
  818431.8 |########################################
  (0 below, 1 above range)

carrier_pre_leaf_threaded (n=6, range 767630.4-788122.5 ns)
  767630.4 |########################################
  768655.0 |
  769679.6 |
  770704.2 |########################################
  771728.8 |
  772753.4 |
  773778.0 |
  774802.6 |
  775827.2 |
  776851.8 |
  777876.4 |
  778901.1 |
  779925.7 |
  780950.3 |########################################
  781974.9 |
  782999.5 |
  784024.1 |
  785048.7 |########################################
  786073.3 |
  787097.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_leaf_direct**: bridge=109.0% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_fntable**: bridge=104.8% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_null**: bridge=108.5% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_regcache**: bridge=105.7% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_switch**: bridge=108.9% of algo (FFI overhead may distort results)
- **carrier_pre_leaf_threaded**: bridge=106.1% of algo (FFI overhead may distort results)
