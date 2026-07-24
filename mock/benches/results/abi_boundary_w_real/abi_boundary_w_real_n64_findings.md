# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_real_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_real_scalar_runtime_w has the worst median (2.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_real_null_entry at 2.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_real_null_entry dominates: 35486% faster than the next best (abi_boundary_w_real_soa_per_w)

abi_boundary_w_real_null_entry (2.49 us) leads abi_boundary_w_real_soa_per_w (887.58 us) by 35486%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.18 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_runtime_w is an outlier: 873.4x slower than the field

abi_boundary_w_real_scalar_runtime_w (2.18 ms) is 873.4x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_real_null_entry shows alternating (throttle bounce) (autocorr -0.67)

abi_boundary_w_real_null_entry's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_runtime_w} (35486% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_runtime_w} with a 35486% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 873.4x the fastest

Fastest abi_boundary_w_real_null_entry (2.49 us) to slowest abi_boundary_w_real_scalar_runtime_w (2.18 ms): 873.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 2494.2 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 873.36x (fastest 2494.2 ns, slowest 2178284.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 4764ns | 4737ns | 4642ns | 4716ns | 4897ns | -99.78% |
| abi_boundary_w_real_scalar_anchor | 2225272ns | 2178404ns | 2161679ns | 2176518ns | 2330200ns | +2.16% |
| abi_boundary_w_real_scalar_dispatch | 2172222ns | 2173031ns | 2165378ns | 2171402ns | 2176873ns | -0.28% |
| abi_boundary_w_real_scalar_per_w | 2172406ns | 2170438ns | 2165230ns | 2169968ns | 2179651ns | -0.27% |
| abi_boundary_w_real_scalar_runtime_w | 2178281ns | 2181347ns | 2164882ns | 2176164ns | 2188156ns | base |
| abi_boundary_w_real_soa_dispatch | 898427ns | 900938ns | 887642ns | 898299ns | 904012ns | -58.76% |
| abi_boundary_w_real_soa_per_w | 891678ns | 890208ns | 883949ns | 888321ns | 900579ns | -59.07% |
| abi_boundary_w_real_soa_runtime_w | 898150ns | 896722ns | 888142ns | 894871ns | 908074ns | -58.77% |
| abi_boundary_w_real_zig_runtime_w | 2098444ns | 2096239ns | 2084138ns | 2093006ns | 2113754ns | -3.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 2501ns | 2433ns | 2559ns | -99.89% | 0.026 |
| abi_boundary_w_real_scalar_anchor | 2222134ns | 2159144ns | 2326499ns | +2.15% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2169232ns | 2162586ns | 2173712ns | -0.28% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2169550ns | 2162605ns | 2176551ns | -0.26% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2175272ns | 2162299ns | 2184826ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 895744ns | 885226ns | 901227ns | -58.82% | 0.000 |
| abi_boundary_w_real_soa_per_w | 889121ns | 881500ns | 897981ns | -59.13% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 895421ns | 885789ns | 905206ns | -58.84% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2095354ns | 2081404ns | 2110240ns | -3.67% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 27837.8 | 2737.8 | 2500.8 | n/a |
| abi_boundary_w_real_scalar_anchor | 55385.8 | 2230286.0 | 2222134.5 | n/a |
| abi_boundary_w_real_scalar_dispatch | 52987.8 | 2169189.5 | 2169231.5 | 0 |
| abi_boundary_w_real_scalar_per_w | 53141.9 | 2171921.0 | 2169549.7 | 0 |
| abi_boundary_w_real_scalar_runtime_w | 52909.6 | 2174699.9 | 2175272.2 | n/a |
| abi_boundary_w_real_soa_dispatch | 41805.8 | 896014.4 | 895743.9 | n/a |
| abi_boundary_w_real_soa_per_w | 41944.8 | 889518.3 | 889121.1 | n/a |
| abi_boundary_w_real_soa_runtime_w | 41525.4 | 896252.1 | 895421.5 | n/a |
| abi_boundary_w_real_zig_runtime_w | 211575.0 | 2096428.6 | 2095353.7 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.026 | 97.6% |
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
| abi_boundary_w_real_null_entry | 4764ns | 4764ns | -99.78% |
| abi_boundary_w_real_scalar_anchor | 2225272ns | 2225272ns | +2.16% |
| abi_boundary_w_real_scalar_dispatch | 2172222ns | 2172222ns | -0.28% |
| abi_boundary_w_real_scalar_per_w | 2172406ns | 2172406ns | -0.27% |
| abi_boundary_w_real_scalar_runtime_w | 2178281ns | 2178281ns | base |
| abi_boundary_w_real_soa_dispatch | 898427ns | 898427ns | -58.76% |
| abi_boundary_w_real_soa_per_w | 891678ns | 891678ns | -59.07% |
| abi_boundary_w_real_soa_runtime_w | 898150ns | 898150ns | -58.77% |
| abi_boundary_w_real_zig_runtime_w | 2098444ns | 2098444ns | -3.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2178284ns | base | --- | [2162707, 2184826] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 2494ns | -2175829.8ns (-99.9%) | [-2182303, -2160182]ns | [2449, 2559] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2175410ns | no significant difference | [-9770, +145664]ns | [2164494, 2326499] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2169988ns | no significant difference | [-14838, +1289]ns | [2163995, 2173712] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_per_w | 2167565ns | no significant difference | [-13396, +1826]ns | [2164533, 2176551] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_real_soa_dispatch | 898208ns | -1281399.6ns (-58.8%) | [-1285582, -1271604]ns | [887796, 901227] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_per_w | 887576ns | -1288791.9ns (-59.2%) | [-1296509, -1273153]ns | [881806, 897981] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_soa_runtime_w | 893997ns | -1275645.4ns (-58.6%) | [-1290829, -1273078]ns | [887061, 905206] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2093261ns | -80928.0ns (-3.7%) | [-87411, -71417]ns | [2082561, 2110240] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2179597ns | -99.9% | -0.2% | -0.2% | -0.2% | -58.7% | -59.3% | -58.5% | -3.7% |
| 2 | 2184952ns | -99.9% | -0.7% | -0.6% | -0.8% | -58.7% | -59.4% | -59.0% | -3.1% |
| 3 | 2184699ns | -99.9% | +11.1% | -0.7% | -0.3% | -58.8% | -58.9% | -59.2% | -3.7% |
| 4 | 2176971ns | -99.9% | +2.2% | -0.2% | -0.5% | -59.1% | -59.5% | -58.4% | -4.3% |
| 5 | 2162299ns | -99.9% | +0.6% | +0.0% | +0.0% | -58.5% | -59.2% | -59.0% | -3.7% |
| 6 | 2163115ns | -99.9% | -0.2% | +0.1% | +0.2% | -59.1% | -58.5% | -58.9% | -3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.675 | HIGH- (thermal bounce) |
| abi_boundary_w_real_scalar_anchor | -0.092 | ok |
| abi_boundary_w_real_scalar_dispatch | 0.188 | ok |
| abi_boundary_w_real_scalar_per_w | -0.015 | ok |
| abi_boundary_w_real_scalar_runtime_w | 0.547 | HIGH+ (drift/warm-up) |
| abi_boundary_w_real_soa_dispatch | 0.045 | ok |
| abi_boundary_w_real_soa_per_w | -0.337 | moderate- |
| abi_boundary_w_real_soa_runtime_w | -0.156 | ok |
| abi_boundary_w_real_zig_runtime_w | 0.446 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 3/6, lost 3/6
- **abi_boundary_w_real_scalar_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_real_scalar_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_real_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 113821.8ns | 2500.8ns | 4551.5% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6732321.9ns | 2222134.5ns | 303.0% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6565068.1ns | 2169231.5ns | 302.6% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6567749.4ns | 2169549.7ns | 302.7% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6581774.7ns | 2175272.2ns | 302.6% | HIGH |
| abi_boundary_w_real_soa_dispatch | 2731861.5ns | 895743.9ns | 305.0% | HIGH |
| abi_boundary_w_real_soa_per_w | 2711544.1ns | 889121.1ns | 305.0% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 2729992.6ns | 895421.5ns | 304.9% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6577279.3ns | 2095353.7ns | 313.9% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 2433.3-2558.9 ns)
   2433.3 |########################################
   2439.6 |
   2445.9 |
   2452.1 |
   2458.4 |
   2464.7 |########################################
   2471.0 |########################################
   2477.3 |
   2483.6 |
   2489.8 |
   2496.1 |
   2502.4 |
   2508.7 |########################################
   2515.0 |
   2521.3 |
   2527.5 |########################################
   2533.8 |
   2540.1 |
   2546.4 |
   2552.7 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2159143.7-2326499.4 ns)
  2159143.7 |#############
  2167511.5 |########################################
  2175879.3 |
  2184247.0 |
  2192614.8 |
  2200982.6 |
  2209350.4 |
  2217718.2 |#############
  2226086.0 |
  2234453.7 |
  2242821.5 |
  2251189.3 |
  2259557.1 |
  2267924.9 |
  2276292.7 |
  2284660.4 |
  2293028.2 |
  2301396.0 |
  2309763.8 |
  2318131.6 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2162586.2-2173711.7 ns)
  2162586.2 |########################################
  2163142.5 |
  2163698.8 |
  2164255.0 |
  2164811.3 |
  2165367.6 |########################################
  2165923.9 |
  2166480.1 |
  2167036.4 |
  2167592.7 |
  2168149.0 |########################################
  2168705.2 |
  2169261.5 |
  2169817.8 |
  2170374.1 |
  2170930.3 |########################################
  2171486.6 |########################################
  2172042.9 |
  2172599.2 |
  2173155.4 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2162605.0-2176551.0 ns)
  2162605.0 |########################################
  2163302.3 |
  2163999.6 |
  2164696.9 |
  2165394.2 |
  2166091.5 |########################################
  2166788.8 |########################################
  2167486.1 |########################################
  2168183.4 |
  2168880.7 |
  2169578.0 |
  2170275.3 |
  2170972.6 |
  2171669.9 |
  2172367.2 |
  2173064.5 |
  2173761.8 |
  2174459.1 |
  2175156.4 |########################################
  2175853.7 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2162298.8-2184825.9 ns)
  2162298.8 |########################################
  2163425.2 |
  2164551.5 |
  2165677.9 |
  2166804.2 |
  2167930.6 |
  2169056.9 |
  2170183.3 |
  2171309.6 |
  2172436.0 |
  2173562.3 |
  2174688.7 |
  2175815.0 |
  2176941.4 |####################
  2178067.7 |
  2179194.1 |####################
  2180320.4 |
  2181446.8 |
  2182573.1 |
  2183699.5 |####################
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 885225.8-901227.1 ns)
  885225.8 |########################################
  886025.9 |
  886825.9 |
  887626.0 |
  888426.1 |
  889226.1 |
  890026.2 |########################################
  890826.3 |
  891626.3 |
  892426.4 |
  893226.4 |
  894026.5 |
  894826.6 |
  895626.6 |
  896426.7 |########################################
  897226.8 |
  898026.8 |
  898826.9 |########################################
  899627.0 |########################################
  900427.0 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 881500.4-897981.1 ns)
  881500.4 |########################################
  882324.4 |
  883148.5 |
  883972.5 |
  884796.5 |
  885620.6 |
  886444.6 |
  887268.6 |########################################
  888092.7 |
  888916.7 |
  889740.7 |
  890564.8 |
  891388.8 |
  892212.8 |
  893036.9 |
  893860.9 |
  894684.9 |
  895509.0 |
  896333.0 |####################
  897157.0 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 885789.2-905205.8 ns)
  885789.2 |########################################
  886760.0 |
  887730.9 |########################################
  888701.7 |
  889672.5 |
  890643.4 |
  891614.2 |########################################
  892585.0 |
  893555.9 |
  894526.7 |
  895497.5 |########################################
  896468.4 |
  897439.2 |
  898410.0 |
  899380.9 |
  900351.7 |
  901322.5 |
  902293.4 |
  903264.2 |
  904235.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2081404.2-2110239.5 ns)
  2081404.2 |########################################
  2082846.0 |########################################
  2084287.7 |
  2085729.5 |
  2087171.3 |########################################
  2088613.0 |
  2090054.8 |
  2091496.6 |
  2092938.3 |
  2094380.1 |
  2095821.9 |
  2097263.6 |########################################
  2098705.4 |
  2100147.2 |
  2101588.9 |
  2103030.7 |########################################
  2104472.5 |
  2105914.2 |
  2107356.0 |
  2108797.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=4558.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=305.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=313.3% of algo (FFI overhead may distort results)
