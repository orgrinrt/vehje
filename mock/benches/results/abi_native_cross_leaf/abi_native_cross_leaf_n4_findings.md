# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (14.08 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 4.04 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 178% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (4.04 us) leads abi_native_cross_leaf_inproc_native (11.22 us) by 178%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 71% (significant)

abi_native_cross_leaf_null_entry is -10.01 us (71%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 3.5x slower than the field

abi_native_cross_leaf_native_ffi_w (14.08 us) is 3.5x the fastest (4.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.5x the fastest

Fastest abi_native_cross_leaf_null_entry (4.04 us) to slowest abi_native_cross_leaf_native_ffi_w (14.08 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 4042.1 ns median (-71.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.48x (fastest 4042.1 ns, slowest 14082.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13616ns | 13507ns | 13436ns | 13493ns | 13889ns | -16.16% |
| abi_native_cross_leaf_native_ffi_w | 16240ns | 16355ns | 15665ns | 16299ns | 16439ns | base |
| abi_native_cross_leaf_null_entry | 6324ns | 6284ns | 5975ns | 6224ns | 6650ns | -61.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11297ns | 11170ns | 11483ns | -19.28% | 0.000 |
| abi_native_cross_leaf_native_ffi_w | 13996ns | 13480ns | 14211ns | base | 0.000 |
| abi_native_cross_leaf_null_entry | 4050ns | 3814ns | 4249ns | -71.07% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5508.5 | 11334.4 | 11297.2 | n/a |
| abi_native_cross_leaf_native_ffi_w | 24700.2 | 14060.0 | 13996.4 | n/a |
| abi_native_cross_leaf_null_entry | 27178.5 | 4226.7 | 4049.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.000 | 34.0% |
| abi_native_cross_leaf_native_ffi_w | 0.000 | 27.1% |
| abi_native_cross_leaf_null_entry | 0.001 | 94.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13616ns | 13616ns | -16.16% |
| abi_native_cross_leaf_native_ffi_w | 16240ns | 16240ns | base |
| abi_native_cross_leaf_null_entry | 6324ns | 6324ns | -61.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 14083ns | base | --- | [13696, 14211] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11221ns | -2788.3ns (-19.8%) | [-2879, -2430]ns | [11188, 11483] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_leaf_null_entry | 4042ns | -10005.8ns (-71.1%) | [-10353, -9481]ns | [3858, 4249] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 14325ns | -18.3% | -72.8% |
| 2 | 13480ns | -16.6% | -68.4% |
| 3 | 13911ns | -19.7% | -70.1% |
| 4 | 14096ns | -20.5% | -72.9% |
| 5 | 14073ns | -20.4% | -72.2% |
| 6 | 14092ns | -20.1% | -70.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.030 | ok |
| abi_native_cross_leaf_native_ffi_w | -0.293 | moderate- |
| abi_native_cross_leaf_null_entry | -0.146 | ok |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 119947.5ns | 11297.2ns | 1061.7% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 145491.6ns | 13996.4ns | 1039.5% | HIGH |
| abi_native_cross_leaf_null_entry | 122315.3ns | 4049.7ns | 3020.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11169.6-11482.7 ns)
  11169.6 |####################
  11185.3 |
  11200.9 |########################################
  11216.6 |
  11232.2 |####################
  11247.9 |####################
  11263.5 |
  11279.2 |
  11294.8 |
  11310.5 |
  11326.2 |
  11341.8 |
  11357.5 |
  11373.1 |
  11388.8 |
  11404.4 |
  11420.1 |
  11435.7 |
  11451.4 |
  11467.0 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 13480.4-14210.6 ns)
  13480.4 |#############
  13516.9 |
  13553.4 |
  13589.9 |
  13626.4 |
  13663.0 |
  13699.5 |
  13736.0 |
  13772.5 |
  13809.0 |
  13845.5 |
  13882.0 |#############
  13918.5 |
  13955.0 |
  13991.5 |
  14028.0 |
  14064.6 |########################################
  14101.1 |
  14137.6 |
  14174.1 |
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 3813.8-4249.2 ns)
   3813.8 |####################
   3835.6 |
   3857.3 |
   3879.1 |
   3900.9 |########################################
   3922.7 |
   3944.4 |
   3966.2 |
   3988.0 |
   4009.7 |
   4031.5 |
   4053.3 |
   4075.0 |
   4096.8 |
   4118.6 |
   4140.4 |
   4162.1 |####################
   4183.9 |
   4205.7 |
   4227.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1062.1% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1025.7% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=3023.5% of algo (FFI overhead may distort results)
