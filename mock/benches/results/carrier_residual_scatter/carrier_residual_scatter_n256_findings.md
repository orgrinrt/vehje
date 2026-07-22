# Residual encoding: register/SSA vs stack bytecode, scatter profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_scatter_register**

## Highlights

Baseline for all deltas below: **carrier_res_scatter_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_scatter_register dominates: 151% faster than the next best (carrier_res_scatter_stack)

carrier_res_scatter_register (10.31 us) leads carrier_res_scatter_stack (25.87 us) by 151%, a clear separation rather than a photo finish. CV 9.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_scatter_register is fastest but the noisiest (CV 9.3%)

carrier_res_scatter_register wins on median (10.31 us) yet has the highest variance (CV 9.3%), while carrier_res_scatter_stack is the steadiest (CV 4.8%, 25.87 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_res_scatter_stack shows alternating (throttle bounce) (autocorr -0.53)

carrier_res_scatter_stack's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_scatter_register)

The baseline carrier_res_scatter_register is the fastest (10.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_scatter_register) is the fastest** at 10314.4 ns median
- 1 variant significantly slower than baseline
- Spread: 2.51x (fastest 10314.4 ns, slowest 25870.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 13341ns | 12757ns | 12359ns | 12634ns | 14893ns | base |
| carrier_res_scatter_stack | 28443ns | 28282ns | 26809ns | 27927ns | 30033ns | +113.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_scatter_register | 10741ns | 10006ns | 11901ns | base | 0.024 |
| carrier_res_scatter_stack | 26048ns | 24592ns | 27497ns | +142.52% | 0.010 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_scatter_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_scatter_register | 0.025 | 97.0% |
| carrier_res_scatter_stack | 0.010 | 38.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_scatter_register | 13341ns | 13341ns | base |
| carrier_res_scatter_stack | 28443ns | 28443ns | +113.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_scatter_register | 10314ns | base | --- | [10007, 11901] | --- | --- | --- | --- |
| carrier_res_scatter_stack | 25870ns | +14959.4ns (+145.0%) | [+14125, +16837]ns | [24776, 27497] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_scatter_register | carrier_res_scatter_stack |
|---|---|---|
| 1 | 10006ns | +149.5% |
| 2 | 10589ns | +160.0% |
| 3 | 10040ns | +166.6% |
| 4 | 10008ns | +149.5% |
| 5 | 12675ns | +116.7% |
| 6 | 11127ns | +121.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_scatter_register | 0.011 | ok |
| carrier_res_scatter_stack | -0.533 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_scatter_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_scatter_register | 93048.9ns | 10740.6ns | 866.3% | HIGH |
| carrier_res_scatter_stack | 83192.3ns | 26047.6ns | 319.4% | HIGH |

## Distribution (algo ns)

```
carrier_res_scatter_register (n=6, range 10005.8-11900.7 ns)
  10005.8 |########################################
  10100.5 |
  10195.3 |
  10290.0 |
  10384.8 |
  10479.5 |
  10574.3 |#############
  10669.0 |
  10763.7 |
  10858.5 |
  10953.2 |
  11048.0 |#############
  11142.7 |
  11237.5 |
  11332.2 |
  11426.9 |
  11521.7 |
  11616.4 |
  11711.2 |
  11805.9 |
  (0 below, 1 above range)

carrier_res_scatter_stack (n=6, range 24591.7-27496.9 ns)
  24591.7 |####################
  24737.0 |
  24882.2 |########################################
  25027.5 |
  25172.7 |
  25318.0 |
  25463.3 |
  25608.5 |
  25753.8 |
  25899.0 |
  26044.3 |
  26189.6 |
  26334.8 |
  26480.1 |
  26625.3 |####################
  26770.6 |
  26915.9 |
  27061.1 |
  27206.4 |
  27351.6 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_scatter_register**: bridge=890.7% of algo (FFI overhead may distort results)
- **carrier_res_scatter_stack**: bridge=320.0% of algo (FFI overhead may distort results)
