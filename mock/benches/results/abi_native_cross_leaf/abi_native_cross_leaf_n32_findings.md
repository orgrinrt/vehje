# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (12.33 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 2.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 412% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (2.23 us) leads abi_native_cross_leaf_inproc_native (11.41 us) by 412%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 82% (significant)

abi_native_cross_leaf_null_entry is -10.06 us (82%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 5.5x slower than the field

abi_native_cross_leaf_native_ffi_w (12.33 us) is 5.5x the fastest (2.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_leaf_inproc_native shows alternating (throttle bounce) (autocorr -0.62)

abi_native_cross_leaf_inproc_native's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest abi_native_cross_leaf_null_entry (2.23 us) to slowest abi_native_cross_leaf_native_ffi_w (12.33 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 2226.7 ns median (-81.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.54x (fastest 2226.7 ns, slowest 12327.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13794ns | 13721ns | 13476ns | 13669ns | 14141ns | -5.65% |
| abi_native_cross_leaf_native_ffi_w | 14620ns | 14610ns | 14442ns | 14567ns | 14788ns | base |
| abi_native_cross_leaf_null_entry | 4474ns | 4415ns | 4406ns | 4412ns | 4601ns | -69.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11486ns | 11295ns | 11746ns | -6.89% | 0.003 |
| abi_native_cross_leaf_native_ffi_w | 12335ns | 12149ns | 12479ns | base | 0.003 |
| abi_native_cross_leaf_null_entry | 2247ns | 2212ns | 2296ns | -81.79% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5495.4 | 11494.4 | 11485.6 | n/a |
| abi_native_cross_leaf_native_ffi_w | 25215.5 | 12517.3 | 12334.9 | n/a |
| abi_native_cross_leaf_null_entry | 26412.2 | 2358.2 | 2246.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.003 | 19.4% |
| abi_native_cross_leaf_native_ffi_w | 0.003 | 17.9% |
| abi_native_cross_leaf_null_entry | 0.014 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13794ns | 13794ns | -5.65% |
| abi_native_cross_leaf_native_ffi_w | 14620ns | 14620ns | base |
| abi_native_cross_leaf_null_entry | 4474ns | 4474ns | -69.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 12327ns | base | --- | [12199, 12479] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11408ns | -896.5ns (-7.3%) | [-1104, -547]ns | [11303, 11746] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_leaf_null_entry | 2227ns | -10061.7ns (-81.6%) | [-10250, -9953]ns | [2217, 2296] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 12317ns | -8.3% | -82.0% |
| 2 | 12460ns | -7.7% | -82.2% |
| 3 | 12149ns | -6.8% | -81.7% |
| 4 | 12337ns | -4.2% | -80.9% |
| 5 | 12498ns | -9.5% | -82.1% |
| 6 | 12249ns | -4.7% | -81.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.620 | HIGH- (thermal bounce) |
| abi_native_cross_leaf_native_ffi_w | -0.469 | moderate- |
| abi_native_cross_leaf_null_entry | -0.170 | ok |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 121475.8ns | 11485.6ns | 1057.6% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 141412.8ns | 12334.9ns | 1146.4% | HIGH |
| abi_native_cross_leaf_null_entry | 114405.1ns | 2246.7ns | 5092.2% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11294.6-11745.6 ns)
  11294.6 |########################################
  11317.2 |####################
  11339.7 |
  11362.3 |
  11384.8 |
  11407.4 |
  11429.9 |
  11452.5 |
  11475.0 |####################
  11497.6 |
  11520.1 |
  11542.7 |
  11565.2 |
  11587.8 |
  11610.3 |
  11632.9 |
  11655.4 |####################
  11678.0 |
  11700.5 |
  11723.1 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 12149.2-12478.8 ns)
  12149.2 |########################################
  12165.7 |
  12182.2 |
  12198.6 |
  12215.1 |
  12231.6 |
  12248.1 |########################################
  12264.5 |
  12281.0 |
  12297.5 |
  12314.0 |########################################
  12330.5 |########################################
  12346.9 |
  12363.4 |
  12379.9 |
  12396.4 |
  12412.8 |
  12429.3 |
  12445.8 |########################################
  12462.3 |
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 2212.1-2296.5 ns)
   2212.1 |####################
   2216.3 |
   2220.5 |########################################
   2224.8 |
   2229.0 |####################
   2233.2 |####################
   2237.4 |
   2241.6 |
   2245.9 |
   2250.1 |
   2254.3 |
   2258.5 |
   2262.7 |
   2267.0 |
   2271.2 |
   2275.4 |
   2279.6 |
   2283.8 |
   2288.1 |
   2292.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1056.8% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1149.8% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=5113.0% of algo (FFI overhead may distort results)
