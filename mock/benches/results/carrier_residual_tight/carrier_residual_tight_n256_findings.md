# Residual encoding: predecoded register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 221% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (7.17 us) leads carrier_res_tight_stack (23.04 us) by 221%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (7.17 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest carrier_res_tight_register (7.17 us) to slowest carrier_res_tight_stack (23.04 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 7175.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.21x (fastest 7175.0 ns, slowest 23042.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 9574ns | 9535ns | 9249ns | 9478ns | 9879ns | base |
| carrier_res_tight_stack | 25470ns | 25486ns | 24742ns | 25316ns | 26065ns | +166.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 7180ns | 6875ns | 7422ns | base | 0.036 |
| carrier_res_tight_stack | 23040ns | 22420ns | 23548ns | +220.89% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_tight_register | 300849 | 960160 | 0.313 | 1.00× |
| carrier_res_tight_stack | 352346 | 1470521 | 0.240 | 1.17× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.036 | 95.8% |
| carrier_res_tight_stack | 0.011 | 29.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 9574ns | 9574ns | base |
| carrier_res_tight_stack | 25470ns | 25470ns | +166.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 7175ns | base | --- | [6944, 7422] | --- | --- | --- | --- |
| carrier_res_tight_stack | 23042ns | +15867.2ns (+221.1%) | [+15588, +16126]ns | [22531, 23548] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 6875ns | +226.1% |
| 2 | 7206ns | +220.1% |
| 3 | 7013ns | +222.9% |
| 4 | 7484ns | +215.5% |
| 5 | 7144ns | +222.2% |
| 6 | 7359ns | +219.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.324 | moderate- |
| carrier_res_tight_stack | -0.254 | moderate- |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 90148.9ns | 7180.1ns | 1255.5% | HIGH |
| carrier_res_tight_stack | 92368.5ns | 23040.3ns | 400.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 6874.6-7421.7 ns)
   6874.6 |########################################
   6902.0 |
   6929.3 |
   6956.7 |
   6984.0 |
   7011.4 |########################################
   7038.7 |
   7066.1 |
   7093.4 |
   7120.8 |########################################
   7148.1 |
   7175.5 |
   7202.9 |########################################
   7230.2 |
   7257.6 |
   7284.9 |
   7312.3 |
   7339.6 |########################################
   7367.0 |
   7394.3 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 22420.0-23547.5 ns)
  22420.0 |########################################
  22476.4 |
  22532.8 |
  22589.1 |########################################
  22645.5 |
  22701.9 |
  22758.2 |
  22814.6 |
  22871.0 |
  22927.4 |
  22983.8 |########################################
  23040.1 |########################################
  23096.5 |
  23152.9 |
  23209.2 |
  23265.6 |
  23322.0 |
  23378.4 |
  23434.8 |########################################
  23491.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=1258.2% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=400.2% of algo (FFI overhead may distort results)
