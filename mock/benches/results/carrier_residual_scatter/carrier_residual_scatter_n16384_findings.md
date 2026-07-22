# Residual encoding: register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (2.61 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 2609555.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 2609555.2 ns, slowest 2832927.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 2607578ns | 2612916ns | 2593351ns | 2608014ns | 2614037ns | base |
| carrier_res_scatter_stack | 2833379ns | 2835214ns | 2801945ns | 2827441ns | 2858002ns | +8.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 2604034ns | 2589589ns | 2610481ns | base | 0.006 |
| carrier_res_scatter_stack | 2830978ns | 2799622ns | 2855385ns | +8.72% | 0.006 |

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.006 | 99.2% |
| carrier_res_scatter_stack | 0.006 | 91.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 2607578ns | 2607578ns | base |
| carrier_res_scatter_stack | 2833379ns | 2833379ns | +8.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 2609555ns | base | --- | [2592066, 2610481] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 2832928ns | +230238.8ns (+8.8%) | [+205193, +245400]ns | [2804622, 2855385] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 2609269ns | +7.7% |
| 2 | 2594543ns | +8.6% |
| 3 | 2589589ns | +8.1% |
| 4 | 2610835ns | +9.1% |
| 5 | 2610128ns | +9.3% |
| 6 | 2609841ns | +9.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | 0.149 | ok |
| carrier_res_scatter_stack | 0.357 | moderate+ |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 2607294.6ns | 2604034.0ns | 100.1% | HIGH |
| carrier_res_scatter_stack | 3331421.5ns | 2830978.2ns | 117.7% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 2589588.7-2610481.0 ns)
  2589588.7 |####################
  2590633.3 |
  2591677.9 |
  2592722.6 |
  2593767.2 |####################
  2594811.8 |
  2595856.4 |
  2596901.0 |
  2597945.6 |
  2598990.3 |
  2600034.9 |
  2601079.5 |
  2602124.1 |
  2603168.7 |
  2604213.3 |
  2605258.0 |
  2606302.6 |
  2607347.2 |
  2608391.8 |####################
  2609436.4 |########################################
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 2799622.1-2855384.8 ns)
  2799622.1 |########################################
  2802410.2 |
  2805198.4 |
  2807986.5 |########################################
  2810774.6 |
  2813562.8 |
  2816350.9 |########################################
  2819139.0 |
  2821927.2 |
  2824715.3 |
  2827503.5 |
  2830291.6 |
  2833079.7 |
  2835867.9 |
  2838656.0 |
  2841444.1 |
  2844232.3 |
  2847020.4 |########################################
  2849808.5 |
  2852596.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=117.6% of algo (FFI overhead may distort results)
