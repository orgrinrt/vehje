# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (13.93 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 3.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 199% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (3.92 us) leads abi_native_cross_scatter_inproc_native (11.74 us) by 199%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 72% (significant)

abi_native_cross_scatter_null_entry is -10.01 us (72%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 3.6x slower than the field

abi_native_cross_scatter_native_ffi_w (13.93 us) is 3.6x the fastest (3.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_scatter_native_ffi_w shows alternating (throttle bounce) (autocorr -0.56)

abi_native_cross_scatter_native_ffi_w's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.6x the fastest

Fastest abi_native_cross_scatter_null_entry (3.92 us) to slowest abi_native_cross_scatter_native_ffi_w (13.93 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 3922.3 ns median (-71.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.55x (fastest 3922.3 ns, slowest 13934.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14022ns | 14002ns | 13535ns | 13855ns | 14515ns | -14.81% |
| abi_native_cross_scatter_native_ffi_w | 16460ns | 16247ns | 15768ns | 16224ns | 17160ns | base |
| abi_native_cross_scatter_null_entry | 6206ns | 6191ns | 6065ns | 6157ns | 6350ns | -62.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11732ns | 11251ns | 12171ns | -17.04% | 0.000 |
| abi_native_cross_scatter_native_ffi_w | 14141ns | 13547ns | 14760ns | base | 0.000 |
| abi_native_cross_scatter_null_entry | 3965ns | 3884ns | 4077ns | -71.96% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5376.0 | 11702.9 | 11731.8 | n/a |
| abi_native_cross_scatter_native_ffi_w | 25799.0 | 14175.3 | 14140.7 | n/a |
| abi_native_cross_scatter_null_entry | 26579.7 | 4072.7 | 3964.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.000 | 33.1% |
| abi_native_cross_scatter_native_ffi_w | 0.000 | 27.9% |
| abi_native_cross_scatter_null_entry | 0.001 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14022ns | 14022ns | -14.81% |
| abi_native_cross_scatter_native_ffi_w | 16460ns | 16460ns | base |
| abi_native_cross_scatter_null_entry | 6206ns | 6206ns | -62.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 13935ns | base | --- | [13728, 14760] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11744ns | -2222.6ns (-15.9%) | [-3095, -1909]ns | [11280, 12171] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_scatter_null_entry | 3922ns | -10005.2ns (-71.8%) | [-10786, -9738]ns | [3894, 4077] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 13947ns | -16.0% | -71.8% |
| 2 | 13908ns | -19.1% | -71.9% |
| 3 | 14843ns | -23.8% | -72.6% |
| 4 | 13922ns | -14.6% | -70.6% |
| 5 | 14677ns | -15.1% | -73.5% |
| 6 | 13547ns | -13.1% | -71.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.292 | moderate+ |
| abi_native_cross_scatter_native_ffi_w | -0.555 | HIGH- (thermal bounce) |
| abi_native_cross_scatter_null_entry | 0.076 | ok |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 122257.3ns | 11731.8ns | 1042.1% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 151635.4ns | 14140.7ns | 1072.3% | HIGH |
| abi_native_cross_scatter_null_entry | 121107.9ns | 3964.5ns | 3054.8% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11251.2-12171.5 ns)
  11251.2 |########################################
  11297.2 |########################################
  11343.2 |
  11389.2 |
  11435.2 |
  11481.3 |
  11527.3 |
  11573.3 |
  11619.3 |
  11665.3 |
  11711.3 |########################################
  11757.3 |########################################
  11803.4 |
  11849.4 |########################################
  11895.4 |
  11941.4 |
  11987.4 |
  12033.4 |
  12079.4 |
  12125.4 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 13547.1-14759.8 ns)
  13547.1 |####################
  13607.7 |
  13668.4 |
  13729.0 |
  13789.6 |
  13850.3 |####################
  13910.9 |########################################
  13971.5 |
  14032.2 |
  14092.8 |
  14153.5 |
  14214.1 |
  14274.7 |
  14335.4 |
  14396.0 |
  14456.6 |
  14517.3 |
  14577.9 |
  14638.5 |####################
  14699.2 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 3883.7-4077.3 ns)
   3883.7 |####################
   3893.4 |
   3903.1 |########################################
   3912.7 |
   3922.4 |
   3932.1 |####################
   3941.8 |
   3951.5 |
   3961.1 |
   3970.8 |
   3980.5 |
   3990.2 |
   3999.9 |
   4009.5 |
   4019.2 |
   4028.9 |
   4038.6 |
   4048.3 |
   4057.9 |####################
   4067.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1048.0% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1065.1% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=3087.5% of algo (FFI overhead may distort results)
