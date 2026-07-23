# Residual encoding: predecoded register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 280% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (26.31 us) leads carrier_res_wideselect_stack (100.03 us) by 280%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (26.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.8x the fastest

Fastest carrier_res_wideselect_register (26.31 us) to slowest carrier_res_wideselect_stack (100.03 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 26306.7 ns median
- 1 variant significantly slower than baseline
- Spread: 3.80x (fastest 26306.7 ns, slowest 100025.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 28703ns | 28605ns | 27831ns | 28500ns | 29442ns | base |
| carrier_res_wideselect_stack | 103085ns | 102464ns | 98876ns | 102017ns | 106790ns | +259.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 26305ns | 25445ns | 26901ns | base | 0.039 |
| carrier_res_wideselect_stack | 100608ns | 96293ns | 104351ns | +282.47% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_wideselect_register | 404824 | 1391601 | 0.291 | 1.00× |
| carrier_res_wideselect_stack | 645374 | 2309737 | 0.279 | 1.59× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.039 | 96.7% |
| carrier_res_wideselect_stack | 0.010 | 25.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 28703ns | 28703ns | base |
| carrier_res_wideselect_stack | 103085ns | 103085ns | +259.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 26307ns | base | --- | [25706, 26901] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 100025ns | +74029.1ns (+281.4%) | [+71075, +77805]ns | [97448, 104351] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 25445ns | +296.4% |
| 2 | 26792ns | +270.2% |
| 3 | 26082ns | +305.1% |
| 4 | 25968ns | +279.7% |
| 5 | 26532ns | +262.9% |
| 6 | 27010ns | +281.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.218 | moderate- |
| carrier_res_wideselect_stack | -0.349 | moderate- |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 104123.9ns | 26304.7ns | 395.8% | HIGH |
| carrier_res_wideselect_stack | 105000.5ns | 100607.9ns | 104.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 25445.4-26901.0 ns)
  25445.4 |########################################
  25518.2 |
  25591.0 |
  25663.7 |
  25736.5 |
  25809.3 |
  25882.1 |
  25954.9 |########################################
  26027.7 |########################################
  26100.4 |
  26173.2 |
  26246.0 |
  26318.8 |
  26391.6 |
  26464.4 |########################################
  26537.1 |
  26609.9 |
  26682.7 |
  26755.5 |########################################
  26828.3 |
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 96292.9-104350.8 ns)
  96292.9 |########################################
  96695.8 |
  97098.7 |
  97501.6 |
  97904.5 |
  98307.4 |########################################
  98710.3 |
  99113.2 |########################################
  99516.1 |
  99919.0 |
  100321.8 |
  100724.7 |########################################
  101127.6 |
  101530.5 |
  101933.4 |
  102336.3 |
  102739.2 |########################################
  103142.1 |
  103545.0 |
  103947.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=396.6% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=104.4% of algo (FFI overhead may distort results)
