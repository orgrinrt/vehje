# abi_native_cross (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_wideselect_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_wideselect_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_wideselect_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_wideselect_native_ffi_w has the worst median (31.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_wideselect_null_entry at 4.81 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_wideselect_null_entry dominates: 142% faster than the next best (abi_native_cross_wideselect_inproc_native)

abi_native_cross_wideselect_null_entry (4.81 us) leads abi_native_cross_wideselect_inproc_native (11.65 us) by 142%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_wideselect_null_entry beats baseline by 85% (significant)

abi_native_cross_wideselect_null_entry is -26.37 us (85%) faster than baseline abi_native_cross_wideselect_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_wideselect_native_ffi_w is an outlier: 6.5x slower than the field

abi_native_cross_wideselect_native_ffi_w (31.16 us) is 6.5x the fastest (4.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_wideselect_native_ffi_w shows alternating (throttle bounce) (autocorr -0.72)

abi_native_cross_wideselect_native_ffi_w's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.5x the fastest

Fastest abi_native_cross_wideselect_null_entry (4.81 us) to slowest abi_native_cross_wideselect_native_ffi_w (31.16 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_wideselect_null_entry** at 4810.4 ns median (-84.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.48x (fastest 4810.4 ns, slowest 31164.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13956ns | 13979ns | 13318ns | 13963ns | 14265ns | -57.62% |
| abi_native_cross_wideselect_native_ffi_w | 32928ns | 33379ns | 29577ns | 32285ns | 35568ns | base |
| abi_native_cross_wideselect_null_entry | 7103ns | 7040ns | 6943ns | 7018ns | 7309ns | -78.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 11629ns | 11162ns | 11846ns | -62.10% | 0.000 |
| abi_native_cross_wideselect_native_ffi_w | 30687ns | 27381ns | 33262ns | base | 0.000 |
| abi_native_cross_wideselect_null_entry | 4864ns | 4760ns | 5017ns | -84.15% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 5275.1 | 11676.7 | 11629.4 | n/a |
| abi_native_cross_wideselect_native_ffi_w | 25807.1 | 30633.2 | 30686.6 | n/a |
| abi_native_cross_wideselect_null_entry | 27383.6 | 4919.1 | 4863.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_native_cross_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | 0.000 | 40.9% |
| abi_native_cross_wideselect_native_ffi_w | 0.000 | 15.3% |
| abi_native_cross_wideselect_null_entry | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 13956ns | 13956ns | -57.62% |
| abi_native_cross_wideselect_native_ffi_w | 32928ns | 32928ns | base |
| abi_native_cross_wideselect_null_entry | 7103ns | 7103ns | -78.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_wideselect_native_ffi_w | 31165ns | base | --- | [27634, 33262] | --- | --- | --- | --- |
| abi_native_cross_wideselect_inproc_native | 11650ns | -19633.3ns (-63.0%) | [-21538, -16000]ns | [11392, 11846] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_wideselect_null_entry | 4810ns | -26365.2ns (-84.6%) | [-28379, -22725]ns | [4764, 5017] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_wideselect_native_ffi_w | abi_native_cross_wideselect_inproc_native | abi_native_cross_wideselect_null_entry |
|---|---|---|---|
| 1 | 30828ns | -63.8% | -84.6% |
| 2 | 31501ns | -62.2% | -84.6% |
| 3 | 31997ns | -63.1% | -85.1% |
| 4 | 27886ns | -58.2% | -82.9% |
| 5 | 34526ns | -66.2% | -85.5% |
| 6 | 27381ns | -57.6% | -81.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_wideselect_inproc_native | -0.250 | moderate- |
| abi_native_cross_wideselect_native_ffi_w | -0.722 | HIGH- (thermal bounce) |
| abi_native_cross_wideselect_null_entry | 0.333 | moderate+ |

**Consistency summary:**

- **abi_native_cross_wideselect_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_wideselect_inproc_native | 122204.5ns | 11629.4ns | 1050.8% | HIGH |
| abi_native_cross_wideselect_native_ffi_w | 189395.1ns | 30686.6ns | 617.2% | HIGH |
| abi_native_cross_wideselect_null_entry | 124466.4ns | 4863.7ns | 2559.1% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_wideselect_inproc_native (n=6, range 11162.1-11846.0 ns)
  11162.1 |####################
  11196.3 |
  11230.5 |
  11264.7 |
  11298.9 |
  11333.1 |
  11367.3 |
  11401.5 |
  11435.7 |
  11469.9 |
  11504.1 |
  11538.3 |
  11572.5 |
  11606.7 |####################
  11640.9 |########################################
  11675.1 |
  11709.3 |
  11743.5 |
  11777.7 |####################
  11811.9 |
  (0 below, 1 above range)

abi_native_cross_wideselect_native_ffi_w (n=6, range 27381.2-33261.6 ns)
  27381.2 |########################################
  27675.2 |########################################
  27969.2 |
  28263.3 |
  28557.3 |
  28851.3 |
  29145.3 |
  29439.4 |
  29733.4 |
  30027.4 |
  30321.4 |
  30615.4 |########################################
  30909.5 |
  31203.5 |
  31497.5 |########################################
  31791.5 |########################################
  32085.6 |
  32379.6 |
  32673.6 |
  32967.6 |
  (0 below, 1 above range)

abi_native_cross_wideselect_null_entry (n=6, range 4760.0-5016.6 ns)
   4760.0 |########################################
   4772.8 |####################
   4785.7 |
   4798.5 |
   4811.3 |
   4824.2 |
   4837.0 |####################
   4849.8 |
   4862.7 |
   4875.5 |
   4888.3 |
   4901.2 |
   4914.0 |
   4926.8 |
   4939.7 |
   4952.5 |
   4965.3 |
   4978.2 |
   4991.0 |####################
   5003.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_wideselect_inproc_native**: bridge=1055.5% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_native_ffi_w**: bridge=604.9% of algo (FFI overhead may distort results)
- **abi_native_cross_wideselect_null_entry**: bridge=2582.8% of algo (FFI overhead may distort results)
