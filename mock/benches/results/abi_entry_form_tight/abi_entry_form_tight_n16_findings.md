# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 79666% faster than the next best (abi_entry_form_tight_per_w_set)

abi_entry_form_tight_null_entry (2.53 us) leads abi_entry_form_tight_per_w_set (2.02 ms) by 79666%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.02 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 804.4x slower than the field

abi_entry_form_tight_scalar_anchor (2.03 ms) is 804.4x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_scalar_anchor} (79666% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_scalar_anchor} with a 79666% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 804.4x the fastest

Fastest abi_entry_form_tight_null_entry (2.53 us) to slowest abi_entry_form_tight_scalar_anchor (2.03 ms): 804.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 2529.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 804.41x (fastest 2529.4 ns, slowest 2034631.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2040202ns | 2024066ns | 2018526ns | 2022768ns | 2077190ns | +0.56% |
| abi_entry_form_tight_null_entry | 4911ns | 4925ns | 4708ns | 4865ns | 5082ns | -99.76% |
| abi_entry_form_tight_per_w_set | 2031320ns | 2020380ns | 2015319ns | 2020093ns | 2056161ns | +0.12% |
| abi_entry_form_tight_runtime_w | 2028921ns | 2027375ns | 2008209ns | 2023977ns | 2046693ns | base |
| abi_entry_form_tight_scalar_anchor | 2126500ns | 2037595ns | 2034918ns | 2036812ns | 2306823ns | +4.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2036964ns | 2015763ns | 2073557ns | +0.55% | 0.000 |
| abi_entry_form_tight_null_entry | 2540ns | 2475ns | 2608ns | -99.87% | 0.006 |
| abi_entry_form_tight_per_w_set | 2028344ns | 2012651ns | 2053031ns | +0.12% | 0.000 |
| abi_entry_form_tight_runtime_w | 2025896ns | 2005560ns | 2043580ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2123420ns | 2032130ns | 2303460ns | +4.81% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 57669.3 | 2034900.9 | 2036963.5 | n/a |
| abi_entry_form_tight_null_entry | 27271.3 | 2641.5 | 2540.0 | n/a |
| abi_entry_form_tight_per_w_set | 77188.8 | 2034049.7 | 2028344.2 | n/a |
| abi_entry_form_tight_runtime_w | 53268.7 | 2025274.1 | 2025896.2 | n/a |
| abi_entry_form_tight_scalar_anchor | 58592.0 | 2037539.8 | 2123420.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_tight_null_entry | 0.006 | 97.9% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.1% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.1% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2040202ns | 2040202ns | +0.56% |
| abi_entry_form_tight_null_entry | 4911ns | 4911ns | -99.76% |
| abi_entry_form_tight_per_w_set | 2031320ns | 2031320ns | +0.12% |
| abi_entry_form_tight_runtime_w | 2028921ns | 2028921ns | base |
| abi_entry_form_tight_scalar_anchor | 2126500ns | 2126500ns | +4.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2024222ns | base | --- | [2009886, 2043580] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2020925ns | no significant difference | [-12609, +44818]ns | [2016409, 2073557] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_null_entry | 2529ns | -2021614.4ns (-99.9%) | [-2041094, -2007361]ns | [2483, 2608] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2017551ns | no significant difference | [-23882, +30710]ns | [2014451, 2053031] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_scalar_anchor | 2034632ns | no significant difference | [-7219, +281433]ns | [2032169, 2303460] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2014213ns | +0.1% | -99.9% | +0.1% | +0.9% |
| 2 | 2005560ns | +0.5% | -99.9% | +0.6% | +1.5% |
| 3 | 2029415ns | -0.3% | -99.9% | -0.5% | +0.2% |
| 4 | 2036971ns | +3.9% | -99.9% | +2.4% | +26.1% |
| 5 | 2050190ns | -1.0% | -99.9% | -1.8% | -0.9% |
| 6 | 2019028ns | -0.0% | -99.9% | -0.0% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | -0.095 | ok |
| abi_entry_form_tight_null_entry | -0.264 | moderate- |
| abi_entry_form_tight_per_w_set | -0.255 | moderate- |
| abi_entry_form_tight_runtime_w | 0.232 | moderate+ |
| abi_entry_form_tight_scalar_anchor | -0.240 | moderate- |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 2/6, lost 3/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_tight_scalar_anchor**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6165533.3ns | 2036963.5ns | 302.7% | HIGH |
| abi_entry_form_tight_null_entry | 117844.4ns | 2540.0ns | 4639.6% | HIGH |
| abi_entry_form_tight_per_w_set | 6175357.4ns | 2028344.2ns | 304.5% | HIGH |
| abi_entry_form_tight_runtime_w | 6126866.1ns | 2025896.2ns | 302.4% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6181818.1ns | 2123420.5ns | 291.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2015762.9-2073556.9 ns)
  2015762.9 |########################################
  2018652.6 |
  2021542.3 |#############
  2024432.0 |
  2027321.7 |
  2030211.4 |#############
  2033101.1 |
  2035990.8 |
  2038880.5 |
  2041770.2 |
  2044659.9 |
  2047549.6 |
  2050439.3 |
  2053329.0 |
  2056218.7 |
  2059108.4 |
  2061998.1 |
  2064887.8 |
  2067777.5 |
  2070667.2 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 2475.0-2607.5 ns)
   2475.0 |########################################
   2481.6 |
   2488.2 |########################################
   2494.9 |########################################
   2501.5 |
   2508.1 |
   2514.8 |
   2521.4 |
   2528.0 |
   2534.6 |
   2541.2 |
   2547.9 |
   2554.5 |########################################
   2561.1 |
   2567.8 |
   2574.4 |
   2581.0 |
   2587.6 |########################################
   2594.2 |
   2600.9 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2012650.8-2053030.6 ns)
  2012650.8 |####################
  2014669.8 |####################
  2016688.8 |########################################
  2018707.8 |####################
  2020726.8 |
  2022745.8 |
  2024764.7 |
  2026783.7 |
  2028802.7 |
  2030821.7 |
  2032840.7 |
  2034859.7 |
  2036878.7 |
  2038897.7 |
  2040916.7 |
  2042935.7 |
  2044954.6 |
  2046973.6 |
  2048992.6 |
  2051011.6 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2005560.0-2043580.2 ns)
  2005560.0 |########################################
  2007461.0 |
  2009362.0 |
  2011263.0 |
  2013164.0 |########################################
  2015065.1 |
  2016966.1 |
  2018867.1 |########################################
  2020768.1 |
  2022669.1 |
  2024570.1 |
  2026471.1 |
  2028372.1 |########################################
  2030273.1 |
  2032174.1 |
  2034075.2 |
  2035976.2 |########################################
  2037877.2 |
  2039778.2 |
  2041679.2 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2032129.6-2303460.2 ns)
  2032129.6 |########################################
  2045696.1 |
  2059262.7 |
  2072829.2 |
  2086395.7 |
  2099962.2 |
  2113528.8 |
  2127095.3 |
  2140661.8 |
  2154228.4 |
  2167794.9 |
  2181361.4 |
  2194928.0 |
  2208494.5 |
  2222061.0 |
  2235627.6 |
  2249194.1 |
  2262760.6 |
  2276327.1 |
  2289893.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=4664.0% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.0% of algo (FFI overhead may distort results)
