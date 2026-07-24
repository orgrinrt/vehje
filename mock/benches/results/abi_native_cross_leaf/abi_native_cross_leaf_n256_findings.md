# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (11.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 3.12 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 264% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (3.12 us) leads abi_native_cross_leaf_inproc_native (11.36 us) by 264%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 73% (significant)

abi_native_cross_leaf_null_entry is -8.50 us (73%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 3.7x slower than the field

abi_native_cross_leaf_native_ffi_w (11.61 us) is 3.7x the fastest (3.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_leaf_null_entry (3.12 us) to slowest abi_native_cross_leaf_native_ffi_w (11.61 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 3122.5 ns median (-73.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.72x (fastest 3122.5 ns, slowest 11608.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13718ns | 13651ns | 13552ns | 13626ns | 13940ns | -0.80% |
| abi_native_cross_leaf_native_ffi_w | 13828ns | 13889ns | 13480ns | 13870ns | 13941ns | base |
| abi_native_cross_leaf_null_entry | 5354ns | 5314ns | 5254ns | 5309ns | 5470ns | -61.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11423ns | 11235ns | 11664ns | -1.28% | 0.022 |
| abi_native_cross_leaf_native_ffi_w | 11571ns | 11303ns | 11662ns | base | 0.022 |
| abi_native_cross_leaf_null_entry | 3133ns | 3086ns | 3180ns | -72.92% | 0.082 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5428.7 | 11447.6 | 11422.7 | n/a |
| abi_native_cross_leaf_native_ffi_w | 24873.0 | 11713.8 | 11570.9 | n/a |
| abi_native_cross_leaf_null_entry | 26921.0 | 3141.9 | 3133.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.023 | 27.2% |
| abi_native_cross_leaf_native_ffi_w | 0.022 | 26.6% |
| abi_native_cross_leaf_null_entry | 0.082 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13718ns | 13718ns | -0.80% |
| abi_native_cross_leaf_native_ffi_w | 13828ns | 13828ns | base |
| abi_native_cross_leaf_null_entry | 5354ns | 5354ns | -61.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 11608ns | base | --- | [11442, 11662] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11355ns | no significant difference | [-378, +180]ns | [11248, 11664] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_leaf_null_entry | 3122ns | -8499.4ns (-73.2%) | [-8539, -8275]ns | [3097, 3180] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 11303ns | +4.6% | -72.5% |
| 2 | 11624ns | -2.4% | -73.5% |
| 3 | 11582ns | -1.8% | -72.1% |
| 4 | 11665ns | -1.4% | -73.2% |
| 5 | 11660ns | -3.4% | -73.1% |
| 6 | 11592ns | -3.1% | -73.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.065 | ok |
| abi_native_cross_leaf_native_ffi_w | -0.025 | ok |
| abi_native_cross_leaf_null_entry | -0.336 | moderate- |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 120922.2ns | 11422.7ns | 1058.6% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 142376.0ns | 11570.9ns | 1230.5% | HIGH |
| abi_native_cross_leaf_null_entry | 119792.2ns | 3133.2ns | 3823.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11235.4-11664.3 ns)
  11235.4 |########################################
  11256.8 |########################################
  11278.3 |
  11299.7 |
  11321.2 |########################################
  11342.6 |
  11364.1 |########################################
  11385.5 |
  11407.0 |
  11428.4 |
  11449.9 |
  11471.3 |
  11492.8 |########################################
  11514.2 |
  11535.7 |
  11557.1 |
  11578.6 |
  11600.0 |
  11621.5 |
  11642.9 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 11302.9-11662.3 ns)
  11302.9 |########################################
  11320.9 |
  11338.8 |
  11356.8 |
  11374.8 |
  11392.8 |
  11410.7 |
  11428.7 |
  11446.7 |
  11464.6 |
  11482.6 |
  11500.6 |
  11518.5 |
  11536.5 |
  11554.5 |
  11572.4 |########################################
  11590.4 |########################################
  11608.4 |########################################
  11626.4 |
  11644.3 |########################################
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 3085.8-3180.2 ns)
   3085.8 |########################################
   3090.5 |
   3095.2 |
   3100.0 |
   3104.7 |########################################
   3109.4 |
   3114.1 |
   3118.8 |########################################
   3123.6 |########################################
   3128.3 |
   3133.0 |########################################
   3137.7 |
   3142.4 |
   3147.2 |
   3151.9 |
   3156.6 |
   3161.3 |
   3166.0 |
   3170.8 |
   3175.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1056.7% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1232.0% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=3835.9% of algo (FFI overhead may distort results)
