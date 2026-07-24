# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (35.02 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 4.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 134% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (4.98 us) leads abi_native_cross_real_inproc_native (11.67 us) by 134%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 86% (significant)

abi_native_cross_real_null_entry is -30.10 us (86%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 7.0x slower than the field

abi_native_cross_real_native_ffi_w (35.02 us) is 7.0x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.0x the fastest

Fastest abi_native_cross_real_null_entry (4.98 us) to slowest abi_native_cross_real_native_ffi_w (35.02 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_real_native_ffi_w is inconsistent: worst-20% is 1.5x its best-20%

abi_native_cross_real_native_ffi_w's best 20% of batches run at 23.70 us but its worst 20% at 35.67 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 4977.5 ns median (-85.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.04x (fastest 4977.5 ns, slowest 35020.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13890ns | 13894ns | 13529ns | 13850ns | 14130ns | -60.11% |
| abi_native_cross_real_native_ffi_w | 34815ns | 37276ns | 25948ns | 35693ns | 37932ns | base |
| abi_native_cross_real_null_entry | 7214ns | 7262ns | 6967ns | 7177ns | 7392ns | -79.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11619ns | 11236ns | 11859ns | -64.32% | 0.000 |
| abi_native_cross_real_native_ffi_w | 32563ns | 23702ns | 35672ns | base | 0.000 |
| abi_native_cross_real_null_entry | 4931ns | 4769ns | 5042ns | -84.86% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5314.7 | 11569.2 | 11619.1 | n/a |
| abi_native_cross_real_native_ffi_w | 26187.2 | 31590.6 | 32562.7 | n/a |
| abi_native_cross_real_null_entry | 28309.9 | 5053.8 | 4931.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 40.9% |
| abi_native_cross_real_native_ffi_w | 0.000 | 13.6% |
| abi_native_cross_real_null_entry | 0.000 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13890ns | 13890ns | -60.11% |
| abi_native_cross_real_native_ffi_w | 34815ns | 34815ns | base |
| abi_native_cross_real_null_entry | 7214ns | 7214ns | -79.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 35020ns | base | --- | [26995, 35672] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11672ns | -23337.7ns (-66.6%) | [-23904, -15589]ns | [11326, 11859] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 4978ns | -30095.6ns (-85.9%) | [-30684, -22115]ns | [4773, 5042] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 35082ns | -67.5% | -86.4% |
| 2 | 34958ns | -65.8% | -85.5% |
| 3 | 30289ns | -61.8% | -83.5% |
| 4 | 23702ns | -52.6% | -79.9% |
| 5 | 35662ns | -67.0% | -85.9% |
| 6 | 35682ns | -67.0% | -86.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.291 | moderate- |
| abi_native_cross_real_native_ffi_w | 0.026 | ok |
| abi_native_cross_real_null_entry | -0.412 | moderate- |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 122645.8ns | 11619.1ns | 1055.6% | HIGH |
| abi_native_cross_real_native_ffi_w | 197120.1ns | 32562.7ns | 605.4% | HIGH |
| abi_native_cross_real_null_entry | 124344.7ns | 4931.0ns | 2521.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11235.8-11859.2 ns)
  11235.8 |####################
  11267.0 |
  11298.1 |
  11329.3 |
  11360.5 |
  11391.6 |####################
  11422.8 |
  11454.0 |
  11485.2 |
  11516.3 |
  11547.5 |####################
  11578.7 |
  11609.8 |
  11641.0 |
  11672.2 |
  11703.4 |
  11734.5 |
  11765.7 |########################################
  11796.9 |
  11828.0 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 23701.7-35672.1 ns)
  23701.7 |####################
  24300.2 |
  24898.7 |
  25497.3 |
  26095.8 |
  26694.3 |
  27292.8 |
  27891.3 |
  28489.9 |
  29088.4 |
  29686.9 |
  30285.4 |####################
  30883.9 |
  31482.5 |
  32081.0 |
  32679.5 |
  33278.0 |
  33876.5 |
  34475.1 |####################
  35073.6 |########################################
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 4769.2-5042.5 ns)
   4769.2 |########################################
   4782.9 |
   4796.5 |
   4810.2 |
   4823.9 |
   4837.5 |
   4851.2 |
   4864.9 |
   4878.5 |
   4892.2 |
   4905.9 |
   4919.5 |
   4933.2 |
   4946.8 |
   4960.5 |####################
   4974.2 |
   4987.8 |####################
   5001.5 |####################
   5015.2 |
   5028.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1053.7% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=577.7% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=2487.2% of algo (FFI overhead may distort results)
