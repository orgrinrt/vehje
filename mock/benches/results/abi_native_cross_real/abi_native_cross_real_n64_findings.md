# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 368% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.51 us) leads abi_native_cross_real_inproc_native (11.72 us) by 368%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -9.54 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_real_native_ffi_w (12.07 us) is 4.8x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_null_entry shows alternating (throttle bounce) (autocorr -0.58)

abi_native_cross_real_null_entry's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_real_null_entry (2.51 us) to slowest abi_native_cross_real_native_ffi_w (12.07 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2505.8 ns median (-79.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.82x (fastest 2505.8 ns, slowest 12070.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14108ns | 14123ns | 13424ns | 14004ns | 14605ns | -1.76% |
| abi_native_cross_real_native_ffi_w | 14360ns | 14330ns | 14224ns | 14296ns | 14523ns | base |
| abi_native_cross_real_null_entry | 4800ns | 4726ns | 4530ns | 4698ns | 5088ns | -66.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11763ns | 11220ns | 12192ns | -2.60% | 0.005 |
| abi_native_cross_real_native_ffi_w | 12077ns | 11972ns | 12187ns | base | 0.005 |
| abi_native_cross_real_null_entry | 2537ns | 2395ns | 2685ns | -79.00% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5422.8 | 11792.6 | 11762.7 | n/a |
| abi_native_cross_real_native_ffi_w | 26066.9 | 12232.3 | 12077.0 | n/a |
| abi_native_cross_real_null_entry | 26975.4 | 2770.9 | 2536.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.005 | 20.4% |
| abi_native_cross_real_native_ffi_w | 0.005 | 19.8% |
| abi_native_cross_real_null_entry | 0.026 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14108ns | 14108ns | -1.76% |
| abi_native_cross_real_native_ffi_w | 14360ns | 14360ns | base |
| abi_native_cross_real_null_entry | 4800ns | 4800ns | -66.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12071ns | base | --- | [11973, 12187] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11718ns | no significant difference | [-717, +115]ns | [11377, 12192] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_real_null_entry | 2506ns | -9536.0ns (-79.0%) | [-9712, -9373]ns | [2419, 2685] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12145ns | -3.5% | -79.6% |
| 2 | 12181ns | -0.2% | -78.0% |
| 3 | 11996ns | -3.9% | -78.8% |
| 4 | 11972ns | -2.1% | -80.0% |
| 5 | 11975ns | +2.2% | -77.6% |
| 6 | 12193ns | -8.0% | -80.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.514 | HIGH- (thermal bounce) |
| abi_native_cross_real_native_ffi_w | 0.106 | ok |
| abi_native_cross_real_null_entry | -0.576 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 120943.8ns | 11762.7ns | 1028.2% | HIGH |
| abi_native_cross_real_native_ffi_w | 139629.1ns | 12077.0ns | 1156.2% | HIGH |
| abi_native_cross_real_null_entry | 113346.8ns | 2536.7ns | 4468.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11220.4-12192.5 ns)
  11220.4 |####################
  11269.0 |
  11317.6 |
  11366.2 |
  11414.8 |
  11463.4 |
  11512.0 |####################
  11560.6 |
  11609.2 |
  11657.8 |
  11706.5 |########################################
  11755.1 |
  11803.7 |
  11852.3 |
  11900.9 |
  11949.5 |
  11998.1 |
  12046.7 |
  12095.3 |
  12143.9 |####################
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11972.1-12187.0 ns)
  11972.1 |########################################
  11982.8 |
  11993.6 |####################
  12004.3 |
  12015.1 |
  12025.8 |
  12036.6 |
  12047.3 |
  12058.1 |
  12068.8 |
  12079.6 |
  12090.3 |
  12101.1 |
  12111.8 |
  12122.6 |
  12133.3 |
  12144.1 |####################
  12154.8 |
  12165.6 |
  12176.3 |####################
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2395.0-2685.2 ns)
   2395.0 |########################################
   2409.5 |
   2424.0 |
   2438.5 |########################################
   2453.0 |
   2467.6 |########################################
   2482.1 |
   2496.6 |
   2511.1 |
   2525.6 |########################################
   2540.1 |
   2554.6 |
   2569.1 |
   2583.6 |
   2598.1 |
   2612.6 |
   2627.2 |
   2641.7 |
   2656.2 |
   2670.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1046.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1152.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4518.3% of algo (FFI overhead may distort results)
