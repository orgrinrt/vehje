# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (11.92 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 2.84 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 314% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (2.84 us) leads abi_native_cross_tight_inproc_native (11.75 us) by 314%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 76% (significant)

abi_native_cross_tight_null_entry is -9.03 us (76%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 4.2x slower than the field

abi_native_cross_tight_native_ffi_w (11.92 us) is 4.2x the fastest (2.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_tight_inproc_native shows alternating (throttle bounce) (autocorr -0.61)

abi_native_cross_tight_inproc_native's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.2x the fastest

Fastest abi_native_cross_tight_null_entry (2.84 us) to slowest abi_native_cross_tight_native_ffi_w (11.92 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 2841.1 ns median (-76.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.20x (fastest 2841.1 ns, slowest 11923.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 14043ns | 14046ns | 13670ns | 13963ns | 14349ns | -1.43% |
| abi_native_cross_tight_native_ffi_w | 14247ns | 14245ns | 13977ns | 14206ns | 14443ns | base |
| abi_native_cross_tight_null_entry | 5171ns | 5224ns | 4844ns | 5172ns | 5335ns | -63.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11765ns | 11474ns | 12024ns | -1.60% | 0.011 |
| abi_native_cross_tight_native_ffi_w | 11957ns | 11731ns | 12172ns | base | 0.011 |
| abi_native_cross_tight_null_entry | 2817ns | 2668ns | 2896ns | -76.44% | 0.045 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5404.1 | 11466.9 | 11764.7 | n/a |
| abi_native_cross_tight_native_ffi_w | 25256.5 | 12089.4 | 11956.6 | n/a |
| abi_native_cross_tight_null_entry | 27940.1 | 2842.7 | 2817.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.011 | 22.7% |
| abi_native_cross_tight_native_ffi_w | 0.011 | 22.4% |
| abi_native_cross_tight_null_entry | 0.045 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 14043ns | 14043ns | -1.43% |
| abi_native_cross_tight_native_ffi_w | 14247ns | 14247ns | base |
| abi_native_cross_tight_null_entry | 5171ns | 5171ns | -63.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 11923ns | base | --- | [11775, 12172] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11754ns | no significant difference | [-354, +26]ns | [11516, 12024] | no | 0.2188 | 0.2188 | 0 |
| abi_native_cross_tight_null_entry | 2841ns | -9028.2ns (-75.7%) | [-9426, -8964]ns | [2714, 2896] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 11957ns | -1.4% | -77.7% |
| 2 | 11922ns | -3.0% | -75.4% |
| 3 | 11731ns | -0.2% | -76.5% |
| 4 | 12386ns | -2.7% | -77.2% |
| 5 | 11819ns | -2.9% | -75.8% |
| 6 | 11925ns | +0.6% | -76.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.605 | HIGH- (thermal bounce) |
| abi_native_cross_tight_native_ffi_w | -0.560 | HIGH- (thermal bounce) |
| abi_native_cross_tight_null_entry | -0.512 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 123263.4ns | 11764.7ns | 1047.7% | HIGH |
| abi_native_cross_tight_native_ffi_w | 143706.6ns | 11956.6ns | 1201.9% | HIGH |
| abi_native_cross_tight_null_entry | 120532.3ns | 2817.1ns | 4278.6% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11473.8-12024.1 ns)
  11473.8 |########################################
  11501.3 |
  11528.8 |
  11556.4 |########################################
  11583.9 |
  11611.4 |
  11638.9 |
  11666.4 |
  11693.9 |########################################
  11721.5 |
  11749.0 |
  11776.5 |########################################
  11804.0 |
  11831.5 |
  11859.0 |
  11886.6 |
  11914.1 |
  11941.6 |
  11969.1 |########################################
  11996.6 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 11730.8-12171.7 ns)
  11730.8 |####################
  11752.8 |
  11774.9 |
  11796.9 |
  11819.0 |####################
  11841.0 |
  11863.1 |
  11885.1 |
  11907.1 |########################################
  11929.2 |
  11951.2 |####################
  11973.3 |
  11995.3 |
  12017.4 |
  12039.4 |
  12061.4 |
  12083.5 |
  12105.5 |
  12127.6 |
  12149.6 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 2668.3-2895.8 ns)
   2668.3 |####################
   2679.7 |
   2691.1 |
   2702.4 |
   2713.8 |
   2725.2 |
   2736.6 |
   2747.9 |
   2759.3 |####################
   2770.7 |
   2782.1 |
   2793.4 |
   2804.8 |
   2816.2 |####################
   2827.6 |
   2838.9 |
   2850.3 |########################################
   2861.7 |
   2873.1 |
   2884.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1050.4% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1211.7% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=4256.5% of algo (FFI overhead may distort results)
