# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 32693% faster than the next best (abi_boundary_w_tight_soa_per_w)

abi_boundary_w_tight_null_entry (2.97 us) leads abi_boundary_w_tight_soa_per_w (974.77 us) by 32693%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.04 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 695.2x slower than the field

abi_boundary_w_tight_scalar_anchor (2.07 ms) is 695.2x the fastest (2.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_soa_dispatch shows alternating (throttle bounce) (autocorr -0.52)

abi_boundary_w_tight_soa_dispatch's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} (32693% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} with a 32693% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 695.2x the fastest

Fastest abi_boundary_w_tight_null_entry (2.97 us) to slowest abi_boundary_w_tight_scalar_anchor (2.07 ms): 695.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 2972.4 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 695.19x (fastest 2972.4 ns, slowest 2066408.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 5279ns | 5250ns | 5095ns | 5208ns | 5478ns | -99.74% |
| abi_boundary_w_tight_scalar_anchor | 2073177ns | 2069928ns | 2050491ns | 2068470ns | 2091581ns | +1.24% |
| abi_boundary_w_tight_scalar_dispatch | 2054211ns | 2052226ns | 2037407ns | 2049537ns | 2069625ns | +0.31% |
| abi_boundary_w_tight_scalar_per_w | 2065603ns | 2048304ns | 2032808ns | 2046624ns | 2110469ns | +0.87% |
| abi_boundary_w_tight_scalar_runtime_w | 2047884ns | 2047490ns | 2044515ns | 2046923ns | 2051010ns | base |
| abi_boundary_w_tight_soa_dispatch | 985122ns | 982491ns | 978216ns | 981886ns | 993429ns | -51.90% |
| abi_boundary_w_tight_soa_per_w | 984350ns | 977436ns | 972997ns | 976146ns | 1002335ns | -51.93% |
| abi_boundary_w_tight_soa_runtime_w | 986702ns | 977879ns | 970513ns | 976387ns | 1010267ns | -51.82% |
| abi_boundary_w_tight_zig_runtime_w | 2025656ns | 2024799ns | 2008523ns | 2024056ns | 2036624ns | -1.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 3018ns | 2924ns | 3144ns | -99.85% | 0.003 |
| abi_boundary_w_tight_scalar_anchor | 2069714ns | 2047553ns | 2087856ns | +1.24% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2050809ns | 2034458ns | 2065938ns | +0.31% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2062330ns | 2029960ns | 2106662ns | +0.88% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2044400ns | 2041423ns | 2047295ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 982211ns | 975395ns | 990494ns | -51.96% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 981219ns | 969910ns | 998448ns | -52.00% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 983654ns | 967709ns | 1006814ns | -51.89% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2022286ns | 2005225ns | 2033095ns | -1.08% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 28195.2 | 3120.3 | 3017.6 | n/a |
| abi_boundary_w_tight_scalar_anchor | 68351.3 | 2072802.4 | 2069713.9 | n/a |
| abi_boundary_w_tight_scalar_dispatch | 66156.1 | 2052586.2 | 2050809.0 | 0 |
| abi_boundary_w_tight_scalar_per_w | 61685.6 | 2059401.6 | 2062329.7 | 0 |
| abi_boundary_w_tight_scalar_runtime_w | 67385.7 | 2044387.7 | 2044400.4 | n/a |
| abi_boundary_w_tight_soa_dispatch | 48810.7 | 983347.7 | 982211.2 | n/a |
| abi_boundary_w_tight_soa_per_w | 53952.6 | 981632.6 | 981219.3 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 51941.7 | 983978.2 | 983654.5 | n/a |
| abi_boundary_w_tight_zig_runtime_w | 228414.2 | 2027341.4 | 2022286.0 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.003 | 98.4% |
| abi_boundary_w_tight_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_tight_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_tight_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_tight_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_tight_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_tight_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_tight_null_entry | 5279ns | 5279ns | -99.74% |
| abi_boundary_w_tight_scalar_anchor | 2073177ns | 2073177ns | +1.24% |
| abi_boundary_w_tight_scalar_dispatch | 2054211ns | 2054211ns | +0.31% |
| abi_boundary_w_tight_scalar_per_w | 2065603ns | 2065603ns | +0.87% |
| abi_boundary_w_tight_scalar_runtime_w | 2047884ns | 2047884ns | base |
| abi_boundary_w_tight_soa_dispatch | 985122ns | 985122ns | -51.90% |
| abi_boundary_w_tight_soa_per_w | 984350ns | 984350ns | -51.93% |
| abi_boundary_w_tight_soa_runtime_w | 986702ns | 986702ns | -51.82% |
| abi_boundary_w_tight_zig_runtime_w | 2025656ns | 2025656ns | -1.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2043963ns | base | --- | [2041944, 2047295] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 2972ns | -2041017.9ns (-99.9%) | [-2044150, -2038980]ns | [2936, 3144] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2066408ns | +22346.2ns (+1.1%) | [+10068, +43526]ns | [2054877, 2087856] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2048784ns | no significant difference | [-4238, +20109]ns | [2037705, 2065938] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2045309ns | no significant difference | [-8446, +60834]ns | [2035018, 2106662] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_dispatch | 979435ns | -1063527.3ns (-52.0%) | [-1067204, -1055836]ns | [976705, 990494] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_per_w | 974766ns | -1067177.3ns (-52.2%) | [-1075887, -1046479]ns | [970444, 998448] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 975003ns | -1069321.5ns (-52.3%) | [-1073903, -1039014]ns | [969146, 1006814] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2021460ns | -21947.7ns (-1.1%) | [-30812, -13583]ns | [2012303, 2033095] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2041423ns | -99.9% | +0.3% | -0.3% | +0.0% | -52.2% | -52.1% | -52.5% | -1.1% |
| 2 | 2048195ns | -99.8% | +0.7% | +1.1% | +0.3% | -51.7% | -52.6% | -51.9% | -1.1% |
| 3 | 2046394ns | -99.8% | +1.2% | +0.4% | +0.1% | -52.2% | -51.8% | -52.3% | -0.3% |
| 4 | 2043461ns | -99.9% | +3.0% | +0.8% | +5.6% | -52.1% | -50.6% | -49.7% | -1.0% |
| 5 | 2044465ns | -99.9% | +1.0% | -0.1% | -0.2% | -51.5% | -52.5% | -52.7% | -1.9% |
| 6 | 2042464ns | -99.9% | +1.3% | -0.1% | -0.6% | -52.1% | -52.5% | -52.3% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.167 | ok |
| abi_boundary_w_tight_scalar_anchor | 0.013 | ok |
| abi_boundary_w_tight_scalar_dispatch | -0.233 | moderate- |
| abi_boundary_w_tight_scalar_per_w | -0.219 | moderate- |
| abi_boundary_w_tight_scalar_runtime_w | -0.182 | ok |
| abi_boundary_w_tight_soa_dispatch | -0.523 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_soa_per_w | -0.055 | ok |
| abi_boundary_w_tight_soa_runtime_w | -0.386 | moderate- |
| abi_boundary_w_tight_zig_runtime_w | 0.128 | ok |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 0/6, lost 6/6
- **abi_boundary_w_tight_scalar_dispatch**: won 1/6, lost 3/6
- **abi_boundary_w_tight_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_tight_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 120087.4ns | 3017.6ns | 3979.6% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6293426.0ns | 2069713.9ns | 304.1% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6229971.4ns | 2050809.0ns | 303.8% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6250417.7ns | 2062329.7ns | 303.1% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6206161.9ns | 2044400.4ns | 303.6% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 2998574.3ns | 982211.2ns | 305.3% | HIGH |
| abi_boundary_w_tight_soa_per_w | 3000735.1ns | 981219.3ns | 305.8% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 3003657.8ns | 983654.5ns | 305.4% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6395559.4ns | 2022286.0ns | 316.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 2923.8-3144.3 ns)
   2923.8 |########################################
   2934.8 |
   2945.9 |########################################
   2956.9 |########################################
   2967.9 |########################################
   2978.9 |
   2990.0 |
   3001.0 |
   3012.0 |
   3023.0 |
   3034.1 |
   3045.1 |
   3056.1 |
   3067.2 |
   3078.2 |
   3089.2 |
   3100.2 |########################################
   3111.3 |
   3122.3 |
   3133.3 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2047552.9-2087856.2 ns)
  2047552.9 |########################################
  2049568.1 |
  2051583.2 |
  2053598.4 |
  2055613.6 |
  2057628.7 |
  2059643.9 |
  2061659.1 |########################################
  2063674.2 |########################################
  2065689.4 |
  2067704.6 |########################################
  2069719.7 |########################################
  2071734.9 |
  2073750.1 |
  2075765.2 |
  2077780.4 |
  2079795.6 |
  2081810.7 |
  2083825.9 |
  2085841.1 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2034457.9-2065937.5 ns)
  2034457.9 |########################################
  2036031.9 |
  2037605.9 |
  2039179.8 |
  2040753.8 |########################################
  2042327.8 |########################################
  2043901.8 |
  2045475.8 |
  2047049.7 |
  2048623.7 |
  2050197.7 |
  2051771.7 |
  2053345.7 |########################################
  2054919.6 |
  2056493.6 |
  2058067.6 |
  2059641.6 |########################################
  2061215.6 |
  2062789.5 |
  2064363.5 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2029960.4-2106661.6 ns)
  2029960.4 |########################################
  2033795.5 |
  2037630.5 |########################################
  2041465.6 |########################################
  2045300.6 |########################################
  2049135.7 |
  2052970.8 |########################################
  2056805.8 |
  2060640.9 |
  2064476.0 |
  2068311.0 |
  2072146.1 |
  2075981.1 |
  2079816.2 |
  2083651.3 |
  2087486.3 |
  2091321.4 |
  2095156.5 |
  2098991.5 |
  2102826.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2041422.9-2047294.8 ns)
  2041422.9 |########################################
  2041716.5 |
  2042010.1 |
  2042303.7 |########################################
  2042597.3 |
  2042890.9 |
  2043184.5 |########################################
  2043478.1 |
  2043771.7 |
  2044065.3 |
  2044358.8 |########################################
  2044652.4 |
  2044946.0 |
  2045239.6 |
  2045533.2 |
  2045826.8 |
  2046120.4 |########################################
  2046414.0 |
  2046707.6 |
  2047001.2 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 975394.6-990493.9 ns)
  975394.6 |####################
  976149.6 |
  976904.5 |
  977659.5 |####################
  978414.5 |
  979169.4 |########################################
  979924.4 |
  980679.4 |
  981434.3 |
  982189.3 |
  982944.3 |
  983699.2 |
  984454.2 |
  985209.2 |
  985964.1 |
  986719.1 |
  987474.1 |
  988229.0 |####################
  988984.0 |
  989739.0 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 969910.0-998448.1 ns)
  969910.0 |########################################
  971336.9 |
  972763.8 |
  974190.7 |
  975617.6 |
  977044.5 |#############
  978471.4 |
  979898.3 |
  981325.2 |
  982752.1 |
  984179.1 |
  985606.0 |#############
  987032.9 |
  988459.8 |
  989886.7 |
  991313.6 |
  992740.5 |
  994167.4 |
  995594.3 |
  997021.2 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 967708.7-1006814.4 ns)
  967708.7 |####################
  969664.0 |####################
  971619.3 |
  973574.6 |########################################
  975529.8 |
  977485.1 |
  979440.4 |
  981395.7 |
  983351.0 |
  985306.3 |####################
  987261.5 |
  989216.8 |
  991172.1 |
  993127.4 |
  995082.7 |
  997038.0 |
  998993.3 |
  1000948.5 |
  1002903.8 |
  1004859.1 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2005225.4-2033095.0 ns)
  2005225.4 |########################################
  2006618.9 |
  2008012.4 |
  2009405.8 |
  2010799.3 |
  2012192.8 |
  2013586.3 |
  2014979.8 |
  2016373.2 |
  2017766.7 |
  2019160.2 |########################################
  2020553.7 |########################################
  2021947.2 |########################################
  2023340.6 |
  2024734.1 |########################################
  2026127.6 |
  2027521.1 |
  2028914.6 |
  2030308.0 |
  2031701.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=4061.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=306.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=305.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=315.7% of algo (FFI overhead may distort results)
