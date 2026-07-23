# Residual encoding: predecoded register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 171% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (1.20 us) leads carrier_res_leaf_stack (3.26 us) by 171%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_leaf_register is fastest but the noisiest (CV 5.7%)

carrier_res_leaf_register wins on median (1.20 us) yet has the highest variance (CV 5.7%), while carrier_res_leaf_stack is the steadiest (CV 1.5%, 3.26 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (1.20 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 1202.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.71x (fastest 1202.5 ns, slowest 3255.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 3742ns | 3699ns | 3545ns | 3671ns | 3946ns | base |
| carrier_res_leaf_stack | 5792ns | 5801ns | 5591ns | 5778ns | 5912ns | +54.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 1224ns | 1163ns | 1292ns | base | 0.052 |
| carrier_res_leaf_stack | 3248ns | 3184ns | 3304ns | +165.42% | 0.020 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_leaf_register | 246326 | 1403838 | 0.175 | 1.00× |
| carrier_res_leaf_stack | 223884 | 998376 | 0.224 | 0.91× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.055 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.053 | 96.7% |
| carrier_res_leaf_stack | 0.020 | 35.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 3742ns | 3742ns | base |
| carrier_res_leaf_stack | 5792ns | 5792ns | +54.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 1202ns | base | --- | [1177, 1292] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 3255ns | +2026.2ns (+168.5%) | [+1941, +2106]ns | [3187, 3304] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 1373ns | +139.0% |
| 2 | 1192ns | +174.0% |
| 3 | 1210ns | +163.1% |
| 4 | 1210ns | +163.5% |
| 5 | 1163ns | +179.0% |
| 6 | 1195ns | +178.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.056 | ok |
| carrier_res_leaf_stack | 0.216 | moderate+ |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 84251.1ns | 1223.9ns | 6883.9% | HIGH |
| carrier_res_leaf_stack | 73607.8ns | 3248.5ns | 2265.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 1162.9-1291.8 ns)
   1162.9 |####################
   1169.3 |
   1175.8 |
   1182.2 |
   1188.7 |########################################
   1195.1 |
   1201.6 |
   1208.0 |########################################
   1214.5 |
   1220.9 |
   1227.4 |
   1233.8 |
   1240.3 |
   1246.7 |
   1253.2 |
   1259.6 |
   1266.1 |
   1272.5 |
   1279.0 |
   1285.4 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 3184.2-3303.6 ns)
   3184.2 |########################################
   3190.2 |
   3196.1 |
   3202.1 |
   3208.1 |
   3214.0 |
   3220.0 |
   3226.0 |
   3231.9 |
   3237.9 |
   3243.9 |####################
   3249.8 |
   3255.8 |
   3261.8 |####################
   3267.7 |
   3273.7 |
   3279.7 |####################
   3285.6 |
   3291.6 |
   3297.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=7006.7% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=2253.4% of algo (FFI overhead may distort results)
