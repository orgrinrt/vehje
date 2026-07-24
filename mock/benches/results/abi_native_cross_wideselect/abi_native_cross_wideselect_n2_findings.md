# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (16.34 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 3.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 233% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (3.52 us) leads abi_native_cross_wideselect_inproc_native (11.71 us) by 233%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 79% (significant)

abi_native_cross_wideselect_null_entry is -12.85 us (79%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 4.6x slower than the field

abi_native_cross_wideselect_native_ffi_w (16.34 us) is 4.6x the fastest (3.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.6x the fastest

Fastest abi_native_cross_wideselect_null_entry (3.52 us) to slowest abi_native_cross_wideselect_native_ffi_w (16.34 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 3520.2 ns median (-78.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.64x (fastest 3520.2 ns, slowest 16335.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13853ns | 13958ns | 13420ns | 13824ns | 14113ns | -26.18% |
| abi_native_cross_wideselect_native_ffi_w | 18767ns | 18639ns | 18404ns | 18602ns | 19195ns | base |
| abi_native_cross_wideselect_null_entry | 5795ns | 5809ns | 5540ns | 5786ns | 5934ns | -69.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11598ns | 11247ns | 11788ns | -29.40% | 0.000 |
| abi_native_cross_wideselect_native_ffi_w | 16428ns | 16137ns | 16741ns | base | 0.000 |
| abi_native_cross_wideselect_null_entry | 3508ns | 3337ns | 3597ns | -78.65% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5353.0 | 11544.0 | 11597.8 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 26291.3 | 16927.2 | 16428.3 | n/a |
| abi_native_cross_wideselect_null_entry | 29020.8 | 3595.2 | 3508.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.000 | 28.5% |
| abi_native_cross_wideselect_native_ffi_w | 0.000 | 20.4% |
| abi_native_cross_wideselect_null_entry | 0.001 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13853ns | 13853ns | -26.18% |
| abi_native_cross_wideselect_native_ffi_w | 18767ns | 18767ns | base |
| abi_native_cross_wideselect_null_entry | 5795ns | 5795ns | -69.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 16336ns | base | --- | [16208, 16741] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11705ns | -4626.9ns (-28.3%) | [-5295, -4569]ns | [11300, 11788] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_wideselect_null_entry | 3520ns | -12849.6ns (-78.7%) | [-13259, -12652]ns | [3407, 3597] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 16280ns | -27.9% | -79.5% |
| 2 | 16429ns | -28.0% | -78.2% |
| 3 | 17053ns | -34.0% | -79.6% |
| 4 | 16327ns | -28.5% | -78.7% |
| 5 | 16137ns | -29.6% | -77.6% |
| 6 | 16345ns | -28.1% | -78.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.470 | moderate- |
| abi_native_cross_wideselect_native_ffi_w | -0.018 | ok |
| abi_native_cross_wideselect_null_entry | -0.202 | moderate- |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 123134.8ns | 11597.8ns | 1061.7% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 158317.7ns | 16428.3ns | 963.7% | HIGH |
| abi_native_cross_wideselect_null_entry | 121743.7ns | 3508.1ns | 3470.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11247.1-11788.5 ns)
  11247.1 |########################################
  11274.2 |
  11301.2 |
  11328.3 |########################################
  11355.4 |
  11382.5 |
  11409.5 |
  11436.6 |
  11463.7 |
  11490.7 |
  11517.8 |
  11544.9 |
  11571.9 |
  11599.0 |
  11626.1 |
  11653.1 |########################################
  11680.2 |
  11707.3 |########################################
  11734.4 |########################################
  11761.4 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 16136.7-16740.8 ns)
  16136.7 |####################
  16166.9 |
  16197.1 |
  16227.3 |
  16257.5 |####################
  16287.7 |
  16317.9 |########################################
  16348.2 |
  16378.4 |
  16408.6 |####################
  16438.8 |
  16469.0 |
  16499.2 |
  16529.4 |
  16559.6 |
  16589.8 |
  16620.0 |
  16650.2 |
  16680.4 |
  16710.6 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 3337.1-3596.9 ns)
   3337.1 |########################################
   3350.1 |
   3363.1 |
   3376.1 |
   3389.1 |
   3402.0 |
   3415.0 |
   3428.0 |
   3441.0 |
   3454.0 |
   3467.0 |########################################
   3480.0 |########################################
   3493.0 |
   3505.9 |
   3518.9 |
   3531.9 |
   3544.9 |
   3557.9 |########################################
   3570.9 |########################################
   3583.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1057.5% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=964.8% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=3443.4% of algo (FFI overhead may distort results)
