# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (17.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 244% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.47 us) leads abi_native_cross_real_inproc_native (11.93 us) by 244%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -13.67 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 5.0x slower than the field

abi_native_cross_real_native_ffi_w (17.21 us) is 5.0x the fastest (3.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_inproc_native shows alternating (throttle bounce) (autocorr -0.58)

abi_native_cross_real_inproc_native's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.0x the fastest

Fastest abi_native_cross_real_null_entry (3.47 us) to slowest abi_native_cross_real_native_ffi_w (17.21 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_real_inproc_native is inconsistent: worst-20% is 3.3x its best-20%

abi_native_cross_real_inproc_native's best 20% of batches run at 11.32 us but its worst 20% at 36.85 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3466.9 ns median (-79.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.97x (fastest 3466.9 ns, slowest 17214.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 22865ns | 14262ns | 13583ns | 14139ns | 40595ns | +11.04% |
| abi_native_cross_real_native_ffi_w | 20592ns | 19580ns | 18298ns | 19257ns | 23742ns | base |
| abi_native_cross_real_null_entry | 5756ns | 5751ns | 5611ns | 5713ns | 5894ns | -72.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 20073ns | 11316ns | 36850ns | +10.81% | 0.000 |
| abi_native_cross_real_native_ffi_w | 18115ns | 16025ns | 20978ns | base | 0.000 |
| abi_native_cross_real_null_entry | 3475ns | 3375ns | 3567ns | -80.82% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 6413.3 | 45046.7 | 20072.9 | 13 |
| abi_native_cross_real_native_ffi_w | 32730.0 | 19251.7 | 18115.3 | n/a |
| abi_native_cross_real_null_entry | 29581.0 | 3563.5 | 3474.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 28.3% |
| abi_native_cross_real_native_ffi_w | 0.000 | 19.6% |
| abi_native_cross_real_null_entry | 0.001 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 22865ns | 22865ns | +11.04% |
| abi_native_cross_real_native_ffi_w | 20592ns | 20592ns | base |
| abi_native_cross_real_null_entry | 5756ns | 5756ns | -72.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 17214ns | base | --- | [16154, 20978] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11927ns | no significant difference | [-6579, +17491]ns | [11442, 36850] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_real_null_entry | 3467ns | -13672.1ns (-79.4%) | [-17485, -12764]ns | [3390, 3567] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 16938ns | -31.7% | -79.5% |
| 2 | 16283ns | +134.8% | -79.1% |
| 3 | 16025ns | -29.4% | -78.9% |
| 4 | 19522ns | -39.7% | -82.2% |
| 5 | 22433ns | +58.1% | -84.3% |
| 6 | 17490ns | -30.9% | -79.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.579 | HIGH- (thermal bounce) |
| abi_native_cross_real_native_ffi_w | 0.213 | moderate+ |
| abi_native_cross_real_null_entry | 0.373 | moderate+ |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 182473.6ns | 20072.9ns | 909.1% | HIGH |
| abi_native_cross_real_native_ffi_w | 169235.3ns | 18115.3ns | 934.2% | HIGH |
| abi_native_cross_real_null_entry | 122925.7ns | 3474.8ns | 3537.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11316.2-36849.6 ns)
  11316.2 |########################################
  12592.9 |
  13869.5 |
  15146.2 |
  16422.9 |
  17699.6 |
  18976.2 |
  20252.9 |
  21529.6 |
  22806.2 |
  24082.9 |
  25359.6 |
  26636.2 |
  27912.9 |
  29189.6 |
  30466.3 |
  31742.9 |
  33019.6 |
  34296.3 |##########
  35572.9 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 16024.6-20977.5 ns)
  16024.6 |########################################
  16272.2 |########################################
  16519.9 |
  16767.5 |########################################
  17015.2 |
  17262.8 |########################################
  17510.5 |
  17758.1 |
  18005.8 |
  18253.4 |
  18501.0 |
  18748.7 |
  18996.3 |
  19244.0 |
  19491.6 |########################################
  19739.3 |
  19986.9 |
  20234.6 |
  20482.2 |
  20729.9 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3374.6-3567.4 ns)
   3374.6 |####################
   3384.2 |
   3393.9 |
   3403.5 |####################
   3413.2 |
   3422.8 |
   3432.5 |
   3442.1 |
   3451.7 |
   3461.4 |########################################
   3471.0 |
   3480.7 |
   3490.3 |
   3500.0 |
   3509.6 |####################
   3519.2 |
   3528.9 |
   3538.5 |
   3548.2 |
   3557.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: CV=59.2% (high variance, measurements may be unstable)
- **abi_native_cross_real_inproc_native**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **abi_native_cross_real_inproc_native**: bridge=1053.4% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=931.5% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3526.7% of algo (FFI overhead may distort results)
