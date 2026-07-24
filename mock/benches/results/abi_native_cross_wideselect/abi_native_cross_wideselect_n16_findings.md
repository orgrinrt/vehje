# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (12.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 366% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (2.50 us) leads abi_native_cross_wideselect_inproc_native (11.64 us) by 366%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 79% (significant)

abi_native_cross_wideselect_null_entry is -9.67 us (79%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_wideselect_native_ffi_w (12.21 us) is 4.9x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_wideselect_inproc_native shows alternating (throttle bounce) (autocorr -0.50)

abi_native_cross_wideselect_inproc_native's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_wideselect_null_entry (2.50 us) to slowest abi_native_cross_wideselect_native_ffi_w (12.21 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 2497.9 ns median (-79.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.89x (fastest 2497.9 ns, slowest 12209.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 14006ns | 13877ns | 13627ns | 13845ns | 14435ns | -3.13% |
| abi_native_cross_wideselect_native_ffi_w | 14459ns | 14465ns | 14380ns | 14439ns | 14528ns | base |
| abi_native_cross_wideselect_null_entry | 4742ns | 4721ns | 4622ns | 4696ns | 4872ns | -67.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11705ns | 11276ns | 12067ns | -4.03% | 0.001 |
| abi_native_cross_wideselect_native_ffi_w | 12197ns | 12107ns | 12264ns | base | 0.001 |
| abi_native_cross_wideselect_null_entry | 2503ns | 2460ns | 2552ns | -79.48% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5364.6 | 11876.2 | 11705.4 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 26334.5 | 12334.8 | 12197.1 | n/a |
| abi_native_cross_wideselect_null_entry | 28095.4 | 2594.6 | 2503.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.001 | 21.1% |
| abi_native_cross_wideselect_native_ffi_w | 0.001 | 20.1% |
| abi_native_cross_wideselect_null_entry | 0.006 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 14006ns | 14006ns | -3.13% |
| abi_native_cross_wideselect_native_ffi_w | 14459ns | 14459ns | base |
| abi_native_cross_wideselect_null_entry | 4742ns | 4742ns | -67.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 12209ns | base | --- | [12118, 12264] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11636ns | -484.1ns (-4.0%) | [-827, -164]ns | [11414, 12067] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_native_cross_wideselect_null_entry | 2498ns | -9667.9ns (-79.2%) | [-9805, -9609]ns | [2460, 2552] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 12107ns | -3.3% | -79.7% |
| 2 | 12240ns | +0.6% | -79.9% |
| 3 | 12193ns | -7.5% | -78.9% |
| 4 | 12225ns | -3.3% | -79.3% |
| 5 | 12289ns | -6.0% | -80.0% |
| 6 | 12128ns | -4.7% | -79.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.500 | HIGH- (thermal bounce) |
| abi_native_cross_wideselect_native_ffi_w | -0.330 | moderate- |
| abi_native_cross_wideselect_null_entry | -0.143 | ok |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 122706.7ns | 11705.4ns | 1048.3% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 142031.7ns | 12197.1ns | 1164.5% | HIGH |
| abi_native_cross_wideselect_null_entry | 117535.8ns | 2503.2ns | 4695.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11275.8-12066.6 ns)
  11275.8 |########################################
  11315.3 |
  11354.9 |
  11394.4 |
  11434.0 |
  11473.5 |
  11513.1 |########################################
  11552.6 |########################################
  11592.1 |
  11631.7 |
  11671.2 |########################################
  11710.8 |
  11750.3 |
  11789.9 |########################################
  11829.4 |
  11868.9 |
  11908.5 |
  11948.0 |
  11987.6 |
  12027.1 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 12107.1-12264.4 ns)
  12107.1 |########################################
  12115.0 |
  12122.8 |########################################
  12130.7 |
  12138.6 |
  12146.4 |
  12154.3 |
  12162.2 |
  12170.0 |
  12177.9 |
  12185.8 |########################################
  12193.6 |
  12201.5 |
  12209.3 |
  12217.2 |########################################
  12225.1 |
  12232.9 |########################################
  12240.8 |
  12248.7 |
  12256.5 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 2459.6-2551.9 ns)
   2459.6 |########################################
   2464.2 |
   2468.8 |
   2473.4 |
   2478.1 |
   2482.7 |
   2487.3 |
   2491.9 |
   2496.5 |
   2501.1 |
   2505.8 |
   2510.4 |
   2515.0 |
   2519.6 |
   2524.2 |
   2528.8 |
   2533.4 |##########################
   2538.1 |
   2542.7 |
   2547.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1059.2% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1162.7% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=4674.7% of algo (FFI overhead may distort results)
