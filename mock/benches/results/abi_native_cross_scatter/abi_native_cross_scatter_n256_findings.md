# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (11.67 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 3.12 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 267% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (3.12 us) leads abi_native_cross_scatter_inproc_native (11.48 us) by 267%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 73% (significant)

abi_native_cross_scatter_null_entry is -8.55 us (73%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 3.7x slower than the field

abi_native_cross_scatter_native_ffi_w (11.67 us) is 3.7x the fastest (3.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_scatter_null_entry (3.12 us) to slowest abi_native_cross_scatter_native_ffi_w (11.67 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 3123.9 ns median (-73.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.74x (fastest 3123.9 ns, slowest 11672.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13788ns | 13773ns | 13537ns | 13706ns | 14037ns | -0.14% |
| abi_native_cross_scatter_native_ffi_w | 13808ns | 13946ns | 13323ns | 13839ns | 14002ns | base |
| abi_native_cross_scatter_null_entry | 5368ns | 5350ns | 5163ns | 5329ns | 5528ns | -61.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11472ns | 11225ns | 11667ns | -0.76% | 0.022 |
| abi_native_cross_scatter_native_ffi_w | 11560ns | 11151ns | 11730ns | base | 0.022 |
| abi_native_cross_scatter_null_entry | 3134ns | 3022ns | 3215ns | -72.89% | 0.082 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5294.8 | 11509.3 | 11472.3 | n/a |
| abi_native_cross_scatter_native_ffi_w | 25279.5 | 11685.1 | 11560.3 | n/a |
| abi_native_cross_scatter_null_entry | 26421.1 | 3137.4 | 3134.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.085 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.022 | 26.3% |
| abi_native_cross_scatter_native_ffi_w | 0.022 | 25.9% |
| abi_native_cross_scatter_null_entry | 0.082 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13788ns | 13788ns | -0.14% |
| abi_native_cross_scatter_native_ffi_w | 13808ns | 13808ns | base |
| abi_native_cross_scatter_null_entry | 5368ns | 5368ns | -61.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 11673ns | base | --- | [11278, 11730] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11480ns | no significant difference | [-453, +265]ns | [11270, 11667] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_scatter_null_entry | 3124ns | -8548.8ns (-73.2%) | [-8649, -8081]ns | [3064, 3215] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 11705ns | -0.7% | -74.2% |
| 2 | 11691ns | -4.0% | -73.2% |
| 3 | 11406ns | -0.6% | -72.8% |
| 4 | 11151ns | +4.6% | -70.5% |
| 5 | 11755ns | -3.7% | -73.3% |
| 6 | 11655ns | +0.2% | -73.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | -0.446 | moderate- |
| abi_native_cross_scatter_native_ffi_w | 0.003 | ok |
| abi_native_cross_scatter_null_entry | -0.068 | ok |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 121394.8ns | 11472.3ns | 1058.2% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 142572.2ns | 11560.3ns | 1233.3% | HIGH |
| abi_native_cross_scatter_null_entry | 119673.1ns | 3134.2ns | 3818.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11225.0-11667.3 ns)
  11225.0 |####################
  11247.1 |
  11269.2 |
  11291.3 |
  11313.5 |########################################
  11335.6 |
  11357.7 |
  11379.8 |
  11401.9 |
  11424.0 |
  11446.1 |
  11468.3 |
  11490.4 |
  11512.5 |
  11534.6 |
  11556.7 |
  11578.8 |
  11601.0 |
  11623.1 |####################
  11645.2 |####################
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 11150.8-11729.8 ns)
  11150.8 |########################################
  11179.8 |
  11208.7 |
  11237.6 |
  11266.6 |
  11295.5 |
  11324.5 |
  11353.4 |
  11382.4 |########################################
  11411.3 |
  11440.3 |
  11469.2 |
  11498.2 |
  11527.1 |
  11556.1 |
  11585.0 |
  11614.0 |
  11642.9 |########################################
  11671.9 |########################################
  11700.8 |########################################
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 3021.7-3215.0 ns)
   3021.7 |####################
   3031.4 |
   3041.0 |
   3050.7 |
   3060.4 |
   3070.0 |
   3079.7 |
   3089.4 |
   3099.0 |####################
   3108.7 |
   3118.3 |########################################
   3128.0 |
   3137.7 |####################
   3147.3 |
   3157.0 |
   3166.7 |
   3176.3 |
   3186.0 |
   3195.7 |
   3205.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1058.7% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1230.7% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=3809.8% of algo (FFI overhead may distort results)
