# abi_marshal (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_scatter_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_scatter_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_scatter_marshal_null dominates: 7769% faster than the next best (abi_marshal_scatter_aos)

abi_marshal_scatter_marshal_null (27.12 us) leads abi_marshal_scatter_aos (2.13 ms) by 7769%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_scatter_marshal_null beats baseline by 99% (significant)

abi_marshal_scatter_marshal_null is -2.11 ms (99%) faster than baseline abi_marshal_scatter_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_scatter_soa_transposed is an outlier: 81.0x slower than the field

abi_marshal_scatter_soa_transposed (2.20 ms) is 81.0x the fastest (27.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_scatter_aos shows alternating (throttle bounce) (autocorr -0.74)

abi_marshal_scatter_aos's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_scatter_marshal_null} vs {abi_marshal_scatter_aos, abi_marshal_scatter_soa_native, abi_marshal_scatter_soa_transposed} (7769% apart)

The field splits into a fast tier {abi_marshal_scatter_marshal_null} and a slow tier {abi_marshal_scatter_aos, abi_marshal_scatter_soa_native, abi_marshal_scatter_soa_transposed} with a 7769% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 81.0x the fastest

Fastest abi_marshal_scatter_marshal_null (27.12 us) to slowest abi_marshal_scatter_soa_transposed (2.20 ms): 81.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_scatter_marshal_null** at 27125.0 ns median (-98.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 80.98x (fastest 27125.0 ns, slowest 2196563.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2135476ns | 2137005ns | 2124110ns | 2134653ns | 2142393ns | base |
| abi_marshal_scatter_marshal_null | 29237ns | 29341ns | 28511ns | 29233ns | 29606ns | -98.63% |
| abi_marshal_scatter_soa_native | 2158312ns | 2157703ns | 2149155ns | 2155305ns | 2167402ns | +1.07% |
| abi_marshal_scatter_soa_transposed | 2198829ns | 2199199ns | 2191662ns | 2198535ns | 2202854ns | +2.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2132986ns | 2121704ns | 2139851ns | base | 0.000 |
| abi_marshal_scatter_marshal_null | 27001ns | 26275ns | 27345ns | -98.73% | 0.001 |
| abi_marshal_scatter_soa_native | 2155799ns | 2146665ns | 2164768ns | +1.07% | 0.000 |
| abi_marshal_scatter_soa_transposed | 2196234ns | 2189282ns | 2200191ns | +2.97% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 39384.4 | 2132983.7 | 2132986.3 | n/a |
| abi_marshal_scatter_marshal_null | 26120.2 | 27458.4 | 27000.8 | n/a |
| abi_marshal_scatter_soa_native | 39484.4 | 2155408.4 | 2155799.0 | n/a |
| abi_marshal_scatter_soa_transposed | 40379.6 | 2197402.1 | 2196233.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_marshal_scatter_marshal_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_scatter_aos | 0.000 | 1.2% |
| abi_marshal_scatter_marshal_null | 0.001 | 96.9% |
| abi_marshal_scatter_soa_native | 0.000 | 1.2% |
| abi_marshal_scatter_soa_transposed | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_scatter_aos | 2135476ns | 2135476ns | base |
| abi_marshal_scatter_marshal_null | 29237ns | 29237ns | -98.63% |
| abi_marshal_scatter_soa_native | 2158312ns | 2158312ns | +1.07% |
| abi_marshal_scatter_soa_transposed | 2198829ns | 2198829ns | +2.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2134500ns | base | --- | [2124608, 2139851] | --- | --- | --- | --- |
| abi_marshal_scatter_marshal_null | 27125ns | -2107822.9ns (-98.8%) | [-2112611, -2097522]ns | [26532, 27345] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_scatter_soa_native | 2155248ns | +27792.5ns (+1.3%) | [+7530, +33116]ns | [2147381, 2164768] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_scatter_soa_transposed | 2196563ns | +64111.5ns (+3.0%) | [+54739, +70892]ns | [2191947, 2200191] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_scatter_aos | abi_marshal_scatter_marshal_null | abi_marshal_scatter_soa_native | abi_marshal_scatter_soa_transposed |
|---|---|---|---|---|
| 1 | 2127511ns | -98.7% | +1.3% | +3.2% |
| 2 | 2140738ns | -98.7% | +0.3% | +2.3% |
| 3 | 2121704ns | -98.7% | +1.6% | +3.4% |
| 4 | 2138964ns | -98.7% | +0.4% | +2.8% |
| 5 | 2133521ns | -98.8% | +1.5% | +3.0% |
| 6 | 2135480ns | -98.7% | +1.3% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_scatter_aos | -0.743 | HIGH- (thermal bounce) |
| abi_marshal_scatter_marshal_null | -0.377 | moderate- |
| abi_marshal_scatter_soa_native | -0.007 | ok |
| abi_marshal_scatter_soa_transposed | 0.100 | ok |

**Consistency summary:**

- **abi_marshal_scatter_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_scatter_soa_native**: won 0/6, lost 6/6
- **abi_marshal_scatter_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 6442889.2ns | 2132986.3ns | 302.1% | HIGH |
| abi_marshal_scatter_marshal_null | 189363.6ns | 27000.8ns | 701.3% | HIGH |
| abi_marshal_scatter_soa_native | 6510041.4ns | 2155799.0ns | 302.0% | HIGH |
| abi_marshal_scatter_soa_transposed | 6631133.3ns | 2196233.8ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_scatter_aos (n=6, range 2121703.8-2139851.0 ns)
  2121703.8 |########################################
  2122611.2 |
  2123518.5 |
  2124425.9 |
  2125333.2 |
  2126240.6 |
  2127148.0 |########################################
  2128055.3 |
  2128962.7 |
  2129870.1 |
  2130777.4 |
  2131684.8 |
  2132592.1 |
  2133499.5 |########################################
  2134406.9 |
  2135314.2 |########################################
  2136221.6 |
  2137129.0 |
  2138036.3 |
  2138943.7 |########################################
  (0 below, 1 above range)

abi_marshal_scatter_marshal_null (n=6, range 26274.6-27345.4 ns)
  26274.6 |########################################
  26328.1 |
  26381.7 |
  26435.2 |
  26488.8 |
  26542.3 |
  26595.8 |
  26649.4 |
  26702.9 |
  26756.5 |########################################
  26810.0 |
  26863.5 |
  26917.1 |
  26970.6 |
  27024.2 |
  27077.7 |########################################
  27131.2 |########################################
  27184.8 |
  27238.3 |
  27291.9 |########################################
  (0 below, 1 above range)

abi_marshal_scatter_soa_native (n=6, range 2146664.6-2164768.4 ns)
  2146664.6 |########################################
  2147569.8 |########################################
  2148475.0 |
  2149380.2 |
  2150285.4 |
  2151190.5 |
  2152095.7 |
  2153000.9 |
  2153906.1 |########################################
  2154811.3 |########################################
  2155716.5 |
  2156621.7 |
  2157526.9 |
  2158432.0 |
  2159337.2 |
  2160242.4 |
  2161147.6 |
  2162052.8 |
  2162958.0 |########################################
  2163863.2 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_transposed (n=6, range 2189281.7-2200191.2 ns)
  2189281.7 |####################
  2189827.2 |
  2190372.7 |
  2190918.1 |
  2191463.6 |
  2192009.1 |
  2192554.6 |
  2193100.0 |
  2193645.5 |
  2194191.0 |####################
  2194736.5 |
  2195282.0 |
  2195827.4 |
  2196372.9 |########################################
  2196918.4 |
  2197463.9 |
  2198009.3 |
  2198554.8 |
  2199100.3 |
  2199645.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_scatter_aos**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_marshal_null**: bridge=700.2% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_native**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_transposed**: bridge=302.2% of algo (FFI overhead may distort results)
