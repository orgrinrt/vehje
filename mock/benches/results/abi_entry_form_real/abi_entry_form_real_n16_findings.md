# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_real_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_real_runtime_w has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_real_null_entry at 2.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_real_null_entry dominates: 85948% faster than the next best (abi_entry_form_real_per_w_set)

abi_entry_form_real_null_entry (2.53 us) leads abi_entry_form_real_per_w_set (2.18 ms) by 85948%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.19 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_runtime_w is an outlier: 865.7x slower than the field

abi_entry_form_real_runtime_w (2.19 ms) is 865.7x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_runtime_w} (85948% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_runtime_w} with a 85948% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 865.7x the fastest

Fastest abi_entry_form_real_null_entry (2.53 us) to slowest abi_entry_form_real_runtime_w (2.19 ms): 865.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 2533.3 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 865.73x (fastest 2533.3 ns, slowest 2193155.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2254479ns | 2191975ns | 2175524ns | 2190592ns | 2389787ns | -1.34% |
| abi_entry_form_real_null_entry | 4895ns | 4805ns | 4724ns | 4805ns | 5115ns | -99.79% |
| abi_entry_form_real_per_w_set | 2198661ns | 2183242ns | 2172337ns | 2180833ns | 2238565ns | -3.78% |
| abi_entry_form_real_runtime_w | 2285095ns | 2196668ns | 2183572ns | 2193672ns | 2472992ns | base |
| abi_entry_form_real_scalar_anchor | 2199143ns | 2195373ns | 2175074ns | 2192350ns | 2221367ns | -3.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2250742ns | 2172348ns | 2385560ns | -1.34% | 0.000 |
| abi_entry_form_real_null_entry | 2559ns | 2503ns | 2633ns | -99.89% | 0.006 |
| abi_entry_form_real_per_w_set | 2195184ns | 2169308ns | 2234693ns | -3.77% | 0.000 |
| abi_entry_form_real_runtime_w | 2281216ns | 2180047ns | 2468393ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2195595ns | 2171877ns | 2217470ns | -3.75% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 78577.5 | 2232884.6 | 2250742.0 | 0 |
| abi_entry_form_real_null_entry | 30094.6 | 2717.3 | 2558.7 | n/a |
| abi_entry_form_real_per_w_set | 72399.5 | 2190808.8 | 2195184.3 | n/a |
| abi_entry_form_real_runtime_w | 77531.9 | 2321737.9 | 2281215.7 | n/a |
| abi_entry_form_real_scalar_anchor | 72050.3 | 2199448.6 | 2195595.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.006 | 98.8% |
| abi_entry_form_real_per_w_set | 0.000 | 0.1% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2254479ns | 2254479ns | -1.34% |
| abi_entry_form_real_null_entry | 4895ns | 4895ns | -99.79% |
| abi_entry_form_real_per_w_set | 2198661ns | 2198661ns | -3.78% |
| abi_entry_form_real_runtime_w | 2285095ns | 2285095ns | base |
| abi_entry_form_real_scalar_anchor | 2199143ns | 2199143ns | -3.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2193155ns | base | --- | [2182099, 2468393] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2188292ns | no significant difference | [-235136, +158497]ns | [2178374, 2385560] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_real_null_entry | 2533ns | -2190578.6ns (-99.9%) | [-2465807, -2179586]ns | [2510, 2633] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2179858ns | no significant difference | [-241556, +5616]ns | [2171002, 2234693] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_real_scalar_anchor | 2191879ns | no significant difference | [-258045, +12602]ns | [2177437, 2217470] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2192754ns | -0.4% | -99.9% | -0.9% | +0.2% |
| 2 | 2184151ns | +14.0% | -99.9% | +0.6% | -0.1% |
| 3 | 2276427ns | -4.0% | -99.9% | -4.2% | -4.0% |
| 4 | 2193557ns | -1.0% | -99.9% | -1.1% | -1.0% |
| 5 | 2660358ns | -14.3% | -99.9% | -14.6% | -16.0% |
| 6 | 2180047ns | +0.5% | -99.9% | -0.0% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.399 | moderate- |
| abi_entry_form_real_null_entry | -0.355 | moderate- |
| abi_entry_form_real_per_w_set | -0.383 | moderate- |
| abi_entry_form_real_runtime_w | -0.347 | moderate- |
| abi_entry_form_real_scalar_anchor | -0.177 | ok |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 4/6, lost 1/6
- **abi_entry_form_real_scalar_anchor**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6816682.4ns | 2250742.0ns | 302.9% | HIGH |
| abi_entry_form_real_null_entry | 120088.0ns | 2558.7ns | 4693.4% | HIGH |
| abi_entry_form_real_per_w_set | 6650210.1ns | 2195184.3ns | 302.9% | HIGH |
| abi_entry_form_real_runtime_w | 7053264.1ns | 2281215.7ns | 309.2% | HIGH |
| abi_entry_form_real_scalar_anchor | 6671573.8ns | 2195595.4ns | 303.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2172348.3-2385560.0 ns)
  2172348.3 |#############
  2183008.9 |########################################
  2193669.5 |
  2204330.1 |
  2214990.6 |
  2225651.2 |
  2236311.8 |
  2246972.4 |
  2257633.0 |
  2268293.6 |
  2278954.1 |#############
  2289614.7 |
  2300275.3 |
  2310935.9 |
  2321596.5 |
  2332257.1 |
  2342917.7 |
  2353578.2 |
  2364238.8 |
  2374899.4 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 2503.3-2632.7 ns)
   2503.3 |########################################
   2509.8 |
   2516.2 |########################################
   2522.7 |########################################
   2529.2 |
   2535.7 |
   2542.1 |########################################
   2548.6 |
   2555.1 |
   2561.5 |
   2568.0 |
   2574.5 |
   2580.9 |
   2587.4 |
   2593.9 |
   2600.3 |
   2606.8 |########################################
   2613.3 |
   2619.8 |
   2626.2 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2169307.9-2234692.9 ns)
  2169307.9 |########################################
  2172577.1 |########################################
  2175846.4 |########################################
  2179115.6 |########################################
  2182384.9 |
  2185654.1 |
  2188923.4 |
  2192192.6 |
  2195461.9 |########################################
  2198731.1 |
  2202000.4 |
  2205269.6 |
  2208538.9 |
  2211808.1 |
  2215077.4 |
  2218346.6 |
  2221615.9 |
  2224885.1 |
  2228154.4 |
  2231423.6 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2180046.7-2468392.7 ns)
  2180046.7 |########################################
  2194464.0 |
  2208881.3 |
  2223298.6 |
  2237715.9 |
  2252133.2 |
  2266550.5 |##########
  2280967.8 |
  2295385.1 |
  2309802.4 |
  2324219.7 |
  2338637.0 |
  2353054.3 |
  2367471.6 |
  2381888.9 |
  2396306.2 |
  2410723.5 |
  2425140.8 |
  2439558.1 |
  2453975.4 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2171877.1-2217470.0 ns)
  2171877.1 |########################################
  2174156.7 |
  2176436.4 |
  2178716.0 |
  2180995.7 |########################################
  2183275.3 |
  2185555.0 |########################################
  2187834.6 |
  2190114.3 |
  2192393.9 |
  2194673.5 |
  2196953.2 |########################################
  2199232.8 |########################################
  2201512.5 |
  2203792.1 |
  2206071.8 |
  2208351.4 |
  2210631.1 |
  2212910.7 |
  2215190.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=4697.8% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=303.9% of algo (FFI overhead may distort results)
