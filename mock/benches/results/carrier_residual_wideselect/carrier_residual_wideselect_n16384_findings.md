# Residual encoding: register/SSA vs stack bytecode, wideselect profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_wideselect_register**

## Highlights

Baseline for all deltas below: **carrier_res_wideselect_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_res_wideselect_register dominates: 17% faster than the next best (carrier_res_wideselect_stack)

carrier_res_wideselect_register (2.15 ms) leads carrier_res_wideselect_stack (2.53 ms) by 17%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_res_wideselect_register shows alternating (throttle bounce) (autocorr -0.54)

carrier_res_wideselect_register's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_res_wideselect_register)

The baseline carrier_res_wideselect_register is the fastest (2.15 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_wideselect_register) is the fastest** at 2153009.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.17x (fastest 2153009.0 ns, slowest 2526563.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 2153277ns | 2156443ns | 2128627ns | 2149290ns | 2171582ns | base |
| carrier_res_wideselect_stack | 2591988ns | 2528928ns | 2516333ns | 2525710ns | 2729232ns | +20.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_wideselect_register | 2150030ns | 2126115ns | 2168547ns | base | 0.008 |
| carrier_res_wideselect_stack | 2589578ns | 2514044ns | 2726634ns | +20.44% | 0.006 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_res_wideselect_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_wideselect_register | 0.008 | 98.8% |
| carrier_res_wideselect_stack | 0.006 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_wideselect_register | 2153277ns | 2153277ns | base |
| carrier_res_wideselect_stack | 2591988ns | 2591988ns | +20.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_wideselect_register | 2153009ns | base | --- | [2128534, 2168547] | --- | --- | --- | --- |
| carrier_res_wideselect_stack | 2526563ns | +398029.3ns (+18.5%) | [+361787, +558828]ns | [2515536, 2726634] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_wideselect_register | carrier_res_wideselect_stack |
|---|---|---|
| 1 | 2138744ns | +17.7% |
| 2 | 2168755ns | +15.9% |
| 3 | 2126115ns | +18.9% |
| 4 | 2167274ns | +22.3% |
| 5 | 2168338ns | +29.3% |
| 6 | 2130953ns | +18.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_wideselect_register | -0.540 | HIGH- (thermal bounce) |
| carrier_res_wideselect_stack | 0.086 | ok |

**Consistency summary:**

- **carrier_res_wideselect_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_wideselect_register | 2154585.8ns | 2150029.9ns | 100.2% | HIGH |
| carrier_res_wideselect_stack | 2538698.6ns | 2589577.9ns | 98.0% | HIGH |

## Distribution (algo ns)

```
carrier_res_wideselect_register (n=6, range 2126114.6-2168546.6 ns)
  2126114.6 |####################
  2128236.2 |
  2130357.8 |####################
  2132479.4 |
  2134601.0 |
  2136722.6 |####################
  2138844.2 |
  2140965.8 |
  2143087.4 |
  2145209.0 |
  2147330.6 |
  2149452.2 |
  2151573.8 |
  2153695.4 |
  2155817.0 |
  2157938.6 |
  2160060.2 |
  2162181.8 |
  2164303.4 |
  2166425.0 |########################################
  (0 below, 1 above range)

carrier_res_wideselect_stack (n=6, range 2514043.8-2726634.2 ns)
  2514043.8 |########################################
  2524673.3 |########################################
  2535302.8 |
  2545932.4 |
  2556561.9 |
  2567191.4 |
  2577820.9 |
  2588450.4 |
  2599080.0 |
  2609709.5 |
  2620339.0 |
  2630968.5 |
  2641598.0 |####################
  2652227.6 |
  2662857.1 |
  2673486.6 |
  2684116.1 |
  2694745.6 |
  2705375.2 |
  2716004.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_wideselect_register**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_res_wideselect_stack**: bridge=100.4% of algo (FFI overhead may distort results)
