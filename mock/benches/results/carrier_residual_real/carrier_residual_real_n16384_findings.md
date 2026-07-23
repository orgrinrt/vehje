# Residual encoding: predecoded register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_real_register dominates: 41% faster than the next best (carrier_res_real_stack)

carrier_res_real_register (1.99 ms) leads carrier_res_real_stack (2.80 ms) by 41%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (1.99 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 1990250.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.41x (fastest 1990250.0 ns, slowest 2801739.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 1989182ns | 1994113ns | 1959880ns | 1993903ns | 1996753ns | base |
| carrier_res_real_stack | 2801087ns | 2804568ns | 2780907ns | 2802913ns | 2808437ns | +40.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 1985550ns | 1956345ns | 1993236ns | base | 0.008 |
| carrier_res_real_stack | 2798353ns | 2778297ns | 2805708ns | +40.94% | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_real_register | 12714692 | 9020863 | 1.409 | 1.00× |
| carrier_res_real_stack | 17537555 | 34466694 | 0.509 | 1.38× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.008 | 98.3% |
| carrier_res_real_stack | 0.006 | 69.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 1989182ns | 1989182ns | base |
| carrier_res_real_stack | 2801087ns | 2801087ns | +40.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 1990250ns | base | --- | [1973162, 1993236] | --- | --- | --- | --- |
| carrier_res_real_stack | 2801739ns | +811171.7ns (+40.8%) | [+794768, +832470]ns | [2787612, 2805708] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 1995319ns | +40.2% |
| 2 | 1989980ns | +41.0% |
| 3 | 1991154ns | +40.5% |
| 4 | 1956345ns | +43.4% |
| 5 | 1990131ns | +41.0% |
| 6 | 1990369ns | +39.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | -0.199 | ok |
| carrier_res_real_stack | -0.196 | ok |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 2073538.8ns | 1985549.7ns | 104.4% | HIGH |
| carrier_res_real_stack | 2808073.9ns | 2798352.9ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 1956344.6-1993236.5 ns)
  1956344.6 |##########
  1958189.2 |
  1960033.8 |
  1961878.4 |
  1963723.0 |
  1965567.6 |
  1967412.2 |
  1969256.8 |
  1971101.4 |
  1972946.0 |
  1974790.6 |
  1976635.1 |
  1978479.7 |
  1980324.3 |
  1982168.9 |
  1984013.5 |
  1985858.1 |
  1987702.7 |
  1989547.3 |########################################
  1991391.9 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 2778296.7-2805708.1 ns)
  2778296.7 |####################
  2779667.3 |
  2781037.8 |
  2782408.4 |
  2783779.0 |
  2785149.5 |
  2786520.1 |
  2787890.7 |
  2789261.3 |
  2790631.8 |
  2792002.4 |
  2793373.0 |
  2794743.5 |
  2796114.1 |####################
  2797484.7 |####################
  2798855.2 |
  2800225.8 |
  2801596.4 |
  2802967.0 |
  2804337.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=104.2% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=100.2% of algo (FFI overhead may distort results)
