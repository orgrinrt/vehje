# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 399% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.30 us) leads abi_native_cross_real_inproc_native (11.47 us) by 399%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 81% (significant)

abi_native_cross_real_null_entry is -9.98 us (81%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 5.3x slower than the field

abi_native_cross_real_native_ffi_w (12.28 us) is 5.3x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.3x the fastest

Fastest abi_native_cross_real_null_entry (2.30 us) to slowest abi_native_cross_real_native_ffi_w (12.28 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2298.6 ns median (-81.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.34x (fastest 2298.6 ns, slowest 12281.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13826ns | 13772ns | 13681ns | 13742ns | 14026ns | -5.17% |
| abi_native_cross_real_native_ffi_w | 14580ns | 14588ns | 14202ns | 14492ns | 14902ns | base |
| abi_native_cross_real_null_entry | 4578ns | 4566ns | 4516ns | 4551ns | 4651ns | -68.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11554ns | 11428ns | 11743ns | -6.10% | 0.003 |
| abi_native_cross_real_native_ffi_w | 12304ns | 11981ns | 12624ns | base | 0.003 |
| abi_native_cross_real_null_entry | 2302ns | 2276ns | 2328ns | -81.30% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5681.1 | 11666.7 | 11554.2 | n/a |
| abi_native_cross_real_native_ffi_w | 27068.1 | 12487.0 | 12304.3 | n/a |
| abi_native_cross_real_null_entry | 29092.6 | 2455.6 | 2301.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.003 | 19.8% |
| abi_native_cross_real_native_ffi_w | 0.003 | 18.5% |
| abi_native_cross_real_null_entry | 0.014 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13826ns | 13826ns | -5.17% |
| abi_native_cross_real_native_ffi_w | 14580ns | 14580ns | base |
| abi_native_cross_real_null_entry | 4578ns | 4578ns | -68.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12282ns | base | --- | [12007, 12624] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11472ns | -746.7ns (-6.1%) | [-1174, -330]ns | [11448, 11743] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2299ns | -9983.4ns (-81.3%) | [-10346, -9679]ns | [2278, 2328] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12332ns | -7.0% | -81.5% |
| 2 | 12232ns | -5.1% | -81.1% |
| 3 | 12032ns | -4.7% | -80.7% |
| 4 | 11981ns | -0.8% | -80.5% |
| 5 | 12403ns | -7.5% | -81.6% |
| 6 | 12845ns | -11.0% | -82.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.352 | moderate- |
| abi_native_cross_real_native_ffi_w | 0.261 | moderate+ |
| abi_native_cross_real_null_entry | 0.204 | moderate+ |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 122344.9ns | 11554.2ns | 1058.9% | HIGH |
| abi_native_cross_real_native_ffi_w | 144188.8ns | 12304.3ns | 1171.9% | HIGH |
| abi_native_cross_real_null_entry | 117696.5ns | 2301.5ns | 5113.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11427.9-11742.7 ns)
  11427.9 |#############
  11443.6 |
  11459.4 |########################################
  11475.1 |
  11490.9 |
  11506.6 |
  11522.3 |
  11538.1 |
  11553.8 |
  11569.6 |
  11585.3 |
  11601.0 |#############
  11616.8 |
  11632.5 |
  11648.3 |
  11664.0 |
  11679.7 |
  11695.5 |
  11711.2 |
  11727.0 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11981.2-12624.1 ns)
  11981.2 |########################################
  12013.3 |########################################
  12045.5 |
  12077.6 |
  12109.8 |
  12141.9 |
  12174.1 |
  12206.2 |########################################
  12238.4 |
  12270.5 |
  12302.7 |########################################
  12334.8 |
  12367.0 |
  12399.1 |########################################
  12431.3 |
  12463.4 |
  12495.6 |
  12527.7 |
  12559.9 |
  12592.0 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2276.2-2327.9 ns)
   2276.2 |########################################
   2278.8 |########################################
   2281.4 |
   2284.0 |
   2286.5 |########################################
   2289.1 |
   2291.7 |
   2294.3 |
   2296.9 |
   2299.5 |
   2302.1 |
   2304.6 |
   2307.2 |
   2309.8 |########################################
   2312.4 |
   2315.0 |
   2317.6 |
   2320.1 |########################################
   2322.7 |
   2325.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1063.2% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1172.9% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=5138.1% of algo (FFI overhead may distort results)
