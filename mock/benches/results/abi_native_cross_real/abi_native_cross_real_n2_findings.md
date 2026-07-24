# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (16.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 252% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.36 us) leads abi_native_cross_real_inproc_native (11.84 us) by 252%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 78% (significant)

abi_native_cross_real_null_entry is -12.78 us (78%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_real_native_ffi_w (16.30 us) is 4.8x the fastest (3.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_real_null_entry (3.36 us) to slowest abi_native_cross_real_native_ffi_w (16.30 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3364.6 ns median (-79.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.84x (fastest 3364.6 ns, slowest 16297.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14162ns | 14245ns | 13326ns | 14204ns | 14518ns | -25.24% |
| abi_native_cross_real_native_ffi_w | 18943ns | 18555ns | 18085ns | 18440ns | 20127ns | base |
| abi_native_cross_real_null_entry | 5640ns | 5600ns | 5483ns | 5575ns | 5817ns | -70.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11794ns | 11134ns | 12124ns | -29.32% | 0.000 |
| abi_native_cross_real_native_ffi_w | 16687ns | 15880ns | 17835ns | base | 0.000 |
| abi_native_cross_real_null_entry | 3407ns | 3335ns | 3513ns | -79.58% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5395.4 | 11834.0 | 11794.2 | n/a |
| abi_native_cross_real_native_ffi_w | 25358.5 | 16671.0 | 16687.1 | n/a |
| abi_native_cross_real_null_entry | 27309.2 | 3492.6 | 3406.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 28.2% |
| abi_native_cross_real_native_ffi_w | 0.000 | 20.5% |
| abi_native_cross_real_null_entry | 0.001 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14162ns | 14162ns | -25.24% |
| abi_native_cross_real_native_ffi_w | 18943ns | 18943ns | base |
| abi_native_cross_real_null_entry | 5640ns | 5640ns | -70.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 16298ns | base | --- | [15929, 17835] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11836ns | -4361.4ns (-26.8%) | [-6233, -4084]ns | [11423, 12124] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3365ns | -12784.4ns (-78.4%) | [-14480, -12576]ns | [3342, 3513] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 19175ns | -38.2% | -82.4% |
| 2 | 16495ns | -25.6% | -79.8% |
| 3 | 16282ns | -31.6% | -78.4% |
| 4 | 16313ns | -27.6% | -78.5% |
| 5 | 15978ns | -25.0% | -79.0% |
| 6 | 15880ns | -26.2% | -78.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.437 | moderate- |
| abi_native_cross_real_native_ffi_w | 0.077 | ok |
| abi_native_cross_real_null_entry | 0.094 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 122524.7ns | 11794.2ns | 1038.9% | HIGH |
| abi_native_cross_real_native_ffi_w | 157538.8ns | 16687.1ns | 944.1% | HIGH |
| abi_native_cross_real_null_entry | 119482.2ns | 3406.7ns | 3507.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11133.7-12123.5 ns)
  11133.7 |########################################
  11183.2 |
  11232.7 |
  11282.2 |
  11331.7 |
  11381.2 |
  11430.7 |
  11480.1 |
  11529.6 |
  11579.1 |
  11628.6 |
  11678.1 |########################################
  11727.6 |
  11777.1 |########################################
  11826.6 |########################################
  11876.1 |
  11925.6 |
  11975.1 |########################################
  12024.6 |
  12074.1 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 15880.0-17835.0 ns)
  15880.0 |########################################
  15977.8 |
  16075.5 |
  16173.2 |
  16271.0 |########################################
  16368.8 |
  16466.5 |####################
  16564.2 |
  16662.0 |
  16759.8 |
  16857.5 |
  16955.2 |
  17053.0 |
  17150.8 |
  17248.5 |
  17346.2 |
  17444.0 |
  17541.8 |
  17639.5 |
  17737.2 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3334.6-3513.1 ns)
   3334.6 |########################################
   3343.5 |########################################
   3352.4 |########################################
   3361.4 |
   3370.3 |########################################
   3379.2 |
   3388.2 |
   3397.1 |
   3406.0 |
   3414.9 |
   3423.8 |
   3432.8 |
   3441.7 |
   3450.6 |
   3459.5 |
   3468.5 |
   3477.4 |
   3486.3 |
   3495.2 |
   3504.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1042.4% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=965.6% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3544.3% of algo (FFI overhead may distort results)
