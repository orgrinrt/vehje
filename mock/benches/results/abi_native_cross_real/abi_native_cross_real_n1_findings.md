# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (32.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 4.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 138% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (4.86 us) leads abi_native_cross_real_inproc_native (11.59 us) by 138%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 85% (significant)

abi_native_cross_real_null_entry is -28.03 us (85%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 6.8x slower than the field

abi_native_cross_real_native_ffi_w (32.90 us) is 6.8x the fastest (4.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_null_entry shows alternating (throttle bounce) (autocorr -0.55)

abi_native_cross_real_null_entry's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.8x the fastest

Fastest abi_native_cross_real_null_entry (4.86 us) to slowest abi_native_cross_real_native_ffi_w (32.90 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 4859.8 ns median (-85.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.77x (fastest 4859.8 ns, slowest 32897.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13942ns | 13839ns | 13691ns | 13813ns | 14259ns | -59.37% |
| abi_native_cross_real_native_ffi_w | 34310ns | 35135ns | 29801ns | 33657ns | 37543ns | base |
| abi_native_cross_real_null_entry | 7172ns | 7141ns | 7059ns | 7120ns | 7307ns | -79.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11610ns | 11370ns | 11795ns | -63.78% | 0.000 |
| abi_native_cross_real_native_ffi_w | 32052ns | 27505ns | 35302ns | base | 0.000 |
| abi_native_cross_real_null_entry | 4879ns | 4795ns | 4962ns | -84.78% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5649.9 | 11646.5 | 11609.5 | n/a |
| abi_native_cross_real_native_ffi_w | 27632.9 | 32148.3 | 32051.8 | n/a |
| abi_native_cross_real_null_entry | 29343.8 | 4964.8 | 4879.0 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 41.4% |
| abi_native_cross_real_native_ffi_w | 0.000 | 14.6% |
| abi_native_cross_real_null_entry | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13942ns | 13942ns | -59.37% |
| abi_native_cross_real_native_ffi_w | 34310ns | 34310ns | base |
| abi_native_cross_real_null_entry | 7172ns | 7172ns | -79.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 32898ns | base | --- | [27955, 35302] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11586ns | -21213.8ns (-64.5%) | [-23715, -16398]ns | [11447, 11795] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 4860ns | -28025.8ns (-85.2%) | [-30479, -23013]ns | [4816, 4962] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 35647ns | -68.1% | -86.5% |
| 2 | 27505ns | -58.1% | -81.7% |
| 3 | 28405ns | -59.2% | -83.0% |
| 4 | 34697ns | -66.0% | -85.9% |
| 5 | 34958ns | -66.2% | -86.1% |
| 6 | 31099ns | -62.8% | -84.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.355 | moderate+ |
| abi_native_cross_real_native_ffi_w | -0.071 | ok |
| abi_native_cross_real_null_entry | -0.549 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 122984.5ns | 11609.5ns | 1059.3% | HIGH |
| abi_native_cross_real_native_ffi_w | 196304.6ns | 32051.8ns | 612.5% | HIGH |
| abi_native_cross_real_null_entry | 125955.8ns | 4879.0ns | 2581.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11369.6-11795.2 ns)
  11369.6 |####################
  11390.9 |
  11412.2 |
  11433.4 |
  11454.7 |
  11476.0 |
  11497.3 |
  11518.6 |####################
  11539.8 |
  11561.1 |
  11582.4 |########################################
  11603.7 |
  11625.0 |
  11646.2 |
  11667.5 |
  11688.8 |
  11710.1 |
  11731.4 |
  11752.6 |
  11773.9 |####################
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 27505.0-35302.5 ns)
  27505.0 |########################################
  27894.9 |
  28284.8 |########################################
  28674.6 |
  29064.5 |
  29454.4 |
  29844.2 |
  30234.1 |
  30624.0 |
  31013.9 |########################################
  31403.8 |
  31793.6 |
  32183.5 |
  32573.4 |
  32963.2 |
  33353.1 |
  33743.0 |
  34132.9 |
  34522.8 |########################################
  34912.6 |########################################
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 4795.0-4961.7 ns)
   4795.0 |########################################
   4803.3 |
   4811.7 |
   4820.0 |
   4828.3 |########################################
   4836.7 |
   4845.0 |########################################
   4853.3 |
   4861.7 |########################################
   4870.0 |########################################
   4878.4 |
   4886.7 |
   4895.0 |
   4903.4 |
   4911.7 |
   4920.0 |
   4928.4 |
   4936.7 |
   4945.0 |
   4953.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1056.5% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=603.3% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=2582.0% of algo (FFI overhead may distort results)
