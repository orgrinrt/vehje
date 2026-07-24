# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (35.73 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 4.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 134% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (4.98 us) leads abi_native_cross_leaf_inproc_native (11.67 us) by 134%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 86% (significant)

abi_native_cross_leaf_null_entry is -30.75 us (86%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 7.2x slower than the field

abi_native_cross_leaf_native_ffi_w (35.73 us) is 7.2x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_leaf_inproc_native shows alternating (throttle bounce) (autocorr -0.74)

abi_native_cross_leaf_inproc_native's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.2x the fastest

Fastest abi_native_cross_leaf_null_entry (4.98 us) to slowest abi_native_cross_leaf_native_ffi_w (35.73 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 4982.9 ns median (-86.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.17x (fastest 4982.9 ns, slowest 35734.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13896ns | 13925ns | 13532ns | 13812ns | 14205ns | -63.37% |
| abi_native_cross_leaf_native_ffi_w | 37933ns | 38010ns | 37666ns | 37953ns | 38038ns | base |
| abi_native_cross_leaf_null_entry | 7254ns | 7294ns | 6955ns | 7203ns | 7478ns | -80.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11608ns | 11310ns | 11822ns | -67.46% | 0.000 |
| abi_native_cross_leaf_native_ffi_w | 35676ns | 35436ns | 35786ns | base | 0.000 |
| abi_native_cross_leaf_null_entry | 4956ns | 4748ns | 5118ns | -86.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5212.6 | 11617.4 | 11607.9 | n/a |
| abi_native_cross_leaf_native_ffi_w | 25692.7 | 33732.5 | 35676.5 | n/a |
| abi_native_cross_leaf_null_entry | 27683.6 | 5270.3 | 4955.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.000 | 40.7% |
| abi_native_cross_leaf_native_ffi_w | 0.000 | 13.3% |
| abi_native_cross_leaf_null_entry | 0.000 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13896ns | 13896ns | -63.37% |
| abi_native_cross_leaf_native_ffi_w | 37933ns | 37933ns | base |
| abi_native_cross_leaf_null_entry | 7254ns | 7254ns | -80.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 35734ns | base | --- | [35509, 35786] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11671ns | -24101.4ns (-67.4%) | [-24346, -23759]ns | [11331, 11822] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_leaf_null_entry | 4983ns | -30751.2ns (-86.1%) | [-30910, -30501]ns | [4767, 5118] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 35770ns | -68.3% | -86.6% |
| 2 | 35436ns | -66.4% | -85.6% |
| 3 | 35582ns | -68.2% | -86.7% |
| 4 | 35758ns | -67.1% | -86.1% |
| 5 | 35802ns | -67.6% | -85.6% |
| 6 | 35710ns | -67.1% | -86.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.738 | HIGH- (thermal bounce) |
| abi_native_cross_leaf_native_ffi_w | 0.070 | ok |
| abi_native_cross_leaf_null_entry | -0.367 | moderate- |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 122136.3ns | 11607.9ns | 1052.2% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 204011.2ns | 35676.5ns | 571.8% | HIGH |
| abi_native_cross_leaf_null_entry | 123928.9ns | 4955.8ns | 2500.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11310.0-11821.9 ns)
  11310.0 |########################################
  11335.6 |########################################
  11361.2 |
  11386.8 |
  11412.4 |
  11438.0 |
  11463.6 |
  11489.2 |
  11514.8 |
  11540.4 |
  11566.0 |
  11591.5 |########################################
  11617.1 |
  11642.7 |
  11668.3 |
  11693.9 |
  11719.5 |########################################
  11745.1 |########################################
  11770.7 |
  11796.3 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 35436.2-35785.8 ns)
  35436.2 |########################################
  35453.7 |
  35471.2 |
  35488.6 |
  35506.1 |
  35523.6 |
  35541.1 |
  35558.6 |
  35576.1 |########################################
  35593.5 |
  35611.0 |
  35628.5 |
  35646.0 |
  35663.5 |
  35681.0 |
  35698.4 |########################################
  35715.9 |
  35733.4 |
  35750.9 |########################################
  35768.4 |########################################
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 4748.3-5117.9 ns)
   4748.3 |########################################
   4766.8 |########################################
   4785.3 |
   4803.7 |
   4822.2 |
   4840.7 |
   4859.2 |
   4877.7 |
   4896.1 |
   4914.6 |
   4933.1 |
   4951.6 |
   4970.1 |########################################
   4988.5 |########################################
   5007.0 |
   5025.5 |
   5044.0 |
   5062.5 |
   5080.9 |########################################
   5099.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1054.8% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=570.2% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=2483.1% of algo (FFI overhead may distort results)
