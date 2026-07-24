# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (16.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 228% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.40 us) leads abi_native_cross_real_inproc_native (11.16 us) by 228%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -12.68 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.7x slower than the field

abi_native_cross_real_native_ffi_w (16.05 us) is 4.7x the fastest (3.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_native_ffi_w shows alternating (throttle bounce) (autocorr -0.59)

abi_native_cross_real_native_ffi_w's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.7x the fastest

Fastest abi_native_cross_real_null_entry (3.40 us) to slowest abi_native_cross_real_native_ffi_w (16.05 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3397.3 ns median (-78.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.72x (fastest 3397.3 ns, slowest 16050.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13588ns | 13368ns | 13107ns | 13336ns | 14206ns | -26.28% |
| abi_native_cross_real_native_ffi_w | 18432ns | 18300ns | 18146ns | 18258ns | 18837ns | base |
| abi_native_cross_real_null_entry | 5745ns | 5703ns | 5431ns | 5648ns | 6047ns | -68.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11315ns | 10947ns | 11818ns | -29.88% | 0.000 |
| abi_native_cross_real_native_ffi_w | 16138ns | 15912ns | 16445ns | base | 0.000 |
| abi_native_cross_real_null_entry | 3449ns | 3297ns | 3630ns | -78.63% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5490.7 | 11386.0 | 11315.1 | n/a |
| abi_native_cross_real_native_ffi_w | 26395.2 | 16322.3 | 16137.7 | n/a |
| abi_native_cross_real_null_entry | 27825.0 | 3554.8 | 3449.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 29.5% |
| abi_native_cross_real_native_ffi_w | 0.000 | 20.5% |
| abi_native_cross_real_null_entry | 0.001 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13588ns | 13588ns | -26.28% |
| abi_native_cross_real_native_ffi_w | 18432ns | 18432ns | base |
| abi_native_cross_real_null_entry | 5745ns | 5745ns | -68.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 16050ns | base | --- | [15918, 16445] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11158ns | -4782.9ns (-29.8%) | [-5059, -4626]ns | [10969, 11818] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3397ns | -12683.5ns (-79.0%) | [-12986, -12396]ns | [3320, 3630] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 15912ns | -29.8% | -78.6% |
| 2 | 16130ns | -31.9% | -79.6% |
| 3 | 15970ns | -30.3% | -78.8% |
| 4 | 16482ns | -27.5% | -79.7% |
| 5 | 15925ns | -31.3% | -77.2% |
| 6 | 16407ns | -28.7% | -77.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.458 | moderate- |
| abi_native_cross_real_native_ffi_w | -0.587 | HIGH- (thermal bounce) |
| abi_native_cross_real_null_entry | 0.331 | moderate+ |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 119869.5ns | 11315.1ns | 1059.4% | HIGH |
| abi_native_cross_real_native_ffi_w | 157434.9ns | 16137.7ns | 975.6% | HIGH |
| abi_native_cross_real_null_entry | 120944.5ns | 3449.2ns | 3506.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 10946.7-11818.5 ns)
  10946.7 |########################################
  10990.3 |########################################
  11033.9 |
  11077.5 |
  11121.1 |########################################
  11164.7 |########################################
  11208.2 |
  11251.8 |
  11295.4 |
  11339.0 |
  11382.6 |
  11426.2 |
  11469.8 |
  11513.4 |
  11557.0 |
  11600.5 |
  11644.1 |
  11687.7 |########################################
  11731.3 |
  11774.9 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 15911.7-16444.6 ns)
  15911.7 |########################################
  15938.3 |
  15965.0 |####################
  15991.6 |
  16018.3 |
  16044.9 |
  16071.6 |
  16098.2 |
  16124.9 |####################
  16151.5 |
  16178.1 |
  16204.8 |
  16231.4 |
  16258.1 |
  16284.7 |
  16311.4 |
  16338.0 |
  16364.7 |
  16391.3 |####################
  16418.0 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3297.1-3630.2 ns)
   3297.1 |########################################
   3313.8 |
   3330.4 |########################################
   3347.1 |
   3363.7 |
   3380.4 |########################################
   3397.0 |########################################
   3413.7 |
   3430.3 |
   3447.0 |
   3463.6 |
   3480.3 |
   3497.0 |
   3513.6 |
   3530.3 |
   3546.9 |
   3563.6 |
   3580.2 |
   3596.9 |
   3613.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1065.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=976.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3559.1% of algo (FFI overhead may distort results)
