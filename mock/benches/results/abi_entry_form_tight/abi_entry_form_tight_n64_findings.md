# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 80989% faster than the next best (abi_entry_form_tight_dispatch_table)

abi_entry_form_tight_null_entry (2.49 us) leads abi_entry_form_tight_dispatch_table (2.02 ms) by 80989%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.02 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 822.0x slower than the field

abi_entry_form_tight_scalar_anchor (2.04 ms) is 822.0x the fastest (2.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_scalar_anchor} (80989% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_scalar_anchor} with a 80989% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 822.0x the fastest

Fastest abi_entry_form_tight_null_entry (2.49 us) to slowest abi_entry_form_tight_scalar_anchor (2.04 ms): 822.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 2486.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 821.98x (fastest 2486.4 ns, slowest 2043823.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2019154ns | 2019377ns | 2007809ns | 2017514ns | 2027287ns | -3.58% |
| abi_entry_form_tight_null_entry | 4777ns | 4779ns | 4638ns | 4761ns | 4872ns | -99.77% |
| abi_entry_form_tight_per_w_set | 2171405ns | 2033133ns | 2017381ns | 2028963ns | 2462080ns | +3.69% |
| abi_entry_form_tight_runtime_w | 2094033ns | 2029552ns | 2018468ns | 2027260ns | 2231975ns | base |
| abi_entry_form_tight_scalar_anchor | 2104682ns | 2047045ns | 2034083ns | 2043578ns | 2231638ns | +0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2016171ns | 2005130ns | 2024198ns | -3.55% | 0.000 |
| abi_entry_form_tight_null_entry | 2483ns | 2421ns | 2527ns | -99.88% | 0.026 |
| abi_entry_form_tight_per_w_set | 2168226ns | 2014708ns | 2458539ns | +3.72% | 0.000 |
| abi_entry_form_tight_runtime_w | 2090399ns | 2015238ns | 2227823ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2101271ns | 2031299ns | 2227459ns | +0.52% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 53941.4 | 2016305.5 | 2016171.3 | n/a |
| abi_entry_form_tight_null_entry | 27657.4 | 2733.8 | 2482.6 | n/a |
| abi_entry_form_tight_per_w_set | 61415.4 | 2075633.3 | 2168225.8 | 0 |
| abi_entry_form_tight_runtime_w | 66684.9 | 2045492.0 | 2090399.4 | n/a |
| abi_entry_form_tight_scalar_anchor | 67100.7 | 2073796.6 | 2101270.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_tight_null_entry | 0.026 | 97.4% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.1% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.1% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2019154ns | 2019154ns | -3.58% |
| abi_entry_form_tight_null_entry | 4777ns | 4777ns | -99.77% |
| abi_entry_form_tight_per_w_set | 2171405ns | 2171405ns | +3.69% |
| abi_entry_form_tight_runtime_w | 2094033ns | 2094033ns | base |
| abi_entry_form_tight_scalar_anchor | 2104682ns | 2104682ns | +0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2025956ns | base | --- | [2017418, 2227823] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2016232ns | no significant difference | [-219740, +2129]ns | [2008084, 2024198] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_tight_null_entry | 2486ns | -2023470.0ns (-99.9%) | [-2225296, -2014984]ns | [2434, 2527] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2030000ns | no significant difference | [-5808, +230716]ns | [2016138, 2458539] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_scalar_anchor | 2043823ns | no significant difference | [-18895, +37656]ns | [2032529, 2227459] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2015238ns | -0.2% | -99.9% | -0.0% | +1.7% |
| 2 | 2098682ns | -4.2% | -99.9% | +5.8% | +2.0% |
| 3 | 2028793ns | -0.3% | -99.9% | -0.3% | +0.2% |
| 4 | 2023120ns | -0.1% | -99.9% | -0.3% | +0.8% |
| 5 | 2356965ns | -14.9% | -99.9% | +14.4% | -1.8% |
| 6 | 2019599ns | +0.3% | -99.9% | +0.9% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | -0.436 | moderate- |
| abi_entry_form_tight_null_entry | -0.398 | moderate- |
| abi_entry_form_tight_per_w_set | -0.390 | moderate- |
| abi_entry_form_tight_runtime_w | -0.375 | moderate- |
| abi_entry_form_tight_scalar_anchor | -0.455 | moderate- |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 5/6, lost 1/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_tight_scalar_anchor**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6108136.3ns | 2016171.3ns | 303.0% | HIGH |
| abi_entry_form_tight_null_entry | 113107.7ns | 2482.6ns | 4556.0% | HIGH |
| abi_entry_form_tight_per_w_set | 6275286.0ns | 2168225.8ns | 289.4% | HIGH |
| abi_entry_form_tight_runtime_w | 6247253.8ns | 2090399.4ns | 298.9% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6298731.2ns | 2101270.6ns | 299.8% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2005129.6-2024197.9 ns)
  2005129.6 |####################
  2006083.0 |
  2007036.4 |
  2007989.8 |
  2008943.3 |
  2009896.7 |
  2010850.1 |########################################
  2011803.5 |
  2012756.9 |
  2013710.3 |
  2014663.8 |
  2015617.2 |
  2016570.6 |
  2017524.0 |
  2018477.4 |
  2019430.8 |
  2020384.2 |####################
  2021337.7 |####################
  2022291.1 |
  2023244.5 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 2420.8-2527.3 ns)
   2420.8 |########################################
   2426.1 |
   2431.5 |
   2436.8 |
   2442.1 |
   2447.4 |########################################
   2452.8 |
   2458.1 |
   2463.4 |
   2468.7 |
   2474.1 |########################################
   2479.4 |
   2484.7 |
   2490.0 |########################################
   2495.4 |
   2500.7 |
   2506.0 |
   2511.3 |
   2516.7 |########################################
   2522.0 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2014707.9-2458539.4 ns)
  2014707.9 |########################################
  2036899.5 |#############
  2059091.0 |
  2081282.6 |
  2103474.2 |
  2125665.8 |
  2147857.4 |
  2170048.9 |
  2192240.5 |
  2214432.1 |#############
  2236623.6 |
  2258815.2 |
  2281006.8 |
  2303198.4 |
  2325389.9 |
  2347581.5 |
  2369773.1 |
  2391964.7 |
  2414156.2 |
  2436347.8 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2015237.9-2227823.4 ns)
  2015237.9 |########################################
  2025867.2 |#############
  2036496.4 |
  2047125.7 |
  2057755.0 |
  2068384.3 |
  2079013.5 |
  2089642.8 |#############
  2100272.1 |
  2110901.4 |
  2121530.6 |
  2132159.9 |
  2142789.2 |
  2153418.4 |
  2164047.7 |
  2174677.0 |
  2185306.3 |
  2195935.5 |
  2206564.8 |
  2217194.1 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2031299.2-2227459.1 ns)
  2031299.2 |########################################
  2041107.2 |#############
  2050915.2 |
  2060723.2 |
  2070531.2 |
  2080339.2 |
  2090147.2 |
  2099955.2 |
  2109763.2 |
  2119571.2 |
  2129379.2 |
  2139187.2 |#############
  2148995.2 |
  2158803.2 |
  2168611.2 |
  2178419.2 |
  2188227.2 |
  2198035.2 |
  2207843.2 |
  2217651.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=4537.6% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.0% of algo (FFI overhead may distort results)
