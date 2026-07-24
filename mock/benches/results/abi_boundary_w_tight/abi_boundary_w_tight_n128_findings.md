# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 36182% faster than the next best (abi_boundary_w_tight_soa_dispatch)

abi_boundary_w_tight_null_entry (2.68 us) leads abi_boundary_w_tight_soa_dispatch (972.51 us) by 36182%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.03 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 765.6x slower than the field

abi_boundary_w_tight_scalar_anchor (2.05 ms) is 765.6x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_anchor} (36182% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_anchor} with a 36182% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 765.6x the fastest

Fastest abi_boundary_w_tight_null_entry (2.68 us) to slowest abi_boundary_w_tight_scalar_anchor (2.05 ms): 765.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 2680.4 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 765.64x (fastest 2680.4 ns, slowest 2052232.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4971ns | 4908ns | 4864ns | 4902ns | 5128ns | -99.76% |
| abi_boundary_w_tight_scalar_anchor | 2061693ns | 2055508ns | 2040299ns | 2053050ns | 2085355ns | +1.28% |
| abi_boundary_w_tight_scalar_dispatch | 2038676ns | 2033082ns | 2029193ns | 2032506ns | 2052672ns | +0.15% |
| abi_boundary_w_tight_scalar_per_w | 2046280ns | 2040235ns | 2024665ns | 2039039ns | 2067949ns | +0.52% |
| abi_boundary_w_tight_scalar_runtime_w | 2035696ns | 2032952ns | 2024625ns | 2031090ns | 2048141ns | base |
| abi_boundary_w_tight_soa_dispatch | 988761ns | 974961ns | 965350ns | 972407ns | 1024998ns | -51.43% |
| abi_boundary_w_tight_soa_per_w | 975330ns | 976812ns | 965770ns | 974760ns | 980966ns | -52.09% |
| abi_boundary_w_tight_soa_runtime_w | 974954ns | 975825ns | 961842ns | 974823ns | 981705ns | -52.11% |
| abi_boundary_w_tight_zig_runtime_w | 2014061ns | 2014350ns | 2005297ns | 2012705ns | 2020475ns | -1.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 2699ns | 2658ns | 2752ns | -99.87% | 0.047 |
| abi_boundary_w_tight_scalar_anchor | 2058443ns | 2037548ns | 2081877ns | +1.27% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2035565ns | 2026423ns | 2049175ns | +0.15% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2043186ns | 2022155ns | 2064522ns | +0.52% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2032589ns | 2021864ns | 2044772ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 985803ns | 962711ns | 1021223ns | -51.50% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 972545ns | 963268ns | 978105ns | -52.15% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 972160ns | 959402ns | 978659ns | -52.17% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2011014ns | 2002280ns | 2017256ns | -1.06% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 27207.4 | 2734.0 | 2699.3 | n/a |
| abi_boundary_w_tight_scalar_anchor | 58929.4 | 2059703.8 | 2058443.0 | n/a |
| abi_boundary_w_tight_scalar_dispatch | 55301.1 | 2037747.4 | 2035565.1 | n/a |
| abi_boundary_w_tight_scalar_per_w | 58506.0 | 2045122.2 | 2043185.6 | n/a |
| abi_boundary_w_tight_scalar_runtime_w | 54728.8 | 2032652.8 | 2032589.0 | n/a |
| abi_boundary_w_tight_soa_dispatch | 49509.4 | 981648.1 | 985802.8 | n/a |
| abi_boundary_w_tight_soa_per_w | 45875.2 | 973135.5 | 972544.5 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 43430.0 | 971163.0 | 972159.9 | n/a |
| abi_boundary_w_tight_zig_runtime_w | 210834.4 | 2010200.5 | 2011013.7 | 7 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.048 | 99.1% |
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
| abi_boundary_w_tight_null_entry | 4971ns | 4971ns | -99.76% |
| abi_boundary_w_tight_scalar_anchor | 2061693ns | 2061693ns | +1.28% |
| abi_boundary_w_tight_scalar_dispatch | 2038676ns | 2038676ns | +0.15% |
| abi_boundary_w_tight_scalar_per_w | 2046280ns | 2046280ns | +0.52% |
| abi_boundary_w_tight_scalar_runtime_w | 2035696ns | 2035696ns | base |
| abi_boundary_w_tight_soa_dispatch | 988761ns | 988761ns | -51.43% |
| abi_boundary_w_tight_soa_per_w | 975330ns | 975330ns | -52.09% |
| abi_boundary_w_tight_soa_runtime_w | 974954ns | 974954ns | -52.11% |
| abi_boundary_w_tight_zig_runtime_w | 2014061ns | 2014061ns | -1.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2029849ns | base | --- | [2023146, 2044772] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 2680ns | -2027125.6ns (-99.9%) | [-2042107, -2020437]ns | [2665, 2752] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2052233ns | +22383.5ns (+1.1%) | [+12636, +42542]ns | [2041219, 2081877] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2030210ns | no significant difference | [-1152, +7532]ns | [2027310, 2049175] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2037218ns | +10773.3ns (+0.5%) | [+516, +20501]ns | [2027817, 2064522] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_dispatch | 972506ns | -1056924.0ns (-52.1%) | [-1065034, -1018400]ns | [963679, 1021223] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_per_w | 973855ns | -1053890.4ns (-51.9%) | [-1077130, -1049112]ns | [965674, 978105] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 973146ns | -1056301.2ns (-52.0%) | [-1074265, -1050721]ns | [964675, 978659] | YES | 0.0357 | 0.0313 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2011451ns | -23332.5ns (-1.1%) | [-33748, -7645]ns | [2004334, 2017256] | YES | 0.0357 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2027945ns | -99.9% | +1.2% | +0.1% | +0.5% | -51.9% | -51.8% | -51.6% | -1.0% |
| 2 | 2056805ns | -99.9% | +2.1% | -0.0% | +1.4% | -48.1% | -52.7% | -52.8% | -1.8% |
| 3 | 2021864ns | -99.9% | +2.0% | +0.2% | +0.0% | -51.8% | -52.1% | -52.5% | -0.3% |
| 4 | 2032738ns | -99.9% | +0.2% | -0.1% | +0.0% | -52.3% | -52.6% | -52.0% | -1.3% |
| 5 | 2024428ns | -99.9% | +1.0% | +0.2% | +0.6% | -52.4% | -51.8% | -52.1% | -0.4% |
| 6 | 2031754ns | -99.9% | +1.1% | +0.5% | +0.6% | -52.5% | -51.9% | -51.9% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.489 | moderate- |
| abi_boundary_w_tight_scalar_anchor | 0.058 | ok |
| abi_boundary_w_tight_scalar_dispatch | -0.442 | moderate- |
| abi_boundary_w_tight_scalar_per_w | -0.367 | moderate- |
| abi_boundary_w_tight_scalar_runtime_w | -0.466 | moderate- |
| abi_boundary_w_tight_soa_dispatch | -0.079 | ok |
| abi_boundary_w_tight_soa_per_w | 0.156 | ok |
| abi_boundary_w_tight_soa_runtime_w | -0.197 | ok |
| abi_boundary_w_tight_zig_runtime_w | -0.362 | moderate- |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 0/6, lost 6/6
- **abi_boundary_w_tight_scalar_dispatch**: won 0/6, lost 3/6
- **abi_boundary_w_tight_scalar_per_w**: won 0/6, lost 4/6
- **abi_boundary_w_tight_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 119978.1ns | 2699.3ns | 4444.8% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6241552.3ns | 2058443.0ns | 303.2% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6177468.4ns | 2035565.1ns | 303.5% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6186223.8ns | 2043185.6ns | 302.8% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6156704.8ns | 2032589.0ns | 302.9% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 3001221.9ns | 985802.8ns | 304.4% | HIGH |
| abi_boundary_w_tight_soa_per_w | 2967113.2ns | 972544.5ns | 305.1% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 2959455.4ns | 972159.9ns | 304.4% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6318881.8ns | 2011013.7ns | 314.2% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 2657.5-2752.5 ns)
   2657.5 |########################################
   2662.2 |
   2667.0 |
   2671.8 |########################################
   2676.5 |########################################
   2681.2 |########################################
   2686.0 |
   2690.8 |
   2695.5 |
   2700.2 |
   2705.0 |
   2709.8 |
   2714.5 |
   2719.2 |
   2724.0 |
   2728.8 |
   2733.5 |########################################
   2738.2 |
   2743.0 |
   2747.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2037547.9-2081876.9 ns)
  2037547.9 |########################################
  2039764.3 |
  2041980.8 |
  2044197.2 |########################################
  2046413.7 |
  2048630.1 |
  2050846.6 |########################################
  2053063.0 |########################################
  2055279.5 |
  2057495.9 |
  2059712.4 |
  2061928.8 |########################################
  2064145.3 |
  2066361.7 |
  2068578.2 |
  2070794.6 |
  2073011.1 |
  2075227.5 |
  2077444.0 |
  2079660.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2026423.3-2049174.8 ns)
  2026423.3 |########################################
  2027560.9 |########################################
  2028698.4 |########################################
  2029836.0 |
  2030973.6 |########################################
  2032111.2 |
  2033248.8 |
  2034386.3 |
  2035523.9 |
  2036661.5 |
  2037799.0 |
  2038936.6 |
  2040074.2 |
  2041211.8 |########################################
  2042349.3 |
  2043486.9 |
  2044624.5 |
  2045762.1 |
  2046899.6 |
  2048037.2 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2022155.4-2064522.3 ns)
  2022155.4 |####################
  2024273.7 |
  2026392.1 |
  2028510.4 |
  2030628.8 |
  2032747.1 |####################
  2034865.5 |
  2036983.8 |########################################
  2039102.2 |
  2041220.5 |
  2043338.9 |####################
  2045457.2 |
  2047575.5 |
  2049693.9 |
  2051812.2 |
  2053930.6 |
  2056048.9 |
  2058167.3 |
  2060285.6 |
  2062404.0 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2021864.2-2044771.6 ns)
  2021864.2 |########################################
  2023009.6 |
  2024154.9 |########################################
  2025300.3 |
  2026445.7 |
  2027591.1 |########################################
  2028736.4 |
  2029881.8 |
  2031027.2 |########################################
  2032172.6 |########################################
  2033317.9 |
  2034463.3 |
  2035608.7 |
  2036754.0 |
  2037899.4 |
  2039044.8 |
  2040190.2 |
  2041335.5 |
  2042480.9 |
  2043626.3 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 962710.8-1021223.3 ns)
  962710.8 |########################################
  965636.4 |
  968562.1 |####################
  971487.7 |
  974413.3 |########################################
  977338.9 |
  980264.6 |
  983190.2 |
  986115.8 |
  989041.4 |
  991967.1 |
  994892.7 |
  997818.3 |
  1000743.9 |
  1003669.6 |
  1006595.2 |
  1009520.8 |
  1012446.4 |
  1015372.1 |
  1018297.7 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 963267.5-978104.8 ns)
  963267.5 |########################################
  964009.4 |
  964751.2 |
  965493.1 |
  966235.0 |
  966976.8 |
  967718.7 |########################################
  968460.6 |
  969202.4 |
  969944.3 |
  970686.2 |
  971428.0 |########################################
  972169.9 |
  972911.7 |
  973653.6 |
  974395.5 |
  975137.3 |########################################
  975879.2 |
  976621.1 |
  977362.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 959401.7-978659.4 ns)
  959401.7 |########################################
  960364.6 |
  961327.5 |
  962290.3 |
  963253.2 |
  964216.1 |
  965179.0 |
  966141.9 |
  967104.8 |
  968067.6 |
  969030.5 |########################################
  969993.4 |########################################
  970956.3 |
  971919.2 |
  972882.1 |
  973844.9 |
  974807.8 |########################################
  975770.7 |########################################
  976733.6 |
  977696.5 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2002280.0-2017256.2 ns)
  2002280.0 |####################
  2003028.8 |
  2003777.6 |
  2004526.4 |
  2005275.2 |
  2006024.1 |####################
  2006772.9 |
  2007521.7 |####################
  2008270.5 |
  2009019.3 |
  2009768.1 |
  2010516.9 |
  2011265.7 |
  2012014.5 |
  2012763.3 |
  2013512.1 |
  2014261.0 |
  2015009.8 |########################################
  2015758.6 |
  2016507.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=4461.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=305.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=313.8% of algo (FFI overhead may distort results)
