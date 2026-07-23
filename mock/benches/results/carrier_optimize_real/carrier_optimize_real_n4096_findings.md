# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_real_dce shows alternating (throttle bounce) (autocorr -0.57)

carrier_opt_real_dce's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_opt_real_all vs stability leader carrier_opt_real_canon (+2% speed for 1.7x steadier)

carrier_opt_real_all is fastest (443.48 us, CV 3.8%); carrier_opt_real_canon gives up 1.7% median for 1.7x lower variance (CV 2.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_opt_real_all** at 443483.5 ns median (-12.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.17x (fastest 443483.5 ns, slowest 520310.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 439818ns | 445756ns | 409442ns | 439744ns | 455119ns | -14.02% |
| carrier_opt_real_canon | 451678ns | 453572ns | 432717ns | 451583ns | 461301ns | -11.70% |
| carrier_opt_real_cse | 457033ns | 456221ns | 440942ns | 454840ns | 468366ns | -10.65% |
| carrier_opt_real_dce | 514839ns | 511198ns | 491406ns | 506700ns | 538764ns | +0.65% |
| carrier_opt_real_fold | 522149ns | 522509ns | 495593ns | 517475ns | 542437ns | +2.08% |
| carrier_opt_real_none | 511523ns | 508792ns | 490865ns | 507236ns | 528283ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 437495ns | 407130ns | 452665ns | -14.09% | 0.009 |
| carrier_opt_real_canon | 449317ns | 430414ns | 458899ns | -11.77% | 0.009 |
| carrier_opt_real_cse | 454747ns | 438726ns | 466132ns | -10.71% | 0.009 |
| carrier_opt_real_dce | 512455ns | 489163ns | 536095ns | +0.63% | 0.008 |
| carrier_opt_real_fold | 519811ns | 493202ns | 539971ns | +2.07% | 0.008 |
| carrier_opt_real_none | 509265ns | 488643ns | 526059ns | base | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_real_all | 2738012 | 3827584 | 0.715 | 0.86× |
| carrier_opt_real_canon | 2786623 | 3835332 | 0.727 | 0.87× |
| carrier_opt_real_cse | 2834594 | 3837293 | 0.739 | 0.89× |
| carrier_opt_real_dce | 3175298 | 4179714 | 0.760 | 1.00× |
| carrier_opt_real_fold | 3228049 | 4181987 | 0.772 | 1.01× |
| carrier_opt_real_none | 3187346 | 4178642 | 0.763 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_opt_real_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.009 | 91.8% |
| carrier_opt_real_canon | 0.009 | 90.2% |
| carrier_opt_real_cse | 0.009 | 89.7% |
| carrier_opt_real_dce | 0.008 | 80.0% |
| carrier_opt_real_fold | 0.008 | 78.2% |
| carrier_opt_real_none | 0.008 | 80.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 439818ns | 439818ns | -14.02% |
| carrier_opt_real_canon | 451678ns | 451678ns | -11.70% |
| carrier_opt_real_cse | 457033ns | 457033ns | -10.65% |
| carrier_opt_real_dce | 514839ns | 514839ns | +0.65% |
| carrier_opt_real_fold | 522149ns | 522149ns | +2.08% |
| carrier_opt_real_none | 511523ns | 511523ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 506465ns | base | --- | [495272, 526059] | --- | --- | --- | --- |
| carrier_opt_real_all | 443484ns | -62762.9ns (-12.4%) | [-102437, -50110]ns | [416338, 452665] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_canon | 451194ns | -57074.4ns (-11.3%) | [-78023, -44745]ns | [437860, 458899] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_cse | 454034ns | -51836.9ns (-10.2%) | [-74700, -37019]ns | [444075, 466132] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_dce | 508979ns | no significant difference | [-15924, +28474]ns | [492291, 536095] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_real_fold | 520310ns | no significant difference | [-26908, +34871]ns | [499151, 539971] | no | 0.8594 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_canon | carrier_opt_real_cse | carrier_opt_real_dce | carrier_opt_real_fold |
|---|---|---|---|---|---|---|
| 1 | 505388ns | -15.8% | -8.8% | -11.1% | +8.8% | +8.5% |
| 2 | 488643ns | -10.4% | -11.9% | -5.5% | +0.1% | +5.5% |
| 3 | 507541ns | -10.8% | -11.0% | -10.7% | +2.5% | +4.7% |
| 4 | 501900ns | -9.8% | -9.0% | -9.4% | -1.3% | +4.6% |
| 5 | 532160ns | -23.5% | -16.3% | -17.6% | -1.8% | -5.1% |
| 6 | 519958ns | -13.6% | -13.3% | -9.5% | -4.2% | -5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.354 | moderate- |
| carrier_opt_real_canon | -0.496 | moderate- |
| carrier_opt_real_cse | -0.519 | HIGH- (thermal bounce) |
| carrier_opt_real_dce | -0.574 | HIGH- (thermal bounce) |
| carrier_opt_real_fold | 0.104 | ok |
| carrier_opt_real_none | 0.180 | ok |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_canon**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 3/6, lost 3/6
- **carrier_opt_real_fold**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 438161.6ns | 437495.4ns | 100.2% | HIGH |
| carrier_opt_real_canon | 449704.6ns | 449317.5ns | 100.1% | HIGH |
| carrier_opt_real_cse | 455625.7ns | 454746.7ns | 100.2% | HIGH |
| carrier_opt_real_dce | 514283.6ns | 512455.1ns | 100.4% | HIGH |
| carrier_opt_real_fold | 520793.9ns | 519810.9ns | 100.2% | HIGH |
| carrier_opt_real_none | 510296.8ns | 509265.1ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 407129.6-452665.2 ns)
  407129.6 |########################################
  409406.4 |
  411683.2 |
  413959.9 |
  416236.7 |
  418513.5 |
  420790.3 |
  423067.1 |
  425343.8 |########################################
  427620.6 |
  429897.4 |
  432174.2 |
  434451.0 |
  436727.7 |########################################
  439004.5 |
  441281.3 |
  443558.1 |
  445834.9 |
  448111.6 |########################################
  450388.4 |########################################
  (0 below, 1 above range)

carrier_opt_real_canon (n=6, range 430414.2-458898.8 ns)
  430414.2 |####################
  431838.4 |
  433262.7 |
  434686.9 |
  436111.1 |
  437535.3 |
  438959.6 |
  440383.8 |
  441808.0 |
  443232.2 |
  444656.5 |####################
  446080.7 |
  447504.9 |
  448929.2 |
  450353.4 |########################################
  451777.6 |
  453201.8 |
  454626.1 |
  456050.3 |####################
  457474.5 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 438726.2-466131.9 ns)
  438726.2 |########################################
  440096.5 |
  441466.8 |
  442837.1 |
  444207.3 |
  445577.6 |
  446947.9 |
  448318.2 |########################################
  449688.5 |
  451058.8 |
  452429.1 |########################################
  453799.3 |########################################
  455169.6 |
  456539.9 |
  457910.2 |
  459280.5 |
  460650.8 |########################################
  462021.0 |
  463391.3 |
  464761.6 |
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 489162.9-536095.2 ns)
  489162.9 |########################################
  491509.5 |
  493856.1 |########################################
  496202.7 |########################################
  498549.4 |
  500896.0 |
  503242.6 |
  505589.2 |
  507935.8 |
  510282.4 |
  512629.0 |
  514975.7 |
  517322.3 |
  519668.9 |########################################
  522015.5 |########################################
  524362.1 |
  526708.7 |
  529055.4 |
  531402.0 |
  533748.6 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 493201.7-539971.2 ns)
  493201.7 |########################################
  495540.2 |
  497878.7 |
  500217.1 |
  502555.6 |
  504894.1 |########################################
  507232.5 |
  509571.0 |
  511909.5 |
  514248.0 |########################################
  516586.4 |
  518924.9 |
  521263.4 |
  523601.9 |########################################
  525940.3 |
  528278.8 |
  530617.3 |########################################
  532955.8 |
  535294.2 |
  537632.7 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 488642.9-526059.2 ns)
  488642.9 |########################################
  490513.7 |
  492384.5 |
  494255.3 |
  496126.2 |
  497997.0 |
  499867.8 |
  501738.6 |########################################
  503609.4 |########################################
  505480.2 |
  507351.0 |########################################
  509221.8 |
  511092.7 |
  512963.5 |
  514834.3 |
  516705.1 |
  518575.9 |########################################
  520446.7 |
  522317.5 |
  524188.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_real_canon**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=100.3% of algo (FFI overhead may distort results)
