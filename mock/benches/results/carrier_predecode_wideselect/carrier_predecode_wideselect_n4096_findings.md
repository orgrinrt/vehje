# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null dominates: 141% faster than the next best (carrier_pre_wideselect_direct)

carrier_pre_wideselect_null (98.05 us) leads carrier_pre_wideselect_direct (235.83 us) by 141%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_wideselect_null beats baseline by 74% (significant)

carrier_pre_wideselect_null is -283.75 us (74%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_wideselect_fntable is an outlier: 4.6x slower than the field

carrier_pre_wideselect_fntable (455.87 us) is 4.6x the fastest (98.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_wideselect_fntable shows alternating (throttle bounce) (autocorr -0.77)

carrier_pre_wideselect_fntable's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_wideselect_null} vs {carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_regcache, carrier_pre_wideselect_switch, carrier_pre_wideselect_fntable} (141% apart)

The field splits into a fast tier {carrier_pre_wideselect_null} and a slow tier {carrier_pre_wideselect_direct, carrier_pre_wideselect_threaded, carrier_pre_wideselect_regcache, carrier_pre_wideselect_switch, carrier_pre_wideselect_fntable} with a 141% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest carrier_pre_wideselect_null (98.05 us) to slowest carrier_pre_wideselect_fntable (455.87 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 98048.8 ns median (-74.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.65x (fastest 98048.8 ns, slowest 455871.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 239266ns | 238050ns | 228338ns | 236698ns | 248584ns | -38.13% |
| carrier_pre_wideselect_fntable | 455503ns | 458183ns | 442972ns | 453590ns | 464638ns | +17.78% |
| carrier_pre_wideselect_null | 100302ns | 100253ns | 97438ns | 99501ns | 102937ns | -74.07% |
| carrier_pre_wideselect_regcache | 381466ns | 380587ns | 370962ns | 378782ns | 390745ns | -1.37% |
| carrier_pre_wideselect_switch | 386754ns | 385142ns | 361180ns | 382815ns | 405448ns | base |
| carrier_pre_wideselect_threaded | 325434ns | 323245ns | 309677ns | 320985ns | 339987ns | -15.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 237062ns | 226162ns | 246380ns | -38.35% | 0.017 |
| carrier_pre_wideselect_fntable | 453279ns | 440801ns | 462456ns | +17.88% | 0.009 |
| carrier_pre_wideselect_null | 98089ns | 95290ns | 100649ns | -74.49% | 0.042 |
| carrier_pre_wideselect_regcache | 379243ns | 368666ns | 388555ns | -1.37% | 0.011 |
| carrier_pre_wideselect_switch | 384530ns | 358994ns | 403272ns | base | 0.011 |
| carrier_pre_wideselect_threaded | 323167ns | 307382ns | 337718ns | -15.96% | 0.013 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 1540228 | 2470256 | 0.624 | 0.62× |
| carrier_pre_wideselect_fntable | 2891052 | 3603296 | 0.802 | 1.16× |
| carrier_pre_wideselect_null | 657073 | 2983533 | 0.220 | 0.26× |
| carrier_pre_wideselect_regcache | 2440565 | 3990534 | 0.612 | 0.98× |
| carrier_pre_wideselect_switch | 2488913 | 2913249 | 0.854 | 1.00× |
| carrier_pre_wideselect_threaded | 2077143 | 3191112 | 0.651 | 0.83× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.017 | 40.4% |
| carrier_pre_wideselect_fntable | 0.009 | 20.9% |
| carrier_pre_wideselect_null | 0.042 | 97.2% |
| carrier_pre_wideselect_regcache | 0.011 | 25.2% |
| carrier_pre_wideselect_switch | 0.011 | 24.9% |
| carrier_pre_wideselect_threaded | 0.013 | 29.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 239266ns | 239266ns | -38.13% |
| carrier_pre_wideselect_fntable | 455503ns | 455503ns | +17.78% |
| carrier_pre_wideselect_null | 100302ns | 100302ns | -74.07% |
| carrier_pre_wideselect_regcache | 381466ns | 381466ns | -1.37% |
| carrier_pre_wideselect_switch | 386754ns | 386754ns | base |
| carrier_pre_wideselect_threaded | 325434ns | 325434ns | -15.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 382930ns | base | --- | [367386, 403272] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 235830ns | -150899.5ns (-39.4%) | [-157188, -134316]ns | [228975, 246380] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 455871ns | +70768.4ns (+18.5%) | [+52599, +82882]ns | [441510, 462456] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 98049ns | -283752.5ns (-74.1%) | [-305628, -269941]ns | [95568, 100649] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 378357ns | no significant difference | [-32454, +20565]ns | [370818, 388555] | no | 1.0000 | 1.0000 | 0 |
| carrier_pre_wideselect_threaded | 320980ns | -59546.5ns (-15.6%) | [-88326, -36216]ns | [310801, 337718] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 401830ns | -37.9% | +13.5% | -76.1% | -8.3% | -19.7% |
| 2 | 375777ns | -39.8% | +22.3% | -74.6% | +1.2% | -15.0% |
| 3 | 376986ns | -37.5% | +17.3% | -74.3% | +2.2% | -16.6% |
| 4 | 388875ns | -39.3% | +19.6% | -73.8% | -3.2% | -11.6% |
| 5 | 358994ns | -35.4% | +22.8% | -72.4% | +9.1% | -7.6% |
| 6 | 404715ns | -39.9% | +12.6% | -75.4% | -7.8% | -24.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.424 | moderate- |
| carrier_pre_wideselect_fntable | -0.772 | HIGH- (thermal bounce) |
| carrier_pre_wideselect_null | 0.340 | moderate+ |
| carrier_pre_wideselect_regcache | -0.383 | moderate- |
| carrier_pre_wideselect_switch | -0.493 | moderate- |
| carrier_pre_wideselect_threaded | -0.128 | ok |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 3/6, lost 3/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 254350.0ns | 237061.7ns | 107.3% | HIGH |
| carrier_pre_wideselect_fntable | 466905.8ns | 453279.1ns | 103.0% | HIGH |
| carrier_pre_wideselect_null | 113046.2ns | 98088.7ns | 115.2% | HIGH |
| carrier_pre_wideselect_regcache | 397272.7ns | 379243.4ns | 104.8% | HIGH |
| carrier_pre_wideselect_switch | 406500.5ns | 384529.5ns | 105.7% | HIGH |
| carrier_pre_wideselect_threaded | 338060.1ns | 323166.6ns | 104.6% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 226161.7-246380.2 ns)
  226161.7 |####################
  227172.6 |
  228183.6 |
  229194.5 |
  230205.4 |
  231216.3 |####################
  232227.2 |
  233238.2 |
  234249.1 |
  235260.0 |########################################
  236271.0 |
  237281.9 |
  238292.8 |
  239303.7 |
  240314.7 |
  241325.6 |
  242336.5 |####################
  243347.4 |
  244358.4 |
  245369.3 |
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 440801.2-462456.5 ns)
  440801.2 |####################
  441884.0 |####################
  442966.7 |
  444049.5 |
  445132.3 |
  446215.0 |
  447297.8 |
  448380.6 |
  449463.3 |
  450546.1 |
  451628.8 |
  452711.6 |
  453794.4 |
  454877.1 |########################################
  455959.9 |
  457042.7 |
  458125.4 |
  459208.2 |####################
  460291.0 |
  461373.7 |
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 95290.4-100649.1 ns)
  95290.4 |########################################
  95558.3 |
  95826.3 |########################################
  96094.2 |
  96362.1 |
  96630.1 |
  96898.0 |########################################
  97166.0 |
  97433.9 |
  97701.8 |
  97969.8 |
  98237.7 |
  98505.6 |
  98773.6 |
  99041.5 |########################################
  99309.5 |########################################
  99577.4 |
  99845.3 |
  100113.3 |
  100381.2 |
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 368666.2-388555.0 ns)
  368666.2 |########################################
  369660.6 |
  370655.1 |
  371649.5 |
  372644.0 |########################################
  373638.4 |
  374632.8 |
  375627.3 |########################################
  376621.7 |
  377616.2 |
  378610.6 |
  379605.0 |########################################
  380599.5 |
  381593.9 |
  382588.4 |
  383582.8 |
  384577.2 |########################################
  385571.7 |
  386566.1 |
  387560.6 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 358994.2-403272.5 ns)
  358994.2 |########################################
  361208.1 |
  363422.0 |
  365635.9 |
  367849.9 |
  370063.8 |
  372277.7 |
  374491.6 |########################################
  376705.5 |########################################
  378919.4 |
  381133.3 |
  383347.3 |
  385561.2 |
  387775.1 |########################################
  389989.0 |
  392202.9 |
  394416.8 |
  396630.8 |
  398844.7 |
  401058.6 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 307381.7-337718.3 ns)
  307381.7 |########################################
  308898.5 |
  310415.4 |
  311932.2 |
  313449.0 |########################################
  314965.9 |
  316482.7 |
  317999.5 |########################################
  319516.4 |
  321033.2 |########################################
  322550.0 |
  324066.9 |
  325583.7 |
  327100.5 |
  328617.4 |
  330134.2 |########################################
  331651.0 |
  333167.9 |
  334684.7 |
  336201.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=106.7% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=102.8% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=115.9% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=104.8% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=106.6% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=104.5% of algo (FFI overhead may distort results)
