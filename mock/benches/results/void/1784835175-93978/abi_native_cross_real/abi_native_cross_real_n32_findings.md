# abi_native_cross (real)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_real_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_real_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_real_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_real_native_ffi_w has the worst median (12.56 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_real_null_entry at 2.28 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_real_null_entry dominates: 402% faster than the next best (abi_native_cross_real_inproc_native)

abi_native_cross_real_null_entry (2.28 us) leads abi_native_cross_real_inproc_native (11.45 us) by 402%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_real_null_entry beats baseline by 82% (significant)

abi_native_cross_real_null_entry is -10.25 us (82%) faster than baseline abi_native_cross_real_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_real_native_ffi_w is an outlier: 5.5x slower than the field

abi_native_cross_real_native_ffi_w (12.56 us) is 5.5x the fastest (2.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_real_inproc_native shows alternating (throttle bounce) (autocorr -0.54)

abi_native_cross_real_inproc_native's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest abi_native_cross_real_null_entry (2.28 us) to slowest abi_native_cross_real_native_ffi_w (12.56 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_real_null_entry** at 2280.4 ns median (-81.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.51x (fastest 2280.4 ns, slowest 12558.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 13829ns | 13770ns | 13490ns | 13697ns | 14196ns | -8.06% |
| abi_native_cross_real_native_ffi_w | 15041ns | 14863ns | 14569ns | 14819ns | 15609ns | base |
| abi_native_cross_real_null_entry | 4531ns | 4562ns | 4286ns | 4526ns | 4661ns | -69.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 11492ns | 11252ns | 11768ns | -9.65% | 0.003 |
| abi_native_cross_real_native_ffi_w | 12720ns | 12309ns | 13233ns | base | 0.003 |
| abi_native_cross_real_null_entry | 2273ns | 2170ns | 2333ns | -82.13% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 5789.1 | 11516.1 | 11492.0 | n/a |
| abi_native_cross_real_native_ffi_w | 26486.1 | 12796.8 | 12719.9 | n/a |
| abi_native_cross_real_null_entry | 27225.9 | 2413.0 | 2272.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_native_cross_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_real_inproc_native | 0.003 | 18.9% |
| abi_native_cross_real_native_ffi_w | 0.003 | 17.3% |
| abi_native_cross_real_null_entry | 0.014 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_real_inproc_native | 13829ns | 13829ns | -8.06% |
| abi_native_cross_real_native_ffi_w | 15041ns | 15041ns | base |
| abi_native_cross_real_null_entry | 4531ns | 4531ns | -69.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_real_native_ffi_w | 12559ns | base | --- | [12368, 13233] | --- | --- | --- | --- |
| abi_native_cross_real_inproc_native | 11454ns | -1090.4ns (-8.7%) | [-1827, -766]ns | [11254, 11768] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_real_null_entry | 2280ns | -10248.8ns (-81.6%) | [-11003, -10089]ns | [2205, 2333] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_real_native_ffi_w | abi_native_cross_real_inproc_native | abi_native_cross_real_null_entry |
|---|---|---|---|
| 1 | 13748ns | -16.0% | -84.2% |
| 2 | 12718ns | -11.5% | -82.0% |
| 3 | 12309ns | -7.8% | -81.2% |
| 4 | 12641ns | -6.5% | -81.4% |
| 5 | 12477ns | -9.8% | -81.8% |
| 6 | 12428ns | -5.7% | -82.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_real_inproc_native | -0.544 | HIGH- (thermal bounce) |
| abi_native_cross_real_native_ffi_w | 0.088 | ok |
| abi_native_cross_real_null_entry | 0.120 | ok |

**Consistency summary:**

- **abi_native_cross_real_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_real_inproc_native | 138554.6ns | 11492.0ns | 1205.7% | HIGH |
| abi_native_cross_real_native_ffi_w | 143105.3ns | 12719.9ns | 1125.0% | HIGH |
| abi_native_cross_real_null_entry | 115619.0ns | 2272.8ns | 5087.0% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_real_inproc_native (n=6, range 11251.7-11768.1 ns)
  11251.7 |########################################
  11277.5 |
  11303.3 |
  11329.2 |####################
  11355.0 |
  11380.8 |
  11406.6 |
  11432.5 |
  11458.3 |
  11484.1 |
  11509.9 |
  11535.7 |####################
  11561.6 |
  11587.4 |
  11613.2 |
  11639.0 |
  11664.9 |
  11690.7 |
  11716.5 |####################
  11742.3 |
  (0 below, 1 above range)

abi_native_cross_real_native_ffi_w (n=6, range 12308.8-13232.9 ns)
  12308.8 |########################################
  12355.0 |
  12401.2 |########################################
  12447.4 |########################################
  12493.6 |
  12539.8 |
  12586.0 |
  12632.2 |########################################
  12678.4 |########################################
  12724.6 |
  12770.8 |
  12817.1 |
  12863.3 |
  12909.5 |
  12955.7 |
  13001.9 |
  13048.1 |
  13094.3 |
  13140.5 |
  13186.7 |
  (0 below, 1 above range)

abi_native_cross_real_null_entry (n=6, range 2169.6-2333.3 ns)
   2169.6 |########################################
   2177.8 |
   2186.0 |
   2194.2 |
   2202.3 |
   2210.5 |
   2218.7 |
   2226.9 |
   2235.1 |########################################
   2243.3 |
   2251.4 |
   2259.6 |
   2267.8 |########################################
   2276.0 |
   2284.2 |########################################
   2292.4 |
   2300.6 |
   2308.7 |
   2316.9 |########################################
   2325.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_real_inproc_native**: bridge=1067.1% of algo (FFI overhead may distort results)
- **abi_native_cross_real_native_ffi_w**: bridge=1131.8% of algo (FFI overhead may distort results)
- **abi_native_cross_real_null_entry**: bridge=5083.8% of algo (FFI overhead may distort results)
