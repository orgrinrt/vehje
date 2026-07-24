# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (35.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 4.98 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 134% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (4.98 us) leads abi_native_cross_scatter_inproc_native (11.64 us) by 134%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 86% (significant)

abi_native_cross_scatter_null_entry is -30.22 us (86%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 7.0x slower than the field

abi_native_cross_scatter_native_ffi_w (35.04 us) is 7.0x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_scatter_inproc_native shows alternating (throttle bounce) (autocorr -0.67)

abi_native_cross_scatter_inproc_native's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.0x the fastest

Fastest abi_native_cross_scatter_null_entry (4.98 us) to slowest abi_native_cross_scatter_native_ffi_w (35.04 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 4976.9 ns median (-85.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.04x (fastest 4976.9 ns, slowest 35037.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13908ns | 13936ns | 13581ns | 13862ns | 14139ns | -61.15% |
| abi_native_cross_scatter_native_ffi_w | 35795ns | 37262ns | 29689ns | 36299ns | 38091ns | base |
| abi_native_cross_scatter_null_entry | 7254ns | 7274ns | 6992ns | 7194ns | 7476ns | -79.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11606ns | 11329ns | 11761ns | -65.41% | 0.000 |
| abi_native_cross_scatter_native_ffi_w | 33552ns | 27491ns | 35820ns | base | 0.000 |
| abi_native_cross_scatter_null_entry | 4959ns | 4777ns | 5103ns | -85.22% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5489.8 | 11635.6 | 11605.6 | n/a |
| abi_native_cross_scatter_native_ffi_w | 25715.8 | 33033.2 | 33552.3 | n/a |
| abi_native_cross_scatter_null_entry | 26702.0 | 5008.1 | 4958.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.000 | 41.0% |
| abi_native_cross_scatter_native_ffi_w | 0.000 | 13.6% |
| abi_native_cross_scatter_null_entry | 0.000 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13908ns | 13908ns | -61.15% |
| abi_native_cross_scatter_native_ffi_w | 35795ns | 35795ns | base |
| abi_native_cross_scatter_null_entry | 7254ns | 7254ns | -79.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 35038ns | base | --- | [29799, 35820] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11641ns | -23469.8ns (-67.0%) | [-24330, -18041]ns | [11414, 11761] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_scatter_null_entry | 4977ns | -30217.5ns (-86.2%) | [-30795, -24768]ns | [4796, 5103] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 34512ns | -66.7% | -86.0% |
| 2 | 27491ns | -56.8% | -81.6% |
| 3 | 35760ns | -68.3% | -86.1% |
| 4 | 32107ns | -63.7% | -84.4% |
| 5 | 35881ns | -67.5% | -85.7% |
| 6 | 35563ns | -67.3% | -86.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | -0.672 | HIGH- (thermal bounce) |
| abi_native_cross_scatter_native_ffi_w | -0.390 | moderate- |
| abi_native_cross_scatter_null_entry | -0.418 | moderate- |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 122557.6ns | 11605.6ns | 1056.0% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 199516.7ns | 33552.3ns | 594.6% | HIGH |
| abi_native_cross_scatter_null_entry | 123290.1ns | 4958.8ns | 2486.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11329.2-11761.5 ns)
  11329.2 |#############
  11350.8 |
  11372.4 |
  11394.0 |
  11415.7 |
  11437.3 |
  11458.9 |
  11480.5 |#############
  11502.1 |
  11523.7 |
  11545.3 |
  11566.9 |
  11588.6 |
  11610.2 |
  11631.8 |########################################
  11653.4 |
  11675.0 |
  11696.6 |
  11718.2 |
  11739.8 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 27491.2-35820.4 ns)
  27491.2 |####################
  27907.7 |
  28324.1 |
  28740.6 |
  29157.0 |
  29573.5 |
  29990.0 |
  30406.4 |
  30822.9 |
  31239.3 |
  31655.8 |
  32072.3 |####################
  32488.7 |
  32905.2 |
  33321.6 |
  33738.1 |
  34154.6 |####################
  34571.0 |
  34987.5 |
  35403.9 |########################################
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 4777.1-5103.1 ns)
   4777.1 |########################################
   4793.4 |
   4809.7 |########################################
   4826.0 |
   4842.3 |
   4858.6 |
   4874.9 |
   4891.2 |
   4907.5 |
   4923.8 |
   4940.1 |########################################
   4956.4 |
   4972.7 |
   4989.0 |########################################
   5005.3 |
   5021.6 |
   5037.9 |
   5054.2 |########################################
   5070.5 |
   5086.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1054.7% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=577.3% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=2482.5% of algo (FFI overhead may distort results)
