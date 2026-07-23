# Residual encoding: predecoded register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 101% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (130.97 us) leads carrier_res_leaf_stack (263.21 us) by 101%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (130.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 130971.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.01x (fastest 130971.2 ns, slowest 263213.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 133438ns | 133589ns | 130109ns | 132893ns | 135920ns | base |
| carrier_res_leaf_stack | 265317ns | 266186ns | 261684ns | 265349ns | 267086ns | +98.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 130957ns | 127934ns | 133443ns | base | 0.031 |
| carrier_res_leaf_stack | 262479ns | 258795ns | 264485ns | +100.43% | 0.016 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_leaf_register | 832754 | 2514093 | 0.331 | 1.00× |
| carrier_res_leaf_stack | 1755262 | 5588758 | 0.314 | 2.11× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.031 | 97.7% |
| carrier_res_leaf_stack | 0.016 | 48.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 133438ns | 133438ns | base |
| carrier_res_leaf_stack | 265317ns | 265317ns | +98.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 130971ns | base | --- | [128458, 133443] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 263214ns | +131736.3ns (+100.6%) | [+128845, +133984]ns | [259739, 264485] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 127934ns | +102.3% |
| 2 | 129477ns | +103.2% |
| 3 | 128983ns | +104.1% |
| 4 | 133855ns | +94.8% |
| 5 | 133031ns | +99.0% |
| 6 | 132466ns | +99.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | 0.358 | moderate+ |
| carrier_res_leaf_stack | -0.134 | ok |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 135864.1ns | 130957.5ns | 103.7% | HIGH |
| carrier_res_leaf_stack | 300298.0ns | 262479.2ns | 114.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 127933.7-133442.9 ns)
  127933.7 |########################################
  128209.2 |
  128484.6 |
  128760.1 |########################################
  129035.5 |
  129311.0 |########################################
  129586.5 |
  129861.9 |
  130137.4 |
  130412.8 |
  130688.3 |
  130963.8 |
  131239.2 |
  131514.7 |
  131790.1 |
  132065.6 |
  132341.1 |########################################
  132616.5 |
  132892.0 |########################################
  133167.4 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 258795.0-264484.8 ns)
  258795.0 |####################
  259079.5 |
  259364.0 |
  259648.5 |
  259933.0 |
  260217.5 |
  260501.9 |####################
  260786.4 |
  261070.9 |
  261355.4 |
  261639.9 |
  261924.4 |
  262208.9 |
  262493.4 |
  262777.9 |
  263062.4 |########################################
  263346.8 |
  263631.3 |
  263915.8 |
  264200.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=103.2% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=114.4% of algo (FFI overhead may distort results)
