# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 28574% faster than the next best (abi_boundary_w_real_soa_per_w)

abi_boundary_w_real_null_entry (3.07 us) leads abi_boundary_w_real_soa_per_w (879.33 us) by 28574%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.14 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_anchor is an outlier: 701.3x slower than the field

abi_boundary_w_real_scalar_anchor (2.15 ms) is 701.3x the fastest (3.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor} (28574% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor} with a 28574% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 701.3x the fastest

Fastest abi_boundary_w_real_null_entry (3.07 us) to slowest abi_boundary_w_real_scalar_anchor (2.15 ms): 701.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 3066.7 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 701.35x (fastest 3066.7 ns, slowest 2150788.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 5413ns | 5370ns | 5102ns | 5313ns | 5718ns | -99.75% |
| abi_boundary_w_real_scalar_anchor | 2154247ns | 2153599ns | 2136670ns | 2148934ns | 2171005ns | +0.22% |
| abi_boundary_w_real_scalar_dispatch | 2141269ns | 2138234ns | 2130634ns | 2137218ns | 2152664ns | -0.39% |
| abi_boundary_w_real_scalar_per_w | 2142074ns | 2141875ns | 2138064ns | 2140650ns | 2146215ns | -0.35% |
| abi_boundary_w_real_scalar_runtime_w | 2149584ns | 2150081ns | 2140627ns | 2148037ns | 2156384ns | base |
| abi_boundary_w_real_soa_dispatch | 892153ns | 890796ns | 889436ns | 890433ns | 896092ns | -58.50% |
| abi_boundary_w_real_soa_per_w | 880569ns | 881678ns | 876081ns | 880369ns | 883114ns | -59.04% |
| abi_boundary_w_real_soa_runtime_w | 885641ns | 885996ns | 881238ns | 884452ns | 889626ns | -58.80% |
| abi_boundary_w_real_zig_runtime_w | 2075659ns | 2075773ns | 2058038ns | 2074747ns | 2085838ns | -3.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 3060ns | 2938ns | 3170ns | -99.86% | 0.003 |
| abi_boundary_w_real_scalar_anchor | 2151430ns | 2134206ns | 2167796ns | +0.21% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2138586ns | 2128042ns | 2149809ns | -0.39% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2139546ns | 2135520ns | 2143601ns | -0.34% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2146873ns | 2137988ns | 2153494ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 889710ns | 886971ns | 893693ns | -58.56% | 0.000 |
| abi_boundary_w_real_soa_per_w | 878209ns | 873845ns | 880720ns | -59.09% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 883287ns | 878899ns | 887154ns | -58.86% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2073000ns | 2055655ns | 2083125ns | -3.44% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 27808.3 | 3154.7 | 3059.9 | n/a |
| abi_boundary_w_real_scalar_anchor | 47797.9 | 2148975.0 | 2151430.0 | n/a |
| abi_boundary_w_real_scalar_dispatch | 43286.9 | 2139684.6 | 2138585.8 | 0 |
| abi_boundary_w_real_scalar_per_w | 39438.3 | 2138558.4 | 2139546.2 | n/a |
| abi_boundary_w_real_scalar_runtime_w | 42874.5 | 2146117.0 | 2146873.3 | n/a |
| abi_boundary_w_real_soa_dispatch | 33868.1 | 890487.3 | 889709.8 | n/a |
| abi_boundary_w_real_soa_per_w | 32227.9 | 878910.7 | 878209.2 | n/a |
| abi_boundary_w_real_soa_runtime_w | 32335.5 | 885008.5 | 883286.5 | n/a |
| abi_boundary_w_real_zig_runtime_w | 185357.2 | 2075643.8 | 2072999.9 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.003 | 95.8% |
| abi_boundary_w_real_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_real_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_real_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_real_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_real_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_real_null_entry | 5413ns | 5413ns | -99.75% |
| abi_boundary_w_real_scalar_anchor | 2154247ns | 2154247ns | +0.22% |
| abi_boundary_w_real_scalar_dispatch | 2141269ns | 2141269ns | -0.39% |
| abi_boundary_w_real_scalar_per_w | 2142074ns | 2142074ns | -0.35% |
| abi_boundary_w_real_scalar_runtime_w | 2149584ns | 2149584ns | base |
| abi_boundary_w_real_soa_dispatch | 892153ns | 892153ns | -58.50% |
| abi_boundary_w_real_soa_per_w | 880569ns | 880569ns | -59.04% |
| abi_boundary_w_real_soa_runtime_w | 885641ns | 885641ns | -58.80% |
| abi_boundary_w_real_zig_runtime_w | 2075659ns | 2075659ns | -3.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2147639ns | base | --- | [2139487, 2153494] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 3067ns | -2144525.7ns (-99.9%) | [-2150551, -2136363]ns | [2943, 3170] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2150789ns | no significant difference | [-9595, +21923]ns | [2135705, 2167796] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2135592ns | no significant difference | [-19344, +2128]ns | [2130357, 2149809] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_per_w | 2139449ns | -7232.5ns (-0.3%) | [-12664, -2084]ns | [2135589, 2143601] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_soa_dispatch | 888424ns | -1258319.8ns (-58.6%) | [-1263790, -1249381]ns | [887012, 893693] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_soa_per_w | 879334ns | -1267658.4ns (-59.0%) | [-1276100, -1262234]ns | [874574, 880720] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_soa_runtime_w | 883690ns | -1267348.8ns (-59.0%) | [-1269138, -1254274]ns | [879016, 887154] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2072974ns | -68406.9ns (-3.2%) | [-87650, -65564]ns | [2062901, 2083125] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2146789ns | -99.9% | +0.0% | -0.9% | -0.3% | -58.7% | -59.0% | -59.1% | -3.2% |
| 2 | 2137988ns | -99.9% | -0.2% | -0.2% | -0.1% | -58.4% | -59.1% | -58.7% | -3.0% |
| 3 | 2148488ns | -99.9% | +0.3% | -0.6% | -0.4% | -58.7% | -59.0% | -59.1% | -4.3% |
| 4 | 2140986ns | -99.9% | +1.8% | +0.1% | -0.1% | -58.4% | -58.9% | -58.6% | -3.2% |
| 5 | 2154375ns | -99.9% | +0.1% | +0.1% | -0.3% | -58.8% | -59.4% | -58.8% | -3.1% |
| 6 | 2152612ns | -99.9% | -0.7% | -0.9% | -0.8% | -58.4% | -59.1% | -58.9% | -3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.002 | ok |
| abi_boundary_w_real_scalar_anchor | 0.142 | ok |
| abi_boundary_w_real_scalar_dispatch | 0.022 | ok |
| abi_boundary_w_real_scalar_per_w | -0.384 | moderate- |
| abi_boundary_w_real_scalar_runtime_w | -0.118 | ok |
| abi_boundary_w_real_soa_dispatch | -0.343 | moderate- |
| abi_boundary_w_real_soa_per_w | -0.468 | moderate- |
| abi_boundary_w_real_soa_runtime_w | -0.019 | ok |
| abi_boundary_w_real_zig_runtime_w | -0.077 | ok |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 2/6, lost 3/6
- **abi_boundary_w_real_scalar_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_real_scalar_per_w**: won 5/6, lost 0/6
- **abi_boundary_w_real_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 119608.7ns | 3059.9ns | 3908.9% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6502046.9ns | 2151430.0ns | 302.2% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6462906.8ns | 2138585.8ns | 302.2% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6457233.9ns | 2139546.2ns | 301.8% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6483030.7ns | 2146873.3ns | 302.0% | HIGH |
| abi_boundary_w_real_soa_dispatch | 2706312.9ns | 889709.8ns | 304.2% | HIGH |
| abi_boundary_w_real_soa_per_w | 2671074.4ns | 878209.2ns | 304.2% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 2686149.6ns | 883286.5ns | 304.1% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6478202.7ns | 2072999.9ns | 312.5% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 2938.3-3170.4 ns)
   2938.3 |########################################
   2949.9 |
   2961.5 |
   2973.1 |
   2984.7 |
   2996.3 |
   3007.9 |
   3019.5 |
   3031.1 |
   3042.7 |
   3054.4 |####################
   3066.0 |####################
   3077.6 |
   3089.2 |
   3100.8 |
   3112.4 |
   3124.0 |
   3135.6 |
   3147.2 |####################
   3158.8 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2134206.2-2167796.2 ns)
  2134206.2 |########################################
  2135885.7 |########################################
  2137565.2 |
  2139244.7 |
  2140924.2 |
  2142603.7 |
  2144283.2 |
  2145962.7 |########################################
  2147642.2 |
  2149321.7 |
  2151001.2 |
  2152680.7 |
  2154360.2 |########################################
  2156039.7 |########################################
  2157719.2 |
  2159398.7 |
  2161078.2 |
  2162757.7 |
  2164437.2 |
  2166116.7 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2128042.5-2149809.1 ns)
  2128042.5 |########################################
  2129130.8 |
  2130219.2 |
  2131307.5 |
  2132395.8 |########################################
  2133484.2 |
  2134572.5 |########################################
  2135660.8 |########################################
  2136749.2 |
  2137837.5 |
  2138925.8 |
  2140014.2 |
  2141102.5 |
  2142190.8 |########################################
  2143279.2 |
  2144367.5 |
  2145455.8 |
  2146544.2 |
  2147632.5 |
  2148720.8 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2135520.0-2143601.2 ns)
  2135520.0 |########################################
  2135924.1 |
  2136328.1 |
  2136732.2 |
  2137136.2 |
  2137540.3 |
  2137944.4 |
  2138348.4 |
  2138752.5 |
  2139156.6 |####################
  2139560.6 |####################
  2139964.7 |####################
  2140368.8 |
  2140772.8 |
  2141176.9 |
  2141580.9 |
  2141985.0 |
  2142389.1 |
  2142793.1 |
  2143197.2 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2137988.3-2153494.0 ns)
  2137988.3 |########################################
  2138763.6 |
  2139538.9 |
  2140314.1 |########################################
  2141089.4 |
  2141864.7 |
  2142640.0 |
  2143415.3 |
  2144190.6 |
  2144965.8 |
  2145741.1 |
  2146516.4 |########################################
  2147291.7 |
  2148067.0 |########################################
  2148842.3 |
  2149617.5 |
  2150392.8 |
  2151168.1 |
  2151943.4 |########################################
  2152718.7 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 886970.8-893693.1 ns)
  886970.8 |########################################
  887306.9 |
  887643.0 |
  887979.1 |####################
  888315.3 |####################
  888651.4 |
  888987.5 |
  889323.6 |
  889659.7 |
  889995.8 |
  890332.0 |
  890668.1 |
  891004.2 |
  891340.3 |
  891676.4 |####################
  892012.5 |
  892348.6 |
  892684.8 |
  893020.9 |
  893357.0 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 873845.4-880720.4 ns)
  873845.4 |########################################
  874189.2 |
  874532.9 |
  874876.7 |
  875220.4 |########################################
  875564.2 |
  875907.9 |
  876251.7 |
  876595.4 |
  876939.2 |
  877282.9 |
  877626.7 |
  877970.4 |
  878314.2 |
  878657.9 |
  879001.7 |########################################
  879345.4 |########################################
  879689.2 |
  880032.9 |
  880376.7 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 878898.7-887153.8 ns)
  878898.7 |########################################
  879311.5 |
  879724.2 |
  880137.0 |
  880549.7 |
  880962.5 |
  881375.2 |
  881788.0 |
  882200.7 |
  882613.5 |
  883026.2 |
  883439.0 |########################################
  883851.7 |
  884264.5 |
  884677.2 |
  885090.0 |
  885502.7 |
  885915.5 |
  886328.2 |####################
  886741.0 |
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2055655.0-2083125.2 ns)
  2055655.0 |####################
  2057028.5 |
  2058402.0 |
  2059775.5 |
  2061149.0 |
  2062522.6 |
  2063896.1 |
  2065269.6 |
  2066643.1 |
  2068016.6 |
  2069390.1 |####################
  2070763.6 |
  2072137.1 |########################################
  2073510.6 |
  2074884.1 |
  2076257.6 |
  2077631.2 |####################
  2079004.7 |
  2080378.2 |
  2081751.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=3888.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=312.8% of algo (FFI overhead may distort results)
