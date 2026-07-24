# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 77156% faster than the next best (abi_zig_entry_real_zig_per_w_set)

abi_zig_entry_real_zig_null (2.68 us) leads abi_zig_entry_real_zig_per_w_set (2.07 ms) by 77156%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.07 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_dispatch is an outlier: 1439.0x slower than the field

abi_zig_entry_real_zig_tail_dispatch (3.85 ms) is 1439.0x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_per_w_set shows alternating (throttle bounce) (autocorr -0.83)

abi_zig_entry_real_zig_per_w_set's per-pass series has lag-1 autocorrelation -0.83, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_runtime_w, abi_zig_entry_real_zig_tail_dispatch} (77156% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_runtime_w, abi_zig_entry_real_zig_tail_dispatch} with a 77156% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1439.0x the fastest

Fastest abi_zig_entry_real_zig_null (2.68 us) to slowest abi_zig_entry_real_zig_tail_dispatch (3.85 ms): 1439.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 2677.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1439.01x (fastest 2677.9 ns, slowest 3853516.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2077006ns | 2076594ns | 2073198ns | 2075722ns | 2080837ns | +0.22% |
| abi_zig_entry_real_zig_dispatch | 2091886ns | 2074042ns | 2064783ns | 2071169ns | 2136514ns | +0.93% |
| abi_zig_entry_real_zig_null | 5008ns | 5004ns | 4917ns | 4984ns | 5089ns | -99.76% |
| abi_zig_entry_real_zig_per_w_set | 2070315ns | 2071508ns | 2064577ns | 2069567ns | 2074305ns | -0.11% |
| abi_zig_entry_real_zig_runtime_w | 2072531ns | 2072282ns | 2068565ns | 2071235ns | 2076458ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3867833ns | 3856424ns | 3841163ns | 3852950ns | 3903493ns | +86.62% |
| abi_zig_entry_real_zig_tail_runtime_w | 3876331ns | 3855975ns | 3846931ns | 3854794ns | 3923336ns | +87.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2074288ns | 2070660ns | 2078077ns | +0.21% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2089108ns | 2062246ns | 2133472ns | +0.93% | 0.000 |
| abi_zig_entry_real_zig_null | 2680ns | 2640ns | 2716ns | -99.87% | 0.048 |
| abi_zig_entry_real_zig_per_w_set | 2067637ns | 2061944ns | 2071573ns | -0.11% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2069866ns | 2065795ns | 2073782ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3864831ns | 3838351ns | 3900285ns | +86.72% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3873197ns | 3843871ns | 3919950ns | +87.12% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 182764.6 | 2071960.7 | 2074288.3 | n/a |
| abi_zig_entry_real_zig_dispatch | 193801.4 | 2080320.7 | 2089107.5 | n/a |
| abi_zig_entry_real_zig_null | 157262.6 | 2765.4 | 2679.5 | n/a |
| abi_zig_entry_real_zig_per_w_set | 183982.6 | 2067992.8 | 2067636.8 | 1 |
| abi_zig_entry_real_zig_runtime_w | 181139.6 | 2070049.0 | 2069865.5 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 203027.9 | 3893059.8 | 3864830.5 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 211291.8 | 3906425.4 | 3873197.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_null | 0.048 | 98.6% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2077006ns | 2077006ns | +0.22% |
| abi_zig_entry_real_zig_dispatch | 2091886ns | 2091886ns | +0.93% |
| abi_zig_entry_real_zig_null | 5008ns | 5008ns | -99.76% |
| abi_zig_entry_real_zig_per_w_set | 2070315ns | 2070315ns | -0.11% |
| abi_zig_entry_real_zig_runtime_w | 2072531ns | 2072531ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3867833ns | 3867833ns | +86.62% |
| abi_zig_entry_real_zig_tail_runtime_w | 3876331ns | 3876331ns | +87.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2069640ns | base | --- | [2066175, 2073782] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2073795ns | +3747.1ns (+0.2%) | [+287, +9234]ns | [2070993, 2078077] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_real_zig_dispatch | 2071278ns | no significant difference | [-10143, +66785]ns | [2062573, 2133472] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_null | 2678ns | -2066948.8ns (-99.9%) | [-2071079, -2063530]ns | [2645, 2716] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2068843ns | no significant difference | [-8798, +3276]ns | [2062494, 2071573] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3853516ns | +1782810.1ns (+86.1%) | [+1767975, +1834110]ns | [3840691, 3900285] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3852993ns | +1784236.9ns (+86.2%) | [+1773932, +1851825]ns | [3846648, 3919950] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2070957ns | -0.0% | -0.1% | -99.9% | -0.2% | +86.3% | +86.2% |
| 2 | 2070455ns | +0.0% | +0.2% | -99.9% | +0.1% | +85.9% | +86.4% |
| 3 | 2065795ns | +0.4% | +6.1% | -99.9% | -0.2% | +90.4% | +92.7% |
| 4 | 2066555ns | +0.3% | +0.3% | -99.9% | +0.2% | +87.1% | +86.3% |
| 5 | 2076606ns | +0.1% | -0.7% | -99.9% | -0.7% | +84.8% | +85.4% |
| 6 | 2068825ns | +0.4% | -0.3% | -99.9% | +0.1% | +85.8% | +85.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.271 | moderate+ |
| abi_zig_entry_real_zig_dispatch | -0.132 | ok |
| abi_zig_entry_real_zig_null | -0.108 | ok |
| abi_zig_entry_real_zig_per_w_set | -0.825 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_runtime_w | -0.233 | moderate- |
| abi_zig_entry_real_zig_tail_dispatch | -0.049 | ok |
| abi_zig_entry_real_zig_tail_runtime_w | -0.177 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 0/6, lost 3/6
- **abi_zig_entry_real_zig_dispatch**: won 2/6, lost 3/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 3/6, lost 1/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6467862.0ns | 2074288.3ns | 311.8% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6542919.1ns | 2089107.5ns | 313.2% | HIGH |
| abi_zig_entry_real_zig_null | 306440.0ns | 2679.5ns | 11436.5% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6457657.6ns | 2067636.8ns | 312.3% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6458334.8ns | 2069865.5ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 11941449.1ns | 3864830.5ns | 309.0% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 12022576.9ns | 3873197.1ns | 310.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2070660.0-2078076.6 ns)
  2070660.0 |########################################
  2071030.8 |########################################
  2071401.7 |
  2071772.5 |
  2072143.3 |
  2072514.2 |########################################
  2072885.0 |
  2073255.8 |
  2073626.7 |
  2073997.5 |
  2074368.3 |
  2074739.2 |########################################
  2075110.0 |
  2075480.8 |
  2075851.7 |
  2076222.5 |
  2076593.3 |
  2076964.2 |
  2077335.0 |
  2077705.8 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2062246.2-2133471.8 ns)
  2062246.2 |########################################
  2065807.5 |####################
  2069368.8 |
  2072930.0 |########################################
  2076491.3 |
  2080052.6 |
  2083613.9 |
  2087175.2 |
  2090736.5 |
  2094297.7 |
  2097859.0 |
  2101420.3 |
  2104981.6 |
  2108542.9 |
  2112104.2 |
  2115665.4 |
  2119226.7 |
  2122788.0 |
  2126349.3 |
  2129910.6 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 2640.4-2715.6 ns)
   2640.4 |####################
   2644.2 |
   2647.9 |####################
   2651.7 |####################
   2655.4 |
   2659.2 |
   2663.0 |
   2666.7 |
   2670.5 |
   2674.2 |
   2678.0 |
   2681.8 |
   2685.5 |
   2689.3 |
   2693.0 |
   2696.8 |
   2700.6 |########################################
   2704.3 |
   2708.1 |
   2711.8 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2061943.8-2071572.7 ns)
  2061943.8 |########################################
  2062425.2 |
  2062906.7 |########################################
  2063388.1 |
  2063869.6 |
  2064351.0 |
  2064832.5 |
  2065313.9 |
  2065795.4 |
  2066276.8 |
  2066758.2 |########################################
  2067239.7 |
  2067721.1 |
  2068202.6 |
  2068684.0 |
  2069165.5 |
  2069646.9 |
  2070128.4 |
  2070609.8 |########################################
  2071091.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2065795.0-2073781.6 ns)
  2065795.0 |########################################
  2066194.3 |########################################
  2066593.7 |
  2066993.0 |
  2067392.3 |
  2067791.7 |
  2068191.0 |
  2068590.3 |########################################
  2068989.7 |
  2069389.0 |
  2069788.3 |
  2070187.7 |########################################
  2070587.0 |########################################
  2070986.3 |
  2071385.7 |
  2071785.0 |
  2072184.3 |
  2072583.7 |
  2072983.0 |
  2073382.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3838351.2-3900284.8 ns)
  3838351.2 |########################################
  3841447.9 |########################################
  3844544.6 |
  3847641.2 |########################################
  3850737.9 |
  3853834.6 |
  3856931.3 |########################################
  3860028.0 |
  3863124.6 |
  3866221.3 |########################################
  3869318.0 |
  3872414.7 |
  3875511.4 |
  3878608.0 |
  3881704.7 |
  3884801.4 |
  3887898.1 |
  3890994.8 |
  3894091.4 |
  3897188.1 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3843870.8-3919950.0 ns)
  3843870.8 |####################
  3847674.8 |########################################
  3851478.7 |
  3855282.7 |########################################
  3859086.6 |
  3862890.6 |
  3866694.6 |
  3870498.5 |
  3874302.5 |
  3878106.4 |
  3881910.4 |
  3885714.4 |
  3889518.3 |
  3893322.3 |
  3897126.2 |
  3900930.2 |
  3904734.2 |
  3908538.1 |
  3912342.1 |
  3916146.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=311.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=11410.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=307.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.6% of algo (FFI overhead may distort results)
