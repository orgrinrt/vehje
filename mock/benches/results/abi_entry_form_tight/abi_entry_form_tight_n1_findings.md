# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 42056% faster than the next best (abi_entry_form_tight_scalar_anchor)

abi_entry_form_tight_null_entry (4.90 us) leads abi_entry_form_tight_scalar_anchor (2.07 ms) by 42056%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.06 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_per_w_set is an outlier: 424.0x slower than the field

abi_entry_form_tight_per_w_set (2.08 ms) is 424.0x the fastest (4.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_scalar_anchor, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set} (42056% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_scalar_anchor, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set} with a 42056% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 424.0x the fastest

Fastest abi_entry_form_tight_null_entry (4.90 us) to slowest abi_entry_form_tight_per_w_set (2.08 ms): 424.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 4901.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 424.02x (fastest 4901.0 ns, slowest 2078128.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2078066ns | 2072332ns | 2056068ns | 2070243ns | 2100798ns | -2.48% |
| abi_entry_form_tight_null_entry | 7335ns | 7215ns | 7100ns | 7213ns | 7636ns | -99.66% |
| abi_entry_form_tight_per_w_set | 2129772ns | 2081774ns | 2053955ns | 2074274ns | 2250927ns | -0.06% |
| abi_entry_form_tight_runtime_w | 2130956ns | 2072854ns | 2058578ns | 2070006ns | 2258569ns | base |
| abi_entry_form_tight_scalar_anchor | 2090193ns | 2069377ns | 2061050ns | 2067126ns | 2139364ns | -1.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2074584ns | 2052741ns | 2097101ns | -2.48% | 0.000 |
| abi_entry_form_tight_null_entry | 4982ns | 4842ns | 5197ns | -99.77% | 0.000 |
| abi_entry_form_tight_per_w_set | 2126151ns | 2050644ns | 2247110ns | -0.05% | 0.000 |
| abi_entry_form_tight_runtime_w | 2127265ns | 2055345ns | 2254206ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2086725ns | 2057569ns | 2135686ns | -1.91% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 66081.4 | 2076337.8 | 2074584.4 | n/a |
| abi_entry_form_tight_null_entry | 30508.8 | 5055.3 | 4982.1 | n/a |
| abi_entry_form_tight_per_w_set | 73441.9 | 2167186.3 | 2126150.6 | 1 |
| abi_entry_form_tight_runtime_w | 72212.8 | 2151439.1 | 2127264.8 | n/a |
| abi_entry_form_tight_scalar_anchor | 70012.4 | 2086131.0 | 2086725.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_tight_null_entry | 0.000 | 98.8% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.2% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.2% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2078066ns | 2078066ns | -2.48% |
| abi_entry_form_tight_null_entry | 7335ns | 7335ns | -99.66% |
| abi_entry_form_tight_per_w_set | 2129772ns | 2129772ns | -0.06% |
| abi_entry_form_tight_runtime_w | 2130956ns | 2130956ns | base |
| abi_entry_form_tight_scalar_anchor | 2090193ns | 2090193ns | -1.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2069359ns | base | --- | [2058230, 2254206] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2068897ns | no significant difference | [-161479, +9665]ns | [2057755, 2097101] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_tight_null_entry | 4901ns | -2064247.5ns (-99.8%) | [-2249272, -2053329]ns | [4849, 5197] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2078129ns | no significant difference | [-64610, +56816]ns | [2053212, 2247110] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_tight_scalar_anchor | 2066041ns | no significant difference | [-118520, +1437]ns | [2058448, 2135686] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2061114ns | +0.3% | -99.8% | +0.7% | -0.2% |
| 2 | 2363136ns | -12.4% | -99.8% | -4.8% | -9.1% |
| 3 | 2145276ns | -1.4% | -99.8% | +4.6% | -1.0% |
| 4 | 2072618ns | -0.5% | -99.7% | +0.4% | -0.3% |
| 5 | 2055345ns | -0.1% | -99.8% | +0.0% | +0.2% |
| 6 | 2066100ns | +0.6% | -99.8% | -0.7% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | -0.198 | ok |
| abi_entry_form_tight_null_entry | 0.011 | ok |
| abi_entry_form_tight_per_w_set | 0.258 | moderate+ |
| abi_entry_form_tight_runtime_w | -0.056 | ok |
| abi_entry_form_tight_scalar_anchor | 0.116 | ok |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_tight_scalar_anchor**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6304662.6ns | 2074584.4ns | 303.9% | HIGH |
| abi_entry_form_tight_null_entry | 127628.1ns | 4982.1ns | 2561.7% | HIGH |
| abi_entry_form_tight_per_w_set | 6517862.2ns | 2126150.6ns | 306.6% | HIGH |
| abi_entry_form_tight_runtime_w | 6504704.5ns | 2127264.8ns | 305.8% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6353817.4ns | 2086725.1ns | 304.5% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2052740.8-2097101.1 ns)
  2052740.8 |########################################
  2054958.8 |
  2057176.8 |
  2059394.8 |
  2061612.9 |########################################
  2063830.9 |
  2066048.9 |########################################
  2068266.9 |########################################
  2070484.9 |
  2072702.9 |
  2074920.9 |
  2077138.9 |########################################
  2079356.9 |
  2081575.0 |
  2083793.0 |
  2086011.0 |
  2088229.0 |
  2090447.0 |
  2092665.0 |
  2094883.0 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 4841.7-5196.6 ns)
   4841.7 |########################################
   4859.4 |
   4877.2 |
   4894.9 |########################################
   4912.7 |
   4930.4 |
   4948.2 |
   4965.9 |
   4983.7 |
   5001.4 |
   5019.2 |####################
   5036.9 |
   5054.7 |
   5072.4 |
   5090.2 |
   5107.9 |
   5125.7 |
   5143.4 |
   5161.2 |
   5178.9 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2050643.7-2247110.4 ns)
  2050643.7 |########################################
  2060467.0 |
  2070290.4 |####################
  2080113.7 |####################
  2089937.0 |
  2099760.4 |
  2109583.7 |
  2119407.0 |
  2129230.4 |
  2139053.7 |
  2148877.1 |
  2158700.4 |
  2168523.7 |
  2178347.1 |
  2188170.4 |
  2197993.7 |
  2207817.1 |
  2217640.4 |
  2227463.7 |
  2237287.1 |####################
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2055345.4-2254205.8 ns)
  2055345.4 |########################################
  2065288.4 |########################################
  2075231.4 |
  2085174.5 |
  2095117.5 |
  2105060.5 |
  2115003.5 |
  2124946.5 |
  2134889.6 |
  2144832.6 |####################
  2154775.6 |
  2164718.6 |
  2174661.6 |
  2184604.7 |
  2194547.7 |
  2204490.7 |
  2214433.7 |
  2224376.7 |
  2234319.8 |
  2244262.8 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2057569.2-2135686.0 ns)
  2057569.2 |########################################
  2061475.0 |####################
  2065380.9 |####################
  2069286.7 |
  2073192.6 |
  2077098.4 |
  2081004.3 |
  2084910.1 |
  2088815.9 |
  2092721.8 |
  2096627.6 |
  2100533.5 |
  2104439.3 |
  2108345.2 |
  2112251.0 |
  2116156.8 |
  2120062.7 |####################
  2123968.5 |
  2127874.4 |
  2131780.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=2573.9% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.3% of algo (FFI overhead may distort results)
