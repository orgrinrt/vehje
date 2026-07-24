# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (14.00 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 3.99 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 189% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (3.99 us) leads abi_native_cross_tight_inproc_native (11.51 us) by 189%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 72% (significant)

abi_native_cross_tight_null_entry is -10.06 us (72%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 3.5x slower than the field

abi_native_cross_tight_native_ffi_w (14.00 us) is 3.5x the fastest (3.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_tight_null_entry shows alternating (throttle bounce) (autocorr -0.56)

abi_native_cross_tight_null_entry's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.5x the fastest

Fastest abi_native_cross_tight_null_entry (3.99 us) to slowest abi_native_cross_tight_native_ffi_w (14.00 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 3985.4 ns median (-71.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.51x (fastest 3985.4 ns, slowest 13999.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13854ns | 13781ns | 13440ns | 13671ns | 14337ns | -15.18% |
| abi_native_cross_tight_native_ffi_w | 16335ns | 16256ns | 15743ns | 16146ns | 16914ns | base |
| abi_native_cross_tight_null_entry | 6273ns | 6249ns | 6040ns | 6191ns | 6514ns | -61.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11539ns | 11242ns | 11865ns | -17.95% | 0.000 |
| abi_native_cross_tight_native_ffi_w | 14063ns | 13533ns | 14584ns | base | 0.000 |
| abi_native_cross_tight_null_entry | 4006ns | 3879ns | 4150ns | -71.52% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5412.1 | 11518.4 | 11539.2 | n/a |
| abi_native_cross_tight_native_ffi_w | 25648.0 | 14094.9 | 14063.2 | n/a |
| abi_native_cross_tight_null_entry | 26610.2 | 4139.2 | 4005.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.000 | 33.7% |
| abi_native_cross_tight_native_ffi_w | 0.000 | 27.7% |
| abi_native_cross_tight_null_entry | 0.001 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13854ns | 13854ns | -15.18% |
| abi_native_cross_tight_native_ffi_w | 16335ns | 16335ns | base |
| abi_native_cross_tight_null_entry | 6273ns | 6273ns | -61.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 13999ns | base | --- | [13607, 14584] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11510ns | -2363.9ns (-16.9%) | [-3074, -2134]ns | [11243, 11865] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_tight_null_entry | 3985ns | -10058.9ns (-71.9%) | [-10599, -9514]ns | [3882, 4150] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 13982ns | -16.0% | -72.3% |
| 2 | 13533ns | -16.9% | -68.9% |
| 3 | 13680ns | -17.8% | -71.0% |
| 4 | 15128ns | -22.4% | -73.0% |
| 5 | 14040ns | -19.6% | -72.3% |
| 6 | 14015ns | -14.5% | -71.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.375 | moderate- |
| abi_native_cross_tight_native_ffi_w | -0.118 | ok |
| abi_native_cross_tight_null_entry | -0.559 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 121963.5ns | 11539.2ns | 1056.9% | HIGH |
| abi_native_cross_tight_native_ffi_w | 147995.5ns | 14063.2ns | 1052.4% | HIGH |
| abi_native_cross_tight_null_entry | 120903.6ns | 4005.8ns | 3018.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11242.5-11864.8 ns)
  11242.5 |########################################
  11273.6 |####################
  11304.7 |
  11335.8 |
  11367.0 |
  11398.1 |
  11429.2 |
  11460.3 |
  11491.4 |
  11522.5 |
  11553.6 |
  11584.8 |
  11615.9 |
  11647.0 |
  11678.1 |
  11709.2 |####################
  11740.3 |####################
  11771.5 |
  11802.6 |
  11833.7 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 13533.3-14584.0 ns)
  13533.3 |####################
  13585.8 |
  13638.4 |####################
  13690.9 |
  13743.4 |
  13796.0 |
  13848.5 |
  13901.0 |
  13953.6 |####################
  14006.1 |########################################
  14058.6 |
  14111.2 |
  14163.7 |
  14216.2 |
  14268.8 |
  14321.3 |
  14373.8 |
  14426.4 |
  14478.9 |
  14531.4 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 3879.2-4150.2 ns)
   3879.2 |########################################
   3892.8 |
   3906.3 |
   3919.8 |
   3933.4 |
   3946.9 |
   3960.5 |####################
   3974.0 |
   3987.6 |####################
   4001.1 |
   4014.7 |
   4028.2 |
   4041.8 |
   4055.3 |
   4068.9 |
   4082.4 |####################
   4096.0 |
   4109.6 |
   4123.1 |
   4136.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1058.4% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1053.3% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=3030.9% of algo (FFI overhead may distort results)
