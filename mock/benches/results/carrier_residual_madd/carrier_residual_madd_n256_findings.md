# Residual encoding: predecoded register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 141% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (9.64 us) leads carrier_res_madd_stack (23.19 us) by 141%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (9.64 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 9641.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.41x (fastest 9641.5 ns, slowest 23192.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 11955ns | 12041ns | 11320ns | 11959ns | 12266ns | base |
| carrier_res_madd_stack | 25604ns | 25705ns | 24381ns | 25546ns | 26303ns | +114.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 9575ns | 9048ns | 9826ns | base | 0.027 |
| carrier_res_madd_stack | 23107ns | 21860ns | 23732ns | +141.33% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_madd_register | 303172 | 729522 | 0.416 | 1.00× |
| carrier_res_madd_stack | 355192 | 1469592 | 0.242 | 1.17× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.028 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.027 | 93.8% |
| carrier_res_madd_stack | 0.011 | 39.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 11955ns | 11955ns | base |
| carrier_res_madd_stack | 25604ns | 25604ns | +114.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 9641ns | base | --- | [9257, 9826] | --- | --- | --- | --- |
| carrier_res_madd_stack | 23193ns | +13688.6ns (+142.0%) | [+12999, +13910]ns | [22397, 23732] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 9048ns | +141.6% |
| 2 | 9465ns | +143.1% |
| 3 | 9535ns | +145.2% |
| 4 | 9758ns | +143.3% |
| 5 | 9748ns | +135.3% |
| 6 | 9895ns | +139.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | 0.310 | moderate+ |
| carrier_res_madd_stack | 0.020 | ok |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 89329.5ns | 9574.8ns | 933.0% | HIGH |
| carrier_res_madd_stack | 93750.5ns | 23107.3ns | 405.7% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 9047.9-9826.2 ns)
   9047.9 |########################################
   9086.8 |
   9125.7 |
   9164.7 |
   9203.6 |
   9242.5 |
   9281.4 |
   9320.3 |
   9359.2 |
   9398.2 |
   9437.1 |########################################
   9476.0 |
   9514.9 |########################################
   9553.8 |
   9592.7 |
   9631.7 |
   9670.6 |
   9709.5 |########################################
   9748.4 |########################################
   9787.3 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 21859.6-23732.3 ns)
  21859.6 |########################################
  21953.2 |
  22046.9 |
  22140.5 |
  22234.1 |
  22327.8 |
  22421.4 |
  22515.0 |
  22608.7 |
  22702.3 |
  22795.9 |
  22889.6 |########################################
  22983.2 |########################################
  23076.9 |
  23170.5 |
  23264.1 |
  23357.8 |########################################
  23451.4 |
  23545.0 |
  23638.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=930.4% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=405.5% of algo (FFI overhead may distort results)
