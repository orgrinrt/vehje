# Residual encoding: register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 91% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (784.82 us) leads carrier_res_madd_stack (1.50 ms) by 91%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (784.82 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 784824.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.91x (fastest 784824.6 ns, slowest 1496456.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 790048ns | 788335ns | 787023ns | 788128ns | 794440ns | base |
| carrier_res_madd_stack | 1500097ns | 1498737ns | 1498633ns | 1498711ns | 1502909ns | +89.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 786651ns | 784168ns | 790904ns | base | 0.021 |
| carrier_res_madd_stack | 1497802ns | 1496365ns | 1500558ns | +90.40% | 0.011 |

## Performance model

- Peak throughput: **0.021 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.021 | 99.9% |
| carrier_res_madd_stack | 0.011 | 52.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 790048ns | 790048ns | base |
| carrier_res_madd_stack | 1500097ns | 1500097ns | +89.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 784825ns | base | --- | [784223, 790904] | --- | --- | --- | --- |
| carrier_res_madd_stack | 1496456ns | +712018.2ns (+90.7%) | [+705519, +715916]ns | [1496391, 1500558] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 784279ns | +90.8% |
| 2 | 785115ns | +91.4% |
| 3 | 784168ns | +91.1% |
| 4 | 795097ns | +88.2% |
| 5 | 786712ns | +90.2% |
| 6 | 784534ns | +90.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.146 | ok |
| carrier_res_madd_stack | -0.051 | ok |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 788797.8ns | 786650.7ns | 100.3% | HIGH |
| carrier_res_madd_stack | 1505565.8ns | 1497801.8ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 784167.5-790904.4 ns)
  784167.5 |########################################
  784504.3 |####################
  784841.2 |####################
  785178.0 |
  785514.9 |
  785851.7 |
  786188.6 |
  786525.4 |####################
  786862.3 |
  787199.1 |
  787535.9 |
  787872.8 |
  788209.6 |
  788546.5 |
  788883.3 |
  789220.2 |
  789557.0 |
  789893.9 |
  790230.7 |
  790567.6 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 1496364.6-1500557.7 ns)
  1496364.6 |########################################
  1496574.3 |
  1496783.9 |
  1496993.6 |
  1497203.2 |
  1497412.9 |
  1497622.5 |
  1497832.2 |
  1498041.8 |
  1498251.5 |##########
  1498461.1 |
  1498670.8 |
  1498880.5 |
  1499090.1 |
  1499299.8 |
  1499509.4 |
  1499719.1 |
  1499928.7 |
  1500138.4 |
  1500348.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=100.6% of algo (FFI overhead may distort results)
