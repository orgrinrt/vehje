# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 58456% faster than the next best (abi_cross_scalar_madd_inproc_direct)

abi_cross_scalar_madd_null_entry (5.11 us) leads abi_cross_scalar_madd_inproc_direct (2.99 ms) by 58456%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.98 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_ffi_batched_scalar is an outlier: 587.9x slower than the field

abi_cross_scalar_madd_ffi_batched_scalar (3.00 ms) is 587.9x the fastest (5.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry is fastest but the noisiest (CV 10.0%)

abi_cross_scalar_madd_null_entry wins on median (5.11 us) yet has the highest variance (CV 10.0%), while abi_cross_scalar_madd_inproc_fnptr is the steadiest (CV 1.5%, 2.99 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} (58456% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} with a 58456% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 587.9x the fastest

Fastest abi_cross_scalar_madd_null_entry (5.11 us) to slowest abi_cross_scalar_madd_ffi_batched_scalar (3.00 ms): 587.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 5105.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 587.87x (fastest 5105.0 ns, slowest 3001054.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3002557ns | 3006115ns | 2883365ns | 2991697ns | 3078443ns | -3.01% |
| abi_cross_scalar_madd_inproc_direct | 3095614ns | 2994437ns | 2832570ns | 2946313ns | 3451087ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2995987ns | 2997834ns | 2930908ns | 2981867ns | 3049706ns | -3.22% |
| abi_cross_scalar_madd_null_entry | 7898ns | 7542ns | 7273ns | 7501ns | 8805ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2997402ns | 2878630ns | 3073129ns | -3.01% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 3090544ns | 2828036ns | 3445150ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 2990569ns | 2926315ns | 3044006ns | -3.23% | 0.000 |
| abi_cross_scalar_madd_null_entry | 5350ns | 4968ns | 5931ns | -99.83% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 108987.9 | 2995671.7 | 2997402.2 | 1 |
| abi_cross_scalar_madd_inproc_direct | 11046.5 | 3084963.8 | 3090543.8 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 10769.1 | 2989220.7 | 2990569.4 | n/a |
| abi_cross_scalar_madd_null_entry | 32649.7 | 5460.7 | 5349.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_madd_null_entry | 0.000 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3002557ns | 3002557ns | -3.01% |
| abi_cross_scalar_madd_inproc_direct | 3095614ns | 3095614ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2995987ns | 2995987ns | -3.22% |
| abi_cross_scalar_madd_null_entry | 7898ns | 7898ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2989308ns | base | --- | [2837174, 3445150] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 3001055ns | no significant difference | [-400430, +138797]ns | [2918023, 3073129] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2992106ns | no significant difference | [-446503, +129309]ns | [2935596, 3044006] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_madd_null_entry | 5105ns | -2984267.2ns (-99.8%) | [-3439218, -2832097]ns | [5013, 5931] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2975825ns | +0.3% | +0.1% | -99.8% |
| 2 | 3002791ns | -1.5% | +1.1% | -99.8% |
| 3 | 3678377ns | -16.5% | -19.9% | -99.9% |
| 4 | 3211922ns | -6.1% | -5.0% | -99.8% |
| 5 | 2828036ns | +8.7% | +6.3% | -99.8% |
| 6 | 2846311ns | +1.1% | +2.8% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | -0.313 | moderate- |
| abi_cross_scalar_madd_inproc_direct | 0.122 | ok |
| abi_cross_scalar_madd_inproc_fnptr | -0.441 | moderate- |
| abi_cross_scalar_madd_null_entry | -0.066 | ok |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 2/6, lost 3/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 9102566.0ns | 2997402.2ns | 303.7% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 9274860.9ns | 3090543.8ns | 300.1% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 8977853.7ns | 2990569.4ns | 300.2% | HIGH |
| abi_cross_scalar_madd_null_entry | 131614.2ns | 5349.8ns | 2460.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2878630.4-3073129.4 ns)
  2878630.4 |########################################
  2888355.4 |
  2898080.3 |
  2907805.2 |
  2917530.2 |
  2927255.1 |
  2936980.1 |
  2946705.0 |
  2956430.0 |########################################
  2966154.9 |
  2975879.9 |
  2985604.9 |########################################
  2995329.8 |
  3005054.8 |
  3014779.7 |########################################
  3024504.6 |
  3034229.6 |
  3043954.5 |
  3053679.5 |
  3063404.4 |########################################
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2828036.2-3445149.6 ns)
  2828036.2 |########################################
  2858891.9 |
  2889747.5 |
  2920603.2 |
  2951458.9 |####################
  2982314.6 |####################
  3013170.2 |
  3044025.9 |
  3074881.6 |
  3105737.2 |
  3136592.9 |
  3167448.6 |
  3198304.2 |####################
  3229159.9 |
  3260015.6 |
  3290871.2 |
  3321726.9 |
  3352582.6 |
  3383438.3 |
  3414293.9 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2926315.4-3044005.6 ns)
  2926315.4 |########################################
  2932199.9 |
  2938084.4 |
  2943968.9 |########################################
  2949853.4 |
  2955737.9 |
  2961622.5 |
  2967507.0 |
  2973391.5 |########################################
  2979276.0 |
  2985160.5 |
  2991045.0 |
  2996929.5 |
  3002814.0 |########################################
  3008698.5 |
  3014583.0 |
  3020467.6 |
  3026352.1 |
  3032236.6 |########################################
  3038121.1 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 4967.5-5931.4 ns)
   4967.5 |########################################
   5015.7 |########################################
   5063.9 |########################################
   5112.1 |########################################
   5160.3 |
   5208.5 |
   5256.7 |
   5304.9 |
   5353.1 |
   5401.3 |########################################
   5449.5 |
   5497.7 |
   5545.9 |
   5594.1 |
   5642.3 |
   5690.5 |
   5738.7 |
   5786.9 |
   5835.1 |
   5883.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=300.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=2503.0% of algo (FFI overhead may distort results)
