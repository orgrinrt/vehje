# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (13.31 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 3.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 285% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (3.03 us) leads abi_native_cross_madd_inproc_native (11.66 us) by 285%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 76% (significant)

abi_native_cross_madd_null_entry is -10.17 us (76%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_madd_native_ffi_w (13.31 us) is 4.4x the fastest (3.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_madd_native_ffi_w shows alternating (throttle bounce) (autocorr -0.56)

abi_native_cross_madd_native_ffi_w's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_madd_null_entry (3.03 us) to slowest abi_native_cross_madd_native_ffi_w (13.31 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 3032.5 ns median (-77.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.39x (fastest 3032.5 ns, slowest 13309.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13838ns | 13915ns | 13281ns | 13756ns | 14241ns | -10.69% |
| abi_native_cross_madd_native_ffi_w | 15494ns | 15570ns | 15138ns | 15463ns | 15718ns | base |
| abi_native_cross_madd_null_entry | 5325ns | 5263ns | 5146ns | 5231ns | 5555ns | -65.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11558ns | 11142ns | 11835ns | -12.76% | 0.001 |
| abi_native_cross_madd_native_ffi_w | 13248ns | 12954ns | 13454ns | base | 0.001 |
| abi_native_cross_madd_null_entry | 3060ns | 2966ns | 3170ns | -76.91% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5315.8 | 11658.3 | 11558.4 | n/a |
| abi_native_cross_madd_native_ffi_w | 25254.6 | 13732.9 | 13248.5 | n/a |
| abi_native_cross_madd_null_entry | 26738.3 | 3092.8 | 3059.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.001 | 25.4% |
| abi_native_cross_madd_native_ffi_w | 0.001 | 22.3% |
| abi_native_cross_madd_null_entry | 0.003 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13838ns | 13838ns | -10.69% |
| abi_native_cross_madd_native_ffi_w | 15494ns | 15494ns | base |
| abi_native_cross_madd_null_entry | 5325ns | 5325ns | -65.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 13310ns | base | --- | [12982, 13454] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11662ns | -1728.4ns (-13.0%) | [-1929, -1413]ns | [11179, 11835] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_madd_null_entry | 3032ns | -10171.7ns (-76.4%) | [-10422, -9972]ns | [2976, 3170] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 13011ns | -9.3% | -77.2% |
| 2 | 13485ns | -12.0% | -77.9% |
| 3 | 13262ns | -15.4% | -76.4% |
| 4 | 13422ns | -13.4% | -76.1% |
| 5 | 12954ns | -14.0% | -76.4% |
| 6 | 13358ns | -12.4% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.291 | moderate- |
| abi_native_cross_madd_native_ffi_w | -0.555 | HIGH- (thermal bounce) |
| abi_native_cross_madd_null_entry | 0.272 | moderate+ |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 122050.3ns | 11558.4ns | 1055.9% | HIGH |
| abi_native_cross_madd_native_ffi_w | 144183.1ns | 13248.5ns | 1088.3% | HIGH |
| abi_native_cross_madd_null_entry | 118958.5ns | 3059.7ns | 3887.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11141.7-11835.0 ns)
  11141.7 |########################################
  11176.4 |
  11211.0 |########################################
  11245.7 |
  11280.4 |
  11315.0 |
  11349.7 |
  11384.4 |
  11419.0 |
  11453.7 |
  11488.4 |
  11523.0 |
  11557.7 |
  11592.3 |########################################
  11627.0 |
  11661.7 |
  11696.3 |########################################
  11731.0 |
  11765.7 |
  11800.3 |########################################
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 12953.7-13453.5 ns)
  12953.7 |########################################
  12978.7 |
  13003.7 |########################################
  13028.7 |
  13053.7 |
  13078.7 |
  13103.7 |
  13128.6 |
  13153.6 |
  13178.6 |
  13203.6 |
  13228.6 |
  13253.6 |########################################
  13278.6 |
  13303.6 |
  13328.6 |
  13353.6 |########################################
  13378.6 |
  13403.6 |########################################
  13428.6 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 2965.8-3170.4 ns)
   2965.8 |########################################
   2976.0 |
   2986.3 |########################################
   2996.5 |
   3006.7 |########################################
   3017.0 |
   3027.2 |
   3037.4 |
   3047.6 |########################################
   3057.9 |
   3068.1 |
   3078.3 |
   3088.6 |
   3098.8 |
   3109.0 |
   3119.2 |
   3129.5 |########################################
   3139.7 |
   3149.9 |
   3160.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1059.2% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1086.8% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=3917.7% of algo (FFI overhead may distort results)
