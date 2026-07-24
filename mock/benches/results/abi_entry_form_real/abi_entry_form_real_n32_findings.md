# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_real_null_entry dominates: 94848% faster than the next best (abi_entry_form_real_scalar_anchor)

abi_entry_form_real_null_entry (2.30 us) leads abi_entry_form_real_scalar_anchor (2.19 ms) by 94848%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.19 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_dispatch_table is an outlier: 953.2x slower than the field

abi_entry_form_real_dispatch_table (2.20 ms) is 953.2x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w, abi_entry_form_real_dispatch_table} (94848% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w, abi_entry_form_real_dispatch_table} with a 94848% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 953.2x the fastest

Fastest abi_entry_form_real_null_entry (2.30 us) to slowest abi_entry_form_real_dispatch_table (2.20 ms): 953.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 2303.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 953.18x (fastest 2303.9 ns, slowest 2196085.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2232498ns | 2199940ns | 2191688ns | 2197325ns | 2305661ns | +1.52% |
| abi_entry_form_real_null_entry | 4609ns | 4605ns | 4494ns | 4591ns | 4692ns | -99.79% |
| abi_entry_form_real_per_w_set | 2246092ns | 2196949ns | 2181446ns | 2196734ns | 2352452ns | +2.14% |
| abi_entry_form_real_runtime_w | 2199091ns | 2198227ns | 2188962ns | 2195172ns | 2210035ns | base |
| abi_entry_form_real_scalar_anchor | 2192095ns | 2191116ns | 2186550ns | 2190800ns | 2196810ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2228597ns | 2187972ns | 2301516ns | +1.51% | 0.000 |
| abi_entry_form_real_null_entry | 2308ns | 2270ns | 2341ns | -99.89% | 0.014 |
| abi_entry_form_real_per_w_set | 2242125ns | 2177830ns | 2348239ns | +2.13% | 0.000 |
| abi_entry_form_real_runtime_w | 2195348ns | 2185122ns | 2206316ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2188411ns | 2182744ns | 2193086ns | -0.32% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 90132.8 | 2216916.1 | 2228597.0 | n/a |
| abi_entry_form_real_null_entry | 30019.7 | 2433.7 | 2308.5 | n/a |
| abi_entry_form_real_per_w_set | 88924.3 | 2289158.4 | 2242124.8 | n/a |
| abi_entry_form_real_runtime_w | 79783.5 | 2197911.7 | 2195347.6 | n/a |
| abi_entry_form_real_scalar_anchor | 80740.8 | 2189005.2 | 2188410.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.014 | 98.5% |
| abi_entry_form_real_per_w_set | 0.000 | 0.1% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2232498ns | 2232498ns | +1.52% |
| abi_entry_form_real_null_entry | 4609ns | 4609ns | -99.79% |
| abi_entry_form_real_per_w_set | 2246092ns | 2246092ns | +2.14% |
| abi_entry_form_real_runtime_w | 2199091ns | 2199091ns | base |
| abi_entry_form_real_scalar_anchor | 2192095ns | 2192095ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2194478ns | base | --- | [2185249, 2206316] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2196086ns | +3554.1ns (+0.2%) | [+994, +95200]ns | [2188190, 2301516] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| abi_entry_form_real_null_entry | 2304ns | -2192159.9ns (-99.9%) | [-2204003, -2182954]ns | [2280, 2341] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2193141ns | no significant difference | [-12350, +151369]ns | [2184994, 2348239] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_real_scalar_anchor | 2187549ns | no significant difference | [-21719, +6429]ns | [2184597, 2193086] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2196822ns | +0.2% | -99.9% | +0.1% | -0.4% |
| 2 | 2185122ns | +0.2% | -99.9% | +0.3% | +0.1% |
| 3 | 2185377ns | +0.1% | -99.9% | -0.3% | +0.5% |
| 4 | 2203320ns | +5.4% | -99.9% | +13.4% | -0.8% |
| 5 | 2192135ns | -0.0% | -99.9% | +0.1% | -0.1% |
| 6 | 2209311ns | +3.2% | -99.9% | -0.8% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.382 | moderate- |
| abi_entry_form_real_null_entry | 0.233 | moderate+ |
| abi_entry_form_real_per_w_set | -0.267 | moderate- |
| abi_entry_form_real_runtime_w | -0.133 | ok |
| abi_entry_form_real_scalar_anchor | -0.369 | moderate- |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 0/6, lost 5/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 2/6, lost 2/6
- **abi_entry_form_real_scalar_anchor**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6758119.9ns | 2228597.0ns | 303.2% | HIGH |
| abi_entry_form_real_null_entry | 118370.0ns | 2308.5ns | 5127.7% | HIGH |
| abi_entry_form_real_per_w_set | 6811354.7ns | 2242124.8ns | 303.8% | HIGH |
| abi_entry_form_real_runtime_w | 6668349.6ns | 2195347.6ns | 303.7% | HIGH |
| abi_entry_form_real_scalar_anchor | 6655563.6ns | 2188410.6ns | 304.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2187971.7-2301515.7 ns)
  2187971.7 |########################################
  2193648.9 |
  2199326.1 |#############
  2205003.3 |
  2210680.5 |
  2216357.7 |
  2222034.9 |
  2227712.1 |
  2233389.3 |
  2239066.5 |
  2244743.7 |
  2250420.9 |
  2256098.1 |
  2261775.3 |
  2267452.5 |
  2273129.7 |
  2278806.9 |#############
  2284484.1 |
  2290161.3 |
  2295838.5 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 2270.4-2341.0 ns)
   2270.4 |########################################
   2273.9 |
   2277.5 |
   2281.0 |
   2284.5 |
   2288.1 |########################################
   2291.6 |
   2295.1 |
   2298.6 |########################################
   2302.2 |
   2305.7 |########################################
   2309.2 |
   2312.8 |########################################
   2316.3 |
   2319.8 |
   2323.3 |
   2326.9 |
   2330.4 |
   2333.9 |
   2337.5 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2177830.0-2348239.2 ns)
  2177830.0 |#############
  2186350.5 |########################################
  2194870.9 |#############
  2203391.4 |
  2211911.8 |
  2220432.3 |
  2228952.7 |
  2237473.2 |
  2245993.7 |
  2254514.1 |
  2263034.6 |
  2271555.0 |
  2280075.5 |
  2288595.9 |
  2297116.4 |
  2305636.9 |
  2314157.3 |
  2322677.8 |
  2331198.2 |
  2339718.7 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2185121.7-2206315.6 ns)
  2185121.7 |########################################
  2186181.4 |
  2187241.1 |
  2188300.8 |
  2189360.5 |
  2190420.2 |
  2191479.9 |####################
  2192539.6 |
  2193599.3 |
  2194659.0 |
  2195718.6 |
  2196778.3 |####################
  2197838.0 |
  2198897.7 |
  2199957.4 |
  2201017.1 |
  2202076.8 |
  2203136.5 |####################
  2204196.2 |
  2205255.9 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2182743.8-2193085.8 ns)
  2182743.8 |########################################
  2183260.9 |
  2183778.0 |
  2184295.1 |
  2184812.2 |
  2185329.3 |
  2185846.4 |
  2186363.5 |########################################
  2186880.6 |########################################
  2187397.7 |########################################
  2187914.8 |
  2188431.9 |
  2188949.0 |
  2189466.1 |
  2189983.2 |
  2190500.3 |########################################
  2191017.4 |
  2191534.5 |
  2192051.6 |
  2192568.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=5146.5% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=304.3% of algo (FFI overhead may distort results)
