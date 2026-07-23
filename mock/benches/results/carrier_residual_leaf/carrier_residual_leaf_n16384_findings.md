# Residual encoding: predecoded register/SSA vs stack bytecode, leaf profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_leaf_register**

## Highlights

Baseline for all deltas below: **carrier_res_leaf_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_leaf_register dominates: 53% faster than the next best (carrier_res_leaf_stack)

carrier_res_leaf_register (759.17 us) leads carrier_res_leaf_stack (1.16 ms) by 53%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_leaf_register shows alternating (throttle bounce) (autocorr -0.59)

carrier_res_leaf_register's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_leaf_register)

The baseline carrier_res_leaf_register is the fastest (759.17 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_leaf_register) is the fastest** at 759167.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.53x (fastest 759167.5 ns, slowest 1163302.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 763565ns | 762350ns | 756300ns | 761588ns | 770164ns | base |
| carrier_res_leaf_stack | 1166269ns | 1165660ns | 1159198ns | 1165015ns | 1171686ns | +52.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_leaf_register | 760271ns | 752774ns | 766982ns | base | 0.022 |
| carrier_res_leaf_stack | 1163929ns | 1156871ns | 1169278ns | +53.09% | 0.014 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_res_leaf_register | 4840284 | 10035446 | 0.482 | 1.00× |
| carrier_res_leaf_stack | 7726428 | 22179546 | 0.348 | 1.60× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_res_leaf_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_leaf_register | 0.022 | 99.2% |
| carrier_res_leaf_stack | 0.014 | 64.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_leaf_register | 763565ns | 763565ns | base |
| carrier_res_leaf_stack | 1166269ns | 1166269ns | +52.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_leaf_register | 759168ns | base | --- | [754665, 766982] | --- | --- | --- | --- |
| carrier_res_leaf_stack | 1163303ns | +406151.2ns (+53.5%) | [+394551, +410271]ns | [1159207, 1169278] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_leaf_register | carrier_res_leaf_stack |
|---|---|---|
| 1 | 761458ns | +54.0% |
| 2 | 758714ns | +53.2% |
| 3 | 759621ns | +52.3% |
| 4 | 752774ns | +54.3% |
| 5 | 772506ns | +50.7% |
| 6 | 756557ns | +54.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_leaf_register | -0.594 | HIGH- (thermal bounce) |
| carrier_res_leaf_stack | 0.104 | ok |

**Consistency summary:**

- **carrier_res_leaf_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_leaf_register | 783041.9ns | 760271.5ns | 103.0% | HIGH |
| carrier_res_leaf_stack | 1299003.4ns | 1163929.1ns | 111.6% | HIGH |

## Distribution (algo ns)

```
carrier_res_leaf_register (n=6, range 752773.8-766981.7 ns)
  752773.8 |########################################
  753484.2 |
  754194.6 |
  754905.0 |
  755615.4 |
  756325.8 |########################################
  757036.2 |
  757746.5 |
  758456.9 |########################################
  759167.3 |########################################
  759877.7 |
  760588.1 |
  761298.5 |########################################
  762008.9 |
  762719.3 |
  763429.7 |
  764140.1 |
  764850.5 |
  765560.9 |
  766271.3 |
  (0 below, 1 above range)

carrier_res_leaf_stack (n=6, range 1156871.2-1169277.7 ns)
  1156871.2 |########################################
  1157491.5 |
  1158111.9 |
  1158732.2 |
  1159352.5 |
  1159972.8 |
  1160593.1 |
  1161213.5 |########################################
  1161833.8 |########################################
  1162454.1 |
  1163074.5 |
  1163694.8 |
  1164315.1 |########################################
  1164935.4 |
  1165555.8 |########################################
  1166176.1 |
  1166796.4 |
  1167416.7 |
  1168037.1 |
  1168657.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_leaf_register**: bridge=103.1% of algo (FFI overhead may distort results)
- **carrier_res_leaf_stack**: bridge=111.6% of algo (FFI overhead may distort results)
