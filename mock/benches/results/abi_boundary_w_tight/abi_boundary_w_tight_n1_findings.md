# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 41440% faster than the next best (abi_boundary_w_tight_zig_runtime_w)

abi_boundary_w_tight_null_entry (4.87 us) leads abi_boundary_w_tight_zig_runtime_w (2.02 ms) by 41440%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.03 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_soa_per_w is an outlier: 422.4x slower than the field

abi_boundary_w_tight_soa_per_w (2.06 ms) is 422.4x the fastest (4.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_scalar_anchor shows warm-up / thermal drift (autocorr +0.53)

abi_boundary_w_tight_scalar_anchor's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_anchor, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_soa_per_w} (41440% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_anchor, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_soa_per_w} with a 41440% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 422.4x the fastest

Fastest abi_boundary_w_tight_null_entry (4.87 us) to slowest abi_boundary_w_tight_soa_per_w (2.06 ms): 422.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 4870.6 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 422.36x (fastest 4870.6 ns, slowest 2057143.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 7108ns | 7130ns | 6998ns | 7108ns | 7163ns | -99.65% |
| abi_boundary_w_tight_scalar_anchor | 2042238ns | 2039004ns | 2026238ns | 2035339ns | 2060585ns | +0.03% |
| abi_boundary_w_tight_scalar_dispatch | 2043455ns | 2040769ns | 2028055ns | 2036949ns | 2060915ns | +0.09% |
| abi_boundary_w_tight_scalar_per_w | 2045342ns | 2048291ns | 2027091ns | 2041409ns | 2060368ns | +0.18% |
| abi_boundary_w_tight_scalar_runtime_w | 2041598ns | 2038500ns | 2025906ns | 2037232ns | 2055994ns | base |
| abi_boundary_w_tight_soa_dispatch | 2043892ns | 2042686ns | 2021620ns | 2040965ns | 2059419ns | +0.11% |
| abi_boundary_w_tight_soa_per_w | 2056737ns | 2060149ns | 2019697ns | 2056042ns | 2076301ns | +0.74% |
| abi_boundary_w_tight_soa_runtime_w | 2041120ns | 2043258ns | 2025789ns | 2037803ns | 2053762ns | -0.02% |
| abi_boundary_w_tight_zig_runtime_w | 2024684ns | 2026486ns | 2004971ns | 2025577ns | 2033201ns | -0.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4862ns | 4787ns | 4908ns | -99.76% | 0.000 |
| abi_boundary_w_tight_scalar_anchor | 2039459ns | 2023670ns | 2057574ns | +0.04% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2040642ns | 2025773ns | 2057542ns | +0.10% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2042418ns | 2024512ns | 2057166ns | +0.19% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2038645ns | 2023273ns | 2052821ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 2040945ns | 2019035ns | 2056129ns | +0.11% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 2053770ns | 2017380ns | 2073077ns | +0.74% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 2038216ns | 2023235ns | 2050694ns | -0.02% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2021537ns | 2002368ns | 2029923ns | -0.84% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 26755.0 | 4915.3 | 4862.2 | n/a |
| abi_boundary_w_tight_scalar_anchor | 49496.8 | 2039120.2 | 2039459.2 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 47499.7 | 2039038.5 | 2040642.2 | 1 |
| abi_boundary_w_tight_scalar_per_w | 51132.5 | 2042935.1 | 2042417.7 | n/a |
| abi_boundary_w_tight_scalar_runtime_w | 49643.4 | 2040439.6 | 2038644.9 | n/a |
| abi_boundary_w_tight_soa_dispatch | 52452.1 | 2038409.3 | 2040944.8 | n/a |
| abi_boundary_w_tight_soa_per_w | 52637.5 | 2053131.4 | 2053770.3 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 49837.4 | 2039993.7 | 2038216.0 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 213439.5 | 2021453.0 | 2021536.8 | 10 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.000 | 98.3% |
| abi_boundary_w_tight_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_tight_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_tight_null_entry | 7108ns | 7108ns | -99.65% |
| abi_boundary_w_tight_scalar_anchor | 2042238ns | 2042238ns | +0.03% |
| abi_boundary_w_tight_scalar_dispatch | 2043455ns | 2043455ns | +0.09% |
| abi_boundary_w_tight_scalar_per_w | 2045342ns | 2045342ns | +0.18% |
| abi_boundary_w_tight_scalar_runtime_w | 2041598ns | 2041598ns | base |
| abi_boundary_w_tight_soa_dispatch | 2043892ns | 2043892ns | +0.11% |
| abi_boundary_w_tight_soa_per_w | 2056737ns | 2056737ns | +0.74% |
| abi_boundary_w_tight_soa_runtime_w | 2041120ns | 2041120ns | -0.02% |
| abi_boundary_w_tight_zig_runtime_w | 2024684ns | 2024684ns | -0.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2035600ns | base | --- | [2027514, 2052821] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 4871ns | -2030737.4ns (-99.8%) | [-2047990, -2022621]ns | [4808, 4908] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2036175ns | no significant difference | [-13109, +18226]ns | [2024629, 2057574] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2038053ns | no significant difference | [-9415, +14812]ns | [2026332, 2057542] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2045292ns | no significant difference | [-15686, +23277]ns | [2024795, 2057166] | no | 1.0000 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_dispatch | 2039879ns | no significant difference | [-12136, +17438]ns | [2026826, 2056129] | no | 1.0000 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_per_w | 2057144ns | no significant difference | [-11521, +38466]ns | [2031089, 2073077] | no | 1.0000 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 2040152ns | no significant difference | [-9439, +9259]ns | [2023802, 2050694] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2023292ns | -15368.1ns (-0.8%) | [-31149, -4807]ns | [2011395, 2029923] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2035176ns | -99.8% | -0.5% | -0.5% | -0.5% | -0.8% | +0.5% | -0.2% | -1.6% |
| 2 | 2023273ns | -99.8% | +0.1% | +0.3% | +0.1% | +0.7% | -0.3% | +0.1% | -0.1% |
| 3 | 2031755ns | -99.8% | -0.4% | -0.2% | +1.6% | +0.1% | +1.8% | -0.4% | -0.3% |
| 4 | 2062970ns | -99.8% | -0.8% | -0.5% | -1.0% | -0.4% | -0.8% | -0.5% | -1.4% |
| 5 | 2042671ns | -99.8% | +0.6% | +0.9% | +0.3% | +0.0% | +1.3% | +0.3% | -1.0% |
| 6 | 2036024ns | -99.8% | +1.2% | +0.5% | +0.7% | +1.1% | +2.0% | +0.6% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.459 | moderate- |
| abi_boundary_w_tight_scalar_anchor | 0.527 | HIGH+ (drift/warm-up) |
| abi_boundary_w_tight_scalar_dispatch | 0.462 | moderate+ |
| abi_boundary_w_tight_scalar_per_w | -0.033 | ok |
| abi_boundary_w_tight_scalar_runtime_w | 0.087 | ok |
| abi_boundary_w_tight_soa_dispatch | 0.092 | ok |
| abi_boundary_w_tight_soa_per_w | -0.037 | ok |
| abi_boundary_w_tight_soa_runtime_w | 0.386 | moderate+ |
| abi_boundary_w_tight_zig_runtime_w | 0.109 | ok |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 3/6, lost 3/6
- **abi_boundary_w_tight_scalar_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_tight_scalar_per_w**: won 2/6, lost 3/6
- **abi_boundary_w_tight_soa_dispatch**: won 2/6, lost 3/6
- **abi_boundary_w_tight_soa_per_w**: won 2/6, lost 4/6
- **abi_boundary_w_tight_soa_runtime_w**: won 3/6, lost 2/6
- **abi_boundary_w_tight_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 123975.5ns | 4862.2ns | 2549.8% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6171582.9ns | 2039459.2ns | 302.6% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6173448.3ns | 2040642.2ns | 302.5% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6184556.2ns | 2042417.7ns | 302.8% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6172534.6ns | 2038644.9ns | 302.8% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 6171158.8ns | 2040944.8ns | 302.4% | HIGH |
| abi_boundary_w_tight_soa_per_w | 6213451.3ns | 2053770.3ns | 302.5% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 6168641.4ns | 2038216.0ns | 302.6% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6356009.3ns | 2021536.8ns | 314.4% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 4786.7-4908.1 ns)
   4786.7 |########################################
   4792.8 |
   4798.8 |
   4804.9 |
   4811.0 |
   4817.1 |
   4823.1 |########################################
   4829.2 |
   4835.3 |
   4841.4 |
   4847.4 |
   4853.5 |
   4859.6 |
   4865.6 |########################################
   4871.7 |########################################
   4877.8 |
   4883.9 |
   4889.9 |
   4896.0 |########################################
   4902.1 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2023670.0-2057573.8 ns)
  2023670.0 |####################
  2025365.2 |########################################
  2027060.4 |
  2028755.6 |
  2030450.8 |
  2032145.9 |
  2033841.1 |
  2035536.3 |
  2037231.5 |
  2038926.7 |
  2040621.9 |
  2042317.1 |
  2044012.2 |
  2045707.4 |####################
  2047402.6 |
  2049097.8 |
  2050793.0 |
  2052488.2 |
  2054183.4 |####################
  2055878.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2025772.9-2057541.9 ns)
  2025772.9 |########################################
  2027361.3 |
  2028949.8 |####################
  2030538.2 |
  2032126.7 |
  2033715.1 |
  2035303.6 |
  2036892.0 |
  2038480.5 |
  2040068.9 |
  2041657.4 |
  2043245.8 |
  2044834.3 |
  2046422.8 |####################
  2048011.2 |
  2049599.6 |
  2051188.1 |
  2052776.5 |####################
  2054365.0 |
  2055953.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2024512.5-2057166.2 ns)
  2024512.5 |########################################
  2026145.2 |
  2027777.9 |
  2029410.6 |
  2031043.2 |
  2032675.9 |
  2034308.6 |
  2035941.3 |
  2037574.0 |
  2039206.7 |
  2040839.4 |####################
  2042472.1 |
  2044104.8 |
  2045737.4 |
  2047370.1 |####################
  2049002.8 |####################
  2050635.5 |
  2052268.2 |
  2053900.9 |
  2055533.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2023272.9-2052820.8 ns)
  2023272.9 |####################
  2024750.3 |
  2026227.7 |
  2027705.1 |
  2029182.5 |
  2030659.9 |####################
  2032137.3 |
  2033614.7 |
  2035092.1 |########################################
  2036569.5 |
  2038046.8 |
  2039524.2 |
  2041001.6 |
  2042479.0 |####################
  2043956.4 |
  2045433.8 |
  2046911.2 |
  2048388.6 |
  2049866.0 |
  2051343.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 2019034.6-2056129.1 ns)
  2019034.6 |########################################
  2020889.3 |
  2022744.1 |
  2024598.8 |
  2026453.5 |
  2028308.2 |
  2030163.0 |
  2032017.7 |
  2033872.4 |########################################
  2035727.1 |########################################
  2037581.9 |
  2039436.6 |
  2041291.3 |########################################
  2043146.1 |
  2045000.8 |
  2046855.5 |
  2048710.2 |
  2050565.0 |
  2052419.7 |
  2054274.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 2017379.6-2073077.4 ns)
  2017379.6 |####################
  2020164.5 |
  2022949.4 |
  2025734.3 |
  2028519.2 |
  2031304.1 |
  2034089.0 |
  2036873.8 |
  2039658.7 |
  2042443.6 |####################
  2045228.5 |####################
  2048013.4 |
  2050798.3 |
  2053583.2 |
  2056368.1 |
  2059153.0 |
  2061937.9 |
  2064722.8 |
  2067507.7 |########################################
  2070292.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 2023234.6-2050693.8 ns)
  2023234.6 |########################################
  2024607.6 |
  2025980.5 |
  2027353.5 |
  2028726.4 |
  2030099.4 |
  2031472.3 |####################
  2032845.3 |
  2034218.3 |
  2035591.2 |
  2036964.2 |
  2038337.1 |
  2039710.1 |
  2041083.0 |
  2042456.0 |
  2043829.0 |
  2045201.9 |
  2046574.9 |
  2047947.8 |########################################
  2049320.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2002367.9-2029923.4 ns)
  2002367.9 |####################
  2003745.7 |
  2005123.4 |
  2006501.2 |
  2007879.0 |
  2009256.8 |
  2010634.5 |
  2012012.3 |
  2013390.1 |
  2014767.9 |
  2016145.6 |
  2017523.4 |
  2018901.2 |
  2020278.9 |########################################
  2021656.7 |
  2023034.5 |
  2024412.3 |####################
  2025790.0 |####################
  2027167.8 |
  2028545.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=2536.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=315.0% of algo (FFI overhead may distort results)
