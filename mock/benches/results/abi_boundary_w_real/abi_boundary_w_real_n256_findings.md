# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 28233% faster than the next best (abi_boundary_w_real_soa_dispatch)

abi_boundary_w_real_null_entry (3.13 us) leads abi_boundary_w_real_soa_dispatch (888.11 us) by 28233%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_anchor is an outlier: 690.8x slower than the field

abi_boundary_w_real_scalar_anchor (2.17 ms) is 690.8x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_real_null_entry shows alternating (throttle bounce) (autocorr -0.65)

abi_boundary_w_real_null_entry's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor} (28233% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_scalar_anchor} with a 28233% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 690.8x the fastest

Fastest abi_boundary_w_real_null_entry (3.13 us) to slowest abi_boundary_w_real_scalar_anchor (2.17 ms): 690.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 3134.6 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 690.82x (fastest 3134.6 ns, slowest 2165432.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 5451ns | 5367ns | 5291ns | 5351ns | 5683ns | -99.75% |
| abi_boundary_w_real_scalar_anchor | 2167927ns | 2168084ns | 2159211ns | 2167270ns | 2173270ns | +0.17% |
| abi_boundary_w_real_scalar_dispatch | 2162807ns | 2160791ns | 2158977ns | 2160397ns | 2168336ns | -0.06% |
| abi_boundary_w_real_scalar_per_w | 2164876ns | 2161537ns | 2158962ns | 2161070ns | 2173543ns | +0.03% |
| abi_boundary_w_real_scalar_runtime_w | 2164186ns | 2162907ns | 2155154ns | 2162299ns | 2171531ns | base |
| abi_boundary_w_real_soa_dispatch | 891050ns | 890442ns | 885485ns | 888887ns | 897076ns | -58.83% |
| abi_boundary_w_real_soa_per_w | 895004ns | 895481ns | 887321ns | 893475ns | 901138ns | -58.64% |
| abi_boundary_w_real_soa_runtime_w | 891410ns | 893118ns | 884011ns | 890627ns | 896283ns | -58.81% |
| abi_boundary_w_real_zig_runtime_w | 2091888ns | 2092730ns | 2086492ns | 2092117ns | 2094241ns | -3.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 3193ns | 3107ns | 3336ns | -99.85% | 0.080 |
| abi_boundary_w_real_scalar_anchor | 2165269ns | 2156595ns | 2170559ns | +0.18% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2160145ns | 2156309ns | 2165554ns | -0.06% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2162201ns | 2156472ns | 2170651ns | +0.04% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2161442ns | 2152595ns | 2168668ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 888603ns | 883024ns | 894536ns | -58.89% | 0.000 |
| abi_boundary_w_real_soa_per_w | 892539ns | 884961ns | 898622ns | -58.71% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 888882ns | 881367ns | 893753ns | -58.88% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2089085ns | 2083886ns | 2091471ns | -3.35% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 27391.2 | 3196.2 | 3192.6 | n/a |
| abi_boundary_w_real_scalar_anchor | 43587.8 | 2163710.5 | 2165269.3 | 0 |
| abi_boundary_w_real_scalar_dispatch | 42763.0 | 2160518.3 | 2160145.1 | n/a |
| abi_boundary_w_real_scalar_per_w | 44307.8 | 2161570.5 | 2162200.7 | n/a |
| abi_boundary_w_real_scalar_runtime_w | 44295.3 | 2160908.8 | 2161441.7 | n/a |
| abi_boundary_w_real_soa_dispatch | 35967.1 | 888832.0 | 888603.4 | n/a |
| abi_boundary_w_real_soa_per_w | 36584.6 | 892393.5 | 892538.6 | n/a |
| abi_boundary_w_real_soa_runtime_w | 36824.9 | 888403.7 | 888882.2 | n/a |
| abi_boundary_w_real_zig_runtime_w | 189520.2 | 2089044.3 | 2089085.3 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.082 | 99.1% |
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
| abi_boundary_w_real_null_entry | 5451ns | 5451ns | -99.75% |
| abi_boundary_w_real_scalar_anchor | 2167927ns | 2167927ns | +0.17% |
| abi_boundary_w_real_scalar_dispatch | 2162807ns | 2162807ns | -0.06% |
| abi_boundary_w_real_scalar_per_w | 2164876ns | 2164876ns | +0.03% |
| abi_boundary_w_real_scalar_runtime_w | 2164186ns | 2164186ns | base |
| abi_boundary_w_real_soa_dispatch | 891050ns | 891050ns | -58.83% |
| abi_boundary_w_real_soa_per_w | 895004ns | 895004ns | -58.64% |
| abi_boundary_w_real_soa_runtime_w | 891410ns | 891410ns | -58.81% |
| abi_boundary_w_real_zig_runtime_w | 2091888ns | 2091888ns | -3.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2160213ns | base | --- | [2155445, 2168668] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 3135ns | -2156877.3ns (-99.8%) | [-2165533, -2152337]ns | [3108, 3336] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2165432ns | no significant difference | [-8851, +13306]ns | [2159817, 2170559] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2158116ns | no significant difference | [-7715, +5055]ns | [2156765, 2165554] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_per_w | 2158944ns | no significant difference | [-4026, +6982]ns | [2157006, 2170651] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_real_soa_dispatch | 888114ns | -1271821.9ns (-58.9%) | [-1283356, -1263337]ns | [883160, 894536] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_per_w | 892990ns | -1263784.1ns (-58.5%) | [-1282385, -1260540]ns | [886004, 898622] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_runtime_w | 890595ns | -1269506.8ns (-58.8%) | [-1285908, -1262264]ns | [882299, 893753] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2089806ns | -71180.8ns (-3.3%) | [-81815, -64074]ns | [2085978, 2091471] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2152595ns | -99.9% | +0.6% | +0.2% | +0.2% | -58.7% | -58.7% | -58.4% | -2.9% |
| 2 | 2160396ns | -99.8% | +0.2% | -0.1% | -0.1% | -58.5% | -58.9% | -58.7% | -3.5% |
| 3 | 2158295ns | -99.9% | +0.6% | -0.0% | -0.0% | -59.1% | -58.4% | -58.7% | -3.1% |
| 4 | 2160030ns | -99.8% | +0.4% | -0.2% | +0.5% | -58.7% | -58.4% | -59.2% | -3.2% |
| 5 | 2160953ns | -99.9% | +0.1% | +0.3% | -0.0% | -59.1% | -58.5% | -58.8% | -3.4% |
| 6 | 2176382ns | -99.9% | -0.9% | -0.5% | -0.2% | -59.2% | -59.3% | -59.4% | -4.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.647 | HIGH- (thermal bounce) |
| abi_boundary_w_real_scalar_anchor | 0.245 | moderate+ |
| abi_boundary_w_real_scalar_dispatch | 0.244 | moderate+ |
| abi_boundary_w_real_scalar_per_w | -0.092 | ok |
| abi_boundary_w_real_scalar_runtime_w | 0.033 | ok |
| abi_boundary_w_real_soa_dispatch | -0.493 | moderate- |
| abi_boundary_w_real_soa_per_w | 0.100 | ok |
| abi_boundary_w_real_soa_runtime_w | 0.006 | ok |
| abi_boundary_w_real_zig_runtime_w | -0.481 | moderate- |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 1/6, lost 4/6
- **abi_boundary_w_real_scalar_dispatch**: won 2/6, lost 2/6
- **abi_boundary_w_real_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_real_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 121266.5ns | 3192.6ns | 3798.4% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6534372.8ns | 2165269.3ns | 301.8% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6527515.8ns | 2160145.1ns | 302.2% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6527274.6ns | 2162200.7ns | 301.9% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6533344.6ns | 2161441.7ns | 302.3% | HIGH |
| abi_boundary_w_real_soa_dispatch | 2704423.5ns | 888603.4ns | 304.3% | HIGH |
| abi_boundary_w_real_soa_per_w | 2715901.3ns | 892538.6ns | 304.3% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 2703808.7ns | 888882.2ns | 304.2% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6526829.4ns | 2089085.3ns | 312.4% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 3107.1-3335.6 ns)
   3107.1 |########################################
   3118.5 |####################
   3129.9 |####################
   3141.4 |
   3152.8 |
   3164.2 |
   3175.7 |
   3187.1 |
   3198.5 |
   3209.9 |
   3221.4 |
   3232.8 |
   3244.2 |
   3255.6 |
   3267.1 |
   3278.5 |
   3289.9 |
   3301.3 |
   3312.8 |
   3324.2 |####################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2156594.6-2170559.0 ns)
  2156594.6 |########################################
  2157292.8 |
  2157991.0 |
  2158689.3 |
  2159387.5 |
  2160085.7 |
  2160783.9 |
  2161482.1 |
  2162180.3 |
  2162878.6 |########################################
  2163576.8 |
  2164275.0 |########################################
  2164973.2 |
  2165671.4 |########################################
  2166369.6 |
  2167067.9 |
  2167766.1 |
  2168464.3 |
  2169162.5 |########################################
  2169860.7 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2156308.8-2165554.1 ns)
  2156308.8 |########################################
  2156771.1 |########################################
  2157233.3 |
  2157695.6 |########################################
  2158157.9 |########################################
  2158620.1 |
  2159082.4 |
  2159544.7 |
  2160006.9 |
  2160469.2 |
  2160931.5 |
  2161393.7 |
  2161856.0 |
  2162318.3 |
  2162780.5 |
  2163242.8 |
  2163705.1 |
  2164167.3 |
  2164629.6 |########################################
  2165091.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2156472.1-2170651.2 ns)
  2156472.1 |####################
  2157181.1 |########################################
  2157890.0 |
  2158599.0 |
  2159307.9 |
  2160016.9 |####################
  2160725.8 |
  2161434.8 |
  2162143.8 |
  2162852.7 |
  2163561.7 |
  2164270.6 |
  2164979.6 |
  2165688.5 |
  2166397.5 |
  2167106.5 |
  2167815.4 |
  2168524.4 |
  2169233.3 |
  2169942.3 |####################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2152594.6-2168667.7 ns)
  2152594.6 |####################
  2153398.3 |
  2154201.9 |
  2155005.6 |
  2155809.2 |
  2156612.9 |
  2157416.5 |
  2158220.2 |####################
  2159023.8 |
  2159827.5 |########################################
  2160631.2 |####################
  2161434.8 |
  2162238.5 |
  2163042.1 |
  2163845.8 |
  2164649.4 |
  2165453.1 |
  2166256.7 |
  2167060.4 |
  2167864.0 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 883023.7-894535.6 ns)
  883023.7 |########################################
  883599.3 |
  884174.9 |
  884750.5 |
  885326.1 |
  885901.7 |
  886477.3 |
  887052.9 |####################
  887628.5 |
  888204.1 |
  888779.7 |####################
  889355.3 |
  889930.9 |
  890506.5 |
  891082.1 |####################
  891657.7 |
  892233.3 |
  892808.9 |
  893384.5 |
  893960.1 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 884960.8-898622.5 ns)
  884960.8 |########################################
  885643.9 |
  886327.0 |
  887010.1 |########################################
  887693.1 |
  888376.2 |
  889059.3 |
  889742.4 |########################################
  890425.5 |
  891108.6 |
  891791.7 |
  892474.7 |
  893157.8 |
  893840.9 |
  894524.0 |
  895207.1 |
  895890.2 |########################################
  896573.2 |
  897256.3 |
  897939.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 881366.7-893752.9 ns)
  881366.7 |########################################
  881986.0 |
  882605.3 |
  883224.6 |########################################
  883843.9 |
  884463.2 |
  885082.6 |
  885701.9 |
  886321.2 |
  886940.5 |
  887559.8 |
  888179.1 |
  888798.4 |
  889417.7 |########################################
  890037.0 |
  890656.3 |
  891275.7 |########################################
  891895.0 |
  892514.3 |########################################
  893133.6 |
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2083885.8-2091471.5 ns)
  2083885.8 |####################
  2084265.1 |
  2084644.4 |
  2085023.6 |
  2085402.9 |
  2085782.2 |
  2086161.5 |
  2086540.8 |
  2086920.1 |
  2087299.3 |
  2087678.6 |
  2088057.9 |####################
  2088437.2 |
  2088816.5 |
  2089195.8 |####################
  2089575.0 |
  2089954.3 |
  2090333.6 |########################################
  2090712.9 |
  2091092.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=3874.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=312.4% of algo (FFI overhead may distort results)
