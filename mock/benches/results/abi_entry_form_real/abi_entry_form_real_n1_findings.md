# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_real_null_entry dominates: 45010% faster than the next best (abi_entry_form_real_runtime_w)

abi_entry_form_real_null_entry (4.89 us) leads abi_entry_form_real_runtime_w (2.21 ms) by 45010%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.20 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_scalar_anchor is an outlier: 485.8x slower than the field

abi_entry_form_real_scalar_anchor (2.37 ms) is 485.8x the fastest (4.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_real_per_w_set shows alternating (throttle bounce) (autocorr -0.53)

abi_entry_form_real_per_w_set's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_runtime_w, abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor} (45010% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_runtime_w, abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor} with a 45010% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 485.8x the fastest

Fastest abi_entry_form_real_null_entry (4.89 us) to slowest abi_entry_form_real_scalar_anchor (2.37 ms): 485.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 4888.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 485.77x (fastest 4888.1 ns, slowest 2374478.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2298596ns | 2283181ns | 2184791ns | 2259727ns | 2413802ns | +3.48% |
| abi_entry_form_real_null_entry | 7234ns | 7148ns | 6915ns | 7111ns | 7577ns | -99.67% |
| abi_entry_form_real_per_w_set | 2284349ns | 2258431ns | 2193622ns | 2237575ns | 2399873ns | +2.84% |
| abi_entry_form_real_runtime_w | 2221220ns | 2208782ns | 2198741ns | 2207758ns | 2252652ns | base |
| abi_entry_form_real_scalar_anchor | 2493832ns | 2378782ns | 2193751ns | 2319100ns | 2905970ns | +12.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2294629ns | 2181508ns | 2409631ns | +3.48% | 0.000 |
| abi_entry_form_real_null_entry | 4912ns | 4747ns | 5091ns | -99.78% | 0.000 |
| abi_entry_form_real_per_w_set | 2280341ns | 2189997ns | 2395637ns | +2.84% | 0.000 |
| abi_entry_form_real_runtime_w | 2217369ns | 2194810ns | 2248655ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2489434ns | 2190118ns | 2900683ns | +12.27% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 83169.2 | 2287466.0 | 2294629.1 | n/a |
| abi_entry_form_real_null_entry | 28656.4 | 5016.6 | 4911.8 | n/a |
| abi_entry_form_real_per_w_set | 91163.3 | 2272577.2 | 2280341.4 | n/a |
| abi_entry_form_real_runtime_w | 76886.9 | 2212920.4 | 2217369.3 | n/a |
| abi_entry_form_real_scalar_anchor | 96142.7 | 2533047.2 | 2489433.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_real_null_entry | 0.000 | 97.1% |
| abi_entry_form_real_per_w_set | 0.000 | 0.2% |
| abi_entry_form_real_runtime_w | 0.000 | 0.2% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2298596ns | 2298596ns | +3.48% |
| abi_entry_form_real_null_entry | 7234ns | 7234ns | -99.67% |
| abi_entry_form_real_per_w_set | 2284349ns | 2284349ns | +2.84% |
| abi_entry_form_real_runtime_w | 2221220ns | 2221220ns | base |
| abi_entry_form_real_scalar_anchor | 2493832ns | 2493832ns | +12.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2205045ns | base | --- | [2198408, 2248655] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2278791ns | no significant difference | [-7532, +162618]ns | [2195466, 2409631] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_real_null_entry | 4888ns | -2200103.4ns (-99.8%) | [-2243675, -2193594]ns | [4756, 5091] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2254376ns | no significant difference | [-7396, +157618]ns | [2191012, 2395637] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_real_scalar_anchor | 2374478ns | no significant difference | [-5268, +692941]ns | [2193140, 2900683] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2209384ns | +1.9% | -99.8% | +4.0% | +19.9% |
| 2 | 2287926ns | +6.4% | -99.8% | +3.1% | +4.7% |
| 3 | 2194810ns | +5.1% | -99.8% | -0.1% | +0.1% |
| 4 | 2206099ns | +8.1% | -99.8% | +10.3% | +42.9% |
| 5 | 2202005ns | -0.9% | -99.8% | -0.5% | -0.5% |
| 6 | 2203990ns | +0.2% | -99.8% | +0.3% | +6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.077 | ok |
| abi_entry_form_real_null_entry | -0.157 | ok |
| abi_entry_form_real_per_w_set | -0.528 | HIGH- (thermal bounce) |
| abi_entry_form_real_runtime_w | -0.250 | moderate- |
| abi_entry_form_real_scalar_anchor | -0.509 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 1/6, lost 5/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 2/6, lost 4/6
- **abi_entry_form_real_scalar_anchor**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6981040.2ns | 2294629.1ns | 304.2% | HIGH |
| abi_entry_form_real_null_entry | 125404.9ns | 4911.8ns | 2553.1% | HIGH |
| abi_entry_form_real_per_w_set | 6906915.3ns | 2280341.4ns | 302.9% | HIGH |
| abi_entry_form_real_runtime_w | 6720738.6ns | 2217369.3ns | 303.1% | HIGH |
| abi_entry_form_real_scalar_anchor | 7630731.2ns | 2489433.5ns | 306.5% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2181508.3-2409630.6 ns)
  2181508.3 |########################################
  2192914.4 |
  2204320.5 |########################################
  2215726.6 |
  2227132.8 |
  2238538.9 |
  2249945.0 |########################################
  2261351.1 |
  2272757.2 |
  2284163.3 |
  2295569.5 |
  2306975.6 |########################################
  2318381.7 |
  2329787.8 |
  2341193.9 |
  2352600.0 |
  2364006.1 |
  2375412.3 |########################################
  2386818.4 |
  2398224.5 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 4746.7-5091.4 ns)
   4746.7 |########################################
   4763.9 |########################################
   4781.2 |
   4798.4 |
   4815.6 |
   4832.9 |
   4850.1 |
   4867.4 |########################################
   4884.6 |########################################
   4901.8 |
   4919.1 |
   4936.3 |
   4953.6 |
   4970.8 |
   4988.0 |
   5005.3 |
   5022.5 |
   5039.7 |
   5057.0 |########################################
   5074.2 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2189996.7-2395636.7 ns)
  2189996.7 |########################################
  2200278.7 |
  2210560.7 |####################
  2220842.7 |
  2231124.7 |
  2241406.7 |
  2251688.7 |
  2261970.7 |
  2272252.7 |
  2282534.7 |
  2292816.7 |####################
  2303098.7 |
  2313380.7 |
  2323662.7 |
  2333944.7 |
  2344226.7 |
  2354508.7 |####################
  2364790.7 |
  2375072.7 |
  2385354.7 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2194810.4-2248655.2 ns)
  2194810.4 |########################################
  2197502.6 |
  2200194.9 |########################################
  2202887.1 |########################################
  2205579.4 |########################################
  2208271.6 |########################################
  2210963.8 |
  2213656.1 |
  2216348.3 |
  2219040.6 |
  2221732.8 |
  2224425.0 |
  2227117.3 |
  2229809.5 |
  2232501.8 |
  2235194.0 |
  2237886.2 |
  2240578.5 |
  2243270.7 |
  2245963.0 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2190118.3-2900682.7 ns)
  2190118.3 |########################################
  2225646.5 |
  2261174.7 |
  2296703.0 |
  2332231.2 |####################
  2367759.4 |####################
  2403287.6 |
  2438815.8 |
  2474344.1 |
  2509872.3 |
  2545400.5 |
  2580928.7 |
  2616456.9 |####################
  2651985.2 |
  2687513.4 |
  2723041.6 |
  2758569.8 |
  2794098.0 |
  2829626.3 |
  2865154.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=2582.5% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=299.6% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=304.6% of algo (FFI overhead may distort results)
