# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 268% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (102.85 us) leads carrier_pre_real_direct (378.88 us) by 268%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 79% (significant)

carrier_pre_real_null is -386.04 us (79%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 5.3x slower than the field

carrier_pre_real_fntable (542.75 us) is 5.3x the fastest (102.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_real_null} vs {carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache, carrier_pre_real_fntable} (268% apart)

The field splits into a fast tier {carrier_pre_real_null} and a slow tier {carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache, carrier_pre_real_fntable} with a 268% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.3x the fastest

Fastest carrier_pre_real_null (102.85 us) to slowest carrier_pre_real_fntable (542.75 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_pre_real_null** at 102847.1 ns median (-78.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.28x (fastest 102847.1 ns, slowest 542749.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 378441ns | 381102ns | 356912ns | 376185ns | 392590ns | -22.33% |
| carrier_pre_real_fntable | 547678ns | 545058ns | 524474ns | 542226ns | 567457ns | +12.41% |
| carrier_pre_real_null | 104786ns | 105109ns | 100951ns | 104184ns | 107605ns | -78.49% |
| carrier_pre_real_regcache | 507775ns | 509613ns | 465671ns | 502763ns | 536347ns | +4.22% |
| carrier_pre_real_switch | 487236ns | 489344ns | 445881ns | 482807ns | 514556ns | base |
| carrier_pre_real_threaded | 461944ns | 465115ns | 391089ns | 451730ns | 512692ns | -5.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 376223ns | 354567ns | 390426ns | -22.43% | 0.011 |
| carrier_pre_real_fntable | 545354ns | 522202ns | 565065ns | +12.44% | 0.008 |
| carrier_pre_real_null | 102527ns | 98770ns | 105273ns | -78.86% | 0.040 |
| carrier_pre_real_regcache | 505534ns | 463340ns | 534100ns | +4.23% | 0.008 |
| carrier_pre_real_switch | 485005ns | 443692ns | 512342ns | base | 0.008 |
| carrier_pre_real_threaded | 459773ns | 388919ns | 510519ns | -5.20% | 0.009 |

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.011 | 26.1% |
| carrier_pre_real_fntable | 0.008 | 18.2% |
| carrier_pre_real_null | 0.040 | 96.0% |
| carrier_pre_real_regcache | 0.008 | 19.5% |
| carrier_pre_real_switch | 0.008 | 20.3% |
| carrier_pre_real_threaded | 0.009 | 21.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 378441ns | 378441ns | -22.33% |
| carrier_pre_real_fntable | 547678ns | 547678ns | +12.41% |
| carrier_pre_real_null | 104786ns | 104786ns | -78.49% |
| carrier_pre_real_regcache | 507775ns | 507775ns | +4.22% |
| carrier_pre_real_switch | 487236ns | 487236ns | base |
| carrier_pre_real_threaded | 461944ns | 461944ns | -5.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 487104ns | base | --- | [455568, 512342] | --- | --- | --- | --- |
| carrier_pre_real_direct | 378879ns | -113703.7ns (-23.3%) | [-130957, -81685]ns | [359363, 390426] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_real_fntable | 542750ns | +52438.8ns (+10.8%) | [+37834, +90776]ns | [528249, 565065] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_real_null | 102847ns | -386037.3ns (-79.3%) | [-410632, -350765]ns | [99459, 105273] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_real_regcache | 507388ns | no significant difference | [-11050, +60959]ns | [475114, 534100] | no | 0.2734 | 0.2188 | 0 |
| carrier_pre_real_threaded | 462945ns | no significant difference | [-75553, +49254]ns | [405853, 510519] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 467443ns | -24.1% | +14.3% | -77.8% | +16.1% | -16.8% |
| 2 | 495369ns | -23.6% | +9.8% | -79.4% | +0.7% | -14.7% |
| 3 | 515611ns | -22.2% | +10.9% | -79.7% | +0.0% | -8.7% |
| 4 | 478839ns | -20.8% | +9.1% | -79.1% | +9.8% | +5.9% |
| 5 | 509072ns | -28.5% | +6.3% | -80.6% | -4.4% | -10.5% |
| 6 | 443692ns | -14.4% | +25.9% | -76.1% | +4.4% | +15.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | 0.004 | ok |
| carrier_pre_real_fntable | -0.382 | moderate- |
| carrier_pre_real_null | -0.283 | moderate- |
| carrier_pre_real_regcache | 0.077 | ok |
| carrier_pre_real_switch | -0.325 | moderate- |
| carrier_pre_real_threaded | 0.195 | ok |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 1/6, lost 4/6
- **carrier_pre_real_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 391395.4ns | 376222.7ns | 104.0% | HIGH |
| carrier_pre_real_fntable | 576233.1ns | 545354.4ns | 105.7% | HIGH |
| carrier_pre_real_null | 112900.5ns | 102526.5ns | 110.1% | HIGH |
| carrier_pre_real_regcache | 516518.8ns | 505534.0ns | 102.2% | HIGH |
| carrier_pre_real_switch | 527961.0ns | 485004.6ns | 108.9% | HIGH |
| carrier_pre_real_threaded | 469339.0ns | 459772.5ns | 102.1% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 354567.1-390426.2 ns)
  354567.1 |####################
  356360.1 |
  358153.0 |
  359946.0 |
  361738.9 |
  363531.9 |####################
  365324.8 |
  367117.8 |
  368910.8 |
  370703.7 |
  372496.7 |
  374289.6 |
  376082.6 |
  377875.5 |########################################
  379668.5 |####################
  381461.5 |
  383254.4 |
  385047.4 |
  386840.3 |
  388633.3 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 522201.7-565064.6 ns)
  522201.7 |########################################
  524344.8 |
  526488.0 |
  528631.1 |
  530774.3 |
  532917.4 |########################################
  535060.6 |
  537203.7 |
  539346.9 |########################################
  541490.0 |
  543633.2 |########################################
  545776.3 |
  547919.4 |
  550062.6 |
  552205.7 |
  554348.9 |
  556492.0 |########################################
  558635.2 |
  560778.3 |
  562921.5 |
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 98769.6-105273.1 ns)
  98769.6 |########################################
  99094.8 |
  99420.0 |
  99745.1 |
  100070.3 |########################################
  100395.5 |
  100720.7 |
  101045.8 |
  101371.0 |
  101696.2 |########################################
  102021.4 |
  102346.5 |
  102671.7 |
  102996.9 |
  103322.1 |
  103647.2 |########################################
  103972.4 |
  104297.6 |
  104622.8 |########################################
  104947.9 |
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 463339.6-534100.0 ns)
  463339.6 |########################################
  466877.6 |
  470415.6 |
  473953.7 |
  477491.7 |
  481029.7 |
  484567.7 |########################################
  488105.7 |
  491643.8 |
  495181.8 |
  498719.8 |########################################
  502257.8 |
  505795.8 |
  509333.9 |
  512871.9 |########################################
  516409.9 |
  519947.9 |
  523485.9 |########################################
  527024.0 |
  530562.0 |
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 443692.5-512341.8 ns)
  443692.5 |########################################
  447125.0 |
  450557.4 |
  453989.9 |
  457422.4 |
  460854.8 |
  464287.3 |########################################
  467719.8 |
  471152.2 |
  474584.7 |
  478017.2 |########################################
  481449.6 |
  484882.1 |
  488314.6 |
  491747.0 |
  495179.5 |########################################
  498612.0 |
  502044.4 |
  505476.9 |
  508909.4 |########################################
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 388919.2-510519.4 ns)
  388919.2 |########################################
  394999.2 |
  401079.2 |
  407159.2 |
  413239.2 |
  419319.2 |########################################
  425399.3 |
  431479.3 |
  437559.3 |
  443639.3 |
  449719.3 |########################################
  455799.3 |
  461879.3 |
  467959.3 |########################################
  474039.3 |
  480119.4 |
  486199.4 |
  492279.4 |
  498359.4 |
  504439.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=103.8% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=107.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=110.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=102.2% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=109.3% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=102.1% of algo (FFI overhead may distort results)
