# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.19 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 366% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.54 us) leads abi_native_cross_real_inproc_native (11.84 us) by 366%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -9.63 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_real_native_ffi_w (12.19 us) is 4.8x the fastest (2.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_real_null_entry (2.54 us) to slowest abi_native_cross_real_native_ffi_w (12.19 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2539.8 ns median (-79.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.80x (fastest 2539.8 ns, slowest 12187.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 14257ns | 14115ns | 13990ns | 14076ns | 14662ns | -1.38% |
| abi_native_cross_real_native_ffi_w | 14456ns | 14457ns | 14107ns | 14446ns | 14646ns | base |
| abi_native_cross_real_null_entry | 4820ns | 4786ns | 4561ns | 4764ns | 5034ns | -66.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11916ns | 11669ns | 12229ns | -2.14% | 0.001 |
| abi_native_cross_real_native_ffi_w | 12177ns | 11858ns | 12330ns | base | 0.001 |
| abi_native_cross_real_null_entry | 2561ns | 2420ns | 2685ns | -78.97% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5472.1 | 11912.6 | 11915.6 | n/a |
| abi_native_cross_real_native_ffi_w | 26188.8 | 12333.9 | 12176.7 | n/a |
| abi_native_cross_real_null_entry | 26646.1 | 2620.8 | 2560.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.001 | 20.4% |
| abi_native_cross_real_native_ffi_w | 0.001 | 19.9% |
| abi_native_cross_real_null_entry | 0.006 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 14257ns | 14257ns | -1.38% |
| abi_native_cross_real_native_ffi_w | 14456ns | 14456ns | base |
| abi_native_cross_real_null_entry | 4820ns | 4820ns | -66.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12187ns | base | --- | [12012, 12330] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11844ns | no significant difference | [-552, +43]ns | [11674, 12229] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_real_null_entry | 2540ns | -9628.5ns (-79.0%) | [-9732, -9487]ns | [2458, 2685] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11858ns | -0.5% | -79.6% |
| 2 | 12376ns | -1.9% | -78.2% |
| 3 | 12170ns | +1.2% | -78.8% |
| 4 | 12205ns | -2.5% | -78.1% |
| 5 | 12285ns | -4.9% | -79.7% |
| 6 | 12167ns | -4.1% | -79.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.341 | moderate+ |
| abi_native_cross_real_native_ffi_w | -0.410 | moderate- |
| abi_native_cross_real_null_entry | -0.334 | moderate- |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 123640.6ns | 11915.6ns | 1037.6% | HIGH |
| abi_native_cross_real_native_ffi_w | 142113.4ns | 12176.7ns | 1167.1% | HIGH |
| abi_native_cross_real_null_entry | 117868.0ns | 2560.8ns | 4602.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11669.2-12228.5 ns)
  11669.2 |########################################
  11697.2 |
  11725.1 |
  11753.1 |
  11781.1 |####################
  11809.0 |
  11837.0 |
  11865.0 |
  11892.9 |####################
  11920.9 |
  11948.9 |
  11976.8 |
  12004.8 |
  12032.8 |
  12060.7 |
  12088.7 |
  12116.7 |####################
  12144.6 |
  12172.6 |
  12200.6 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11857.9-12330.4 ns)
  11857.9 |####################
  11881.5 |
  11905.1 |
  11928.8 |
  11952.4 |
  11976.0 |
  11999.6 |
  12023.3 |
  12046.9 |
  12070.5 |
  12094.1 |
  12117.8 |
  12141.4 |
  12165.0 |########################################
  12188.6 |####################
  12212.3 |
  12235.9 |
  12259.5 |
  12283.1 |####################
  12306.8 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2419.6-2684.9 ns)
   2419.6 |########################################
   2432.9 |
   2446.1 |
   2459.4 |
   2472.7 |
   2485.9 |########################################
   2499.2 |########################################
   2512.5 |
   2525.7 |
   2539.0 |
   2552.3 |
   2565.5 |########################################
   2578.8 |
   2592.1 |
   2605.3 |
   2618.6 |
   2631.9 |
   2645.1 |
   2658.4 |########################################
   2671.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1043.5% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1160.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4632.1% of algo (FFI overhead may distort results)
