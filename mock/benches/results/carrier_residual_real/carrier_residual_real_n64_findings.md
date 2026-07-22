# Residual encoding: register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 147% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (2.41 us) leads carrier_res_real_stack (5.95 us) by 147%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_real_register is fastest but the noisiest (CV 8.5%)

carrier_res_real_register wins on median (2.41 us) yet has the highest variance (CV 8.5%), while carrier_res_real_stack is the steadiest (CV 4.6%, 5.95 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (2.41 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 2408.1 ns median
- 1 variant significantly slower than baseline
- Spread: 2.47x (fastest 2408.1 ns, slowest 5948.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 4964ns | 4936ns | 4592ns | 4829ns | 5351ns | base |
| carrier_res_real_stack | 8391ns | 8356ns | 7705ns | 8241ns | 8959ns | +69.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 2408ns | 2132ns | 2631ns | base | 0.027 |
| carrier_res_real_stack | 5892ns | 5436ns | 6165ns | +144.65% | 0.011 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.027 | 88.5% |
| carrier_res_real_stack | 0.011 | 35.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 4964ns | 4964ns | base |
| carrier_res_real_stack | 8391ns | 8391ns | +69.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 2408ns | base | --- | [2185, 2631] | --- | --- | --- | --- |
| carrier_res_real_stack | 5948ns | +3572.7ns (+148.4%) | [+3154, +3723]ns | [5561, 6165] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 2787ns | +107.8% |
| 2 | 2239ns | +154.0% |
| 3 | 2405ns | +153.9% |
| 4 | 2411ns | +155.4% |
| 5 | 2475ns | +149.4% |
| 6 | 2132ns | +155.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.324 | moderate- |
| carrier_res_real_stack | -0.044 | ok |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 86266.0ns | 2408.1ns | 3582.3% | HIGH |
| carrier_res_real_stack | 89733.1ns | 5891.5ns | 1523.1% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 2131.7-2631.1 ns)
   2131.7 |########################################
   2156.7 |
   2181.6 |
   2206.6 |
   2231.6 |########################################
   2256.5 |
   2281.5 |
   2306.5 |
   2331.4 |
   2356.4 |
   2381.4 |########################################
   2406.3 |########################################
   2431.3 |
   2456.3 |########################################
   2481.2 |
   2506.2 |
   2531.2 |
   2556.1 |
   2581.1 |
   2606.1 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 5436.2-6164.8 ns)
   5436.2 |########################################
   5472.6 |
   5509.1 |
   5545.5 |
   5581.9 |
   5618.3 |
   5654.8 |########################################
   5691.2 |
   5727.6 |
   5764.1 |########################################
   5800.5 |
   5836.9 |
   5873.4 |
   5909.8 |
   5946.2 |
   5982.6 |
   6019.1 |
   6055.5 |
   6091.9 |########################################
   6128.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=3598.0% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=1515.5% of algo (FFI overhead may distort results)
