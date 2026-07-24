# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 38056% faster than the next best (abi_boundary_w_tight_soa_per_w)

abi_boundary_w_tight_null_entry (2.55 us) leads abi_boundary_w_tight_soa_per_w (974.07 us) by 38056%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.04 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 808.6x slower than the field

abi_boundary_w_tight_scalar_anchor (2.06 ms) is 808.6x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} (38056% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} with a 38056% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 808.6x the fastest

Fastest abi_boundary_w_tight_null_entry (2.55 us) to slowest abi_boundary_w_tight_scalar_anchor (2.06 ms): 808.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 2552.9 ns median (-99.9% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 808.59x (fastest 2552.9 ns, slowest 2064260.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4858ns | 4865ns | 4702ns | 4837ns | 4966ns | -99.76% |
| abi_boundary_w_tight_scalar_anchor | 2065247ns | 2067565ns | 2052989ns | 2065252ns | 2071368ns | +1.17% |
| abi_boundary_w_tight_scalar_dispatch | 2048916ns | 2045982ns | 2035781ns | 2044593ns | 2061968ns | +0.37% |
| abi_boundary_w_tight_scalar_per_w | 2045915ns | 2043682ns | 2036179ns | 2041911ns | 2056789ns | +0.22% |
| abi_boundary_w_tight_scalar_runtime_w | 2041346ns | 2044237ns | 2026859ns | 2041943ns | 2047692ns | base |
| abi_boundary_w_tight_soa_dispatch | 983775ns | 981685ns | 978090ns | 981092ns | 990643ns | -51.81% |
| abi_boundary_w_tight_soa_per_w | 974704ns | 977194ns | 962284ns | 974546ns | 981150ns | -52.25% |
| abi_boundary_w_tight_soa_runtime_w | 986129ns | 988682ns | 973404ns | 986483ns | 991960ns | -51.69% |
| abi_boundary_w_tight_zig_runtime_w | 2031074ns | 2030895ns | 2015646ns | 2027293ns | 2044460ns | -0.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 2552ns | 2502ns | 2589ns | -99.87% | 0.006 |
| abi_boundary_w_tight_scalar_anchor | 2061889ns | 2049653ns | 2067852ns | +1.18% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2045518ns | 2032696ns | 2058443ns | +0.37% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2042552ns | 2032856ns | 2053364ns | +0.23% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2037898ns | 2023503ns | 2044045ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 980818ns | 975194ns | 987752ns | -51.87% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 971764ns | 959740ns | 978004ns | -52.32% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 982983ns | 970429ns | 988639ns | -51.76% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2027520ns | 2012248ns | 2040767ns | -0.51% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 28295.4 | 2635.9 | 2552.3 | n/a |
| abi_boundary_w_tight_scalar_anchor | 65964.6 | 2056503.9 | 2061888.5 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 64366.1 | 2046474.7 | 2045517.6 | 0 |
| abi_boundary_w_tight_scalar_per_w | 69270.4 | 2044225.1 | 2042552.1 | n/a |
| abi_boundary_w_tight_scalar_runtime_w | 66263.3 | 2045285.7 | 2037897.8 | n/a |
| abi_boundary_w_tight_soa_dispatch | 49368.8 | 978914.3 | 980818.5 | n/a |
| abi_boundary_w_tight_soa_per_w | 51452.8 | 969295.3 | 971763.7 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 54670.3 | 984597.2 | 982983.3 | n/a |
| abi_boundary_w_tight_zig_runtime_w | 243751.5 | 2028234.7 | 2027520.3 | 17 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.006 | 98.0% |
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
| abi_boundary_w_tight_null_entry | 4858ns | 4858ns | -99.76% |
| abi_boundary_w_tight_scalar_anchor | 2065247ns | 2065247ns | +1.17% |
| abi_boundary_w_tight_scalar_dispatch | 2048916ns | 2048916ns | +0.37% |
| abi_boundary_w_tight_scalar_per_w | 2045915ns | 2045915ns | +0.22% |
| abi_boundary_w_tight_scalar_runtime_w | 2041346ns | 2041346ns | base |
| abi_boundary_w_tight_soa_dispatch | 983775ns | 983775ns | -51.81% |
| abi_boundary_w_tight_soa_per_w | 974704ns | 974704ns | -52.25% |
| abi_boundary_w_tight_soa_runtime_w | 986129ns | 986129ns | -51.69% |
| abi_boundary_w_tight_zig_runtime_w | 2031074ns | 2031074ns | -0.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2040775ns | base | --- | [2028873, 2044045] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 2553ns | -2038216.0ns (-99.9%) | [-2041479, -2026341]ns | [2515, 2589] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2064260ns | +23923.4ns (+1.2%) | [+14053, +33996]ns | [2053553, 2067852] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2042542ns | no significant difference | [-2006, +19922]ns | [2035568, 2058443] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2040224ns | no significant difference | [-7881, +15088]ns | [2034068, 2053364] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_dispatch | 978910ns | -1057415.9ns (-51.8%) | [-1063721, -1050101]ns | [975793, 987752] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_per_w | 974072ns | -1067129.8ns (-52.3%) | [-1077560, -1053712]ns | [963215, 978004] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 985539ns | -1054231.9ns (-51.7%) | [-1067178, -1043333]ns | [974772, 988639] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2027362ns | no significant difference | [-26904, +8417]ns | [2014432, 2040767] | no | 0.7857 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2034242ns | -99.9% | +1.7% | -0.1% | +0.7% | -51.9% | -52.0% | -51.6% | +0.3% |
| 2 | 2042310ns | -99.9% | +1.1% | +0.9% | +0.8% | -52.1% | -52.7% | -51.6% | -0.1% |
| 3 | 2044659ns | -99.9% | +1.1% | +0.5% | -0.6% | -51.5% | -52.4% | -52.5% | -1.2% |
| 4 | 2043432ns | -99.9% | +0.3% | -0.1% | +0.0% | -51.8% | -52.0% | -51.6% | -1.3% |
| 5 | 2023503ns | -99.9% | +1.7% | +1.0% | +0.6% | -51.7% | -51.8% | -51.3% | +0.5% |
| 6 | 2039240ns | -99.9% | +1.2% | -0.0% | -0.2% | -52.2% | -52.9% | -52.0% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.392 | moderate- |
| abi_boundary_w_tight_scalar_anchor | 0.013 | ok |
| abi_boundary_w_tight_scalar_dispatch | -0.129 | ok |
| abi_boundary_w_tight_scalar_per_w | -0.100 | ok |
| abi_boundary_w_tight_scalar_runtime_w | -0.150 | ok |
| abi_boundary_w_tight_soa_dispatch | 0.151 | ok |
| abi_boundary_w_tight_soa_per_w | -0.091 | ok |
| abi_boundary_w_tight_soa_runtime_w | -0.499 | moderate- |
| abi_boundary_w_tight_zig_runtime_w | -0.004 | ok |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 0/6, lost 6/6
- **abi_boundary_w_tight_scalar_dispatch**: won 1/6, lost 3/6
- **abi_boundary_w_tight_scalar_per_w**: won 2/6, lost 3/6
- **abi_boundary_w_tight_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_zig_runtime_w**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 119458.0ns | 2552.3ns | 4680.4% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6249663.8ns | 2061888.5ns | 303.1% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6203930.3ns | 2045517.6ns | 303.3% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6197991.7ns | 2042552.1ns | 303.4% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6198221.4ns | 2037897.8ns | 304.1% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 2989450.3ns | 980818.5ns | 304.8% | HIGH |
| abi_boundary_w_tight_soa_per_w | 2961401.3ns | 971763.7ns | 304.7% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 3008054.1ns | 982983.3ns | 306.0% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6413925.5ns | 2027520.3ns | 316.3% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 2502.5-2588.8 ns)
   2502.5 |########################################
   2506.8 |
   2511.1 |
   2515.4 |
   2519.8 |
   2524.1 |########################################
   2528.4 |
   2532.7 |
   2537.0 |
   2541.3 |########################################
   2545.7 |
   2550.0 |
   2554.3 |
   2558.6 |########################################
   2562.9 |
   2567.2 |
   2571.5 |########################################
   2575.9 |
   2580.2 |
   2584.5 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2049652.9-2067852.4 ns)
  2049652.9 |########################################
  2050562.9 |
  2051472.9 |
  2052382.8 |
  2053292.8 |
  2054202.8 |
  2055112.8 |
  2056022.7 |
  2056932.7 |########################################
  2057842.7 |
  2058752.7 |
  2059662.7 |
  2060572.6 |
  2061482.6 |
  2062392.6 |
  2063302.6 |########################################
  2064212.5 |########################################
  2065122.5 |
  2066032.5 |
  2066942.5 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2032695.8-2058443.1 ns)
  2032695.8 |########################################
  2033983.2 |
  2035270.5 |
  2036557.9 |
  2037845.3 |########################################
  2039132.6 |
  2040420.0 |########################################
  2041707.4 |
  2042994.7 |########################################
  2044282.1 |
  2045569.5 |
  2046856.8 |
  2048144.2 |
  2049431.6 |
  2050718.9 |
  2052006.3 |
  2053293.7 |
  2054581.0 |########################################
  2055868.4 |
  2057155.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2032856.2-2053364.4 ns)
  2032856.2 |########################################
  2033881.6 |
  2034907.0 |########################################
  2035932.4 |########################################
  2036957.8 |
  2037983.2 |
  2039008.7 |
  2040034.1 |
  2041059.5 |
  2042084.9 |
  2043110.3 |
  2044135.7 |########################################
  2045161.1 |
  2046186.5 |
  2047211.9 |########################################
  2048237.3 |
  2049262.8 |
  2050288.2 |
  2051313.6 |
  2052339.0 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2023503.3-2044045.2 ns)
  2023503.3 |########################################
  2024530.4 |
  2025557.5 |
  2026584.6 |
  2027611.7 |
  2028638.8 |
  2029665.9 |
  2030693.0 |
  2031720.1 |
  2032747.2 |
  2033774.2 |########################################
  2034801.3 |
  2035828.4 |
  2036855.5 |
  2037882.6 |
  2038909.7 |########################################
  2039936.8 |
  2040963.9 |
  2041991.0 |########################################
  2043018.1 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 975193.8-987752.3 ns)
  975193.8 |####################
  975821.7 |####################
  976449.7 |
  977077.6 |
  977705.5 |
  978333.4 |########################################
  978961.4 |
  979589.3 |
  980217.2 |
  980845.1 |
  981473.1 |
  982101.0 |
  982728.9 |
  983356.8 |####################
  983984.8 |
  984612.7 |
  985240.6 |
  985868.5 |
  986496.5 |
  987124.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 959740.0-978003.9 ns)
  959740.0 |########################################
  960653.2 |
  961566.4 |
  962479.6 |
  963392.8 |
  964306.0 |
  965219.2 |
  966132.4 |########################################
  967045.6 |
  967958.8 |
  968872.0 |
  969785.2 |
  970698.4 |
  971611.6 |
  972524.8 |
  973438.0 |########################################
  974351.2 |########################################
  975264.4 |########################################
  976177.6 |
  977090.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 970428.8-988639.2 ns)
  970428.8 |########################################
  971339.3 |
  972249.8 |
  973160.4 |
  974070.9 |
  974981.4 |
  975891.9 |
  976802.4 |
  977712.9 |
  978623.5 |########################################
  979534.0 |
  980444.5 |
  981355.0 |
  982265.5 |
  983176.0 |
  984086.6 |
  984997.1 |########################################
  985907.6 |########################################
  986818.1 |
  987728.6 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2012248.3-2040767.0 ns)
  2012248.3 |########################################
  2013674.2 |
  2015100.2 |
  2016526.1 |########################################
  2017952.1 |
  2019378.0 |
  2020803.9 |########################################
  2022229.9 |
  2023655.8 |
  2025081.7 |
  2026507.7 |
  2027933.6 |
  2029359.5 |
  2030785.5 |
  2032211.4 |
  2033637.4 |########################################
  2035063.3 |
  2036489.2 |
  2037915.2 |
  2039341.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=4676.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=305.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=305.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=305.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=316.1% of algo (FFI overhead may distort results)
