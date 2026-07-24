# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 106012% faster than the next best (abi_cross_scalar_madd_inproc_direct)

abi_cross_scalar_madd_null_entry (2.87 us) leads abi_cross_scalar_madd_inproc_direct (3.04 ms) by 106012%, a clear separation rather than a photo finish. CV 6.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -3.04 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_ffi_batched_scalar is an outlier: 1103.7x slower than the field

abi_cross_scalar_madd_ffi_batched_scalar (3.16 ms) is 1103.7x the fastest (2.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry is fastest but the noisiest (CV 6.5%)

abi_cross_scalar_madd_null_entry wins on median (2.87 us) yet has the highest variance (CV 6.5%), while abi_cross_scalar_madd_ffi_batched_scalar is the steadiest (CV 4.2%, 3.16 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} (106012% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} with a 106012% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1103.7x the fastest

Fastest abi_cross_scalar_madd_null_entry (2.87 us) to slowest abi_cross_scalar_madd_ffi_batched_scalar (3.16 ms): 1103.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 2865.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1103.69x (fastest 2865.6 ns, slowest 3162790.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3136109ns | 3168750ns | 2899856ns | 3128052ns | 3266321ns | +3.10% |
| abi_cross_scalar_madd_inproc_direct | 3041920ns | 3045773ns | 2833203ns | 3016579ns | 3184291ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 3137272ns | 3111550ns | 2876596ns | 3072038ns | 3365460ns | +3.13% |
| abi_cross_scalar_madd_null_entry | 5571ns | 5263ns | 5121ns | 5234ns | 6302ns | -99.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3130168ns | 2894619ns | 3259950ns | +3.09% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 3036484ns | 2828582ns | 3178598ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 3131741ns | 2871549ns | 3359506ns | +3.14% | 0.000 |
| abi_cross_scalar_madd_null_entry | 2931ns | 2785ns | 3137ns | -99.90% | 0.044 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 115961.1 | 3157987.0 | 3130168.5 | n/a |
| abi_cross_scalar_madd_inproc_direct | 11631.9 | 3050473.1 | 3036483.7 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 11108.4 | 3197516.1 | 3131741.0 | 0 |
| abi_cross_scalar_madd_null_entry | 33711.7 | 2943.0 | 2931.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.046 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.045 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3136109ns | 3136109ns | +3.10% |
| abi_cross_scalar_madd_inproc_direct | 3041920ns | 3041920ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 3137272ns | 3137272ns | +3.13% |
| abi_cross_scalar_madd_null_entry | 5571ns | 5571ns | -99.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 3040798ns | base | --- | [2890056, 3178598] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 3162791ns | no significant difference | [-117178, +304500]ns | [2967765, 3259950] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 3105917ns | no significant difference | [-230757, +444845]ns | [2929800, 3359506] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_madd_null_entry | 2866ns | -3037902.0ns (-99.9%) | [-3175535, -2887221]ns | [2791, 3137] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 3082318ns | +6.1% | -6.8% | -99.9% |
| 2 | 3274877ns | -1.5% | -7.7% | -99.9% |
| 3 | 3080854ns | -6.0% | -3.0% | -99.9% |
| 4 | 2951530ns | +3.0% | +8.0% | -99.9% |
| 5 | 3000741ns | +3.3% | +14.0% | -99.9% |
| 6 | 2828582ns | +14.9% | +16.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.101 | ok |
| abi_cross_scalar_madd_inproc_direct | 0.250 | moderate+ |
| abi_cross_scalar_madd_inproc_fnptr | 0.466 | moderate+ |
| abi_cross_scalar_madd_null_entry | -0.374 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 9469388.9ns | 3130168.5ns | 302.5% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 9148769.4ns | 3036483.7ns | 301.3% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 9564212.3ns | 3131741.0ns | 305.4% | HIGH |
| abi_cross_scalar_madd_null_entry | 127852.7ns | 2931.1ns | 4361.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2894619.2-3259950.2 ns)
  2894619.2 |########################################
  2912885.8 |
  2931152.3 |
  2949418.9 |
  2967685.4 |
  2985952.0 |
  3004218.5 |
  3022485.1 |
  3040751.6 |########################################
  3059018.2 |
  3077284.7 |
  3095551.2 |########################################
  3113817.8 |
  3132084.4 |
  3150350.9 |
  3168617.5 |
  3186884.0 |
  3205150.6 |
  3223417.1 |########################################
  3241683.7 |########################################
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2828581.7-3178597.5 ns)
  2828581.7 |####################
  2846082.5 |
  2863583.3 |
  2881084.1 |
  2898584.9 |
  2916085.7 |
  2933586.4 |
  2951087.2 |####################
  2968588.0 |
  2986088.8 |####################
  3003589.6 |
  3021090.4 |
  3038591.2 |
  3056092.0 |
  3073592.8 |########################################
  3091093.5 |
  3108594.3 |
  3126095.1 |
  3143595.9 |
  3161096.7 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2871548.7-3359506.5 ns)
  2871548.7 |########################################
  2895946.6 |
  2920344.5 |
  2944742.4 |
  2969140.2 |########################################
  2993538.1 |
  3017936.0 |########################################
  3042333.9 |
  3066731.8 |
  3091129.7 |
  3115527.6 |
  3139925.5 |
  3164323.4 |########################################
  3188721.2 |
  3213119.1 |
  3237517.0 |
  3261914.9 |
  3286312.8 |########################################
  3310710.7 |
  3335108.6 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 2785.4-3137.1 ns)
   2785.4 |########################################
   2803.0 |
   2820.6 |
   2838.2 |####################
   2855.7 |
   2873.3 |####################
   2890.9 |
   2908.5 |
   2926.1 |
   2943.7 |####################
   2961.2 |
   2978.8 |
   2996.4 |
   3014.0 |
   3031.6 |
   3049.2 |
   3066.8 |
   3084.3 |
   3101.9 |
   3119.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=301.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=4408.1% of algo (FFI overhead may distort results)
