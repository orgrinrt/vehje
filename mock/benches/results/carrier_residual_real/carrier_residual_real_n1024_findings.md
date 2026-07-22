# Residual encoding: register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 160% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (45.19 us) leads carrier_res_real_stack (117.42 us) by 160%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (45.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 45192.1 ns median
- 1 variant significantly slower than baseline
- Spread: 2.60x (fastest 45192.1 ns, slowest 117416.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 47527ns | 47718ns | 43876ns | 47290ns | 49710ns | base |
| carrier_res_real_stack | 121210ns | 119708ns | 115440ns | 118852ns | 127632ns | +155.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 44994ns | 41465ns | 47037ns | base | 0.023 |
| carrier_res_real_stack | 118787ns | 113122ns | 124984ns | +164.01% | 0.009 |

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.023 | 91.8% |
| carrier_res_real_stack | 0.009 | 35.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 47527ns | 47527ns | base |
| carrier_res_real_stack | 121210ns | 121210ns | +155.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 45192ns | base | --- | [42752, 47037] | --- | --- | --- | --- |
| carrier_res_real_stack | 117417ns | +71742.5ns (+158.8%) | [+68536, +81102]ns | [113961, 124984] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 47818ns | +151.5% |
| 2 | 44039ns | +161.3% |
| 3 | 46255ns | +144.6% |
| 4 | 44594ns | +157.4% |
| 5 | 45790ns | +161.6% |
| 6 | 41465ns | +212.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.318 | moderate- |
| carrier_res_real_stack | 0.244 | moderate+ |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 109999.9ns | 44993.5ns | 244.5% | HIGH |
| carrier_res_real_stack | 109704.0ns | 118787.1ns | 92.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 41464.6-47036.7 ns)
  41464.6 |########################################
  41743.2 |
  42021.8 |
  42300.4 |
  42579.0 |
  42857.6 |
  43136.2 |
  43414.8 |
  43693.4 |
  43972.0 |########################################
  44250.6 |
  44529.2 |########################################
  44807.8 |
  45086.4 |
  45365.0 |
  45643.6 |########################################
  45922.2 |
  46200.8 |########################################
  46479.4 |
  46758.0 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 113122.1-124983.9 ns)
  113122.1 |########################################
  113715.2 |
  114308.3 |########################################
  114901.4 |########################################
  115494.5 |
  116087.6 |
  116680.7 |
  117273.7 |
  117866.8 |
  118459.9 |
  119053.0 |
  119646.1 |########################################
  120239.2 |########################################
  120832.3 |
  121425.4 |
  122018.5 |
  122611.6 |
  123204.7 |
  123797.8 |
  124390.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=235.2% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=92.3% of algo (FFI overhead may distort results)
