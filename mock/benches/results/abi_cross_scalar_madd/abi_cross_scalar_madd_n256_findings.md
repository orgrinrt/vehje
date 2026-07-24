# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_madd_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_madd_inproc_direct has the worst median (2.93 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_madd_null_entry at 3.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_madd_null_entry dominates: 88817% faster than the next best (abi_cross_scalar_madd_inproc_fnptr)

abi_cross_scalar_madd_null_entry (3.25 us) leads abi_cross_scalar_madd_inproc_fnptr (2.89 ms) by 88817%, a clear separation rather than a photo finish. CV 13.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.93 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_inproc_direct is an outlier: 901.9x slower than the field

abi_cross_scalar_madd_inproc_direct (2.93 ms) is 901.9x the fastest (3.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry is fastest but the noisiest (CV 13.7%)

abi_cross_scalar_madd_null_entry wins on median (3.25 us) yet has the highest variance (CV 13.7%), while abi_cross_scalar_madd_inproc_fnptr is the steadiest (CV 2.0%, 2.89 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_cross_scalar_madd_inproc_fnptr shows alternating (throttle bounce) (autocorr -0.55)

abi_cross_scalar_madd_inproc_fnptr's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar, abi_cross_scalar_madd_inproc_direct} (88817% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar, abi_cross_scalar_madd_inproc_direct} with a 88817% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 901.9x the fastest

Fastest abi_cross_scalar_madd_null_entry (3.25 us) to slowest abi_cross_scalar_madd_inproc_direct (2.93 ms): 901.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 3249.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 901.90x (fastest 3249.6 ns, slowest 2930815.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2934917ns | 2900562ns | 2843411ns | 2889896ns | 3048203ns | -3.51% |
| abi_cross_scalar_madd_inproc_direct | 3041552ns | 2935650ns | 2827617ns | 2911676ns | 3343332ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2907363ns | 2894166ns | 2837992ns | 2884141ns | 2976883ns | -4.41% |
| abi_cross_scalar_madd_null_entry | 6044ns | 5760ns | 5410ns | 5695ns | 6885ns | -99.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2929853ns | 2838786ns | 3042678ns | -3.51% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 3036290ns | 2823264ns | 3336910ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 2902444ns | 2833393ns | 2971301ns | -4.41% | 0.000 |
| abi_cross_scalar_madd_null_entry | 3436ns | 3180ns | 3856ns | -99.89% | 0.075 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 108009.4 | 2938562.8 | 2929852.8 | 1 |
| abi_cross_scalar_madd_inproc_direct | 12558.0 | 3064055.0 | 3036290.2 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 10831.4 | 2898260.7 | 2902443.5 | n/a |
| abi_cross_scalar_madd_null_entry | 34863.5 | 3637.1 | 3436.2 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.081 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.079 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2934917ns | 2934917ns | -3.51% |
| abi_cross_scalar_madd_inproc_direct | 3041552ns | 3041552ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2907363ns | 2907363ns | -4.41% |
| abi_cross_scalar_madd_null_entry | 6044ns | 6044ns | -99.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2930816ns | base | --- | [2841145, 3336910] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 2895534ns | no significant difference | [-371780, +131938]ns | [2851346, 3042678] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2889463ns | no significant difference | [-394685, +49057]ns | [2846566, 2971301] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_madd_null_entry | 3250ns | -2927009.9ns (-99.9%) | [-3333660, -2837892]ns | [3203, 3856] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2859026ns | +1.7% | +1.7% | -99.9% |
| 2 | 3238358ns | -10.9% | -11.4% | -99.9% |
| 3 | 3435461ns | -11.3% | -12.3% | -99.9% |
| 4 | 2981693ns | -4.0% | -4.1% | -99.9% |
| 5 | 2879938ns | -1.4% | +1.7% | -99.8% |
| 6 | 2823264ns | +7.7% | +0.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | -0.390 | moderate- |
| abi_cross_scalar_madd_inproc_direct | 0.213 | moderate+ |
| abi_cross_scalar_madd_inproc_fnptr | -0.551 | HIGH- (thermal bounce) |
| abi_cross_scalar_madd_null_entry | -0.296 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 4/6, lost 2/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 8911464.1ns | 2929852.8ns | 304.2% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 9109132.0ns | 3036290.2ns | 300.0% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 8722081.9ns | 2902443.5ns | 300.5% | HIGH |
| abi_cross_scalar_madd_null_entry | 131173.4ns | 3436.2ns | 3817.4% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2838786.2-3042678.3 ns)
  2838786.2 |########################################
  2848980.8 |
  2859175.4 |########################################
  2869370.0 |
  2879564.6 |########################################
  2889759.2 |
  2899953.8 |########################################
  2910148.4 |
  2920343.0 |
  2930537.6 |
  2940732.2 |
  2950926.9 |
  2961121.5 |
  2971316.1 |
  2981510.7 |
  2991705.3 |
  3001899.9 |
  3012094.5 |
  3022289.1 |
  3032483.7 |########################################
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2823264.2-3336909.8 ns)
  2823264.2 |########################################
  2848946.5 |########################################
  2874628.8 |########################################
  2900311.0 |
  2925993.3 |
  2951675.6 |
  2977357.9 |########################################
  3003040.1 |
  3028722.4 |
  3054404.7 |
  3080087.0 |
  3105769.3 |
  3131451.5 |
  3157133.8 |
  3182816.1 |
  3208498.4 |
  3234180.6 |########################################
  3259862.9 |
  3285545.2 |
  3311227.5 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2833393.3-2971301.2 ns)
  2833393.3 |########################################
  2840288.7 |
  2847184.1 |
  2854079.5 |########################################
  2860974.9 |
  2867870.3 |########################################
  2874765.7 |
  2881661.1 |
  2888556.5 |
  2895451.9 |
  2902347.2 |########################################
  2909242.6 |
  2916138.0 |
  2923033.4 |########################################
  2929928.8 |
  2936824.2 |
  2943719.6 |
  2950615.0 |
  2957510.4 |
  2964405.8 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 3179.6-3855.8 ns)
   3179.6 |####################
   3213.4 |########################################
   3247.2 |########################################
   3281.0 |
   3314.8 |
   3348.7 |
   3382.5 |
   3416.3 |
   3450.1 |
   3483.9 |
   3517.7 |
   3551.5 |
   3585.3 |
   3619.2 |
   3653.0 |
   3686.8 |
   3720.6 |
   3754.4 |
   3788.2 |
   3822.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=307.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=305.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=3803.5% of algo (FFI overhead may distort results)
