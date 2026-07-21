# Partial-eval specialization: fold ratio on a block-structured template (reduction metric)

3 variants, 6 samples per variant.
Baseline: **pe_struct_sf50**

## Highlights

Baseline for all deltas below: **pe_struct_sf50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### pe_struct_sf50 shows alternating (throttle bounce) (autocorr -0.55)

pe_struct_sf50's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: pe_struct_sf70** at 487474.6 ns median (-8.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.40x (fastest 487474.6 ns, slowest 681123.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_struct_sf50 | 537087ns | 536816ns | 524550ns | 535916ns | 545112ns | base |
| pe_struct_sf70 | 488915ns | 489796ns | 481018ns | 488091ns | 494100ns | -8.97% |
| pe_struct_sf90 | 689481ns | 683495ns | 679927ns | 682410ns | 704864ns | +28.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_struct_sf50 | 534740ns | 522180ns | 542785ns | base | 0.008 |
| pe_struct_sf70 | 486580ns | 478687ns | 491766ns | -9.01% | 0.008 |
| pe_struct_sf90 | 687066ns | 677620ns | 702329ns | +28.49% | 0.006 |

## Performance model

- Peak throughput: **0.009 Gops/s** (pe_struct_sf70; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_struct_sf50 | 0.008 | 89.6% |
| pe_struct_sf70 | 0.008 | 98.2% |
| pe_struct_sf90 | 0.006 | 70.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_struct_sf50 | 537087ns | 537087ns | base |
| pe_struct_sf70 | 488915ns | 488915ns | -8.97% |
| pe_struct_sf90 | 689481ns | 689481ns | +28.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_struct_sf50 | 534445ns | base | --- | [526991, 542785] | --- | --- | --- | --- |
| pe_struct_sf70 | 487475ns | -49330.0ns (-9.2%) | [-56693, -38456]ns | [480501, 491766] | YES | 0.0313 | 0.0313 | 0 |
| pe_struct_sf90 | 681124ns | +145130.8ns (+27.2%) | [+136508, +175338]ns | [677744, 702329] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_struct_sf50 | pe_struct_sf70 | pe_struct_sf90 |
|---|---|---|---|
| 1 | 540808ns | -9.1% | +25.3% |
| 2 | 531803ns | -9.3% | +29.9% |
| 3 | 536104ns | -8.7% | +27.1% |
| 4 | 532785ns | -10.2% | +27.2% |
| 5 | 544762ns | -10.9% | +25.0% |
| 6 | 522180ns | -5.8% | +36.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_struct_sf50 | -0.549 | HIGH- (thermal bounce) |
| pe_struct_sf70 | -0.375 | moderate- |
| pe_struct_sf90 | -0.110 | ok |

**Consistency summary:**

- **pe_struct_sf70**: won 6/6, lost 0/6
- **pe_struct_sf90**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_struct_sf50 | 8.6ns | 534740.2ns | 0.0% |  |
| pe_struct_sf70 | 5.9ns | 486580.5ns | 0.0% |  |
| pe_struct_sf90 | 9.9ns | 687065.7ns | 0.0% |  |

## Distribution (algo ns)

```
pe_struct_sf50 (n=6, range 522179.6-542784.8 ns)
  522179.6 |########################################
  523209.9 |
  524240.1 |
  525270.4 |
  526300.6 |
  527330.9 |
  528361.2 |
  529391.4 |
  530421.7 |
  531451.9 |########################################
  532482.2 |########################################
  533512.5 |
  534542.7 |
  535573.0 |########################################
  536603.2 |
  537633.5 |
  538663.8 |
  539694.0 |
  540724.3 |########################################
  541754.5 |
  (0 below, 1 above range)

pe_struct_sf70 (n=6, range 478686.7-491766.2 ns)
  478686.7 |########################################
  479340.7 |
  479994.7 |
  480648.6 |
  481302.6 |
  481956.6 |########################################
  482610.5 |
  483264.5 |
  483918.5 |
  484572.5 |
  485226.5 |########################################
  485880.4 |
  486534.4 |
  487188.4 |
  487842.4 |
  488496.3 |
  489150.3 |########################################
  489804.3 |
  490458.2 |
  491112.2 |########################################
  (0 below, 1 above range)

pe_struct_sf90 (n=6, range 677620.4-702329.4 ns)
  677620.4 |########################################
  678855.8 |
  680091.3 |########################################
  681326.7 |
  682562.2 |
  683797.6 |
  685033.1 |
  686268.5 |
  687504.0 |
  688739.4 |
  689974.9 |####################
  691210.3 |
  692445.8 |
  693681.2 |
  694916.7 |
  696152.1 |
  697387.6 |
  698623.0 |
  699858.5 |
  701093.9 |
  (0 below, 1 above range)

```
