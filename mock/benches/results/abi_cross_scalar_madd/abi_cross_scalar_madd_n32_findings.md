# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 117137% faster than the next best (abi_cross_scalar_madd_ffi_batched_scalar)

abi_cross_scalar_madd_null_entry (2.39 us) leads abi_cross_scalar_madd_ffi_batched_scalar (2.80 ms) by 117137%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.81 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_inproc_fnptr is an outlier: 1203.3x slower than the field

abi_cross_scalar_madd_inproc_fnptr (2.88 ms) is 1203.3x the fastest (2.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry shows alternating (throttle bounce) (autocorr -0.61)

abi_cross_scalar_madd_null_entry's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_ffi_batched_scalar, abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr} (117137% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_ffi_batched_scalar, abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr} with a 117137% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1203.3x the fastest

Fastest abi_cross_scalar_madd_null_entry (2.39 us) to slowest abi_cross_scalar_madd_inproc_fnptr (2.88 ms): 1203.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 2391.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1203.33x (fastest 2391.9 ns, slowest 2878237.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2842683ns | 2808314ns | 2752665ns | 2790939ns | 2965307ns | -0.41% |
| abi_cross_scalar_madd_inproc_direct | 2854342ns | 2819048ns | 2719246ns | 2786213ns | 3024083ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 3036724ns | 2882642ns | 2739625ns | 2847428ns | 3469217ns | +6.39% |
| abi_cross_scalar_madd_null_entry | 4906ns | 4826ns | 4549ns | 4821ns | 5210ns | -99.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2838576ns | 2749035ns | 2960676ns | -0.41% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 2850132ns | 2715662ns | 3019399ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 3032011ns | 2736079ns | 3463094ns | +6.38% | 0.000 |
| abi_cross_scalar_madd_null_entry | 2416ns | 2290ns | 2529ns | -99.92% | 0.013 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 87046.7 | 2829830.7 | 2838576.0 | 7 |
| abi_cross_scalar_madd_inproc_direct | 9458.0 | 2828830.3 | 2850131.7 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 10203.6 | 3064961.5 | 3032010.8 | n/a |
| abi_cross_scalar_madd_null_entry | 35942.4 | 2635.8 | 2415.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.013 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2842683ns | 2842683ns | -0.41% |
| abi_cross_scalar_madd_inproc_direct | 2854342ns | 2854342ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 3036724ns | 3036724ns | +6.39% |
| abi_cross_scalar_madd_null_entry | 4906ns | 4906ns | -99.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2814681ns | base | --- | [2716315, 3019399] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 2804192ns | no significant difference | [-106466, +43238]ns | [2750859, 2960676] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2878237ns | no significant difference | [-141162, +655049]ns | [2754701, 3463094] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_scalar_madd_null_entry | 2392ns | -2812235.3ns (-99.9%) | [-3016973, -2713939]ns | [2327, 2529] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2716969ns | +1.9% | +2.1% | -99.9% |
| 2 | 2715662ns | +1.2% | +6.7% | -99.9% |
| 3 | 2728933ns | +0.9% | +0.3% | -99.9% |
| 4 | 3130367ns | -4.6% | -7.9% | -99.9% |
| 5 | 2908431ns | -2.4% | -1.2% | -99.9% |
| 6 | 2900428ns | +1.2% | +38.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.021 | ok |
| abi_cross_scalar_madd_inproc_direct | 0.145 | ok |
| abi_cross_scalar_madd_inproc_fnptr | -0.013 | ok |
| abi_cross_scalar_madd_null_entry | -0.614 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 2/6, lost 4/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 8596363.0ns | 2838576.0ns | 302.8% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 8511718.1ns | 2850131.7ns | 298.6% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 9239262.8ns | 3032010.8ns | 304.7% | HIGH |
| abi_cross_scalar_madd_null_entry | 124974.0ns | 2415.8ns | 5173.3% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2749035.4-2960676.5 ns)
  2749035.4 |########################################
  2759617.5 |####################
  2770199.5 |
  2780781.6 |
  2791363.6 |
  2801945.7 |
  2812527.7 |
  2823109.8 |
  2833691.8 |####################
  2844273.9 |
  2854855.9 |
  2865438.0 |
  2876020.0 |
  2886602.1 |
  2897184.1 |
  2907766.2 |
  2918348.2 |
  2928930.3 |####################
  2939512.3 |
  2950094.4 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2715661.7-3019399.0 ns)
  2715661.7 |########################################
  2730848.6 |
  2746035.4 |
  2761222.3 |
  2776409.2 |
  2791596.0 |
  2806782.9 |
  2821969.7 |
  2837156.6 |
  2852343.5 |
  2867530.3 |
  2882717.2 |
  2897904.1 |##########################
  2913090.9 |
  2928277.8 |
  2943464.6 |
  2958651.5 |
  2973838.4 |
  2989025.2 |
  3004212.1 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2736078.8-3463094.0 ns)
  2736078.8 |####################
  2772429.6 |####################
  2808780.3 |
  2845131.1 |####################
  2881481.8 |########################################
  2917832.6 |
  2954183.3 |
  2990534.1 |
  3026884.9 |
  3063235.6 |
  3099586.4 |
  3135937.1 |
  3172287.9 |
  3208638.6 |
  3244989.4 |
  3281340.2 |
  3317690.9 |
  3354041.7 |
  3390392.4 |
  3426743.2 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 2290.4-2528.8 ns)
   2290.4 |####################
   2302.3 |
   2314.2 |
   2326.2 |
   2338.1 |
   2350.0 |
   2361.9 |####################
   2373.8 |
   2385.7 |########################################
   2397.7 |
   2409.6 |
   2421.5 |
   2433.4 |
   2445.3 |
   2457.2 |
   2469.2 |
   2481.1 |
   2493.0 |####################
   2504.9 |
   2516.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=299.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=5216.1% of algo (FFI overhead may distort results)
