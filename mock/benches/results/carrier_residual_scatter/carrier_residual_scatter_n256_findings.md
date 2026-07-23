# Residual encoding: predecoded register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 239% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (7.16 us) leads carrier_res_scatter_stack (24.29 us) by 239%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (7.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest carrier_res_scatter_register (7.16 us) to slowest carrier_res_scatter_stack (24.29 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 7158.4 ns median
- 1 variant significantly slower than baseline
- Spread: 3.39x (fastest 7158.4 ns, slowest 24293.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 9480ns | 9610ns | 8878ns | 9487ns | 9770ns | base |
| carrier_res_scatter_stack | 26565ns | 26778ns | 25305ns | 26757ns | 26906ns | +180.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 7101ns | 6756ns | 7334ns | base | 0.036 |
| carrier_res_scatter_stack | 24113ns | 23098ns | 24429ns | +239.57% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_scatter_register | 295462 | 1030966 | 0.287 | 1.00× |
| carrier_res_scatter_stack | 309700 | 1137280 | 0.272 | 1.05× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.036 | 94.4% |
| carrier_res_scatter_stack | 0.011 | 27.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 9480ns | 9480ns | base |
| carrier_res_scatter_stack | 26565ns | 26565ns | +180.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 7158ns | base | --- | [6811, 7334] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 24293ns | +17165.2ns (+239.8%) | [+16388, +17482]ns | [23616, 24429] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 6756ns | +260.8% |
| 2 | 6865ns | +252.6% |
| 3 | 7325ns | +233.9% |
| 4 | 7342ns | +214.6% |
| 5 | 7114ns | +239.2% |
| 6 | 7202ns | +238.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | 0.297 | moderate+ |
| carrier_res_scatter_stack | -0.234 | moderate- |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 88017.4ns | 7100.9ns | 1239.5% | HIGH |
| carrier_res_scatter_stack | 79301.2ns | 24112.7ns | 328.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 6755.8-7333.8 ns)
   6755.8 |########################################
   6784.7 |
   6813.6 |
   6842.5 |########################################
   6871.4 |
   6900.3 |
   6929.2 |
   6958.1 |
   6987.0 |
   7015.9 |
   7044.8 |
   7073.7 |
   7102.6 |########################################
   7131.5 |
   7160.4 |
   7189.3 |########################################
   7218.2 |
   7247.1 |
   7276.0 |
   7304.9 |########################################
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 23097.9-24429.2 ns)
  23097.9 |####################
  23164.5 |
  23231.0 |
  23297.6 |
  23364.2 |
  23430.7 |
  23497.3 |
  23563.8 |
  23630.4 |
  23697.0 |
  23763.5 |
  23830.1 |
  23896.7 |
  23963.2 |
  24029.8 |
  24096.3 |####################
  24162.9 |####################
  24229.5 |
  24296.0 |
  24362.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=1234.7% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=328.4% of algo (FFI overhead may distort results)
