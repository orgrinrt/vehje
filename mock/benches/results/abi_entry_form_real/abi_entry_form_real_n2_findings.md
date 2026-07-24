# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_real_null_entry dominates: 63257% faster than the next best (abi_entry_form_real_scalar_anchor)

abi_entry_form_real_null_entry (3.46 us) leads abi_entry_form_real_scalar_anchor (2.19 ms) by 63257%, a clear separation rather than a photo finish. CV 17.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.22 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_dispatch_table is an outlier: 672.7x slower than the field

abi_entry_form_real_dispatch_table (2.33 ms) is 672.7x the fastest (3.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_real_null_entry is fastest but the noisiest (CV 17.6%)

abi_entry_form_real_null_entry wins on median (3.46 us) yet has the highest variance (CV 17.6%), while abi_entry_form_real_dispatch_table is the steadiest (CV 2.8%, 2.33 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w, abi_entry_form_real_dispatch_table} (63257% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w, abi_entry_form_real_dispatch_table} with a 63257% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 672.7x the fastest

Fastest abi_entry_form_real_null_entry (3.46 us) to slowest abi_entry_form_real_dispatch_table (2.33 ms): 672.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 3462.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 672.71x (fastest 3462.5 ns, slowest 2329263.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2301005ns | 2333732ns | 2204784ns | 2293553ns | 2360294ns | +1.71% |
| abi_entry_form_real_null_entry | 6214ns | 5813ns | 5576ns | 5740ns | 7244ns | -99.73% |
| abi_entry_form_real_per_w_set | 2273510ns | 2201578ns | 2181809ns | 2195017ns | 2437100ns | +0.49% |
| abi_entry_form_real_runtime_w | 2262361ns | 2230029ns | 2205474ns | 2223295ns | 2349403ns | base |
| abi_entry_form_real_scalar_anchor | 2234683ns | 2197142ns | 2180566ns | 2195150ns | 2321042ns | -1.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2296747ns | 2200979ns | 2355734ns | +1.70% | 0.000 |
| abi_entry_form_real_null_entry | 3713ns | 3337ns | 4324ns | -99.84% | 0.001 |
| abi_entry_form_real_per_w_set | 2269660ns | 2178174ns | 2432769ns | +0.50% | 0.000 |
| abi_entry_form_real_runtime_w | 2258300ns | 2201621ns | 2345071ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2230936ns | 2177275ns | 2316570ns | -1.21% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 87013.1 | 2282849.6 | 2296746.9 | n/a |
| abi_entry_form_real_null_entry | 35024.9 | 3731.8 | 3712.9 | n/a |
| abi_entry_form_real_per_w_set | 84594.0 | 2309305.6 | 2269660.5 | 0 |
| abi_entry_form_real_runtime_w | 85132.2 | 2241348.2 | 2258299.8 | n/a |
| abi_entry_form_real_scalar_anchor | 78467.2 | 2239652.4 | 2230935.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.001 | 96.4% |
| abi_entry_form_real_per_w_set | 0.000 | 0.2% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2301005ns | 2301005ns | +1.71% |
| abi_entry_form_real_null_entry | 6214ns | 6214ns | -99.73% |
| abi_entry_form_real_per_w_set | 2273510ns | 2273510ns | +0.49% |
| abi_entry_form_real_runtime_w | 2262361ns | 2262361ns | base |
| abi_entry_form_real_scalar_anchor | 2234683ns | 2234683ns | -1.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2225889ns | base | --- | [2203939, 2345071] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2329263ns | no significant difference | [-59483, +136915]ns | [2205243, 2355734] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_real_null_entry | 3462ns | -2222522.0ns (-99.8%) | [-2341513, -2199726]ns | [3352, 4324] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2197848ns | no significant difference | [-158616, +213950]ns | [2178364, 2432769] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_real_scalar_anchor | 2193745ns | no significant difference | [-151376, +112630]ns | [2182493, 2316570] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2206258ns | +6.6% | -99.8% | +10.6% | +9.3% |
| 2 | 2220398ns | -0.5% | -99.8% | -0.9% | -1.5% |
| 3 | 2243175ns | +3.4% | -99.8% | -2.2% | -2.4% |
| 4 | 2231381ns | +5.7% | -99.9% | +8.7% | -2.4% |
| 5 | 2446967ns | -4.4% | -99.9% | -11.0% | -10.2% |
| 6 | 2201621ns | -0.0% | -99.8% | -1.1% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.263 | moderate- |
| abi_entry_form_real_null_entry | -0.092 | ok |
| abi_entry_form_real_per_w_set | -0.300 | moderate- |
| abi_entry_form_real_runtime_w | -0.292 | moderate- |
| abi_entry_form_real_scalar_anchor | -0.042 | ok |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 2/6, lost 3/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_real_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6952488.2ns | 2296746.9ns | 302.7% | HIGH |
| abi_entry_form_real_null_entry | 130147.3ns | 3712.9ns | 3505.3% | HIGH |
| abi_entry_form_real_per_w_set | 6936318.9ns | 2269660.5ns | 305.6% | HIGH |
| abi_entry_form_real_runtime_w | 6825990.5ns | 2258299.8ns | 302.3% | HIGH |
| abi_entry_form_real_scalar_anchor | 6796340.4ns | 2230935.9ns | 304.6% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2200978.8-2355734.2 ns)
  2200978.8 |########################################
  2208716.6 |########################################
  2216454.3 |
  2224192.1 |
  2231929.9 |
  2239667.6 |
  2247405.4 |
  2255143.2 |
  2262880.9 |
  2270618.7 |
  2278356.5 |
  2286094.2 |
  2293832.0 |
  2301569.8 |
  2309307.5 |
  2317045.3 |########################################
  2324783.1 |
  2332520.8 |########################################
  2340258.6 |
  2347996.4 |########################################
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 3337.1-4324.1 ns)
   3337.1 |########################################
   3386.5 |####################
   3435.8 |
   3485.2 |####################
   3534.5 |
   3583.9 |####################
   3633.2 |
   3682.6 |
   3731.9 |
   3781.3 |
   3830.6 |
   3880.0 |
   3929.3 |
   3978.7 |
   4028.0 |
   4077.4 |
   4126.7 |
   4176.1 |
   4225.4 |
   4274.8 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2178173.8-2432769.0 ns)
  2178173.8 |########################################
  2190903.6 |########################################
  2203633.3 |
  2216363.1 |
  2229092.8 |
  2241822.6 |
  2254552.3 |
  2267282.1 |
  2280011.9 |
  2292741.6 |
  2305471.4 |
  2318201.1 |
  2330930.9 |
  2343660.6 |
  2356390.4 |
  2369120.2 |
  2381849.9 |
  2394579.7 |
  2407309.4 |
  2420039.2 |####################
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2201621.2-2345070.9 ns)
  2201621.2 |########################################
  2208793.7 |
  2215966.2 |####################
  2223138.6 |
  2230311.1 |####################
  2237483.6 |####################
  2244656.1 |
  2251828.6 |
  2259001.1 |
  2266173.5 |
  2273346.0 |
  2280518.5 |
  2287691.0 |
  2294863.5 |
  2302036.0 |
  2309208.4 |
  2316380.9 |
  2323553.4 |
  2330725.9 |
  2337898.4 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2177275.0-2316569.6 ns)
  2177275.0 |####################
  2184239.7 |########################################
  2191204.5 |
  2198169.2 |####################
  2205133.9 |
  2212098.6 |
  2219063.4 |####################
  2226028.1 |
  2232992.8 |
  2239957.6 |
  2246922.3 |
  2253887.0 |
  2260851.8 |
  2267816.5 |
  2274781.2 |
  2281746.0 |
  2288710.7 |
  2295675.4 |
  2302640.1 |
  2309604.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=299.1% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=3519.8% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=303.2% of algo (FFI overhead may distort results)
