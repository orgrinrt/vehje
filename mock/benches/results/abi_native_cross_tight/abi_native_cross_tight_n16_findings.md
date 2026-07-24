# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (12.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 357% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (2.51 us) leads abi_native_cross_tight_inproc_native (11.47 us) by 357%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 79% (significant)

abi_native_cross_tight_null_entry is -9.71 us (79%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_tight_native_ffi_w (12.23 us) is 4.9x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_tight_native_ffi_w shows alternating (throttle bounce) (autocorr -0.50)

abi_native_cross_tight_native_ffi_w's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_tight_null_entry (2.51 us) to slowest abi_native_cross_tight_native_ffi_w (12.23 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 2511.6 ns median (-79.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.87x (fastest 2511.6 ns, slowest 12228.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13881ns | 13732ns | 13612ns | 13697ns | 14291ns | -4.02% |
| abi_native_cross_tight_native_ffi_w | 14462ns | 14501ns | 14215ns | 14454ns | 14599ns | base |
| abi_native_cross_tight_null_entry | 4760ns | 4786ns | 4622ns | 4741ns | 4857ns | -67.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11554ns | 11290ns | 11884ns | -5.18% | 0.001 |
| abi_native_cross_tight_native_ffi_w | 12186ns | 11994ns | 12270ns | base | 0.001 |
| abi_native_cross_tight_null_entry | 2521ns | 2460ns | 2588ns | -79.31% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5326.8 | 11646.2 | 11554.4 | n/a |
| abi_native_cross_tight_native_ffi_w | 25351.6 | 12347.6 | 12185.6 | n/a |
| abi_native_cross_tight_null_entry | 26681.7 | 2592.2 | 2520.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.001 | 21.4% |
| abi_native_cross_tight_native_ffi_w | 0.001 | 20.1% |
| abi_native_cross_tight_null_entry | 0.006 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13881ns | 13881ns | -4.02% |
| abi_native_cross_tight_native_ffi_w | 14462ns | 14462ns | base |
| abi_native_cross_tight_null_entry | 4760ns | 4760ns | -67.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 12229ns | base | --- | [12058, 12270] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11473ns | -766.7ns (-6.3%) | [-954, -173]ns | [11306, 11884] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_tight_null_entry | 2512ns | -9708.8ns (-79.4%) | [-9790, -9496]ns | [2462, 2588] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 12251ns | -5.9% | -79.9% |
| 2 | 12230ns | -7.7% | -79.9% |
| 3 | 12228ns | -6.6% | -78.9% |
| 4 | 11994ns | -0.4% | -78.3% |
| 5 | 12290ns | -7.9% | -79.6% |
| 6 | 12121ns | -2.5% | -79.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.443 | moderate- |
| abi_native_cross_tight_native_ffi_w | -0.500 | HIGH- (thermal bounce) |
| abi_native_cross_tight_null_entry | 0.187 | ok |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 122104.2ns | 11554.4ns | 1056.8% | HIGH |
| abi_native_cross_tight_native_ffi_w | 140910.3ns | 12185.6ns | 1156.4% | HIGH |
| abi_native_cross_tight_null_entry | 117620.8ns | 2520.7ns | 4666.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11290.0-11884.4 ns)
  11290.0 |########################################
  11319.7 |########################################
  11349.4 |
  11379.2 |
  11408.9 |########################################
  11438.6 |
  11468.3 |
  11498.0 |########################################
  11527.7 |
  11557.5 |
  11587.2 |
  11616.9 |
  11646.6 |
  11676.3 |
  11706.0 |
  11735.8 |
  11765.5 |
  11795.2 |########################################
  11824.9 |
  11854.6 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 11993.8-12270.4 ns)
  11993.8 |########################################
  12007.6 |
  12021.5 |
  12035.3 |
  12049.1 |
  12063.0 |
  12076.8 |
  12090.6 |
  12104.4 |
  12118.3 |########################################
  12132.1 |
  12145.9 |
  12159.8 |
  12173.6 |
  12187.4 |
  12201.2 |
  12215.1 |########################################
  12228.9 |########################################
  12242.7 |########################################
  12256.6 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 2460.4-2588.3 ns)
   2460.4 |########################################
   2466.8 |
   2473.2 |
   2479.6 |
   2486.0 |
   2492.4 |
   2498.8 |####################
   2505.2 |
   2511.6 |
   2518.0 |####################
   2524.4 |
   2530.7 |
   2537.1 |
   2543.5 |
   2549.9 |
   2556.3 |
   2562.7 |
   2569.1 |
   2575.5 |####################
   2581.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1060.3% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1151.7% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=4669.1% of algo (FFI overhead may distort results)
