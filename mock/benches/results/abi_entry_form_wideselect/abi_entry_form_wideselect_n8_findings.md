# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_wideselect_null_entry dominates: 67489% faster than the next best (abi_entry_form_wideselect_runtime_w)

abi_entry_form_wideselect_null_entry (3.06 us) leads abi_entry_form_wideselect_runtime_w (2.07 ms) by 67489%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.07 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_scalar_anchor is an outlier: 679.0x slower than the field

abi_entry_form_wideselect_scalar_anchor (2.08 ms) is 679.0x the fastest (3.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.56)

abi_entry_form_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_scalar_anchor} (67489% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_scalar_anchor} with a 67489% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 679.0x the fastest

Fastest abi_entry_form_wideselect_null_entry (3.06 us) to slowest abi_entry_form_wideselect_scalar_anchor (2.08 ms): 679.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 3060.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 678.99x (fastest 3060.2 ns, slowest 2077854.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2075694ns | 2073495ns | 2059004ns | 2072452ns | 2088901ns | -0.53% |
| abi_entry_form_wideselect_null_entry | 5414ns | 5433ns | 5288ns | 5401ns | 5495ns | -99.74% |
| abi_entry_form_wideselect_per_w_set | 2165552ns | 2078532ns | 2064873ns | 2075355ns | 2351188ns | +3.78% |
| abi_entry_form_wideselect_runtime_w | 2086739ns | 2071213ns | 2068231ns | 2070241ns | 2120738ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2122176ns | 2081078ns | 2067464ns | 2077948ns | 2215875ns | +1.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2072515ns | 2056179ns | 2085529ns | -0.54% | 0.000 |
| abi_entry_form_wideselect_null_entry | 3053ns | 2999ns | 3077ns | -99.85% | 0.003 |
| abi_entry_form_wideselect_per_w_set | 2162138ns | 2061817ns | 2347136ns | +3.77% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2083670ns | 2065378ns | 2117228ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2118874ns | 2064566ns | 2212247ns | +1.69% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 62316.1 | 2073402.1 | 2072514.5 | 0 |
| abi_entry_form_wideselect_null_entry | 30257.7 | 3152.9 | 3053.2 | n/a |
| abi_entry_form_wideselect_per_w_set | 71665.1 | 2148954.6 | 2162138.4 | n/a |
| abi_entry_form_wideselect_runtime_w | 56778.9 | 2083022.2 | 2083669.5 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 65182.9 | 2130909.4 | 2118874.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_wideselect_null_entry | 0.003 | 98.0% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.1% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.1% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2075694ns | 2075694ns | -0.53% |
| abi_entry_form_wideselect_null_entry | 5414ns | 5414ns | -99.74% |
| abi_entry_form_wideselect_per_w_set | 2165552ns | 2165552ns | +3.78% |
| abi_entry_form_wideselect_runtime_w | 2086739ns | 2086739ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2122176ns | 2122176ns | +1.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2068353ns | base | --- | [2065427, 2117228] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2070333ns | no significant difference | [-48223, +18665]ns | [2061682, 2085529] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_wideselect_null_entry | 3060ns | -2065292.9ns (-99.9%) | [-2114172, -2062384]ns | [3023, 3077] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2075384ns | no significant difference | [-48476, +281524]ns | [2063895, 2347136] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2077855ns | no significant difference | [-16165, +118077]ns | [2066521, 2212247] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2068456ns | +0.1% | -99.9% | +0.2% | -0.2% |
| 2 | 2065378ns | -0.4% | -99.9% | +0.6% | +0.2% |
| 3 | 2065477ns | +1.3% | -99.9% | +26.6% | +11.2% |
| 4 | 2068250ns | +0.5% | -99.9% | -0.3% | +0.2% |
| 5 | 2077965ns | -0.5% | -99.9% | +0.0% | +0.2% |
| 6 | 2156491ns | -4.0% | -99.9% | -4.2% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.237 | moderate- |
| abi_entry_form_wideselect_null_entry | -0.556 | HIGH- (thermal bounce) |
| abi_entry_form_wideselect_per_w_set | -0.241 | moderate- |
| abi_entry_form_wideselect_runtime_w | 0.087 | ok |
| abi_entry_form_wideselect_scalar_anchor | -0.323 | moderate- |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 3/6, lost 2/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_wideselect_scalar_anchor**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6286452.6ns | 2072514.5ns | 303.3% | HIGH |
| abi_entry_form_wideselect_null_entry | 121463.8ns | 3053.2ns | 3978.2% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6539079.9ns | 2162138.4ns | 302.4% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6310903.0ns | 2083669.5ns | 302.9% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6417177.5ns | 2118874.3ns | 302.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2056179.2-2085528.6 ns)
  2056179.2 |####################
  2057646.7 |
  2059114.1 |
  2060581.6 |
  2062049.1 |
  2063516.5 |
  2064984.0 |
  2066451.5 |####################
  2067918.9 |
  2069386.4 |########################################
  2070853.9 |
  2072321.3 |
  2073788.8 |
  2075256.3 |
  2076723.7 |
  2078191.2 |####################
  2079658.7 |
  2081126.1 |
  2082593.6 |
  2084061.1 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 2998.8-3076.6 ns)
   2998.8 |########################################
   3002.7 |
   3006.6 |
   3010.5 |
   3014.4 |
   3018.3 |
   3022.2 |
   3026.0 |
   3029.9 |
   3033.8 |
   3037.7 |
   3041.6 |
   3045.5 |########################################
   3049.4 |
   3053.3 |########################################
   3057.2 |
   3061.1 |########################################
   3065.0 |########################################
   3068.9 |
   3072.8 |
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2061817.1-2347136.2 ns)
  2061817.1 |########################################
  2076083.1 |##########################
  2090349.0 |
  2104615.0 |
  2118880.9 |
  2133146.9 |
  2147412.8 |
  2161678.8 |
  2175944.8 |
  2190210.7 |
  2204476.7 |
  2218742.6 |
  2233008.6 |
  2247274.5 |
  2261540.5 |
  2275806.5 |
  2290072.4 |
  2304338.4 |
  2318604.3 |
  2332870.3 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2065377.5-2117228.1 ns)
  2065377.5 |########################################
  2067970.0 |########################################
  2070562.6 |
  2073155.1 |
  2075747.6 |####################
  2078340.1 |
  2080932.7 |
  2083525.2 |
  2086117.7 |
  2088710.3 |
  2091302.8 |
  2093895.3 |
  2096487.9 |
  2099080.4 |
  2101672.9 |
  2104265.5 |
  2106858.0 |
  2109450.5 |
  2112043.0 |
  2114635.6 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2064565.8-2212246.8 ns)
  2064565.8 |########################################
  2071949.9 |####################
  2079333.9 |####################
  2086718.0 |
  2094102.0 |
  2101486.1 |
  2108870.1 |
  2116254.2 |
  2123638.2 |####################
  2131022.3 |
  2138406.3 |
  2145790.4 |
  2153174.4 |
  2160558.5 |
  2167942.5 |
  2175326.6 |
  2182710.6 |
  2190094.7 |
  2197478.7 |
  2204862.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=3962.9% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)
