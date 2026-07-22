# Residual encoding: register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (583.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_res_scatter_register vs stability leader carrier_res_scatter_stack (+7% speed for 1.9x steadier)

carrier_res_scatter_register is fastest (583.50 us, CV 3.4%); carrier_res_scatter_stack gives up 6.9% median for 1.9x lower variance (CV 1.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 583497.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 583497.9 ns, slowest 623917.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 592516ns | 586993ns | 573184ns | 585731ns | 612360ns | base |
| carrier_res_scatter_stack | 625615ns | 627168ns | 607720ns | 623908ns | 637124ns | +5.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 589078ns | 569671ns | 608989ns | base | 0.007 |
| carrier_res_scatter_stack | 622114ns | 603799ns | 633802ns | +5.61% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.007 | 97.6% |
| carrier_res_scatter_stack | 0.007 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 592516ns | 592516ns | base |
| carrier_res_scatter_stack | 625615ns | 625615ns | +5.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 583498ns | base | --- | [574746, 608989] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 623918ns | +42076.9ns (+7.2%) | [+6849, +50183]ns | [608622, 633802] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 569671ns | +7.7% |
| 2 | 631290ns | -4.4% |
| 3 | 586688ns | +7.3% |
| 4 | 585620ns | +7.0% |
| 5 | 581376ns | +9.7% |
| 6 | 579820ns | +7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | -0.351 | moderate- |
| carrier_res_scatter_stack | 0.153 | ok |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 589854.3ns | 589077.6ns | 100.1% | HIGH |
| carrier_res_scatter_stack | 733423.9ns | 622114.0ns | 117.9% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 569670.8-608989.4 ns)
  569670.8 |####################
  571636.7 |
  573602.7 |
  575568.6 |
  577534.5 |
  579500.4 |########################################
  581466.4 |
  583432.3 |
  585398.2 |########################################
  587364.1 |
  589330.1 |
  591296.0 |
  593261.9 |
  595227.9 |
  597193.8 |
  599159.7 |
  601125.6 |
  603091.6 |
  605057.5 |
  607023.4 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 603799.2-633801.9 ns)
  603799.2 |########################################
  605299.3 |
  606799.5 |
  608299.6 |
  609799.7 |
  611299.9 |
  612800.0 |########################################
  614300.1 |
  615800.3 |
  617300.4 |
  618800.5 |
  620300.7 |########################################
  621800.8 |
  623301.0 |
  624801.1 |
  626301.2 |########################################
  627801.4 |
  629301.5 |########################################
  630801.6 |
  632301.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=117.4% of algo (FFI overhead may distort results)
