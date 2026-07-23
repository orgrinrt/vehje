# Residual encoding: predecoded register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 217% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (31.44 us) leads carrier_res_tight_stack (99.56 us) by 217%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (31.44 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.2x the fastest

Fastest carrier_res_tight_register (31.44 us) to slowest carrier_res_tight_stack (99.56 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 31439.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.17x (fastest 31439.4 ns, slowest 99564.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 34258ns | 33818ns | 33331ns | 33786ns | 35428ns | base |
| carrier_res_tight_stack | 103218ns | 102354ns | 99811ns | 101836ns | 106993ns | +201.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 31808ns | 30929ns | 32896ns | base | 0.032 |
| carrier_res_tight_stack | 100622ns | 97355ns | 104500ns | +216.34% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_tight_register | 398830 | 1126377 | 0.354 | 1.00× |
| carrier_res_tight_stack | 606565 | 2377404 | 0.255 | 1.52× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.033 | 98.4% |
| carrier_res_tight_stack | 0.010 | 31.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 34258ns | 34258ns | base |
| carrier_res_tight_stack | 103218ns | 103218ns | +201.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 31439ns | base | --- | [31089, 32896] | --- | --- | --- | --- |
| carrier_res_tight_stack | 99564ns | +67260.0ns (+213.9%) | [+66121, +73061]ns | [97802, 104500] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 30929ns | +214.8% |
| 2 | 31609ns | +223.1% |
| 3 | 31270ns | +241.7% |
| 4 | 31250ns | +214.4% |
| 5 | 33087ns | +204.1% |
| 6 | 32706ns | +201.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | 0.264 | moderate+ |
| carrier_res_tight_stack | -0.165 | ok |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 95417.2ns | 31808.4ns | 300.0% | HIGH |
| carrier_res_tight_stack | 96620.4ns | 100622.4ns | 96.0% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 30929.2-32896.4 ns)
  30929.2 |####################
  31027.6 |
  31125.9 |
  31224.3 |########################################
  31322.7 |
  31421.0 |
  31519.4 |####################
  31617.7 |
  31716.1 |
  31814.5 |
  31912.8 |
  32011.2 |
  32109.5 |
  32207.9 |
  32306.3 |
  32404.6 |
  32503.0 |
  32601.4 |
  32699.7 |####################
  32798.1 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 97355.0-104500.2 ns)
  97355.0 |########################################
  97712.3 |
  98069.5 |########################################
  98426.8 |########################################
  98784.0 |
  99141.3 |
  99498.6 |
  99855.8 |
  100213.1 |
  100570.3 |########################################
  100927.6 |
  101284.9 |
  101642.1 |
  101999.4 |########################################
  102356.6 |
  102713.9 |
  103071.2 |
  103428.4 |
  103785.7 |
  104142.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=300.2% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=95.7% of algo (FFI overhead may distort results)
