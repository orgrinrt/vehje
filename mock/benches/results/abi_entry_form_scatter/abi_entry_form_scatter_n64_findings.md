# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_scatter_null_entry dominates: 86696% faster than the next best (abi_entry_form_scatter_per_w_set)

abi_entry_form_scatter_null_entry (2.50 us) leads abi_entry_form_scatter_per_w_set (2.17 ms) by 86696%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_dispatch_table is an outlier: 874.5x slower than the field

abi_entry_form_scatter_dispatch_table (2.19 ms) is 874.5x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_dispatch_table} (86696% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_dispatch_table} with a 86696% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 874.5x the fastest

Fastest abi_entry_form_scatter_null_entry (2.50 us) to slowest abi_entry_form_scatter_dispatch_table (2.19 ms): 874.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 2503.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 874.55x (fastest 2503.6 ns, slowest 2189473.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2218412ns | 2192923ns | 2185655ns | 2191247ns | 2275539ns | -1.57% |
| abi_entry_form_scatter_null_entry | 4945ns | 4771ns | 4719ns | 4755ns | 5341ns | -99.78% |
| abi_entry_form_scatter_per_w_set | 2202564ns | 2176337ns | 2166278ns | 2176041ns | 2260491ns | -2.27% |
| abi_entry_form_scatter_runtime_w | 2253687ns | 2179617ns | 2164697ns | 2175825ns | 2414976ns | base |
| abi_entry_form_scatter_scalar_anchor | 2193590ns | 2179217ns | 2166582ns | 2178569ns | 2229626ns | -2.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2214880ns | 2182162ns | 2271777ns | -1.55% | 0.000 |
| abi_entry_form_scatter_null_entry | 2521ns | 2445ns | 2602ns | -99.89% | 0.025 |
| abi_entry_form_scatter_per_w_set | 2199067ns | 2162950ns | 2256756ns | -2.25% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2249702ns | 2161402ns | 2409908ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2190096ns | 2163503ns | 2225940ns | -2.65% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 67227.7 | 2206605.7 | 2214879.5 | n/a |
| abi_entry_form_scatter_null_entry | 29398.8 | 2756.2 | 2521.1 | n/a |
| abi_entry_form_scatter_per_w_set | 74354.9 | 2246283.3 | 2199067.4 | 0 |
| abi_entry_form_scatter_runtime_w | 71758.3 | 2263947.8 | 2249702.2 | n/a |
| abi_entry_form_scatter_scalar_anchor | 68773.5 | 2178979.8 | 2190096.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_scatter_null_entry | 0.026 | 97.7% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.1% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.1% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2218412ns | 2218412ns | -1.57% |
| abi_entry_form_scatter_null_entry | 4945ns | 4945ns | -99.78% |
| abi_entry_form_scatter_per_w_set | 2202564ns | 2202564ns | -2.27% |
| abi_entry_form_scatter_runtime_w | 2253687ns | 2253687ns | base |
| abi_entry_form_scatter_scalar_anchor | 2193590ns | 2193590ns | -2.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2176075ns | base | --- | [2163124, 2409908] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2189473ns | no significant difference | [-216015, +92497]ns | [2183388, 2271777] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_scatter_null_entry | 2504ns | -2173571.6ns (-99.9%) | [-2407305, -2160666]ns | [2458, 2602] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2172988ns | no significant difference | [-236366, +87549]ns | [2167458, 2256756] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2175655ns | no significant difference | [-235520, +62817]ns | [2168692, 2225940] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2181765ns | +0.1% | -99.9% | -0.4% | -0.8% |
| 2 | 2622656ns | -16.6% | -99.9% | -16.7% | -17.1% |
| 3 | 2161402ns | +7.6% | -99.9% | +7.8% | +5.3% |
| 4 | 2164846ns | +0.8% | -99.9% | +0.3% | +0.5% |
| 5 | 2197159ns | +1.0% | -99.9% | -1.6% | -1.0% |
| 6 | 2170385ns | +1.0% | -99.9% | +0.1% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.386 | moderate- |
| abi_entry_form_scatter_null_entry | -0.401 | moderate- |
| abi_entry_form_scatter_per_w_set | -0.154 | ok |
| abi_entry_form_scatter_runtime_w | -0.251 | moderate- |
| abi_entry_form_scatter_scalar_anchor | -0.190 | ok |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 1/6, lost 5/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_scatter_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6700497.1ns | 2214879.5ns | 302.5% | HIGH |
| abi_entry_form_scatter_null_entry | 115370.0ns | 2521.1ns | 4576.1% | HIGH |
| abi_entry_form_scatter_per_w_set | 6721681.7ns | 2199067.4ns | 305.7% | HIGH |
| abi_entry_form_scatter_runtime_w | 6830528.3ns | 2249702.2ns | 303.6% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6621557.5ns | 2190096.0ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2182161.7-2271777.3 ns)
  2182161.7 |########################################
  2186642.5 |####################
  2191123.3 |####################
  2195604.0 |
  2200084.8 |
  2204565.6 |
  2209046.4 |
  2213527.2 |
  2218007.9 |####################
  2222488.7 |
  2226969.5 |
  2231450.3 |
  2235931.1 |
  2240411.8 |
  2244892.6 |
  2249373.4 |
  2253854.2 |
  2258335.0 |
  2262815.7 |
  2267296.5 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 2445.4-2602.3 ns)
   2445.4 |########################################
   2453.2 |
   2461.1 |
   2468.9 |########################################
   2476.8 |
   2484.6 |########################################
   2492.5 |
   2500.3 |
   2508.2 |
   2516.0 |########################################
   2523.9 |
   2531.7 |########################################
   2539.5 |
   2547.4 |
   2555.2 |
   2563.1 |
   2570.9 |
   2578.8 |
   2586.6 |
   2594.5 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2162949.6-2256756.5 ns)
  2162949.6 |####################
  2167639.9 |####################
  2172330.3 |########################################
  2177020.6 |
  2181711.0 |####################
  2186401.3 |
  2191091.7 |
  2195782.0 |
  2200472.3 |
  2205162.7 |
  2209853.0 |
  2214543.4 |
  2219233.7 |
  2223924.1 |
  2228614.4 |
  2233304.7 |
  2237995.1 |
  2242685.4 |
  2247375.8 |
  2252066.1 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2161401.7-2409907.5 ns)
  2161401.7 |########################################
  2173827.0 |#############
  2186252.3 |#############
  2198677.6 |
  2211102.9 |
  2223528.2 |
  2235953.4 |
  2248378.7 |
  2260804.0 |
  2273229.3 |
  2285654.6 |
  2298079.9 |
  2310505.2 |
  2322930.5 |
  2335355.8 |
  2347781.0 |
  2360206.3 |
  2372631.6 |
  2385056.9 |
  2397482.2 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2163503.3-2225940.4 ns)
  2163503.3 |####################
  2166625.2 |
  2169747.0 |
  2172868.9 |########################################
  2175990.7 |########################################
  2179112.6 |
  2182234.4 |
  2185356.3 |
  2188478.1 |
  2191600.0 |
  2194721.8 |
  2197843.7 |
  2200965.6 |
  2204087.4 |
  2207209.3 |
  2210331.1 |
  2213453.0 |
  2216574.8 |
  2219696.7 |
  2222818.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=4568.7% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.3% of algo (FFI overhead may distort results)
