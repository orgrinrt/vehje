# abi_marshal (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_scatter_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_scatter_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_scatter_marshal_null dominates: 10015% faster than the next best (abi_marshal_scatter_aos)

abi_marshal_scatter_marshal_null (21.07 us) leads abi_marshal_scatter_aos (2.13 ms) by 10015%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_scatter_marshal_null beats baseline by 99% (significant)

abi_marshal_scatter_marshal_null is -2.11 ms (99%) faster than baseline abi_marshal_scatter_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_scatter_soa_transposed is an outlier: 102.3x slower than the field

abi_marshal_scatter_soa_transposed (2.16 ms) is 102.3x the fastest (21.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_scatter_marshal_null} vs {abi_marshal_scatter_aos, abi_marshal_scatter_soa_native, abi_marshal_scatter_soa_transposed} (10015% apart)

The field splits into a fast tier {abi_marshal_scatter_marshal_null} and a slow tier {abi_marshal_scatter_aos, abi_marshal_scatter_soa_native, abi_marshal_scatter_soa_transposed} with a 10015% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 102.3x the fastest

Fastest abi_marshal_scatter_marshal_null (21.07 us) to slowest abi_marshal_scatter_soa_transposed (2.16 ms): 102.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_scatter_marshal_null** at 21067.3 ns median (-99.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 102.35x (fastest 21067.3 ns, slowest 2156156.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2209553ns | 2133500ns | 2118745ns | 2130411ns | 2373670ns | base |
| abi_marshal_scatter_marshal_null | 23296ns | 23341ns | 22380ns | 23284ns | 23773ns | -98.95% |
| abi_marshal_scatter_soa_native | 2136271ns | 2136893ns | 2130452ns | 2135959ns | 2139648ns | -3.32% |
| abi_marshal_scatter_soa_transposed | 2159807ns | 2158716ns | 2152995ns | 2157527ns | 2166633ns | -2.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2206817ns | 2116321ns | 2370474ns | base | 0.000 |
| abi_marshal_scatter_marshal_null | 21020ns | 20192ns | 21451ns | -99.05% | 0.000 |
| abi_marshal_scatter_soa_native | 2133679ns | 2127820ns | 2137148ns | -3.31% | 0.000 |
| abi_marshal_scatter_soa_transposed | 2157170ns | 2150276ns | 2163983ns | -2.25% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 47379.2 | 2211002.9 | 2206816.5 | n/a |
| abi_marshal_scatter_marshal_null | 26834.5 | 21178.8 | 21020.4 | n/a |
| abi_marshal_scatter_soa_native | 39570.3 | 2132809.3 | 2133679.4 | n/a |
| abi_marshal_scatter_soa_transposed | 38141.0 | 2157444.3 | 2157170.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_scatter_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_scatter_aos | 0.000 | 0.9% |
| abi_marshal_scatter_marshal_null | 0.000 | 95.8% |
| abi_marshal_scatter_soa_native | 0.000 | 0.9% |
| abi_marshal_scatter_soa_transposed | 0.000 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_scatter_aos | 2209553ns | 2209553ns | base |
| abi_marshal_scatter_marshal_null | 23296ns | 23296ns | -98.95% |
| abi_marshal_scatter_soa_native | 2136271ns | 2136271ns | -3.32% |
| abi_marshal_scatter_soa_transposed | 2159807ns | 2159807ns | -2.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2130942ns | base | --- | [2119034, 2370474] | --- | --- | --- | --- |
| abi_marshal_scatter_marshal_null | 21067ns | -2109922.8ns (-99.0%) | [-2349136, -2098330]ns | [20542, 21451] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_scatter_soa_native | 2134289ns | no significant difference | [-237621, +15412]ns | [2129601, 2137148] | no | 1.0000 | 1.0000 | 0 |
| abi_marshal_scatter_soa_transposed | 2156157ns | no significant difference | [-214389, +35389]ns | [2151370, 2163983] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_scatter_aos | abi_marshal_scatter_marshal_null | abi_marshal_scatter_soa_native | abi_marshal_scatter_soa_transposed |
|---|---|---|---|---|
| 1 | 2127152ns | -99.0% | +0.6% | +1.5% |
| 2 | 2116321ns | -99.0% | +0.9% | +1.8% |
| 3 | 2121747ns | -99.0% | +0.3% | +1.3% |
| 4 | 2605527ns | -99.2% | -18.2% | -17.4% |
| 5 | 2134732ns | -99.0% | -0.0% | +1.6% |
| 6 | 2135421ns | -99.0% | -0.1% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_scatter_aos | -0.223 | moderate- |
| abi_marshal_scatter_marshal_null | -0.067 | ok |
| abi_marshal_scatter_soa_native | 0.165 | ok |
| abi_marshal_scatter_soa_transposed | 0.128 | ok |

**Consistency summary:**

- **abi_marshal_scatter_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_scatter_soa_native**: won 1/6, lost 3/6
- **abi_marshal_scatter_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 6665782.4ns | 2206816.5ns | 302.1% | HIGH |
| abi_marshal_scatter_marshal_null | 165372.4ns | 21020.4ns | 786.7% | HIGH |
| abi_marshal_scatter_soa_native | 6444578.0ns | 2133679.4ns | 302.0% | HIGH |
| abi_marshal_scatter_soa_transposed | 6511635.7ns | 2157170.1ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_scatter_aos (n=6, range 2116320.8-2370474.0 ns)
  2116320.8 |########################################
  2129028.5 |##########################
  2141736.1 |
  2154443.8 |
  2167151.4 |
  2179859.1 |
  2192566.7 |
  2205274.4 |
  2217982.1 |
  2230689.7 |
  2243397.4 |
  2256105.0 |
  2268812.7 |
  2281520.3 |
  2294228.0 |
  2306935.7 |
  2319643.3 |
  2332351.0 |
  2345058.6 |
  2357766.3 |
  (0 below, 1 above range)

abi_marshal_scatter_marshal_null (n=6, range 20192.1-21451.4 ns)
  20192.1 |########################################
  20255.1 |
  20318.0 |
  20381.0 |
  20444.0 |
  20506.9 |
  20569.9 |
  20632.9 |
  20695.8 |
  20758.8 |
  20821.8 |
  20884.7 |########################################
  20947.7 |########################################
  21010.7 |
  21073.6 |
  21136.6 |########################################
  21199.6 |########################################
  21262.5 |
  21325.5 |
  21388.5 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_native (n=6, range 2127820.0-2137148.3 ns)
  2127820.0 |####################
  2128286.4 |
  2128752.8 |
  2129219.2 |
  2129685.7 |
  2130152.1 |
  2130618.5 |
  2131084.9 |####################
  2131551.3 |
  2132017.7 |
  2132484.1 |
  2132950.6 |
  2133417.0 |
  2133883.4 |########################################
  2134349.8 |
  2134816.2 |####################
  2135282.6 |
  2135749.1 |
  2136215.5 |
  2136681.9 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_transposed (n=6, range 2150276.2-2163983.3 ns)
  2150276.2 |########################################
  2150961.6 |
  2151646.9 |
  2152332.3 |########################################
  2153017.6 |########################################
  2153703.0 |
  2154388.3 |
  2155073.7 |
  2155759.0 |
  2156444.4 |
  2157129.8 |
  2157815.1 |
  2158500.5 |########################################
  2159185.8 |########################################
  2159871.2 |
  2160556.5 |
  2161241.9 |
  2161927.2 |
  2162612.6 |
  2163297.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_scatter_aos**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_marshal_null**: bridge=783.3% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_native**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)
