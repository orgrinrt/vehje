# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (16.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 3.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 247% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (3.42 us) leads abi_native_cross_tight_inproc_native (11.84 us) by 247%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 79% (significant)

abi_native_cross_tight_null_entry is -13.04 us (79%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_tight_native_ffi_w (16.51 us) is 4.8x the fastest (3.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_tight_null_entry (3.42 us) to slowest abi_native_cross_tight_native_ffi_w (16.51 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 3415.4 ns median (-79.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.83x (fastest 3415.4 ns, slowest 16513.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 14140ns | 14286ns | 13330ns | 14083ns | 14631ns | -26.87% |
| abi_native_cross_tight_native_ffi_w | 19337ns | 18827ns | 18542ns | 18763ns | 20594ns | base |
| abi_native_cross_tight_null_entry | 5682ns | 5672ns | 5461ns | 5616ns | 5890ns | -70.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11752ns | 11158ns | 12135ns | -31.01% | 0.000 |
| abi_native_cross_tight_native_ffi_w | 17034ns | 16260ns | 18290ns | base | 0.000 |
| abi_native_cross_tight_null_entry | 3439ns | 3315ns | 3574ns | -79.81% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5330.7 | 11798.1 | 11752.4 | n/a |
| abi_native_cross_tight_native_ffi_w | 26161.1 | 17307.8 | 17034.0 | n/a |
| abi_native_cross_tight_null_entry | 26764.3 | 3508.2 | 3438.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.000 | 28.0% |
| abi_native_cross_tight_native_ffi_w | 0.000 | 20.1% |
| abi_native_cross_tight_null_entry | 0.001 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 14140ns | 14140ns | -26.87% |
| abi_native_cross_tight_native_ffi_w | 19337ns | 19337ns | base |
| abi_native_cross_tight_null_entry | 5682ns | 5682ns | -70.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 16513ns | base | --- | [16299, 18290] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11835ns | -4883.1ns (-29.6%) | [-6797, -4165]ns | [11287, 12135] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_tight_null_entry | 3415ns | -13043.3ns (-79.0%) | [-14857, -12886]ns | [3327, 3574] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 16372ns | -30.3% | -79.6% |
| 2 | 16655ns | -28.9% | -78.4% |
| 3 | 16339ns | -25.8% | -78.6% |
| 4 | 19053ns | -37.9% | -81.4% |
| 5 | 16260ns | -25.3% | -79.5% |
| 6 | 17526ns | -36.3% | -81.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.230 | moderate- |
| abi_native_cross_tight_native_ffi_w | -0.473 | moderate- |
| abi_native_cross_tight_null_entry | -0.007 | ok |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 122214.4ns | 11752.4ns | 1039.9% | HIGH |
| abi_native_cross_tight_native_ffi_w | 158723.8ns | 17034.0ns | 931.8% | HIGH |
| abi_native_cross_tight_null_entry | 119472.4ns | 3438.7ns | 3474.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11158.3-12134.5 ns)
  11158.3 |########################################
  11207.1 |
  11255.9 |
  11304.7 |
  11353.5 |
  11402.4 |########################################
  11451.2 |
  11500.0 |
  11548.8 |
  11597.6 |
  11646.4 |
  11695.2 |
  11744.0 |
  11792.9 |########################################
  11841.7 |########################################
  11890.5 |
  11939.3 |
  11988.1 |
  12036.9 |
  12085.7 |########################################
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 16259.6-18289.6 ns)
  16259.6 |########################################
  16361.1 |####################
  16462.6 |
  16564.1 |####################
  16665.6 |
  16767.1 |
  16868.6 |
  16970.1 |
  17071.6 |
  17173.1 |
  17274.6 |
  17376.1 |
  17477.6 |####################
  17579.1 |
  17680.6 |
  17782.1 |
  17883.6 |
  17985.1 |
  18086.6 |
  18188.1 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 3315.4-3573.8 ns)
   3315.4 |########################################
   3328.3 |########################################
   3341.2 |########################################
   3354.2 |
   3367.1 |
   3380.0 |
   3392.9 |
   3405.8 |
   3418.7 |
   3431.7 |
   3444.6 |
   3457.5 |
   3470.4 |
   3483.3 |########################################
   3496.2 |
   3509.2 |
   3522.1 |
   3535.0 |
   3547.9 |########################################
   3560.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1038.9% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=960.8% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=3489.0% of algo (FFI overhead may distort results)
