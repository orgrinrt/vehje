# Residual encoding: predecoded register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 37% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (456.41 us) leads carrier_res_scatter_stack (626.81 us) by 37%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (456.41 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 456406.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.37x (fastest 456406.2 ns, slowest 626812.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 456792ns | 459116ns | 435065ns | 451421ns | 475712ns | base |
| carrier_res_scatter_stack | 626474ns | 630000ns | 611050ns | 624196ns | 637603ns | +37.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 453825ns | 431909ns | 472706ns | base | 0.009 |
| carrier_res_scatter_stack | 623206ns | 607477ns | 634395ns | +37.32% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_scatter_register | 2933774 | 2453305 | 1.196 | 1.00× |
| carrier_res_scatter_stack | 4255509 | 9050708 | 0.470 | 1.45× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.009 | 94.6% |
| carrier_res_scatter_stack | 0.007 | 68.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 456792ns | 456792ns | base |
| carrier_res_scatter_stack | 626474ns | 626474ns | +37.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 456406ns | base | --- | [432362, 472706] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 626812ns | +171301.2ns (+37.5%) | [+144956, +191887]ns | [608412, 634395] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 461277ns | +36.2% |
| 2 | 431909ns | +40.6% |
| 3 | 466793ns | +34.1% |
| 4 | 432814ns | +48.0% |
| 5 | 451536ns | +39.0% |
| 6 | 478619ns | +27.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | -0.413 | moderate- |
| carrier_res_scatter_stack | -0.077 | ok |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 481344.7ns | 453824.7ns | 106.1% | HIGH |
| carrier_res_scatter_stack | 737983.6ns | 623206.2ns | 118.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 431909.2-472706.1 ns)
  431909.2 |########################################
  433949.0 |
  435988.9 |
  438028.7 |
  440068.6 |
  442108.4 |
  444148.3 |
  446188.1 |
  448227.9 |
  450267.8 |####################
  452307.6 |
  454347.5 |
  456387.3 |
  458427.2 |
  460467.0 |####################
  462506.8 |
  464546.7 |
  466586.5 |####################
  468626.4 |
  470666.2 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 607477.1-634394.8 ns)
  607477.1 |########################################
  608823.0 |########################################
  610168.9 |
  611514.7 |
  612860.6 |
  614206.5 |
  615552.4 |
  616898.3 |
  618244.2 |
  619590.0 |
  620935.9 |
  622281.8 |
  623627.7 |
  624973.6 |########################################
  626319.5 |########################################
  627665.3 |########################################
  629011.2 |
  630357.1 |
  631703.0 |
  633048.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=106.9% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=117.1% of algo (FFI overhead may distort results)
