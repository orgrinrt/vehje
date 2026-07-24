# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (24.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 4.99 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 140% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (4.99 us) leads abi_native_cross_real_inproc_native (11.97 us) by 140%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -19.01 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_real_native_ffi_w (24.03 us) is 4.8x the fastest (4.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_real_null_entry (4.99 us) to slowest abi_native_cross_real_native_ffi_w (24.03 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 4990.4 ns median (-79.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.81x (fastest 4990.4 ns, slowest 24026.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14285ns | 14328ns | 13685ns | 14174ns | 14751ns | -48.00% |
| abi_native_cross_real_native_ffi_w | 27472ns | 26369ns | 25369ns | 26166ns | 30481ns | base |
| abi_native_cross_real_null_entry | 7264ns | 7297ns | 6982ns | 7276ns | 7386ns | -73.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11941ns | 11391ns | 12333ns | -52.58% | 0.000 |
| abi_native_cross_real_native_ffi_w | 25179ns | 23186ns | 28183ns | base | 0.000 |
| abi_native_cross_real_null_entry | 4959ns | 4742ns | 5031ns | -80.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5399.0 | 11907.5 | 11941.2 | n/a |
| abi_native_cross_real_native_ffi_w | 26571.3 | 26573.3 | 25179.4 | n/a |
| abi_native_cross_real_null_entry | 27749.4 | 5018.6 | 4959.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 39.6% |
| abi_native_cross_real_native_ffi_w | 0.000 | 19.7% |
| abi_native_cross_real_null_entry | 0.000 | 95.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14285ns | 14285ns | -48.00% |
| abi_native_cross_real_native_ffi_w | 27472ns | 27472ns | base |
| abi_native_cross_real_null_entry | 7264ns | 7264ns | -73.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 24027ns | base | --- | [23329, 28183] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11967ns | -11851.7ns (-49.3%) | [-16369, -11494]ns | [11524, 12333] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 4990ns | -19012.8ns (-79.1%) | [-23185, -18464]ns | [4856, 5031] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 23186ns | -50.9% | -79.5% |
| 2 | 23932ns | -48.0% | -78.9% |
| 3 | 24122ns | -49.4% | -79.4% |
| 4 | 26545ns | -54.9% | -81.2% |
| 5 | 23472ns | -49.0% | -78.8% |
| 6 | 29821ns | -60.9% | -83.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.194 | ok |
| abi_native_cross_real_native_ffi_w | -0.239 | moderate- |
| abi_native_cross_real_null_entry | -0.293 | moderate- |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 123761.2ns | 11941.2ns | 1036.4% | HIGH |
| abi_native_cross_real_native_ffi_w | 178947.4ns | 25179.4ns | 710.7% | HIGH |
| abi_native_cross_real_null_entry | 124911.8ns | 4959.0ns | 2518.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11390.8-12332.7 ns)
  11390.8 |####################
  11437.9 |
  11485.0 |
  11532.1 |
  11579.2 |
  11626.3 |####################
  11673.4 |
  11720.5 |
  11767.6 |
  11814.7 |
  11861.8 |
  11908.8 |
  11955.9 |########################################
  12003.0 |
  12050.1 |
  12097.2 |
  12144.3 |
  12191.4 |####################
  12238.5 |
  12285.6 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 23185.8-28182.7 ns)
  23185.8 |########################################
  23435.6 |########################################
  23685.5 |########################################
  23935.3 |########################################
  24185.2 |
  24435.0 |
  24684.9 |
  24934.7 |
  25184.6 |
  25434.4 |
  25684.2 |
  25934.1 |
  26183.9 |
  26433.8 |########################################
  26683.6 |
  26933.5 |
  27183.3 |
  27433.2 |
  27683.0 |
  27932.9 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 4742.1-5030.6 ns)
   4742.1 |####################
   4756.5 |
   4771.0 |
   4785.4 |
   4799.8 |
   4814.2 |
   4828.7 |
   4843.1 |
   4857.5 |
   4871.9 |
   4886.4 |
   4900.8 |
   4915.2 |
   4929.6 |
   4944.1 |
   4958.5 |####################
   4972.9 |
   4987.3 |########################################
   5001.8 |####################
   5016.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1027.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=723.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=2503.4% of algo (FFI overhead may distort results)
