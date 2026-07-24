# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 66937% faster than the next best (abi_entry_form_tight_runtime_w)

abi_entry_form_tight_null_entry (3.02 us) leads abi_entry_form_tight_runtime_w (2.02 ms) by 66937%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.02 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 675.6x slower than the field

abi_entry_form_tight_scalar_anchor (2.04 ms) is 675.6x the fastest (3.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_scalar_anchor} (66937% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_scalar_anchor} with a 66937% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 675.6x the fastest

Fastest abi_entry_form_tight_null_entry (3.02 us) to slowest abi_entry_form_tight_scalar_anchor (2.04 ms): 675.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 3015.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 675.63x (fastest 3015.9 ns, slowest 2037605.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2057238ns | 2040219ns | 2020595ns | 2034850ns | 2109139ns | -3.88% |
| abi_entry_form_tight_null_entry | 5288ns | 5260ns | 5121ns | 5221ns | 5473ns | -99.75% |
| abi_entry_form_tight_per_w_set | 2074949ns | 2026412ns | 2020563ns | 2024917ns | 2177190ns | -3.06% |
| abi_entry_form_tight_runtime_w | 2140374ns | 2024908ns | 2016888ns | 2022574ns | 2378816ns | base |
| abi_entry_form_tight_scalar_anchor | 2099570ns | 2040704ns | 2035364ns | 2039359ns | 2221990ns | -1.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2053821ns | 2017611ns | 2105331ns | -3.88% | 0.000 |
| abi_entry_form_tight_null_entry | 3018ns | 2923ns | 3098ns | -99.86% | 0.003 |
| abi_entry_form_tight_per_w_set | 2071590ns | 2017662ns | 2173348ns | -3.05% | 0.000 |
| abi_entry_form_tight_runtime_w | 2136803ns | 2013763ns | 2374328ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2096290ns | 2032319ns | 2218226ns | -1.90% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 67284.7 | 2046291.9 | 2053820.8 | n/a |
| abi_entry_form_tight_null_entry | 27684.1 | 3099.1 | 3018.3 | n/a |
| abi_entry_form_tight_per_w_set | 66484.6 | 2078323.7 | 2071589.6 | n/a |
| abi_entry_form_tight_runtime_w | 69592.0 | 2080311.8 | 2136802.7 | n/a |
| abi_entry_form_tight_scalar_anchor | 63059.4 | 2062560.8 | 2096289.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_tight_null_entry | 0.003 | 96.9% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.1% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.1% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2057238ns | 2057238ns | -3.88% |
| abi_entry_form_tight_null_entry | 5288ns | 5288ns | -99.75% |
| abi_entry_form_tight_per_w_set | 2074949ns | 2074949ns | -3.06% |
| abi_entry_form_tight_runtime_w | 2140374ns | 2140374ns | base |
| abi_entry_form_tight_scalar_anchor | 2099570ns | 2099570ns | -1.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2021733ns | base | --- | [2014347, 2374328] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2036776ns | no significant difference | [-321961, +67953]ns | [2019355, 2105331] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_tight_null_entry | 3016ns | -2018721.9ns (-99.9%) | [-2371305, -2011326]ns | [2941, 3098] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2023228ns | no significant difference | [-352410, +153743]ns | [2018193, 2173348] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_tight_scalar_anchor | 2037605ns | no significant difference | [-326938, +187062]ns | [2033038, 2218226] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2042401ns | +0.4% | -99.9% | -1.2% | +0.7% |
| 2 | 2025447ns | +6.3% | -99.8% | +14.3% | +17.5% |
| 3 | 2706256ns | -23.9% | -99.9% | -25.2% | -24.7% |
| 4 | 2014930ns | +0.3% | -99.8% | +0.1% | +0.9% |
| 5 | 2018020ns | +0.2% | -99.9% | +0.2% | +1.0% |
| 6 | 2013763ns | +0.2% | -99.9% | +0.9% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.174 | ok |
| abi_entry_form_tight_null_entry | 0.198 | ok |
| abi_entry_form_tight_per_w_set | -0.238 | moderate- |
| abi_entry_form_tight_runtime_w | -0.239 | moderate- |
| abi_entry_form_tight_scalar_anchor | -0.171 | ok |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 1/6, lost 5/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 2/6, lost 4/6
- **abi_entry_form_tight_scalar_anchor**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6220155.6ns | 2053820.8ns | 302.9% | HIGH |
| abi_entry_form_tight_null_entry | 119961.2ns | 3018.3ns | 3974.5% | HIGH |
| abi_entry_form_tight_per_w_set | 6312124.2ns | 2071589.6ns | 304.7% | HIGH |
| abi_entry_form_tight_runtime_w | 6357050.2ns | 2136802.7ns | 297.5% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6269636.9ns | 2096289.9ns | 299.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2017611.2-2105331.5 ns)
  2017611.2 |########################################
  2021997.2 |
  2026383.2 |
  2030769.2 |
  2035155.2 |
  2039541.3 |
  2043927.3 |
  2048313.3 |#############
  2052699.3 |
  2057085.3 |#############
  2061471.3 |
  2065857.3 |
  2070243.4 |
  2074629.4 |
  2079015.4 |
  2083401.4 |
  2087787.4 |
  2092173.4 |
  2096559.4 |
  2100945.4 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 2922.9-3097.7 ns)
   2922.9 |########################################
   2931.6 |
   2940.4 |
   2949.1 |
   2957.9 |########################################
   2966.6 |########################################
   2975.3 |
   2984.1 |
   2992.8 |
   3001.6 |
   3010.3 |
   3019.0 |
   3027.8 |
   3036.5 |
   3045.3 |
   3054.0 |
   3062.7 |########################################
   3071.5 |########################################
   3080.2 |
   3089.0 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2017662.5-2173347.9 ns)
  2017662.5 |########################################
  2025446.8 |##########
  2033231.0 |
  2041015.3 |
  2048799.6 |
  2056583.9 |
  2064368.1 |
  2072152.4 |
  2079936.7 |
  2087720.9 |
  2095505.2 |
  2103289.5 |
  2111073.7 |
  2118858.0 |
  2126642.3 |
  2134426.6 |
  2142210.8 |
  2149995.1 |
  2157779.4 |
  2165563.6 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2013763.3-2374328.3 ns)
  2013763.3 |########################################
  2031791.6 |##########
  2049819.8 |
  2067848.1 |
  2085876.3 |
  2103904.5 |
  2121932.8 |
  2139961.0 |
  2157989.3 |
  2176017.5 |
  2194045.8 |
  2212074.0 |
  2230102.3 |
  2248130.5 |
  2266158.8 |
  2284187.0 |
  2302215.3 |
  2320243.5 |
  2338271.8 |
  2356300.0 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2032319.2-2218226.5 ns)
  2032319.2 |########################################
  2041614.6 |
  2050909.9 |##########
  2060205.3 |
  2069500.6 |
  2078796.0 |
  2088091.4 |
  2097386.7 |
  2106682.1 |
  2115977.5 |
  2125272.8 |
  2134568.2 |
  2143863.6 |
  2153158.9 |
  2162454.3 |
  2171749.6 |
  2181045.0 |
  2190340.4 |
  2199635.7 |
  2208931.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=3954.1% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.1% of algo (FFI overhead may distort results)
