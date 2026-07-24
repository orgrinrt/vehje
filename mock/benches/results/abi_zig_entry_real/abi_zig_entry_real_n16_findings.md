# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 83392% faster than the next best (abi_zig_entry_real_zig_dispatch)

abi_zig_entry_real_zig_null (2.47 us) leads abi_zig_entry_real_zig_dispatch (2.07 ms) by 83392%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.07 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 1558.5x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.86 ms) is 1558.5x the fastest (2.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (83392% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 83392% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1558.5x the fastest

Fastest abi_zig_entry_real_zig_null (2.47 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.86 ms): 1558.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 2474.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1558.51x (fastest 2474.8 ns, slowest 3856915.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2076972ns | 2077830ns | 2068370ns | 2077457ns | 2080546ns | -0.18% |
| abi_zig_entry_real_zig_dispatch | 2123357ns | 2068818ns | 2063834ns | 2067471ns | 2236948ns | +2.05% |
| abi_zig_entry_real_zig_null | 4787ns | 4793ns | 4692ns | 4769ns | 4861ns | -99.77% |
| abi_zig_entry_real_zig_per_w_set | 2245265ns | 2082992ns | 2074392ns | 2081279ns | 2576681ns | +7.91% |
| abi_zig_entry_real_zig_runtime_w | 2080652ns | 2072158ns | 2060445ns | 2071527ns | 2104442ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3647384ns | 3849875ns | 3113685ns | 3605656ns | 3976826ns | +75.30% |
| abi_zig_entry_real_zig_tail_runtime_w | 3858020ns | 3859686ns | 3840359ns | 3858432ns | 3866234ns | +85.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2074317ns | 2065733ns | 2077884ns | -0.17% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2120545ns | 2061252ns | 2233670ns | +2.05% | 0.000 |
| abi_zig_entry_real_zig_null | 2482ns | 2460ns | 2511ns | -99.88% | 0.006 |
| abi_zig_entry_real_zig_per_w_set | 2242414ns | 2071732ns | 2573493ns | +7.91% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2077951ns | 2057969ns | 2101525ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3644416ns | 3110634ns | 3973627ns | +75.39% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3855169ns | 3837592ns | 3863322ns | +85.53% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 182268.3 | 2074681.7 | 2074317.3 | n/a |
| abi_zig_entry_real_zig_dispatch | 201592.9 | 2118778.8 | 2120544.5 | n/a |
| abi_zig_entry_real_zig_null | 155311.7 | 2705.2 | 2482.4 | n/a |
| abi_zig_entry_real_zig_per_w_set | 197314.4 | 2099243.9 | 2242413.7 | n/a |
| abi_zig_entry_real_zig_runtime_w | 185946.2 | 2082957.5 | 2077950.8 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 207711.8 | 3646402.3 | 3644415.9 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 197046.1 | 3861508.1 | 3855169.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_null | 0.006 | 99.4% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2076972ns | 2076972ns | -0.18% |
| abi_zig_entry_real_zig_dispatch | 2123357ns | 2123357ns | +2.05% |
| abi_zig_entry_real_zig_null | 4787ns | 4787ns | -99.77% |
| abi_zig_entry_real_zig_per_w_set | 2245265ns | 2245265ns | +7.91% |
| abi_zig_entry_real_zig_runtime_w | 2080652ns | 2080652ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3647384ns | 3647384ns | +75.30% |
| abi_zig_entry_real_zig_tail_runtime_w | 3858020ns | 3858020ns | +85.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2069507ns | base | --- | [2062820, 2101525] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2075169ns | no significant difference | [-25661, +13531]ns | [2069899, 2077884] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_dispatch | 2066214ns | no significant difference | [-31024, +165925]ns | [2061750, 2233670] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_real_zig_null | 2475ns | -2067024.2ns (-99.9%) | [-2099064, -2060317]ns | [2461, 2511] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2080280ns | no significant difference | [-598, +482144]ns | [2073468, 2573493] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3847044ns | +1777536.8ns (+85.9%) | [+1044432, +1877427]ns | [3112577, 3973627] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3856915ns | +1783839.8ns (+86.2%) | [+1754132, +1793683]ns | [3845270, 3863322] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2057969ns | +1.0% | +16.4% | -99.9% | +25.8% | +51.3% | +86.5% |
| 2 | 2078321ns | -0.2% | -0.5% | -99.9% | -0.1% | +49.7% | +85.9% |
| 3 | 2067672ns | +0.3% | -0.3% | -99.9% | +0.8% | +86.6% | +86.7% |
| 4 | 2069812ns | -0.2% | -0.4% | -99.9% | +0.1% | +86.0% | +86.1% |
| 5 | 2124729ns | -2.2% | -2.5% | -99.9% | +20.4% | +92.5% | +81.4% |
| 6 | 2069202ns | +0.3% | -0.3% | -99.9% | +0.4% | +85.8% | +86.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.221 | moderate- |
| abi_zig_entry_real_zig_dispatch | -0.023 | ok |
| abi_zig_entry_real_zig_null | -0.142 | ok |
| abi_zig_entry_real_zig_per_w_set | -0.336 | moderate- |
| abi_zig_entry_real_zig_runtime_w | -0.253 | moderate- |
| abi_zig_entry_real_zig_tail_dispatch | 0.441 | moderate+ |
| abi_zig_entry_real_zig_tail_runtime_w | -0.246 | moderate- |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 3/6, lost 3/6
- **abi_zig_entry_real_zig_dispatch**: won 5/6, lost 1/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 1/6, lost 4/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6476330.2ns | 2074317.3ns | 312.2% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6623752.1ns | 2120544.5ns | 312.4% | HIGH |
| abi_zig_entry_real_zig_null | 299947.4ns | 2482.4ns | 12082.9% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6574471.0ns | 2242413.7ns | 293.2% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6535829.7ns | 2077950.8ns | 314.5% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 11235776.4ns | 3644415.9ns | 308.3% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11840132.0ns | 3855169.0ns | 307.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2065732.9-2077883.9 ns)
  2065732.9 |####################
  2066340.5 |
  2066948.0 |
  2067555.6 |
  2068163.1 |
  2068770.7 |
  2069378.2 |
  2069985.8 |
  2070593.3 |
  2071200.9 |
  2071808.4 |
  2072416.0 |
  2073023.5 |
  2073631.1 |########################################
  2074238.6 |
  2074846.2 |
  2075453.7 |
  2076061.3 |####################
  2076668.8 |
  2077276.4 |####################
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2061251.7-2233669.8 ns)
  2061251.7 |########################################
  2069872.6 |##########
  2078493.5 |
  2087114.4 |
  2095735.3 |
  2104356.2 |
  2112977.1 |
  2121598.0 |
  2130218.9 |
  2138839.8 |
  2147460.8 |
  2156081.7 |
  2164702.6 |
  2173323.5 |
  2181944.4 |
  2190565.3 |
  2199186.2 |
  2207807.1 |
  2216428.0 |
  2225048.9 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 2459.6-2511.2 ns)
   2459.6 |########################################
   2462.2 |########################################
   2464.8 |
   2467.3 |
   2469.9 |
   2472.5 |########################################
   2475.1 |########################################
   2477.7 |
   2480.3 |
   2482.8 |
   2485.4 |
   2488.0 |
   2490.6 |########################################
   2493.2 |
   2495.8 |
   2498.3 |
   2500.9 |
   2503.5 |
   2506.1 |
   2508.7 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2071731.7-2573492.9 ns)
  2071731.7 |########################################
  2096819.8 |
  2121907.8 |
  2146995.9 |
  2172083.9 |
  2197172.0 |
  2222260.1 |
  2247348.1 |
  2272436.2 |
  2297524.2 |
  2322612.3 |
  2347700.4 |
  2372788.4 |
  2397876.5 |
  2422964.5 |
  2448052.6 |
  2473140.7 |
  2498228.7 |
  2523316.8 |
  2548404.8 |##########
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2057968.7-2101525.0 ns)
  2057968.7 |####################
  2060146.5 |
  2062324.3 |
  2064502.1 |
  2066680.0 |####################
  2068857.8 |########################################
  2071035.6 |
  2073213.4 |
  2075391.2 |
  2077569.0 |####################
  2079746.9 |
  2081924.7 |
  2084102.5 |
  2086280.3 |
  2088458.1 |
  2090635.9 |
  2092813.7 |
  2094991.6 |
  2097169.4 |
  2099347.2 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3110634.2-3973626.9 ns)
  3110634.2 |##########################
  3153783.8 |
  3196933.5 |
  3240083.1 |
  3283232.7 |
  3326382.4 |
  3369532.0 |
  3412681.6 |
  3455831.3 |
  3498980.9 |
  3542130.5 |
  3585280.2 |
  3628429.8 |
  3671579.5 |
  3714729.1 |
  3757878.7 |
  3801028.4 |
  3844178.0 |########################################
  3887327.6 |
  3930477.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3837592.5-3863321.6 ns)
  3837592.5 |########################################
  3838879.0 |
  3840165.4 |
  3841451.9 |
  3842738.3 |
  3844024.8 |
  3845311.2 |
  3846597.7 |
  3847884.2 |
  3849170.6 |
  3850457.1 |
  3851743.5 |########################################
  3853030.0 |########################################
  3854316.4 |
  3855602.9 |
  3856889.4 |
  3858175.8 |
  3859462.3 |########################################
  3860748.7 |
  3862035.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=12139.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=307.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.0% of algo (FFI overhead may distort results)
