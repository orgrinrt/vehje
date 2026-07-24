# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_scatter_null_entry dominates: 35219% faster than the next best (abi_boundary_w_scatter_soa_per_w)

abi_boundary_w_scatter_null_entry (2.53 us) leads abi_boundary_w_scatter_soa_per_w (894.59 us) by 35219%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_dispatch is an outlier: 853.6x slower than the field

abi_boundary_w_scatter_scalar_dispatch (2.16 ms) is 853.6x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_soa_dispatch shows alternating (throttle bounce) (autocorr -0.74)

abi_boundary_w_scatter_soa_dispatch's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_dispatch} (35219% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_runtime_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_dispatch} with a 35219% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 853.6x the fastest

Fastest abi_boundary_w_scatter_null_entry (2.53 us) to slowest abi_boundary_w_scatter_scalar_dispatch (2.16 ms): 853.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 2532.9 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 853.59x (fastest 2532.9 ns, slowest 2162065.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4870ns | 4836ns | 4634ns | 4808ns | 5082ns | -99.77% |
| abi_boundary_w_scatter_scalar_anchor | 2165247ns | 2163954ns | 2152851ns | 2161665ns | 2176818ns | +0.17% |
| abi_boundary_w_scatter_scalar_dispatch | 2174911ns | 2164904ns | 2152022ns | 2162399ns | 2205124ns | +0.61% |
| abi_boundary_w_scatter_scalar_per_w | 2158639ns | 2159179ns | 2146771ns | 2157710ns | 2165968ns | -0.14% |
| abi_boundary_w_scatter_scalar_runtime_w | 2161663ns | 2160520ns | 2151582ns | 2160243ns | 2168833ns | base |
| abi_boundary_w_scatter_soa_dispatch | 906701ns | 908136ns | 896065ns | 906136ns | 912866ns | -58.06% |
| abi_boundary_w_scatter_soa_per_w | 902142ns | 896938ns | 887228ns | 896037ns | 918756ns | -58.27% |
| abi_boundary_w_scatter_soa_runtime_w | 902134ns | 897982ns | 893235ns | 897535ns | 913480ns | -58.27% |
| abi_boundary_w_scatter_zig_runtime_w | 2131344ns | 2129385ns | 2126400ns | 2128511ns | 2138066ns | -1.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 2569ns | 2460ns | 2681ns | -99.88% | 0.006 |
| abi_boundary_w_scatter_scalar_anchor | 2162612ns | 2150410ns | 2174106ns | +0.17% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2171978ns | 2149435ns | 2201850ns | +0.60% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2156029ns | 2144306ns | 2163294ns | -0.14% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2158970ns | 2148925ns | 2166073ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 904106ns | 893557ns | 910236ns | -58.12% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 899706ns | 884942ns | 916107ns | -58.33% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 899574ns | 890894ns | 910868ns | -58.33% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2128594ns | 2123684ns | 2135206ns | -1.41% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 27271.5 | 2634.4 | 2568.6 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 41111.6 | 2158938.3 | 2162612.1 | n/a |
| abi_boundary_w_scatter_scalar_dispatch | 47889.1 | 2173131.4 | 2171977.9 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 42500.2 | 2157309.1 | 2156028.5 | 1 |
| abi_boundary_w_scatter_scalar_runtime_w | 40736.7 | 2154415.3 | 2158969.6 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 35754.4 | 910510.8 | 904105.6 | n/a |
| abi_boundary_w_scatter_soa_per_w | 34908.3 | 905650.0 | 899706.0 | n/a |
| abi_boundary_w_scatter_soa_runtime_w | 36090.8 | 900815.2 | 899574.5 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 188958.6 | 2128153.7 | 2128594.4 | 5 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.006 | 97.1% |
| abi_boundary_w_scatter_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_scatter_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_scatter_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_scatter_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_scatter_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_scatter_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 4870ns | 4870ns | -99.77% |
| abi_boundary_w_scatter_scalar_anchor | 2165247ns | 2165247ns | +0.17% |
| abi_boundary_w_scatter_scalar_dispatch | 2174911ns | 2174911ns | +0.61% |
| abi_boundary_w_scatter_scalar_per_w | 2158639ns | 2158639ns | -0.14% |
| abi_boundary_w_scatter_scalar_runtime_w | 2161663ns | 2161663ns | base |
| abi_boundary_w_scatter_soa_dispatch | 906701ns | 906701ns | -58.06% |
| abi_boundary_w_scatter_soa_per_w | 902142ns | 902142ns | -58.27% |
| abi_boundary_w_scatter_soa_runtime_w | 902134ns | 902134ns | -58.27% |
| abi_boundary_w_scatter_zig_runtime_w | 2131344ns | 2131344ns | -1.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2157939ns | base | --- | [2152897, 2166073] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 2533ns | -2155341.9ns (-99.9%) | [-2163573, -2150288]ns | [2492, 2681] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2161244ns | no significant difference | [-4513, +8895]ns | [2152486, 2174106] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2162065ns | no significant difference | [-911, +38808]ns | [2152019, 2201850] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2156561ns | no significant difference | [-16545, +9458]ns | [2148231, 2163294] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 905489ns | -1253721.5ns (-58.1%) | [-1262045, -1248826]ns | [896591, 910236] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 894589ns | -1264681.6ns (-58.6%) | [-1274665, -1238444]ns | [888422, 916107] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 895412ns | -1262328.6ns (-58.5%) | [-1268318, -1247538]ns | [892444, 910868] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2126716ns | -27119.6ns (-1.3%) | [-42212, -21794]ns | [2123861, 2135206] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2167337ns | -99.9% | +0.4% | +2.7% | -1.1% | -58.2% | -59.2% | -58.1% | -2.0% |
| 2 | 2156868ns | -99.9% | -0.3% | -0.1% | +0.1% | -58.1% | -58.6% | -58.7% | -0.9% |
| 3 | 2148925ns | -99.9% | +0.4% | +0.0% | +0.5% | -58.1% | -58.3% | -58.3% | -1.1% |
| 4 | 2158746ns | -99.9% | +0.2% | +0.9% | +0.4% | -57.8% | -56.7% | -58.6% | -1.4% |
| 5 | 2157131ns | -99.9% | -0.1% | +0.0% | -0.2% | -58.6% | -58.6% | -57.6% | -1.1% |
| 6 | 2164809ns | -99.9% | +0.4% | +0.1% | -0.5% | -58.0% | -58.5% | -58.7% | -1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.206 | moderate+ |
| abi_boundary_w_scatter_scalar_anchor | -0.410 | moderate- |
| abi_boundary_w_scatter_scalar_dispatch | -0.173 | ok |
| abi_boundary_w_scatter_scalar_per_w | -0.032 | ok |
| abi_boundary_w_scatter_scalar_runtime_w | -0.021 | ok |
| abi_boundary_w_scatter_soa_dispatch | -0.737 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_soa_per_w | -0.130 | ok |
| abi_boundary_w_scatter_soa_runtime_w | -0.447 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | -0.619 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 2/6, lost 4/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 1/6, lost 2/6
- **abi_boundary_w_scatter_scalar_per_w**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 118223.2ns | 2568.6ns | 4602.6% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6522431.9ns | 2162612.1ns | 301.6% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6569201.6ns | 2171977.9ns | 302.5% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6512155.5ns | 2156028.5ns | 302.0% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6508609.6ns | 2158969.6ns | 301.5% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 2767671.7ns | 904105.6ns | 306.1% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 2736018.3ns | 899706.0ns | 304.1% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 2738545.3ns | 899574.5ns | 304.4% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6643913.8ns | 2128594.4ns | 312.1% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 2459.6-2680.6 ns)
   2459.6 |####################
   2470.7 |
   2481.7 |
   2492.8 |
   2503.8 |
   2514.8 |########################################
   2525.9 |
   2536.9 |####################
   2548.0 |
   2559.0 |
   2570.1 |
   2581.2 |
   2592.2 |
   2603.2 |
   2614.3 |
   2625.3 |
   2636.4 |
   2647.4 |
   2658.5 |####################
   2669.5 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2150409.6-2174106.2 ns)
  2150409.6 |########################################
  2151594.4 |
  2152779.3 |
  2153964.1 |########################################
  2155148.9 |
  2156333.8 |
  2157518.6 |########################################
  2158703.4 |
  2159888.3 |
  2161073.1 |
  2162257.9 |
  2163442.8 |########################################
  2164627.6 |
  2165812.4 |
  2166997.3 |
  2168182.1 |
  2169366.9 |
  2170551.8 |
  2171736.6 |########################################
  2172921.4 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2149434.6-2201850.0 ns)
  2149434.6 |########################################
  2152055.4 |########################################
  2154676.1 |
  2157296.9 |########################################
  2159917.7 |
  2162538.5 |
  2165159.2 |########################################
  2167780.0 |
  2170400.8 |
  2173021.5 |
  2175642.3 |########################################
  2178263.1 |
  2180883.8 |
  2183504.6 |
  2186125.4 |
  2188746.1 |
  2191366.9 |
  2193987.7 |
  2196608.5 |
  2199229.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2144306.2-2163294.1 ns)
  2144306.2 |########################################
  2145255.6 |
  2146205.0 |
  2147154.4 |
  2148103.8 |
  2149053.2 |
  2150002.6 |
  2150952.0 |
  2151901.4 |########################################
  2152850.8 |
  2153800.2 |########################################
  2154749.6 |
  2155699.0 |
  2156648.4 |
  2157597.8 |########################################
  2158547.2 |
  2159496.6 |########################################
  2160446.0 |
  2161395.4 |
  2162344.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2148925.4-2166073.2 ns)
  2148925.4 |####################
  2149782.8 |
  2150640.2 |
  2151497.6 |
  2152355.0 |
  2153212.3 |
  2154069.7 |
  2154927.1 |
  2155784.5 |
  2156641.9 |########################################
  2157499.3 |
  2158356.7 |####################
  2159214.1 |
  2160071.4 |
  2160928.8 |
  2161786.2 |
  2162643.6 |
  2163501.0 |
  2164358.4 |####################
  2165215.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 893557.1-910236.4 ns)
  893557.1 |########################################
  894391.1 |
  895225.0 |
  896059.0 |
  896893.0 |
  897726.9 |
  898560.9 |
  899394.9 |########################################
  900228.8 |
  901062.8 |
  901896.8 |
  902730.7 |
  903564.7 |########################################
  904398.7 |
  905232.6 |
  906066.6 |########################################
  906900.6 |
  907734.5 |
  908568.5 |
  909402.5 |########################################
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 884941.7-916107.1 ns)
  884941.7 |########################################
  886500.0 |
  888058.2 |
  889616.5 |
  891174.8 |########################################
  892733.1 |########################################
  894291.3 |
  895849.6 |########################################
  897407.9 |########################################
  898966.1 |
  900524.4 |
  902082.7 |
  903640.9 |
  905199.2 |
  906757.5 |
  908315.8 |
  909874.0 |
  911432.3 |
  912990.6 |
  914548.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 890893.8-910867.9 ns)
  890893.8 |####################
  891892.5 |
  892891.2 |
  893889.9 |########################################
  894888.6 |
  895887.3 |####################
  896886.0 |
  897884.7 |
  898883.4 |
  899882.1 |
  900880.9 |
  901879.6 |
  902878.3 |
  903877.0 |
  904875.7 |
  905874.4 |
  906873.1 |####################
  907871.8 |
  908870.5 |
  909869.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2123683.8-2135205.9 ns)
  2123683.8 |########################################
  2124259.9 |
  2124836.0 |####################
  2125412.1 |
  2125988.2 |
  2126564.3 |
  2127140.4 |
  2127716.5 |
  2128292.6 |####################
  2128868.7 |
  2129444.8 |
  2130020.9 |
  2130597.0 |
  2131173.1 |
  2131749.2 |
  2132325.3 |
  2132901.4 |####################
  2133477.5 |
  2134053.6 |
  2134629.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=4662.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=312.1% of algo (FFI overhead may distort results)
