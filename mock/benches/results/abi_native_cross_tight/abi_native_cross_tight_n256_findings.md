# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (11.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 3.17 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 263% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (3.17 us) leads abi_native_cross_tight_inproc_native (11.50 us) by 263%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 73% (significant)

abi_native_cross_tight_null_entry is -8.48 us (73%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 3.7x slower than the field

abi_native_cross_tight_native_ffi_w (11.61 us) is 3.7x the fastest (3.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_tight_inproc_native shows alternating (throttle bounce) (autocorr -0.59)

abi_native_cross_tight_inproc_native's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.7x the fastest

Fastest abi_native_cross_tight_null_entry (3.17 us) to slowest abi_native_cross_tight_native_ffi_w (11.61 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 3172.1 ns median (-72.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.66x (fastest 3172.1 ns, slowest 11611.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13766ns | 13741ns | 13393ns | 13672ns | 14092ns | -1.10% |
| abi_native_cross_tight_native_ffi_w | 13919ns | 13877ns | 13479ns | 13790ns | 14334ns | base |
| abi_native_cross_tight_null_entry | 5429ns | 5416ns | 5253ns | 5387ns | 5580ns | -60.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11494ns | 11226ns | 11750ns | -1.47% | 0.022 |
| abi_native_cross_tight_native_ffi_w | 11666ns | 11265ns | 12057ns | base | 0.022 |
| abi_native_cross_tight_null_entry | 3180ns | 3078ns | 3263ns | -72.74% | 0.081 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5382.1 | 11573.4 | 11493.6 | n/a |
| abi_native_cross_tight_native_ffi_w | 25526.2 | 11794.8 | 11665.6 | n/a |
| abi_native_cross_tight_null_entry | 27137.0 | 3180.5 | 3179.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.022 | 26.8% |
| abi_native_cross_tight_native_ffi_w | 0.022 | 26.5% |
| abi_native_cross_tight_null_entry | 0.081 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13766ns | 13766ns | -1.10% |
| abi_native_cross_tight_native_ffi_w | 13919ns | 13919ns | base |
| abi_native_cross_tight_null_entry | 5429ns | 5429ns | -60.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 11611ns | base | --- | [11328, 12057] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11500ns | no significant difference | [-536, +165]ns | [11230, 11750] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_tight_null_entry | 3172ns | -8478.4ns (-73.0%) | [-8914, -8065]ns | [3104, 3263] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 11679ns | -0.2% | -73.6% |
| 2 | 11597ns | -2.2% | -73.0% |
| 3 | 11265ns | -0.3% | -71.3% |
| 4 | 12435ns | -5.5% | -74.2% |
| 5 | 11626ns | -3.4% | -73.1% |
| 6 | 11392ns | +3.1% | -71.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.586 | HIGH- (thermal bounce) |
| abi_native_cross_tight_native_ffi_w | -0.361 | moderate- |
| abi_native_cross_tight_null_entry | -0.104 | ok |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 121711.0ns | 11493.6ns | 1058.9% | HIGH |
| abi_native_cross_tight_native_ffi_w | 142253.9ns | 11665.6ns | 1219.4% | HIGH |
| abi_native_cross_tight_null_entry | 120427.6ns | 3179.7ns | 3787.4% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11226.2-11750.4 ns)
  11226.2 |########################################
  11252.4 |
  11278.6 |
  11304.8 |
  11331.0 |####################
  11357.2 |
  11383.5 |
  11409.7 |
  11435.9 |
  11462.1 |
  11488.3 |
  11514.5 |
  11540.7 |
  11566.9 |
  11593.1 |
  11619.4 |
  11645.6 |####################
  11671.8 |
  11698.0 |
  11724.2 |####################
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 11264.6-12057.3 ns)
  11264.6 |########################################
  11304.2 |
  11343.9 |
  11383.5 |########################################
  11423.1 |
  11462.8 |
  11502.4 |
  11542.0 |
  11581.7 |########################################
  11621.3 |########################################
  11661.0 |########################################
  11700.6 |
  11740.2 |
  11779.9 |
  11819.5 |
  11859.1 |
  11898.8 |
  11938.4 |
  11978.0 |
  12017.7 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 3077.5-3263.1 ns)
   3077.5 |########################################
   3086.8 |
   3096.1 |
   3105.3 |
   3114.6 |
   3123.9 |########################################
   3133.2 |########################################
   3142.5 |
   3151.7 |
   3161.0 |
   3170.3 |
   3179.6 |
   3188.9 |
   3198.1 |
   3207.4 |########################################
   3216.7 |
   3226.0 |
   3235.3 |########################################
   3244.5 |
   3253.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1059.7% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1226.5% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=3801.7% of algo (FFI overhead may distort results)
