# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (13.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 4.03 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 187% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (4.03 us) leads abi_native_cross_real_inproc_native (11.56 us) by 187%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 72% (significant)

abi_native_cross_real_null_entry is -9.90 us (72%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 3.4x slower than the field

abi_native_cross_real_native_ffi_w (13.85 us) is 3.4x the fastest (4.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.4x the fastest

Fastest abi_native_cross_real_null_entry (4.03 us) to slowest abi_native_cross_real_native_ffi_w (13.85 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 4025.4 ns median (-70.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.44x (fastest 4025.4 ns, slowest 13847.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14069ns | 13901ns | 13483ns | 13847ns | 14695ns | -16.17% |
| abi_native_cross_real_native_ffi_w | 16784ns | 16072ns | 15718ns | 15957ns | 18556ns | base |
| abi_native_cross_real_null_entry | 6355ns | 6288ns | 6086ns | 6237ns | 6665ns | -62.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11726ns | 11301ns | 12248ns | -18.76% | 0.000 |
| abi_native_cross_real_native_ffi_w | 14434ns | 13466ns | 15962ns | base | 0.000 |
| abi_native_cross_real_null_entry | 4043ns | 3882ns | 4187ns | -71.99% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5842.8 | 11679.8 | 11726.0 | n/a |
| abi_native_cross_real_native_ffi_w | 30535.4 | 14460.9 | 14433.6 | n/a |
| abi_native_cross_real_null_entry | 29511.2 | 4179.6 | 4042.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 33.6% |
| abi_native_cross_real_native_ffi_w | 0.000 | 28.0% |
| abi_native_cross_real_null_entry | 0.001 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14069ns | 14069ns | -16.17% |
| abi_native_cross_real_native_ffi_w | 16784ns | 16784ns | base |
| abi_native_cross_real_null_entry | 6355ns | 6355ns | -62.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 13848ns | base | --- | [13491, 15962] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11556ns | -2368.1ns (-17.1%) | [-3869, -1886]ns | [11375, 12248] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 4025ns | -9905.0ns (-71.5%) | [-11775, -9493]ns | [3915, 4187] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 17284ns | -26.3% | -75.2% |
| 2 | 14640ns | -21.8% | -72.0% |
| 3 | 13763ns | -15.3% | -70.9% |
| 4 | 13516ns | -15.3% | -70.1% |
| 5 | 13932ns | -18.9% | -72.1% |
| 6 | 13466ns | -12.7% | -70.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.104 | ok |
| abi_native_cross_real_native_ffi_w | 0.189 | ok |
| abi_native_cross_real_null_entry | 0.253 | moderate+ |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 123261.4ns | 11726.0ns | 1051.2% | HIGH |
| abi_native_cross_real_native_ffi_w | 154845.7ns | 14433.6ns | 1072.8% | HIGH |
| abi_native_cross_real_null_entry | 124370.3ns | 4042.6ns | 3076.5% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11301.2-12247.7 ns)
  11301.2 |####################
  11348.5 |
  11395.9 |
  11443.2 |########################################
  11490.5 |
  11537.8 |
  11585.2 |
  11632.5 |####################
  11679.8 |
  11727.1 |####################
  11774.5 |
  11821.8 |
  11869.1 |
  11916.4 |
  11963.8 |
  12011.1 |
  12058.4 |
  12105.7 |
  12153.1 |
  12200.4 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 13466.2-15962.1 ns)
  13466.2 |########################################
  13591.0 |
  13715.8 |####################
  13840.6 |####################
  13965.4 |
  14090.2 |
  14215.0 |
  14339.8 |
  14464.6 |
  14589.4 |####################
  14714.1 |
  14838.9 |
  14963.7 |
  15088.5 |
  15213.3 |
  15338.1 |
  15462.9 |
  15587.7 |
  15712.5 |
  15837.3 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3882.1-4187.1 ns)
   3882.1 |########################################
   3897.3 |
   3912.6 |
   3927.8 |
   3943.1 |########################################
   3958.3 |
   3973.6 |
   3988.8 |########################################
   4004.1 |
   4019.3 |
   4034.6 |########################################
   4049.8 |
   4065.1 |
   4080.3 |########################################
   4095.6 |
   4110.8 |
   4126.1 |
   4141.3 |
   4156.6 |
   4171.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1055.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1085.2% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3053.1% of algo (FFI overhead may distort results)
