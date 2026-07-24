# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_native_cross_wideselect_null_entry dominates: 345% faster than the next best (abi_native_cross_wideselect_native_ffi_w)

abi_native_cross_wideselect_null_entry (2.66 us) leads abi_native_cross_wideselect_native_ffi_w (11.83 us) by 345%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 77% (significant)

abi_native_cross_wideselect_null_entry is -9.16 us (77%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_inproc_native is an outlier: 4.5x slower than the field

abi_native_cross_wideselect_inproc_native (11.86 us) is 4.5x the fastest (2.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.5x the fastest

Fastest abi_native_cross_wideselect_null_entry (2.66 us) to slowest abi_native_cross_wideselect_inproc_native (11.86 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_wideselect_inproc_native's edge over baseline is significant but tiny (25 ns, 0.21%)

abi_native_cross_wideselect_inproc_native differs from baseline abi_native_cross_wideselect_native_ffi_w by 25 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 2656.9 ns median (-77.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.46x (fastest 2656.9 ns, slowest 11855.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 14154ns | 14165ns | 13590ns | 14036ns | 14614ns | +0.72% |
| abi_native_cross_wideselect_native_ffi_w | 14053ns | 14095ns | 13708ns | 14081ns | 14185ns | base |
| abi_native_cross_wideselect_null_entry | 4893ns | 4857ns | 4804ns | 4852ns | 4999ns | -65.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11863ns | 11431ns | 12247ns | +0.65% | 0.011 |
| abi_native_cross_wideselect_native_ffi_w | 11786ns | 11450ns | 11901ns | base | 0.011 |
| abi_native_cross_wideselect_null_entry | 2677ns | 2628ns | 2738ns | -77.29% | 0.048 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5331.0 | 11572.3 | 11862.8 | 261 |
| abi_native_cross_wideselect_native_ffi_w | 25311.7 | 11873.0 | 11786.2 | n/a |
| abi_native_cross_wideselect_null_entry | 27807.8 | 2712.7 | 2677.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.011 | 22.2% |
| abi_native_cross_wideselect_native_ffi_w | 0.011 | 22.2% |
| abi_native_cross_wideselect_null_entry | 0.048 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 14154ns | 14154ns | +0.72% |
| abi_native_cross_wideselect_native_ffi_w | 14053ns | 14053ns | base |
| abi_native_cross_wideselect_null_entry | 4893ns | 4893ns | -65.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 11831ns | base | --- | [11628, 11901] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11855ns | no significant difference | [-414, +619]ns | [11486, 12247] | no | 1.0000 | 1.0000 | 0 |
| abi_native_cross_wideselect_null_entry | 2657ns | -9162.5ns (-77.4%) | [-9220, -8945]ns | [2637, 2738] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 11844ns | -1.3% | -77.8% |
| 2 | 11871ns | -2.8% | -77.7% |
| 3 | 11930ns | -4.2% | -76.8% |
| 4 | 11450ns | +5.4% | -76.8% |
| 5 | 11805ns | +5.3% | -77.0% |
| 6 | 11818ns | +1.7% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.436 | moderate+ |
| abi_native_cross_wideselect_native_ffi_w | -0.255 | moderate- |
| abi_native_cross_wideselect_null_entry | -0.338 | moderate- |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 3/6, lost 3/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 123191.0ns | 11862.8ns | 1038.5% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 144546.0ns | 11786.2ns | 1226.4% | HIGH |
| abi_native_cross_wideselect_null_entry | 119489.3ns | 2677.2ns | 4463.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11431.2-12246.8 ns)
  11431.2 |########################################
  11472.0 |
  11512.8 |########################################
  11553.5 |
  11594.3 |
  11635.1 |
  11675.9 |########################################
  11716.7 |
  11757.5 |
  11798.2 |
  11839.0 |
  11879.8 |
  11920.6 |
  11961.4 |
  12002.2 |########################################
  12042.9 |########################################
  12083.7 |
  12124.5 |
  12165.3 |
  12206.1 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 11450.0-11900.6 ns)
  11450.0 |########################################
  11472.5 |
  11495.1 |
  11517.6 |
  11540.1 |
  11562.6 |
  11585.2 |
  11607.7 |
  11630.2 |
  11652.8 |
  11675.3 |
  11697.8 |
  11720.4 |
  11742.9 |
  11765.4 |
  11787.9 |########################################
  11810.5 |########################################
  11833.0 |########################################
  11855.5 |########################################
  11878.1 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 2627.9-2737.5 ns)
   2627.9 |####################
   2633.4 |
   2638.9 |
   2644.3 |####################
   2649.8 |
   2655.3 |########################################
   2660.8 |
   2666.3 |
   2671.7 |
   2677.2 |
   2682.7 |
   2688.2 |
   2693.7 |
   2699.1 |
   2704.6 |####################
   2710.1 |
   2715.6 |
   2721.1 |
   2726.5 |
   2732.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1044.0% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1221.6% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=4489.0% of algo (FFI overhead may distort results)
