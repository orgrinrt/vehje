# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_madd_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_madd_inproc_direct has the worst median (2.77 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_madd_null_entry at 3.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_madd_null_entry dominates: 88951% faster than the next best (abi_cross_scalar_madd_inproc_fnptr)

abi_cross_scalar_madd_null_entry (3.09 us) leads abi_cross_scalar_madd_inproc_fnptr (2.75 ms) by 88951%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.77 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_inproc_direct is an outlier: 895.8x slower than the field

abi_cross_scalar_madd_inproc_direct (2.77 ms) is 895.8x the fastest (3.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry shows alternating (throttle bounce) (autocorr -0.60)

abi_cross_scalar_madd_null_entry's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar, abi_cross_scalar_madd_inproc_direct} (88951% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar, abi_cross_scalar_madd_inproc_direct} with a 88951% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 895.8x the fastest

Fastest abi_cross_scalar_madd_null_entry (3.09 us) to slowest abi_cross_scalar_madd_inproc_direct (2.77 ms): 895.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 3090.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 895.80x (fastest 3090.4 ns, slowest 2768389.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2784655ns | 2771647ns | 2750288ns | 2766895ns | 2828478ns | -0.35% |
| abi_cross_scalar_madd_inproc_direct | 2794305ns | 2772229ns | 2723789ns | 2760486ns | 2880290ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2804499ns | 2755937ns | 2738743ns | 2755020ns | 2911597ns | +0.36% |
| abi_cross_scalar_madd_null_entry | 5447ns | 5435ns | 5251ns | 5383ns | 5640ns | -99.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2780840ns | 2746989ns | 2824455ns | -0.35% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 2790548ns | 2720509ns | 2876283ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 2800706ns | 2735502ns | 2907389ns | +0.36% | 0.000 |
| abi_cross_scalar_madd_null_entry | 3108ns | 3036ns | 3194ns | -99.89% | 0.003 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 82437.2 | 2781041.1 | 2780839.9 | 8 |
| abi_cross_scalar_madd_inproc_direct | 8884.7 | 2854237.2 | 2790547.7 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 8983.3 | 2790874.0 | 2800705.8 | n/a |
| abi_cross_scalar_madd_null_entry | 28292.4 | 3170.6 | 3107.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.003 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2784655ns | 2784655ns | -0.35% |
| abi_cross_scalar_madd_inproc_direct | 2794305ns | 2794305ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2804499ns | 2804499ns | +0.36% |
| abi_cross_scalar_madd_null_entry | 5447ns | 5447ns | -99.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2768390ns | base | --- | [2726970, 2876283] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 2767804ns | no significant difference | [-94772, +41806]ns | [2750261, 2824455] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2752042ns | no significant difference | [-101426, +117513]ns | [2742686, 2907389] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_madd_null_entry | 3090ns | -2765243.0ns (-99.9%) | [-2873193, -2723884]ns | [3038, 3194] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2733431ns | +1.7% | +0.7% | -99.9% |
| 2 | 2742340ns | +0.4% | +0.3% | -99.9% |
| 3 | 2947727ns | -6.8% | -7.2% | -99.9% |
| 4 | 2720509ns | +1.3% | +1.1% | -99.9% |
| 5 | 2794440ns | +1.3% | +0.4% | -99.9% |
| 6 | 2804839ns | +0.4% | +7.3% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.370 | moderate+ |
| abi_cross_scalar_madd_inproc_direct | -0.453 | moderate- |
| abi_cross_scalar_madd_inproc_fnptr | 0.170 | ok |
| abi_cross_scalar_madd_null_entry | -0.603 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 8418029.6ns | 2780839.9ns | 302.7% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 8488469.5ns | 2790547.7ns | 304.2% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 8407890.5ns | 2800705.8ns | 300.2% | HIGH |
| abi_cross_scalar_madd_null_entry | 120333.0ns | 3107.7ns | 3872.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2746989.2-2824454.6 ns)
  2746989.2 |########################################
  2750862.5 |########################################
  2754735.7 |########################################
  2758609.0 |
  2762482.3 |
  2766355.6 |
  2770228.8 |
  2774102.1 |
  2777975.4 |########################################
  2781848.6 |
  2785721.9 |
  2789595.2 |
  2793468.4 |
  2797341.7 |
  2801215.0 |
  2805088.2 |
  2808961.5 |
  2812834.8 |
  2816708.1 |########################################
  2820581.3 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2720509.2-2876283.2 ns)
  2720509.2 |########################################
  2728297.9 |########################################
  2736086.6 |########################################
  2743875.3 |
  2751664.0 |
  2759452.7 |
  2767241.4 |
  2775030.1 |
  2782818.8 |
  2790607.5 |########################################
  2798396.2 |########################################
  2806184.9 |
  2813973.6 |
  2821762.3 |
  2829551.0 |
  2837339.7 |
  2845128.4 |
  2852917.1 |
  2860705.8 |
  2868494.5 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2735501.7-2907389.4 ns)
  2735501.7 |#############
  2744096.1 |########################################
  2752690.5 |
  2761284.9 |
  2769879.2 |
  2778473.6 |
  2787068.0 |
  2795662.4 |
  2804256.8 |#############
  2812851.2 |
  2821445.6 |
  2830039.9 |
  2838634.3 |
  2847228.7 |
  2855823.1 |
  2864417.5 |
  2873011.9 |
  2881606.2 |
  2890200.6 |
  2898795.0 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 3035.8-3194.2 ns)
   3035.8 |########################################
   3043.7 |
   3051.6 |
   3059.6 |####################
   3067.5 |
   3075.4 |
   3083.3 |
   3091.2 |
   3099.1 |
   3107.1 |
   3115.0 |####################
   3122.9 |
   3130.8 |####################
   3138.7 |
   3146.6 |
   3154.6 |
   3162.5 |
   3170.4 |
   3178.3 |
   3186.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=299.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=300.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=3884.8% of algo (FFI overhead may distort results)
