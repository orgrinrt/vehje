# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_scatter_null_entry dominates: 94693% faster than the next best (abi_entry_form_scatter_per_w_set)

abi_entry_form_scatter_null_entry (2.29 us) leads abi_entry_form_scatter_per_w_set (2.17 ms) by 94693%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_scalar_anchor is an outlier: 951.3x slower than the field

abi_entry_form_scatter_scalar_anchor (2.18 ms) is 951.3x the fastest (2.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_scatter_runtime_w shows alternating (throttle bounce) (autocorr -0.54)

abi_entry_form_scatter_runtime_w's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_scalar_anchor} (94693% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_scalar_anchor} with a 94693% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 951.3x the fastest

Fastest abi_entry_form_scatter_null_entry (2.29 us) to slowest abi_entry_form_scatter_scalar_anchor (2.18 ms): 951.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 2291.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 951.27x (fastest 2291.1 ns, slowest 2179416.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2178671ns | 2178904ns | 2173262ns | 2177581ns | 2183010ns | +0.04% |
| abi_entry_form_scatter_null_entry | 4619ns | 4634ns | 4519ns | 4597ns | 4702ns | -99.79% |
| abi_entry_form_scatter_per_w_set | 2195650ns | 2175169ns | 2169498ns | 2173356ns | 2242166ns | +0.82% |
| abi_entry_form_scatter_runtime_w | 2177708ns | 2175246ns | 2164552ns | 2173778ns | 2190183ns | base |
| abi_entry_form_scatter_scalar_anchor | 2190475ns | 2182890ns | 2176242ns | 2182424ns | 2209668ns | +0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2175129ns | 2169757ns | 2179392ns | +0.04% | 0.000 |
| abi_entry_form_scatter_null_entry | 2287ns | 2231ns | 2324ns | -99.89% | 0.014 |
| abi_entry_form_scatter_per_w_set | 2192208ns | 2166450ns | 2238379ns | +0.82% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2174296ns | 2161080ns | 2186638ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2186985ns | 2173009ns | 2206082ns | +0.58% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 69335.6 | 2176016.2 | 2175129.2 | n/a |
| abi_entry_form_scatter_null_entry | 26977.4 | 2410.2 | 2286.7 | n/a |
| abi_entry_form_scatter_per_w_set | 69956.7 | 2192714.0 | 2192207.9 | n/a |
| abi_entry_form_scatter_runtime_w | 65800.6 | 2174158.7 | 2174296.0 | n/a |
| abi_entry_form_scatter_scalar_anchor | 69129.3 | 2184294.9 | 2186984.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_scatter_null_entry | 0.014 | 97.4% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.1% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.1% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2178671ns | 2178671ns | +0.04% |
| abi_entry_form_scatter_null_entry | 4619ns | 4619ns | -99.79% |
| abi_entry_form_scatter_per_w_set | 2195650ns | 2195650ns | +0.82% |
| abi_entry_form_scatter_runtime_w | 2177708ns | 2177708ns | base |
| abi_entry_form_scatter_scalar_anchor | 2190475ns | 2190475ns | +0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2171875ns | base | --- | [2164374, 2186638] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2175303ns | no significant difference | [-14128, +11674]ns | [2170692, 2179392] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_scatter_null_entry | 2291ns | -2169610.0ns (-99.9%) | [-2184338, -2162079]ns | [2245, 2324] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2171750ns | no significant difference | [-5783, +55487]ns | [2166495, 2238379] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2179416ns | +10885.2ns (+0.5%) | [+697, +26484]ns | [2175456, 2206082] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2161080ns | +0.8% | -99.9% | +0.3% | +1.1% |
| 2 | 2198116ns | -1.0% | -99.9% | +4.7% | +1.3% |
| 3 | 2169395ns | +0.3% | -99.9% | -0.1% | +0.4% |
| 4 | 2174356ns | +0.3% | -99.9% | +0.0% | +0.2% |
| 5 | 2175161ns | -0.2% | -99.9% | -0.4% | -0.1% |
| 6 | 2167668ns | +0.2% | -99.9% | +0.4% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.124 | ok |
| abi_entry_form_scatter_null_entry | 0.201 | moderate+ |
| abi_entry_form_scatter_per_w_set | -0.291 | moderate- |
| abi_entry_form_scatter_runtime_w | -0.540 | HIGH- (thermal bounce) |
| abi_entry_form_scatter_scalar_anchor | -0.059 | ok |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 2/6, lost 4/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_scatter_scalar_anchor**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6599370.5ns | 2175129.2ns | 303.4% | HIGH |
| abi_entry_form_scatter_null_entry | 115859.3ns | 2286.7ns | 5066.6% | HIGH |
| abi_entry_form_scatter_per_w_set | 6686941.9ns | 2192207.9ns | 305.0% | HIGH |
| abi_entry_form_scatter_runtime_w | 6591154.2ns | 2174296.0ns | 303.1% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6633577.9ns | 2186984.6ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2169757.1-2179391.9 ns)
  2169757.1 |####################
  2170238.8 |
  2170720.6 |
  2171202.3 |####################
  2171684.1 |
  2172165.8 |
  2172647.5 |
  2173129.3 |
  2173611.0 |
  2174092.8 |
  2174574.5 |
  2175056.2 |########################################
  2175538.0 |
  2176019.7 |
  2176501.5 |
  2176983.2 |
  2177464.9 |
  2177946.7 |####################
  2178428.4 |
  2178910.2 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 2231.2-2324.3 ns)
   2231.2 |########################################
   2235.9 |
   2240.5 |
   2245.2 |
   2249.8 |
   2254.5 |########################################
   2259.1 |
   2263.8 |
   2268.5 |
   2273.1 |
   2277.8 |
   2282.4 |########################################
   2287.1 |
   2291.7 |
   2296.4 |########################################
   2301.1 |
   2305.7 |
   2310.4 |
   2315.0 |########################################
   2319.7 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2166450.0-2238379.0 ns)
  2166450.0 |########################################
  2170046.4 |
  2173642.9 |##########################
  2177239.3 |
  2180835.8 |
  2184432.2 |
  2188028.7 |
  2191625.1 |
  2195221.6 |
  2198818.0 |
  2202414.5 |
  2206010.9 |
  2209607.4 |
  2213203.8 |
  2216800.3 |
  2220396.7 |
  2223993.2 |
  2227589.6 |
  2231186.1 |
  2234782.5 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2161080.4-2186638.3 ns)
  2161080.4 |########################################
  2162358.3 |
  2163636.2 |
  2164914.1 |
  2166192.0 |
  2167469.9 |########################################
  2168747.8 |########################################
  2170025.7 |
  2171303.6 |
  2172581.5 |
  2173859.3 |########################################
  2175137.2 |########################################
  2176415.1 |
  2177693.0 |
  2178970.9 |
  2180248.8 |
  2181526.7 |
  2182804.6 |
  2184082.5 |
  2185360.4 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2173008.8-2206081.7 ns)
  2173008.8 |########################################
  2174662.4 |
  2176316.1 |########################################
  2177969.7 |########################################
  2179623.4 |########################################
  2181277.0 |
  2182930.7 |
  2184584.3 |########################################
  2186238.0 |
  2187891.6 |
  2189545.2 |
  2191198.9 |
  2192852.5 |
  2194506.2 |
  2196159.8 |
  2197813.5 |
  2199467.1 |
  2201120.8 |
  2202774.4 |
  2204428.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=5064.6% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)
