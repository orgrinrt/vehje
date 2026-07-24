# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (12.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 379% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (2.49 us) leads abi_native_cross_leaf_inproc_native (11.91 us) by 379%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 79% (significant)

abi_native_cross_leaf_null_entry is -9.64 us (79%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_leaf_native_ffi_w (12.21 us) is 4.9x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_leaf_null_entry (2.49 us) to slowest abi_native_cross_leaf_native_ffi_w (12.21 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 2487.2 ns median (-79.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.91x (fastest 2487.2 ns, slowest 12207.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 14197ns | 14261ns | 13451ns | 14136ns | 14660ns | -2.88% |
| abi_native_cross_leaf_native_ffi_w | 14618ns | 14475ns | 14000ns | 14323ns | 15369ns | base |
| abi_native_cross_leaf_null_entry | 4747ns | 4702ns | 4622ns | 4689ns | 4896ns | -67.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11867ns | 11300ns | 12242ns | -3.37% | 0.001 |
| abi_native_cross_leaf_native_ffi_w | 12281ns | 11765ns | 12835ns | base | 0.001 |
| abi_native_cross_leaf_null_entry | 2506ns | 2462ns | 2564ns | -79.59% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5510.0 | 11909.6 | 11867.3 | n/a |
| abi_native_cross_leaf_native_ffi_w | 25354.2 | 12432.8 | 12281.0 | n/a |
| abi_native_cross_leaf_null_entry | 27129.6 | 2607.0 | 2506.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.001 | 20.7% |
| abi_native_cross_leaf_native_ffi_w | 0.001 | 20.2% |
| abi_native_cross_leaf_null_entry | 0.006 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 14197ns | 14197ns | -2.88% |
| abi_native_cross_leaf_native_ffi_w | 14618ns | 14618ns | base |
| abi_native_cross_leaf_null_entry | 4747ns | 4747ns | -67.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 12207ns | base | --- | [11801, 12835] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11907ns | no significant difference | [-1221, +188]ns | [11452, 12242] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_leaf_null_entry | 2487ns | -9643.6ns (-79.0%) | [-10362, -9319]ns | [2467, 2564] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 12361ns | -3.5% | -80.1% |
| 2 | 13310ns | -15.1% | -81.3% |
| 3 | 11836ns | +0.7% | -79.0% |
| 4 | 12158ns | -2.1% | -78.4% |
| 5 | 12257ns | +2.4% | -79.6% |
| 6 | 11765ns | -1.4% | -79.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.249 | moderate- |
| abi_native_cross_leaf_native_ffi_w | -0.198 | ok |
| abi_native_cross_leaf_null_entry | -0.072 | ok |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 122685.0ns | 11867.3ns | 1033.8% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 143061.8ns | 12281.0ns | 1164.9% | HIGH |
| abi_native_cross_leaf_null_entry | 117647.0ns | 2506.0ns | 4694.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11299.6-12242.5 ns)
  11299.6 |####################
  11346.7 |
  11393.9 |
  11441.0 |
  11488.2 |
  11535.3 |
  11582.5 |####################
  11629.6 |
  11676.8 |
  11723.9 |
  11771.0 |
  11818.2 |
  11865.3 |####################
  11912.5 |########################################
  11959.6 |
  12006.8 |
  12053.9 |
  12101.1 |
  12148.2 |
  12195.4 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 11765.4-12835.2 ns)
  11765.4 |########################################
  11818.9 |########################################
  11872.4 |
  11925.9 |
  11979.4 |
  12032.9 |
  12086.3 |
  12139.8 |########################################
  12193.3 |
  12246.8 |########################################
  12300.3 |
  12353.8 |########################################
  12407.3 |
  12460.8 |
  12514.3 |
  12567.8 |
  12621.2 |
  12674.7 |
  12728.2 |
  12781.7 |
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 2462.1-2563.7 ns)
   2462.1 |########################################
   2467.2 |########################################
   2472.3 |
   2477.3 |
   2482.4 |########################################
   2487.5 |########################################
   2492.6 |########################################
   2497.7 |
   2502.7 |
   2507.8 |
   2512.9 |
   2518.0 |
   2523.1 |
   2528.1 |
   2533.2 |
   2538.3 |
   2543.4 |
   2548.5 |
   2553.5 |
   2558.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1031.8% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1171.1% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=4712.0% of algo (FFI overhead may distort results)
