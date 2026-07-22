# Residual encoding: register/SSA vs stack bytecode, tight profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_tight_register**

## Highlights

Baseline for all deltas below: **carrier_res_tight_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_tight_register dominates: 162% faster than the next best (carrier_res_tight_stack)

carrier_res_tight_register (10.50 us) leads carrier_res_tight_stack (27.55 us) by 162%, a clear separation rather than a photo finish. CV 17.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_tight_register is fastest but the noisiest (CV 17.5%)

carrier_res_tight_register wins on median (10.50 us) yet has the highest variance (CV 17.5%), while carrier_res_tight_stack is the steadiest (CV 11.1%, 27.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_res_tight_stack shows alternating (throttle bounce) (autocorr -0.56)

carrier_res_tight_stack's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_tight_register)

The baseline carrier_res_tight_register is the fastest (10.50 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_tight_register) is the fastest** at 10503.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.62x (fastest 10503.5 ns, slowest 27550.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_tight_register | 13895ns | 13022ns | 12242ns | 12836ns | 16310ns | base |
| carrier_res_tight_stack | 30171ns | 30378ns | 26109ns | 28961ns | 34018ns | +117.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_tight_register | 11221ns | 10016ns | 13061ns | base | 0.023 |
| carrier_res_tight_stack | 27413ns | 23824ns | 30847ns | +144.31% | 0.009 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_res_tight_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_tight_register | 0.024 | 95.4% |
| carrier_res_tight_stack | 0.009 | 36.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_tight_register | 13895ns | 13895ns | base |
| carrier_res_tight_stack | 30171ns | 30171ns | +117.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_tight_register | 10504ns | base | --- | [10097, 13061] | --- | --- | --- | --- |
| carrier_res_tight_stack | 27551ns | +15928.5ns (+151.6%) | [+12232, +20416]ns | [23841, 30847] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_tight_register | carrier_res_tight_stack |
|---|---|---|
| 1 | 10627ns | +124.5% |
| 2 | 10845ns | +199.4% |
| 3 | 10177ns | +134.1% |
| 4 | 15278ns | +73.5% |
| 5 | 10380ns | +175.4% |
| 6 | 10016ns | +191.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_tight_register | -0.298 | moderate- |
| carrier_res_tight_stack | -0.563 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_tight_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_tight_register | 95724.9ns | 11220.6ns | 853.1% | HIGH |
| carrier_res_tight_stack | 97811.5ns | 27413.0ns | 356.8% | HIGH |

## Distribution (algo ns)

```
carrier_res_tight_register (n=6, range 10016.2-13061.5 ns)
  10016.2 |########################################
  10168.5 |########################################
  10320.7 |########################################
  10473.0 |
  10625.2 |########################################
  10777.5 |########################################
  10929.8 |
  11082.0 |
  11234.3 |
  11386.6 |
  11538.8 |
  11691.1 |
  11843.4 |
  11995.6 |
  12147.9 |
  12300.1 |
  12452.4 |
  12604.7 |
  12756.9 |
  12909.2 |
  (0 below, 1 above range)

carrier_res_tight_stack (n=6, range 23824.2-30847.1 ns)
  23824.2 |########################################
  24175.3 |
  24526.5 |
  24877.6 |
  25228.8 |
  25579.9 |
  25931.1 |
  26282.2 |####################
  26633.4 |
  26984.5 |
  27335.7 |
  27686.8 |
  28037.9 |
  28389.1 |####################
  28740.2 |
  29091.4 |####################
  29442.5 |
  29793.7 |
  30144.8 |
  30496.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_tight_register**: bridge=895.9% of algo (FFI overhead may distort results)
- **carrier_res_tight_stack**: bridge=357.8% of algo (FFI overhead may distort results)
