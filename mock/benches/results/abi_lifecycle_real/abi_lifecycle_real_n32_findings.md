# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 90442% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (2.37 us) leads abi_lifecycle_real_held_handle (2.15 ms) by 90442%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 947.2x slower than the field

abi_lifecycle_real_fresh_per_batch (2.25 ms) is 947.2x the fastest (2.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (90442% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 90442% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 947.2x the fastest

Fastest abi_lifecycle_real_null_entry (2.37 us) to slowest abi_lifecycle_real_fresh_per_batch (2.25 ms): 947.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 2370.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 947.16x (fastest 2370.6 ns, slowest 2245327.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2271803ns | 2247877ns | 2236355ns | 2247749ns | 2325608ns | +5.64% |
| abi_lifecycle_real_fresh_per_column | 2265146ns | 2171404ns | 2150721ns | 2169934ns | 2465176ns | +5.33% |
| abi_lifecycle_real_held_handle | 2150563ns | 2148984ns | 2141857ns | 2146881ns | 2160439ns | base |
| abi_lifecycle_real_null_entry | 4688ns | 4734ns | 4373ns | 4642ns | 4912ns | -99.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2269043ns | 2233418ns | 2322651ns | +5.64% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2262414ns | 2148152ns | 2462474ns | +5.33% | 0.000 |
| abi_lifecycle_real_held_handle | 2147974ns | 2139321ns | 2157836ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 2338ns | 2211ns | 2419ns | -99.89% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 45731.7 | 2392796.4 | 2269042.8 | n/a |
| abi_lifecycle_real_fresh_per_column | 43921.1 | 2171957.7 | 2262414.2 | n/a |
| abi_lifecycle_real_held_handle | 41900.2 | 2148678.2 | 2147973.9 | n/a |
| abi_lifecycle_real_null_entry | 28677.1 | 2472.1 | 2338.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_real_held_handle | 0.000 | 0.1% |
| abi_lifecycle_real_null_entry | 0.013 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2271803ns | 2271803ns | +5.64% |
| abi_lifecycle_real_fresh_per_column | 2265146ns | 2265146ns | +5.33% |
| abi_lifecycle_real_held_handle | 2150563ns | 2150563ns | base |
| abi_lifecycle_real_null_entry | 4688ns | 4688ns | -99.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2146395ns | base | --- | [2139690, 2157836] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2245328ns | +104468.4ns (+4.9%) | [+88695, +170044]ns | [2239150, 2322651] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2168566ns | +21858.6ns (+1.0%) | [+13855, +307607]ns | [2156203, 2462474] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_null_entry | 2371ns | -2144065.2ns (-99.9%) | [-2155465, -2137377]ns | [2224, 2419] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2164025ns | +10.8% | +27.1% | -99.9% |
| 2 | 2143465ns | +4.9% | +1.0% | -99.9% |
| 3 | 2151647ns | +4.3% | +1.1% | -99.9% |
| 4 | 2139321ns | +5.0% | +1.3% | -99.9% |
| 5 | 2149326ns | +3.9% | +0.9% | -99.9% |
| 6 | 2140059ns | +4.9% | +0.4% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | ok |
| abi_lifecycle_real_fresh_per_column | -0.040 | ok |
| abi_lifecycle_real_held_handle | -0.332 | moderate- |
| abi_lifecycle_real_null_entry | -0.254 | moderate- |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 6974271.5ns | 2269042.8ns | 307.4% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6583291.2ns | 2262414.2ns | 291.0% | HIGH |
| abi_lifecycle_real_held_handle | 6492038.7ns | 2147973.9ns | 302.2% | HIGH |
| abi_lifecycle_real_null_entry | 118237.8ns | 2338.0ns | 5057.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2233418.3-2322650.7 ns)
  2233418.3 |#############
  2237879.9 |
  2242341.5 |########################################
  2246803.2 |#############
  2251264.8 |
  2255726.4 |
  2260188.0 |
  2264649.6 |
  2269111.2 |
  2273572.9 |
  2278034.5 |
  2282496.1 |
  2286957.7 |
  2291419.3 |
  2295880.9 |
  2300342.6 |
  2304804.2 |
  2309265.8 |
  2313727.4 |
  2318189.0 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2148152.1-2462474.0 ns)
  2148152.1 |##########
  2163868.2 |########################################
  2179584.3 |
  2195300.4 |
  2211016.5 |
  2226732.6 |
  2242448.7 |
  2258164.7 |
  2273880.8 |
  2289596.9 |
  2305313.0 |
  2321029.1 |
  2336745.2 |
  2352461.3 |
  2368177.4 |
  2383893.5 |
  2399609.6 |
  2415325.7 |
  2431041.8 |
  2446757.9 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2139321.2-2157836.0 ns)
  2139321.2 |########################################
  2140246.9 |
  2141172.7 |
  2142098.4 |
  2143024.2 |####################
  2143949.9 |
  2144875.7 |
  2145801.4 |
  2146727.1 |
  2147652.9 |
  2148578.6 |####################
  2149504.4 |
  2150430.1 |
  2151355.9 |####################
  2152281.6 |
  2153207.3 |
  2154133.1 |
  2155058.8 |
  2155984.6 |
  2156910.3 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 2210.8-2419.2 ns)
   2210.8 |########################################
   2221.2 |
   2231.6 |########################################
   2242.1 |
   2252.5 |
   2262.9 |
   2273.3 |
   2283.7 |
   2294.1 |
   2304.6 |
   2315.0 |
   2325.4 |
   2335.8 |########################################
   2346.2 |
   2356.6 |
   2367.1 |
   2377.5 |
   2387.9 |
   2398.3 |########################################
   2408.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=4984.8% of algo (FFI overhead may distort results)
