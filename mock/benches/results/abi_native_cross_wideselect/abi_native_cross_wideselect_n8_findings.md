# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (13.33 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 3.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 289% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (3.01 us) leads abi_native_cross_wideselect_inproc_native (11.70 us) by 289%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 77% (significant)

abi_native_cross_wideselect_null_entry is -10.28 us (77%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_wideselect_native_ffi_w (13.33 us) is 4.4x the fastest (3.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_wideselect_null_entry (3.01 us) to slowest abi_native_cross_wideselect_native_ffi_w (13.33 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 3008.8 ns median (-77.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.43x (fastest 3008.8 ns, slowest 13330.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 14051ns | 13996ns | 13398ns | 13931ns | 14557ns | -9.65% |
| abi_native_cross_wideselect_native_ffi_w | 15552ns | 15591ns | 15239ns | 15574ns | 15676ns | base |
| abi_native_cross_wideselect_null_entry | 5278ns | 5255ns | 5085ns | 5202ns | 5489ns | -66.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11758ns | 11200ns | 12197ns | -11.51% | 0.001 |
| abi_native_cross_wideselect_native_ffi_w | 13288ns | 13039ns | 13400ns | base | 0.001 |
| abi_native_cross_wideselect_null_entry | 3024ns | 2922ns | 3141ns | -77.24% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5396.6 | 11808.8 | 11758.3 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 25340.1 | 13902.8 | 13288.5 | n/a |
| abi_native_cross_wideselect_null_entry | 28124.0 | 3107.0 | 3024.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.001 | 25.0% |
| abi_native_cross_wideselect_native_ffi_w | 0.001 | 21.9% |
| abi_native_cross_wideselect_null_entry | 0.003 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 14051ns | 14051ns | -9.65% |
| abi_native_cross_wideselect_native_ffi_w | 15552ns | 15552ns | base |
| abi_native_cross_wideselect_null_entry | 5278ns | 5278ns | -66.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 13330ns | base | --- | [13135, 13400] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11699ns | -1502.5ns (-11.3%) | [-1892, -1196]ns | [11380, 12197] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_wideselect_null_entry | 3009ns | -10280.2ns (-77.1%) | [-10415, -10097]ns | [2923, 3141] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 13363ns | -12.8% | -78.1% |
| 2 | 13438ns | -9.4% | -76.2% |
| 3 | 13347ns | -8.5% | -76.9% |
| 4 | 13039ns | -9.9% | -76.3% |
| 5 | 13231ns | -15.4% | -77.8% |
| 6 | 13313ns | -13.2% | -78.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.347 | moderate+ |
| abi_native_cross_wideselect_native_ffi_w | 0.187 | ok |
| abi_native_cross_wideselect_null_entry | -0.007 | ok |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 122205.3ns | 11758.3ns | 1039.3% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 144068.9ns | 13288.5ns | 1084.2% | HIGH |
| abi_native_cross_wideselect_null_entry | 119482.3ns | 3024.4ns | 3950.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11199.6-12196.7 ns)
  11199.6 |########################################
  11249.5 |
  11299.3 |
  11349.2 |
  11399.0 |
  11448.9 |
  11498.7 |
  11548.6 |########################################
  11598.4 |
  11648.3 |########################################
  11698.1 |########################################
  11748.0 |
  11797.8 |
  11847.7 |
  11897.5 |
  11947.4 |
  11997.2 |
  12047.1 |
  12096.9 |
  12146.8 |########################################
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 13038.8-13400.4 ns)
  13038.8 |####################
  13056.9 |
  13075.0 |
  13093.0 |
  13111.1 |
  13129.2 |
  13147.3 |
  13165.4 |
  13183.4 |
  13201.5 |
  13219.6 |####################
  13237.7 |
  13255.8 |
  13273.8 |
  13291.9 |
  13310.0 |####################
  13328.1 |
  13346.2 |########################################
  13364.2 |
  13382.3 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 2921.7-3141.4 ns)
   2921.7 |########################################
   2932.7 |####################
   2943.7 |
   2954.7 |
   2965.6 |
   2976.6 |
   2987.6 |
   2998.6 |
   3009.6 |
   3020.6 |
   3031.6 |
   3042.6 |
   3053.5 |
   3064.5 |
   3075.5 |########################################
   3086.5 |
   3097.5 |
   3108.5 |
   3119.5 |
   3130.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1047.6% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1081.7% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=3969.8% of algo (FFI overhead may distort results)
