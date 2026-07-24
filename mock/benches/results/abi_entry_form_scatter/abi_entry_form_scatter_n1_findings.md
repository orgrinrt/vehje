# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_scatter_null_entry dominates: 43968% faster than the next best (abi_entry_form_scatter_scalar_anchor)

abi_entry_form_scatter_null_entry (4.94 us) leads abi_entry_form_scatter_scalar_anchor (2.18 ms) by 43968%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.22 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_dispatch_table is an outlier: 454.9x slower than the field

abi_entry_form_scatter_dispatch_table (2.25 ms) is 454.9x the fastest (4.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_scatter_null_entry shows alternating (throttle bounce) (autocorr -0.64)

abi_entry_form_scatter_null_entry's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_dispatch_table} (43968% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_dispatch_table} with a 43968% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 454.9x the fastest

Fastest abi_entry_form_scatter_null_entry (4.94 us) to slowest abi_entry_form_scatter_dispatch_table (2.25 ms): 454.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 4937.5 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 454.90x (fastest 4937.5 ns, slowest 2246062.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2338662ns | 2250121ns | 2170789ns | 2226248ns | 2591220ns | +4.74% |
| abi_entry_form_scatter_null_entry | 7365ns | 7260ns | 7180ns | 7245ns | 7637ns | -99.67% |
| abi_entry_form_scatter_per_w_set | 2222648ns | 2216133ns | 2173688ns | 2202594ns | 2277208ns | -0.46% |
| abi_entry_form_scatter_runtime_w | 2232931ns | 2232482ns | 2177126ns | 2219333ns | 2281231ns | base |
| abi_entry_form_scatter_scalar_anchor | 2183949ns | 2179542ns | 2166298ns | 2177868ns | 2201896ns | -2.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2334682ns | 2167582ns | 2586646ns | +4.74% | 0.000 |
| abi_entry_form_scatter_null_entry | 4977ns | 4837ns | 5118ns | -99.78% | 0.000 |
| abi_entry_form_scatter_per_w_set | 2218883ns | 2170490ns | 2273147ns | -0.46% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2229059ns | 2173968ns | 2276820ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2180469ns | 2163312ns | 2198289ns | -2.18% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 80420.7 | 2320764.2 | 2334682.3 | n/a |
| abi_entry_form_scatter_null_entry | 28771.6 | 5046.7 | 4977.2 | n/a |
| abi_entry_form_scatter_per_w_set | 73304.4 | 2218461.9 | 2218882.8 | n/a |
| abi_entry_form_scatter_runtime_w | 74245.1 | 2232533.6 | 2229058.7 | n/a |
| abi_entry_form_scatter_scalar_anchor | 68304.1 | 2180119.2 | 2180468.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_scatter_null_entry | 0.000 | 98.0% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.2% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.2% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2338662ns | 2338662ns | +4.74% |
| abi_entry_form_scatter_null_entry | 7365ns | 7365ns | -99.67% |
| abi_entry_form_scatter_per_w_set | 2222648ns | 2222648ns | -0.46% |
| abi_entry_form_scatter_runtime_w | 2232931ns | 2232931ns | base |
| abi_entry_form_scatter_scalar_anchor | 2183949ns | 2183949ns | -2.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2228685ns | base | --- | [2181671, 2276820] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2246062ns | no significant difference | [-61897, +327792]ns | [2171339, 2586646] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_scatter_null_entry | 4938ns | -2223668.2ns (-99.8%) | [-2271842, -2176734]ns | [4876, 5118] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2212025ns | no significant difference | [-75175, +44898]ns | [2171476, 2273147] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2175871ns | -43793.7ns (-2.0%) | [-88551, -13425]ns | [2167247, 2198289] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2282408ns | -5.0% | -99.8% | -4.9% | -4.9% |
| 2 | 2189375ns | -0.4% | -99.8% | +0.3% | -0.7% |
| 3 | 2271231ns | +13.5% | -99.8% | -0.3% | -2.4% |
| 4 | 2246476ns | +15.5% | -99.8% | +1.6% | -2.9% |
| 5 | 2210894ns | +4.6% | -99.8% | -1.7% | -1.5% |
| 6 | 2173968ns | +0.1% | -99.8% | +2.5% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.241 | moderate+ |
| abi_entry_form_scatter_null_entry | -0.640 | HIGH- (thermal bounce) |
| abi_entry_form_scatter_per_w_set | -0.038 | ok |
| abi_entry_form_scatter_runtime_w | -0.240 | moderate- |
| abi_entry_form_scatter_scalar_anchor | -0.091 | ok |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 2/6, lost 3/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_scatter_scalar_anchor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 7091636.3ns | 2334682.3ns | 303.8% | HIGH |
| abi_entry_form_scatter_null_entry | 125455.5ns | 4977.2ns | 2520.6% | HIGH |
| abi_entry_form_scatter_per_w_set | 6722259.7ns | 2218882.8ns | 303.0% | HIGH |
| abi_entry_form_scatter_runtime_w | 6769385.6ns | 2229058.7ns | 303.7% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6610398.7ns | 2180468.8ns | 303.2% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2167581.7-2586646.0 ns)
  2167581.7 |########################################
  2188534.9 |
  2209488.1 |
  2230441.4 |
  2251394.6 |
  2272347.8 |
  2293301.0 |#############
  2314254.2 |
  2335207.4 |
  2356160.7 |
  2377113.9 |
  2398067.1 |
  2419020.3 |
  2439973.5 |
  2460926.7 |
  2481880.0 |
  2502833.2 |
  2523786.4 |
  2544739.6 |
  2565692.8 |#############
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 4836.7-5118.1 ns)
   4836.7 |########################################
   4850.8 |
   4864.8 |
   4878.9 |
   4893.0 |
   4907.1 |########################################
   4921.1 |########################################
   4935.2 |########################################
   4949.3 |
   4963.4 |
   4977.4 |
   4991.5 |
   5005.6 |
   5019.6 |
   5033.7 |########################################
   5047.8 |
   5061.9 |
   5075.9 |
   5090.0 |
   5104.1 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2170489.6-2273147.0 ns)
  2170489.6 |########################################
  2175622.5 |
  2180755.3 |
  2185888.2 |
  2191021.1 |
  2196154.0 |####################
  2201286.8 |
  2206419.7 |
  2211552.6 |
  2216685.5 |
  2221818.3 |
  2226951.2 |####################
  2232084.1 |
  2237216.9 |
  2242349.8 |
  2247482.7 |
  2252615.6 |
  2257748.4 |
  2262881.3 |####################
  2268014.2 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2173967.9-2276819.8 ns)
  2173967.9 |########################################
  2179110.5 |
  2184253.1 |########################################
  2189395.7 |
  2194538.3 |
  2199680.9 |
  2204823.5 |
  2209966.0 |########################################
  2215108.6 |
  2220251.2 |
  2225393.8 |
  2230536.4 |
  2235679.0 |
  2240821.6 |
  2245964.2 |########################################
  2251106.8 |
  2256249.4 |
  2261392.0 |
  2266534.6 |########################################
  2271677.2 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2163311.7-2198288.5 ns)
  2163311.7 |########################################
  2165060.5 |
  2166809.4 |
  2168558.2 |
  2170307.1 |########################################
  2172055.9 |########################################
  2173804.8 |
  2175553.6 |
  2177302.4 |########################################
  2179051.3 |########################################
  2180800.1 |
  2182549.0 |
  2184297.8 |
  2186046.7 |
  2187795.5 |
  2189544.3 |
  2191293.2 |
  2193042.0 |
  2194790.9 |
  2196539.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=2528.8% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.6% of algo (FFI overhead may distort results)
