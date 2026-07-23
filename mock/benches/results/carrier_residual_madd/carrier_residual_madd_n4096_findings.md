# Residual encoding: predecoded register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 140% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (151.35 us) leads carrier_res_madd_stack (363.14 us) by 140%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (151.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 151345.7 ns median
- 1 variant significantly slower than baseline
- Spread: 2.40x (fastest 151345.7 ns, slowest 363138.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 154341ns | 153731ns | 153221ns | 153632ns | 155963ns | base |
| carrier_res_madd_stack | 366205ns | 366205ns | 364348ns | 365903ns | 367586ns | +137.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 151807ns | 150568ns | 153315ns | base | 0.027 |
| carrier_res_madd_stack | 363095ns | 360886ns | 364511ns | +139.18% | 0.011 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_madd_register | 960688 | 2210970 | 0.435 | 1.00× |
| carrier_res_madd_stack | 2351556 | 9467283 | 0.248 | 2.45× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.027 | 99.5% |
| carrier_res_madd_stack | 0.011 | 41.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 154341ns | 154341ns | base |
| carrier_res_madd_stack | 366205ns | 366205ns | +137.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 151346ns | base | --- | [150761, 153315] | --- | --- | --- | --- |
| carrier_res_madd_stack | 363138ns | +211643.8ns (+139.8%) | [+209422, +212798]ns | [361637, 364511] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 152605ns | +138.6% |
| 2 | 151237ns | +141.3% |
| 3 | 151455ns | +138.3% |
| 4 | 154025ns | +136.0% |
| 5 | 150955ns | +140.4% |
| 6 | 150568ns | +140.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.226 | moderate- |
| carrier_res_madd_stack | -0.296 | moderate- |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 152721.2ns | 151807.4ns | 100.6% | HIGH |
| carrier_res_madd_stack | 384038.0ns | 363095.4ns | 105.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 150567.5-153315.2 ns)
  150567.5 |########################################
  150704.9 |
  150842.3 |########################################
  150979.7 |
  151117.0 |########################################
  151254.4 |
  151391.8 |########################################
  151529.2 |
  151666.6 |
  151804.0 |
  151941.4 |
  152078.7 |
  152216.1 |
  152353.5 |
  152490.9 |########################################
  152628.3 |
  152765.7 |
  152903.0 |
  153040.4 |
  153177.8 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 360886.2-364510.7 ns)
  360886.2 |########################################
  361067.4 |
  361248.6 |
  361429.9 |
  361611.1 |
  361792.3 |
  361973.5 |
  362154.8 |
  362336.0 |########################################
  362517.2 |
  362698.4 |########################################
  362879.6 |
  363060.9 |
  363242.1 |
  363423.3 |########################################
  363604.5 |
  363785.8 |
  363967.0 |########################################
  364148.2 |
  364329.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=105.9% of algo (FFI overhead may distort results)
