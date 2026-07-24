# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_real_null_entry dominates: 59846% faster than the next best (abi_boundary_w_real_zig_runtime_w)

abi_boundary_w_real_null_entry (3.46 us) leads abi_boundary_w_real_zig_runtime_w (2.07 ms) by 59846%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.14 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_soa_runtime_w is an outlier: 621.1x slower than the field

abi_boundary_w_real_soa_runtime_w (2.15 ms) is 621.1x the fastest (3.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_soa_runtime_w} (59846% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_scalar_runtime_w, abi_boundary_w_real_soa_runtime_w} with a 59846% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 621.1x the fastest

Fastest abi_boundary_w_real_null_entry (3.46 us) to slowest abi_boundary_w_real_soa_runtime_w (2.15 ms): 621.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 3459.1 ns median (-99.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 621.08x (fastest 3459.1 ns, slowest 2148402.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 5704ns | 5714ns | 5448ns | 5646ns | 5920ns | -99.73% |
| abi_boundary_w_real_scalar_anchor | 2129648ns | 2133793ns | 2112814ns | 2131119ns | 2135859ns | -0.68% |
| abi_boundary_w_real_scalar_dispatch | 2138426ns | 2139248ns | 2126289ns | 2135555ns | 2148800ns | -0.27% |
| abi_boundary_w_real_scalar_per_w | 2129233ns | 2129783ns | 2120945ns | 2128229ns | 2134885ns | -0.70% |
| abi_boundary_w_real_scalar_runtime_w | 2144188ns | 2143682ns | 2121717ns | 2142325ns | 2158219ns | base |
| abi_boundary_w_real_soa_dispatch | 2138985ns | 2142282ns | 2127474ns | 2138261ns | 2145827ns | -0.24% |
| abi_boundary_w_real_soa_per_w | 2140752ns | 2140702ns | 2136246ns | 2139838ns | 2144375ns | -0.16% |
| abi_boundary_w_real_soa_runtime_w | 2152644ns | 2150999ns | 2144566ns | 2149398ns | 2161553ns | +0.39% |
| abi_boundary_w_real_zig_runtime_w | 2076595ns | 2076195ns | 2068857ns | 2074491ns | 2083620ns | -3.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 3442ns | 3268ns | 3581ns | -99.84% | 0.001 |
| abi_boundary_w_real_scalar_anchor | 2127135ns | 2110300ns | 2133219ns | -0.68% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2135889ns | 2123880ns | 2146229ns | -0.27% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2126729ns | 2118591ns | 2132280ns | -0.70% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2141717ns | 2119405ns | 2155659ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 2136444ns | 2125005ns | 2143346ns | -0.25% | 0.000 |
| abi_boundary_w_real_soa_per_w | 2138209ns | 2133736ns | 2141835ns | -0.16% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 2150080ns | 2142122ns | 2159074ns | +0.39% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2074035ns | 2066318ns | 2080996ns | -3.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 26612.0 | 3540.4 | 3441.9 | n/a |
| abi_boundary_w_real_scalar_anchor | 36207.4 | 2126129.0 | 2127134.9 | n/a |
| abi_boundary_w_real_scalar_dispatch | 37669.7 | 2134973.5 | 2135889.0 | n/a |
| abi_boundary_w_real_scalar_per_w | 36848.5 | 2127715.8 | 2126729.2 | n/a |
| abi_boundary_w_real_scalar_runtime_w | 38146.9 | 2143125.4 | 2141716.6 | n/a |
| abi_boundary_w_real_soa_dispatch | 37934.9 | 2135398.3 | 2136444.3 | n/a |
| abi_boundary_w_real_soa_per_w | 37363.9 | 2137224.6 | 2138209.0 | n/a |
| abi_boundary_w_real_soa_runtime_w | 37755.6 | 2150390.9 | 2150080.1 | 0 |
| abi_boundary_w_real_zig_runtime_w | 179010.6 | 2075423.8 | 2074034.7 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.001 | 94.5% |
| abi_boundary_w_real_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_real_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_real_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_real_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_real_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_real_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_real_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_real_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_real_null_entry | 5704ns | 5704ns | -99.73% |
| abi_boundary_w_real_scalar_anchor | 2129648ns | 2129648ns | -0.68% |
| abi_boundary_w_real_scalar_dispatch | 2138426ns | 2138426ns | -0.27% |
| abi_boundary_w_real_scalar_per_w | 2129233ns | 2129233ns | -0.70% |
| abi_boundary_w_real_scalar_runtime_w | 2144188ns | 2144188ns | base |
| abi_boundary_w_real_soa_dispatch | 2138985ns | 2138985ns | -0.24% |
| abi_boundary_w_real_soa_per_w | 2140752ns | 2140752ns | -0.16% |
| abi_boundary_w_real_soa_runtime_w | 2152644ns | 2152644ns | +0.39% |
| abi_boundary_w_real_zig_runtime_w | 2076595ns | 2076595ns | -3.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2141179ns | base | --- | [2128312, 2155659] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 3459ns | -2137654.0ns (-99.8%) | [-2152259, -2124911]ns | [3286, 3581] | YES (adj: no) | 0.0833 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2131314ns | no significant difference | [-38787, +3818]ns | [2116872, 2133219] | no | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2136624ns | no significant difference | [-21515, +11806]ns | [2124815, 2146229] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_per_w | 2127282ns | -13574.6ns (-0.6%) | [-28376, -3011]ns | [2120625, 2132280] | YES (adj: no) | 0.0833 | 0.0313 | 0 |
| abi_boundary_w_real_soa_dispatch | 2139770ns | no significant difference | [-24292, +12554]ns | [2126216, 2143346] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_soa_per_w | 2138095ns | no significant difference | [-19442, +13440]ns | [2134696, 2141835] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_real_soa_runtime_w | 2148403ns | no significant difference | [-7744, +24688]ns | [2142764, 2159074] | no | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2073625ns | -63762.1ns (-3.0%) | [-83873, -55410]ns | [2067482, 2080996] | YES (adj: no) | 0.0833 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2137219ns | -99.8% | -0.3% | -0.2% | -0.7% | +0.1% | +0.4% | +1.0% | -3.0% |
| 2 | 2158242ns | -99.8% | -2.2% | -0.5% | -1.3% | -1.5% | -1.1% | -0.7% | -4.2% |
| 3 | 2142774ns | -99.8% | -0.5% | +0.1% | -0.6% | -0.7% | -0.3% | +0.0% | -2.9% |
| 4 | 2139584ns | -99.8% | -0.3% | -0.6% | -0.2% | +0.0% | -0.1% | +0.5% | -2.7% |
| 5 | 2153076ns | -99.8% | -1.4% | -1.4% | -1.3% | -0.4% | -0.7% | +0.3% | -3.6% |
| 6 | 2119405ns | -99.8% | +0.6% | +1.0% | -0.0% | +1.1% | +0.9% | +1.3% | -2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.135 | ok |
| abi_boundary_w_real_scalar_anchor | -0.391 | moderate- |
| abi_boundary_w_real_scalar_dispatch | 0.094 | ok |
| abi_boundary_w_real_scalar_per_w | 0.159 | ok |
| abi_boundary_w_real_scalar_runtime_w | -0.364 | moderate- |
| abi_boundary_w_real_soa_dispatch | 0.330 | moderate+ |
| abi_boundary_w_real_soa_per_w | -0.241 | moderate- |
| abi_boundary_w_real_soa_runtime_w | -0.175 | ok |
| abi_boundary_w_real_zig_runtime_w | 0.127 | ok |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 5/6, lost 1/6
- **abi_boundary_w_real_scalar_dispatch**: won 4/6, lost 2/6
- **abi_boundary_w_real_scalar_per_w**: won 5/6, lost 0/6
- **abi_boundary_w_real_soa_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_real_soa_per_w**: won 3/6, lost 2/6
- **abi_boundary_w_real_soa_runtime_w**: won 1/6, lost 4/6
- **abi_boundary_w_real_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 119517.3ns | 3441.9ns | 3472.4% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6417447.2ns | 2127134.9ns | 301.7% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6446243.1ns | 2135889.0ns | 301.8% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6418680.0ns | 2126729.2ns | 301.8% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6468105.7ns | 2141716.6ns | 302.0% | HIGH |
| abi_boundary_w_real_soa_dispatch | 6452478.8ns | 2136444.3ns | 302.0% | HIGH |
| abi_boundary_w_real_soa_per_w | 6452457.9ns | 2138209.0ns | 301.8% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 6490803.5ns | 2150080.1ns | 301.9% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6472566.2ns | 2074034.7ns | 312.1% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 3267.9-3581.1 ns)
   3267.9 |########################################
   3283.6 |
   3299.2 |########################################
   3314.9 |
   3330.5 |
   3346.2 |
   3361.8 |
   3377.5 |
   3393.2 |
   3408.8 |########################################
   3424.5 |
   3440.1 |
   3455.8 |
   3471.4 |
   3487.1 |########################################
   3502.8 |
   3518.4 |########################################
   3534.1 |
   3549.7 |
   3565.4 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2110300.4-2133219.4 ns)
  2110300.4 |####################
  2111446.4 |
  2112592.3 |
  2113738.2 |
  2114884.2 |
  2116030.1 |
  2117176.1 |
  2118322.1 |
  2119468.0 |
  2120614.0 |
  2121759.9 |
  2122905.9 |####################
  2124051.8 |
  2125197.8 |
  2126343.7 |
  2127489.7 |
  2128635.6 |
  2129781.6 |
  2130927.5 |########################################
  2132073.5 |####################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2123880.0-2146228.8 ns)
  2123880.0 |########################################
  2124997.4 |########################################
  2126114.9 |
  2127232.3 |
  2128349.8 |
  2129467.2 |
  2130584.6 |
  2131702.1 |
  2132819.5 |########################################
  2133936.9 |
  2135054.4 |
  2136171.8 |
  2137289.2 |
  2138406.7 |
  2139524.1 |########################################
  2140641.6 |
  2141759.0 |
  2142876.4 |
  2143993.9 |
  2145111.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2118590.8-2132280.4 ns)
  2118590.8 |########################################
  2119275.3 |
  2119959.8 |
  2120644.2 |
  2121328.7 |
  2122013.2 |########################################
  2122697.7 |
  2123382.2 |
  2124066.6 |
  2124751.1 |
  2125435.6 |########################################
  2126120.1 |
  2126804.6 |
  2127489.0 |
  2128173.5 |
  2128858.0 |########################################
  2129542.5 |########################################
  2130227.0 |
  2130911.4 |
  2131595.9 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2119405.0-2155658.8 ns)
  2119405.0 |########################################
  2121217.7 |
  2123030.4 |
  2124843.1 |
  2126655.8 |
  2128468.4 |
  2130281.1 |
  2132093.8 |
  2133906.5 |
  2135719.2 |########################################
  2137531.9 |
  2139344.6 |########################################
  2141157.2 |########################################
  2142969.9 |
  2144782.6 |
  2146595.3 |
  2148408.0 |
  2150220.7 |
  2152033.4 |########################################
  2153846.1 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 2125005.4-2143346.0 ns)
  2125005.4 |########################################
  2125922.4 |
  2126839.5 |########################################
  2127756.5 |
  2128673.5 |
  2129590.6 |
  2130507.6 |
  2131424.6 |
  2132341.7 |
  2133258.7 |
  2134175.7 |
  2135092.8 |
  2136009.8 |
  2136926.8 |
  2137843.9 |
  2138760.9 |########################################
  2139677.9 |########################################
  2140595.0 |
  2141512.0 |########################################
  2142429.0 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 2133735.8-2141835.4 ns)
  2133735.8 |########################################
  2134140.8 |
  2134545.8 |
  2134950.7 |
  2135355.7 |########################################
  2135760.7 |
  2136165.7 |
  2136570.7 |
  2136975.6 |
  2137380.6 |########################################
  2137785.6 |
  2138190.6 |########################################
  2138595.6 |########################################
  2139000.5 |
  2139405.5 |
  2139810.5 |
  2140215.5 |
  2140620.5 |
  2141025.4 |
  2141430.4 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 2142122.1-2159074.0 ns)
  2142122.1 |########################################
  2142969.7 |########################################
  2143817.3 |
  2144664.9 |
  2145512.5 |
  2146360.1 |########################################
  2147207.7 |
  2148055.3 |
  2148902.9 |
  2149750.5 |########################################
  2150598.0 |
  2151445.6 |
  2152293.2 |
  2153140.8 |
  2153988.4 |
  2154836.0 |
  2155683.6 |
  2156531.2 |
  2157378.8 |
  2158226.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2066317.5-2080996.4 ns)
  2066317.5 |########################################
  2067051.4 |
  2067785.4 |
  2068519.3 |########################################
  2069253.3 |
  2069987.2 |
  2070721.2 |
  2071455.1 |
  2072189.1 |########################################
  2072923.0 |
  2073657.0 |
  2074390.9 |########################################
  2075124.9 |
  2075858.8 |
  2076592.8 |
  2077326.7 |
  2078060.7 |
  2078794.6 |
  2079528.6 |########################################
  2080262.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=3451.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=312.0% of algo (FFI overhead may distort results)
