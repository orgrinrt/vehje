# Residual encoding: predecoded register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 230% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (7.01 us) leads carrier_res_real_stack (23.15 us) by 230%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (7.01 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.3x the fastest

Fastest carrier_res_real_register (7.01 us) to slowest carrier_res_real_stack (23.15 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 7013.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.30x (fastest 7013.4 ns, slowest 23154.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 9399ns | 9364ns | 8948ns | 9360ns | 9684ns | base |
| carrier_res_real_stack | 25492ns | 25593ns | 24299ns | 25496ns | 26084ns | +171.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 7030ns | 6663ns | 7265ns | base | 0.036 |
| carrier_res_real_stack | 23035ns | 21911ns | 23614ns | +227.65% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_real_register | 297871 | 1003927 | 0.297 | 1.00× |
| carrier_res_real_stack | 354617 | 1383701 | 0.256 | 1.19× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.037 | 95.0% |
| carrier_res_real_stack | 0.011 | 28.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 9399ns | 9399ns | base |
| carrier_res_real_stack | 25492ns | 25492ns | +171.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 7013ns | base | --- | [6812, 7265] | --- | --- | --- | --- |
| carrier_res_real_stack | 23154ns | +16212.7ns (+231.2%) | [+15201, +16599]ns | [22336, 23614] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 6663ns | +248.4% |
| 2 | 7307ns | +199.9% |
| 3 | 7008ns | +237.5% |
| 4 | 7223ns | +226.4% |
| 5 | 7019ns | +229.0% |
| 6 | 6962ns | +226.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.448 | moderate- |
| carrier_res_real_stack | -0.268 | moderate- |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 89883.7ns | 7030.3ns | 1278.5% | HIGH |
| carrier_res_real_stack | 92718.3ns | 23034.7ns | 402.5% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 6662.9-7265.2 ns)
   6662.9 |####################
   6693.0 |
   6723.1 |
   6753.2 |
   6783.4 |
   6813.5 |
   6843.6 |
   6873.7 |
   6903.8 |
   6933.9 |####################
   6964.1 |
   6994.2 |########################################
   7024.3 |
   7054.4 |
   7084.5 |
   7114.6 |
   7144.7 |
   7174.9 |
   7205.0 |####################
   7235.1 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 21911.2-23614.4 ns)
  21911.2 |########################################
  21996.4 |
  22081.5 |
  22166.7 |
  22251.8 |
  22337.0 |
  22422.2 |
  22507.3 |
  22592.5 |
  22677.6 |########################################
  22762.8 |
  22848.0 |
  22933.1 |
  23018.3 |########################################
  23103.4 |
  23188.6 |########################################
  23273.8 |
  23358.9 |
  23444.1 |
  23529.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=1290.2% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=401.9% of algo (FFI overhead may distort results)
