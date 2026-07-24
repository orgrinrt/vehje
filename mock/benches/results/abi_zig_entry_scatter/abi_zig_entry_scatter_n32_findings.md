# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 90229% faster than the next best (abi_zig_entry_scatter_zig_anchor)

abi_zig_entry_scatter_zig_null (2.32 us) leads abi_zig_entry_scatter_zig_anchor (2.09 ms) by 90229%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.10 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_dispatch is an outlier: 1669.8x slower than the field

abi_zig_entry_scatter_zig_tail_dispatch (3.87 ms) is 1669.8x the fastest (2.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_scatter_zig_per_w_set shows alternating (throttle bounce) (autocorr -0.59)

abi_zig_entry_scatter_zig_per_w_set's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} (90229% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} with a 90229% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1669.8x the fastest

Fastest abi_zig_entry_scatter_zig_null (2.32 us) to slowest abi_zig_entry_scatter_zig_tail_dispatch (3.87 ms): 1669.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 2318.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1669.82x (fastest 2318.6 ns, slowest 3871559.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2096834ns | 2096968ns | 2089372ns | 2094847ns | 2103546ns | -0.35% |
| abi_zig_entry_scatter_zig_dispatch | 2127601ns | 2107344ns | 2080719ns | 2104286ns | 2186016ns | +1.11% |
| abi_zig_entry_scatter_zig_null | 4627ns | 4636ns | 4508ns | 4633ns | 4678ns | -99.78% |
| abi_zig_entry_scatter_zig_per_w_set | 2102989ns | 2106120ns | 2090370ns | 2101927ns | 2110891ns | -0.06% |
| abi_zig_entry_scatter_zig_runtime_w | 2104237ns | 2107568ns | 2089015ns | 2103986ns | 2112225ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3972154ns | 3874577ns | 3853816ns | 3872630ns | 4180609ns | +88.77% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3894646ns | 3870478ns | 3857555ns | 3867148ns | 3954439ns | +85.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2094109ns | 2086470ns | 2100816ns | -0.35% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2124716ns | 2078217ns | 2182746ns | +1.11% | 0.000 |
| abi_zig_entry_scatter_zig_null | 2310ns | 2262ns | 2325ns | -99.89% | 0.014 |
| abi_zig_entry_scatter_zig_per_w_set | 2100198ns | 2087764ns | 2108033ns | -0.06% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2101483ns | 2086482ns | 2109266ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3969239ns | 3851042ns | 4177601ns | +88.88% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3891602ns | 3854697ns | 3951166ns | +85.18% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 175616.2 | 2094504.2 | 2094109.3 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 193266.6 | 2129714.3 | 2124716.2 | n/a |
| abi_zig_entry_scatter_zig_null | 158171.8 | 2612.0 | 2310.1 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 185051.8 | 2101677.2 | 2100198.3 | 5 |
| abi_zig_entry_scatter_zig_runtime_w | 178509.6 | 2103254.2 | 2101483.5 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 201793.1 | 3933495.4 | 3969238.9 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 205525.9 | 3889236.7 | 3891601.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_null | 0.014 | 97.6% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2096834ns | 2096834ns | -0.35% |
| abi_zig_entry_scatter_zig_dispatch | 2127601ns | 2127601ns | +1.11% |
| abi_zig_entry_scatter_zig_null | 4627ns | 4627ns | -99.78% |
| abi_zig_entry_scatter_zig_per_w_set | 2102989ns | 2102989ns | -0.06% |
| abi_zig_entry_scatter_zig_runtime_w | 2104237ns | 2104237ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3972154ns | 3972154ns | +88.77% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3894646ns | 3894646ns | +85.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2104821ns | base | --- | [2090364, 2109266] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2094320ns | no significant difference | [-16412, +4708]ns | [2087191, 2100816] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2104543ns | no significant difference | [-14402, +85880]ns | [2086859, 2182746] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_null | 2319ns | -2102501.0ns (-99.9%) | [-2106947, -2088072]ns | [2287, 2325] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2103334ns | no significant difference | [-20038, +14934]ns | [2089228, 2108033] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3871560ns | +1769718.6ns (+84.1%) | [+1752055, +2081493]ns | [3858556, 4177601] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3867534ns | +1760692.1ns (+83.7%) | [+1753691, +1855972]ns | [3856105, 3951166] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2094245ns | -0.4% | -0.8% | -99.9% | +0.7% | +84.6% | +84.1% |
| 2 | 2110952ns | -0.8% | -0.6% | -99.9% | -1.1% | +83.4% | +83.3% |
| 3 | 2086482ns | +0.8% | +8.0% | -99.9% | +0.7% | +114.3% | +92.9% |
| 4 | 2105735ns | -0.3% | +0.2% | -99.9% | -0.0% | +84.5% | +83.6% |
| 5 | 2107580ns | -0.6% | +0.2% | -99.9% | -0.8% | +82.7% | +83.0% |
| 6 | 2103908ns | -0.8% | -0.4% | -99.9% | +0.1% | +84.0% | +84.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.218 | moderate+ |
| abi_zig_entry_scatter_zig_dispatch | -0.166 | ok |
| abi_zig_entry_scatter_zig_null | -0.024 | ok |
| abi_zig_entry_scatter_zig_per_w_set | -0.585 | HIGH- (thermal bounce) |
| abi_zig_entry_scatter_zig_runtime_w | -0.546 | HIGH- (thermal bounce) |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.198 | ok |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.196 | ok |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_scatter_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 2/6, lost 3/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6523967.8ns | 2094109.3ns | 311.5% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6652840.0ns | 2124716.2ns | 313.1% | HIGH |
| abi_zig_entry_scatter_zig_null | 300287.8ns | 2310.1ns | 12999.1% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6556883.5ns | 2100198.3ns | 312.2% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6551140.7ns | 2101483.5ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 12038569.1ns | 3969238.9ns | 303.3% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 12037518.8ns | 3891601.7ns | 309.3% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2086470.4-2100816.5 ns)
  2086470.4 |########################################
  2087187.7 |
  2087905.0 |########################################
  2088622.3 |
  2089339.6 |
  2090056.9 |
  2090774.2 |
  2091491.5 |
  2092208.8 |
  2092926.1 |
  2093643.4 |########################################
  2094360.8 |########################################
  2095078.1 |
  2095795.4 |
  2096512.7 |
  2097230.0 |
  2097947.3 |
  2098664.6 |########################################
  2099381.9 |
  2100099.2 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2078216.7-2182746.5 ns)
  2078216.7 |####################
  2083443.2 |
  2088669.7 |
  2093896.2 |########################################
  2099122.6 |
  2104349.1 |
  2109575.6 |########################################
  2114802.1 |
  2120028.6 |
  2125255.1 |
  2130481.6 |
  2135708.1 |
  2140934.6 |
  2146161.0 |
  2151387.5 |
  2156614.0 |
  2161840.5 |
  2167067.0 |
  2172293.5 |
  2177520.0 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 2262.5-2325.0 ns)
   2262.5 |####################
   2265.6 |
   2268.8 |
   2271.9 |
   2275.0 |
   2278.1 |
   2281.2 |
   2284.4 |
   2287.5 |
   2290.6 |
   2293.8 |
   2296.9 |
   2300.0 |
   2303.1 |
   2306.2 |
   2309.4 |####################
   2312.5 |
   2315.6 |####################
   2318.8 |########################################
   2321.9 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2087764.2-2108032.9 ns)
  2087764.2 |########################################
  2088777.6 |
  2089791.1 |########################################
  2090804.5 |
  2091817.9 |
  2092831.4 |
  2093844.8 |
  2094858.2 |
  2095871.7 |
  2096885.1 |
  2097898.5 |
  2098912.0 |
  2099925.4 |
  2100938.9 |########################################
  2101952.3 |
  2102965.7 |
  2103979.2 |
  2104992.6 |########################################
  2106006.0 |########################################
  2107019.5 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2086482.1-2109265.9 ns)
  2086482.1 |########################################
  2087621.3 |
  2088760.5 |
  2089899.7 |
  2091038.9 |
  2092178.0 |
  2093317.2 |########################################
  2094456.4 |
  2095595.6 |
  2096734.8 |
  2097874.0 |
  2099013.2 |
  2100152.4 |
  2101291.5 |
  2102430.7 |
  2103569.9 |########################################
  2104709.1 |########################################
  2105848.3 |
  2106987.5 |########################################
  2108126.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3851041.7-4177601.5 ns)
  3851041.7 |########################################
  3867369.7 |########################################
  3883697.7 |####################
  3900025.7 |
  3916353.7 |
  3932681.6 |
  3949009.6 |
  3965337.6 |
  3981665.6 |
  3997993.6 |
  4014321.6 |
  4030649.6 |
  4046977.6 |
  4063305.5 |
  4079633.5 |
  4095961.5 |
  4112289.5 |
  4128617.5 |
  4144945.5 |
  4161273.5 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3854697.1-3951166.5 ns)
  3854697.1 |########################################
  3859520.6 |
  3864344.0 |########################################
  3869167.5 |
  3873991.0 |####################
  3878814.4 |
  3883637.9 |
  3888461.4 |
  3893284.8 |
  3898108.3 |
  3902931.8 |
  3907755.2 |
  3912578.7 |
  3917402.2 |
  3922225.6 |
  3927049.1 |
  3931872.6 |
  3936696.0 |
  3941519.5 |
  3946343.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=13002.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=307.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=307.0% of algo (FFI overhead may distort results)
