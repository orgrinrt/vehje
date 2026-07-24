# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_madd_null_entry dominates: 75991% faster than the next best (abi_residency_madd_reused_buffer)

abi_residency_madd_null_entry (3.56 us) leads abi_residency_madd_reused_buffer (2.71 ms) by 75991%, a clear separation rather than a photo finish. CV 25.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.71 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_fresh_alloc is an outlier: 765.9x slower than the field

abi_residency_madd_fresh_alloc (2.73 ms) is 765.9x the fastest (3.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_madd_null_entry is fastest but the noisiest (CV 25.4%)

abi_residency_madd_null_entry wins on median (3.56 us) yet has the highest variance (CV 25.4%), while abi_residency_madd_reused_buffer is the steadiest (CV 7.3%, 2.71 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 765.9x the fastest

Fastest abi_residency_madd_null_entry (3.56 us) to slowest abi_residency_madd_fresh_alloc (2.73 ms): 765.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 3562.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 765.94x (fastest 3562.1 ns, slowest 2728326.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2881197ns | 2730912ns | 2719397ns | 2729322ns | 3189908ns | +2.21% |
| abi_residency_madd_null_entry | 6495ns | 5875ns | 5479ns | 5834ns | 7996ns | -99.77% |
| abi_residency_madd_reused_buffer | 2818804ns | 2713029ns | 2703820ns | 2710524ns | 3038715ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2878091ns | 2716848ns | 3185795ns | +2.24% | 0.000 |
| abi_residency_madd_null_entry | 3897ns | 3303ns | 4774ns | -99.86% | 0.001 |
| abi_residency_madd_reused_buffer | 2815153ns | 2701165ns | 3033034ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 47070.2 | 2756811.5 | 2878091.3 | 0 |
| abi_residency_madd_null_entry | 30945.8 | 4405.9 | 3896.8 | n/a |
| abi_residency_madd_reused_buffer | 74088.7 | 2865351.4 | 2815152.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.001 | 92.7% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2881197ns | 2881197ns | +2.21% |
| abi_residency_madd_null_entry | 6495ns | 6495ns | -99.77% |
| abi_residency_madd_reused_buffer | 2818804ns | 2818804ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2710382ns | base | --- | [2702041, 3033034] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2728327ns | no significant difference | [-39784, +211372]ns | [2720152, 3185795] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_madd_null_entry | 3562ns | -2706930.2ns (-99.9%) | [-3028261, -2698576]ns | [3355, 4774] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2710950ns | +0.5% | -99.9% |
| 2 | 2701165ns | +1.2% | -99.9% |
| 3 | 2709814ns | +0.3% | -99.9% |
| 4 | 2702918ns | +0.8% | -99.9% |
| 5 | 3247682ns | +12.0% | -99.8% |
| 6 | 2818387ns | -3.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.226 | moderate- |
| abi_residency_madd_null_entry | -0.146 | ok |
| abi_residency_madd_reused_buffer | -0.049 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8323214.4ns | 2878091.3ns | 289.2% | HIGH |
| abi_residency_madd_null_entry | 131050.8ns | 3896.8ns | 3363.0% | HIGH |
| abi_residency_madd_reused_buffer | 8597271.4ns | 2815152.7ns | 305.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2716847.9-3185794.8 ns)
  2716847.9 |########################################
  2740295.2 |
  2763742.6 |
  2787189.9 |
  2810637.3 |
  2834084.6 |
  2857532.0 |
  2880979.3 |
  2904426.7 |
  2927874.0 |
  2951321.3 |
  2974768.7 |
  2998216.0 |
  3021663.4 |
  3045110.7 |
  3068558.1 |
  3092005.4 |
  3115452.8 |
  3138900.1 |
  3162347.5 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 3302.9-4773.6 ns)
   3302.9 |####################
   3376.4 |####################
   3450.0 |####################
   3523.5 |
   3597.0 |########################################
   3670.6 |
   3744.1 |
   3817.6 |
   3891.2 |
   3964.7 |
   4038.2 |
   4111.8 |
   4185.3 |
   4258.8 |
   4332.4 |
   4405.9 |
   4479.4 |
   4553.0 |
   4626.5 |
   4700.0 |
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2701164.6-3033034.4 ns)
  2701164.6 |########################################
  2717758.1 |
  2734351.6 |
  2750945.1 |
  2767538.6 |
  2784132.1 |
  2800725.5 |
  2817319.0 |##########
  2833912.5 |
  2850506.0 |
  2867099.5 |
  2883693.0 |
  2900286.5 |
  2916880.0 |
  2933473.5 |
  2950067.0 |
  2966660.4 |
  2983253.9 |
  2999847.4 |
  3016440.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: CV=23.3% (high variance, measurements may be unstable)
- **abi_residency_madd_null_entry**: bridge=3403.4% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.8% of algo (FFI overhead may distort results)
