# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.89 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 326% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.73 us) leads abi_native_cross_real_inproc_native (11.62 us) by 326%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 77% (significant)

abi_native_cross_real_null_entry is -9.17 us (77%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_real_native_ffi_w (11.89 us) is 4.4x the fastest (2.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_real_null_entry (2.73 us) to slowest abi_native_cross_real_native_ffi_w (11.89 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2725.2 ns median (-77.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.36x (fastest 2725.2 ns, slowest 11893.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14652ns | 13911ns | 13495ns | 13783ns | 16534ns | +2.38% |
| abi_native_cross_real_native_ffi_w | 14312ns | 14180ns | 14108ns | 14162ns | 14638ns | base |
| abi_native_cross_real_null_entry | 4993ns | 4975ns | 4825ns | 4955ns | 5135ns | -65.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 12018ns | 11316ns | 13104ns | +0.00% | 0.011 |
| abi_native_cross_real_native_ffi_w | 12018ns | 11839ns | 12318ns | base | 0.011 |
| abi_native_cross_real_null_entry | 2727ns | 2628ns | 2793ns | -77.31% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5682.2 | 11859.6 | 12018.5 | 76726 |
| abi_native_cross_real_native_ffi_w | 27421.1 | 12019.5 | 12018.2 | n/a |
| abi_native_cross_real_null_entry | 27603.4 | 2753.6 | 2726.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.011 | 22.6% |
| abi_native_cross_real_native_ffi_w | 0.011 | 22.1% |
| abi_native_cross_real_null_entry | 0.047 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14652ns | 14652ns | +2.38% |
| abi_native_cross_real_native_ffi_w | 14312ns | 14312ns | base |
| abi_native_cross_real_null_entry | 4993ns | 4993ns | -65.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11893ns | base | --- | [11843, 12318] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11620ns | no significant difference | [-547, +799]ns | [11332, 13104] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_real_null_entry | 2725ns | -9169.6ns (-77.1%) | [-9611, -9094]ns | [2662, 2793] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11839ns | -0.3% | -77.8% |
| 2 | 11911ns | -5.0% | -77.4% |
| 3 | 11848ns | -4.2% | -76.9% |
| 4 | 11903ns | -3.9% | -76.2% |
| 5 | 12726ns | +12.8% | -78.6% |
| 6 | 11883ns | -0.3% | -76.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.109 | ok |
| abi_native_cross_real_native_ffi_w | -0.198 | ok |
| abi_native_cross_real_null_entry | 0.100 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 124882.9ns | 12018.5ns | 1039.1% | HIGH |
| abi_native_cross_real_native_ffi_w | 148091.3ns | 12018.2ns | 1232.2% | HIGH |
| abi_native_cross_real_null_entry | 119814.3ns | 2726.7ns | 4394.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11315.8-13103.5 ns)
  11315.8 |########################################
  11405.2 |####################
  11494.6 |
  11584.0 |
  11673.3 |
  11762.7 |########################################
  11852.1 |
  11941.5 |
  12030.9 |
  12120.3 |
  12209.7 |
  12299.1 |
  12388.4 |
  12477.8 |
  12567.2 |
  12656.6 |
  12746.0 |
  12835.4 |
  12924.8 |
  13014.2 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11838.8-12318.3 ns)
  11838.8 |########################################
  11862.8 |####################
  11886.8 |####################
  11910.7 |####################
  11934.7 |
  11958.7 |
  11982.6 |
  12006.6 |
  12030.6 |
  12054.6 |
  12078.5 |
  12102.5 |
  12126.5 |
  12150.5 |
  12174.4 |
  12198.4 |
  12222.4 |
  12246.4 |
  12270.3 |
  12294.3 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2627.5-2792.7 ns)
   2627.5 |########################################
   2635.8 |
   2644.0 |
   2652.3 |
   2660.5 |
   2668.8 |
   2677.1 |
   2685.3 |
   2693.6 |########################################
   2701.8 |
   2710.1 |########################################
   2718.4 |
   2726.6 |########################################
   2734.9 |
   2743.1 |
   2751.4 |########################################
   2759.7 |
   2767.9 |
   2776.2 |
   2784.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1054.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1230.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4398.0% of algo (FFI overhead may distort results)
