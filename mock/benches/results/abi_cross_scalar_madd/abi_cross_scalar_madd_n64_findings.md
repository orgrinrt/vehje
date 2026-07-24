# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 102031% faster than the next best (abi_cross_scalar_madd_inproc_direct)

abi_cross_scalar_madd_null_entry (2.87 us) leads abi_cross_scalar_madd_inproc_direct (2.93 ms) by 102031%, a clear separation rather than a photo finish. CV 55.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.93 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_ffi_batched_scalar is an outlier: 1091.1x slower than the field

abi_cross_scalar_madd_ffi_batched_scalar (3.13 ms) is 1091.1x the fastest (2.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_null_entry is fastest but the noisiest (CV 55.2%)

abi_cross_scalar_madd_null_entry wins on median (2.87 us) yet has the highest variance (CV 55.2%), while abi_cross_scalar_madd_ffi_batched_scalar is the steadiest (CV 4.7%, 3.13 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} (102031% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} with a 102031% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1091.1x the fastest

Fastest abi_cross_scalar_madd_null_entry (2.87 us) to slowest abi_cross_scalar_madd_ffi_batched_scalar (3.13 ms): 1091.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_scalar_madd_null_entry is inconsistent: worst-20% is 2.0x its best-20%

abi_cross_scalar_madd_null_entry's best 20% of batches run at 2.53 us but its worst 20% at 5.00 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 2872.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1091.13x (fastest 2872.1 ns, slowest 3133780.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3065190ns | 3138550ns | 2852570ns | 3048726ns | 3196194ns | +1.71% |
| abi_cross_scalar_madd_inproc_direct | 3013760ns | 2938565ns | 2896968ns | 2925920ns | 3203916ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 3090903ns | 3035820ns | 2894213ns | 2999890ns | 3325767ns | +2.56% |
| abi_cross_scalar_madd_null_entry | 6759ns | 5432ns | 4995ns | 5372ns | 9722ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3060040ns | 2847698ns | 3190356ns | +1.71% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 3008525ns | 2892328ns | 3198587ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 3085639ns | 2889280ns | 3320092ns | +2.56% | 0.000 |
| abi_cross_scalar_madd_null_entry | 3500ns | 2531ns | 4996ns | -99.88% | 0.018 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 113256.0 | 3079138.5 | 3060040.4 | n/a |
| abi_cross_scalar_madd_inproc_direct | 11088.8 | 3018022.3 | 3008525.3 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 11651.0 | 3085893.2 | 3085638.8 | n/a |
| abi_cross_scalar_madd_null_entry | 44674.3 | 3546.0 | 3500.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.025 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.022 | 88.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 3065190ns | 3065190ns | +1.71% |
| abi_cross_scalar_madd_inproc_direct | 3013760ns | 3013760ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 3090903ns | 3090903ns | +2.56% |
| abi_cross_scalar_madd_null_entry | 6759ns | 6759ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2933250ns | base | --- | [2893739, 3198587] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 3133781ns | no significant difference | [-156160, +258247]ns | [2855984, 3190356] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 3030773ns | no significant difference | [-11613, +194354]ns | [2906051, 3320092] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_madd_null_entry | 2872ns | -2930402.0ns (-99.9%) | [-3195715, -2888959]ns | [2632, 4996] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 3376288ns | -7.4% | -0.2% | -99.9% |
| 2 | 3020887ns | +5.0% | +8.3% | -99.9% |
| 3 | 2892328ns | +8.6% | +4.8% | -99.8% |
| 4 | 2939465ns | +9.1% | -0.6% | -99.9% |
| 5 | 2927035ns | -2.1% | +3.5% | -99.9% |
| 6 | 2895150ns | -1.6% | -0.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.320 | moderate+ |
| abi_cross_scalar_madd_inproc_direct | 0.150 | ok |
| abi_cross_scalar_madd_inproc_fnptr | 0.383 | moderate+ |
| abi_cross_scalar_madd_null_entry | -0.166 | ok |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 9335129.5ns | 3060040.4ns | 305.1% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 9081212.6ns | 3008525.3ns | 301.8% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 9306306.4ns | 3085638.8ns | 301.6% | HIGH |
| abi_cross_scalar_madd_null_entry | 147290.6ns | 3500.1ns | 4208.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2847697.9-3190356.2 ns)
  2847697.9 |########################################
  2864830.8 |
  2881963.7 |
  2899096.7 |
  2916229.6 |
  2933362.5 |
  2950495.4 |
  2967628.3 |
  2984761.2 |
  3001894.2 |
  3019027.1 |
  3036160.0 |
  3053292.9 |
  3070425.8 |
  3087558.7 |
  3104691.7 |
  3121824.6 |####################
  3138957.5 |####################
  3156090.4 |
  3173223.3 |####################
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2892327.5-3198587.3 ns)
  2892327.5 |########################################
  2907640.5 |
  2922953.5 |####################
  2938266.5 |####################
  2953579.5 |
  2968892.5 |
  2984205.4 |
  2999518.4 |
  3014831.4 |####################
  3030144.4 |
  3045457.4 |
  3060770.4 |
  3076083.4 |
  3091396.4 |
  3106709.4 |
  3122022.3 |
  3137335.3 |
  3152648.3 |
  3167961.3 |
  3183274.3 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2889280.0-3320091.9 ns)
  2889280.0 |####################
  2910820.6 |####################
  2932361.2 |
  2953901.8 |
  2975442.4 |
  2996983.0 |
  3018523.6 |########################################
  3040064.2 |
  3061604.8 |
  3083145.4 |
  3104686.0 |
  3126226.5 |
  3147767.1 |
  3169307.7 |
  3190848.3 |
  3212388.9 |
  3233929.5 |
  3255470.1 |####################
  3277010.7 |
  3298551.3 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 2531.2-4995.6 ns)
   2531.2 |####################
   2654.4 |####################
   2777.6 |########################################
   2900.9 |####################
   3024.1 |
   3147.3 |
   3270.5 |
   3393.8 |
   3517.0 |
   3640.2 |
   3763.4 |
   3886.6 |
   4009.9 |
   4133.1 |
   4256.3 |
   4379.5 |
   4502.8 |
   4626.0 |
   4749.2 |
   4872.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: CV=45.3% (high variance, measurements may be unstable)
- **abi_cross_scalar_madd_null_entry**: bridge=4292.3% of algo (FFI overhead may distort results)
