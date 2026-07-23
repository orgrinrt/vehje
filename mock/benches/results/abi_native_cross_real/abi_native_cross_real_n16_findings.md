# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 352% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.52 us) leads abi_native_cross_real_inproc_native (11.42 us) by 352%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -9.69 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_real_native_ffi_w (12.25 us) is 4.9x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_real_null_entry (2.52 us) to slowest abi_native_cross_real_native_ffi_w (12.25 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2524.8 ns median (-79.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.85x (fastest 2524.8 ns, slowest 12246.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13724ns | 13694ns | 13428ns | 13624ns | 14023ns | -6.06% |
| abi_native_cross_real_native_ffi_w | 14610ns | 14515ns | 14231ns | 14471ns | 15007ns | base |
| abi_native_cross_real_null_entry | 4798ns | 4774ns | 4679ns | 4754ns | 4923ns | -67.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11479ns | 11240ns | 11736ns | -6.80% | 0.001 |
| abi_native_cross_real_native_ffi_w | 12316ns | 11994ns | 12619ns | base | 0.001 |
| abi_native_cross_real_null_entry | 2533ns | 2475ns | 2581ns | -79.43% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5506.5 | 11521.4 | 11478.6 | n/a |
| abi_native_cross_real_native_ffi_w | 27146.6 | 12403.5 | 12315.9 | n/a |
| abi_native_cross_real_null_entry | 28486.8 | 2639.9 | 2533.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.001 | 21.7% |
| abi_native_cross_real_native_ffi_w | 0.001 | 20.2% |
| abi_native_cross_real_null_entry | 0.006 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13724ns | 13724ns | -6.06% |
| abi_native_cross_real_native_ffi_w | 14610ns | 14610ns | base |
| abi_native_cross_real_null_entry | 4798ns | 4798ns | -67.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12247ns | base | --- | [12082, 12619] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11420ns | -709.2ns (-5.8%) | [-1292, -511]ns | [11280, 11736] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2525ns | -9691.9ns (-79.1%) | [-10069, -9587]ns | [2494, 2581] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 12170ns | -7.6% | -79.4% |
| 2 | 11994ns | -5.1% | -79.4% |
| 3 | 12974ns | -12.7% | -80.2% |
| 4 | 12255ns | -4.4% | -78.8% |
| 5 | 12239ns | -3.9% | -79.5% |
| 6 | 12265ns | -6.6% | -79.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.261 | moderate+ |
| abi_native_cross_real_native_ffi_w | -0.345 | moderate- |
| abi_native_cross_real_null_entry | -0.005 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 121715.1ns | 11478.6ns | 1060.4% | HIGH |
| abi_native_cross_real_native_ffi_w | 144716.9ns | 12315.9ns | 1175.0% | HIGH |
| abi_native_cross_real_null_entry | 118533.4ns | 2533.3ns | 4678.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11239.6-11735.7 ns)
  11239.6 |########################################
  11264.4 |
  11289.2 |
  11314.0 |########################################
  11338.8 |
  11363.6 |########################################
  11388.4 |
  11413.2 |
  11438.0 |########################################
  11462.8 |
  11487.6 |
  11512.4 |
  11537.2 |
  11562.0 |
  11586.8 |
  11611.6 |
  11636.4 |
  11661.2 |
  11686.0 |
  11710.8 |########################################
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11993.8-12619.2 ns)
  11993.8 |####################
  12025.1 |
  12056.3 |
  12087.6 |
  12118.9 |
  12150.1 |####################
  12181.4 |
  12212.7 |####################
  12244.0 |########################################
  12275.2 |
  12306.5 |
  12337.8 |
  12369.0 |
  12400.3 |
  12431.6 |
  12462.9 |
  12494.1 |
  12525.4 |
  12556.7 |
  12587.9 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2475.4-2581.4 ns)
   2475.4 |########################################
   2480.7 |
   2486.0 |
   2491.3 |
   2496.6 |
   2501.9 |
   2507.2 |########################################
   2512.5 |########################################
   2517.8 |
   2523.1 |
   2528.4 |
   2533.7 |########################################
   2539.0 |
   2544.3 |
   2549.6 |
   2554.9 |
   2560.2 |########################################
   2565.5 |
   2570.8 |
   2576.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1059.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1184.2% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4706.6% of algo (FFI overhead may distort results)
