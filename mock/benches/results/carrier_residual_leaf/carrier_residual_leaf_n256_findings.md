# Residual encoding: predecoded register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 157% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (5.16 us) leads carrier_res_leaf_stack (13.28 us) by 157%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_leaf_register is fastest but the noisiest (CV 8.3%)

carrier_res_leaf_register wins on median (5.16 us) yet has the highest variance (CV 8.3%), while carrier_res_leaf_stack is the steadiest (CV 4.5%, 13.28 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (5.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 5156.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.57x (fastest 5156.4 ns, slowest 13277.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 7725ns | 7538ns | 7102ns | 7465ns | 8426ns | base |
| carrier_res_leaf_stack | 15595ns | 15680ns | 14430ns | 15534ns | 16270ns | +101.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 5324ns | 4917ns | 5813ns | base | 0.048 |
| carrier_res_leaf_stack | 13188ns | 12104ns | 13788ns | +147.73% | 0.019 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_leaf_register | 284165 | 1424288 | 0.200 | 1.00× |
| carrier_res_leaf_stack | 281112 | 1180927 | 0.238 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.052 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.050 | 95.4% |
| carrier_res_leaf_stack | 0.019 | 37.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 7725ns | 7725ns | base |
| carrier_res_leaf_stack | 15595ns | 15595ns | +101.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 5156ns | base | --- | [5001, 5813] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 13278ns | +8117.2ns (+157.4%) | [+6806, +8670]ns | [12499, 13788] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 4917ns | +168.1% |
| 2 | 6222ns | +107.2% |
| 3 | 5150ns | +172.0% |
| 4 | 5086ns | +166.7% |
| 5 | 5404ns | +147.5% |
| 6 | 5163ns | +134.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.469 | moderate- |
| carrier_res_leaf_stack | -0.028 | ok |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 88552.5ns | 5323.6ns | 1663.4% | HIGH |
| carrier_res_leaf_stack | 80876.9ns | 13187.9ns | 613.3% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 4916.7-5812.8 ns)
   4916.7 |####################
   4961.5 |
   5006.3 |
   5051.1 |####################
   5095.9 |
   5140.7 |########################################
   5185.5 |
   5230.3 |
   5275.1 |
   5319.9 |
   5364.7 |####################
   5409.5 |
   5454.3 |
   5499.1 |
   5543.9 |
   5588.7 |
   5633.5 |
   5678.3 |
   5723.1 |
   5767.9 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 12103.8-13787.7 ns)
  12103.8 |########################################
  12188.0 |
  12272.2 |
  12356.4 |
  12440.6 |
  12524.8 |
  12609.0 |
  12693.2 |
  12777.4 |
  12861.6 |########################################
  12945.8 |
  13029.9 |
  13114.1 |########################################
  13198.3 |
  13282.5 |
  13366.7 |########################################
  13450.9 |
  13535.1 |########################################
  13619.3 |
  13703.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=1702.3% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=613.5% of algo (FFI overhead may distort results)
