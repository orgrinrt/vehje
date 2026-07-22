# Residual encoding: register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 98% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (47.00 us) leads carrier_res_madd_stack (93.07 us) by 98%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (47.00 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 47001.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.98x (fastest 47001.8 ns, slowest 93069.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 49715ns | 49184ns | 48740ns | 49151ns | 51047ns | base |
| carrier_res_madd_stack | 95190ns | 95366ns | 91251ns | 95038ns | 97387ns | +91.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 47452ns | 46575ns | 48740ns | base | 0.022 |
| carrier_res_madd_stack | 92770ns | 89092ns | 94810ns | +95.50% | 0.011 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.022 | 99.1% |
| carrier_res_madd_stack | 0.011 | 50.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 49715ns | 49715ns | base |
| carrier_res_madd_stack | 95190ns | 95190ns | +91.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 47002ns | base | --- | [46615, 48740] | --- | --- | --- | --- |
| carrier_res_madd_stack | 93069ns | +45445.0ns (+96.7%) | [+42722, +47786]ns | [90431, 94810] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 47064ns | +95.0% |
| 2 | 47135ns | +103.6% |
| 3 | 46575ns | +100.3% |
| 4 | 50346ns | +86.0% |
| 5 | 46940ns | +89.8% |
| 6 | 46654ns | +99.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.312 | moderate- |
| carrier_res_madd_stack | -0.173 | ok |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 95080.8ns | 47452.3ns | 200.4% | HIGH |
| carrier_res_madd_stack | 98227.9ns | 92769.9ns | 105.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 46575.4-48740.2 ns)
  46575.4 |########################################
  46683.6 |
  46791.9 |
  46900.1 |####################
  47008.4 |####################
  47116.6 |####################
  47224.8 |
  47333.1 |
  47441.3 |
  47549.6 |
  47657.8 |
  47766.0 |
  47874.3 |
  47982.5 |
  48090.8 |
  48199.0 |
  48307.2 |
  48415.5 |
  48523.7 |
  48632.0 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 89091.7-94809.8 ns)
  89091.7 |########################################
  89377.6 |
  89663.5 |
  89949.4 |
  90235.3 |
  90521.2 |
  90807.1 |
  91093.0 |
  91378.9 |
  91664.8 |########################################
  91950.8 |
  92236.7 |
  92522.6 |
  92808.5 |########################################
  93094.4 |########################################
  93380.3 |########################################
  93666.2 |
  93952.1 |
  94238.0 |
  94523.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=105.9% of algo (FFI overhead may distort results)
