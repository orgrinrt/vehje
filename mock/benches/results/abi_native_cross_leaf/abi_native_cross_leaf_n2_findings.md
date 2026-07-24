# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (17.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 3.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 244% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (3.40 us) leads abi_native_cross_leaf_inproc_native (11.68 us) by 244%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 80% (significant)

abi_native_cross_leaf_null_entry is -14.42 us (80%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 5.3x slower than the field

abi_native_cross_leaf_native_ffi_w (17.97 us) is 5.3x the fastest (3.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_leaf_null_entry shows alternating (throttle bounce) (autocorr -0.61)

abi_native_cross_leaf_null_entry's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.3x the fastest

Fastest abi_native_cross_leaf_null_entry (3.40 us) to slowest abi_native_cross_leaf_native_ffi_w (17.97 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 3400.8 ns median (-81.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.28x (fastest 3400.8 ns, slowest 17965.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 14038ns | 13995ns | 13391ns | 13966ns | 14470ns | -29.55% |
| abi_native_cross_leaf_native_ffi_w | 19925ns | 20223ns | 17956ns | 19660ns | 21308ns | base |
| abi_native_cross_leaf_null_entry | 5766ns | 5678ns | 5511ns | 5667ns | 6043ns | -71.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11707ns | 11178ns | 12066ns | -33.78% | 0.000 |
| abi_native_cross_leaf_native_ffi_w | 17678ns | 15768ns | 19047ns | base | 0.000 |
| abi_native_cross_leaf_null_entry | 3471ns | 3340ns | 3657ns | -80.37% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5511.0 | 11726.2 | 11707.3 | n/a |
| abi_native_cross_leaf_native_ffi_w | 25129.4 | 17755.8 | 17678.2 | n/a |
| abi_native_cross_leaf_null_entry | 27881.0 | 3573.0 | 3470.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.000 | 28.6% |
| abi_native_cross_leaf_native_ffi_w | 0.000 | 18.6% |
| abi_native_cross_leaf_null_entry | 0.001 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 14038ns | 14038ns | -29.55% |
| abi_native_cross_leaf_native_ffi_w | 19925ns | 19925ns | base |
| abi_native_cross_leaf_null_entry | 5766ns | 5766ns | -71.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 17966ns | base | --- | [16022, 19047] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11683ns | -6371.1ns (-35.5%) | [-7304, -4238]ns | [11373, 12066] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_leaf_null_entry | 3401ns | -14424.8ns (-80.3%) | [-15561, -12637]ns | [3354, 3657] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 16277ns | -28.2% | -79.5% |
| 2 | 19129ns | -36.0% | -80.9% |
| 3 | 18965ns | -39.0% | -82.2% |
| 4 | 15768ns | -24.6% | -78.2% |
| 5 | 17035ns | -34.4% | -78.6% |
| 6 | 18896ns | -38.2% | -82.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.319 | moderate- |
| abi_native_cross_leaf_native_ffi_w | -0.193 | ok |
| abi_native_cross_leaf_null_entry | -0.607 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 122079.9ns | 11707.3ns | 1042.8% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 157393.1ns | 17678.2ns | 890.3% | HIGH |
| abi_native_cross_leaf_null_entry | 120963.4ns | 3470.5ns | 3485.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11178.3-12065.6 ns)
  11178.3 |####################
  11222.7 |
  11267.0 |
  11311.4 |
  11355.8 |
  11400.1 |
  11444.5 |
  11488.9 |
  11533.2 |####################
  11577.6 |
  11621.9 |
  11666.3 |########################################
  11710.7 |
  11755.0 |
  11799.4 |
  11843.8 |####################
  11888.1 |
  11932.5 |
  11976.9 |
  12021.2 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 15767.5-19046.8 ns)
  15767.5 |####################
  15931.5 |
  16095.4 |
  16259.4 |####################
  16423.4 |
  16587.3 |
  16751.3 |
  16915.3 |####################
  17079.2 |
  17243.2 |
  17407.2 |
  17571.1 |
  17735.1 |
  17899.1 |
  18063.0 |
  18227.0 |
  18391.0 |
  18554.9 |
  18718.9 |
  18882.9 |########################################
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 3339.6-3657.1 ns)
   3339.6 |####################
   3355.5 |########################################
   3371.3 |
   3387.2 |
   3403.1 |
   3419.0 |####################
   3434.8 |
   3450.7 |
   3466.6 |
   3482.5 |
   3498.3 |
   3514.2 |
   3530.1 |
   3545.9 |
   3561.8 |
   3577.7 |
   3593.6 |
   3609.4 |
   3625.3 |
   3641.2 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1047.6% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=879.1% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=3565.2% of algo (FFI overhead may distort results)
