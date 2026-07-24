# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_real_null_entry dominates: 71129% faster than the next best (abi_entry_form_real_dispatch_table)

abi_entry_form_real_null_entry (3.07 us) leads abi_entry_form_real_dispatch_table (2.19 ms) by 71129%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.19 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_scalar_anchor is an outlier: 713.9x slower than the field

abi_entry_form_real_scalar_anchor (2.19 ms) is 713.9x the fastest (3.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_real_scalar_anchor shows alternating (throttle bounce) (autocorr -0.51)

abi_entry_form_real_scalar_anchor's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_dispatch_table, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w, abi_entry_form_real_scalar_anchor} (71129% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_dispatch_table, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w, abi_entry_form_real_scalar_anchor} with a 71129% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 713.9x the fastest

Fastest abi_entry_form_real_null_entry (3.07 us) to slowest abi_entry_form_real_scalar_anchor (2.19 ms): 713.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 3070.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 713.87x (fastest 3070.2 ns, slowest 2191721.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2204937ns | 2190671ns | 2171725ns | 2187280ns | 2248028ns | +0.49% |
| abi_entry_form_real_null_entry | 5385ns | 5393ns | 5218ns | 5354ns | 5515ns | -99.75% |
| abi_entry_form_real_per_w_set | 2234516ns | 2191504ns | 2175535ns | 2187045ns | 2335214ns | +1.83% |
| abi_entry_form_real_runtime_w | 2194283ns | 2194625ns | 2186909ns | 2194191ns | 2198107ns | base |
| abi_entry_form_real_scalar_anchor | 2286210ns | 2195299ns | 2184072ns | 2194128ns | 2475401ns | +4.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2201138ns | 2168331ns | 2243944ns | +0.48% | 0.000 |
| abi_entry_form_real_null_entry | 3081ns | 2980ns | 3168ns | -99.86% | 0.003 |
| abi_entry_form_real_per_w_set | 2230755ns | 2172187ns | 2331009ns | +1.83% | 0.000 |
| abi_entry_form_real_runtime_w | 2190563ns | 2183184ns | 2194329ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2281547ns | 2180732ns | 2468462ns | +4.15% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 78379.9 | 2196855.4 | 2201137.7 | n/a |
| abi_entry_form_real_null_entry | 30497.4 | 3195.6 | 3080.6 | n/a |
| abi_entry_form_real_per_w_set | 84384.3 | 2242837.1 | 2230754.7 | n/a |
| abi_entry_form_real_runtime_w | 73293.4 | 2190236.9 | 2190563.4 | n/a |
| abi_entry_form_real_scalar_anchor | 82508.2 | 2342603.6 | 2281547.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.003 | 97.1% |
| abi_entry_form_real_per_w_set | 0.000 | 0.1% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2204937ns | 2204937ns | +0.49% |
| abi_entry_form_real_null_entry | 5385ns | 5385ns | -99.75% |
| abi_entry_form_real_per_w_set | 2234516ns | 2234516ns | +1.83% |
| abi_entry_form_real_runtime_w | 2194283ns | 2194283ns | base |
| abi_entry_form_real_scalar_anchor | 2286210ns | 2286210ns | +4.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2190969ns | base | --- | [2186392, 2194329] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2186858ns | no significant difference | [-18575, +52975]ns | [2172611, 2243944] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_real_null_entry | 3070ns | -2187887.6ns (-99.9%) | [-2191294, -2183267]ns | [3004, 3168] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2187824ns | no significant difference | [-14614, +140040]ns | [2173431, 2331009] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_real_scalar_anchor | 2191722ns | no significant difference | [-3521, +277493]ns | [2184457, 2468462] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2192772ns | -0.7% | -99.9% | -0.8% | -0.2% |
| 2 | 2192169ns | +2.3% | -99.9% | +2.8% | +8.7% |
| 3 | 2195886ns | -0.5% | -99.9% | -0.5% | -0.1% |
| 4 | 2189769ns | +2.6% | -99.9% | +10.0% | +16.6% |
| 5 | 2183184ns | +0.3% | -99.9% | -0.5% | -0.1% |
| 6 | 2189600ns | -1.0% | -99.9% | +0.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.446 | moderate- |
| abi_entry_form_real_null_entry | 0.273 | moderate+ |
| abi_entry_form_real_per_w_set | -0.439 | moderate- |
| abi_entry_form_real_runtime_w | 0.227 | moderate+ |
| abi_entry_form_real_scalar_anchor | -0.506 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 3/6, lost 2/6
- **abi_entry_form_real_scalar_anchor**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6678436.3ns | 2201137.7ns | 303.4% | HIGH |
| abi_entry_form_real_null_entry | 122685.3ns | 3080.6ns | 3982.5% | HIGH |
| abi_entry_form_real_per_w_set | 6792336.9ns | 2230754.7ns | 304.5% | HIGH |
| abi_entry_form_real_runtime_w | 6649947.0ns | 2190563.4ns | 303.6% | HIGH |
| abi_entry_form_real_scalar_anchor | 6996393.9ns | 2281547.1ns | 306.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2168331.2-2243944.2 ns)
  2168331.2 |########################################
  2172111.8 |
  2175892.5 |########################################
  2179673.1 |
  2183453.8 |########################################
  2187234.4 |########################################
  2191015.1 |
  2194795.7 |
  2198576.4 |
  2202357.0 |
  2206137.7 |
  2209918.3 |
  2213699.0 |
  2217479.6 |
  2221260.3 |
  2225040.9 |
  2228821.6 |
  2232602.2 |
  2236382.9 |
  2240163.5 |########################################
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 2980.4-3167.7 ns)
   2980.4 |########################################
   2989.8 |
   2999.1 |
   3008.5 |
   3017.9 |
   3027.2 |########################################
   3036.6 |########################################
   3046.0 |
   3055.3 |
   3064.7 |
   3074.1 |
   3083.4 |
   3092.8 |########################################
   3102.1 |
   3111.5 |
   3120.9 |
   3130.2 |
   3139.6 |
   3149.0 |########################################
   3158.3 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2172187.1-2331009.2 ns)
  2172187.1 |########################################
  2180128.2 |####################
  2188069.3 |####################
  2196010.4 |
  2203951.5 |
  2211892.6 |
  2219833.7 |
  2227774.8 |
  2235715.9 |
  2243657.0 |
  2251598.2 |####################
  2259539.3 |
  2267480.4 |
  2275421.5 |
  2283362.6 |
  2291303.7 |
  2299244.8 |
  2307185.9 |
  2315127.0 |
  2323068.1 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2183184.2-2194328.8 ns)
  2183184.2 |####################
  2183741.4 |
  2184298.7 |
  2184855.9 |
  2185413.1 |
  2185970.3 |
  2186527.6 |
  2187084.8 |
  2187642.0 |
  2188199.2 |
  2188756.5 |
  2189313.7 |########################################
  2189870.9 |
  2190428.2 |
  2190985.4 |
  2191542.6 |
  2192099.8 |####################
  2192657.1 |####################
  2193214.3 |
  2193771.5 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2180732.1-2468462.3 ns)
  2180732.1 |########################################
  2195118.6 |
  2209505.1 |
  2223891.6 |
  2238278.1 |
  2252664.6 |
  2267051.2 |
  2281437.7 |
  2295824.2 |
  2310210.7 |
  2324597.2 |
  2338983.7 |
  2353370.2 |
  2367756.7 |
  2382143.2 |##########
  2396529.8 |
  2410916.3 |
  2425302.8 |
  2439689.3 |
  2454075.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=4013.4% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=303.6% of algo (FFI overhead may distort results)
