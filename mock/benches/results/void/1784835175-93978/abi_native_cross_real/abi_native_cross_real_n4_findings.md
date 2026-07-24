# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (13.98 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 3.88 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 195% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (3.88 us) leads abi_native_cross_real_inproc_native (11.43 us) by 195%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 72% (significant)

abi_native_cross_real_null_entry is -10.00 us (72%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 3.6x slower than the field

abi_native_cross_real_native_ffi_w (13.98 us) is 3.6x the fastest (3.88 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_native_ffi_w shows alternating (throttle bounce) (autocorr -0.51)

abi_native_cross_real_native_ffi_w's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.6x the fastest

Fastest abi_native_cross_real_null_entry (3.88 us) to slowest abi_native_cross_real_native_ffi_w (13.98 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 3878.7 ns median (-72.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.61x (fastest 3878.7 ns, slowest 13983.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13698ns | 13660ns | 13400ns | 13591ns | 14006ns | -15.39% |
| abi_native_cross_real_native_ffi_w | 16190ns | 16242ns | 15685ns | 16207ns | 16417ns | base |
| abi_native_cross_real_null_entry | 6110ns | 6075ns | 5982ns | 6045ns | 6273ns | -62.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11411ns | 11125ns | 11675ns | -18.07% | 0.000 |
| abi_native_cross_real_native_ffi_w | 13926ns | 13450ns | 14126ns | base | 0.000 |
| abi_native_cross_real_null_entry | 3911ns | 3837ns | 4015ns | -71.91% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5380.1 | 11451.4 | 11410.5 | n/a |
| abi_native_cross_real_native_ffi_w | 25702.5 | 14075.6 | 13926.3 | n/a |
| abi_native_cross_real_null_entry | 26497.6 | 4048.7 | 3911.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.000 | 33.6% |
| abi_native_cross_real_native_ffi_w | 0.000 | 27.4% |
| abi_native_cross_real_null_entry | 0.001 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13698ns | 13698ns | -15.39% |
| abi_native_cross_real_native_ffi_w | 16190ns | 16190ns | base |
| abi_native_cross_real_null_entry | 6110ns | 6110ns | -62.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 13983ns | base | --- | [13670, 14126] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11428ns | -2450.5ns (-17.5%) | [-2790, -2307]ns | [11128, 11675] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 3879ns | -9999.0ns (-71.5%) | [-10286, -9760]ns | [3840, 4015] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 14182ns | -17.3% | -72.9% |
| 2 | 13450ns | -17.2% | -71.1% |
| 3 | 14057ns | -20.9% | -70.8% |
| 4 | 14069ns | -17.4% | -72.7% |
| 5 | 13890ns | -19.1% | -72.1% |
| 6 | 13910ns | -16.5% | -71.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.369 | moderate- |
| abi_native_cross_real_native_ffi_w | -0.513 | HIGH- (thermal bounce) |
| abi_native_cross_real_null_entry | -0.285 | moderate- |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 120843.3ns | 11410.5ns | 1059.1% | HIGH |
| abi_native_cross_real_native_ffi_w | 148390.7ns | 13926.3ns | 1065.5% | HIGH |
| abi_native_cross_real_null_entry | 120383.1ns | 3911.3ns | 3077.8% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11125.0-11675.2 ns)
  11125.0 |########################################
  11152.5 |
  11180.0 |
  11207.5 |
  11235.0 |####################
  11262.5 |
  11290.1 |
  11317.6 |
  11345.1 |
  11372.6 |
  11400.1 |
  11427.6 |
  11455.1 |
  11482.6 |
  11510.1 |
  11537.7 |
  11565.2 |
  11592.7 |########################################
  11620.2 |
  11647.7 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 13450.4-14125.7 ns)
  13450.4 |####################
  13484.2 |
  13517.9 |
  13551.7 |
  13585.5 |
  13619.2 |
  13653.0 |
  13686.7 |
  13720.5 |
  13754.3 |
  13788.0 |
  13821.8 |
  13855.6 |
  13889.3 |########################################
  13923.1 |
  13956.8 |
  13990.6 |
  14024.4 |####################
  14058.1 |####################
  14091.9 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 3837.1-4015.2 ns)
   3837.1 |########################################
   3846.0 |
   3854.9 |
   3863.8 |####################
   3872.7 |
   3881.6 |####################
   3890.5 |
   3899.4 |
   3908.3 |
   3917.2 |
   3926.1 |####################
   3935.1 |
   3944.0 |
   3952.9 |
   3961.8 |
   3970.7 |
   3979.6 |
   3988.5 |
   3997.4 |
   4006.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1060.6% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1056.5% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=3102.4% of algo (FFI overhead may distort results)
