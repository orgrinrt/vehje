# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 50425% faster than the next best (abi_boundary_w_wideselect_zig_runtime_w)

abi_boundary_w_wideselect_null_entry (3.95 us) leads abi_boundary_w_wideselect_zig_runtime_w (2.00 ms) by 50425%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.08 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_soa_runtime_w is an outlier: 527.3x slower than the field

abi_boundary_w_wideselect_soa_runtime_w (2.09 ms) is 527.3x the fastest (3.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_scalar_per_w shows alternating (throttle bounce) (autocorr -0.52)

abi_boundary_w_wideselect_scalar_per_w's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_soa_runtime_w} (50425% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_soa_runtime_w} with a 50425% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 527.3x the fastest

Fastest abi_boundary_w_wideselect_null_entry (3.95 us) to slowest abi_boundary_w_wideselect_soa_runtime_w (2.09 ms): 527.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 3954.9 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 527.25x (fastest 3954.9 ns, slowest 2085266.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 6283ns | 6206ns | 6094ns | 6177ns | 6536ns | -99.70% |
| abi_boundary_w_wideselect_scalar_anchor | 2079885ns | 2081414ns | 2071631ns | 2079179ns | 2085071ns | -0.25% |
| abi_boundary_w_wideselect_scalar_dispatch | 2086069ns | 2086240ns | 2064149ns | 2082423ns | 2102498ns | +0.04% |
| abi_boundary_w_wideselect_scalar_per_w | 2085766ns | 2082926ns | 2079387ns | 2082067ns | 2094503ns | +0.03% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2085180ns | 2084754ns | 2080130ns | 2084350ns | 2088952ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 2080156ns | 2081755ns | 2063800ns | 2079647ns | 2089098ns | -0.24% |
| abi_boundary_w_wideselect_soa_per_w | 2080200ns | 2083612ns | 2059330ns | 2082582ns | 2087062ns | -0.24% |
| abi_boundary_w_wideselect_soa_runtime_w | 2086899ns | 2088768ns | 2069732ns | 2084802ns | 2098627ns | +0.08% |
| abi_boundary_w_wideselect_zig_runtime_w | 2002920ns | 2001884ns | 1997078ns | 2000458ns | 2009535ns | -3.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4012ns | 3902ns | 4175ns | -99.81% | 0.001 |
| abi_boundary_w_wideselect_scalar_anchor | 2076555ns | 2068564ns | 2081772ns | -0.26% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2082588ns | 2061418ns | 2098753ns | +0.03% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2082272ns | 2075872ns | 2090922ns | +0.02% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2081902ns | 2077134ns | 2085471ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 2076687ns | 2060817ns | 2085279ns | -0.25% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 2076954ns | 2056870ns | 2083662ns | -0.24% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 2083577ns | 2066996ns | 2095058ns | +0.08% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 1999289ns | 1993605ns | 2005808ns | -3.97% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 28449.0 | 4141.8 | 4011.6 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 69400.4 | 2076444.7 | 2076555.2 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 72682.8 | 2080423.6 | 2082587.8 | n/a |
| abi_boundary_w_wideselect_scalar_per_w | 80234.8 | 2081473.9 | 2082271.7 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 68081.0 | 2082705.2 | 2081901.6 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 73239.9 | 2078191.9 | 2076686.5 | 1 |
| abi_boundary_w_wideselect_soa_per_w | 74339.0 | 2080710.7 | 2076953.8 | 1 |
| abi_boundary_w_wideselect_soa_runtime_w | 68674.7 | 2083044.9 | 2083576.7 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 243957.8 | 2000016.3 | 1999288.9 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.001 | 98.7% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 6283ns | 6283ns | -99.70% |
| abi_boundary_w_wideselect_scalar_anchor | 2079885ns | 2079885ns | -0.25% |
| abi_boundary_w_wideselect_scalar_dispatch | 2086069ns | 2086069ns | +0.04% |
| abi_boundary_w_wideselect_scalar_per_w | 2085766ns | 2085766ns | +0.03% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2085180ns | 2085180ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 2080156ns | 2080156ns | -0.24% |
| abi_boundary_w_wideselect_soa_per_w | 2080200ns | 2080200ns | -0.24% |
| abi_boundary_w_wideselect_soa_runtime_w | 2086899ns | 2086899ns | +0.08% |
| abi_boundary_w_wideselect_zig_runtime_w | 2002920ns | 2002920ns | -3.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2081470ns | base | --- | [2078763, 2085471] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 3955ns | -2077565.2ns (-99.8%) | [-2081376, -2074728]ns | [3905, 4175] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2077904ns | no significant difference | [-14590, +2639]ns | [2069990, 2081772] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2082737ns | no significant difference | [-17968, +19281]ns | [2066273, 2098753] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2079463ns | no significant difference | [-6513, +8930]ns | [2076429, 2090922] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 2078301ns | no significant difference | [-14621, +3036]ns | [2066480, 2085279] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 2080267ns | no significant difference | [-16938, +4233]ns | [2066933, 2083662] | no | 0.5833 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 2085267ns | no significant difference | [-11956, +15364]ns | [2070406, 2095058] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 1998256ns | -81517.5ns (-3.9%) | [-86958, -79363]ns | [1993803, 2005808] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2081810ns | -99.8% | -0.6% | +1.3% | -0.1% | -1.0% | -0.1% | +0.3% | -4.0% |
| 2 | 2077134ns | -99.8% | +0.1% | +0.6% | -0.1% | +0.3% | +0.5% | +0.4% | -3.8% |
| 3 | 2083592ns | -99.8% | -0.3% | -0.2% | +0.6% | -0.4% | -0.2% | -0.5% | -3.8% |
| 4 | 2081131ns | -99.8% | +0.1% | -0.9% | -0.1% | -0.0% | -0.1% | -0.7% | -4.2% |
| 5 | 2080392ns | -99.8% | -0.1% | +0.3% | +0.2% | -0.4% | -0.2% | +1.1% | -4.2% |
| 6 | 2087350ns | -99.8% | -0.8% | -0.8% | -0.5% | +0.0% | -1.5% | -0.1% | -3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.296 | moderate+ |
| abi_boundary_w_wideselect_scalar_anchor | -0.077 | ok |
| abi_boundary_w_wideselect_scalar_dispatch | 0.069 | ok |
| abi_boundary_w_wideselect_scalar_per_w | -0.523 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_scalar_runtime_w | -0.275 | moderate- |
| abi_boundary_w_wideselect_soa_dispatch | -0.399 | moderate- |
| abi_boundary_w_wideselect_soa_per_w | 0.150 | ok |
| abi_boundary_w_wideselect_soa_runtime_w | -0.193 | ok |
| abi_boundary_w_wideselect_zig_runtime_w | -0.309 | moderate- |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 3/6, lost 2/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 3/6, lost 1/6
- **abi_boundary_w_wideselect_soa_per_w**: won 3/6, lost 1/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 3/6, lost 3/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 123344.8ns | 4011.6ns | 3074.7% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6308994.5ns | 2076555.2ns | 303.8% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6322368.1ns | 2082587.8ns | 303.6% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6326385.3ns | 2082271.7ns | 303.8% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6320216.3ns | 2081901.6ns | 303.6% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 6311236.5ns | 2076686.5ns | 303.9% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 6310431.0ns | 2076953.8ns | 303.8% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 6318779.2ns | 2083576.7ns | 303.3% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6336192.6ns | 1999288.9ns | 316.9% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 3902.1-4174.8 ns)
   3902.1 |########################################
   3915.7 |####################
   3929.4 |
   3943.0 |
   3956.6 |
   3970.3 |####################
   3983.9 |
   3997.5 |
   4011.2 |
   4024.8 |
   4038.4 |
   4052.1 |
   4065.7 |
   4079.4 |
   4093.0 |
   4106.6 |
   4120.3 |
   4133.9 |####################
   4147.5 |
   4161.2 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2068564.2-2081771.9 ns)
  2068564.2 |########################################
  2069224.6 |
  2069885.0 |
  2070545.3 |
  2071205.7 |########################################
  2071866.1 |
  2072526.5 |
  2073186.9 |
  2073847.3 |
  2074507.6 |
  2075168.0 |
  2075828.4 |
  2076488.8 |########################################
  2077149.2 |
  2077809.6 |
  2078469.9 |########################################
  2079130.3 |########################################
  2079790.7 |
  2080451.1 |
  2081111.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2061417.9-2098753.1 ns)
  2061417.9 |########################################
  2063284.7 |
  2065151.4 |
  2067018.2 |
  2068884.9 |
  2070751.7 |########################################
  2072618.5 |
  2074485.2 |
  2076352.0 |
  2078218.7 |########################################
  2080085.5 |
  2081952.3 |
  2083819.0 |
  2085685.8 |########################################
  2087552.5 |########################################
  2089419.3 |
  2091286.1 |
  2093152.8 |
  2095019.6 |
  2096886.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2075872.1-2090922.5 ns)
  2075872.1 |########################################
  2076624.6 |########################################
  2077377.1 |
  2078129.7 |########################################
  2078882.2 |
  2079634.7 |
  2080387.2 |########################################
  2081139.7 |
  2081892.3 |
  2082644.8 |
  2083397.3 |
  2084149.8 |########################################
  2084902.3 |
  2085654.9 |
  2086407.4 |
  2087159.9 |
  2087912.4 |
  2088664.9 |
  2089417.5 |
  2090170.0 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2077134.2-2085471.4 ns)
  2077134.2 |########################################
  2077551.1 |
  2077967.9 |
  2078384.8 |
  2078801.6 |
  2079218.5 |
  2079635.4 |
  2080052.2 |########################################
  2080469.1 |
  2080886.0 |########################################
  2081302.8 |
  2081719.7 |########################################
  2082136.6 |
  2082553.4 |
  2082970.3 |
  2083387.1 |########################################
  2083804.0 |
  2084220.9 |
  2084637.7 |
  2085054.6 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 2060816.7-2085278.6 ns)
  2060816.7 |########################################
  2062039.8 |
  2063262.9 |
  2064486.0 |
  2065709.1 |
  2066932.2 |
  2068155.3 |
  2069378.3 |
  2070601.4 |
  2071824.5 |########################################
  2073047.6 |
  2074270.7 |
  2075493.8 |########################################
  2076716.9 |
  2077940.0 |
  2079163.1 |
  2080386.2 |########################################
  2081609.3 |
  2082832.4 |########################################
  2084055.5 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 2056869.6-2083661.9 ns)
  2056869.6 |#############
  2058209.2 |
  2059548.8 |
  2060888.4 |
  2062228.1 |
  2063567.7 |
  2064907.3 |
  2066246.9 |
  2067586.5 |
  2068926.1 |
  2070265.8 |
  2071605.4 |
  2072945.0 |
  2074284.6 |
  2075624.2 |
  2076963.8 |#############
  2078303.4 |
  2079643.1 |########################################
  2080982.7 |
  2082322.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 2066996.2-2095057.5 ns)
  2066996.2 |########################################
  2068399.3 |
  2069802.3 |
  2071205.4 |
  2072608.5 |########################################
  2074011.5 |
  2075414.6 |
  2076817.7 |
  2078220.7 |
  2079623.8 |
  2081026.9 |
  2082429.9 |
  2083833.0 |########################################
  2085236.0 |########################################
  2086639.1 |########################################
  2088042.2 |
  2089445.2 |
  2090848.3 |
  2092251.4 |
  2093654.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 1993605.0-2005807.9 ns)
  1993605.0 |########################################
  1994215.1 |
  1994825.3 |
  1995435.4 |
  1996045.6 |
  1996655.7 |
  1997265.9 |
  1997876.0 |####################
  1998486.2 |####################
  1999096.3 |
  1999706.4 |
  2000316.6 |
  2000926.7 |
  2001536.9 |
  2002147.0 |
  2002757.2 |
  2003367.3 |
  2003977.5 |####################
  2004587.6 |
  2005197.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=3111.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=317.2% of algo (FFI overhead may distort results)
