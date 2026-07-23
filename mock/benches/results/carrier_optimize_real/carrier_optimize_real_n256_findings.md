# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_opt_real_canon, carrier_opt_real_all) are a dead heat (<1%)

carrier_opt_real_canon (7.98 us) and carrier_opt_real_all (7.99 us) differ by 0.09%, inside the noise, even though the wider field spreads 6.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_real_none shows alternating (throttle bounce) (autocorr -0.57)

carrier_opt_real_none's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_opt_real_canon vs stability leader carrier_opt_real_all (+0% speed for 1.3x steadier)

carrier_opt_real_canon is fastest (7.98 us, CV 5.7%); carrier_opt_real_all gives up 0.1% median for 1.3x lower variance (CV 4.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_opt_real_fold's edge over baseline is significant but tiny (-43 ns, 0.51%)

carrier_opt_real_fold differs from baseline carrier_opt_real_none by -43 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_real_canon** at 7983.1 ns median (-3.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.06x (fastest 7983.1 ns, slowest 8465.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 10279ns | 10280ns | 9638ns | 10162ns | 10775ns | -3.91% |
| carrier_opt_real_canon | 10377ns | 10232ns | 9814ns | 10094ns | 11081ns | -2.99% |
| carrier_opt_real_cse | 10466ns | 10514ns | 9813ns | 10318ns | 11015ns | -2.15% |
| carrier_opt_real_dce | 10862ns | 10744ns | 10142ns | 10579ns | 11647ns | +1.55% |
| carrier_opt_real_fold | 10712ns | 10629ns | 10116ns | 10460ns | 11388ns | +0.14% |
| carrier_opt_real_none | 10697ns | 10527ns | 10116ns | 10416ns | 11407ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 7992ns | 7510ns | 8378ns | -5.02% | 0.032 |
| carrier_opt_real_canon | 8097ns | 7658ns | 8647ns | -3.77% | 0.032 |
| carrier_opt_real_cse | 8165ns | 7657ns | 8599ns | -2.96% | 0.031 |
| carrier_opt_real_dce | 8561ns | 7957ns | 9191ns | +1.74% | 0.030 |
| carrier_opt_real_fold | 8419ns | 7933ns | 8953ns | +0.05% | 0.030 |
| carrier_opt_real_none | 8414ns | 7955ns | 8973ns | base | 0.030 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_real_all | 299435 | 1580968 | 0.189 | 1.02× |
| carrier_opt_real_canon | 296564 | 1569882 | 0.189 | 1.01× |
| carrier_opt_real_cse | 291266 | 1540564 | 0.189 | 0.99× |
| carrier_opt_real_dce | 297446 | 1535308 | 0.194 | 1.01× |
| carrier_opt_real_fold | 293878 | 1537228 | 0.191 | 1.00× |
| carrier_opt_real_none | 294886 | 1545700 | 0.191 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_opt_real_all; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.032 | 94.0% |
| carrier_opt_real_canon | 0.032 | 94.1% |
| carrier_opt_real_cse | 0.031 | 91.6% |
| carrier_opt_real_dce | 0.030 | 88.7% |
| carrier_opt_real_fold | 0.031 | 89.9% |
| carrier_opt_real_none | 0.031 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 10279ns | 10279ns | -3.91% |
| carrier_opt_real_canon | 10377ns | 10377ns | -2.99% |
| carrier_opt_real_cse | 10466ns | 10466ns | -2.15% |
| carrier_opt_real_dce | 10862ns | 10862ns | +1.55% |
| carrier_opt_real_fold | 10712ns | 10712ns | +0.14% |
| carrier_opt_real_none | 10697ns | 10697ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 8281ns | base | --- | [7989, 8973] | --- | --- | --- | --- |
| carrier_opt_real_all | 7990ns | -415.2ns (-5.0%) | [-596, -257]ns | [7608, 8378] | YES (adj: no) | 0.1563 | 0.0313 | 0 |
| carrier_opt_real_canon | 7983ns | no significant difference | [-632, +39]ns | [7662, 8647] | no | 0.3646 | 0.2188 | 0 |
| carrier_opt_real_cse | 8202ns | no significant difference | [-397, +12]ns | [7694, 8599] | no | 0.3646 | 0.2188 | 0 |
| carrier_opt_real_dce | 8466ns | no significant difference | [-255, +517]ns | [8026, 9191] | no | 0.8594 | 0.6875 | 0 |
| carrier_opt_real_fold | 8357ns | no significant difference | [-191, +247]ns | [7947, 8953] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_canon | carrier_opt_real_cse | carrier_opt_real_dce | carrier_opt_real_fold |
|---|---|---|---|---|---|---|
| 1 | 7955ns | -5.6% | -3.7% | -3.7% | +1.9% | +0.1% |
| 2 | 8942ns | -6.1% | -2.2% | -4.3% | +2.6% | +0.2% |
| 3 | 8024ns | -3.5% | +3.4% | +4.0% | +10.0% | -1.1% |
| 4 | 8470ns | -2.7% | -9.5% | -4.8% | -4.4% | +5.6% |
| 5 | 9005ns | -7.2% | -5.1% | -4.0% | +2.2% | -3.1% |
| 6 | 8091ns | -4.8% | -5.2% | -4.4% | -1.7% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.508 | HIGH- (thermal bounce) |
| carrier_opt_real_canon | -0.512 | HIGH- (thermal bounce) |
| carrier_opt_real_cse | -0.465 | moderate- |
| carrier_opt_real_dce | -0.563 | HIGH- (thermal bounce) |
| carrier_opt_real_fold | -0.568 | HIGH- (thermal bounce) |
| carrier_opt_real_none | -0.572 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_canon**: won 5/6, lost 1/6
- **carrier_opt_real_cse**: won 5/6, lost 1/6
- **carrier_opt_real_dce**: won 2/6, lost 4/6
- **carrier_opt_real_fold**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 90900.6ns | 7991.9ns | 1137.4% | HIGH |
| carrier_opt_real_canon | 89446.4ns | 8097.3ns | 1104.6% | HIGH |
| carrier_opt_real_cse | 88416.8ns | 8165.3ns | 1082.8% | HIGH |
| carrier_opt_real_dce | 89602.6ns | 8560.8ns | 1046.7% | HIGH |
| carrier_opt_real_fold | 88565.9ns | 8418.9ns | 1052.0% | HIGH |
| carrier_opt_real_none | 88861.0ns | 8414.4ns | 1056.1% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 7510.4-8377.8 ns)
   7510.4 |########################################
   7553.8 |
   7597.1 |
   7640.5 |
   7683.9 |########################################
   7727.2 |########################################
   7770.6 |
   7814.0 |
   7857.3 |
   7900.7 |
   7944.1 |
   7987.4 |
   8030.8 |
   8074.2 |
   8117.5 |
   8160.9 |
   8204.3 |########################################
   8247.6 |
   8291.0 |
   8334.4 |########################################
  (0 below, 1 above range)

carrier_opt_real_canon (n=6, range 7657.5-8647.1 ns)
   7657.5 |########################################
   7707.0 |
   7756.5 |
   7805.9 |
   7855.4 |
   7904.9 |
   7954.4 |
   8003.9 |
   8053.3 |
   8102.8 |
   8152.3 |
   8201.8 |
   8251.3 |#############
   8300.7 |
   8350.2 |
   8399.7 |
   8449.2 |
   8498.7 |#############
   8548.1 |
   8597.6 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 7657.1-8599.3 ns)
   7657.1 |########################################
   7704.2 |########################################
   7751.3 |
   7798.4 |
   7845.6 |
   7892.7 |
   7939.8 |
   7986.9 |
   8034.0 |########################################
   8081.1 |
   8128.2 |
   8175.3 |
   8222.4 |
   8269.6 |
   8316.7 |########################################
   8363.8 |
   8410.9 |
   8458.0 |
   8505.1 |
   8552.2 |########################################
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 7956.7-9190.8 ns)
   7956.7 |####################
   8018.4 |
   8080.1 |########################################
   8141.8 |
   8203.5 |
   8265.2 |
   8326.9 |
   8388.6 |
   8450.3 |
   8512.0 |
   8573.8 |
   8635.5 |
   8697.2 |
   8758.9 |
   8820.6 |####################
   8882.3 |
   8944.0 |
   9005.7 |
   9067.4 |
   9129.1 |####################
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 7932.9-8953.1 ns)
   7932.9 |########################################
   7983.9 |####################
   8034.9 |
   8085.9 |
   8136.9 |
   8187.9 |
   8239.0 |
   8290.0 |
   8341.0 |
   8392.0 |
   8443.0 |
   8494.0 |
   8545.0 |
   8596.0 |
   8647.0 |
   8698.0 |####################
   8749.1 |
   8800.1 |
   8851.1 |
   8902.1 |####################
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 7954.6-8973.4 ns)
   7954.6 |########################################
   8005.5 |########################################
   8056.5 |########################################
   8107.4 |
   8158.4 |
   8209.3 |
   8260.2 |
   8311.2 |
   8362.1 |
   8413.0 |
   8464.0 |########################################
   8514.9 |
   8565.9 |
   8616.8 |
   8667.7 |
   8718.7 |
   8769.6 |
   8820.5 |
   8871.5 |
   8922.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=1137.4% of algo (FFI overhead may distort results)
- **carrier_opt_real_canon**: bridge=1126.9% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=1061.5% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=1056.7% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=1061.0% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=1072.4% of algo (FFI overhead may distort results)
