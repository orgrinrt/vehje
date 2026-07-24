# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (12.35 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 2.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 405% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (2.30 us) leads abi_native_cross_scatter_inproc_native (11.60 us) by 405%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 82% (significant)

abi_native_cross_scatter_null_entry is -10.10 us (82%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 5.4x slower than the field

abi_native_cross_scatter_native_ffi_w (12.35 us) is 5.4x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_scatter_inproc_native shows alternating (throttle bounce) (autocorr -0.56)

abi_native_cross_scatter_inproc_native's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.4x the fastest

Fastest abi_native_cross_scatter_null_entry (2.30 us) to slowest abi_native_cross_scatter_native_ffi_w (12.35 us): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 2295.4 ns median (-81.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.38x (fastest 2295.4 ns, slowest 12347.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14057ns | 13904ns | 13518ns | 13872ns | 14603ns | -4.31% |
| abi_native_cross_scatter_native_ffi_w | 14690ns | 14642ns | 14595ns | 14630ns | 14829ns | base |
| abi_native_cross_scatter_null_entry | 4626ns | 4628ns | 4461ns | 4577ns | 4783ns | -68.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11726ns | 11269ns | 12208ns | -5.44% | 0.003 |
| abi_native_cross_scatter_native_ffi_w | 12401ns | 12317ns | 12532ns | base | 0.003 |
| abi_native_cross_scatter_null_entry | 2310ns | 2226ns | 2399ns | -81.37% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5184.1 | 11772.7 | 11726.1 | n/a |
| abi_native_cross_scatter_native_ffi_w | 25589.5 | 12570.0 | 12400.5 | n/a |
| abi_native_cross_scatter_null_entry | 26560.2 | 2426.4 | 2310.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.003 | 19.2% |
| abi_native_cross_scatter_native_ffi_w | 0.003 | 18.0% |
| abi_native_cross_scatter_null_entry | 0.014 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 14057ns | 14057ns | -4.31% |
| abi_native_cross_scatter_native_ffi_w | 14690ns | 14690ns | base |
| abi_native_cross_scatter_null_entry | 4626ns | 4626ns | -68.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 12347ns | base | --- | [12322, 12532] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11600ns | -729.4ns (-5.9%) | [-970, -324]ns | [11370, 12208] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_scatter_null_entry | 2295ns | -10096.2ns (-81.8%) | [-10201, -9974]ns | [2236, 2399] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 12363ns | -7.2% | -81.8% |
| 2 | 12483ns | -1.8% | -82.2% |
| 3 | 12317ns | -8.5% | -81.0% |
| 4 | 12331ns | -5.9% | -80.8% |
| 5 | 12581ns | -3.4% | -80.6% |
| 6 | 12328ns | -5.9% | -81.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | -0.559 | HIGH- (thermal bounce) |
| abi_native_cross_scatter_native_ffi_w | -0.515 | HIGH- (thermal bounce) |
| abi_native_cross_scatter_null_entry | 0.119 | ok |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 121644.2ns | 11726.1ns | 1037.4% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 141568.6ns | 12400.5ns | 1141.6% | HIGH |
| abi_native_cross_scatter_null_entry | 115198.5ns | 2310.1ns | 4986.7% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11269.2-12208.0 ns)
  11269.2 |########################################
  11316.1 |
  11363.1 |
  11410.0 |
  11457.0 |########################################
  11503.9 |
  11550.8 |########################################
  11597.8 |########################################
  11644.7 |
  11691.6 |
  11738.6 |
  11785.5 |
  11832.5 |
  11879.4 |
  11926.3 |
  11973.3 |
  12020.2 |
  12067.1 |
  12114.1 |########################################
  12161.0 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 12316.7-12532.0 ns)
  12316.7 |####################
  12327.5 |########################################
  12338.2 |
  12349.0 |
  12359.8 |####################
  12370.5 |
  12381.3 |
  12392.1 |
  12402.8 |
  12413.6 |
  12424.4 |
  12435.1 |
  12445.9 |
  12456.7 |
  12467.4 |
  12478.2 |####################
  12489.0 |
  12499.7 |
  12510.5 |
  12521.3 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 2226.2-2399.2 ns)
   2226.2 |########################################
   2234.8 |
   2243.5 |########################################
   2252.1 |########################################
   2260.8 |
   2269.4 |
   2278.1 |
   2286.7 |
   2295.4 |
   2304.0 |
   2312.7 |
   2321.3 |
   2330.0 |########################################
   2338.6 |
   2347.3 |
   2355.9 |########################################
   2364.6 |
   2373.2 |
   2381.9 |
   2390.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1052.0% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1148.6% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=5011.3% of algo (FFI overhead may distort results)
