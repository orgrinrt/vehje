# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_wideselect_null_entry dominates: 82215% faster than the next best (abi_entry_form_wideselect_dispatch_table)

abi_entry_form_wideselect_null_entry (2.51 us) leads abi_entry_form_wideselect_dispatch_table (2.07 ms) by 82215%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.07 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_per_w_set is an outlier: 829.8x slower than the field

abi_entry_form_wideselect_per_w_set (2.08 ms) is 829.8x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_per_w_set} (82215% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_per_w_set} with a 82215% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 829.8x the fastest

Fastest abi_entry_form_wideselect_null_entry (2.51 us) to slowest abi_entry_form_wideselect_per_w_set (2.08 ms): 829.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 2512.5 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 829.80x (fastest 2512.5 ns, slowest 2084883.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2092447ns | 2071257ns | 2064191ns | 2069223ns | 2141411ns | +0.17% |
| abi_entry_form_wideselect_null_entry | 4817ns | 4776ns | 4639ns | 4769ns | 4979ns | -99.77% |
| abi_entry_form_wideselect_per_w_set | 2145047ns | 2087996ns | 2072835ns | 2084135ns | 2272520ns | +2.69% |
| abi_entry_form_wideselect_runtime_w | 2088908ns | 2077656ns | 2063835ns | 2077042ns | 2119242ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2233705ns | 2073665ns | 2067772ns | 2072697ns | 2558183ns | +6.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2089272ns | 2061278ns | 2137744ns | +0.17% | 0.000 |
| abi_entry_form_wideselect_null_entry | 2537ns | 2459ns | 2615ns | -99.88% | 0.006 |
| abi_entry_form_wideselect_per_w_set | 2141715ns | 2069705ns | 2268749ns | +2.69% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2085625ns | 2060781ns | 2115518ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2230280ns | 2064848ns | 2553802ns | +6.94% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 63996.4 | 2094959.4 | 2089272.3 | 0 |
| abi_entry_form_wideselect_null_entry | 28536.1 | 2645.1 | 2537.4 | n/a |
| abi_entry_form_wideselect_per_w_set | 66065.2 | 2099195.1 | 2141715.1 | n/a |
| abi_entry_form_wideselect_runtime_w | 65256.2 | 2086922.0 | 2085625.3 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 67281.7 | 2257798.3 | 2230280.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_wideselect_null_entry | 0.006 | 97.9% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.1% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.1% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2092447ns | 2092447ns | +0.17% |
| abi_entry_form_wideselect_null_entry | 4817ns | 4817ns | -99.77% |
| abi_entry_form_wideselect_per_w_set | 2145047ns | 2145047ns | +2.69% |
| abi_entry_form_wideselect_runtime_w | 2088908ns | 2088908ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2233705ns | 2233705ns | +6.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2074592ns | base | --- | [2066766, 2115518] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2068158ns | no significant difference | [-11977, +34022]ns | [2061915, 2137744] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_wideselect_null_entry | 2512ns | -2071977.5ns (-99.9%) | [-2113033, -2064254]ns | [2485, 2615] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2084884ns | no significant difference | [-2436, +157469]ns | [2071512, 2268749] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2070664ns | no significant difference | [-12759, +450080]ns | [2066374, 2553802] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2060781ns | +0.6% | -99.9% | +0.6% | +0.5% |
| 2 | 2072751ns | -0.6% | -99.9% | +0.5% | -0.2% |
| 3 | 2073288ns | -0.5% | -99.9% | +0.7% | -0.1% |
| 4 | 2084372ns | -0.6% | -99.9% | -0.7% | -0.9% |
| 5 | 2146662ns | +2.6% | -99.9% | +4.7% | +41.4% |
| 6 | 2075898ns | -0.6% | -99.9% | +10.4% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.202 | moderate- |
| abi_entry_form_wideselect_null_entry | -0.366 | moderate- |
| abi_entry_form_wideselect_per_w_set | 0.389 | moderate+ |
| abi_entry_form_wideselect_runtime_w | -0.037 | ok |
| abi_entry_form_wideselect_scalar_anchor | -0.237 | moderate- |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 1/6, lost 5/6
- **abi_entry_form_wideselect_scalar_anchor**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6353036.8ns | 2089272.3ns | 304.1% | HIGH |
| abi_entry_form_wideselect_null_entry | 117809.0ns | 2537.4ns | 4642.8% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6432565.7ns | 2141715.1ns | 300.3% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6326159.2ns | 2085625.3ns | 303.3% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6849736.5ns | 2230280.1ns | 307.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2061277.9-2137743.5 ns)
  2061277.9 |########################################
  2065101.2 |
  2068924.5 |#############
  2072747.7 |#############
  2076571.0 |
  2080394.3 |
  2084217.6 |
  2088040.9 |
  2091864.2 |
  2095687.4 |
  2099510.7 |
  2103334.0 |
  2107157.3 |
  2110980.6 |
  2114803.9 |
  2118627.1 |
  2122450.4 |
  2126273.7 |
  2130097.0 |
  2133920.3 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 2459.2-2615.0 ns)
   2459.2 |####################
   2467.0 |
   2474.8 |
   2482.6 |
   2490.4 |
   2498.1 |
   2505.9 |########################################
   2513.7 |########################################
   2521.5 |
   2529.3 |
   2537.1 |
   2544.9 |
   2552.7 |
   2560.5 |
   2568.3 |
   2576.1 |
   2583.8 |
   2591.6 |
   2599.4 |
   2607.2 |
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2069705.0-2268749.0 ns)
  2069705.0 |########################################
  2079657.2 |########################################
  2089609.4 |
  2099561.6 |
  2109513.8 |
  2119466.0 |
  2129418.2 |
  2139370.4 |
  2149322.6 |
  2159274.8 |
  2169227.0 |
  2179179.2 |
  2189131.4 |
  2199083.6 |
  2209035.8 |
  2218988.0 |
  2228940.2 |
  2238892.4 |####################
  2248844.6 |
  2258796.8 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2060780.8-2115517.5 ns)
  2060780.8 |####################
  2063517.6 |
  2066254.5 |
  2068991.3 |
  2071728.1 |########################################
  2074465.0 |####################
  2077201.8 |
  2079938.6 |
  2082675.5 |####################
  2085412.3 |
  2088149.1 |
  2090886.0 |
  2093622.8 |
  2096359.7 |
  2099096.5 |
  2101833.3 |
  2104570.2 |
  2107307.0 |
  2110043.8 |
  2112780.7 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2064847.5-2553801.9 ns)
  2064847.5 |########################################
  2089295.2 |
  2113742.9 |
  2138190.7 |
  2162638.4 |
  2187086.1 |
  2211533.8 |
  2235981.5 |
  2260429.2 |
  2284877.0 |
  2309324.7 |
  2333772.4 |
  2358220.1 |
  2382667.8 |
  2407115.5 |
  2431563.3 |
  2456011.0 |
  2480458.7 |
  2504906.4 |
  2529354.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=4669.4% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=302.7% of algo (FFI overhead may distort results)
