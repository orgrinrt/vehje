# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.94 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.72 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 315% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.72 us) leads abi_native_cross_real_inproc_native (11.31 us) by 315%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 77% (significant)

abi_native_cross_real_null_entry is -9.19 us (77%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_real_native_ffi_w (11.94 us) is 4.4x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_real_null_entry (2.72 us) to slowest abi_native_cross_real_native_ffi_w (11.94 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2722.5 ns median (-77.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.39x (fastest 2722.5 ns, slowest 11940.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13712ns | 13629ns | 13499ns | 13587ns | 14006ns | -3.83% |
| abi_native_cross_real_native_ffi_w | 14257ns | 14285ns | 13837ns | 14184ns | 14577ns | base |
| abi_native_cross_real_null_entry | 5037ns | 5012ns | 4930ns | 4987ns | 5166ns | -64.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11436ns | 11272ns | 11718ns | -4.10% | 0.011 |
| abi_native_cross_real_native_ffi_w | 11925ns | 11477ns | 12328ns | base | 0.011 |
| abi_native_cross_real_null_entry | 2718ns | 2655ns | 2762ns | -77.20% | 0.047 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5577.0 | 11361.7 | 11435.6 | n/a |
| abi_native_cross_real_native_ffi_w | 27119.2 | 11990.8 | 11924.7 | n/a |
| abi_native_cross_real_null_entry | 29416.6 | 2800.3 | 2718.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.011 | 23.5% |
| abi_native_cross_real_native_ffi_w | 0.011 | 22.2% |
| abi_native_cross_real_null_entry | 0.047 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13712ns | 13712ns | -3.83% |
| abi_native_cross_real_native_ffi_w | 14257ns | 14257ns | base |
| abi_native_cross_real_null_entry | 5037ns | 5037ns | -64.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11940ns | base | --- | [11506, 12328] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11308ns | -290.0ns (-2.4%) | [-1023, -155]ns | [11281, 11718] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2722ns | -9187.6ns (-76.9%) | [-9657, -8774]ns | [2671, 2762] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12371ns | -8.7% | -78.5% |
| 2 | 11535ns | -2.3% | -76.4% |
| 3 | 11477ns | -1.6% | -76.1% |
| 4 | 12009ns | -1.1% | -76.8% |
| 5 | 11872ns | -2.7% | -77.1% |
| 6 | 12285ns | -7.9% | -78.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.080 | ok |
| abi_native_cross_real_native_ffi_w | -0.088 | ok |
| abi_native_cross_real_null_entry | 0.115 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 120846.7ns | 11435.6ns | 1056.8% | HIGH |
| abi_native_cross_real_native_ffi_w | 143998.1ns | 11924.7ns | 1207.6% | HIGH |
| abi_native_cross_real_null_entry | 121441.0ns | 2718.3ns | 4467.5% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11272.1-11717.9 ns)
  11272.1 |########################################
  11294.4 |####################
  11316.7 |####################
  11339.0 |
  11361.3 |
  11383.6 |
  11405.8 |
  11428.1 |
  11450.4 |
  11472.7 |
  11495.0 |
  11517.3 |
  11539.6 |####################
  11561.9 |
  11584.2 |
  11606.5 |
  11628.7 |
  11651.0 |
  11673.3 |
  11695.6 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11476.7-12327.9 ns)
  11476.7 |########################################
  11519.3 |########################################
  11561.8 |
  11604.4 |
  11646.9 |
  11689.5 |
  11732.1 |
  11774.6 |
  11817.2 |
  11859.7 |########################################
  11902.3 |
  11944.9 |
  11987.4 |########################################
  12030.0 |
  12072.5 |
  12115.1 |
  12157.7 |
  12200.2 |
  12242.8 |########################################
  12285.3 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2655.4-2761.9 ns)
   2655.4 |########################################
   2660.7 |
   2666.0 |
   2671.4 |
   2676.7 |
   2682.0 |########################################
   2687.3 |
   2692.7 |
   2698.0 |
   2703.3 |
   2708.6 |
   2713.9 |
   2719.3 |########################################
   2724.6 |########################################
   2729.9 |
   2735.2 |########################################
   2740.6 |
   2745.9 |
   2751.2 |
   2756.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1053.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1189.4% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4468.2% of algo (FFI overhead may distort results)
