# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.24 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 356% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.55 us) leads abi_native_cross_real_inproc_native (11.63 us) by 356%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -9.68 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_real_native_ffi_w (12.24 us) is 4.8x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_inproc_native shows alternating (throttle bounce) (autocorr -0.75)

abi_native_cross_real_inproc_native's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_real_null_entry (2.55 us) to slowest abi_native_cross_real_native_ffi_w (12.24 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2548.8 ns median (-79.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.80x (fastest 2548.8 ns, slowest 12236.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14008ns | 13890ns | 13449ns | 13830ns | 14554ns | -3.76% |
| abi_native_cross_real_native_ffi_w | 14555ns | 14528ns | 14069ns | 14485ns | 14904ns | base |
| abi_native_cross_real_null_entry | 4864ns | 4881ns | 4630ns | 4815ns | 5053ns | -66.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11709ns | 11292ns | 12166ns | -4.60% | 0.001 |
| abi_native_cross_real_native_ffi_w | 12273ns | 11862ns | 12588ns | base | 0.001 |
| abi_native_cross_real_null_entry | 2557ns | 2460ns | 2645ns | -79.17% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5332.2 | 11747.2 | 11708.8 | n/a |
| abi_native_cross_real_native_ffi_w | 25041.1 | 12408.0 | 12273.1 | n/a |
| abi_native_cross_real_null_entry | 27957.8 | 2666.1 | 2557.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.001 | 21.2% |
| abi_native_cross_real_native_ffi_w | 0.001 | 20.1% |
| abi_native_cross_real_null_entry | 0.006 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14008ns | 14008ns | -3.76% |
| abi_native_cross_real_native_ffi_w | 14555ns | 14555ns | base |
| abi_native_cross_real_null_entry | 4864ns | 4864ns | -66.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12237ns | base | --- | [11995, 12588] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11627ns | no significant difference | [-1255, +128]ns | [11333, 12166] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_real_null_entry | 2549ns | -9680.9ns (-79.1%) | [-9995, -9472]ns | [2478, 2645] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12259ns | -6.2% | -79.9% |
| 2 | 12215ns | +1.2% | -79.1% |
| 3 | 12277ns | -8.0% | -79.0% |
| 4 | 11862ns | +0.9% | -78.5% |
| 5 | 12899ns | -11.8% | -79.0% |
| 6 | 12127ns | -3.1% | -79.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.748 | HIGH- (thermal bounce) |
| abi_native_cross_real_native_ffi_w | -0.597 | HIGH- (thermal bounce) |
| abi_native_cross_real_null_entry | -0.273 | moderate- |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 121820.0ns | 11708.8ns | 1040.4% | HIGH |
| abi_native_cross_real_native_ffi_w | 142097.1ns | 12273.1ns | 1157.8% | HIGH |
| abi_native_cross_real_null_entry | 118553.8ns | 2557.0ns | 4636.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11292.5-12166.0 ns)
  11292.5 |########################################
  11336.2 |########################################
  11379.9 |
  11423.5 |
  11467.2 |########################################
  11510.9 |
  11554.6 |
  11598.2 |
  11641.9 |
  11685.6 |
  11729.3 |########################################
  11773.0 |
  11816.6 |
  11860.3 |
  11904.0 |
  11947.7 |########################################
  11991.3 |
  12035.0 |
  12078.7 |
  12122.4 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11862.5-12587.9 ns)
  11862.5 |########################################
  11898.8 |
  11935.0 |
  11971.3 |
  12007.6 |
  12043.9 |
  12080.1 |
  12116.4 |########################################
  12152.7 |
  12188.9 |########################################
  12225.2 |########################################
  12261.5 |########################################
  12297.7 |
  12334.0 |
  12370.3 |
  12406.6 |
  12442.8 |
  12479.1 |
  12515.4 |
  12551.6 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2459.6-2644.8 ns)
   2459.6 |####################
   2468.9 |
   2478.1 |
   2487.4 |####################
   2496.6 |
   2505.9 |
   2515.2 |
   2524.4 |
   2533.7 |
   2542.9 |########################################
   2552.2 |
   2561.5 |
   2570.7 |
   2580.0 |####################
   2589.2 |
   2598.5 |
   2607.8 |
   2617.0 |
   2626.3 |
   2635.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1048.7% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1162.0% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4658.4% of algo (FFI overhead may distort results)
