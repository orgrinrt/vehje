# Residual encoding: register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (557.40 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader carrier_res_real_register vs stability leader carrier_res_real_stack (+10% speed for 2.6x steadier)

carrier_res_real_register is fastest (557.40 us, CV 4.9%); carrier_res_real_stack gives up 9.6% median for 2.6x lower variance (CV 1.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 557401.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.10x (fastest 557401.4 ns, slowest 610792.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 575069ns | 560317ns | 546197ns | 559788ns | 612426ns | base |
| carrier_res_real_stack | 613805ns | 614051ns | 596773ns | 611503ns | 625774ns | +6.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 571933ns | 542956ns | 609014ns | base | 0.007 |
| carrier_res_real_stack | 610528ns | 593404ns | 622636ns | +6.75% | 0.007 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.007 | 97.4% |
| carrier_res_real_stack | 0.007 | 88.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 575069ns | 575069ns | base |
| carrier_res_real_stack | 613805ns | 613805ns | +6.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 557401ns | base | --- | [549384, 609014] | --- | --- | --- | --- |
| carrier_res_real_stack | 610792ns | +47809.5ns (+8.6%) | [+1778, +66196]ns | [598154, 622636] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 557070ns | +12.9% |
| 2 | 555811ns | +10.8% |
| 3 | 617178ns | -1.6% |
| 4 | 542956ns | +11.0% |
| 5 | 557733ns | +6.4% |
| 6 | 600850ns | +2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.409 | moderate- |
| carrier_res_real_stack | 0.228 | moderate+ |

**Consistency summary:**

- **carrier_res_real_stack**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 573042.0ns | 571932.9ns | 100.2% | HIGH |
| carrier_res_real_stack | 613646.8ns | 610527.5ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 542956.2-609013.6 ns)
  542956.2 |####################
  546259.1 |
  549561.9 |
  552864.8 |####################
  556167.7 |########################################
  559470.5 |
  562773.4 |
  566076.3 |
  569379.1 |
  572682.0 |
  575984.9 |
  579287.7 |
  582590.6 |
  585893.5 |
  589196.3 |
  592499.2 |
  595802.1 |
  599104.9 |####################
  602407.8 |
  605710.7 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 593403.8-622636.2 ns)
  593403.8 |########################################
  594865.4 |
  596327.0 |
  597788.7 |
  599250.3 |
  600711.9 |
  602173.5 |########################################
  603635.2 |
  605096.8 |
  606558.4 |########################################
  608020.0 |
  609481.6 |
  610943.3 |
  612404.9 |
  613866.5 |########################################
  615328.1 |########################################
  616789.8 |
  618251.4 |
  619713.0 |
  621174.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=100.6% of algo (FFI overhead may distort results)
