# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 58336% faster than the next best (abi_zig_entry_scatter_zig_dispatch)

abi_zig_entry_scatter_zig_null (3.59 us) leads abi_zig_entry_scatter_zig_dispatch (2.10 ms) by 58336%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.10 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_runtime_w is an outlier: 1078.8x slower than the field

abi_zig_entry_scatter_zig_tail_runtime_w (3.87 ms) is 1078.8x the fastest (3.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} (58336% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} with a 58336% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1078.8x the fastest

Fastest abi_zig_entry_scatter_zig_null (3.59 us) to slowest abi_zig_entry_scatter_zig_tail_runtime_w (3.87 ms): 1078.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 3587.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1078.75x (fastest 3587.7 ns, slowest 3870242.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2121716ns | 2106834ns | 2102682ns | 2105783ns | 2155132ns | -0.06% |
| abi_zig_entry_scatter_zig_dispatch | 2098523ns | 2099099ns | 2088881ns | 2097980ns | 2104159ns | -1.15% |
| abi_zig_entry_scatter_zig_null | 5874ns | 5897ns | 5740ns | 5874ns | 5942ns | -99.72% |
| abi_zig_entry_scatter_zig_per_w_set | 2100390ns | 2103541ns | 2083610ns | 2101283ns | 2107441ns | -1.06% |
| abi_zig_entry_scatter_zig_runtime_w | 2122997ns | 2102568ns | 2091688ns | 2099208ns | 2174334ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3874181ns | 3858746ns | 3838297ns | 3855245ns | 3920526ns | +82.49% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3975795ns | 3872991ns | 3849671ns | 3870212ns | 4197233ns | +87.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2118934ns | 2099940ns | 2152065ns | -0.06% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2095953ns | 2086268ns | 2101686ns | -1.15% | 0.000 |
| abi_zig_entry_scatter_zig_null | 3582ns | 3502ns | 3624ns | -99.83% | 0.002 |
| abi_zig_entry_scatter_zig_per_w_set | 2097724ns | 2081183ns | 2104794ns | -1.06% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2120234ns | 2089057ns | 2171366ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3871214ns | 3835802ns | 3917451ns | +82.58% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3973009ns | 3846766ns | 4194447ns | +87.39% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 187701.8 | 2115778.0 | 2118934.0 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 174068.7 | 2097526.9 | 2095952.8 | n/a |
| abi_zig_entry_scatter_zig_null | 156253.3 | 3790.5 | 3581.9 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 180125.8 | 2099086.5 | 2097724.3 | n/a |
| abi_zig_entry_scatter_zig_runtime_w | 187292.5 | 2118000.5 | 2120234.1 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 201001.7 | 3875949.2 | 3871213.5 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 187206.9 | 3882336.1 | 3973008.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_null | 0.002 | 97.6% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2121716ns | 2121716ns | -0.06% |
| abi_zig_entry_scatter_zig_dispatch | 2098523ns | 2098523ns | -1.15% |
| abi_zig_entry_scatter_zig_null | 5874ns | 5874ns | -99.72% |
| abi_zig_entry_scatter_zig_per_w_set | 2100390ns | 2100390ns | -1.06% |
| abi_zig_entry_scatter_zig_runtime_w | 2122997ns | 2122997ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3874181ns | 3874181ns | +82.49% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3975795ns | 3975795ns | +87.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2099926ns | base | --- | [2089410, 2171366] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2104317ns | no significant difference | [-41676, +29121]ns | [2100420, 2152065] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2096515ns | no significant difference | [-80335, +9710]ns | [2089657, 2101686] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_null | 3588ns | -2096357.9ns (-99.8%) | [-2167784, -2085814]ns | [3534, 3624] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2100723ns | no significant difference | [-71861, +7995]ns | [2087655, 2104794] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3855702ns | +1753078.9ns (+83.5%) | [+1683715, +1816144]ns | [3840488, 3917451] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3870242ns | +1778308.1ns (+84.7%) | [+1753030, +2026986]ns | [3854336, 4194447] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2089057ns | +0.6% | +0.4% | -99.8% | +0.2% | +84.1% | +85.2% |
| 2 | 2106045ns | +2.0% | -0.9% | -99.8% | -0.3% | +88.8% | +83.4% |
| 3 | 2103284ns | -0.2% | -0.0% | -99.8% | -0.1% | +83.2% | +84.5% |
| 4 | 2089763ns | +0.8% | +0.5% | -99.8% | -0.4% | +83.6% | +85.3% |
| 5 | 2096568ns | +0.3% | -0.2% | -99.8% | +0.5% | +84.0% | +83.5% |
| 6 | 2236687ns | -3.6% | -6.3% | -99.8% | -6.0% | +72.5% | +101.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | -0.372 | moderate- |
| abi_zig_entry_scatter_zig_dispatch | -0.331 | moderate- |
| abi_zig_entry_scatter_zig_null | 0.253 | moderate+ |
| abi_zig_entry_scatter_zig_per_w_set | -0.464 | moderate- |
| abi_zig_entry_scatter_zig_runtime_w | -0.051 | ok |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.247 | moderate- |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.069 | ok |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 2/6, lost 4/6
- **abi_zig_entry_scatter_zig_dispatch**: won 3/6, lost 2/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 3/6, lost 2/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6622626.5ns | 2118934.0ns | 312.5% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6532061.4ns | 2095952.8ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_null | 305924.5ns | 3581.9ns | 8540.9% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6540145.0ns | 2097724.3ns | 311.8% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6607358.4ns | 2120234.1ns | 311.6% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11952887.4ns | 3871213.5ns | 308.8% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 11886336.2ns | 3973008.7ns | 299.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2099940.4-2152065.0 ns)
  2099940.4 |########################################
  2102546.6 |
  2105152.9 |#############
  2107759.1 |
  2110365.3 |
  2112971.5 |
  2115577.8 |
  2118184.0 |
  2120790.2 |
  2123396.5 |
  2126002.7 |
  2128608.9 |
  2131215.2 |
  2133821.4 |
  2136427.6 |
  2139033.9 |
  2141640.1 |
  2144246.3 |
  2146852.5 |#############
  2149458.8 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2086268.3-2101686.0 ns)
  2086268.3 |########################################
  2087039.2 |
  2087810.1 |
  2088581.0 |
  2089351.9 |
  2090122.7 |
  2090893.6 |
  2091664.5 |
  2092435.4 |########################################
  2093206.3 |
  2093977.2 |
  2094748.1 |
  2095518.9 |########################################
  2096289.8 |
  2097060.7 |########################################
  2097831.6 |
  2098602.5 |
  2099373.4 |
  2100144.3 |
  2100915.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 3501.7-3624.2 ns)
   3501.7 |########################################
   3507.8 |
   3513.9 |
   3520.1 |
   3526.2 |
   3532.3 |
   3538.4 |
   3544.6 |
   3550.7 |
   3556.8 |
   3562.9 |########################################
   3569.1 |
   3575.2 |########################################
   3581.3 |
   3587.4 |
   3593.6 |########################################
   3599.7 |
   3605.8 |
   3611.9 |########################################
   3618.1 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2081183.3-2104794.3 ns)
  2081183.3 |####################
  2082363.9 |
  2083544.4 |
  2084725.0 |
  2085905.5 |
  2087086.1 |
  2088266.6 |
  2089447.2 |
  2090627.7 |
  2091808.3 |
  2092988.8 |####################
  2094169.4 |
  2095349.9 |
  2096530.5 |
  2097711.0 |
  2098891.6 |####################
  2100072.1 |
  2101252.7 |########################################
  2102433.2 |
  2103613.8 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2089057.1-2171366.0 ns)
  2089057.1 |########################################
  2093172.5 |####################
  2097288.0 |
  2101403.4 |####################
  2105518.9 |####################
  2109634.3 |
  2113749.8 |
  2117865.2 |
  2121980.7 |
  2126096.1 |
  2130211.6 |
  2134327.0 |
  2138442.5 |
  2142557.9 |
  2146673.4 |
  2150788.8 |
  2154904.3 |
  2159019.7 |
  2163135.2 |
  2167250.6 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3835802.5-3917450.8 ns)
  3835802.5 |####################
  3839884.9 |
  3843967.3 |####################
  3848049.7 |
  3852132.2 |####################
  3856214.6 |########################################
  3860297.0 |
  3864379.4 |
  3868461.8 |
  3872544.2 |
  3876626.6 |
  3880709.1 |
  3884791.5 |
  3888873.9 |
  3892956.3 |
  3897038.7 |
  3901121.1 |
  3905203.6 |
  3909286.0 |
  3913368.4 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3846766.2-4194447.3 ns)
  3846766.2 |##########################
  3864150.3 |########################################
  3881534.3 |
  3898918.4 |
  3916302.4 |
  3933686.5 |
  3951070.5 |
  3968454.6 |
  3985838.6 |
  4003222.7 |
  4020606.8 |
  4037990.8 |
  4055374.9 |
  4072758.9 |
  4090143.0 |
  4107527.0 |
  4124911.1 |
  4142295.1 |
  4159679.2 |
  4177063.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=8500.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=307.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=307.0% of algo (FFI overhead may distort results)
