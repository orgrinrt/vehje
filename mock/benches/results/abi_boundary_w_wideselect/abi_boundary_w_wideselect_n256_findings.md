# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 29280% faster than the next best (abi_boundary_w_wideselect_soa_per_w)

abi_boundary_w_wideselect_null_entry (3.20 us) leads abi_boundary_w_wideselect_soa_per_w (939.66 us) by 29280%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.10 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_scalar_dispatch is an outlier: 667.8x slower than the field

abi_boundary_w_wideselect_scalar_dispatch (2.14 ms) is 667.8x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_dispatch} (29280% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_dispatch} with a 29280% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 667.8x the fastest

Fastest abi_boundary_w_wideselect_null_entry (3.20 us) to slowest abi_boundary_w_wideselect_scalar_dispatch (2.14 ms): 667.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_boundary_w_wideselect_soa_per_w is inconsistent: worst-20% is 1.5x its best-20%

abi_boundary_w_wideselect_soa_per_w's best 20% of batches run at 930.15 us but its worst 20% at 1.41 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 3198.4 ns median (-99.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 667.80x (fastest 3198.4 ns, slowest 2135869.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5835ns | 5559ns | 5360ns | 5519ns | 6546ns | -99.74% |
| abi_boundary_w_wideselect_scalar_anchor | 2285928ns | 2115604ns | 2087143ns | 2110228ns | 2648871ns | +2.48% |
| abi_boundary_w_wideselect_scalar_dispatch | 2261876ns | 2139720ns | 2099062ns | 2126890ns | 2545761ns | +1.40% |
| abi_boundary_w_wideselect_scalar_per_w | 2270689ns | 2095270ns | 2074891ns | 2092929ns | 2635227ns | +1.79% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2230660ns | 2106610ns | 2083473ns | 2101595ns | 2497851ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 984039ns | 945378ns | 933758ns | 942312ns | 1071772ns | -55.89% |
| abi_boundary_w_wideselect_soa_per_w | 1098575ns | 942653ns | 932859ns | 940982ns | 1417823ns | -50.75% |
| abi_boundary_w_wideselect_soa_runtime_w | 988731ns | 954826ns | 929991ns | 952466ns | 1072497ns | -55.68% |
| abi_boundary_w_wideselect_zig_runtime_w | 2220193ns | 2035369ns | 2020858ns | 2033238ns | 2600291ns | -0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 3387ns | 3120ns | 3814ns | -99.85% | 0.076 |
| abi_boundary_w_wideselect_scalar_anchor | 2281650ns | 2083461ns | 2642918ns | +2.47% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2257628ns | 2095588ns | 2540365ns | +1.39% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2266333ns | 2071794ns | 2628698ns | +1.78% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2226683ns | 2080221ns | 2492776ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 980609ns | 930852ns | 1067394ns | -55.96% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 1094838ns | 930147ns | 1412363ns | -50.83% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 985073ns | 927204ns | 1067416ns | -55.76% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2215718ns | 2017317ns | 2594071ns | -0.49% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 33320.4 | 3379.9 | 3386.7 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 86503.2 | 2286049.3 | 2281650.3 | n/a |
| abi_boundary_w_wideselect_scalar_dispatch | 88261.3 | 2246725.9 | 2257628.4 | n/a |
| abi_boundary_w_wideselect_scalar_per_w | 86912.5 | 2270765.4 | 2266332.8 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 82073.0 | 2235383.4 | 2226683.5 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 64046.6 | 982449.9 | 980608.7 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 72182.6 | 1056311.1 | 1094837.7 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 68853.3 | 986008.5 | 985072.8 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 319813.9 | 2213625.3 | 2215718.5 | 22 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.080 | 97.6% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5835ns | 5835ns | -99.74% |
| abi_boundary_w_wideselect_scalar_anchor | 2285928ns | 2285928ns | +2.48% |
| abi_boundary_w_wideselect_scalar_dispatch | 2261876ns | 2261876ns | +1.40% |
| abi_boundary_w_wideselect_scalar_per_w | 2270689ns | 2270689ns | +1.79% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2230660ns | 2230660ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 984039ns | 984039ns | -55.89% |
| abi_boundary_w_wideselect_soa_per_w | 1098575ns | 1098575ns | -50.75% |
| abi_boundary_w_wideselect_soa_runtime_w | 988731ns | 988731ns | -55.68% |
| abi_boundary_w_wideselect_zig_runtime_w | 2220193ns | 2220193ns | -0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2103069ns | base | --- | [2084206, 2492776] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 3198ns | -2099906.7ns (-99.8%) | [-2488962, -2081021]ns | [3148, 3814] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2112140ns | no significant difference | [-4241, +154714]ns | [2089893, 2642918] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2135869ns | no significant difference | [-41759, +117139]ns | [2096651, 2540365] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2091838ns | no significant difference | [-94420, +222509]ns | [2078463, 2628698] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 942342ns | -1170861.9ns (-55.7%) | [-1425382, -1141980]ns | [932090, 1067394] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 939664ns | -1157460.2ns (-55.0%) | [-1193530, -1044548]ns | [932486, 1412363] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 951829ns | -1157957.1ns (-55.1%) | [-1425360, -1141515]ns | [935973, 1067416] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2031740ns | no significant difference | [-90284, +126194]ns | [2021345, 2594071] | no | 0.2500 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2106059ns | -99.8% | +0.8% | +2.9% | -0.5% | -55.7% | -55.3% | -55.1% | -3.8% |
| 2 | 2088190ns | -99.8% | +0.4% | +0.8% | -0.1% | -55.3% | -55.0% | -54.1% | -2.4% |
| 3 | 2080221ns | -99.8% | +1.0% | +0.8% | -0.4% | -54.3% | -55.1% | -55.4% | -3.0% |
| 4 | 2100079ns | -99.9% | -0.8% | -0.2% | -0.6% | -55.7% | -55.7% | -55.0% | -3.6% |
| 5 | 2540108ns | -99.9% | +0.5% | -3.1% | -6.9% | -60.1% | -37.2% | -59.2% | -4.0% |
| 6 | 2445444ns | -99.8% | +11.8% | +7.1% | +18.3% | -54.2% | -49.8% | -55.0% | +12.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.358 | moderate+ |
| abi_boundary_w_wideselect_scalar_anchor | 0.409 | moderate+ |
| abi_boundary_w_wideselect_scalar_dispatch | 0.410 | moderate+ |
| abi_boundary_w_wideselect_scalar_per_w | 0.271 | moderate+ |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.388 | moderate+ |
| abi_boundary_w_wideselect_soa_dispatch | 0.285 | moderate+ |
| abi_boundary_w_wideselect_soa_per_w | 0.161 | ok |
| abi_boundary_w_wideselect_soa_runtime_w | 0.380 | moderate+ |
| abi_boundary_w_wideselect_zig_runtime_w | 0.385 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 1/6, lost 5/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 2/6, lost 4/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 127757.6ns | 3386.7ns | 3772.3% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6936300.1ns | 2281650.3ns | 304.0% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6850517.4ns | 2257628.4ns | 303.4% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6902944.7ns | 2266332.8ns | 304.6% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6780947.6ns | 2226683.5ns | 304.5% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 3013974.2ns | 980608.7ns | 307.4% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 3263677.0ns | 1094837.7ns | 298.1% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 3031208.6ns | 985072.8ns | 307.7% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 7069245.5ns | 2215718.5ns | 319.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 3120.4-3813.6 ns)
   3120.4 |####################
   3155.1 |####################
   3189.7 |########################################
   3224.4 |
   3259.0 |
   3293.7 |
   3328.3 |
   3363.0 |
   3397.7 |
   3432.3 |
   3467.0 |
   3501.6 |
   3536.3 |
   3570.9 |
   3605.6 |
   3640.3 |####################
   3674.9 |
   3709.6 |
   3744.2 |
   3778.9 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2083461.2-2642917.9 ns)
  2083461.2 |########################################
  2111434.0 |#############
  2139406.9 |
  2167379.7 |
  2195352.5 |
  2223325.4 |
  2251298.2 |
  2279271.0 |
  2307243.9 |
  2335216.7 |
  2363189.5 |
  2391162.4 |
  2419135.2 |
  2447108.1 |
  2475080.9 |
  2503053.7 |
  2531026.6 |#############
  2558999.4 |
  2586972.2 |
  2614945.1 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2095587.9-2540365.0 ns)
  2095587.9 |########################################
  2117826.8 |
  2140065.6 |
  2162304.5 |#############
  2184543.3 |
  2206782.2 |
  2229021.0 |
  2251259.9 |
  2273498.7 |
  2295737.6 |
  2317976.5 |
  2340215.3 |
  2362454.2 |
  2384693.0 |
  2406931.9 |
  2429170.7 |
  2451409.6 |#############
  2473648.4 |
  2495887.3 |
  2518126.1 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2071793.8-2628697.9 ns)
  2071793.8 |########################################
  2099639.0 |
  2127484.2 |
  2155329.4 |
  2183174.6 |
  2211019.8 |
  2238865.0 |
  2266710.2 |
  2294555.4 |
  2322400.6 |
  2350245.9 |##########
  2378091.1 |
  2405936.3 |
  2433781.5 |
  2461626.7 |
  2489471.9 |
  2517317.1 |
  2545162.3 |
  2573007.5 |
  2600852.7 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2080221.2-2492776.0 ns)
  2080221.2 |########################################
  2100848.9 |#############
  2121476.7 |
  2142104.4 |
  2162732.2 |
  2183359.9 |
  2203987.7 |
  2224615.4 |
  2245243.1 |
  2265870.9 |
  2286498.6 |
  2307126.4 |
  2327754.1 |
  2348381.9 |
  2369009.6 |
  2389637.3 |
  2410265.1 |
  2430892.8 |#############
  2451520.6 |
  2472148.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 930851.7-1067393.9 ns)
  930851.7 |########################################
  937678.8 |
  944505.9 |#############
  951333.0 |
  958160.1 |
  964987.3 |
  971814.4 |
  978641.5 |
  985468.6 |
  992295.7 |
  999122.8 |
  1005949.9 |
  1012777.0 |#############
  1019604.2 |
  1026431.3 |
  1033258.4 |
  1040085.5 |
  1046912.6 |
  1053739.7 |
  1060566.8 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 930147.1-1412363.1 ns)
  930147.1 |########################################
  954257.9 |
  978368.7 |
  1002479.5 |
  1026590.3 |
  1050701.1 |
  1074811.9 |
  1098922.7 |
  1123033.5 |
  1147144.3 |
  1171255.1 |
  1195365.9 |
  1219476.7 |##########
  1243587.5 |
  1267698.3 |
  1291809.1 |
  1315919.9 |
  1340030.7 |
  1364141.5 |
  1388252.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 927204.2-1067415.9 ns)
  927204.2 |####################
  934214.8 |
  941225.4 |########################################
  948235.9 |
  955246.5 |####################
  962257.1 |
  969267.7 |
  976278.3 |
  983288.9 |
  990299.4 |
  997310.0 |
  1004320.6 |
  1011331.2 |
  1018341.8 |
  1025352.4 |
  1032362.9 |####################
  1039373.5 |
  1046384.1 |
  1053394.7 |
  1060405.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 2017316.7-2594071.0 ns)
  2017316.7 |########################################
  2046154.4 |
  2074992.1 |
  2103829.9 |
  2132667.6 |
  2161505.3 |
  2190343.0 |
  2219180.7 |
  2248018.4 |
  2276856.2 |
  2305693.9 |
  2334531.6 |
  2363369.3 |
  2392207.0 |
  2421044.7 |##########
  2449882.5 |
  2478720.2 |
  2507557.9 |
  2536395.6 |
  2565233.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=3922.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=305.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: CV=22.7% (high variance, measurements may be unstable)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=305.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=316.8% of algo (FFI overhead may distort results)
