# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 335% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.66 us) leads abi_native_cross_real_inproc_native (11.56 us) by 335%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 78% (significant)

abi_native_cross_real_null_entry is -9.27 us (78%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.5x slower than the field

abi_native_cross_real_native_ffi_w (11.90 us) is 4.5x the fastest (2.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.5x the fastest

Fastest abi_native_cross_real_null_entry (2.66 us) to slowest abi_native_cross_real_native_ffi_w (11.90 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2658.6 ns median (-77.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.48x (fastest 2658.6 ns, slowest 11898.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13904ns | 13828ns | 13473ns | 13759ns | 14336ns | -1.98% |
| abi_native_cross_real_native_ffi_w | 14185ns | 14228ns | 13691ns | 14163ns | 14466ns | base |
| abi_native_cross_real_null_entry | 4892ns | 4836ns | 4788ns | 4827ns | 5040ns | -65.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11609ns | 11292ns | 11954ns | -2.37% | 0.011 |
| abi_native_cross_real_native_ffi_w | 11891ns | 11477ns | 12192ns | base | 0.011 |
| abi_native_cross_real_null_entry | 2680ns | 2627ns | 2754ns | -77.46% | 0.048 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5209.2 | 11560.5 | 11609.3 | n/a |
| abi_native_cross_real_native_ffi_w | 25983.0 | 11965.0 | 11890.9 | n/a |
| abi_native_cross_real_null_entry | 26495.8 | 2709.2 | 2680.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.011 | 22.7% |
| abi_native_cross_real_native_ffi_w | 0.011 | 22.1% |
| abi_native_cross_real_null_entry | 0.048 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13904ns | 13904ns | -1.98% |
| abi_native_cross_real_native_ffi_w | 14185ns | 14185ns | base |
| abi_native_cross_real_null_entry | 4892ns | 4892ns | -65.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11898ns | base | --- | [11582, 12192] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11563ns | no significant difference | [-644, +130]ns | [11311, 11954] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_real_null_entry | 2659ns | -9270.2ns (-77.9%) | [-9493, -8869]ns | [2628, 2754] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11925ns | -4.6% | -78.0% |
| 2 | 11477ns | -1.3% | -76.9% |
| 3 | 11687ns | +3.3% | -76.3% |
| 4 | 12357ns | -4.2% | -77.8% |
| 5 | 12028ns | -6.1% | -77.9% |
| 6 | 11872ns | -1.0% | -77.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.144 | ok |
| abi_native_cross_real_native_ffi_w | 0.081 | ok |
| abi_native_cross_real_null_entry | 0.239 | moderate+ |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 121699.6ns | 11609.3ns | 1048.3% | HIGH |
| abi_native_cross_real_native_ffi_w | 144176.2ns | 11890.9ns | 1212.5% | HIGH |
| abi_native_cross_real_null_entry | 118900.8ns | 2680.1ns | 4436.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11292.1-11953.8 ns)
  11292.1 |########################################
  11325.2 |########################################
  11358.3 |########################################
  11391.3 |
  11424.4 |
  11457.5 |
  11490.6 |
  11523.7 |
  11556.8 |
  11589.8 |
  11622.9 |
  11656.0 |
  11689.1 |
  11722.2 |########################################
  11755.3 |
  11788.3 |
  11821.4 |########################################
  11854.5 |
  11887.6 |
  11920.7 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11477.1-12192.5 ns)
  11477.1 |########################################
  11512.9 |
  11548.6 |
  11584.4 |
  11620.2 |
  11656.0 |########################################
  11691.7 |
  11727.5 |
  11763.3 |
  11799.0 |
  11834.8 |
  11870.6 |########################################
  11906.3 |########################################
  11942.1 |
  11977.9 |
  12013.6 |########################################
  12049.4 |
  12085.2 |
  12121.0 |
  12156.7 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2627.1-2753.8 ns)
   2627.1 |########################################
   2633.4 |
   2639.8 |
   2646.1 |
   2652.4 |####################
   2658.8 |####################
   2665.1 |
   2671.4 |
   2677.8 |
   2684.1 |
   2690.4 |
   2696.8 |
   2703.1 |
   2709.4 |
   2715.8 |
   2722.1 |
   2728.4 |
   2734.8 |####################
   2741.1 |
   2747.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1053.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1214.5% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4466.3% of algo (FFI overhead may distort results)
