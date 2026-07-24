# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (16.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 3.37 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 235% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (3.37 us) leads abi_native_cross_madd_inproc_native (11.30 us) by 235%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 79% (significant)

abi_native_cross_madd_null_entry is -12.96 us (79%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_madd_native_ffi_w (16.41 us) is 4.9x the fastest (3.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_madd_inproc_native shows alternating (throttle bounce) (autocorr -0.53)

abi_native_cross_madd_inproc_native's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_madd_null_entry (3.37 us) to slowest abi_native_cross_madd_native_ffi_w (16.41 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 3372.1 ns median (-79.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.87x (fastest 3372.1 ns, slowest 16407.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13835ns | 13628ns | 13390ns | 13552ns | 14482ns | -28.36% |
| abi_native_cross_madd_native_ffi_w | 19313ns | 18674ns | 18123ns | 18514ns | 21107ns | base |
| abi_native_cross_madd_null_entry | 5680ns | 5613ns | 5383ns | 5606ns | 5941ns | -70.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11486ns | 11138ns | 12005ns | -32.67% | 0.000 |
| abi_native_cross_madd_native_ffi_w | 17059ns | 15888ns | 18843ns | base | 0.000 |
| abi_native_cross_madd_null_entry | 3415ns | 3265ns | 3562ns | -79.98% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5265.2 | 11537.9 | 11485.8 | n/a |
| abi_native_cross_madd_native_ffi_w | 25405.5 | 16950.5 | 17058.9 | n/a |
| abi_native_cross_madd_null_entry | 26422.8 | 3493.3 | 3415.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.000 | 28.9% |
| abi_native_cross_madd_native_ffi_w | 0.000 | 19.9% |
| abi_native_cross_madd_null_entry | 0.001 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13835ns | 13835ns | -28.36% |
| abi_native_cross_madd_native_ffi_w | 19313ns | 19313ns | base |
| abi_native_cross_madd_null_entry | 5680ns | 5680ns | -70.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 16407ns | base | --- | [15926, 18843] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11303ns | -5001.5ns (-30.5%) | [-7547, -4171]ns | [11150, 12005] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_madd_null_entry | 3372ns | -12956.6ns (-79.0%) | [-15414, -12560]ns | [3312, 3562] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 15965ns | -30.0% | -79.0% |
| 2 | 16463ns | -25.2% | -77.9% |
| 3 | 18491ns | -39.6% | -81.8% |
| 4 | 19195ns | -40.5% | -81.8% |
| 5 | 16351ns | -31.9% | -80.0% |
| 6 | 15888ns | -26.4% | -78.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.529 | HIGH- (thermal bounce) |
| abi_native_cross_madd_native_ffi_w | 0.217 | moderate+ |
| abi_native_cross_madd_null_entry | -0.362 | moderate- |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 120519.1ns | 11485.8ns | 1049.3% | HIGH |
| abi_native_cross_madd_native_ffi_w | 156938.3ns | 17058.9ns | 920.0% | HIGH |
| abi_native_cross_madd_null_entry | 119256.9ns | 3415.4ns | 3491.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11137.9-12004.8 ns)
  11137.9 |########################################
  11181.2 |
  11224.6 |
  11267.9 |
  11311.3 |
  11354.6 |
  11398.0 |#############
  11441.3 |
  11484.7 |
  11528.0 |
  11571.3 |
  11614.7 |
  11658.0 |#############
  11701.4 |
  11744.7 |
  11788.1 |
  11831.4 |
  11874.8 |
  11918.1 |
  11961.5 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 15887.5-18842.9 ns)
  15887.5 |########################################
  16035.3 |
  16183.0 |
  16330.8 |########################################
  16478.6 |
  16626.3 |
  16774.1 |
  16921.9 |
  17069.7 |
  17217.4 |
  17365.2 |
  17513.0 |
  17660.7 |
  17808.5 |
  17956.3 |
  18104.1 |
  18251.8 |
  18399.6 |####################
  18547.4 |
  18695.1 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 3264.6-3562.1 ns)
   3264.6 |####################
   3279.5 |
   3294.3 |
   3309.2 |
   3324.1 |
   3339.0 |
   3353.8 |####################
   3368.7 |########################################
   3383.6 |
   3398.5 |
   3413.3 |
   3428.2 |
   3443.1 |
   3458.0 |
   3472.8 |####################
   3487.7 |
   3502.6 |
   3517.5 |
   3532.3 |
   3547.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1059.2% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=955.5% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=3526.0% of algo (FFI overhead may distort results)
