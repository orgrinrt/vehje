# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, scatter profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_scatter_none**

## Highlights

Baseline for all deltas below: **carrier_opt_scatter_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_scatter_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_scatter_none has the worst median (8.88 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_scatter_cse at 8.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_scatter_all shows alternating (throttle bounce) (autocorr -0.60)

carrier_opt_scatter_all's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_opt_scatter_cse's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_opt_scatter_cse are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader carrier_opt_scatter_cse vs stability leader carrier_opt_scatter_none (+7% speed for 1.3x steadier)

carrier_opt_scatter_cse is fastest (8.29 us, CV 4.9%); carrier_opt_scatter_none gives up 7.1% median for 1.3x lower variance (CV 3.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_opt_scatter_fold's edge over baseline is significant but tiny (-2 ns, 0.03%)

carrier_opt_scatter_fold differs from baseline carrier_opt_scatter_none by -2 ns (0.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_scatter_cse** at 8287.3 ns median (-6.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.07x (fastest 8287.3 ns, slowest 8879.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_scatter_all | 10838ns | 11025ns | 10100ns | 10746ns | 11344ns | -3.47% |
| carrier_opt_scatter_canon | 10819ns | 10919ns | 10082ns | 10666ns | 11417ns | -3.64% |
| carrier_opt_scatter_cse | 10715ns | 10722ns | 10013ns | 10521ns | 11357ns | -4.56% |
| carrier_opt_scatter_dce | 11155ns | 11205ns | 10502ns | 10992ns | 11726ns | -0.65% |
| carrier_opt_scatter_fold | 10971ns | 10870ns | 10380ns | 10745ns | 11606ns | -2.28% |
| carrier_opt_scatter_none | 11227ns | 11252ns | 10616ns | 11045ns | 11808ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_scatter_all | 8457ns | 7899ns | 8828ns | -4.27% | 0.030 |
| carrier_opt_scatter_canon | 8457ns | 7906ns | 8948ns | -4.27% | 0.030 |
| carrier_opt_scatter_cse | 8303ns | 7762ns | 8784ns | -6.00% | 0.031 |
| carrier_opt_scatter_dce | 8767ns | 8268ns | 9211ns | -0.75% | 0.029 |
| carrier_opt_scatter_fold | 8673ns | 8222ns | 9193ns | -1.81% | 0.030 |
| carrier_opt_scatter_none | 8834ns | 8417ns | 9203ns | base | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_scatter_all | 291692 | 1523505 | 0.191 | 0.98× |
| carrier_opt_scatter_canon | 291660 | 1545132 | 0.189 | 0.98× |
| carrier_opt_scatter_cse | 297160 | 1570290 | 0.189 | 1.00× |
| carrier_opt_scatter_dce | 300686 | 1586564 | 0.190 | 1.01× |
| carrier_opt_scatter_fold | 304359 | 1593797 | 0.191 | 1.02× |
| carrier_opt_scatter_none | 297698 | 1567012 | 0.190 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_opt_scatter_cse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_scatter_all | 0.030 | 90.2% |
| carrier_opt_scatter_canon | 0.030 | 91.3% |
| carrier_opt_scatter_cse | 0.031 | 93.7% |
| carrier_opt_scatter_dce | 0.029 | 88.0% |
| carrier_opt_scatter_fold | 0.030 | 90.8% |
| carrier_opt_scatter_none | 0.029 | 87.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_scatter_all | 10838ns | 10838ns | -3.47% |
| carrier_opt_scatter_canon | 10819ns | 10819ns | -3.64% |
| carrier_opt_scatter_cse | 10715ns | 10715ns | -4.56% |
| carrier_opt_scatter_dce | 11155ns | 11155ns | -0.65% |
| carrier_opt_scatter_fold | 10971ns | 10971ns | -2.28% |
| carrier_opt_scatter_none | 11227ns | 11227ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_scatter_none | 8879ns | base | --- | [8418, 9203] | --- | --- | --- | --- |
| carrier_opt_scatter_all | 8608ns | -442.0ns (-5.0%) | [-572, -117]ns | [7934, 8828] | YES (adj: no) | 0.3646 | 0.2188 | 0 |
| carrier_opt_scatter_canon | 8503ns | -425.6ns (-4.8%) | [-512, -193]ns | [7919, 8948] | YES (adj: no) | 0.1562 | 0.0313 | 0 |
| carrier_opt_scatter_cse | 8287ns | -416.5ns (-4.7%) | [-1016, -158]ns | [7839, 8784] | YES (adj: no) | 0.1562 | 0.0625 | **1** (17%, HIGH) |
| carrier_opt_scatter_dce | 8819ns | no significant difference | [-330, +234]ns | [8272, 9211] | no | 0.8594 | 0.6875 | 0 |
| carrier_opt_scatter_fold | 8550ns | no significant difference | [-534, +56]ns | [8278, 9193] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_scatter_none | carrier_opt_scatter_all | carrier_opt_scatter_canon | carrier_opt_scatter_cse | carrier_opt_scatter_dce | carrier_opt_scatter_fold |
|---|---|---|---|---|---|---|
| 1 | 9206ns | -3.6% | -2.9% | -15.7% | -0.7% | -10.7% |
| 2 | 8420ns | -6.2% | -6.1% | -3.9% | -1.7% | +0.4% |
| 3 | 9076ns | -4.8% | -1.3% | -6.5% | -5.6% | +0.4% |
| 4 | 9200ns | -6.8% | -4.0% | -3.4% | +0.9% | +0.8% |
| 5 | 8417ns | -5.3% | -5.8% | -5.9% | -1.8% | -1.0% |
| 6 | 8683ns | +1.1% | -5.9% | +0.0% | +4.5% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_scatter_all | -0.604 | HIGH- (thermal bounce) |
| carrier_opt_scatter_canon | -0.311 | moderate- |
| carrier_opt_scatter_cse | -0.187 | ok |
| carrier_opt_scatter_dce | -0.580 | HIGH- (thermal bounce) |
| carrier_opt_scatter_fold | 0.078 | ok |
| carrier_opt_scatter_none | -0.365 | moderate- |

**Consistency summary:**

- **carrier_opt_scatter_all**: won 5/6, lost 1/6
- **carrier_opt_scatter_canon**: won 6/6, lost 0/6
- **carrier_opt_scatter_cse**: won 5/6, lost 0/6
- **carrier_opt_scatter_dce**: won 4/6, lost 2/6
- **carrier_opt_scatter_fold**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_scatter_all | 88510.9ns | 8456.5ns | 1046.7% | HIGH |
| carrier_opt_scatter_canon | 88723.3ns | 8456.7ns | 1049.1% | HIGH |
| carrier_opt_scatter_cse | 88588.3ns | 8303.4ns | 1066.9% | HIGH |
| carrier_opt_scatter_dce | 90562.3ns | 8767.0ns | 1033.0% | HIGH |
| carrier_opt_scatter_fold | 90372.1ns | 8673.4ns | 1041.9% | HIGH |
| carrier_opt_scatter_none | 90131.9ns | 8833.5ns | 1020.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_scatter_all (n=6, range 7898.7-8827.7 ns)
   7898.7 |########################################
   7945.1 |########################################
   7991.6 |
   8038.1 |
   8084.5 |
   8130.9 |
   8177.4 |
   8223.9 |
   8270.3 |
   8316.8 |
   8363.2 |
   8409.6 |
   8456.1 |
   8502.6 |
   8549.0 |########################################
   8595.5 |########################################
   8641.9 |
   8688.4 |
   8734.8 |########################################
   8781.2 |
  (0 below, 1 above range)

carrier_opt_scatter_canon (n=6, range 7906.2-8947.7 ns)
   7906.2 |########################################
   7958.3 |
   8010.4 |
   8062.4 |
   8114.5 |
   8166.6 |####################
   8218.6 |
   8270.7 |
   8322.8 |
   8374.9 |
   8427.0 |
   8479.0 |
   8531.1 |
   8583.2 |
   8635.2 |
   8687.3 |
   8739.4 |
   8791.5 |####################
   8843.6 |
   8895.6 |####################
  (0 below, 1 above range)

carrier_opt_scatter_cse (n=6, range 7761.7-8784.0 ns)
   7761.7 |########################################
   7812.8 |
   7863.9 |
   7915.0 |########################################
   7966.1 |
   8017.3 |
   8068.4 |########################################
   8119.5 |
   8170.6 |
   8221.7 |
   8272.8 |
   8323.9 |
   8375.1 |
   8426.2 |
   8477.3 |########################################
   8528.4 |
   8579.5 |
   8630.6 |
   8681.7 |########################################
   8732.8 |
  (0 below, 1 above range)

carrier_opt_scatter_dce (n=6, range 8267.9-9210.6 ns)
   8267.9 |########################################
   8315.0 |
   8362.2 |
   8409.3 |
   8456.4 |
   8503.6 |
   8550.7 |####################
   8597.9 |
   8645.0 |
   8692.1 |
   8739.3 |
   8786.4 |
   8833.5 |
   8880.7 |
   8927.8 |
   8975.0 |
   9022.1 |
   9069.2 |####################
   9116.4 |####################
   9163.5 |
  (0 below, 1 above range)

carrier_opt_scatter_fold (n=6, range 8222.5-9193.2 ns)
   8222.5 |########################################
   8271.0 |
   8319.6 |########################################
   8368.1 |
   8416.6 |########################################
   8465.2 |
   8513.7 |
   8562.2 |
   8610.8 |########################################
   8659.3 |
   8707.8 |
   8756.4 |
   8804.9 |
   8853.4 |
   8902.0 |
   8950.5 |
   8999.0 |
   9047.6 |
   9096.1 |########################################
   9144.6 |
  (0 below, 1 above range)

carrier_opt_scatter_none (n=6, range 8416.7-9203.1 ns)
   8416.7 |########################################
   8456.0 |
   8495.3 |
   8534.7 |
   8574.0 |
   8613.3 |
   8652.6 |####################
   8691.9 |
   8731.3 |
   8770.6 |
   8809.9 |
   8849.2 |
   8888.5 |
   8927.9 |
   8967.2 |
   9006.5 |
   9045.8 |####################
   9085.1 |
   9124.5 |
   9163.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_scatter_all**: bridge=1029.9% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_canon**: bridge=1045.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_cse**: bridge=1070.2% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_dce**: bridge=1034.0% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_fold**: bridge=1062.6% of algo (FFI overhead may distort results)
- **carrier_opt_scatter_none**: bridge=1013.8% of algo (FFI overhead may distort results)
