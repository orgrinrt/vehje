# Residual encoding: register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_stack shows alternating (throttle bounce) (autocorr -0.73)

carrier_res_leaf_stack's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (1.06 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 1062350.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 1062350.0 ns, slowest 1145374.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 1066245ns | 1065813ns | 1058243ns | 1065032ns | 1072066ns | base |
| carrier_res_leaf_stack | 1149204ns | 1147684ns | 1143109ns | 1146260ns | 1156667ns | +7.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 1062756ns | 1054360ns | 1068322ns | base | 0.015 |
| carrier_res_leaf_stack | 1146866ns | 1140676ns | 1154336ns | +7.91% | 0.014 |

## Performance model

- Peak throughput: **0.016 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.015 | 99.2% |
| carrier_res_leaf_stack | 0.014 | 92.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 1066245ns | 1066245ns | base |
| carrier_res_leaf_stack | 1149204ns | 1149204ns | +7.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 1062350ns | base | --- | [1057595, 1068322] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 1145375ns | +84511.3ns (+8.0%) | [+78536, +89283]ns | [1140886, 1154336] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 1054360ns | +8.2% |
| 2 | 1068568ns | +8.4% |
| 3 | 1062525ns | +7.4% |
| 4 | 1060829ns | +8.4% |
| 5 | 1062175ns | +7.4% |
| 6 | 1068076ns | +7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.378 | moderate- |
| carrier_res_leaf_stack | -0.728 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 1064684.4ns | 1062755.6ns | 100.2% | HIGH |
| carrier_res_leaf_stack | 1296270.9ns | 1146865.8ns | 113.0% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 1054360.4-1068322.2 ns)
  1054360.4 |####################
  1055058.5 |
  1055756.6 |
  1056454.7 |
  1057152.8 |
  1057850.9 |
  1058549.0 |
  1059247.0 |
  1059945.1 |
  1060643.2 |####################
  1061341.3 |
  1062039.4 |########################################
  1062737.5 |
  1063435.6 |
  1064133.7 |
  1064831.8 |
  1065529.9 |
  1066228.0 |
  1066926.1 |
  1067624.2 |####################
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 1140675.8-1154336.4 ns)
  1140675.8 |########################################
  1141358.8 |
  1142041.9 |
  1142724.9 |
  1143407.9 |
  1144091.0 |
  1144774.0 |
  1145457.0 |
  1146140.1 |
  1146823.1 |
  1147506.1 |
  1148189.2 |
  1148872.2 |#############
  1149555.2 |#############
  1150238.3 |
  1150921.3 |
  1151604.3 |
  1152287.4 |
  1152970.4 |
  1153653.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=113.2% of algo (FFI overhead may distort results)
