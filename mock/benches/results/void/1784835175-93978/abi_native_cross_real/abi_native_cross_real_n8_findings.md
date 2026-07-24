# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (13.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.00 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 284% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.00 us) leads abi_native_cross_real_inproc_native (11.52 us) by 284%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 77% (significant)

abi_native_cross_real_null_entry is -10.22 us (77%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_real_native_ffi_w (13.23 us) is 4.4x the fastest (3.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_real_null_entry (3.00 us) to slowest abi_native_cross_real_native_ffi_w (13.23 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3001.2 ns median (-77.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.41x (fastest 3001.2 ns, slowest 13227.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13979ns | 13858ns | 13562ns | 13805ns | 14449ns | -9.98% |
| abi_native_cross_real_native_ffi_w | 15528ns | 15504ns | 15006ns | 15452ns | 15902ns | base |
| abi_native_cross_real_null_entry | 5250ns | 5190ns | 5093ns | 5175ns | 5440ns | -66.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11604ns | 11325ns | 11960ns | -12.65% | 0.001 |
| abi_native_cross_real_native_ffi_w | 13285ns | 12847ns | 13631ns | base | 0.001 |
| abi_native_cross_real_null_entry | 3020ns | 2920ns | 3115ns | -77.27% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5606.2 | 11684.1 | 11603.7 | n/a |
| abi_native_cross_real_native_ffi_w | 25688.3 | 13776.7 | 13284.5 | n/a |
| abi_native_cross_real_null_entry | 26246.9 | 3128.8 | 3019.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.001 | 25.4% |
| abi_native_cross_real_native_ffi_w | 0.001 | 22.1% |
| abi_native_cross_real_null_entry | 0.003 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13979ns | 13979ns | -9.98% |
| abi_native_cross_real_native_ffi_w | 15528ns | 15528ns | base |
| abi_native_cross_real_null_entry | 5250ns | 5250ns | -66.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 13228ns | base | --- | [12995, 13631] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11518ns | -1685.4ns (-12.7%) | [-2132, -1225]ns | [11333, 11960] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3001ns | -10215.8ns (-77.2%) | [-10574, -10004]ns | [2943, 3115] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 13142ns | -11.5% | -77.8% |
| 2 | 13442ns | -15.8% | -77.6% |
| 3 | 13255ns | -7.6% | -76.4% |
| 4 | 13820ns | -15.5% | -77.6% |
| 5 | 12847ns | -11.2% | -76.9% |
| 6 | 13201ns | -14.1% | -77.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.172 | ok |
| abi_native_cross_real_native_ffi_w | -0.454 | moderate- |
| abi_native_cross_real_null_entry | 0.184 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 122226.4ns | 11603.7ns | 1053.3% | HIGH |
| abi_native_cross_real_native_ffi_w | 143762.4ns | 13284.5ns | 1082.2% | HIGH |
| abi_native_cross_real_null_entry | 118070.4ns | 3019.8ns | 3909.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11324.6-11959.5 ns)
  11324.6 |########################################
  11356.3 |
  11388.1 |####################
  11419.8 |
  11451.6 |
  11483.3 |
  11515.1 |
  11546.8 |
  11578.6 |
  11610.3 |####################
  11642.1 |####################
  11673.8 |
  11705.6 |
  11737.3 |
  11769.1 |
  11800.8 |
  11832.6 |
  11864.3 |
  11896.1 |
  11927.8 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 12847.1-13630.9 ns)
  12847.1 |########################################
  12886.3 |
  12925.5 |
  12964.7 |
  13003.9 |
  13043.0 |
  13082.2 |
  13121.4 |########################################
  13160.6 |
  13199.8 |########################################
  13239.0 |########################################
  13278.2 |
  13317.4 |
  13356.5 |
  13395.7 |
  13434.9 |########################################
  13474.1 |
  13513.3 |
  13552.5 |
  13591.7 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2920.0-3114.8 ns)
   2920.0 |########################################
   2929.7 |
   2939.5 |
   2949.2 |
   2959.0 |########################################
   2968.7 |
   2978.4 |
   2988.2 |########################################
   2997.9 |
   3007.7 |########################################
   3017.4 |
   3027.1 |
   3036.9 |
   3046.6 |
   3056.4 |
   3066.1 |
   3075.8 |
   3085.6 |
   3095.3 |########################################
   3105.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1061.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1090.4% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3934.2% of algo (FFI overhead may distort results)
