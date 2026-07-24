# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 262% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.16 us) leads abi_native_cross_real_inproc_native (11.44 us) by 262%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 73% (significant)

abi_native_cross_real_null_entry is -8.40 us (73%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 3.7x slower than the field

abi_native_cross_real_native_ffi_w (11.57 us) is 3.7x the fastest (3.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_real_null_entry (3.16 us) to slowest abi_native_cross_real_native_ffi_w (11.57 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_real_inproc_native is inconsistent: worst-20% is 2.6x its best-20%

abi_native_cross_real_inproc_native's best 20% of batches run at 11.15 us but its worst 20% at 29.54 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3160.4 ns median (-72.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.66x (fastest 3160.4 ns, slowest 11572.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 19720ns | 13693ns | 13299ns | 13603ns | 32105ns | +39.95% |
| abi_native_cross_real_native_ffi_w | 14090ns | 13836ns | 13705ns | 13803ns | 14713ns | base |
| abi_native_cross_real_null_entry | 5482ns | 5451ns | 5345ns | 5421ns | 5641ns | -61.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 17395ns | 11152ns | 29539ns | +47.46% | 0.015 |
| abi_native_cross_real_native_ffi_w | 11796ns | 11477ns | 12326ns | base | 0.022 |
| abi_native_cross_real_null_entry | 3173ns | 3127ns | 3230ns | -73.10% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5515.6 | 11690.8 | 17395.0 | 4 |
| abi_native_cross_real_native_ffi_w | 25473.6 | 11888.6 | 11796.2 | n/a |
| abi_native_cross_real_null_entry | 27192.4 | 3192.3 | 3173.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.022 | 27.3% |
| abi_native_cross_real_native_ffi_w | 0.022 | 27.0% |
| abi_native_cross_real_null_entry | 0.081 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 19720ns | 19720ns | +39.95% |
| abi_native_cross_real_native_ffi_w | 14090ns | 14090ns | base |
| abi_native_cross_real_null_entry | 5482ns | 5482ns | -61.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11573ns | base | --- | [11490, 12326] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11438ns | no significant difference | [-397, +17429]ns | [11208, 29539] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_real_null_entry | 3160ns | -8402.0ns (-72.6%) | [-9115, -8351]ns | [3130, 3230] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11477ns | -1.1% | -72.5% |
| 2 | 11537ns | -3.3% | -72.8% |
| 3 | 11608ns | -3.0% | -72.4% |
| 4 | 12243ns | +284.5% | -73.4% |
| 5 | 12408ns | -3.3% | -74.5% |
| 6 | 11503ns | +0.2% | -72.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.222 | moderate- |
| abi_native_cross_real_native_ffi_w | 0.164 | ok |
| abi_native_cross_real_null_entry | 0.182 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 163489.6ns | 17395.0ns | 939.9% | HIGH |
| abi_native_cross_real_native_ffi_w | 142400.5ns | 11796.2ns | 1207.2% | HIGH |
| abi_native_cross_real_null_entry | 120137.9ns | 3173.4ns | 3785.8% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11152.5-29539.2 ns)
  11152.5 |########################################
  12071.8 |
  12991.2 |
  13910.5 |
  14829.8 |
  15749.2 |
  16668.5 |
  17587.8 |
  18507.2 |
  19426.5 |
  20345.8 |
  21265.2 |
  22184.5 |
  23103.9 |
  24023.2 |
  24942.5 |
  25861.9 |
  26781.2 |
  27700.5 |
  28619.9 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11477.1-12325.6 ns)
  11477.1 |########################################
  11519.5 |####################
  11562.0 |
  11604.4 |####################
  11646.8 |
  11689.2 |
  11731.6 |
  11774.1 |
  11816.5 |
  11858.9 |
  11901.3 |
  11943.8 |
  11986.2 |
  12028.6 |
  12071.0 |
  12113.5 |
  12155.9 |
  12198.3 |
  12240.7 |####################
  12283.2 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3126.7-3230.2 ns)
   3126.7 |########################################
   3131.9 |########################################
   3137.1 |
   3142.2 |
   3147.4 |########################################
   3152.6 |
   3157.8 |
   3162.9 |
   3168.1 |########################################
   3173.3 |
   3178.5 |
   3183.7 |
   3188.8 |
   3194.0 |
   3199.2 |
   3204.4 |########################################
   3209.5 |
   3214.7 |
   3219.9 |
   3225.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: CV=76.3% (high variance, measurements may be unstable)
- **abi_native_cross_real_inproc_native**: bridge=1069.7% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1227.2% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3792.2% of algo (FFI overhead may distort results)
