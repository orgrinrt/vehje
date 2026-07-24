# abi_lifecycle (madd)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_madd_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_madd_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_madd_null_entry dominates: 120202% faster than the next best (abi_lifecycle_madd_held_handle)

abi_lifecycle_madd_null_entry (2.26 us) leads abi_lifecycle_madd_held_handle (2.72 ms) by 120202%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_madd_null_entry beats baseline by 100% (significant)

abi_lifecycle_madd_null_entry is -2.72 ms (100%) faster than baseline abi_lifecycle_madd_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_madd_fresh_per_batch is an outlier: 1256.8x slower than the field

abi_lifecycle_madd_fresh_per_batch (2.84 ms) is 1256.8x the fastest (2.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_madd_null_entry} vs {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} (120202% apart)

The field splits into a fast tier {abi_lifecycle_madd_null_entry} and a slow tier {abi_lifecycle_madd_held_handle, abi_lifecycle_madd_fresh_per_column, abi_lifecycle_madd_fresh_per_batch} with a 120202% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1256.8x the fastest

Fastest abi_lifecycle_madd_null_entry (2.26 us) to slowest abi_lifecycle_madd_fresh_per_batch (2.84 ms): 1256.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_madd_null_entry** at 2262.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1256.82x (fastest 2262.3 ns, slowest 2843302.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2868855ns | 2846670ns | 2830100ns | 2841918ns | 2928639ns | +3.13% |
| abi_lifecycle_madd_fresh_per_column | 2869760ns | 2743826ns | 2736291ns | 2741926ns | 3128245ns | +3.16% |
| abi_lifecycle_madd_held_handle | 2781724ns | 2725026ns | 2720556ns | 2723972ns | 2898936ns | base |
| abi_lifecycle_madd_null_entry | 4540ns | 4508ns | 4412ns | 4505ns | 4657ns | -99.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2865281ns | 2826951ns | 2924418ns | +3.14% | 0.000 |
| abi_lifecycle_madd_fresh_per_column | 2866366ns | 2733448ns | 3124225ns | +3.18% | 0.000 |
| abi_lifecycle_madd_held_handle | 2777987ns | 2717428ns | 2894271ns | base | 0.000 |
| abi_lifecycle_madd_null_entry | 2276ns | 2215ns | 2337ns | -99.92% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 74724.5 | 2874128.8 | 2865281.0 | n/a |
| abi_lifecycle_madd_fresh_per_column | 69081.5 | 2874986.3 | 2866366.3 | 0 |
| abi_lifecycle_madd_held_handle | 74334.5 | 2782389.1 | 2777987.4 | n/a |
| abi_lifecycle_madd_null_entry | 27186.6 | 2392.6 | 2275.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_lifecycle_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_madd_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_madd_held_handle | 0.000 | 0.1% |
| abi_lifecycle_madd_null_entry | 0.014 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 2868855ns | 2868855ns | +3.13% |
| abi_lifecycle_madd_fresh_per_column | 2869760ns | 2869760ns | +3.16% |
| abi_lifecycle_madd_held_handle | 2781724ns | 2781724ns | base |
| abi_lifecycle_madd_null_entry | 4540ns | 4540ns | -99.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_madd_held_handle | 2721594ns | base | --- | [2718097, 2894271] | --- | --- | --- | --- |
| abi_lifecycle_madd_fresh_per_batch | 2843303ns | no significant difference | [-42479, +197141]ns | [2828123, 2924418] | no | 0.6875 | 0.6875 | 0 |
| abi_lifecycle_madd_fresh_per_column | 2740690ns | no significant difference | [-67548, +314474]ns | [2734184, 3124225] | no | 0.3281 | 0.2188 | 0 |
| abi_lifecycle_madd_null_entry | 2262ns | -2719340.9ns (-99.9%) | [-2891935, -2715859]ns | [2228, 2337] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_madd_held_handle | abi_lifecycle_madd_fresh_per_batch | abi_lifecycle_madd_fresh_per_column | abi_lifecycle_madd_null_entry |
|---|---|---|---|---|
| 1 | 2718766ns | +4.0% | +0.8% | -99.9% |
| 2 | 2720147ns | +4.4% | +0.8% | -99.9% |
| 3 | 2928118ns | -2.5% | -5.0% | -99.9% |
| 4 | 2860424ns | -0.5% | +21.2% | -99.9% |
| 5 | 2723041ns | +3.9% | +0.4% | -99.9% |
| 6 | 2717428ns | +10.1% | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | -0.126 | ok |
| abi_lifecycle_madd_fresh_per_column | -0.199 | ok |
| abi_lifecycle_madd_held_handle | 0.138 | ok |
| abi_lifecycle_madd_null_entry | -0.165 | ok |

**Consistency summary:**

- **abi_lifecycle_madd_fresh_per_batch**: won 2/6, lost 4/6
- **abi_lifecycle_madd_fresh_per_column**: won 1/6, lost 5/6
- **abi_lifecycle_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_madd_fresh_per_batch | 8679639.8ns | 2865281.0ns | 302.9% | HIGH |
| abi_lifecycle_madd_fresh_per_column | 8646208.2ns | 2866366.3ns | 301.6% | HIGH |
| abi_lifecycle_madd_held_handle | 8412677.6ns | 2777987.4ns | 302.8% | HIGH |
| abi_lifecycle_madd_null_entry | 115149.8ns | 2275.7ns | 5060.0% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_madd_fresh_per_batch (n=6, range 2826951.2-2924417.5 ns)
  2826951.2 |########################################
  2831824.5 |
  2836697.8 |####################
  2841571.1 |
  2846444.5 |####################
  2851317.8 |####################
  2856191.1 |
  2861064.4 |
  2865937.7 |
  2870811.0 |
  2875684.4 |
  2880557.7 |
  2885431.0 |
  2890304.3 |
  2895177.6 |
  2900050.9 |
  2904924.2 |
  2909797.6 |
  2914670.9 |
  2919544.2 |
  (0 below, 1 above range)

abi_lifecycle_madd_fresh_per_column (n=6, range 2733448.3-3124225.2 ns)
  2733448.3 |########################################
  2752987.1 |
  2772526.0 |##########
  2792064.8 |
  2811603.7 |
  2831142.5 |
  2850681.4 |
  2870220.2 |
  2889759.1 |
  2909297.9 |
  2928836.8 |
  2948375.6 |
  2967914.4 |
  2987453.3 |
  3006992.1 |
  3026531.0 |
  3046069.8 |
  3065608.7 |
  3085147.5 |
  3104686.4 |
  (0 below, 1 above range)

abi_lifecycle_madd_held_handle (n=6, range 2717427.5-2894271.2 ns)
  2717427.5 |########################################
  2726269.7 |
  2735111.9 |
  2743954.1 |
  2752796.2 |
  2761638.4 |
  2770480.6 |
  2779322.8 |
  2788165.0 |
  2797007.2 |
  2805849.4 |
  2814691.6 |
  2823533.8 |
  2832375.9 |
  2841218.1 |
  2850060.3 |
  2858902.5 |##########
  2867744.7 |
  2876586.9 |
  2885429.1 |
  (0 below, 1 above range)

abi_lifecycle_madd_null_entry (n=6, range 2214.6-2337.1 ns)
   2214.6 |####################
   2220.7 |
   2226.8 |
   2233.0 |
   2239.1 |####################
   2245.2 |
   2251.3 |
   2257.5 |####################
   2263.6 |########################################
   2269.7 |
   2275.9 |
   2282.0 |
   2288.1 |
   2294.2 |
   2300.4 |
   2306.5 |
   2312.6 |
   2318.7 |
   2324.9 |
   2331.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_madd_fresh_per_batch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_fresh_per_column**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_held_handle**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_madd_null_entry**: bridge=5085.9% of algo (FFI overhead may distort results)
