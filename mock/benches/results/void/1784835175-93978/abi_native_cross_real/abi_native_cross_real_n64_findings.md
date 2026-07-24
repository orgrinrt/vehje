# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (11.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 348% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.51 us) leads abi_native_cross_real_inproc_native (11.21 us) by 348%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 79% (significant)

abi_native_cross_real_null_entry is -9.38 us (79%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 4.7x slower than the field

abi_native_cross_real_native_ffi_w (11.90 us) is 4.7x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_native_ffi_w shows alternating (throttle bounce) (autocorr -0.72)

abi_native_cross_real_native_ffi_w's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.7x the fastest

Fastest abi_native_cross_real_null_entry (2.51 us) to slowest abi_native_cross_real_native_ffi_w (11.90 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2505.8 ns median (-78.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.75x (fastest 2505.8 ns, slowest 11899.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13645ns | 13511ns | 13196ns | 13455ns | 14153ns | -3.74% |
| abi_native_cross_real_native_ffi_w | 14175ns | 14150ns | 13896ns | 14116ns | 14401ns | base |
| abi_native_cross_real_null_entry | 4818ns | 4776ns | 4530ns | 4747ns | 5070ns | -66.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11345ns | 10981ns | 11754ns | -4.73% | 0.006 |
| abi_native_cross_real_native_ffi_w | 11909ns | 11635ns | 12103ns | base | 0.005 |
| abi_native_cross_real_null_entry | 2526ns | 2385ns | 2658ns | -78.79% | 0.025 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5337.5 | 11340.1 | 11345.3 | n/a |
| abi_native_cross_real_native_ffi_w | 25232.0 | 12064.2 | 11908.5 | n/a |
| abi_native_cross_real_null_entry | 26879.0 | 2752.6 | 2525.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.006 | 21.3% |
| abi_native_cross_real_native_ffi_w | 0.005 | 20.0% |
| abi_native_cross_real_null_entry | 0.026 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13645ns | 13645ns | -3.74% |
| abi_native_cross_real_native_ffi_w | 14175ns | 14175ns | base |
| abi_native_cross_real_null_entry | 4818ns | 4818ns | -66.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 11900ns | base | --- | [11723, 12103] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11214ns | -553.5ns (-4.7%) | [-1000, -136]ns | [11069, 11754] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2506ns | -9381.0ns (-78.8%) | [-9651, -9116]ns | [2414, 2658] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 11810ns | -1.0% | -79.8% |
| 2 | 12175ns | -9.8% | -79.8% |
| 3 | 11635ns | -4.1% | -78.1% |
| 4 | 11969ns | -1.3% | -78.0% |
| 5 | 11831ns | -5.3% | -77.3% |
| 6 | 12031ns | -6.7% | -79.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.361 | moderate- |
| abi_native_cross_real_native_ffi_w | -0.720 | HIGH- (thermal bounce) |
| abi_native_cross_real_null_entry | 0.205 | moderate+ |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 120369.5ns | 11345.3ns | 1061.0% | HIGH |
| abi_native_cross_real_native_ffi_w | 141068.1ns | 11908.5ns | 1184.6% | HIGH |
| abi_native_cross_real_null_entry | 113216.5ns | 2525.7ns | 4482.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 10981.2-11753.5 ns)
  10981.2 |########################################
  11019.8 |
  11058.4 |
  11097.1 |
  11135.7 |########################################
  11174.3 |########################################
  11212.9 |########################################
  11251.5 |
  11290.1 |
  11328.8 |
  11367.4 |
  11406.0 |
  11444.6 |
  11483.2 |
  11521.8 |
  11560.5 |
  11599.1 |
  11637.7 |
  11676.3 |########################################
  11714.9 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 11635.0-12103.1 ns)
  11635.0 |########################################
  11658.4 |
  11681.8 |
  11705.2 |
  11728.6 |
  11752.0 |
  11775.4 |
  11798.8 |########################################
  11822.2 |########################################
  11845.6 |
  11869.0 |
  11892.5 |
  11915.9 |
  11939.3 |
  11962.7 |########################################
  11986.1 |
  12009.5 |########################################
  12032.9 |
  12056.3 |
  12079.7 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2384.6-2657.7 ns)
   2384.6 |########################################
   2398.3 |
   2411.9 |
   2425.6 |
   2439.2 |########################################
   2452.9 |########################################
   2466.5 |
   2480.2 |
   2493.8 |
   2507.5 |
   2521.1 |
   2534.8 |
   2548.5 |########################################
   2562.1 |
   2575.8 |
   2589.4 |
   2603.1 |
   2616.7 |
   2630.4 |########################################
   2644.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1058.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1188.2% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=4519.2% of algo (FFI overhead may distort results)
