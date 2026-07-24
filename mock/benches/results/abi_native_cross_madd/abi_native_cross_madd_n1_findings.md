# abi_native_cross (madd)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_madd_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_madd_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_madd_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_madd_native_ffi_w has the worst median (35.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_madd_null_entry at 4.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_madd_null_entry dominates: 136% faster than the next best (abi_native_cross_madd_inproc_native)

abi_native_cross_madd_null_entry (4.98 us) leads abi_native_cross_madd_inproc_native (11.74 us) by 136%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_madd_null_entry beats baseline by 86% (significant)

abi_native_cross_madd_null_entry is -30.74 us (86%) faster than baseline abi_native_cross_madd_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_madd_native_ffi_w is an outlier: 7.1x slower than the field

abi_native_cross_madd_native_ffi_w (35.59 us) is 7.1x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_madd_inproc_native shows alternating (throttle bounce) (autocorr -0.59)

abi_native_cross_madd_inproc_native's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.1x the fastest

Fastest abi_native_cross_madd_null_entry (4.98 us) to slowest abi_native_cross_madd_native_ffi_w (35.59 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_madd_null_entry** at 4981.9 ns median (-86.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.14x (fastest 4981.9 ns, slowest 35592.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 14000ns | 14010ns | 13845ns | 13980ns | 14108ns | -60.25% |
| abi_native_cross_madd_native_ffi_w | 35223ns | 37842ns | 26762ns | 35864ns | 38491ns | base |
| abi_native_cross_madd_null_entry | 7210ns | 7286ns | 6814ns | 7179ns | 7456ns | -79.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 11712ns | 11613ns | 11785ns | -64.48% | 0.000 |
| abi_native_cross_madd_native_ffi_w | 32976ns | 24521ns | 36230ns | base | 0.000 |
| abi_native_cross_madd_null_entry | 4937ns | 4655ns | 5109ns | -85.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 5342.4 | 11690.7 | 11711.7 | n/a |
| abi_native_cross_madd_native_ffi_w | 25078.2 | 31575.1 | 32975.7 | n/a |
| abi_native_cross_madd_null_entry | 26347.4 | 4996.8 | 4937.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_madd_inproc_native | 0.000 | 39.7% |
| abi_native_cross_madd_native_ffi_w | 0.000 | 13.1% |
| abi_native_cross_madd_null_entry | 0.000 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_madd_inproc_native | 14000ns | 14000ns | -60.25% |
| abi_native_cross_madd_native_ffi_w | 35223ns | 35223ns | base |
| abi_native_cross_madd_null_entry | 7210ns | 7210ns | -79.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_madd_native_ffi_w | 35593ns | base | --- | [27104, 36230] | --- | --- | --- | --- |
| abi_native_cross_madd_inproc_native | 11736ns | -23808.1ns (-66.9%) | [-24540, -15444]ns | [11615, 11785] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_madd_null_entry | 4982ns | -30744.5ns (-86.4%) | [-31351, -22020]ns | [4721, 5109] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_madd_native_ffi_w | abi_native_cross_madd_inproc_native | abi_native_cross_madd_null_entry |
|---|---|---|---|
| 1 | 35665ns | -67.0% | -86.9% |
| 2 | 29687ns | -60.9% | -82.5% |
| 3 | 36671ns | -67.9% | -86.4% |
| 4 | 24521ns | -52.3% | -79.7% |
| 5 | 35790ns | -67.5% | -85.9% |
| 6 | 35520ns | -66.8% | -86.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_madd_inproc_native | -0.592 | HIGH- (thermal bounce) |
| abi_native_cross_madd_native_ffi_w | -0.586 | HIGH- (thermal bounce) |
| abi_native_cross_madd_null_entry | -0.380 | moderate- |

**Consistency summary:**

- **abi_native_cross_madd_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_madd_inproc_native | 123492.1ns | 11711.7ns | 1054.4% | HIGH |
| abi_native_cross_madd_native_ffi_w | 198324.4ns | 32975.7ns | 601.4% | HIGH |
| abi_native_cross_madd_null_entry | 122672.9ns | 4937.1ns | 2484.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_madd_inproc_native (n=6, range 11612.9-11784.8 ns)
  11612.9 |########################################
  11621.5 |
  11630.1 |
  11638.7 |
  11647.3 |
  11655.9 |
  11664.5 |
  11673.1 |
  11681.7 |
  11690.3 |
  11698.8 |####################
  11707.4 |
  11716.0 |
  11724.6 |
  11733.2 |
  11741.8 |
  11750.4 |
  11759.0 |####################
  11767.6 |
  11776.2 |####################
  (0 below, 1 above range)

abi_native_cross_madd_native_ffi_w (n=6, range 24520.8-36230.4 ns)
  24520.8 |####################
  25106.3 |
  25691.8 |
  26277.2 |
  26862.7 |
  27448.2 |
  28033.7 |
  28619.2 |
  29204.6 |####################
  29790.1 |
  30375.6 |
  30961.1 |
  31546.6 |
  32132.0 |
  32717.5 |
  33303.0 |
  33888.5 |
  34474.0 |
  35059.4 |####################
  35644.9 |########################################
  (0 below, 1 above range)

abi_native_cross_madd_null_entry (n=6, range 4655.0-5108.5 ns)
   4655.0 |####################
   4677.7 |
   4700.4 |
   4723.0 |
   4745.7 |
   4768.4 |####################
   4791.1 |
   4813.7 |
   4836.4 |
   4859.1 |
   4881.8 |
   4904.5 |
   4927.1 |
   4949.8 |
   4972.5 |########################################
   4995.2 |
   5017.8 |####################
   5040.5 |
   5063.2 |
   5085.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_madd_inproc_native**: bridge=1054.1% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_native_ffi_w**: bridge=572.7% of algo (FFI overhead may distort results)
- **abi_native_cross_madd_null_entry**: bridge=2472.1% of algo (FFI overhead may distort results)
