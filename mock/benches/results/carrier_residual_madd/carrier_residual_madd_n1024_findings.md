# Residual encoding: predecoded register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 129% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (41.43 us) leads carrier_res_madd_stack (94.80 us) by 129%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_madd_stack shows alternating (throttle bounce) (autocorr -0.63)

carrier_res_madd_stack's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (41.43 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 41430.0 ns median
- 1 variant significantly slower than baseline
- Spread: 2.29x (fastest 41430.0 ns, slowest 94798.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 43687ns | 43855ns | 40991ns | 43424ns | 45430ns | base |
| carrier_res_madd_stack | 97944ns | 97312ns | 95422ns | 96809ns | 100907ns | +124.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 41209ns | 38586ns | 42901ns | base | 0.025 |
| carrier_res_madd_stack | 95405ns | 93105ns | 98262ns | +131.51% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_madd_register | 460013 | 1034162 | 0.445 | 1.00× |
| carrier_res_madd_stack | 599569 | 2381450 | 0.252 | 1.30× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.025 | 93.1% |
| carrier_res_madd_stack | 0.011 | 40.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 43687ns | 43687ns | base |
| carrier_res_madd_stack | 97944ns | 97944ns | +124.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 41430ns | base | --- | [39296, 42901] | --- | --- | --- | --- |
| carrier_res_madd_stack | 94799ns | +53422.8ns (+128.9%) | [+51574, +57590]ns | [93153, 98262] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 38586ns | +142.6% |
| 2 | 40007ns | +148.5% |
| 3 | 41635ns | +123.6% |
| 4 | 44166ns | +117.4% |
| 5 | 41338ns | +134.9% |
| 6 | 41522ns | +124.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | 0.249 | moderate+ |
| carrier_res_madd_stack | -0.628 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 110412.2ns | 41209.0ns | 267.9% | HIGH |
| carrier_res_madd_stack | 96138.8ns | 95404.8ns | 100.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 38585.8-42900.6 ns)
  38585.8 |########################################
  38801.5 |
  39017.3 |
  39233.0 |
  39448.8 |
  39664.5 |
  39880.2 |########################################
  40096.0 |
  40311.7 |
  40527.5 |
  40743.2 |
  40958.9 |
  41174.7 |########################################
  41390.4 |########################################
  41606.2 |########################################
  41821.9 |
  42037.6 |
  42253.4 |
  42469.1 |
  42684.9 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 93105.4-98262.5 ns)
  93105.4 |########################################
  93363.3 |####################
  93621.1 |
  93879.0 |
  94136.8 |
  94394.7 |
  94652.5 |
  94910.4 |
  95168.2 |
  95426.1 |
  95683.9 |
  95941.8 |####################
  96199.7 |
  96457.5 |
  96715.4 |
  96973.2 |####################
  97231.1 |
  97488.9 |
  97746.8 |
  98004.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=273.5% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=100.8% of algo (FFI overhead may distort results)
