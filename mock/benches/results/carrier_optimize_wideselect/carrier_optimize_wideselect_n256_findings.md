# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_cse is fastest but the noisiest (CV 5.8%)

carrier_opt_wideselect_cse wins on median (8.07 us) yet has the highest variance (CV 5.8%), while carrier_opt_wideselect_dce is the steadiest (CV 0.3%, 8.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_opt_wideselect_cse, carrier_opt_wideselect_all) are a dead heat (<1%)

carrier_opt_wideselect_cse (8.07 us) and carrier_opt_wideselect_all (8.10 us) differ by 0.31%, inside the noise, even though the wider field spreads 5.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_wideselect_eqsat shows alternating (throttle bounce) (autocorr -0.63)

carrier_opt_wideselect_eqsat's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (440 ns) is smaller than the fastest variant's own run-to-run std-dev (468 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_opt_wideselect_cse vs stability leader carrier_opt_wideselect_dce (+5% speed for 18.5x steadier)

carrier_opt_wideselect_cse is fastest (8.07 us, CV 5.8%); carrier_opt_wideselect_dce gives up 5.5% median for 18.5x lower variance (CV 0.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_opt_wideselect_dce's edge over baseline is significant but tiny (-13 ns, 0.15%)

carrier_opt_wideselect_dce differs from baseline carrier_opt_wideselect_none by -13 ns (0.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_opt_wideselect_cse** at 8073.4 ns median (-5.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.05x (fastest 8073.4 ns, slowest 8513.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 10652ns | 10648ns | 10509ns | 10644ns | 10737ns | -4.25% |
| carrier_opt_wideselect_cse | 10492ns | 10614ns | 9243ns | 10566ns | 11004ns | -5.70% |
| carrier_opt_wideselect_cseeqsat | 10841ns | 10859ns | 10693ns | 10834ns | 10925ns | -2.56% |
| carrier_opt_wideselect_dce | 11088ns | 11108ns | 10961ns | 11077ns | 11167ns | -0.34% |
| carrier_opt_wideselect_eqsat | 10714ns | 10710ns | 10460ns | 10658ns | 10924ns | -3.71% |
| carrier_opt_wideselect_fold | 10964ns | 10962ns | 10856ns | 10931ns | 11066ns | -1.46% |
| carrier_opt_wideselect_none | 11126ns | 11096ns | 11049ns | 11092ns | 11216ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 8080ns | 7930ns | 8130ns | -4.95% | 0.032 |
| carrier_opt_wideselect_cse | 7988ns | 7053ns | 8362ns | -6.03% | 0.032 |
| carrier_opt_wideselect_cseeqsat | 8292ns | 8244ns | 8340ns | -2.46% | 0.031 |
| carrier_opt_wideselect_dce | 8513ns | 8471ns | 8544ns | +0.14% | 0.030 |
| carrier_opt_wideselect_eqsat | 8220ns | 8062ns | 8364ns | -3.31% | 0.031 |
| carrier_opt_wideselect_fold | 8459ns | 8379ns | 8510ns | -0.49% | 0.030 |
| carrier_opt_wideselect_none | 8501ns | 8434ns | 8549ns | base | 0.030 |

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_opt_wideselect_cse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.032 | 87.1% |
| carrier_opt_wideselect_cse | 0.032 | 87.4% |
| carrier_opt_wideselect_cseeqsat | 0.031 | 85.1% |
| carrier_opt_wideselect_dce | 0.030 | 82.8% |
| carrier_opt_wideselect_eqsat | 0.031 | 85.7% |
| carrier_opt_wideselect_fold | 0.030 | 83.3% |
| carrier_opt_wideselect_none | 0.030 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 10652ns | 10652ns | -4.25% |
| carrier_opt_wideselect_cse | 10492ns | 10492ns | -5.70% |
| carrier_opt_wideselect_cseeqsat | 10841ns | 10841ns | -2.56% |
| carrier_opt_wideselect_dce | 11088ns | 11088ns | -0.34% |
| carrier_opt_wideselect_eqsat | 10714ns | 10714ns | -3.71% |
| carrier_opt_wideselect_fold | 10964ns | 10964ns | -1.46% |
| carrier_opt_wideselect_none | 11126ns | 11126ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 8509ns | base | --- | [8444, 8549] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 8099ns | -414.8ns (-4.9%) | [-531, -316]ns | [8011, 8130] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 8073ns | -448.3ns (-5.3%) | [-952, -138]ns | [7529, 8362] | YES (adj: no) | 0.2625 | 0.2188 | 0 |
| carrier_opt_wideselect_cseeqsat | 8289ns | -201.2ns (-2.4%) | [-303, -122]ns | [8247, 8340] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 8514ns | no significant difference | [-51, +101]ns | [8481, 8544] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_eqsat | 8232ns | -258.5ns (-3.0%) | [-449, -135]ns | [8064, 8364] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_opt_wideselect_fold | 8466ns | no significant difference | [-116, +29]ns | [8400, 8510] | no | 0.2625 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_cse | carrier_opt_wideselect_cseeqsat | carrier_opt_wideselect_dce | carrier_opt_wideselect_eqsat | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|---|
| 1 | 8434ns | -3.3% | -16.4% | -0.7% | +1.4% | -1.2% | +1.1% |
| 2 | 8453ns | -4.2% | -4.4% | -2.2% | +1.0% | -2.8% | -0.4% |
| 3 | 8528ns | -5.0% | -6.1% | -2.6% | -0.7% | -3.3% | -0.5% |
| 4 | 8535ns | -5.0% | +1.1% | -3.4% | -0.3% | -5.5% | -0.4% |
| 5 | 8563ns | -7.4% | -5.5% | -3.7% | -0.5% | -2.0% | -1.4% |
| 6 | 8491ns | -4.7% | -5.0% | -2.2% | +0.0% | -5.0% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.112 | ok |
| carrier_opt_wideselect_cse | -0.001 | ok |
| carrier_opt_wideselect_cseeqsat | -0.135 | ok |
| carrier_opt_wideselect_dce | -0.022 | ok |
| carrier_opt_wideselect_eqsat | -0.631 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_fold | -0.128 | ok |
| carrier_opt_wideselect_none | 0.348 | moderate+ |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 5/6, lost 1/6
- **carrier_opt_wideselect_cseeqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 3/6, lost 2/6
- **carrier_opt_wideselect_eqsat**: won 6/6, lost 0/6
- **carrier_opt_wideselect_fold**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 87958.4ns | 8079.8ns | 1088.6% | HIGH |
| carrier_opt_wideselect_cse | 89078.2ns | 7988.1ns | 1115.1% | HIGH |
| carrier_opt_wideselect_cseeqsat | 90049.6ns | 8292.0ns | 1086.0% | HIGH |
| carrier_opt_wideselect_dce | 92346.0ns | 8513.1ns | 1084.8% | HIGH |
| carrier_opt_wideselect_eqsat | 90131.4ns | 8219.8ns | 1096.5% | HIGH |
| carrier_opt_wideselect_fold | 91463.9ns | 8458.8ns | 1081.3% | HIGH |
| carrier_opt_wideselect_none | 91936.7ns | 8500.8ns | 1081.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 7930.4-8130.2 ns)
   7930.4 |#############
   7940.4 |
   7950.4 |
   7960.4 |
   7970.4 |
   7980.4 |
   7990.3 |
   8000.3 |
   8010.3 |
   8020.3 |
   8030.3 |
   8040.3 |
   8050.3 |
   8060.3 |
   8070.3 |
   8080.2 |
   8090.2 |########################################
   8100.2 |#############
   8110.2 |
   8120.2 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 7053.3-8362.2 ns)
   7053.3 |#############
   7118.7 |
   7184.2 |
   7249.6 |
   7315.1 |
   7380.5 |
   7446.0 |
   7511.4 |
   7576.9 |
   7642.3 |
   7707.8 |
   7773.2 |
   7838.7 |
   7904.1 |
   7969.6 |#############
   8035.0 |########################################
   8100.5 |
   8165.9 |
   8231.4 |
   8296.8 |
  (0 below, 1 above range)

carrier_opt_wideselect_cseeqsat (n=6, range 8243.8-8340.5 ns)
   8243.8 |####################
   8248.6 |####################
   8253.5 |
   8258.3 |
   8263.1 |
   8268.0 |####################
   8272.8 |
   8277.6 |
   8282.5 |
   8287.3 |
   8292.1 |
   8297.0 |
   8301.8 |
   8306.6 |########################################
   8311.5 |
   8316.3 |
   8321.1 |
   8326.0 |
   8330.8 |
   8335.6 |
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 8471.2-8544.2 ns)
   8471.2 |########################################
   8474.9 |
   8478.5 |
   8482.2 |
   8485.8 |
   8489.5 |########################################
   8493.1 |
   8496.8 |
   8500.4 |
   8504.1 |
   8507.7 |########################################
   8511.4 |
   8515.0 |########################################
   8518.7 |
   8522.3 |
   8526.0 |
   8529.6 |
   8533.2 |
   8536.9 |########################################
   8540.6 |
  (0 below, 1 above range)

carrier_opt_wideselect_eqsat (n=6, range 8062.1-8363.5 ns)
   8062.1 |########################################
   8077.2 |
   8092.2 |
   8107.3 |
   8122.4 |
   8137.5 |
   8152.5 |
   8167.6 |
   8182.7 |
   8197.8 |
   8212.8 |####################
   8227.9 |
   8243.0 |####################
   8258.0 |
   8273.1 |
   8288.2 |
   8303.3 |
   8318.3 |####################
   8333.4 |
   8348.5 |
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 8378.8-8510.5 ns)
   8378.8 |########################################
   8385.4 |
   8392.0 |
   8398.5 |
   8405.1 |
   8411.7 |
   8418.3 |########################################
   8424.9 |
   8431.5 |
   8438.0 |########################################
   8444.6 |
   8451.2 |
   8457.8 |
   8464.4 |
   8471.0 |
   8477.5 |
   8484.1 |########################################
   8490.7 |########################################
   8497.3 |
   8503.9 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 8434.2-8549.3 ns)
   8434.2 |########################################
   8440.0 |
   8445.7 |
   8451.5 |########################################
   8457.2 |
   8463.0 |
   8468.7 |
   8474.5 |
   8480.3 |
   8486.0 |########################################
   8491.8 |
   8497.5 |
   8503.3 |
   8509.0 |
   8514.8 |
   8520.6 |
   8526.3 |########################################
   8532.1 |########################################
   8537.8 |
   8543.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=1086.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=1094.7% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cseeqsat**: bridge=1087.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=1084.4% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_eqsat**: bridge=1094.7% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=1080.5% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=1081.2% of algo (FFI overhead may distort results)
