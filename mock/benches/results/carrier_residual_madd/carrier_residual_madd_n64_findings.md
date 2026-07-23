# Residual encoding: predecoded register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 274% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (1.55 us) leads carrier_res_madd_stack (5.79 us) by 274%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (1.55 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.7x the fastest

Fastest carrier_res_madd_register (1.55 us) to slowest carrier_res_madd_stack (5.79 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 1549.0 ns median
- 1 variant significantly slower than baseline
- Spread: 3.74x (fastest 1549.0 ns, slowest 5794.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 3878ns | 3850ns | 3768ns | 3831ns | 4002ns | base |
| carrier_res_madd_stack | 8149ns | 8244ns | 7642ns | 8092ns | 8488ns | +110.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 1554ns | 1483ns | 1613ns | base | 0.041 |
| carrier_res_madd_stack | 5719ns | 5388ns | 5931ns | +267.95% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_madd_register | 261515 | 992509 | 0.263 | 1.00× |
| carrier_res_madd_stack | 269016 | 1086975 | 0.247 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.041 | 95.8% |
| carrier_res_madd_stack | 0.011 | 25.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 3878ns | 3878ns | base |
| carrier_res_madd_stack | 8149ns | 8149ns | +110.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 1549ns | base | --- | [1500, 1613] | --- | --- | --- | --- |
| carrier_res_madd_stack | 5795ns | +4181.4ns (+270.0%) | [+3909, +4403]ns | [5431, 5931] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 1525ns | +258.8% |
| 2 | 1632ns | +254.0% |
| 3 | 1483ns | +306.4% |
| 4 | 1518ns | +255.1% |
| 5 | 1595ns | +264.5% |
| 6 | 1572ns | +271.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.388 | moderate- |
| carrier_res_madd_stack | -0.408 | moderate- |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 84616.6ns | 1554.2ns | 5444.3% | HIGH |
| carrier_res_madd_stack | 83846.1ns | 5718.8ns | 1466.2% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 1483.3-1613.3 ns)
   1483.3 |########################################
   1489.8 |
   1496.3 |
   1502.8 |
   1509.3 |
   1515.8 |########################################
   1522.3 |########################################
   1528.8 |
   1535.3 |
   1541.8 |
   1548.3 |
   1554.8 |
   1561.3 |
   1567.8 |########################################
   1574.3 |
   1580.8 |
   1587.3 |
   1593.8 |########################################
   1600.3 |
   1606.8 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 5387.9-5930.9 ns)
   5387.9 |########################################
   5415.0 |
   5442.2 |
   5469.3 |########################################
   5496.5 |
   5523.6 |
   5550.8 |
   5577.9 |
   5605.1 |
   5632.2 |
   5659.4 |
   5686.5 |
   5713.7 |
   5740.8 |
   5768.0 |########################################
   5795.1 |########################################
   5822.3 |########################################
   5849.4 |
   5876.6 |
   5903.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=5463.0% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=1449.4% of algo (FFI overhead may distort results)
