# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.05 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 367% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.43 us) leads abi_native_cross_real_inproc_native (11.36 us) by 367%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 80% (significant)

abi_native_cross_real_null_entry is -9.59 us (80%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 5.0x slower than the field

abi_native_cross_real_native_ffi_w (12.05 us) is 5.0x the fastest (2.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_null_entry shows alternating (throttle bounce) (autocorr -0.59)

abi_native_cross_real_null_entry's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.0x the fastest

Fastest abi_native_cross_real_null_entry (2.43 us) to slowest abi_native_cross_real_native_ffi_w (12.05 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2432.9 ns median (-79.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.95x (fastest 2432.9 ns, slowest 12046.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13684ns | 13580ns | 13513ns | 13563ns | 13952ns | -4.61% |
| abi_native_cross_real_native_ffi_w | 14345ns | 14344ns | 13920ns | 14293ns | 14635ns | base |
| abi_native_cross_real_null_entry | 4698ns | 4703ns | 4555ns | 4692ns | 4779ns | -67.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11451ns | 11322ns | 11663ns | -5.00% | 0.006 |
| abi_native_cross_real_native_ffi_w | 12053ns | 11715ns | 12332ns | base | 0.005 |
| abi_native_cross_real_null_entry | 2441ns | 2392ns | 2485ns | -79.75% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5526.9 | 11506.6 | 11451.1 | n/a |
| abi_native_cross_real_native_ffi_w | 27298.6 | 12217.8 | 12053.3 | n/a |
| abi_native_cross_real_null_entry | 27565.4 | 2707.0 | 2440.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.006 | 21.1% |
| abi_native_cross_real_native_ffi_w | 0.005 | 19.9% |
| abi_native_cross_real_null_entry | 0.026 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13684ns | 13684ns | -4.61% |
| abi_native_cross_real_native_ffi_w | 14345ns | 14345ns | base |
| abi_native_cross_real_null_entry | 4698ns | 4698ns | -67.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12047ns | base | --- | [11781, 12332] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11363ns | -580.6ns (-4.8%) | [-1003, -223]ns | [11327, 11663] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2433ns | -9592.9ns (-79.6%) | [-9928, -9317]ns | [2404, 2485] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12057ns | -2.8% | -79.9% |
| 2 | 11846ns | -3.8% | -79.4% |
| 3 | 11715ns | -0.9% | -78.8% |
| 4 | 12453ns | -9.0% | -80.6% |
| 5 | 12036ns | -5.9% | -79.3% |
| 6 | 12212ns | -7.3% | -80.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.103 | ok |
| abi_native_cross_real_native_ffi_w | -0.220 | moderate- |
| abi_native_cross_real_null_entry | -0.593 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 121815.5ns | 11451.1ns | 1063.8% | HIGH |
| abi_native_cross_real_native_ffi_w | 142641.1ns | 12053.3ns | 1183.4% | HIGH |
| abi_native_cross_real_null_entry | 112216.6ns | 2440.7ns | 4597.8% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11322.5-11663.3 ns)
  11322.5 |########################################
  11339.5 |
  11356.6 |
  11373.6 |#############
  11390.7 |
  11407.7 |
  11424.7 |
  11441.8 |
  11458.8 |
  11475.9 |
  11492.9 |
  11509.9 |
  11527.0 |
  11544.0 |
  11561.1 |
  11578.1 |
  11595.1 |#############
  11612.2 |
  11629.2 |
  11646.3 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11715.4-12332.5 ns)
  11715.4 |########################################
  11746.3 |
  11777.1 |
  11808.0 |
  11838.8 |########################################
  11869.7 |
  11900.5 |
  11931.4 |
  11962.2 |
  11993.1 |
  12024.0 |########################################
  12054.8 |########################################
  12085.7 |
  12116.5 |
  12147.4 |
  12178.2 |
  12209.1 |########################################
  12239.9 |
  12270.8 |
  12301.6 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2392.5-2485.0 ns)
   2392.5 |####################
   2397.1 |
   2401.8 |
   2406.4 |
   2411.0 |
   2415.6 |########################################
   2420.2 |
   2424.9 |
   2429.5 |
   2434.1 |
   2438.8 |
   2443.4 |####################
   2448.0 |
   2452.6 |
   2457.2 |
   2461.9 |
   2466.5 |
   2471.1 |
   2475.8 |
   2480.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1063.7% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1189.6% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4601.6% of algo (FFI overhead may distort results)
