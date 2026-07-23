# Residual encoding: predecoded register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 38% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (438.63 us) leads carrier_res_real_stack (605.52 us) by 38%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_real_stack shows alternating (throttle bounce) (autocorr -0.56)

carrier_res_real_stack's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (438.63 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 438629.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.38x (fastest 438629.4 ns, slowest 605522.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 439963ns | 441425ns | 422975ns | 439083ns | 449777ns | base |
| carrier_res_real_stack | 611851ns | 608746ns | 604578ns | 607538ns | 621957ns | +39.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 437009ns | 419836ns | 446875ns | base | 0.009 |
| carrier_res_real_stack | 608410ns | 600782ns | 618576ns | +39.22% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_real_register | 2848184 | 2276363 | 1.251 | 1.00× |
| carrier_res_real_stack | 3840997 | 8655589 | 0.444 | 1.35× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.009 | 95.7% |
| carrier_res_real_stack | 0.007 | 69.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 439963ns | 439963ns | base |
| carrier_res_real_stack | 611851ns | 611851ns | +39.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 438629ns | base | --- | [425521, 446875] | --- | --- | --- | --- |
| carrier_res_real_stack | 605522ns | +169852.7ns (+38.7%) | [+164552, +179800]ns | [601132, 618576] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 419836ns | +43.3% |
| 2 | 451258ns | +36.7% |
| 3 | 441760ns | +37.0% |
| 4 | 442492ns | +40.2% |
| 5 | 431206ns | +39.3% |
| 6 | 435499ns | +39.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.297 | moderate- |
| carrier_res_real_stack | -0.561 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 471554.9ns | 437008.5ns | 107.9% | HIGH |
| carrier_res_real_stack | 616761.4ns | 608410.1ns | 101.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 419835.8-446875.2 ns)
  419835.8 |####################
  421187.8 |
  422539.7 |
  423891.7 |
  425243.7 |
  426595.6 |
  427947.6 |
  429299.6 |
  430651.6 |####################
  432003.5 |
  433355.5 |
  434707.5 |####################
  436059.4 |
  437411.4 |
  438763.4 |
  440115.3 |
  441467.3 |########################################
  442819.3 |
  444171.3 |
  445523.2 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 600781.7-618576.2 ns)
  600781.7 |########################################
  601671.4 |
  602561.2 |
  603450.9 |
  604340.6 |
  605230.3 |########################################
  606120.1 |
  607009.8 |
  607899.5 |
  608789.2 |
  609679.0 |
  610568.7 |
  611458.4 |
  612348.2 |
  613237.9 |
  614127.6 |
  615017.3 |
  615907.1 |####################
  616796.8 |
  617686.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=107.3% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=102.1% of algo (FFI overhead may distort results)
