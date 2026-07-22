# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_real_eqsat shows alternating (throttle bounce) (autocorr -0.62)

carrier_opt_real_eqsat's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_opt_real_eqsat vs stability leader carrier_opt_real_all (+5% speed for 1.1x steadier)

carrier_opt_real_eqsat is fastest (456.24 us, CV 4.0%); carrier_opt_real_all gives up 4.9% median for 1.1x lower variance (CV 3.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_opt_real_eqsat** at 456240.7 ns median (-16.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.21x (fastest 456240.7 ns, slowest 550567.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 477297ns | 481272ns | 450254ns | 474734ns | 494664ns | -12.26% |
| carrier_opt_real_cse | 471373ns | 474753ns | 410681ns | 465847ns | 510008ns | -13.35% |
| carrier_opt_real_cseeqsat | 472063ns | 479138ns | 425603ns | 471330ns | 496391ns | -13.22% |
| carrier_opt_real_dce | 551277ns | 553786ns | 510822ns | 545688ns | 579890ns | +1.34% |
| carrier_opt_real_eqsat | 463676ns | 459398ns | 438187ns | 457083ns | 486311ns | -14.76% |
| carrier_opt_real_fold | 525301ns | 520374ns | 498427ns | 515550ns | 553364ns | -3.43% |
| carrier_opt_real_none | 543986ns | 550962ns | 499770ns | 536304ns | 577616ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 474302ns | 446990ns | 491750ns | -12.33% | 0.009 |
| carrier_opt_real_cse | 468432ns | 408358ns | 506637ns | -13.42% | 0.009 |
| carrier_opt_real_cseeqsat | 469405ns | 423282ns | 493780ns | -13.24% | 0.009 |
| carrier_opt_real_dce | 548255ns | 507731ns | 576891ns | +1.34% | 0.007 |
| carrier_opt_real_eqsat | 460593ns | 435378ns | 483140ns | -14.87% | 0.009 |
| carrier_opt_real_fold | 522381ns | 496067ns | 550169ns | -3.45% | 0.008 |
| carrier_opt_real_none | 541025ns | 496864ns | 574300ns | base | 0.008 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_opt_real_cse; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.009 | 85.4% |
| carrier_opt_real_cse | 0.009 | 86.6% |
| carrier_opt_real_cseeqsat | 0.009 | 85.7% |
| carrier_opt_real_dce | 0.007 | 74.2% |
| carrier_opt_real_eqsat | 0.009 | 89.5% |
| carrier_opt_real_fold | 0.008 | 78.9% |
| carrier_opt_real_none | 0.007 | 74.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 477297ns | 477297ns | -12.26% |
| carrier_opt_real_cse | 471373ns | 471373ns | -13.35% |
| carrier_opt_real_cseeqsat | 472063ns | 472063ns | -13.22% |
| carrier_opt_real_dce | 551277ns | 551277ns | +1.34% |
| carrier_opt_real_eqsat | 463676ns | 463676ns | -14.76% |
| carrier_opt_real_fold | 525301ns | 525301ns | -3.43% |
| carrier_opt_real_none | 543986ns | 543986ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 548004ns | base | --- | [500772, 574300] | --- | --- | --- | --- |
| carrier_opt_real_all | 478401ns | -93563.8ns (-17.1%) | [-97584, -9022]ns | [452756, 491750] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_real_cse | 471713ns | -76536.5ns (-14.0%) | [-114255, -26990]ns | [426946, 506637] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| carrier_opt_real_cseeqsat | 476388ns | -74670.6ns (-13.6%) | [-106053, -34138]ns | [438046, 493780] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_real_dce | 550567ns | no significant difference | [-36650, +55477]ns | [517308, 576891] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_real_eqsat | 456241ns | -84842.0ns (-15.5%) | [-118118, -38337]ns | [442399, 483140] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_real_fold | 517468ns | no significant difference | [-74793, +42202]ns | [499507, 550169] | no | 0.8250 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_cse | carrier_opt_real_cseeqsat | carrier_opt_real_dce | carrier_opt_real_eqsat | carrier_opt_real_fold |
|---|---|---|---|---|---|---|---|
| 1 | 565082ns | -16.4% | -11.4% | -13.8% | -6.8% | -15.6% | -11.0% |
| 2 | 553173ns | -17.1% | -16.4% | -12.9% | +6.4% | -18.8% | -8.2% |
| 3 | 542835ns | -17.7% | -11.4% | -7.8% | -6.5% | -15.0% | -0.2% |
| 4 | 504680ns | -1.9% | +1.6% | -16.1% | +8.5% | -13.7% | +4.5% |
| 5 | 496864ns | -1.7% | -17.8% | -5.2% | +13.7% | -1.5% | +12.4% |
| 6 | 583518ns | -17.0% | -23.6% | -22.4% | -5.1% | -22.7% | -15.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | 0.187 | ok |
| carrier_opt_real_cse | -0.136 | ok |
| carrier_opt_real_cseeqsat | -0.244 | moderate- |
| carrier_opt_real_dce | -0.591 | HIGH- (thermal bounce) |
| carrier_opt_real_eqsat | -0.624 | HIGH- (thermal bounce) |
| carrier_opt_real_fold | -0.227 | moderate- |
| carrier_opt_real_none | -0.004 | ok |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 5/6, lost 1/6
- **carrier_opt_real_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 3/6, lost 3/6
- **carrier_opt_real_eqsat**: won 6/6, lost 0/6
- **carrier_opt_real_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 474949.7ns | 474302.3ns | 100.1% | HIGH |
| carrier_opt_real_cse | 468821.9ns | 468431.9ns | 100.1% | HIGH |
| carrier_opt_real_cseeqsat | 470027.3ns | 469405.0ns | 100.1% | HIGH |
| carrier_opt_real_dce | 548976.9ns | 548255.5ns | 100.1% | HIGH |
| carrier_opt_real_eqsat | 461227.7ns | 460593.3ns | 100.1% | HIGH |
| carrier_opt_real_fold | 523401.0ns | 522381.1ns | 100.2% | HIGH |
| carrier_opt_real_none | 541965.1ns | 541025.5ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 446989.6-491750.4 ns)
  446989.6 |########################################
  449227.6 |
  451465.7 |
  453703.7 |
  455941.8 |
  458179.8 |########################################
  460417.8 |
  462655.9 |
  464893.9 |
  467132.0 |
  469370.0 |
  471608.0 |########################################
  473846.1 |
  476084.1 |
  478322.2 |
  480560.2 |
  482798.2 |########################################
  485036.3 |
  487274.3 |########################################
  489512.4 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 408357.5-506637.0 ns)
  408357.5 |########################################
  413271.5 |
  418185.5 |
  423099.4 |
  428013.4 |
  432927.4 |
  437841.4 |
  442755.3 |########################################
  447669.3 |
  452583.3 |
  457497.3 |
  462411.3 |########################################
  467325.2 |
  472239.2 |
  477153.2 |########################################
  482067.2 |
  486981.1 |
  491895.1 |
  496809.1 |########################################
  501723.1 |
  (0 below, 1 above range)

carrier_opt_real_cseeqsat (n=6, range 423282.5-493780.5 ns)
  423282.5 |########################################
  426807.4 |
  430332.3 |
  433857.2 |
  437382.1 |
  440907.0 |
  444431.9 |
  447956.8 |
  451481.7 |########################################
  455006.6 |
  458531.5 |
  462056.4 |
  465581.3 |
  469106.2 |########################################
  472631.1 |
  476156.0 |
  479680.9 |########################################
  483205.8 |
  486730.7 |########################################
  490255.6 |
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 507731.2-576890.8 ns)
  507731.2 |########################################
  511189.2 |
  514647.2 |
  518105.1 |
  521563.1 |
  525021.1 |########################################
  528479.1 |
  531937.1 |
  535395.1 |
  538853.0 |
  542311.0 |
  545769.0 |########################################
  549227.0 |
  552685.0 |########################################
  556143.0 |
  559600.9 |
  563058.9 |########################################
  566516.9 |
  569974.9 |
  573432.9 |
  (0 below, 1 above range)

carrier_opt_real_eqsat (n=6, range 435377.5-483140.2 ns)
  435377.5 |########################################
  437765.6 |
  440153.8 |
  442541.9 |
  444930.0 |
  447318.2 |########################################
  449706.3 |########################################
  452094.5 |
  454482.6 |
  456870.7 |
  459258.9 |########################################
  461647.0 |
  464035.2 |
  466423.3 |
  468811.4 |
  471199.6 |
  473587.7 |
  475975.8 |########################################
  478364.0 |
  480752.1 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 496066.7-550168.6 ns)
  496066.7 |########################################
  498771.8 |
  501476.9 |########################################
  504182.0 |
  506887.1 |########################################
  509592.2 |
  512297.3 |
  515002.3 |
  517707.4 |
  520412.5 |
  523117.6 |
  525822.7 |########################################
  528527.8 |
  531232.9 |
  533938.0 |
  536643.1 |
  539348.2 |########################################
  542053.3 |
  544758.4 |
  547463.5 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 496864.2-574300.0 ns)
  496864.2 |########################################
  500736.0 |
  504607.8 |########################################
  508479.6 |
  512351.4 |
  516223.2 |
  520094.9 |
  523966.7 |
  527838.5 |
  531710.3 |
  535582.1 |
  539453.9 |########################################
  543325.7 |
  547197.5 |
  551069.3 |########################################
  554941.1 |
  558812.8 |
  562684.6 |########################################
  566556.4 |
  570428.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_opt_real_cseeqsat**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_eqsat**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=100.2% of algo (FFI overhead may distort results)
