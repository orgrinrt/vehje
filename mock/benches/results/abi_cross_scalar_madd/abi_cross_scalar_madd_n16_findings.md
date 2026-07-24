# abi_cross_scalar (madd)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_madd_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_madd_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_madd_null_entry dominates: 108431% faster than the next best (abi_cross_scalar_madd_inproc_direct)

abi_cross_scalar_madd_null_entry (2.56 us) leads abi_cross_scalar_madd_inproc_direct (2.78 ms) by 108431%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_madd_null_entry beats baseline by 100% (significant)

abi_cross_scalar_madd_null_entry is -2.77 ms (100%) faster than baseline abi_cross_scalar_madd_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_madd_ffi_batched_scalar is an outlier: 1098.5x slower than the field

abi_cross_scalar_madd_ffi_batched_scalar (2.81 ms) is 1098.5x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_madd_inproc_direct shows alternating (throttle bounce) (autocorr -0.53)

abi_cross_scalar_madd_inproc_direct's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_madd_null_entry} vs {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} (108431% apart)

The field splits into a fast tier {abi_cross_scalar_madd_null_entry} and a slow tier {abi_cross_scalar_madd_inproc_direct, abi_cross_scalar_madd_inproc_fnptr, abi_cross_scalar_madd_ffi_batched_scalar} with a 108431% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1098.5x the fastest

Fastest abi_cross_scalar_madd_null_entry (2.56 us) to slowest abi_cross_scalar_madd_ffi_batched_scalar (2.81 ms): 1098.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_madd_null_entry** at 2557.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1098.52x (fastest 2557.3 ns, slowest 2809245.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2820084ns | 2813327ns | 2745808ns | 2810693ns | 2871309ns | +1.57% |
| abi_cross_scalar_madd_inproc_direct | 2776485ns | 2779148ns | 2718455ns | 2775985ns | 2806249ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2854279ns | 2806479ns | 2743718ns | 2788564ns | 3008132ns | +2.80% |
| abi_cross_scalar_madd_null_entry | 4936ns | 4805ns | 4727ns | 4793ns | 5256ns | -99.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2816083ns | 2742556ns | 2866948ns | +1.56% | 0.000 |
| abi_cross_scalar_madd_inproc_direct | 2772815ns | 2714950ns | 2802399ns | base | 0.000 |
| abi_cross_scalar_madd_inproc_fnptr | 2850493ns | 2740504ns | 3004120ns | +2.80% | 0.000 |
| abi_cross_scalar_madd_null_entry | 2595ns | 2483ns | 2725ns | -99.91% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 88028.0 | 2809723.1 | 2816082.6 | n/a |
| abi_cross_scalar_madd_inproc_direct | 8904.7 | 2766619.4 | 2772814.8 | n/a |
| abi_cross_scalar_madd_inproc_fnptr | 8975.4 | 2854820.2 | 2850493.1 | n/a |
| abi_cross_scalar_madd_null_entry | 29919.2 | 2680.6 | 2594.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_cross_scalar_madd_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_madd_inproc_fnptr | 0.000 | 0.1% |
| abi_cross_scalar_madd_null_entry | 0.006 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 2820084ns | 2820084ns | +1.57% |
| abi_cross_scalar_madd_inproc_direct | 2776485ns | 2776485ns | base |
| abi_cross_scalar_madd_inproc_fnptr | 2854279ns | 2854279ns | +2.80% |
| abi_cross_scalar_madd_null_entry | 4936ns | 4936ns | -99.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_madd_inproc_direct | 2775453ns | base | --- | [2740592, 2802399] | --- | --- | --- | --- |
| abi_cross_scalar_madd_ffi_batched_scalar | 2809245ns | no significant difference | [-3252, +88016]ns | [2772055, 2866948] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_madd_inproc_fnptr | 2802479ns | no significant difference | [-13123, +213305]ns | [2744880, 3004120] | no | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_madd_null_entry | 2557ns | -2772746.5ns (-99.9%) | [-2799848, -2738066]ns | [2501, 2725] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_madd_inproc_direct | abi_cross_scalar_madd_ffi_batched_scalar | abi_cross_scalar_madd_inproc_fnptr | abi_cross_scalar_madd_null_entry |
|---|---|---|---|---|
| 1 | 2766234ns | +1.5% | +1.8% | -99.9% |
| 2 | 2767024ns | +1.6% | +1.5% | -99.9% |
| 3 | 2783882ns | +1.6% | +0.5% | -99.9% |
| 4 | 2815395ns | +3.2% | +13.4% | -99.9% |
| 5 | 2714950ns | +3.2% | +0.9% | -99.9% |
| 6 | 2789403ns | -1.7% | -1.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 0.067 | ok |
| abi_cross_scalar_madd_inproc_direct | -0.528 | HIGH- (thermal bounce) |
| abi_cross_scalar_madd_inproc_fnptr | -0.280 | moderate- |
| abi_cross_scalar_madd_null_entry | -0.106 | ok |

**Consistency summary:**

- **abi_cross_scalar_madd_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_madd_inproc_fnptr**: won 1/6, lost 5/6
- **abi_cross_scalar_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_madd_ffi_batched_scalar | 8533213.7ns | 2816082.6ns | 303.0% | HIGH |
| abi_cross_scalar_madd_inproc_direct | 8308587.1ns | 2772814.8ns | 299.6% | HIGH |
| abi_cross_scalar_madd_inproc_fnptr | 8496870.3ns | 2850493.1ns | 298.1% | HIGH |
| abi_cross_scalar_madd_null_entry | 120392.6ns | 2594.6ns | 4640.2% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_madd_ffi_batched_scalar (n=6, range 2742556.2-2866947.7 ns)
  2742556.2 |########################################
  2748775.8 |
  2754995.4 |
  2761214.9 |
  2767434.5 |
  2773654.1 |
  2779873.7 |
  2786093.2 |
  2792312.8 |
  2798532.4 |########################################
  2804752.0 |########################################
  2810971.5 |########################################
  2817191.1 |
  2823410.7 |########################################
  2829630.2 |
  2835849.8 |
  2842069.4 |
  2848289.0 |
  2854508.6 |
  2860728.1 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_direct (n=6, range 2714950.0-2802399.1 ns)
  2714950.0 |####################
  2719322.5 |
  2723694.9 |
  2728067.4 |
  2732439.8 |
  2736812.3 |
  2741184.7 |
  2745557.2 |
  2749929.7 |
  2754302.1 |
  2758674.6 |
  2763047.0 |########################################
  2767419.5 |
  2771791.9 |
  2776164.4 |
  2780536.9 |####################
  2784909.3 |
  2789281.8 |####################
  2793654.2 |
  2798026.7 |
  (0 below, 1 above range)

abi_cross_scalar_madd_inproc_fnptr (n=6, range 2740504.2-3004120.0 ns)
  2740504.2 |########################################
  2753685.0 |
  2766865.8 |
  2780046.6 |
  2793227.4 |####################
  2806408.2 |########################################
  2819588.9 |
  2832769.7 |
  2845950.5 |
  2859131.3 |
  2872312.1 |
  2885492.9 |
  2898673.7 |
  2911854.5 |
  2925035.3 |
  2938216.0 |
  2951396.8 |
  2964577.6 |
  2977758.4 |
  2990939.2 |
  (0 below, 1 above range)

abi_cross_scalar_madd_null_entry (n=6, range 2483.3-2725.4 ns)
   2483.3 |########################################
   2495.4 |
   2507.5 |########################################
   2519.6 |
   2531.7 |
   2543.8 |########################################
   2555.9 |
   2568.0 |########################################
   2580.1 |########################################
   2592.2 |
   2604.4 |
   2616.5 |
   2628.6 |
   2640.7 |
   2652.8 |
   2664.9 |
   2677.0 |
   2689.1 |
   2701.2 |
   2713.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_madd_ffi_batched_scalar**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_direct**: bridge=299.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_inproc_fnptr**: bridge=301.1% of algo (FFI overhead may distort results)
- **abi_cross_scalar_madd_null_entry**: bridge=4638.6% of algo (FFI overhead may distort results)
