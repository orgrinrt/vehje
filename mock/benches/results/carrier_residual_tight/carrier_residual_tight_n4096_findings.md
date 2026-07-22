# Residual encoding: register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 113% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (187.24 us) leads carrier_res_tight_stack (398.66 us) by 113%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (187.24 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 187244.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.13x (fastest 187244.8 ns, slowest 398658.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 189271ns | 189572ns | 185766ns | 189401ns | 190830ns | base |
| carrier_res_tight_stack | 401804ns | 401344ns | 398274ns | 400849ns | 405002ns | +112.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 186930ns | 183500ns | 188406ns | base | 0.022 |
| carrier_res_tight_stack | 399150ns | 396118ns | 401891ns | +113.53% | 0.010 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.022 | 98.0% |
| carrier_res_tight_stack | 0.010 | 46.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 189271ns | 189271ns | base |
| carrier_res_tight_stack | 401804ns | 401804ns | +112.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 187245ns | base | --- | [185139, 188406] | --- | --- | --- | --- |
| carrier_res_tight_stack | 398658ns | +212873.3ns (+113.7%) | [+209162, +214624]ns | [396901, 401891] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 183500ns | +116.7% |
| 2 | 187270ns | +112.4% |
| 3 | 186779ns | +113.9% |
| 4 | 188294ns | +110.4% |
| 5 | 188518ns | +114.1% |
| 6 | 187220ns | +113.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | 0.073 | ok |
| carrier_res_tight_stack | -0.249 | moderate- |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 188039.9ns | 186930.0ns | 100.6% | HIGH |
| carrier_res_tight_stack | 386144.1ns | 399150.1ns | 96.7% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 183499.6-188405.8 ns)
  183499.6 |####################
  183744.9 |
  183990.2 |
  184235.5 |
  184480.9 |
  184726.2 |
  184971.5 |
  185216.8 |
  185462.1 |
  185707.4 |
  185952.7 |
  186198.0 |
  186443.3 |
  186688.7 |####################
  186934.0 |
  187179.3 |########################################
  187424.6 |
  187669.9 |
  187915.2 |
  188160.5 |####################
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 396118.3-401890.7 ns)
  396118.3 |####################
  396406.9 |
  396695.5 |
  396984.2 |
  397272.8 |
  397561.4 |########################################
  397850.0 |
  398138.6 |
  398427.2 |
  398715.9 |
  399004.5 |
  399293.1 |####################
  399581.7 |
  399870.3 |
  400158.9 |####################
  400447.6 |
  400736.2 |
  401024.8 |
  401313.4 |
  401602.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=96.7% of algo (FFI overhead may distort results)
