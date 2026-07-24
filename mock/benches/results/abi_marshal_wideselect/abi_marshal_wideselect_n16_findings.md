# abi_marshal (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_wideselect_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_wideselect_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_wideselect_marshal_null dominates: 7422% faster than the next best (abi_marshal_wideselect_aos)

abi_marshal_wideselect_marshal_null (27.59 us) leads abi_marshal_wideselect_aos (2.07 ms) by 7422%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_wideselect_marshal_null beats baseline by 99% (significant)

abi_marshal_wideselect_marshal_null is -2.05 ms (99%) faster than baseline abi_marshal_wideselect_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_wideselect_soa_transposed is an outlier: 77.5x slower than the field

abi_marshal_wideselect_soa_transposed (2.14 ms) is 77.5x the fastest (27.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_wideselect_marshal_null} vs {abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_native, abi_marshal_wideselect_soa_transposed} (7422% apart)

The field splits into a fast tier {abi_marshal_wideselect_marshal_null} and a slow tier {abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_native, abi_marshal_wideselect_soa_transposed} with a 7422% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 77.5x the fastest

Fastest abi_marshal_wideselect_marshal_null (27.59 us) to slowest abi_marshal_wideselect_soa_transposed (2.14 ms): 77.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_wideselect_marshal_null** at 27585.2 ns median (-98.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 77.47x (fastest 27585.2 ns, slowest 2136891.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2111888ns | 2077386ns | 2075115ns | 2077032ns | 2182559ns | base |
| abi_marshal_wideselect_marshal_null | 29671ns | 29929ns | 28518ns | 29667ns | 30252ns | -98.60% |
| abi_marshal_wideselect_soa_native | 2108066ns | 2100669ns | 2090225ns | 2097679ns | 2132567ns | -0.18% |
| abi_marshal_wideselect_soa_transposed | 2138255ns | 2139556ns | 2125788ns | 2137604ns | 2145466ns | +1.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2109293ns | 2072447ns | 2179888ns | base | 0.000 |
| abi_marshal_wideselect_marshal_null | 27379ns | 26329ns | 27932ns | -98.70% | 0.001 |
| abi_marshal_wideselect_soa_native | 2105393ns | 2087580ns | 2129641ns | -0.18% | 0.000 |
| abi_marshal_wideselect_soa_transposed | 2135644ns | 2123319ns | 2142782ns | +1.25% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 40723.1 | 2141927.6 | 2109292.9 | n/a |
| abi_marshal_wideselect_marshal_null | 28479.2 | 27949.1 | 27378.7 | n/a |
| abi_marshal_wideselect_soa_native | 45415.6 | 2104998.4 | 2105393.1 | 1 |
| abi_marshal_wideselect_soa_transposed | 42207.4 | 2135114.6 | 2135643.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_marshal_wideselect_marshal_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_wideselect_aos | 0.000 | 1.3% |
| abi_marshal_wideselect_marshal_null | 0.001 | 95.4% |
| abi_marshal_wideselect_soa_native | 0.000 | 1.3% |
| abi_marshal_wideselect_soa_transposed | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_wideselect_aos | 2111888ns | 2111888ns | base |
| abi_marshal_wideselect_marshal_null | 29671ns | 29671ns | -98.60% |
| abi_marshal_wideselect_soa_native | 2108066ns | 2108066ns | -0.18% |
| abi_marshal_wideselect_soa_transposed | 2138255ns | 2138255ns | +1.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2074843ns | base | --- | [2073148, 2179888] | --- | --- | --- | --- |
| abi_marshal_wideselect_marshal_null | 27585ns | -2047635.6ns (-98.7%) | [-2152231, -2045876]ns | [26618, 27932] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_wideselect_soa_native | 2098200ns | no significant difference | [-64336, +34350]ns | [2088338, 2129641] | no | 0.2188 | 0.2188 | 0 |
| abi_marshal_wideselect_soa_transposed | 2136891ns | no significant difference | [-45080, +67660]ns | [2127257, 2142782] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_wideselect_aos | abi_marshal_wideselect_marshal_null | abi_marshal_wideselect_soa_native | abi_marshal_wideselect_soa_transposed |
|---|---|---|---|---|
| 1 | 2082448ns | -98.7% | +0.2% | +2.7% |
| 2 | 2075428ns | -98.7% | +1.0% | +2.7% |
| 3 | 2073849ns | -98.7% | +0.7% | +2.4% |
| 4 | 2072447ns | -98.6% | +1.3% | +3.1% |
| 5 | 2277327ns | -98.8% | -5.9% | -6.1% |
| 6 | 2074258ns | -98.7% | +2.0% | +3.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_wideselect_aos | -0.255 | moderate- |
| abi_marshal_wideselect_marshal_null | -0.432 | moderate- |
| abi_marshal_wideselect_soa_native | 0.251 | moderate+ |
| abi_marshal_wideselect_soa_transposed | 0.180 | ok |

**Consistency summary:**

- **abi_marshal_wideselect_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_wideselect_soa_native**: won 1/6, lost 5/6
- **abi_marshal_wideselect_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 6349721.5ns | 2109292.9ns | 301.0% | HIGH |
| abi_marshal_wideselect_marshal_null | 192714.4ns | 27378.7ns | 703.9% | HIGH |
| abi_marshal_wideselect_soa_native | 6363877.5ns | 2105393.1ns | 302.3% | HIGH |
| abi_marshal_wideselect_soa_transposed | 6449653.0ns | 2135643.5ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_marshal_wideselect_aos (n=6, range 2072446.7-2179887.5 ns)
  2072446.7 |########################################
  2077818.7 |##########
  2083190.8 |
  2088562.8 |
  2093934.9 |
  2099306.9 |
  2104678.9 |
  2110051.0 |
  2115423.0 |
  2120795.1 |
  2126167.1 |
  2131539.1 |
  2136911.2 |
  2142283.2 |
  2147655.3 |
  2153027.3 |
  2158399.3 |
  2163771.4 |
  2169143.4 |
  2174515.5 |
  (0 below, 1 above range)

abi_marshal_wideselect_marshal_null (n=6, range 26328.7-27932.3 ns)
  26328.7 |########################################
  26408.9 |
  26489.1 |
  26569.2 |
  26649.4 |
  26729.6 |
  26809.8 |
  26890.0 |########################################
  26970.1 |
  27050.3 |
  27130.5 |
  27210.7 |
  27290.9 |
  27371.0 |
  27451.2 |########################################
  27531.4 |
  27611.6 |########################################
  27691.8 |
  27771.9 |########################################
  27852.1 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_native (n=6, range 2087580.0-2129641.0 ns)
  2087580.0 |########################################
  2089683.1 |
  2091786.1 |
  2093889.2 |
  2095992.2 |####################
  2098095.3 |####################
  2100198.3 |
  2102301.4 |
  2104404.4 |
  2106507.5 |
  2108610.5 |
  2110713.6 |
  2112816.6 |
  2114919.7 |####################
  2117022.7 |
  2119125.8 |
  2121228.8 |
  2123331.9 |
  2125434.9 |
  2127538.0 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_transposed (n=6, range 2123318.8-2142782.3 ns)
  2123318.8 |########################################
  2124292.0 |
  2125265.1 |
  2126238.3 |
  2127211.5 |
  2128184.7 |
  2129157.8 |
  2130131.0 |
  2131104.2 |########################################
  2132077.4 |
  2133050.5 |
  2134023.7 |
  2134996.9 |
  2135970.1 |########################################
  2136943.2 |########################################
  2137916.4 |
  2138889.6 |########################################
  2139862.8 |
  2140835.9 |
  2141809.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_wideselect_aos**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_marshal_null**: bridge=702.8% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_native**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)
