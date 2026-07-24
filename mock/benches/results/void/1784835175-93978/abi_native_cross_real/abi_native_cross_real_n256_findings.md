# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.67 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.13 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 268% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.13 us) leads abi_native_cross_real_inproc_native (11.53 us) by 268%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 74% (significant)

abi_native_cross_real_null_entry is -8.58 us (74%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 3.7x slower than the field

abi_native_cross_real_native_ffi_w (11.67 us) is 3.7x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_real_null_entry (3.13 us) to slowest abi_native_cross_real_native_ffi_w (11.67 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3134.8 ns median (-73.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.72x (fastest 3134.8 ns, slowest 11671.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13872ns | 13898ns | 13499ns | 13834ns | 14114ns | -0.60% |
| abi_native_cross_real_native_ffi_w | 13955ns | 13985ns | 13641ns | 13933ns | 14146ns | base |
| abi_native_cross_real_null_entry | 5381ns | 5388ns | 5155ns | 5372ns | 5505ns | -61.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11517ns | 11209ns | 11742ns | -1.25% | 0.022 |
| abi_native_cross_real_native_ffi_w | 11663ns | 11439ns | 11844ns | base | 0.022 |
| abi_native_cross_real_null_entry | 3130ns | 3005ns | 3194ns | -73.17% | 0.082 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5498.9 | 11700.1 | 11517.3 | n/a |
| abi_native_cross_real_native_ffi_w | 25868.6 | 11759.4 | 11662.6 | n/a |
| abi_native_cross_real_null_entry | 26815.2 | 3147.2 | 3129.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.085 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.022 | 26.1% |
| abi_native_cross_real_native_ffi_w | 0.022 | 25.7% |
| abi_native_cross_real_null_entry | 0.082 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13872ns | 13872ns | -0.60% |
| abi_native_cross_real_native_ffi_w | 13955ns | 13955ns | base |
| abi_native_cross_real_null_entry | 5381ns | 5381ns | -61.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11671ns | base | --- | [11472, 11844] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11531ns | no significant difference | [-274, +12]ns | [11279, 11742] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_real_null_entry | 3135ns | -8580.9ns (-73.5%) | [-8719, -8300]ns | [3060, 3194] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11667ns | +0.3% | -74.2% |
| 2 | 11961ns | -1.6% | -73.4% |
| 3 | 11439ns | -2.0% | -72.0% |
| 4 | 11506ns | -1.4% | -72.7% |
| 5 | 11676ns | -2.7% | -73.3% |
| 6 | 11728ns | -0.1% | -73.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.057 | ok |
| abi_native_cross_real_native_ffi_w | -0.189 | ok |
| abi_native_cross_real_null_entry | -0.083 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 121976.5ns | 11517.3ns | 1059.1% | HIGH |
| abi_native_cross_real_native_ffi_w | 144078.9ns | 11662.6ns | 1235.4% | HIGH |
| abi_native_cross_real_null_entry | 120085.1ns | 3129.7ns | 3837.0% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11209.2-11741.9 ns)
  11209.2 |####################
  11235.8 |
  11262.5 |
  11289.1 |
  11315.7 |
  11342.4 |########################################
  11369.0 |
  11395.6 |
  11422.3 |
  11448.9 |
  11475.6 |
  11502.2 |
  11528.8 |
  11555.5 |
  11582.1 |
  11608.7 |
  11635.4 |
  11662.0 |
  11688.6 |########################################
  11715.3 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11438.7-11844.1 ns)
  11438.7 |####################
  11459.0 |
  11479.2 |
  11499.5 |####################
  11519.8 |
  11540.1 |
  11560.3 |
  11580.6 |
  11600.9 |
  11621.2 |
  11641.4 |
  11661.7 |########################################
  11682.0 |
  11702.2 |
  11722.5 |####################
  11742.8 |
  11763.1 |
  11783.3 |
  11803.6 |
  11823.9 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3005.0-3193.8 ns)
   3005.0 |########################################
   3014.4 |
   3023.9 |
   3033.3 |
   3042.8 |
   3052.2 |
   3061.6 |
   3071.1 |
   3080.5 |
   3089.9 |
   3099.4 |
   3108.8 |########################################
   3118.2 |########################################
   3127.7 |
   3137.1 |########################################
   3146.6 |
   3156.0 |
   3165.4 |
   3174.9 |
   3184.3 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1061.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1231.5% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3833.1% of algo (FFI overhead may distort results)
