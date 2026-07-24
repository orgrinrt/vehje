# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (12.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 360% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (2.50 us) leads abi_native_cross_wideselect_inproc_native (11.50 us) by 360%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 80% (significant)

abi_native_cross_wideselect_null_entry is -9.58 us (80%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_wideselect_native_ffi_w (12.04 us) is 4.8x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_wideselect_null_entry (2.50 us) to slowest abi_native_cross_wideselect_native_ffi_w (12.04 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 2499.6 ns median (-79.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.82x (fastest 2499.6 ns, slowest 12042.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13796ns | 13774ns | 13445ns | 13679ns | 14147ns | -4.03% |
| abi_native_cross_wideselect_native_ffi_w | 14375ns | 14355ns | 13783ns | 14337ns | 14730ns | base |
| abi_native_cross_wideselect_null_entry | 4712ns | 4705ns | 4558ns | 4696ns | 4812ns | -67.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11536ns | 11232ns | 11842ns | -4.74% | 0.006 |
| abi_native_cross_wideselect_native_ffi_w | 12111ns | 11607ns | 12486ns | base | 0.005 |
| abi_native_cross_wideselect_null_entry | 2490ns | 2392ns | 2538ns | -79.44% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5355.2 | 11628.5 | 11536.3 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 25474.9 | 12242.6 | 12110.9 | n/a |
| abi_native_cross_wideselect_null_entry | 27620.5 | 2674.9 | 2490.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.006 | 20.8% |
| abi_native_cross_wideselect_native_ffi_w | 0.005 | 19.9% |
| abi_native_cross_wideselect_null_entry | 0.026 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13796ns | 13796ns | -4.03% |
| abi_native_cross_wideselect_native_ffi_w | 14375ns | 14375ns | base |
| abi_native_cross_wideselect_null_entry | 4712ns | 4712ns | -67.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 12042ns | base | --- | [11805, 12486] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11501ns | -652.0ns (-5.4%) | [-824, -247]ns | [11266, 11842] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_wideselect_null_entry | 2500ns | -9579.3ns (-79.5%) | [-9951, -9331]ns | [2434, 2538] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 12003ns | -1.0% | -80.1% |
| 2 | 12041ns | -6.2% | -79.3% |
| 3 | 11607ns | -3.2% | -78.7% |
| 4 | 12708ns | -7.1% | -79.8% |
| 5 | 12043ns | -5.7% | -79.1% |
| 6 | 12263ns | -5.0% | -79.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.415 | moderate- |
| abi_native_cross_wideselect_native_ffi_w | -0.471 | moderate- |
| abi_native_cross_wideselect_null_entry | 0.041 | ok |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 121999.4ns | 11536.3ns | 1057.5% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 141850.1ns | 12110.9ns | 1171.3% | HIGH |
| abi_native_cross_wideselect_null_entry | 114276.3ns | 2490.3ns | 4588.8% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11232.5-11841.8 ns)
  11232.5 |########################################
  11263.0 |
  11293.4 |########################################
  11323.9 |########################################
  11354.4 |
  11384.8 |
  11415.3 |
  11445.8 |
  11476.2 |
  11506.7 |
  11537.2 |
  11567.6 |
  11598.1 |
  11628.6 |########################################
  11659.0 |
  11689.5 |
  11720.0 |
  11750.4 |
  11780.9 |########################################
  11811.4 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 11607.1-12485.6 ns)
  11607.1 |#############
  11651.0 |
  11695.0 |
  11738.9 |
  11782.8 |
  11826.7 |
  11870.6 |
  11914.6 |
  11958.5 |
  12002.4 |########################################
  12046.3 |
  12090.3 |
  12134.2 |
  12178.1 |
  12222.0 |#############
  12266.0 |
  12309.9 |
  12353.8 |
  12397.7 |
  12441.7 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 2392.1-2537.7 ns)
   2392.1 |########################################
   2399.4 |
   2406.7 |
   2413.9 |
   2421.2 |
   2428.5 |
   2435.8 |
   2443.1 |
   2450.3 |
   2457.6 |
   2464.9 |
   2472.2 |########################################
   2479.5 |
   2486.7 |########################################
   2494.0 |
   2501.3 |########################################
   2508.6 |########################################
   2515.9 |
   2523.1 |
   2530.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1059.9% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1178.9% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=4578.9% of algo (FFI overhead may distort results)
