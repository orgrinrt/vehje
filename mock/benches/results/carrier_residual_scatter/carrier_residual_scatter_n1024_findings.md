# Residual encoding: predecoded register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 260% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (31.70 us) leads carrier_res_scatter_stack (114.13 us) by 260%, a clear separation rather than a photo finish. CV 26.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_scatter_register is fastest but the noisiest (CV 26.1%)

carrier_res_scatter_register wins on median (31.70 us) yet has the highest variance (CV 26.1%), while carrier_res_scatter_stack is the steadiest (CV 9.2%, 114.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (31.70 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.6x the fastest

Fastest carrier_res_scatter_register (31.70 us) to slowest carrier_res_scatter_stack (114.13 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 31704.6 ns median
- 1 variant significantly slower than baseline
- Spread: 3.60x (fastest 31704.6 ns, slowest 114125.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 36933ns | 34124ns | 31334ns | 33441ns | 44971ns | base |
| carrier_res_scatter_stack | 119034ns | 116854ns | 106173ns | 114632ns | 132068ns | +222.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 34531ns | 29012ns | 42586ns | base | 0.030 |
| carrier_res_scatter_stack | 116376ns | 103648ns | 129364ns | +237.02% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_scatter_register | 419627 | 1187368 | 0.353 | 1.00× |
| carrier_res_scatter_stack | 802233 | 2258692 | 0.355 | 1.91× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.032 | 91.5% |
| carrier_res_scatter_stack | 0.009 | 25.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 36933ns | 36933ns | base |
| carrier_res_scatter_stack | 119034ns | 119034ns | +222.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 31705ns | base | --- | [29302, 42586] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 114125ns | +81822.7ns (+258.1%) | [+66052, +97660]ns | [105638, 129364] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 29592ns | +250.3% |
| 2 | 52786ns | +110.0% |
| 3 | 31045ns | +301.0% |
| 4 | 32387ns | +232.3% |
| 5 | 32365ns | +314.8% |
| 6 | 29012ns | +304.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | -0.317 | moderate- |
| carrier_res_scatter_stack | -0.279 | moderate- |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 101385.7ns | 34531.0ns | 293.6% | HIGH |
| carrier_res_scatter_stack | 141867.5ns | 116375.8ns | 121.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 29012.1-42586.4 ns)
  29012.1 |########################################
  29690.8 |
  30369.5 |####################
  31048.3 |
  31727.0 |########################################
  32405.7 |
  33084.4 |
  33763.1 |
  34441.8 |
  35120.6 |
  35799.3 |
  36478.0 |
  37156.7 |
  37835.4 |
  38514.1 |
  39192.9 |
  39871.6 |
  40550.3 |
  41229.0 |
  41907.7 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 103647.9-129364.4 ns)
  103647.9 |########################################
  104933.7 |
  106219.5 |
  107505.4 |########################################
  108791.2 |
  110077.0 |########################################
  111362.9 |
  112648.7 |
  113934.5 |
  115220.3 |
  116506.1 |########################################
  117792.0 |
  119077.8 |
  120363.6 |
  121649.5 |
  122935.3 |
  124221.1 |########################################
  125506.9 |
  126792.8 |
  128078.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: CV=23.9% (high variance, measurements may be unstable)
- **carrier_res_scatter_register**: bridge=299.5% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=124.3% of algo (FFI overhead may distort results)
