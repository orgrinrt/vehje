# Residual encoding: predecoded register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 129% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (624.16 us) leads carrier_res_madd_stack (1.43 ms) by 129%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (624.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 624158.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.29x (fastest 624158.3 ns, slowest 1429637.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 627571ns | 627557ns | 623858ns | 626969ns | 630330ns | base |
| carrier_res_madd_stack | 1435204ns | 1432052ns | 1428914ns | 1431319ns | 1444176ns | +128.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 624263ns | 620441ns | 627248ns | base | 0.026 |
| carrier_res_madd_stack | 1432758ns | 1426662ns | 1441555ns | +129.51% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_madd_register | 3921326 | 8790902 | 0.446 | 1.00× |
| carrier_res_madd_stack | 9233060 | 37875335 | 0.244 | 2.35× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.026 | 99.4% |
| carrier_res_madd_stack | 0.011 | 43.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 627571ns | 627571ns | base |
| carrier_res_madd_stack | 1435204ns | 1435204ns | +128.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 624158ns | base | --- | [621384, 627248] | --- | --- | --- | --- |
| carrier_res_madd_stack | 1429638ns | +805522.8ns (+129.1%) | [+801815, +818147]ns | [1427082, 1441555] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 622327ns | +132.5% |
| 2 | 620441ns | +130.1% |
| 3 | 623828ns | +128.7% |
| 4 | 626048ns | +128.4% |
| 5 | 624488ns | +129.9% |
| 6 | 628448ns | +127.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | 0.245 | moderate+ |
| carrier_res_madd_stack | -0.155 | ok |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 626311.5ns | 624263.4ns | 100.3% | HIGH |
| carrier_res_madd_stack | 1516075.0ns | 1432758.3ns | 105.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 620441.2-627247.7 ns)
  620441.2 |########################################
  620781.5 |
  621121.8 |
  621462.2 |
  621802.5 |
  622142.8 |########################################
  622483.1 |
  622823.5 |
  623163.8 |
  623504.1 |########################################
  623844.4 |
  624184.8 |########################################
  624525.1 |
  624865.4 |
  625205.8 |
  625546.1 |
  625886.4 |########################################
  626226.7 |
  626567.0 |
  626907.4 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 1426662.1-1441554.8 ns)
  1426662.1 |########################################
  1427406.7 |########################################
  1428151.4 |
  1428896.0 |########################################
  1429640.6 |########################################
  1430385.3 |
  1431129.9 |
  1431874.5 |
  1432619.2 |
  1433363.8 |
  1434108.4 |
  1434853.1 |
  1435597.7 |########################################
  1436342.4 |
  1437087.0 |
  1437831.6 |
  1438576.3 |
  1439320.9 |
  1440065.5 |
  1440810.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=105.8% of algo (FFI overhead may distort results)
