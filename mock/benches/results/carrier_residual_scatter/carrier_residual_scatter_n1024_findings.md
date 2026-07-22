# Residual encoding: register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 151% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (45.19 us) leads carrier_res_scatter_stack (113.59 us) by 151%, a clear separation rather than a photo finish. CV 12.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (45.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 45186.7 ns median
- 1 variant significantly slower than baseline
- Spread: 2.51x (fastest 45186.7 ns, slowest 113586.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 50746ns | 47761ns | 45652ns | 47215ns | 58589ns | base |
| carrier_res_scatter_stack | 121065ns | 116210ns | 106025ns | 113292ns | 140245ns | +138.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 48217ns | 43077ns | 56174ns | base | 0.021 |
| carrier_res_scatter_stack | 118511ns | 103772ns | 137511ns | +145.79% | 0.009 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.023 | 95.3% |
| carrier_res_scatter_stack | 0.009 | 37.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 50746ns | 50746ns | base |
| carrier_res_scatter_stack | 121065ns | 121065ns | +138.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 45187ns | base | --- | [43290, 56174] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 113586ns | +69202.1ns (+153.1%) | [+60344, +81337]ns | [104436, 137511] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 45266ns | +165.2% |
| 2 | 43077ns | +140.9% |
| 3 | 43502ns | +146.3% |
| 4 | 45108ns | +133.0% |
| 5 | 56973ns | +139.5% |
| 6 | 55375ns | +150.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | 0.459 | moderate+ |
| carrier_res_scatter_stack | 0.333 | moderate+ |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 107282.9ns | 48216.7ns | 222.5% | HIGH |
| carrier_res_scatter_stack | 149971.0ns | 118510.9ns | 126.5% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 43077.1-56173.9 ns)
  43077.1 |########################################
  43731.9 |
  44386.8 |
  45041.6 |########################################
  45696.5 |
  46351.3 |
  47006.2 |
  47661.0 |
  48315.8 |
  48970.7 |
  49625.5 |
  50280.4 |
  50935.2 |
  51590.1 |
  52244.9 |
  52899.7 |
  53554.6 |
  54209.4 |
  54864.3 |####################
  55519.1 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 103772.1-137510.6 ns)
  103772.1 |########################################
  105459.0 |####################
  107146.0 |
  108832.9 |
  110519.8 |
  112206.7 |
  113893.7 |
  115580.6 |
  117267.5 |
  118954.4 |####################
  120641.4 |
  122328.3 |
  124015.2 |
  125702.2 |
  127389.1 |
  129076.0 |
  130762.9 |
  132449.9 |
  134136.8 |
  135823.7 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=234.5% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=133.6% of algo (FFI overhead may distort results)
