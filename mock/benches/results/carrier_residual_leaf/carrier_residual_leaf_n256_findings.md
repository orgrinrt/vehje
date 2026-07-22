# Residual encoding: register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 36% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (9.13 us) leads carrier_res_leaf_stack (12.45 us) by 36%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_leaf_register shows alternating (throttle bounce) (autocorr -0.68)

carrier_res_leaf_register's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (9.13 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 9133.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.36x (fastest 9133.4 ns, slowest 12449.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 11496ns | 11499ns | 10792ns | 11327ns | 12102ns | base |
| carrier_res_leaf_stack | 14958ns | 14769ns | 13980ns | 14599ns | 15988ns | +30.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 9143ns | 8583ns | 9644ns | base | 0.028 |
| carrier_res_leaf_stack | 12616ns | 11733ns | 13524ns | +37.99% | 0.020 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.028 | 94.0% |
| carrier_res_leaf_stack | 0.021 | 68.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 11496ns | 11496ns | base |
| carrier_res_leaf_stack | 14958ns | 14958ns | +30.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 9133ns | base | --- | [8651, 9644] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 12450ns | +3489.4ns (+38.2%) | [+2976, +3955]ns | [11875, 13524] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 9269ns | +39.9% |
| 2 | 9420ns | +31.2% |
| 3 | 8583ns | +40.0% |
| 4 | 9869ns | +42.7% |
| 5 | 8718ns | +34.6% |
| 6 | 8998ns | +39.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.681 | HIGH- (thermal bounce) |
| carrier_res_leaf_stack | -0.586 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 91573.2ns | 9142.9ns | 1001.6% | HIGH |
| carrier_res_leaf_stack | 78980.1ns | 12616.3ns | 626.0% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 8583.3-9644.4 ns)
   8583.3 |########################################
   8636.4 |
   8689.4 |########################################
   8742.5 |
   8795.5 |
   8848.6 |
   8901.6 |
   8954.7 |########################################
   9007.7 |
   9060.8 |
   9113.9 |
   9166.9 |
   9220.0 |########################################
   9273.0 |
   9326.1 |
   9379.1 |########################################
   9432.2 |
   9485.2 |
   9538.3 |
   9591.3 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 11732.9-13524.0 ns)
  11732.9 |########################################
  11822.5 |
  11912.0 |
  12001.6 |########################################
  12091.1 |
  12180.7 |
  12270.2 |########################################
  12359.8 |
  12449.3 |
  12538.9 |########################################
  12628.4 |
  12718.0 |
  12807.5 |
  12897.1 |########################################
  12986.6 |
  13076.2 |
  13165.7 |
  13255.3 |
  13344.8 |
  13434.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=994.8% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=623.2% of algo (FFI overhead may distort results)
