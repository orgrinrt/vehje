# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (13.36 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 280% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.01 us) leads abi_native_cross_real_inproc_native (11.41 us) by 280%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 77% (significant)

abi_native_cross_real_null_entry is -10.30 us (77%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_real_native_ffi_w (13.36 us) is 4.4x the fastest (3.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_real_null_entry (3.01 us) to slowest abi_native_cross_real_native_ffi_w (13.36 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3006.4 ns median (-77.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.44x (fastest 3006.4 ns, slowest 13358.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13799ns | 13678ns | 13361ns | 13639ns | 14257ns | -11.68% |
| abi_native_cross_real_native_ffi_w | 15623ns | 15637ns | 15132ns | 15475ns | 16091ns | base |
| abi_native_cross_real_null_entry | 5310ns | 5229ns | 5149ns | 5224ns | 5520ns | -66.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11490ns | 11145ns | 11853ns | -14.01% | 0.001 |
| abi_native_cross_real_native_ffi_w | 13362ns | 12938ns | 13785ns | base | 0.001 |
| abi_native_cross_real_null_entry | 3061ns | 2963ns | 3198ns | -77.09% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5257.8 | 11529.2 | 11490.1 | n/a |
| abi_native_cross_real_native_ffi_w | 24987.6 | 13970.3 | 13362.3 | n/a |
| abi_native_cross_real_null_entry | 26490.3 | 3102.6 | 3061.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.001 | 26.0% |
| abi_native_cross_real_native_ffi_w | 0.001 | 22.2% |
| abi_native_cross_real_null_entry | 0.003 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13799ns | 13799ns | -11.68% |
| abi_native_cross_real_native_ffi_w | 15623ns | 15623ns | base |
| abi_native_cross_real_null_entry | 5310ns | 5310ns | -66.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 13358ns | base | --- | [12944, 13785] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11411ns | -1900.0ns (-14.2%) | [-1978, -1738]ns | [11206, 11853] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3006ns | -10298.3ns (-77.1%) | [-10784, -9822]ns | [2979, 3198] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 13338ns | -14.4% | -77.8% |
| 2 | 13389ns | -14.8% | -77.5% |
| 3 | 12950ns | -13.9% | -75.0% |
| 4 | 13379ns | -14.0% | -76.4% |
| 5 | 14180ns | -13.9% | -78.9% |
| 6 | 12938ns | -12.9% | -76.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.175 | ok |
| abi_native_cross_real_native_ffi_w | -0.345 | moderate- |
| abi_native_cross_real_null_entry | 0.166 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 120533.6ns | 11490.1ns | 1049.0% | HIGH |
| abi_native_cross_real_native_ffi_w | 142270.7ns | 13362.3ns | 1064.7% | HIGH |
| abi_native_cross_real_null_entry | 118787.9ns | 3061.0ns | 3880.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11145.0-11853.3 ns)
  11145.0 |####################
  11180.4 |
  11215.8 |
  11251.2 |####################
  11286.7 |
  11322.1 |
  11357.5 |
  11392.9 |########################################
  11428.3 |
  11463.7 |
  11499.1 |####################
  11534.6 |
  11570.0 |
  11605.4 |
  11640.8 |
  11676.2 |
  11711.6 |
  11747.1 |
  11782.5 |
  11817.9 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 12938.3-13784.6 ns)
  12938.3 |########################################
  12980.6 |
  13022.9 |
  13065.2 |
  13107.6 |
  13149.9 |
  13192.2 |
  13234.5 |
  13276.8 |
  13319.1 |####################
  13361.5 |########################################
  13403.8 |
  13446.1 |
  13488.4 |
  13530.7 |
  13573.0 |
  13615.3 |
  13657.7 |
  13700.0 |
  13742.3 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2962.9-3197.7 ns)
   2962.9 |####################
   2974.6 |
   2986.4 |####################
   2998.1 |########################################
   3009.9 |
   3021.6 |
   3033.3 |
   3045.1 |
   3056.8 |
   3068.6 |
   3080.3 |
   3092.0 |
   3103.8 |
   3115.5 |
   3127.3 |
   3139.0 |
   3150.7 |####################
   3162.5 |
   3174.2 |
   3186.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1058.9% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1073.0% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3942.2% of algo (FFI overhead may distort results)
