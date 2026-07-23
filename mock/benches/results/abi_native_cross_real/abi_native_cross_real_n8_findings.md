# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (13.42 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 280% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.07 us) leads abi_native_cross_real_inproc_native (11.65 us) by 280%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 77% (significant)

abi_native_cross_real_null_entry is -10.40 us (77%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_real_native_ffi_w (13.42 us) is 4.4x the fastest (3.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_real_null_entry (3.07 us) to slowest abi_native_cross_real_native_ffi_w (13.42 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_native_cross_real_inproc_native is inconsistent: worst-20% is 2.4x its best-20%

abi_native_cross_real_inproc_native's best 20% of batches run at 11.38 us but its worst 20% at 26.92 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3067.3 ns median (-77.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.37x (fastest 3067.3 ns, slowest 13418.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 19138ns | 13953ns | 13555ns | 13859ns | 29849ns | +20.73% |
| abi_native_cross_real_native_ffi_w | 15852ns | 15779ns | 15427ns | 15678ns | 16327ns | base |
| abi_native_cross_real_null_entry | 5554ns | 5433ns | 5308ns | 5403ns | 5904ns | -64.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 16668ns | 11384ns | 26925ns | +23.19% | 0.000 |
| abi_native_cross_real_native_ffi_w | 13530ns | 13194ns | 13965ns | base | 0.001 |
| abi_native_cross_real_null_entry | 3134ns | 3023ns | 3312ns | -76.84% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 6083.5 | 24599.8 | 16667.7 | 7 |
| abi_native_cross_real_native_ffi_w | 27981.2 | 14020.8 | 13530.3 | n/a |
| abi_native_cross_real_null_entry | 29989.5 | 3204.2 | 3134.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.001 | 26.0% |
| abi_native_cross_real_native_ffi_w | 0.001 | 22.5% |
| abi_native_cross_real_null_entry | 0.003 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 19138ns | 19138ns | +20.73% |
| abi_native_cross_real_native_ffi_w | 15852ns | 15852ns | base |
| abi_native_cross_real_null_entry | 5554ns | 5554ns | -64.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 13418ns | base | --- | [13208, 13965] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11649ns | no significant difference | [-1888, +12966]ns | [11430, 26925] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_real_null_entry | 3067ns | -10395.0ns (-77.5%) | [-10653, -10140]ns | [3023, 3312] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 14196ns | +193.9% | -75.4% |
| 2 | 13735ns | -11.7% | -77.2% |
| 3 | 13194ns | -13.0% | -76.8% |
| 4 | 13221ns | -13.9% | -76.7% |
| 5 | 13294ns | -12.0% | -77.3% |
| 6 | 13542ns | -14.3% | -77.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.015 | ok |
| abi_native_cross_real_native_ffi_w | 0.322 | moderate+ |
| abi_native_cross_real_null_entry | 0.135 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 159868.4ns | 16667.7ns | 959.2% | HIGH |
| abi_native_cross_real_native_ffi_w | 146232.5ns | 13530.3ns | 1080.8% | HIGH |
| abi_native_cross_real_null_entry | 122954.5ns | 3134.2ns | 3923.0% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11383.7-26924.8 ns)
  11383.7 |########################################
  12160.8 |
  12937.8 |
  13714.9 |
  14491.9 |
  15269.0 |
  16046.0 |
  16823.1 |
  17600.1 |
  18377.2 |
  19154.2 |
  19931.3 |
  20708.4 |
  21485.4 |
  22262.5 |
  23039.5 |
  23816.6 |
  24593.6 |
  25370.7 |
  26147.7 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 13194.2-13965.4 ns)
  13194.2 |########################################
  13232.8 |
  13271.3 |####################
  13309.9 |
  13348.4 |
  13387.0 |
  13425.6 |
  13464.1 |
  13502.7 |
  13541.2 |####################
  13579.8 |
  13618.4 |
  13656.9 |
  13695.5 |
  13734.0 |####################
  13772.6 |
  13811.2 |
  13849.7 |
  13888.3 |
  13926.8 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3022.9-3312.5 ns)
   3022.9 |########################################
   3037.4 |
   3051.9 |####################
   3066.3 |####################
   3080.8 |
   3095.3 |
   3109.8 |
   3124.3 |####################
   3138.7 |
   3153.2 |
   3167.7 |
   3182.2 |
   3196.7 |
   3211.1 |
   3225.6 |
   3240.1 |
   3254.6 |
   3269.1 |
   3283.5 |
   3298.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: CV=67.3% (high variance, measurements may be unstable)
- **abi_native_cross_real_inproc_native**: bridge=1053.0% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1084.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3948.7% of algo (FFI overhead may distort results)
