# Residual encoding: register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 91% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (198.42 us) leads carrier_res_madd_stack (379.46 us) by 91%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (198.42 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 198418.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 198418.8 ns, slowest 379462.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 200944ns | 201306ns | 198589ns | 200978ns | 202071ns | base |
| carrier_res_madd_stack | 382151ns | 382349ns | 380867ns | 382124ns | 382833ns | +90.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 198227ns | 195902ns | 199599ns | base | 0.021 |
| carrier_res_madd_stack | 379545ns | 378436ns | 380281ns | +91.47% | 0.011 |

## Performance model

- Peak throughput: **0.021 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.021 | 98.7% |
| carrier_res_madd_stack | 0.011 | 51.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 200944ns | 200944ns | base |
| carrier_res_madd_stack | 382151ns | 382151ns | +90.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 198419ns | base | --- | [196662, 199599] | --- | --- | --- | --- |
| carrier_res_madd_stack | 379462ns | +181212.1ns (+91.3%) | [+179292, +183451]ns | [378891, 380281] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 195902ns | +94.4% |
| 2 | 198632ns | +91.2% |
| 3 | 199380ns | +89.8% |
| 4 | 198205ns | +91.4% |
| 5 | 197421ns | +92.2% |
| 6 | 199819ns | +89.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.175 | ok |
| carrier_res_madd_stack | 0.056 | ok |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 199120.0ns | 198226.5ns | 100.5% | HIGH |
| carrier_res_madd_stack | 382634.0ns | 379544.9ns | 100.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 195902.5-199599.2 ns)
  195902.5 |########################################
  196087.3 |
  196272.2 |
  196457.0 |
  196641.8 |
  196826.7 |
  197011.5 |
  197196.3 |
  197381.2 |########################################
  197566.0 |
  197750.8 |
  197935.7 |
  198120.5 |########################################
  198305.3 |
  198490.2 |########################################
  198675.0 |
  198859.8 |
  199044.7 |
  199229.5 |########################################
  199414.3 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 378436.2-380280.8 ns)
  378436.2 |########################################
  378528.4 |
  378620.7 |
  378712.9 |
  378805.1 |
  378897.4 |
  378989.6 |
  379081.8 |
  379174.1 |
  379266.3 |########################################
  379358.5 |########################################
  379450.8 |########################################
  379543.0 |
  379635.2 |
  379727.5 |########################################
  379819.7 |
  379911.9 |
  380004.2 |
  380096.4 |
  380188.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=100.8% of algo (FFI overhead may distort results)
