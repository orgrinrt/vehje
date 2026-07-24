# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (11.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 2.56 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 354% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (2.56 us) leads abi_native_cross_madd_inproc_native (11.61 us) by 354%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 79% (significant)

abi_native_cross_madd_null_entry is -9.45 us (79%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 4.7x slower than the field

abi_native_cross_madd_native_ffi_w (11.98 us) is 4.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.7x the fastest

Fastest abi_native_cross_madd_null_entry (2.56 us) to slowest abi_native_cross_madd_native_ffi_w (11.98 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 2555.8 ns median (-78.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.69x (fastest 2555.8 ns, slowest 11979.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13922ns | 13924ns | 13560ns | 13811ns | 14269ns | -2.45% |
| abi_native_cross_madd_native_ffi_w | 14272ns | 14256ns | 13973ns | 14245ns | 14460ns | base |
| abi_native_cross_madd_null_entry | 4830ns | 4800ns | 4652ns | 4755ns | 5033ns | -66.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11614ns | 11250ns | 11932ns | -3.31% | 0.001 |
| abi_native_cross_madd_native_ffi_w | 12012ns | 11796ns | 12199ns | base | 0.001 |
| abi_native_cross_madd_null_entry | 2560ns | 2465ns | 2648ns | -78.69% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5445.8 | 11619.7 | 11614.4 | n/a |
| abi_native_cross_madd_native_ffi_w | 25881.7 | 12159.8 | 12012.5 | n/a |
| abi_native_cross_madd_null_entry | 26798.8 | 2632.0 | 2560.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.001 | 21.2% |
| abi_native_cross_madd_native_ffi_w | 0.001 | 20.6% |
| abi_native_cross_madd_null_entry | 0.006 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13922ns | 13922ns | -2.45% |
| abi_native_cross_madd_native_ffi_w | 14272ns | 14272ns | base |
| abi_native_cross_madd_null_entry | 4830ns | 4830ns | -66.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 11980ns | base | --- | [11859, 12199] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11612ns | no significant difference | [-809, +72]ns | [11299, 11932] | no | 0.6875 | 0.6875 | 0 |
| abi_native_cross_madd_null_entry | 2556ns | -9450.4ns (-78.9%) | [-9694, -9211]ns | [2478, 2648] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 12183ns | -2.6% | -79.8% |
| 2 | 12214ns | -7.1% | -79.2% |
| 3 | 12004ns | -6.3% | -78.6% |
| 4 | 11796ns | +0.7% | -77.8% |
| 5 | 11955ns | -5.0% | -79.2% |
| 6 | 11922ns | +0.5% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.427 | moderate- |
| abi_native_cross_madd_native_ffi_w | 0.407 | moderate+ |
| abi_native_cross_madd_null_entry | -0.328 | moderate- |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 4/6, lost 2/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 122073.2ns | 11614.4ns | 1051.0% | HIGH |
| abi_native_cross_madd_native_ffi_w | 143632.8ns | 12012.5ns | 1195.7% | HIGH |
| abi_native_cross_madd_null_entry | 117199.6ns | 2560.4ns | 4577.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11250.0-11931.6 ns)
  11250.0 |####################
  11284.1 |
  11318.2 |####################
  11352.2 |####################
  11386.3 |
  11420.4 |
  11454.5 |
  11488.6 |
  11522.7 |
  11556.7 |
  11590.8 |
  11624.9 |
  11659.0 |
  11693.1 |
  11727.2 |
  11761.2 |
  11795.3 |
  11829.4 |
  11863.5 |########################################
  11897.6 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 11796.2-12198.5 ns)
  11796.2 |########################################
  11816.3 |
  11836.4 |
  11856.6 |
  11876.7 |
  11896.8 |
  11916.9 |########################################
  11937.0 |########################################
  11957.1 |
  11977.3 |
  11997.4 |########################################
  12017.5 |
  12037.6 |
  12057.7 |
  12077.8 |
  12098.0 |
  12118.1 |
  12138.2 |
  12158.3 |
  12178.4 |########################################
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 2465.0-2647.9 ns)
   2465.0 |########################################
   2474.1 |
   2483.3 |########################################
   2492.4 |
   2501.6 |
   2510.7 |
   2519.9 |
   2529.0 |
   2538.2 |########################################
   2547.3 |
   2556.5 |
   2565.6 |########################################
   2574.8 |
   2583.9 |
   2593.1 |
   2602.2 |
   2611.4 |########################################
   2620.5 |
   2629.7 |
   2638.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1049.3% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1204.2% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=4582.1% of algo (FFI overhead may distort results)
