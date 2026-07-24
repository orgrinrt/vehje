# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (12.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 2.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 400% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (2.27 us) leads abi_native_cross_madd_inproc_native (11.32 us) by 400%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 82% (significant)

abi_native_cross_madd_null_entry is -10.14 us (82%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 5.5x slower than the field

abi_native_cross_madd_native_ffi_w (12.44 us) is 5.5x the fastest (2.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.5x the fastest

Fastest abi_native_cross_madd_null_entry (2.27 us) to slowest abi_native_cross_madd_native_ffi_w (12.44 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 2265.2 ns median (-81.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.49x (fastest 2265.2 ns, slowest 12436.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13636ns | 13565ns | 13399ns | 13531ns | 13914ns | -8.07% |
| abi_native_cross_madd_native_ffi_w | 14833ns | 14702ns | 14405ns | 14674ns | 15286ns | base |
| abi_native_cross_madd_null_entry | 4531ns | 4469ns | 4388ns | 4464ns | 4704ns | -69.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11371ns | 11192ns | 11590ns | -9.35% | 0.003 |
| abi_native_cross_madd_native_ffi_w | 12543ns | 12129ns | 12964ns | base | 0.003 |
| abi_native_cross_madd_null_entry | 2290ns | 2203ns | 2381ns | -81.74% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5228.3 | 11421.2 | 11371.2 | n/a |
| abi_native_cross_madd_native_ffi_w | 25585.2 | 12726.9 | 12543.4 | n/a |
| abi_native_cross_madd_null_entry | 26301.6 | 2414.2 | 2289.9 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.003 | 19.5% |
| abi_native_cross_madd_native_ffi_w | 0.003 | 17.7% |
| abi_native_cross_madd_null_entry | 0.014 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 13636ns | 13636ns | -8.07% |
| abi_native_cross_madd_native_ffi_w | 14833ns | 14833ns | base |
| abi_native_cross_madd_null_entry | 4531ns | 4531ns | -69.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 12436ns | base | --- | [12230, 12964] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11319ns | -1070.2ns (-8.6%) | [-1495, -952]ns | [11205, 11590] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_madd_null_entry | 2265ns | -10138.9ns (-81.5%) | [-10695, -9927]ns | [2224, 2381] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 12331ns | -8.2% | -81.7% |
| 2 | 12129ns | -7.7% | -81.2% |
| 3 | 13023ns | -13.9% | -82.1% |
| 4 | 12904ns | -9.2% | -82.9% |
| 5 | 12446ns | -9.1% | -82.0% |
| 6 | 12427ns | -7.8% | -80.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.227 | moderate- |
| abi_native_cross_madd_native_ffi_w | 0.064 | ok |
| abi_native_cross_madd_null_entry | -0.197 | ok |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 119772.0ns | 11371.2ns | 1053.3% | HIGH |
| abi_native_cross_madd_native_ffi_w | 142101.7ns | 12543.4ns | 1132.9% | HIGH |
| abi_native_cross_madd_null_entry | 114819.4ns | 2289.9ns | 5014.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11192.1-11590.4 ns)
  11192.1 |####################
  11212.0 |####################
  11231.9 |
  11251.8 |
  11271.8 |
  11291.7 |
  11311.6 |########################################
  11331.5 |
  11351.4 |
  11371.3 |
  11391.2 |
  11411.2 |
  11431.1 |
  11451.0 |####################
  11470.9 |
  11490.8 |
  11510.7 |
  11530.7 |
  11550.6 |
  11570.5 |
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 12129.2-12963.5 ns)
  12129.2 |####################
  12170.9 |
  12212.6 |
  12254.4 |
  12296.1 |####################
  12337.8 |
  12379.5 |
  12421.2 |########################################
  12462.9 |
  12504.7 |
  12546.4 |
  12588.1 |
  12629.8 |
  12671.5 |
  12713.2 |
  12755.0 |
  12796.7 |
  12838.4 |
  12880.1 |####################
  12921.8 |
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 2202.9-2380.6 ns)
   2202.9 |########################################
   2211.8 |
   2220.7 |
   2229.6 |
   2238.4 |########################################
   2247.3 |########################################
   2256.2 |
   2265.1 |
   2274.0 |########################################
   2282.9 |
   2291.8 |
   2300.7 |
   2309.5 |
   2318.4 |
   2327.3 |########################################
   2336.2 |
   2345.1 |
   2354.0 |
   2362.9 |
   2371.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1058.0% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=1145.2% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=5029.1% of algo (FFI overhead may distort results)
