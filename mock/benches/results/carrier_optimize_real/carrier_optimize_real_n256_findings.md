# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

7 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_opt_real_all, carrier_opt_real_eqsat) are a dead heat (<1%)

carrier_opt_real_all (8.48 us) and carrier_opt_real_eqsat (8.49 us) differ by 0.13%, inside the noise, even though the wider field spreads 7.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_opt_real_all shows alternating (throttle bounce) (autocorr -0.82)

carrier_opt_real_all's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_real_all** at 8475.6 ns median (-5.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.08x (fastest 8475.6 ns, slowest 9143.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 11056ns | 11042ns | 10872ns | 11022ns | 11199ns | -3.21% |
| carrier_opt_real_cse | 10923ns | 11142ns | 9685ns | 11121ns | 11246ns | -4.37% |
| carrier_opt_real_cseeqsat | 11096ns | 11123ns | 10855ns | 11037ns | 11304ns | -2.86% |
| carrier_opt_real_dce | 11659ns | 11604ns | 11439ns | 11585ns | 11880ns | +2.06% |
| carrier_opt_real_eqsat | 11067ns | 11070ns | 10974ns | 11056ns | 11130ns | -3.12% |
| carrier_opt_real_fold | 11508ns | 11456ns | 11397ns | 11451ns | 11648ns | +0.75% |
| carrier_opt_real_none | 11423ns | 11569ns | 10388ns | 11539ns | 11766ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 8489ns | 8386ns | 8602ns | -4.53% | 0.030 |
| carrier_opt_real_cse | 8434ns | 7508ns | 8642ns | -5.14% | 0.030 |
| carrier_opt_real_cseeqsat | 8505ns | 8323ns | 8636ns | -4.35% | 0.030 |
| carrier_opt_real_dce | 9140ns | 8969ns | 9245ns | +2.80% | 0.028 |
| carrier_opt_real_eqsat | 8494ns | 8434ns | 8556ns | -4.47% | 0.030 |
| carrier_opt_real_fold | 8965ns | 8926ns | 9005ns | +0.83% | 0.029 |
| carrier_opt_real_none | 8892ns | 8088ns | 9155ns | base | 0.029 |

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_opt_real_cse; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.030 | 88.6% |
| carrier_opt_real_cse | 0.030 | 87.2% |
| carrier_opt_real_cseeqsat | 0.030 | 88.3% |
| carrier_opt_real_dce | 0.028 | 82.1% |
| carrier_opt_real_eqsat | 0.030 | 88.5% |
| carrier_opt_real_fold | 0.029 | 83.8% |
| carrier_opt_real_none | 0.028 | 83.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 11056ns | 11056ns | -3.21% |
| carrier_opt_real_cse | 10923ns | 10923ns | -4.37% |
| carrier_opt_real_cseeqsat | 11096ns | 11096ns | -2.86% |
| carrier_opt_real_dce | 11659ns | 11659ns | +2.06% |
| carrier_opt_real_eqsat | 11067ns | 11067ns | -3.12% |
| carrier_opt_real_fold | 11508ns | 11508ns | +0.75% |
| carrier_opt_real_none | 11423ns | 11423ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 9001ns | base | --- | [8519, 9155] | --- | --- | --- | --- |
| carrier_opt_real_all | 8476ns | -525.3ns (-5.8%) | [-676, -9]ns | [8388, 8602] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| carrier_opt_real_cse | 8611ns | no significant difference | [-1060, +103]ns | [8050, 8642] | no | 0.3281 | 0.2188 | 0 |
| carrier_opt_real_cseeqsat | 8501ns | -500.1ns (-5.6%) | [-548, -111]ns | [8379, 8636] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| carrier_opt_real_dce | 9144ns | no significant difference | [-40, +642]ns | [9032, 9245] | no | 0.6875 | 0.6875 | 0 |
| carrier_opt_real_eqsat | 8486ns | -525.8ns (-5.8%) | [-634, -33]ns | [8440, 8556] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| carrier_opt_real_fold | 8955ns | no significant difference | [-202, +483]ns | [8937, 9005] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_cse | carrier_opt_real_cseeqsat | carrier_opt_real_dce | carrier_opt_real_eqsat | carrier_opt_real_fold |
|---|---|---|---|---|---|---|---|
| 1 | 8997ns | -6.0% | -16.6% | -5.7% | +1.1% | -6.1% | -0.8% |
| 2 | 9225ns | -7.1% | -6.8% | -6.3% | -0.5% | -7.6% | -2.9% |
| 3 | 8088ns | +3.7% | +6.4% | +2.9% | +12.6% | +5.3% | +11.9% |
| 4 | 8950ns | -3.5% | -3.4% | -5.8% | +3.0% | -5.5% | +0.0% |
| 5 | 9086ns | -7.7% | -5.1% | -5.0% | +2.1% | -5.5% | -1.5% |
| 6 | 9005ns | -5.7% | -4.0% | -5.4% | -0.4% | -6.3% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.821 | HIGH- (thermal bounce) |
| carrier_opt_real_cse | -0.008 | ok |
| carrier_opt_real_cseeqsat | -0.308 | moderate- |
| carrier_opt_real_dce | -0.321 | moderate- |
| carrier_opt_real_eqsat | -0.621 | HIGH- (thermal bounce) |
| carrier_opt_real_fold | -0.109 | ok |
| carrier_opt_real_none | -0.300 | moderate- |

**Consistency summary:**

- **carrier_opt_real_all**: won 5/6, lost 1/6
- **carrier_opt_real_cse**: won 5/6, lost 1/6
- **carrier_opt_real_cseeqsat**: won 5/6, lost 1/6
- **carrier_opt_real_dce**: won 2/6, lost 4/6
- **carrier_opt_real_eqsat**: won 5/6, lost 1/6
- **carrier_opt_real_fold**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 91197.2ns | 8488.7ns | 1074.3% | HIGH |
| carrier_opt_real_cse | 87473.4ns | 8434.4ns | 1037.1% | HIGH |
| carrier_opt_real_cseeqsat | 91875.8ns | 8505.3ns | 1080.2% | HIGH |
| carrier_opt_real_dce | 89994.2ns | 9140.3ns | 984.6% | HIGH |
| carrier_opt_real_eqsat | 91728.2ns | 8494.2ns | 1079.9% | HIGH |
| carrier_opt_real_fold | 89298.1ns | 8965.5ns | 996.0% | HIGH |
| carrier_opt_real_none | 88942.2ns | 8891.8ns | 1000.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 8386.2-8602.1 ns)
   8386.2 |########################################
   8397.0 |
   8407.8 |
   8418.6 |
   8429.4 |
   8440.2 |
   8451.0 |####################
   8461.8 |
   8472.6 |
   8483.4 |####################
   8494.2 |
   8504.9 |
   8515.7 |
   8526.5 |
   8537.3 |
   8548.1 |
   8558.9 |####################
   8569.7 |
   8580.5 |
   8591.3 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 7507.5-8642.1 ns)
   7507.5 |#############
   7564.2 |
   7621.0 |
   7677.7 |
   7734.4 |
   7791.1 |
   7847.9 |
   7904.6 |
   7961.3 |
   8018.1 |
   8074.8 |
   8131.5 |
   8188.3 |
   8245.0 |
   8301.7 |
   8358.5 |
   8415.2 |
   8471.9 |
   8528.6 |
   8585.4 |########################################
  (0 below, 2 above range)

carrier_opt_real_cseeqsat (n=6, range 8323.3-8636.2 ns)
   8323.3 |########################################
   8338.9 |
   8354.6 |
   8370.2 |
   8385.9 |
   8401.5 |
   8417.2 |
   8432.8 |########################################
   8448.5 |
   8464.1 |
   8479.8 |########################################
   8495.4 |
   8511.1 |########################################
   8526.7 |
   8542.4 |
   8558.0 |
   8573.7 |
   8589.3 |
   8605.0 |
   8620.6 |########################################
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 8968.8-9245.4 ns)
   8968.8 |####################
   8982.6 |
   8996.5 |
   9010.3 |
   9024.1 |
   9038.0 |
   9051.8 |
   9065.6 |
   9079.4 |
   9093.3 |########################################
   9107.1 |
   9120.9 |
   9134.8 |
   9148.6 |
   9162.4 |
   9176.2 |####################
   9190.1 |
   9203.9 |####################
   9217.7 |
   9231.6 |
  (0 below, 1 above range)

carrier_opt_real_eqsat (n=6, range 8433.8-8555.9 ns)
   8433.8 |########################################
   8439.9 |
   8446.0 |########################################
   8452.1 |########################################
   8458.2 |
   8464.3 |
   8470.4 |
   8476.5 |
   8482.6 |
   8488.7 |
   8494.8 |
   8500.9 |
   8507.0 |
   8513.1 |########################################
   8519.2 |
   8525.3 |########################################
   8531.4 |
   8537.5 |
   8543.6 |
   8549.7 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 8926.2-9005.0 ns)
   8926.2 |########################################
   8930.1 |
   8934.1 |
   8938.0 |
   8942.0 |
   8945.9 |########################################
   8949.8 |########################################
   8953.8 |########################################
   8957.7 |########################################
   8961.7 |
   8965.6 |
   8969.5 |
   8973.5 |
   8977.4 |
   8981.4 |
   8985.3 |
   8989.2 |
   8993.2 |
   8997.1 |
   9001.1 |
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 8087.9-9155.4 ns)
   8087.9 |####################
   8141.3 |
   8194.6 |
   8248.0 |
   8301.4 |
   8354.8 |
   8408.1 |
   8461.5 |
   8514.9 |
   8568.3 |
   8621.7 |
   8675.0 |
   8728.4 |
   8781.8 |
   8835.2 |
   8888.5 |
   8941.9 |####################
   8995.3 |########################################
   9048.7 |####################
   9102.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=1075.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=1009.3% of algo (FFI overhead may distort results)
- **carrier_opt_real_cseeqsat**: bridge=1079.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=985.6% of algo (FFI overhead may distort results)
- **carrier_opt_real_eqsat**: bridge=1082.7% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=996.6% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=987.5% of algo (FFI overhead may distort results)
