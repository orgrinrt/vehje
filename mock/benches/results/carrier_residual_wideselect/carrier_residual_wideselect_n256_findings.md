# Residual encoding: register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 121% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (10.34 us) leads carrier_res_wideselect_stack (22.82 us) by 121%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_wideselect_register shows alternating (throttle bounce) (autocorr -0.50)

carrier_res_wideselect_register's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (10.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 10339.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.21x (fastest 10339.6 ns, slowest 22823.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 12808ns | 12798ns | 11990ns | 12666ns | 13430ns | base |
| carrier_res_wideselect_stack | 25333ns | 25150ns | 24385ns | 24975ns | 26344ns | +97.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 10297ns | 9717ns | 10749ns | base | 0.025 |
| carrier_res_wideselect_stack | 23004ns | 22096ns | 23980ns | +123.40% | 0.011 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.025 | 94.0% |
| carrier_res_wideselect_stack | 0.011 | 42.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 12808ns | 12808ns | base |
| carrier_res_wideselect_stack | 25333ns | 25333ns | +97.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 10340ns | base | --- | [9804, 10749] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 22823ns | +12506.0ns (+121.0%) | [+11829, +13785]ns | [22209, 23980] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 10180ns | +117.1% |
| 2 | 10918ns | +113.2% |
| 3 | 9891ns | +137.9% |
| 4 | 10500ns | +132.7% |
| 5 | 10580ns | +111.0% |
| 6 | 9717ns | +130.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.503 | HIGH- (thermal bounce) |
| carrier_res_wideselect_stack | 0.026 | ok |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 90123.8ns | 10297.4ns | 875.2% | HIGH |
| carrier_res_wideselect_stack | 92454.2ns | 23003.9ns | 401.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 9716.7-10748.8 ns)
   9716.7 |########################################
   9768.3 |
   9819.9 |
   9871.5 |########################################
   9923.1 |
   9974.7 |
  10026.3 |
  10077.9 |
  10129.5 |########################################
  10181.1 |
  10232.7 |
  10284.3 |
  10335.9 |
  10387.5 |
  10439.1 |
  10490.7 |########################################
  10542.3 |########################################
  10593.9 |
  10645.5 |
  10697.1 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 22096.2-23979.8 ns)
  22096.2 |####################
  22190.4 |
  22284.6 |########################################
  22378.7 |
  22472.9 |
  22567.1 |
  22661.3 |
  22755.4 |
  22849.6 |
  22943.8 |
  23038.0 |
  23132.2 |
  23226.3 |####################
  23320.5 |
  23414.7 |
  23508.9 |####################
  23603.0 |
  23697.2 |
  23791.4 |
  23885.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=866.1% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=402.6% of algo (FFI overhead may distort results)
