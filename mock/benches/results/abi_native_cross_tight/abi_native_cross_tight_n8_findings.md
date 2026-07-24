# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (13.45 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 3.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 272% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (3.07 us) leads abi_native_cross_tight_inproc_native (11.44 us) by 272%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 77% (significant)

abi_native_cross_tight_null_entry is -10.32 us (77%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 4.4x slower than the field

abi_native_cross_tight_native_ffi_w (13.45 us) is 4.4x the fastest (3.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_tight_inproc_native shows alternating (throttle bounce) (autocorr -0.57)

abi_native_cross_tight_inproc_native's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest abi_native_cross_tight_null_entry (3.07 us) to slowest abi_native_cross_tight_native_ffi_w (13.45 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 3072.9 ns median (-77.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.38x (fastest 3072.9 ns, slowest 13448.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13770ns | 13745ns | 13419ns | 13656ns | 14115ns | -13.18% |
| abi_native_cross_tight_native_ffi_w | 15860ns | 15748ns | 15095ns | 15733ns | 16432ns | base |
| abi_native_cross_tight_null_entry | 5338ns | 5362ns | 5142ns | 5297ns | 5496ns | -66.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11462ns | 11186ns | 11762ns | -15.62% | 0.001 |
| abi_native_cross_tight_native_ffi_w | 13584ns | 12915ns | 14146ns | base | 0.001 |
| abi_native_cross_tight_null_entry | 3064ns | 2962ns | 3147ns | -77.44% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5418.1 | 11553.1 | 11461.7 | n/a |
| abi_native_cross_tight_native_ffi_w | 25889.8 | 14067.4 | 13584.2 | n/a |
| abi_native_cross_tight_null_entry | 26841.7 | 3106.2 | 3064.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.001 | 25.9% |
| abi_native_cross_tight_native_ffi_w | 0.001 | 22.0% |
| abi_native_cross_tight_null_entry | 0.003 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13770ns | 13770ns | -13.18% |
| abi_native_cross_tight_native_ffi_w | 15860ns | 15860ns | base |
| abi_native_cross_tight_null_entry | 5338ns | 5338ns | -66.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 13448ns | base | --- | [13158, 14146] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11436ns | -1997.7ns (-14.9%) | [-2710, -1659]ns | [11187, 11762] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_tight_null_entry | 3073ns | -10320.6ns (-76.7%) | [-11146, -10093]ns | [2972, 3147] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 14134ns | -17.7% | -79.0% |
| 2 | 14159ns | -20.7% | -78.5% |
| 3 | 13448ns | -16.8% | -76.9% |
| 4 | 13402ns | -11.9% | -76.5% |
| 5 | 12915ns | -13.4% | -76.9% |
| 6 | 13448ns | -12.9% | -76.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.570 | HIGH- (thermal bounce) |
| abi_native_cross_tight_native_ffi_w | 0.413 | moderate+ |
| abi_native_cross_tight_null_entry | -0.247 | moderate- |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 121378.2ns | 11461.7ns | 1059.0% | HIGH |
| abi_native_cross_tight_native_ffi_w | 145171.5ns | 13584.2ns | 1068.7% | HIGH |
| abi_native_cross_tight_null_entry | 118975.5ns | 3064.2ns | 3882.8% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11185.8-11762.5 ns)
  11185.8 |########################################
  11214.6 |####################
  11243.5 |
  11272.3 |
  11301.1 |
  11330.0 |
  11358.8 |
  11387.6 |
  11416.5 |
  11445.3 |
  11474.1 |
  11503.0 |
  11531.8 |
  11560.7 |
  11589.5 |
  11618.3 |####################
  11647.2 |
  11676.0 |
  11704.8 |####################
  11733.7 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 12914.6-14146.2 ns)
  12914.6 |####################
  12976.2 |
  13037.8 |
  13099.3 |
  13160.9 |
  13222.5 |
  13284.1 |
  13345.7 |####################
  13407.3 |########################################
  13468.8 |
  13530.4 |
  13592.0 |
  13653.6 |
  13715.2 |
  13776.8 |
  13838.3 |
  13899.9 |
  13961.5 |
  14023.1 |
  14084.7 |####################
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 2962.5-3147.1 ns)
   2962.5 |########################################
   2971.7 |
   2981.0 |########################################
   2990.2 |
   2999.4 |
   3008.6 |
   3017.9 |
   3027.1 |
   3036.3 |########################################
   3045.5 |
   3054.8 |
   3064.0 |
   3073.2 |
   3082.5 |
   3091.7 |
   3100.9 |########################################
   3110.1 |
   3119.4 |
   3128.6 |
   3137.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1057.3% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1078.3% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=3854.8% of algo (FFI overhead may distort results)
