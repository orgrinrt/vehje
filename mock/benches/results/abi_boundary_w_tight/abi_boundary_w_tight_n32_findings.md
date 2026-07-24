# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 42600% faster than the next best (abi_boundary_w_tight_soa_per_w)

abi_boundary_w_tight_null_entry (2.26 us) leads abi_boundary_w_tight_soa_per_w (966.44 us) by 42600%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.03 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 908.5x slower than the field

abi_boundary_w_tight_scalar_anchor (2.06 ms) is 908.5x the fastest (2.26 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_scalar_per_w shows alternating (throttle bounce) (autocorr -0.55)

abi_boundary_w_tight_scalar_per_w's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_anchor} (42600% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_anchor} with a 42600% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 908.5x the fastest

Fastest abi_boundary_w_tight_null_entry (2.26 us) to slowest abi_boundary_w_tight_scalar_anchor (2.06 ms): 908.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 2263.3 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 908.50x (fastest 2263.3 ns, slowest 2056253.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4567ns | 4495ns | 4384ns | 4476ns | 4796ns | -99.78% |
| abi_boundary_w_tight_scalar_anchor | 2062061ns | 2059341ns | 2046846ns | 2055290ns | 2079825ns | +1.18% |
| abi_boundary_w_tight_scalar_dispatch | 2041002ns | 2039197ns | 2021788ns | 2038545ns | 2054294ns | +0.15% |
| abi_boundary_w_tight_scalar_per_w | 2047010ns | 2047556ns | 2036858ns | 2045010ns | 2055086ns | +0.44% |
| abi_boundary_w_tight_scalar_runtime_w | 2038007ns | 2037040ns | 2028688ns | 2034986ns | 2047197ns | base |
| abi_boundary_w_tight_soa_dispatch | 976817ns | 976630ns | 966430ns | 975492ns | 983998ns | -52.07% |
| abi_boundary_w_tight_soa_per_w | 974206ns | 969338ns | 959383ns | 966337ns | 993421ns | -52.20% |
| abi_boundary_w_tight_soa_runtime_w | 1031048ns | 981265ns | 976101ns | 980480ns | 1134373ns | -49.41% |
| abi_boundary_w_tight_zig_runtime_w | 2024393ns | 2026146ns | 2008250ns | 2021643ns | 2036589ns | -0.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 2276ns | 2211ns | 2334ns | -99.89% | 0.014 |
| abi_boundary_w_tight_scalar_anchor | 2058798ns | 2043822ns | 2076123ns | +1.18% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2037838ns | 2019058ns | 2050881ns | +0.15% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2043654ns | 2034078ns | 2051500ns | +0.43% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2034821ns | 2025684ns | 2043594ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 973919ns | 963632ns | 980921ns | -52.14% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 971322ns | 956853ns | 990246ns | -52.27% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 1027584ns | 972994ns | 1129947ns | -49.50% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2021020ns | 2005363ns | 2033044ns | -0.68% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 28160.8 | 2394.1 | 2275.6 | n/a |
| abi_boundary_w_tight_scalar_anchor | 62023.3 | 2059762.4 | 2058798.0 | n/a |
| abi_boundary_w_tight_scalar_dispatch | 57166.0 | 2039562.1 | 2037838.1 | 1 |
| abi_boundary_w_tight_scalar_per_w | 67905.9 | 2043497.7 | 2043653.9 | n/a |
| abi_boundary_w_tight_scalar_runtime_w | 58865.4 | 2034165.8 | 2034820.7 | n/a |
| abi_boundary_w_tight_soa_dispatch | 47410.8 | 974068.7 | 973919.3 | n/a |
| abi_boundary_w_tight_soa_per_w | 47953.0 | 971704.9 | 971321.5 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 60201.4 | 1031431.4 | 1027584.3 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 227545.6 | 2017415.0 | 2021019.9 | 12 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.014 | 97.7% |
| abi_boundary_w_tight_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_tight_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_tight_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4567ns | 4567ns | -99.78% |
| abi_boundary_w_tight_scalar_anchor | 2062061ns | 2062061ns | +1.18% |
| abi_boundary_w_tight_scalar_dispatch | 2041002ns | 2041002ns | +0.15% |
| abi_boundary_w_tight_scalar_per_w | 2047010ns | 2047010ns | +0.44% |
| abi_boundary_w_tight_scalar_runtime_w | 2038007ns | 2038007ns | base |
| abi_boundary_w_tight_soa_dispatch | 976817ns | 976817ns | -52.07% |
| abi_boundary_w_tight_soa_per_w | 974206ns | 974206ns | -52.20% |
| abi_boundary_w_tight_soa_runtime_w | 1031048ns | 1031048ns | -49.41% |
| abi_boundary_w_tight_zig_runtime_w | 2024393ns | 2024393ns | -0.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2033914ns | base | --- | [2026954, 2043594] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 2263ns | -2031651.1ns (-99.9%) | [-2041282, -2024702]ns | [2229, 2334] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2056253ns | +21399.6ns (+1.1%) | [+6543, +43989]ns | [2044018, 2076123] | YES (adj: no) | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2036206ns | no significant difference | [-8426, +14758]ns | [2026428, 2050881] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2044087ns | +6727.9ns (+0.3%) | [+1855, +17917]ns | [2035374, 2051500] | YES (adj: no) | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_tight_soa_dispatch | 973773ns | -1060216.0ns (-52.1%) | [-1073695, -1048793]ns | [967064, 980921] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_per_w | 966439ns | -1063651.7ns (-52.3%) | [-1074948, -1051898]ns | [957279, 990246] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 978209ns | -1052041.1ns (-51.7%) | [-1063921, -905747]ns | [974597, 1129947] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2022644ns | -19286.1ns (-0.9%) | [-21577, -540]ns | [2007372, 2033044] | YES (adj: no) | 0.2500 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2038584ns | -99.9% | +1.7% | -0.0% | +0.3% | -52.3% | -51.8% | -52.3% | -0.9% |
| 2 | 2028225ns | -99.9% | +0.9% | -0.5% | +0.3% | -52.0% | -52.8% | -51.9% | -1.1% |
| 3 | 2025684ns | -99.9% | +2.6% | +0.4% | +1.3% | -51.5% | -52.2% | -51.0% | +0.4% |
| 4 | 2029245ns | -99.9% | +0.7% | +1.0% | +0.4% | -52.2% | -52.8% | -51.8% | -1.0% |
| 5 | 2045704ns | -99.9% | -0.1% | +0.3% | -0.1% | -52.9% | -51.2% | -38.0% | -1.0% |
| 6 | 2041483ns | -99.9% | +1.2% | -0.4% | +0.5% | -52.0% | -52.8% | -52.0% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.398 | moderate- |
| abi_boundary_w_tight_scalar_anchor | -0.459 | moderate- |
| abi_boundary_w_tight_scalar_dispatch | 0.185 | ok |
| abi_boundary_w_tight_scalar_per_w | -0.552 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_scalar_runtime_w | 0.293 | moderate+ |
| abi_boundary_w_tight_soa_dispatch | -0.230 | moderate- |
| abi_boundary_w_tight_soa_per_w | -0.512 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_soa_runtime_w | -0.250 | moderate- |
| abi_boundary_w_tight_zig_runtime_w | -0.470 | moderate- |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 0/6, lost 5/6
- **abi_boundary_w_tight_scalar_dispatch**: won 2/6, lost 3/6
- **abi_boundary_w_tight_scalar_per_w**: won 1/6, lost 5/6
- **abi_boundary_w_tight_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_zig_runtime_w**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 117691.4ns | 2275.6ns | 5172.0% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6241982.4ns | 2058798.0ns | 303.2% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6175405.5ns | 2037838.1ns | 303.0% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6202367.9ns | 2043653.9ns | 303.5% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6163997.3ns | 2034820.7ns | 302.9% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 2970786.7ns | 973919.3ns | 305.0% | HIGH |
| abi_boundary_w_tight_soa_per_w | 2963370.5ns | 971321.5ns | 305.1% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 3156761.2ns | 1027584.3ns | 307.2% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6368664.9ns | 2021019.9ns | 315.1% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 2210.8-2333.9 ns)
   2210.8 |########################################
   2217.0 |
   2223.1 |
   2229.3 |
   2235.4 |
   2241.6 |
   2247.7 |########################################
   2253.9 |########################################
   2260.1 |
   2266.2 |
   2272.4 |########################################
   2278.5 |
   2284.7 |
   2290.8 |########################################
   2297.0 |
   2303.2 |
   2309.3 |
   2315.5 |
   2321.6 |
   2327.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2043822.5-2076123.1 ns)
  2043822.5 |########################################
  2045437.5 |
  2047052.6 |####################
  2048667.6 |
  2050282.6 |
  2051897.7 |
  2053512.7 |
  2055127.7 |
  2056742.8 |
  2058357.8 |
  2059972.8 |
  2061587.9 |
  2063202.9 |
  2064817.9 |####################
  2066433.0 |
  2068048.0 |
  2069663.0 |
  2071278.1 |
  2072893.1 |####################
  2074508.1 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2019057.5-2050881.1 ns)
  2019057.5 |####################
  2020648.7 |
  2022239.9 |
  2023831.0 |
  2025422.2 |
  2027013.4 |
  2028604.6 |
  2030195.7 |
  2031786.9 |
  2033378.1 |########################################
  2034969.3 |
  2036560.5 |####################
  2038151.6 |
  2039742.8 |
  2041334.0 |
  2042925.2 |
  2044516.3 |
  2046107.5 |
  2047698.7 |
  2049289.9 |####################
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2034077.9-2051500.4 ns)
  2034077.9 |########################################
  2034949.0 |
  2035820.1 |########################################
  2036691.3 |
  2037562.4 |
  2038433.5 |
  2039304.6 |
  2040175.8 |
  2041046.9 |
  2041918.0 |
  2042789.1 |########################################
  2043660.3 |
  2044531.4 |########################################
  2045402.5 |
  2046273.6 |
  2047144.8 |
  2048015.9 |
  2048887.0 |
  2049758.1 |
  2050629.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2025683.8-2043593.5 ns)
  2025683.8 |########################################
  2026579.3 |
  2027474.8 |########################################
  2028370.3 |########################################
  2029265.8 |
  2030161.2 |
  2031056.7 |
  2031952.2 |
  2032847.7 |
  2033743.2 |
  2034638.7 |
  2035534.2 |
  2036429.6 |
  2037325.1 |
  2038220.6 |########################################
  2039116.1 |
  2040011.6 |
  2040907.1 |########################################
  2041802.6 |
  2042698.1 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 963632.5-980920.8 ns)
  963632.5 |########################################
  964496.9 |
  965361.3 |
  966225.8 |
  967090.2 |
  967954.6 |
  968819.0 |
  969683.4 |########################################
  970547.8 |
  971412.3 |
  972276.7 |
  973141.1 |########################################
  974005.5 |########################################
  974869.9 |
  975734.3 |
  976598.8 |
  977463.2 |
  978327.6 |
  979192.0 |########################################
  980056.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 956853.3-990246.1 ns)
  956853.3 |########################################
  958522.9 |
  960192.6 |
  961862.2 |
  963531.9 |####################
  965201.5 |
  966871.1 |
  968540.8 |####################
  970210.4 |
  971880.0 |
  973549.7 |
  975219.3 |
  976889.0 |
  978558.6 |
  980228.2 |
  981897.9 |####################
  983567.5 |
  985237.1 |
  986906.8 |
  988576.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 972993.7-1129947.2 ns)
  972993.7 |########################################
  980841.4 |
  988689.1 |##########
  996536.7 |
  1004384.4 |
  1012232.1 |
  1020079.8 |
  1027927.4 |
  1035775.1 |
  1043622.8 |
  1051470.5 |
  1059318.2 |
  1067165.8 |
  1075013.5 |
  1082861.2 |
  1090708.9 |
  1098556.5 |
  1106404.2 |
  1114251.9 |
  1122099.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2005363.3-2033043.5 ns)
  2005363.3 |########################################
  2006747.3 |
  2008131.3 |########################################
  2009515.3 |
  2010899.4 |
  2012283.4 |
  2013667.4 |
  2015051.4 |
  2016435.4 |
  2017819.4 |
  2019203.4 |########################################
  2020587.4 |
  2021971.4 |
  2023355.5 |
  2024739.5 |########################################
  2026123.5 |
  2027507.5 |
  2028891.5 |
  2030275.5 |
  2031659.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=5198.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=305.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=306.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=316.1% of algo (FFI overhead may distort results)
