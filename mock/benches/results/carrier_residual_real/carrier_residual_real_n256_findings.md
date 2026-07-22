# Residual encoding: register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 141% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (10.44 us) leads carrier_res_real_stack (25.12 us) by 141%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (10.44 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 10439.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.41x (fastest 10439.6 ns, slowest 25117.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 12696ns | 12736ns | 12080ns | 12632ns | 13100ns | base |
| carrier_res_real_stack | 27566ns | 27582ns | 26295ns | 27304ns | 28595ns | +117.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 10382ns | 9891ns | 10702ns | base | 0.025 |
| carrier_res_real_stack | 25104ns | 24052ns | 25986ns | +141.82% | 0.010 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.025 | 94.7% |
| carrier_res_real_stack | 0.010 | 39.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 12696ns | 12696ns | base |
| carrier_res_real_stack | 27566ns | 27566ns | +117.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 10440ns | base | --- | [10003, 10702] | --- | --- | --- | --- |
| carrier_res_real_stack | 25118ns | +14802.1ns (+141.8%) | [+13820, +15546]ns | [24209, 25986] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 9891ns | +143.2% |
| 2 | 10494ns | +147.4% |
| 3 | 10517ns | +138.6% |
| 4 | 10385ns | +150.5% |
| 5 | 10114ns | +148.6% |
| 6 | 10888ns | +123.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.293 | moderate- |
| carrier_res_real_stack | -0.286 | moderate- |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 92053.1ns | 10381.6ns | 886.7% | HIGH |
| carrier_res_real_stack | 98391.1ns | 25104.3ns | 391.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 9891.2-10702.5 ns)
   9891.2 |########################################
   9931.8 |
   9972.3 |
  10012.9 |
  10053.5 |
  10094.0 |########################################
  10134.6 |
  10175.2 |
  10215.7 |
  10256.3 |
  10296.9 |
  10337.4 |
  10378.0 |########################################
  10418.5 |
  10459.1 |########################################
  10499.7 |########################################
  10540.2 |
  10580.8 |
  10621.4 |
  10661.9 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 24052.1-25985.8 ns)
  24052.1 |########################################
  24148.8 |
  24245.5 |
  24342.2 |########################################
  24438.8 |
  24535.5 |
  24632.2 |
  24728.9 |
  24825.6 |
  24922.3 |
  25019.0 |########################################
  25115.6 |########################################
  25212.3 |
  25309.0 |
  25405.7 |
  25502.4 |
  25599.1 |
  25695.7 |
  25792.4 |
  25889.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=884.1% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=393.2% of algo (FFI overhead may distort results)
