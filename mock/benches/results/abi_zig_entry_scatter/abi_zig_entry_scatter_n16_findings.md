# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 83471% faster than the next best (abi_zig_entry_scatter_zig_dispatch)

abi_zig_entry_scatter_zig_null (2.51 us) leads abi_zig_entry_scatter_zig_dispatch (2.10 ms) by 83471%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.10 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_dispatch is an outlier: 1545.7x slower than the field

abi_zig_entry_scatter_zig_tail_dispatch (3.88 ms) is 1545.7x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_scatter_zig_anchor shows alternating (throttle bounce) (autocorr -0.59)

abi_zig_entry_scatter_zig_anchor's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} (83471% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} with a 83471% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1545.7x the fastest

Fastest abi_zig_entry_scatter_zig_null (2.51 us) to slowest abi_zig_entry_scatter_zig_tail_dispatch (3.88 ms): 1545.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_zig_entry_scatter_zig_dispatch's edge over baseline is significant but tiny (30 ns, 0.00%)

abi_zig_entry_scatter_zig_dispatch differs from baseline abi_zig_entry_scatter_zig_runtime_w by 30 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 2509.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1545.71x (fastest 2509.6 ns, slowest 3879026.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2101663ns | 2104079ns | 2090186ns | 2099892ns | 2110057ns | +0.04% |
| abi_zig_entry_scatter_zig_dispatch | 2109973ns | 2099989ns | 2095895ns | 2099428ns | 2132831ns | +0.44% |
| abi_zig_entry_scatter_zig_null | 4973ns | 4846ns | 4739ns | 4830ns | 5305ns | -99.76% |
| abi_zig_entry_scatter_zig_per_w_set | 2105600ns | 2103711ns | 2101615ns | 2103289ns | 2111059ns | +0.23% |
| abi_zig_entry_scatter_zig_runtime_w | 2100766ns | 2102701ns | 2083409ns | 2101192ns | 2108807ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3993094ns | 3881866ns | 3854540ns | 3879810ns | 4232298ns | +90.08% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3868713ns | 3869851ns | 3855613ns | 3865298ns | 3880385ns | +84.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2099055ns | 2087682ns | 2107360ns | +0.04% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2107201ns | 2093144ns | 2129940ns | +0.43% | 0.000 |
| abi_zig_entry_scatter_zig_null | 2579ns | 2479ns | 2747ns | -99.88% | 0.006 |
| abi_zig_entry_scatter_zig_per_w_set | 2102862ns | 2098995ns | 2108260ns | +0.23% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2098125ns | 2080868ns | 2106122ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3990068ns | 3851625ns | 4228986ns | +90.17% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3865942ns | 3852838ns | 3877550ns | +84.26% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 176600.7 | 2097408.1 | 2099055.0 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 185910.8 | 2107004.1 | 2107201.0 | n/a |
| abi_zig_entry_scatter_zig_null | 164848.5 | 2811.6 | 2579.0 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 181806.3 | 2102856.3 | 2102862.3 | n/a |
| abi_zig_entry_scatter_zig_runtime_w | 176564.4 | 2098648.5 | 2098125.1 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 206150.9 | 3973952.7 | 3990068.1 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 188413.8 | 3869088.1 | 3865941.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_null | 0.006 | 98.8% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2101663ns | 2101663ns | +0.04% |
| abi_zig_entry_scatter_zig_dispatch | 2109973ns | 2109973ns | +0.44% |
| abi_zig_entry_scatter_zig_null | 4973ns | 4973ns | -99.76% |
| abi_zig_entry_scatter_zig_per_w_set | 2105600ns | 2105600ns | +0.23% |
| abi_zig_entry_scatter_zig_runtime_w | 2100766ns | 2100766ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3993094ns | 3993094ns | +90.08% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3868713ns | 3868713ns | +84.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2100021ns | base | --- | [2088232, 2106122] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2101526ns | no significant difference | [-10586, +7671]ns | [2088279, 2107360] | no | 1.0000 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2097244ns | no significant difference | [-8160, +35358]ns | [2094419, 2129940] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_null | 2510ns | -2097338.1ns (-99.9%) | [-2103555, -2085746]ns | [2480, 2747] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2101040ns | no significant difference | [-2393, +13840]ns | [2099287, 2108260] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3879027ns | +1779166.5ns (+84.7%) | [+1758539, +2138123]ns | [3862192, 4228986] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3867113ns | +1768041.2ns (+84.2%) | [+1757718, +1777690]ns | [3853161, 3877550] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2095596ns | +0.4% | +0.1% | -99.9% | +0.3% | +85.1% | +84.8% |
| 2 | 2080868ns | +0.3% | +0.7% | -99.9% | +0.9% | +86.6% | +85.2% |
| 3 | 2104125ns | +0.3% | -0.0% | -99.9% | -0.2% | +84.3% | +84.5% |
| 4 | 2108120ns | -0.4% | -0.7% | -99.9% | -0.0% | +82.7% | +83.7% |
| 5 | 2099185ns | +0.3% | -0.1% | -99.9% | +0.5% | +84.5% | +83.5% |
| 6 | 2100857ns | -0.6% | +2.7% | -99.9% | -0.1% | +117.8% | +83.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | -0.590 | HIGH- (thermal bounce) |
| abi_zig_entry_scatter_zig_dispatch | -0.036 | ok |
| abi_zig_entry_scatter_zig_null | -0.172 | ok |
| abi_zig_entry_scatter_zig_per_w_set | 0.138 | ok |
| abi_zig_entry_scatter_zig_runtime_w | 0.030 | ok |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.032 | ok |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.338 | moderate- |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 2/6, lost 4/6
- **abi_zig_entry_scatter_zig_dispatch**: won 1/6, lost 2/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 1/6, lost 3/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6538556.6ns | 2099055.0ns | 311.5% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6696479.4ns | 2107201.0ns | 317.8% | HIGH |
| abi_zig_entry_scatter_zig_null | 312605.6ns | 2579.0ns | 12121.4% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6554904.9ns | 2102862.3ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6534858.8ns | 2098125.1ns | 311.5% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 12128230.6ns | 3990068.1ns | 304.0% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 11865553.6ns | 3865941.5ns | 306.9% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2087681.7-2107359.6 ns)
  2087681.7 |########################################
  2088665.6 |########################################
  2089649.5 |
  2090633.4 |
  2091617.3 |
  2092601.2 |
  2093585.1 |
  2094569.0 |
  2095552.9 |
  2096536.8 |
  2097520.6 |
  2098504.5 |########################################
  2099488.4 |
  2100472.3 |
  2101456.2 |
  2102440.1 |
  2103424.0 |########################################
  2104407.9 |########################################
  2105391.8 |
  2106375.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2093144.2-2129940.5 ns)
  2093144.2 |####################
  2094984.0 |########################################
  2096823.8 |####################
  2098663.6 |
  2100503.5 |
  2102343.3 |####################
  2104183.1 |
  2106022.9 |
  2107862.7 |
  2109702.5 |
  2111542.3 |
  2113382.1 |
  2115222.0 |
  2117061.8 |
  2118901.6 |
  2120741.4 |
  2122581.2 |
  2124421.0 |
  2126260.8 |
  2128100.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 2479.2-2746.8 ns)
   2479.2 |########################################
   2492.6 |
   2506.0 |
   2519.3 |#############
   2532.7 |
   2546.1 |
   2559.5 |
   2572.9 |
   2586.3 |
   2599.6 |#############
   2613.0 |
   2626.4 |
   2639.8 |
   2653.2 |
   2666.6 |
   2679.9 |
   2693.3 |
   2706.7 |
   2720.1 |
   2733.5 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2098995.4-2108259.5 ns)
  2098995.4 |########################################
  2099458.6 |########################################
  2099921.8 |
  2100385.0 |########################################
  2100848.2 |
  2101311.4 |########################################
  2101774.6 |
  2102237.9 |
  2102701.1 |
  2103164.3 |
  2103627.5 |
  2104090.7 |
  2104553.9 |
  2105017.1 |
  2105480.3 |
  2105943.5 |
  2106406.7 |
  2106869.9 |
  2107333.1 |########################################
  2107796.3 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2080868.3-2106122.5 ns)
  2080868.3 |########################################
  2082131.0 |
  2083393.7 |
  2084656.4 |
  2085919.1 |
  2087181.9 |
  2088444.6 |
  2089707.3 |
  2090970.0 |
  2092232.7 |
  2093495.4 |
  2094758.1 |########################################
  2096020.8 |
  2097283.5 |
  2098546.2 |########################################
  2099809.0 |########################################
  2101071.7 |
  2102334.4 |
  2103597.1 |########################################
  2104859.8 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3851625.0-4228985.6 ns)
  3851625.0 |##########
  3870493.0 |########################################
  3889361.1 |
  3908229.1 |
  3927097.1 |
  3945965.1 |
  3964833.2 |
  3983701.2 |
  4002569.2 |
  4021437.3 |
  4040305.3 |
  4059173.3 |
  4078041.4 |
  4096909.4 |
  4115777.4 |
  4134645.4 |
  4153513.5 |
  4172381.5 |
  4191249.5 |
  4210117.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3852837.5-3877549.8 ns)
  3852837.5 |########################################
  3854073.1 |
  3855308.7 |
  3856544.3 |
  3857780.0 |
  3859015.6 |
  3860251.2 |
  3861486.8 |####################
  3862722.4 |
  3863958.0 |
  3865193.6 |
  3866429.2 |
  3867664.9 |
  3868900.5 |
  3870136.1 |
  3871371.7 |####################
  3872607.3 |####################
  3873842.9 |
  3875078.5 |
  3876314.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=11941.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=306.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=307.0% of algo (FFI overhead may distort results)
