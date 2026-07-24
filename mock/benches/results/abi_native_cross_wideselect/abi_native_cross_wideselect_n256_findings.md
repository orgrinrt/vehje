# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (11.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 3.12 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 260% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (3.12 us) leads abi_native_cross_wideselect_inproc_native (11.24 us) by 260%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 72% (significant)

abi_native_cross_wideselect_null_entry is -8.14 us (72%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 3.6x slower than the field

abi_native_cross_wideselect_native_ffi_w (11.28 us) is 3.6x the fastest (3.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.6x the fastest

Fastest abi_native_cross_wideselect_null_entry (3.12 us) to slowest abi_native_cross_wideselect_native_ffi_w (11.28 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_wideselect_inproc_native's edge over baseline is significant but tiny (6 ns, 0.06%)

abi_native_cross_wideselect_inproc_native differs from baseline abi_native_cross_wideselect_native_ffi_w by 6 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 3118.5 ns median (-72.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.62x (fastest 3118.5 ns, slowest 11283.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13611ns | 13430ns | 13324ns | 13422ns | 14039ns | +0.21% |
| abi_native_cross_wideselect_native_ffi_w | 13582ns | 13471ns | 13382ns | 13452ns | 13878ns | base |
| abi_native_cross_wideselect_null_entry | 5385ns | 5363ns | 5281ns | 5348ns | 5491ns | -60.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11363ns | 11175ns | 11675ns | -0.03% | 0.023 |
| abi_native_cross_wideselect_native_ffi_w | 11366ns | 11178ns | 11612ns | base | 0.023 |
| abi_native_cross_wideselect_null_entry | 3138ns | 3080ns | 3202ns | -72.39% | 0.082 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5296.9 | 11423.7 | 11363.1 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 25280.5 | 11463.1 | 11365.9 | n/a |
| abi_native_cross_wideselect_null_entry | 28637.0 | 3157.2 | 3137.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.023 | 27.4% |
| abi_native_cross_wideselect_native_ffi_w | 0.023 | 27.3% |
| abi_native_cross_wideselect_null_entry | 0.082 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13611ns | 13611ns | +0.21% |
| abi_native_cross_wideselect_native_ffi_w | 13582ns | 13582ns | base |
| abi_native_cross_wideselect_null_entry | 5385ns | 5385ns | -60.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 11283ns | base | --- | [11202, 11612] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11238ns | no significant difference | [-406, +391]ns | [11176, 11675] | no | 1.0000 | 1.0000 | 0 |
| abi_native_cross_wideselect_null_entry | 3118ns | -8143.5ns (-72.2%) | [-8450, -8092]ns | [3092, 3202] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 11250ns | +3.4% | -72.6% |
| 2 | 11178ns | -0.0% | -72.2% |
| 3 | 11544ns | -2.7% | -72.2% |
| 4 | 11316ns | +3.6% | -71.7% |
| 5 | 11227ns | +0.1% | -72.2% |
| 6 | 11681ns | -4.3% | -73.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.309 | moderate- |
| abi_native_cross_wideselect_native_ffi_w | -0.285 | moderate- |
| abi_native_cross_wideselect_null_entry | 0.212 | moderate+ |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 2/6, lost 3/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 120287.4ns | 11363.1ns | 1058.6% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 140744.5ns | 11365.9ns | 1238.3% | HIGH |
| abi_native_cross_wideselect_null_entry | 121320.5ns | 3137.6ns | 3866.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11175.0-11674.5 ns)
  11175.0 |########################################
  11200.0 |
  11225.0 |########################################
  11249.9 |
  11274.9 |
  11299.9 |
  11324.9 |
  11349.8 |
  11374.8 |
  11399.8 |
  11424.8 |
  11449.8 |
  11474.7 |
  11499.7 |
  11524.7 |
  11549.7 |
  11574.6 |
  11599.6 |
  11624.6 |####################
  11649.6 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 11177.9-11612.3 ns)
  11177.9 |########################################
  11199.6 |
  11221.3 |########################################
  11243.1 |########################################
  11264.8 |
  11286.5 |
  11308.2 |########################################
  11329.9 |
  11351.7 |
  11373.4 |
  11395.1 |
  11416.8 |
  11438.5 |
  11460.3 |
  11482.0 |
  11503.7 |
  11525.4 |########################################
  11547.1 |
  11568.9 |
  11590.6 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 3080.0-3201.7 ns)
   3080.0 |########################################
   3086.1 |
   3092.2 |
   3098.3 |
   3104.3 |########################################
   3110.4 |########################################
   3116.5 |########################################
   3122.6 |
   3128.7 |
   3134.8 |
   3140.8 |
   3146.9 |
   3153.0 |
   3159.1 |
   3165.2 |
   3171.3 |
   3177.4 |
   3183.4 |
   3189.5 |
   3195.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1062.1% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1244.7% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=3895.7% of algo (FFI overhead may distort results)
