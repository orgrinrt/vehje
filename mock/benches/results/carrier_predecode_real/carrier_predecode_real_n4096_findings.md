# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 279% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (103.19 us) leads carrier_pre_real_direct (391.32 us) by 279%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 80% (significant)

carrier_pre_real_null is -397.38 us (80%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 5.6x slower than the field

carrier_pre_real_fntable (573.94 us) is 5.6x the fastest (103.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_pre_real_null shows alternating (throttle bounce) (autocorr -0.79)

carrier_pre_real_null's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_real_null} vs {carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache, carrier_pre_real_fntable} (279% apart)

The field splits into a fast tier {carrier_pre_real_null} and a slow tier {carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache, carrier_pre_real_fntable} with a 279% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.6x the fastest

Fastest carrier_pre_real_null (103.19 us) to slowest carrier_pre_real_fntable (573.94 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_real_null** at 103190.9 ns median (-79.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.56x (fastest 103190.9 ns, slowest 573938.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 393189ns | 393456ns | 374309ns | 391472ns | 405205ns | -21.45% |
| carrier_pre_real_fntable | 573090ns | 576148ns | 560909ns | 571268ns | 581913ns | +14.49% |
| carrier_pre_real_null | 104877ns | 105411ns | 100898ns | 104216ns | 107859ns | -79.05% |
| carrier_pre_real_regcache | 507889ns | 504946ns | 485903ns | 501719ns | 528138ns | +1.47% |
| carrier_pre_real_switch | 500551ns | 501281ns | 489793ns | 497467ns | 510557ns | base |
| carrier_pre_real_threaded | 496994ns | 498181ns | 453430ns | 492074ns | 526158ns | -0.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 391015ns | 372146ns | 403061ns | -21.54% | 0.010 |
| carrier_pre_real_fntable | 570872ns | 558504ns | 579757ns | +14.54% | 0.007 |
| carrier_pre_real_null | 102664ns | 98769ns | 105582ns | -79.40% | 0.040 |
| carrier_pre_real_regcache | 505683ns | 483505ns | 525951ns | +1.46% | 0.008 |
| carrier_pre_real_switch | 498383ns | 487638ns | 508351ns | base | 0.008 |
| carrier_pre_real_threaded | 494830ns | 451268ns | 524007ns | -0.71% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_real_direct | 2502159 | 2240725 | 1.117 | 0.79× |
| carrier_pre_real_fntable | 3567826 | 3377883 | 1.056 | 1.13× |
| carrier_pre_real_null | 668636 | 2904234 | 0.230 | 0.21× |
| carrier_pre_real_regcache | 3211702 | 3868361 | 0.830 | 1.02× |
| carrier_pre_real_switch | 3160575 | 2880986 | 1.097 | 1.00× |
| carrier_pre_real_threaded | 3145464 | 2961853 | 1.062 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.010 | 25.2% |
| carrier_pre_real_fntable | 0.007 | 17.2% |
| carrier_pre_real_null | 0.040 | 95.7% |
| carrier_pre_real_regcache | 0.008 | 19.6% |
| carrier_pre_real_switch | 0.008 | 19.8% |
| carrier_pre_real_threaded | 0.008 | 19.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 393189ns | 393189ns | -21.45% |
| carrier_pre_real_fntable | 573090ns | 573090ns | +14.49% |
| carrier_pre_real_null | 104877ns | 104877ns | -79.05% |
| carrier_pre_real_regcache | 507889ns | 507889ns | +1.47% |
| carrier_pre_real_switch | 500551ns | 500551ns | base |
| carrier_pre_real_threaded | 496994ns | 496994ns | -0.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 499135ns | base | --- | [487665, 508351] | --- | --- | --- | --- |
| carrier_pre_real_direct | 391317ns | -109532.9ns (-21.9%) | [-121339, -91235]ns | [378665, 403061] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_real_fntable | 573939ns | +75687.3ns (+15.2%) | [+57464, +84314]ns | [558920, 579757] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_real_null | 103191ns | -397380.6ns (-79.6%) | [-405160, -384618]ns | [99219, 105582] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_real_regcache | 502780ns | no significant difference | [-13162, +22735]ns | [488317, 525951] | no | 0.8594 | 0.6875 | 0 |
| carrier_pre_real_threaded | 495994ns | no significant difference | [-30954, +25856]ns | [464489, 524007] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 508665ns | -23.6% | +12.7% | -79.7% | +4.1% | +2.5% |
| 2 | 495076ns | -17.9% | +16.1% | -78.8% | -0.4% | -2.3% |
| 3 | 503194ns | -20.6% | +15.9% | -80.4% | +3.8% | -5.1% |
| 4 | 487638ns | -23.7% | +14.7% | -78.2% | +5.1% | +8.0% |
| 5 | 487692ns | -19.3% | +18.2% | -79.6% | +1.1% | -7.5% |
| 6 | 508037ns | -24.2% | +9.9% | -79.7% | -4.8% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | -0.183 | ok |
| carrier_pre_real_fntable | -0.442 | moderate- |
| carrier_pre_real_null | -0.785 | HIGH- (thermal bounce) |
| carrier_pre_real_regcache | -0.116 | ok |
| carrier_pre_real_switch | -0.194 | ok |
| carrier_pre_real_threaded | -0.620 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 2/6, lost 4/6
- **carrier_pre_real_threaded**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 405403.9ns | 391014.6ns | 103.7% | HIGH |
| carrier_pre_real_fntable | 564596.9ns | 570871.9ns | 98.9% | HIGH |
| carrier_pre_real_null | 114414.7ns | 102664.0ns | 111.4% | HIGH |
| carrier_pre_real_regcache | 516672.4ns | 505682.6ns | 102.2% | HIGH |
| carrier_pre_real_switch | 505759.1ns | 498383.5ns | 101.5% | HIGH |
| carrier_pre_real_threaded | 505375.2ns | 494829.9ns | 102.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 372145.8-403061.5 ns)
  372145.8 |########################################
  373691.6 |
  375237.4 |
  376783.1 |
  378328.9 |
  379874.7 |
  381420.5 |
  382966.3 |
  384512.1 |########################################
  386057.8 |
  387603.6 |########################################
  389149.4 |
  390695.2 |
  392241.0 |
  393786.8 |########################################
  395332.5 |
  396878.3 |
  398424.1 |########################################
  399969.9 |
  401515.7 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 558504.2-579756.9 ns)
  558504.2 |########################################
  559566.8 |
  560629.5 |
  561692.1 |
  562754.7 |
  563817.4 |
  564880.0 |
  565942.6 |
  567005.3 |
  568067.9 |
  569130.5 |
  570193.2 |
  571255.8 |
  572318.5 |####################
  573381.1 |
  574443.7 |####################
  575506.4 |####################
  576569.0 |
  577631.6 |
  578694.3 |
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 98768.7-105582.3 ns)
  98768.7 |########################################
  99109.4 |
  99450.1 |########################################
  99790.7 |
  100131.4 |
  100472.1 |
  100812.8 |
  101153.5 |
  101494.1 |
  101834.8 |
  102175.5 |
  102516.2 |
  102856.9 |########################################
  103197.5 |########################################
  103538.2 |
  103878.9 |
  104219.6 |
  104560.3 |########################################
  104900.9 |
  105241.6 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 483505.4-525951.1 ns)
  483505.4 |####################
  485627.7 |
  487750.0 |
  489872.2 |
  491994.5 |########################################
  494116.8 |
  496239.1 |
  498361.4 |
  500483.7 |
  502605.9 |
  504728.2 |
  506850.5 |
  508972.8 |
  511095.1 |####################
  513217.4 |
  515339.6 |
  517461.9 |
  519584.2 |
  521706.5 |####################
  523828.8 |
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 487637.9-508350.8 ns)
  487637.9 |########################################
  488673.5 |
  489709.2 |
  490744.8 |
  491780.5 |
  492816.1 |
  493851.8 |
  494887.4 |####################
  495923.1 |
  496958.7 |
  497994.4 |
  499030.0 |
  500065.7 |
  501101.3 |
  502137.0 |
  503172.6 |####################
  504208.3 |
  505243.9 |
  506279.6 |
  507315.2 |####################
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 451267.9-524007.1 ns)
  451267.9 |########################################
  454904.9 |
  458541.8 |
  462178.8 |
  465815.7 |
  469452.7 |
  473089.7 |
  476726.6 |########################################
  480363.6 |########################################
  484000.5 |
  487637.5 |
  491274.5 |
  494911.4 |
  498548.4 |
  502185.3 |
  505822.3 |########################################
  509459.3 |
  513096.2 |
  516733.2 |
  520370.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=103.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=98.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=111.5% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=101.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=102.3% of algo (FFI overhead may distort results)
