# Residual encoding: predecoded register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 202% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (21.99 us) leads carrier_res_leaf_stack (66.44 us) by 202%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (21.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_res_leaf_register (21.99 us) to slowest carrier_res_leaf_stack (66.44 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 21991.7 ns median
- 1 variant significantly slower than baseline
- Spread: 3.02x (fastest 21991.7 ns, slowest 66441.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 24338ns | 24389ns | 23572ns | 24221ns | 24895ns | base |
| carrier_res_leaf_stack | 69202ns | 68817ns | 67865ns | 68541ns | 70860ns | +184.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 21881ns | 21136ns | 22254ns | base | 0.047 |
| carrier_res_leaf_stack | 66743ns | 65552ns | 68194ns | +205.03% | 0.015 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_leaf_register | 349661 | 1620983 | 0.216 | 1.00× |
| carrier_res_leaf_stack | 620456 | 2111900 | 0.294 | 1.77× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.048 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.047 | 96.1% |
| carrier_res_leaf_stack | 0.015 | 31.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 24338ns | 24338ns | base |
| carrier_res_leaf_stack | 69202ns | 69202ns | +184.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 21992ns | base | --- | [21397, 22254] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 66441ns | +44641.1ns (+203.0%) | [+43742, +46203]ns | [65593, 68194] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 21136ns | +210.1% |
| 2 | 22043ns | +204.6% |
| 3 | 21658ns | +203.0% |
| 4 | 22139ns | +202.7% |
| 5 | 21940ns | +215.6% |
| 6 | 22369ns | +194.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.181 | ok |
| carrier_res_leaf_stack | -0.277 | moderate- |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 90886.0ns | 21880.9ns | 415.4% | HIGH |
| carrier_res_leaf_stack | 133807.0ns | 66743.0ns | 200.5% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 21135.8-22254.0 ns)
  21135.8 |########################################
  21191.7 |
  21247.6 |
  21303.5 |
  21359.4 |
  21415.3 |
  21471.3 |
  21527.2 |
  21583.1 |
  21639.0 |########################################
  21694.9 |
  21750.8 |
  21806.7 |
  21862.6 |
  21918.5 |########################################
  21974.5 |
  22030.4 |########################################
  22086.3 |########################################
  22142.2 |
  22198.1 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 65552.1-68194.4 ns)
  65552.1 |########################################
  65684.2 |
  65816.3 |####################
  65948.4 |
  66080.6 |
  66212.7 |
  66344.8 |
  66476.9 |
  66609.0 |
  66741.1 |
  66873.2 |####################
  67005.4 |
  67137.5 |####################
  67269.6 |
  67401.7 |
  67533.8 |
  67665.9 |
  67798.1 |
  67930.2 |
  68062.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=409.6% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=200.8% of algo (FFI overhead may distort results)
