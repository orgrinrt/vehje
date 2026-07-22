# Residual encoding: register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 54% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (1.97 us) leads carrier_res_leaf_stack (3.04 us) by 54%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_leaf_stack shows alternating (throttle bounce) (autocorr -0.72)

carrier_res_leaf_stack's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (1.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 1974.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.54x (fastest 1974.8 ns, slowest 3043.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 4421ns | 4325ns | 4138ns | 4290ns | 4759ns | base |
| carrier_res_leaf_stack | 5645ns | 5481ns | 5193ns | 5406ns | 6229ns | +27.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 2008ns | 1916ns | 2112ns | base | 0.032 |
| carrier_res_leaf_stack | 3136ns | 2858ns | 3450ns | +56.17% | 0.020 |

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.032 | 97.0% |
| carrier_res_leaf_stack | 0.021 | 63.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 4421ns | 4421ns | base |
| carrier_res_leaf_stack | 5645ns | 5645ns | +27.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 1975ns | base | --- | [1937, 2112] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 3044ns | +1015.8ns (+51.4%) | [+938, +1429]ns | [2913, 3450] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 1978ns | +50.0% |
| 2 | 1916ns | +75.0% |
| 3 | 1958ns | +51.6% |
| 4 | 2126ns | +66.9% |
| 5 | 1971ns | +45.0% |
| 6 | 2098ns | +48.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.177 | ok |
| carrier_res_leaf_stack | -0.719 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 86574.4ns | 2007.8ns | 4311.9% | HIGH |
| carrier_res_leaf_stack | 72898.1ns | 3135.5ns | 2324.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 1916.2-2111.9 ns)
   1916.2 |########################################
   1926.0 |
   1935.8 |
   1945.5 |
   1955.3 |########################################
   1965.1 |########################################
   1974.9 |########################################
   1984.7 |
   1994.5 |
   2004.2 |
   2014.0 |
   2023.8 |
   2033.6 |
   2043.4 |
   2053.2 |
   2062.9 |
   2072.7 |
   2082.5 |
   2092.3 |########################################
   2102.1 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 2857.9-3450.4 ns)
   2857.9 |####################
   2887.5 |
   2917.2 |
   2946.8 |########################################
   2976.4 |
   3006.0 |
   3035.7 |
   3065.3 |
   3094.9 |####################
   3124.5 |
   3154.1 |
   3183.8 |
   3213.4 |
   3243.0 |
   3272.6 |
   3302.3 |
   3331.9 |####################
   3361.5 |
   3391.1 |
   3420.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=4390.0% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=2385.2% of algo (FFI overhead may distort results)
