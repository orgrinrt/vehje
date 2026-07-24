# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (13.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 3.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 274% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (3.01 us) leads abi_native_cross_scatter_inproc_native (11.24 us) by 274%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 77% (significant)

abi_native_cross_scatter_null_entry is -10.25 us (77%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_scatter_native_ffi_w (13.26 us) is 4.4x the fastest (3.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_scatter_null_entry (3.01 us) to slowest abi_native_cross_scatter_native_ffi_w (13.26 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 3005.8 ns median (-77.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.41x (fastest 3005.8 ns, slowest 13256.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13547ns | 13473ns | 13390ns | 13446ns | 13778ns | -12.81% |
| abi_native_cross_scatter_native_ffi_w | 15538ns | 15498ns | 15055ns | 15451ns | 15909ns | base |
| abi_native_cross_scatter_null_entry | 5320ns | 5265ns | 5195ns | 5247ns | 5493ns | -65.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11307ns | 11217ns | 11464ns | -14.98% | 0.001 |
| abi_native_cross_scatter_native_ffi_w | 13298ns | 12853ns | 13642ns | base | 0.001 |
| abi_native_cross_scatter_null_entry | 3037ns | 2980ns | 3126ns | -77.16% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5424.0 | 11348.8 | 11306.6 | n/a |
| abi_native_cross_scatter_native_ffi_w | 25273.0 | 13880.5 | 13298.2 | n/a |
| abi_native_cross_scatter_null_entry | 26579.2 | 3100.2 | 3037.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.001 | 26.5% |
| abi_native_cross_scatter_native_ffi_w | 0.001 | 22.5% |
| abi_native_cross_scatter_null_entry | 0.003 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13547ns | 13547ns | -12.81% |
| abi_native_cross_scatter_native_ffi_w | 15538ns | 15538ns | base |
| abi_native_cross_scatter_null_entry | 5320ns | 5320ns | -65.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 13256ns | base | --- | [12996, 13642] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11238ns | -1957.0ns (-14.8%) | [-2405, -1613]ns | [11218, 11464] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_scatter_null_entry | 3006ns | -10250.4ns (-77.3%) | [-10588, -9945]ns | [2980, 3126] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 13281ns | -15.3% | -77.4% |
| 2 | 13139ns | -14.5% | -77.3% |
| 3 | 12853ns | -12.6% | -75.7% |
| 4 | 13994ns | -19.8% | -77.6% |
| 5 | 13231ns | -15.2% | -77.3% |
| 6 | 13290ns | -12.1% | -77.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | -0.064 | ok |
| abi_native_cross_scatter_native_ffi_w | -0.396 | moderate- |
| abi_native_cross_scatter_null_entry | 0.146 | ok |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 119810.0ns | 11306.6ns | 1059.6% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 141609.3ns | 13298.2ns | 1064.9% | HIGH |
| abi_native_cross_scatter_null_entry | 118375.6ns | 3037.3ns | 3897.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11217.1-11464.1 ns)
  11217.1 |########################################
  11229.5 |########################################
  11241.8 |####################
  11254.2 |
  11266.5 |
  11278.9 |
  11291.2 |
  11303.6 |
  11315.9 |
  11328.3 |
  11340.6 |
  11353.0 |
  11365.3 |
  11377.7 |
  11390.0 |
  11402.4 |
  11414.7 |
  11427.1 |
  11439.4 |
  11451.8 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 12853.3-13642.3 ns)
  12853.3 |########################################
  12892.8 |
  12932.2 |
  12971.6 |
  13011.1 |
  13050.5 |
  13090.0 |
  13129.4 |########################################
  13168.9 |
  13208.3 |########################################
  13247.8 |########################################
  13287.2 |########################################
  13326.7 |
  13366.1 |
  13405.6 |
  13445.0 |
  13484.5 |
  13523.9 |
  13563.4 |
  13602.8 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 2980.0-3125.6 ns)
   2980.0 |########################################
   2987.3 |
   2994.6 |
   3001.8 |########################################
   3009.1 |
   3016.4 |
   3023.7 |
   3031.0 |
   3038.2 |
   3045.5 |
   3052.8 |
   3060.1 |
   3067.4 |
   3074.6 |
   3081.9 |
   3089.2 |
   3096.5 |
   3103.8 |
   3111.0 |
   3118.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1061.2% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1069.2% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=3935.0% of algo (FFI overhead may distort results)
