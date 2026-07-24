# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (12.14 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 2.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 366% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (2.47 us) leads abi_native_cross_scatter_inproc_native (11.50 us) by 366%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 79% (significant)

abi_native_cross_scatter_null_entry is -9.61 us (79%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_scatter_native_ffi_w (12.14 us) is 4.9x the fastest (2.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_scatter_native_ffi_w shows alternating (throttle bounce) (autocorr -0.55)

abi_native_cross_scatter_native_ffi_w's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_scatter_null_entry (2.47 us) to slowest abi_native_cross_scatter_native_ffi_w (12.14 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 2468.6 ns median (-79.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.92x (fastest 2468.6 ns, slowest 12141.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13930ns | 13810ns | 13485ns | 13771ns | 14390ns | -4.39% |
| abi_native_cross_scatter_native_ffi_w | 14569ns | 14440ns | 13824ns | 14404ns | 15191ns | base |
| abi_native_cross_scatter_null_entry | 4743ns | 4725ns | 4638ns | 4704ns | 4855ns | -67.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11590ns | 11248ns | 11944ns | -5.16% | 0.006 |
| abi_native_cross_scatter_native_ffi_w | 12220ns | 11628ns | 12705ns | base | 0.005 |
| abi_native_cross_scatter_null_entry | 2479ns | 2433ns | 2531ns | -79.71% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5278.0 | 11668.5 | 11589.5 | n/a |
| abi_native_cross_scatter_native_ffi_w | 26221.6 | 12406.9 | 12220.4 | n/a |
| abi_native_cross_scatter_null_entry | 26157.1 | 2716.5 | 2479.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.006 | 21.2% |
| abi_native_cross_scatter_native_ffi_w | 0.005 | 20.0% |
| abi_native_cross_scatter_null_entry | 0.026 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13930ns | 13930ns | -4.39% |
| abi_native_cross_scatter_native_ffi_w | 14569ns | 14569ns | base |
| abi_native_cross_scatter_null_entry | 4743ns | 4743ns | -67.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 12142ns | base | --- | [11815, 12705] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11505ns | -588.6ns (-4.8%) | [-940, -364]ns | [11320, 11944] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_scatter_null_entry | 2469ns | -9611.0ns (-79.2%) | [-10236, -9376]ns | [2439, 2531] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 12002ns | -3.4% | -79.6% |
| 2 | 12393ns | -7.9% | -80.3% |
| 3 | 12162ns | -6.3% | -79.4% |
| 4 | 12122ns | -2.9% | -78.9% |
| 5 | 13016ns | -6.9% | -80.9% |
| 6 | 11628ns | -3.3% | -79.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | -0.172 | ok |
| abi_native_cross_scatter_native_ffi_w | -0.551 | HIGH- (thermal bounce) |
| abi_native_cross_scatter_null_entry | 0.243 | moderate+ |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 121314.8ns | 11589.5ns | 1046.8% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 142594.1ns | 12220.4ns | 1166.9% | HIGH |
| abi_native_cross_scatter_null_entry | 111586.1ns | 2479.5ns | 4500.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11247.9-11944.2 ns)
  11247.9 |####################
  11282.7 |
  11317.5 |
  11352.3 |
  11387.1 |########################################
  11422.0 |
  11456.8 |
  11491.6 |
  11526.4 |
  11561.2 |####################
  11596.0 |
  11630.8 |
  11665.7 |
  11700.5 |
  11735.3 |
  11770.1 |####################
  11804.9 |
  11839.7 |
  11874.5 |
  11909.3 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 11627.5-12704.5 ns)
  11627.5 |####################
  11681.4 |
  11735.2 |
  11789.1 |
  11842.9 |
  11896.8 |
  11950.6 |####################
  12004.5 |
  12058.3 |
  12112.2 |########################################
  12166.0 |
  12219.9 |
  12273.7 |
  12327.6 |
  12381.4 |####################
  12435.3 |
  12489.1 |
  12543.0 |
  12596.8 |
  12650.7 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 2433.3-2530.8 ns)
   2433.3 |####################
   2438.2 |
   2443.1 |########################################
   2447.9 |
   2452.8 |
   2457.7 |
   2462.6 |
   2467.4 |
   2472.3 |
   2477.2 |
   2482.1 |
   2487.0 |####################
   2491.8 |
   2496.7 |
   2501.6 |
   2506.5 |####################
   2511.3 |
   2516.2 |
   2521.1 |
   2526.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1059.5% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1163.1% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=4503.4% of algo (FFI overhead may distort results)
