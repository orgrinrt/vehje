# Residual encoding: register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 91% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (831.19 us) leads carrier_res_tight_stack (1.59 ms) by 91%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (831.19 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 831187.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 831187.3 ns, slowest 1590564.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 835337ns | 834969ns | 831883ns | 834619ns | 838141ns | base |
| carrier_res_tight_stack | 1592583ns | 1592957ns | 1589038ns | 1591683ns | 1595706ns | +90.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 832006ns | 829648ns | 834775ns | base | 0.020 |
| carrier_res_tight_stack | 1590193ns | 1586612ns | 1593372ns | +91.13% | 0.010 |

## Performance model

- Peak throughput: **0.020 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.020 | 99.8% |
| carrier_res_tight_stack | 0.010 | 52.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 835337ns | 835337ns | base |
| carrier_res_tight_stack | 1592583ns | 1592583ns | +90.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 831187ns | base | --- | [830056, 834775] | --- | --- | --- | --- |
| carrier_res_tight_stack | 1590564ns | +758556.7ns (+91.3%) | [+755791, +760211]ns | [1586641, 1593372] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 831988ns | +91.4% |
| 2 | 831139ns | +91.5% |
| 3 | 837562ns | +90.4% |
| 4 | 830464ns | +91.1% |
| 5 | 829648ns | +91.6% |
| 6 | 831235ns | +90.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.197 | ok |
| carrier_res_tight_stack | -0.116 | ok |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 834818.3ns | 832006.3ns | 100.3% | HIGH |
| carrier_res_tight_stack | 1544328.6ns | 1590192.6ns | 97.1% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 829648.3-834775.4 ns)
  829648.3 |########################################
  829904.7 |
  830161.0 |
  830417.4 |########################################
  830673.7 |
  830930.1 |########################################
  831186.4 |########################################
  831442.8 |
  831699.1 |
  831955.5 |########################################
  832211.9 |
  832468.2 |
  832724.6 |
  832980.9 |
  833237.3 |
  833493.6 |
  833750.0 |
  834006.3 |
  834262.7 |
  834519.0 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 1586612.5-1593372.5 ns)
  1586612.5 |########################################
  1586950.5 |
  1587288.5 |
  1587626.5 |
  1587964.5 |
  1588302.5 |
  1588640.5 |
  1588978.5 |
  1589316.5 |
  1589654.5 |####################
  1589992.5 |
  1590330.5 |
  1590668.5 |
  1591006.5 |####################
  1591344.5 |
  1591682.5 |
  1592020.5 |####################
  1592358.5 |
  1592696.5 |
  1593034.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=97.0% of algo (FFI overhead may distort results)
