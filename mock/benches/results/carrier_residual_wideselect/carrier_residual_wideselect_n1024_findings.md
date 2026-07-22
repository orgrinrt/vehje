# Residual encoding: register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 148% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (42.31 us) leads carrier_res_wideselect_stack (105.03 us) by 148%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_wideselect_register is fastest but the noisiest (CV 6.2%)

carrier_res_wideselect_register wins on median (42.31 us) yet has the highest variance (CV 6.2%), while carrier_res_wideselect_stack is the steadiest (CV 1.5%, 105.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_res_wideselect_register shows alternating (throttle bounce) (autocorr -0.56)

carrier_res_wideselect_register's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (42.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 42314.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.48x (fastest 42314.8 ns, slowest 105035.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 45284ns | 44724ns | 42273ns | 44387ns | 48134ns | base |
| carrier_res_wideselect_stack | 107690ns | 107325ns | 104940ns | 107279ns | 109680ns | +137.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 42823ns | 39711ns | 45630ns | base | 0.024 |
| carrier_res_wideselect_stack | 105303ns | 102509ns | 107214ns | +145.90% | 0.010 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.024 | 93.8% |
| carrier_res_wideselect_stack | 0.010 | 37.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 45284ns | 45284ns | base |
| carrier_res_wideselect_stack | 107690ns | 107690ns | +137.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 42315ns | base | --- | [40525, 45630] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 105035ns | +63645.0ns (+150.4%) | [+58198, +65595]ns | [103659, 107214] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 39711ns | +163.9% |
| 2 | 48216ns | +112.6% |
| 3 | 41338ns | +159.9% |
| 4 | 42050ns | +154.5% |
| 5 | 42580ns | +146.4% |
| 6 | 43045ns | +144.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.564 | HIGH- (thermal bounce) |
| carrier_res_wideselect_stack | -0.099 | ok |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 115566.7ns | 42823.2ns | 269.9% | HIGH |
| carrier_res_wideselect_stack | 107834.1ns | 105302.7ns | 102.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 39710.8-45630.2 ns)
  39710.8 |########################################
  40006.8 |
  40302.7 |
  40598.7 |
  40894.7 |
  41190.7 |########################################
  41486.6 |
  41782.6 |########################################
  42078.6 |
  42374.5 |########################################
  42670.5 |
  42966.5 |########################################
  43262.4 |
  43558.4 |
  43854.4 |
  44150.3 |
  44446.3 |
  44742.3 |
  45038.3 |
  45334.2 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 102508.8-107213.9 ns)
  102508.8 |########################################
  102744.1 |
  102979.3 |
  103214.6 |
  103449.8 |
  103685.1 |
  103920.3 |
  104155.6 |
  104390.9 |
  104626.1 |########################################
  104861.4 |########################################
  105096.6 |########################################
  105331.9 |
  105567.1 |
  105802.4 |
  106037.7 |
  106272.9 |
  106508.2 |
  106743.4 |
  106978.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=281.6% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=102.5% of algo (FFI overhead may distort results)
