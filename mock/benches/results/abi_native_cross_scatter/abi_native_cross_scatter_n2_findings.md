# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (16.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 3.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 244% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (3.43 us) leads abi_native_cross_scatter_inproc_native (11.79 us) by 244%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 78% (significant)

abi_native_cross_scatter_null_entry is -12.86 us (78%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_scatter_native_ffi_w (16.40 us) is 4.8x the fastest (3.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_scatter_null_entry shows alternating (throttle bounce) (autocorr -0.55)

abi_native_cross_scatter_null_entry's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_scatter_null_entry (3.43 us) to slowest abi_native_cross_scatter_native_ffi_w (16.40 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 3431.1 ns median (-79.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.78x (fastest 3431.1 ns, slowest 16396.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14099ns | 14140ns | 13386ns | 13951ns | 14677ns | -25.90% |
| abi_native_cross_scatter_native_ffi_w | 19028ns | 18643ns | 18527ns | 18612ns | 19901ns | base |
| abi_native_cross_scatter_null_entry | 5745ns | 5709ns | 5500ns | 5642ns | 6021ns | -69.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11756ns | 11142ns | 12233ns | -29.93% | 0.000 |
| abi_native_cross_scatter_native_ffi_w | 16779ns | 16252ns | 17645ns | base | 0.000 |
| abi_native_cross_scatter_null_entry | 3473ns | 3338ns | 3649ns | -79.30% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5465.7 | 11791.2 | 11756.3 | n/a |
| abi_native_cross_scatter_native_ffi_w | 25086.5 | 17232.6 | 16778.7 | n/a |
| abi_native_cross_scatter_null_entry | 26004.2 | 3542.3 | 3472.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.000 | 28.3% |
| abi_native_cross_scatter_native_ffi_w | 0.000 | 20.4% |
| abi_native_cross_scatter_null_entry | 0.001 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14099ns | 14099ns | -25.90% |
| abi_native_cross_scatter_native_ffi_w | 19028ns | 19028ns | base |
| abi_native_cross_scatter_null_entry | 5745ns | 5745ns | -69.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 16396ns | base | --- | [16295, 17645] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11795ns | -4752.5ns (-29.0%) | [-6212, -4103]ns | [11241, 12233] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_scatter_null_entry | 3431ns | -12863.6ns (-78.5%) | [-14280, -12774]ns | [3339, 3649] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 18864ns | -37.8% | -82.3% |
| 2 | 16420ns | -25.1% | -77.9% |
| 3 | 16252ns | -25.1% | -79.4% |
| 4 | 16337ns | -27.4% | -78.5% |
| 5 | 16427ns | -32.2% | -77.7% |
| 6 | 16373ns | -30.7% | -79.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.431 | moderate+ |
| abi_native_cross_scatter_native_ffi_w | -0.005 | ok |
| abi_native_cross_scatter_null_entry | -0.549 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 120218.0ns | 11756.3ns | 1022.6% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 158245.8ns | 16778.7ns | 943.1% | HIGH |
| abi_native_cross_scatter_null_entry | 119475.4ns | 3472.9ns | 3440.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11142.1-12233.0 ns)
  11142.1 |########################################
  11196.6 |
  11251.2 |
  11305.7 |########################################
  11360.3 |
  11414.8 |
  11469.4 |
  11523.9 |
  11578.4 |
  11633.0 |
  11687.5 |########################################
  11742.1 |
  11796.6 |
  11851.2 |########################################
  11905.7 |
  11960.2 |
  12014.8 |
  12069.3 |
  12123.9 |########################################
  12178.4 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 16252.5-17645.2 ns)
  16252.5 |####################
  16322.1 |########################################
  16391.8 |########################################
  16461.4 |
  16531.0 |
  16600.7 |
  16670.3 |
  16740.0 |
  16809.6 |
  16879.2 |
  16948.9 |
  17018.5 |
  17088.2 |
  17157.8 |
  17227.4 |
  17297.1 |
  17366.7 |
  17436.3 |
  17506.0 |
  17575.6 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 3337.5-3649.2 ns)
   3337.5 |########################################
   3353.1 |
   3368.7 |
   3384.2 |
   3399.8 |
   3415.4 |
   3431.0 |
   3446.6 |
   3462.2 |
   3477.7 |
   3493.3 |
   3508.9 |#############
   3524.5 |
   3540.1 |
   3555.7 |
   3571.2 |
   3586.8 |
   3602.4 |
   3618.0 |#############
   3633.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1028.2% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=965.1% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=3470.2% of algo (FFI overhead may distort results)
