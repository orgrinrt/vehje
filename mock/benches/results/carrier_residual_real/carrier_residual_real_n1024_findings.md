# Residual encoding: predecoded register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 380% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (27.26 us) leads carrier_res_real_stack (130.77 us) by 380%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_real_stack shows alternating (throttle bounce) (autocorr -0.53)

carrier_res_real_stack's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (27.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_res_real_register (27.26 us) to slowest carrier_res_real_stack (130.77 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 27263.3 ns median
- 1 variant significantly slower than baseline
- Spread: 4.80x (fastest 27263.3 ns, slowest 130774.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 30206ns | 29772ns | 28963ns | 29541ns | 31825ns | base |
| carrier_res_real_stack | 127250ns | 133256ns | 108552ns | 128559ns | 134637ns | +321.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 27817ns | 26741ns | 29338ns | base | 0.037 |
| carrier_res_real_stack | 124768ns | 106100ns | 132071ns | +348.53% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_real_register | 425078 | 1374204 | 0.309 | 1.00× |
| carrier_res_real_stack | 722826 | 2173558 | 0.333 | 1.70× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.038 | 98.1% |
| carrier_res_real_stack | 0.008 | 20.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 30206ns | 30206ns | base |
| carrier_res_real_stack | 127250ns | 127250ns | +321.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 27263ns | base | --- | [26849, 29338] | --- | --- | --- | --- |
| carrier_res_real_stack | 130774ns | +103743.3ns (+380.5%) | [+82120, +104990]ns | [111458, 132071] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 27422ns | +383.1% |
| 2 | 28050ns | +316.5% |
| 3 | 26958ns | +385.7% |
| 4 | 27105ns | +381.9% |
| 5 | 30625ns | +246.4% |
| 6 | 26741ns | +392.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.448 | moderate- |
| carrier_res_real_stack | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 106565.2ns | 27816.8ns | 383.1% | HIGH |
| carrier_res_real_stack | 103219.3ns | 124767.9ns | 82.7% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 26740.8-29337.9 ns)
  26740.8 |########################################
  26870.7 |########################################
  27000.5 |########################################
  27130.4 |
  27260.2 |
  27390.1 |########################################
  27519.9 |
  27649.8 |
  27779.6 |
  27909.5 |
  28039.3 |########################################
  28169.2 |
  28299.1 |
  28428.9 |
  28558.8 |
  28688.6 |
  28818.5 |
  28948.3 |
  29078.2 |
  29208.0 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 106099.6-132071.2 ns)
  106099.6 |####################
  107398.2 |
  108696.8 |
  109995.3 |
  111293.9 |
  112592.5 |
  113891.1 |
  115189.7 |
  116488.3 |####################
  117786.8 |
  119085.4 |
  120384.0 |
  121682.6 |
  122981.2 |
  124279.8 |
  125578.3 |
  126876.9 |
  128175.5 |
  129474.1 |####################
  130772.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=394.3% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=78.0% of algo (FFI overhead may distort results)
