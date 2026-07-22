# Residual encoding: register/SSA vs stack bytecode, madd profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_madd_register**

## Highlights

Baseline for all deltas below: **carrier_res_madd_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_madd_register dominates: 93% faster than the next best (carrier_res_madd_stack)

carrier_res_madd_register (12.70 us) leads carrier_res_madd_stack (24.48 us) by 93%, a clear separation rather than a photo finish. CV 15.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_madd_register is fastest but the noisiest (CV 15.5%)

carrier_res_madd_register wins on median (12.70 us) yet has the highest variance (CV 15.5%), while carrier_res_madd_stack is the steadiest (CV 5.7%, 24.48 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_res_madd_register shows alternating (throttle bounce) (autocorr -0.53)

carrier_res_madd_register's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_madd_register)

The baseline carrier_res_madd_register is the fastest (12.70 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_madd_register) is the fastest** at 12703.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.93x (fastest 12703.1 ns, slowest 24482.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_madd_register | 15939ns | 15786ns | 13387ns | 15004ns | 18618ns | base |
| carrier_res_madd_stack | 27560ns | 26884ns | 25819ns | 26752ns | 29644ns | +72.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_madd_register | 13243ns | 11165ns | 15848ns | base | 0.019 |
| carrier_res_madd_stack | 25078ns | 23400ns | 26934ns | +89.37% | 0.010 |

## Performance model

- Peak throughput: **0.023 Gops/s** (carrier_res_madd_register; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_madd_register | 0.020 | 87.9% |
| carrier_res_madd_stack | 0.010 | 45.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_madd_register | 15939ns | 15939ns | base |
| carrier_res_madd_stack | 27560ns | 27560ns | +72.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_madd_register | 12703ns | base | --- | [11179, 15848] | --- | --- | --- | --- |
| carrier_res_madd_stack | 24483ns | +11387.5ns (+89.6%) | [+11005, +13113]ns | [23819, 26934] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_madd_register | carrier_res_madd_stack |
|---|---|---|
| 1 | 12410ns | +88.6% |
| 2 | 16244ns | +68.6% |
| 3 | 11165ns | +118.1% |
| 4 | 12996ns | +89.4% |
| 5 | 11192ns | +116.6% |
| 6 | 15452ns | +71.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_madd_register | -0.529 | HIGH- (thermal bounce) |
| carrier_res_madd_stack | -0.521 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_res_madd_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_madd_register | 94659.8ns | 13243.3ns | 714.8% | HIGH |
| carrier_res_madd_stack | 98327.9ns | 25078.5ns | 392.1% | HIGH |

## Distribution (algo ns)

```
carrier_res_madd_register (n=6, range 11164.6-15848.1 ns)
  11164.6 |########################################
  11398.8 |
  11633.0 |
  11867.1 |
  12101.3 |
  12335.5 |####################
  12569.7 |
  12803.8 |####################
  13038.0 |
  13272.2 |
  13506.4 |
  13740.6 |
  13974.7 |
  14208.9 |
  14443.1 |
  14677.3 |
  14911.4 |
  15145.6 |
  15379.8 |####################
  15614.0 |
  (0 below, 1 above range)

carrier_res_madd_stack (n=6, range 23400.4-26933.5 ns)
  23400.4 |########################################
  23577.1 |
  23753.7 |
  23930.4 |
  24107.0 |########################################
  24283.7 |########################################
  24460.3 |########################################
  24637.0 |
  24813.7 |
  24990.3 |
  25167.0 |
  25343.6 |
  25520.3 |
  25696.9 |
  25873.6 |
  26050.3 |
  26226.9 |
  26403.6 |########################################
  26580.2 |
  26756.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_madd_register**: bridge=759.6% of algo (FFI overhead may distort results)
- **carrier_res_madd_stack**: bridge=398.1% of algo (FFI overhead may distort results)
