# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (13.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 3.97 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 191% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (3.97 us) leads abi_native_cross_wideselect_inproc_native (11.54 us) by 191%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 71% (significant)

abi_native_cross_wideselect_null_entry is -9.81 us (71%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 3.5x slower than the field

abi_native_cross_wideselect_native_ffi_w (13.82 us) is 3.5x the fastest (3.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.5x the fastest

Fastest abi_native_cross_wideselect_null_entry (3.97 us) to slowest abi_native_cross_wideselect_native_ffi_w (13.82 us): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 3971.1 ns median (-71.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.48x (fastest 3971.1 ns, slowest 13821.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13802ns | 13764ns | 13359ns | 13693ns | 14188ns | -14.05% |
| abi_native_cross_wideselect_native_ffi_w | 16058ns | 16118ns | 15773ns | 16021ns | 16255ns | base |
| abi_native_cross_wideselect_null_entry | 6282ns | 6229ns | 6081ns | 6188ns | 6524ns | -60.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11556ns | 11221ns | 11850ns | -16.20% | 0.000 |
| abi_native_cross_wideselect_native_ffi_w | 13790ns | 13577ns | 13955ns | base | 0.000 |
| abi_native_cross_wideselect_null_entry | 4006ns | 3883ns | 4161ns | -70.95% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5374.2 | 11501.7 | 11556.2 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 26498.4 | 13889.4 | 13789.9 | n/a |
| abi_native_cross_wideselect_null_entry | 29383.7 | 4193.1 | 4006.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.000 | 33.6% |
| abi_native_cross_wideselect_native_ffi_w | 0.000 | 28.1% |
| abi_native_cross_wideselect_null_entry | 0.001 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13802ns | 13802ns | -14.05% |
| abi_native_cross_wideselect_native_ffi_w | 16058ns | 16058ns | base |
| abi_native_cross_wideselect_null_entry | 6282ns | 6282ns | -60.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 13821ns | base | --- | [13594, 13955] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11545ns | -2272.9ns (-16.4%) | [-2362, -2066]ns | [11274, 11850] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_wideselect_null_entry | 3971ns | -9807.3ns (-71.0%) | [-10066, -9478]ns | [3887, 4161] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 13990ns | -16.2% | -72.2% |
| 2 | 13915ns | -14.0% | -72.0% |
| 3 | 13611ns | -16.8% | -70.7% |
| 4 | 13919ns | -15.7% | -70.7% |
| 5 | 13577ns | -17.3% | -68.7% |
| 6 | 13728ns | -17.2% | -71.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.142 | ok |
| abi_native_cross_wideselect_native_ffi_w | -0.225 | moderate- |
| abi_native_cross_wideselect_null_entry | 0.212 | moderate+ |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 121678.0ns | 11556.2ns | 1052.9% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 149488.8ns | 13789.9ns | 1084.0% | HIGH |
| abi_native_cross_wideselect_null_entry | 124070.6ns | 4006.2ns | 3096.9% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11221.2-11850.4 ns)
  11221.2 |####################
  11252.7 |
  11284.1 |
  11315.6 |####################
  11347.0 |####################
  11378.5 |
  11410.0 |
  11441.4 |
  11472.9 |
  11504.3 |
  11535.8 |
  11567.3 |
  11598.7 |
  11630.2 |
  11661.6 |
  11693.1 |
  11724.6 |########################################
  11756.0 |
  11787.5 |
  11818.9 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 13576.7-13954.6 ns)
  13576.7 |########################################
  13595.6 |########################################
  13614.5 |
  13633.4 |
  13652.3 |
  13671.2 |
  13690.1 |
  13709.0 |########################################
  13727.9 |
  13746.8 |
  13765.6 |
  13784.5 |
  13803.4 |
  13822.3 |
  13841.2 |
  13860.1 |
  13879.0 |
  13897.9 |########################################
  13916.8 |########################################
  13935.7 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 3882.9-4161.1 ns)
   3882.9 |########################################
   3896.8 |
   3910.7 |
   3924.6 |
   3938.5 |
   3952.4 |####################
   3966.3 |
   3980.3 |####################
   3994.2 |
   4008.1 |
   4022.0 |
   4035.9 |
   4049.8 |
   4063.7 |####################
   4077.6 |
   4091.5 |
   4105.4 |
   4119.3 |
   4133.2 |
   4147.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1055.0% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=1084.5% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=3135.6% of algo (FFI overhead may distort results)
