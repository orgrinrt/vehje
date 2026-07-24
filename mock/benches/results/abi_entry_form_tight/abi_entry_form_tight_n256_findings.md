# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 62841% faster than the next best (abi_entry_form_tight_runtime_w)

abi_entry_form_tight_null_entry (3.20 us) leads abi_entry_form_tight_runtime_w (2.01 ms) by 62841%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.01 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 635.5x slower than the field

abi_entry_form_tight_scalar_anchor (2.03 ms) is 635.5x the fastest (3.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_runtime_w, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_per_w_set, abi_entry_form_tight_scalar_anchor} (62841% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_runtime_w, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_per_w_set, abi_entry_form_tight_scalar_anchor} with a 62841% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 635.5x the fastest

Fastest abi_entry_form_tight_null_entry (3.20 us) to slowest abi_entry_form_tight_scalar_anchor (2.03 ms): 635.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 3198.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 635.52x (fastest 3198.8 ns, slowest 2032879.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2041533ns | 2022650ns | 2013755ns | 2020200ns | 2087423ns | +0.68% |
| abi_entry_form_tight_null_entry | 5506ns | 5489ns | 5322ns | 5438ns | 5701ns | -99.73% |
| abi_entry_form_tight_per_w_set | 2071371ns | 2025080ns | 2011701ns | 2022194ns | 2174971ns | +2.15% |
| abi_entry_form_tight_runtime_w | 2027696ns | 2016234ns | 2008474ns | 2015593ns | 2055461ns | base |
| abi_entry_form_tight_scalar_anchor | 2035580ns | 2035978ns | 2028495ns | 2034457ns | 2040807ns | +0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2038452ns | 2010907ns | 2084001ns | +0.67% | 0.000 |
| abi_entry_form_tight_null_entry | 3196ns | 3097ns | 3284ns | -99.84% | 0.080 |
| abi_entry_form_tight_per_w_set | 2068113ns | 2008756ns | 2171153ns | +2.14% | 0.000 |
| abi_entry_form_tight_runtime_w | 2024792ns | 2005709ns | 2052470ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2032602ns | 2025642ns | 2037798ns | +0.39% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 55071.1 | 2046395.1 | 2038451.8 | n/a |
| abi_entry_form_tight_null_entry | 27851.9 | 3270.9 | 3196.3 | n/a |
| abi_entry_form_tight_per_w_set | 120200.4 | 2065269.5 | 2068112.8 | n/a |
| abi_entry_form_tight_runtime_w | 53005.3 | 2014962.0 | 2024791.7 | n/a |
| abi_entry_form_tight_scalar_anchor | 54859.4 | 2033218.3 | 2032602.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_tight_null_entry | 0.080 | 96.8% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.2% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.2% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2041533ns | 2041533ns | +0.68% |
| abi_entry_form_tight_null_entry | 5506ns | 5506ns | -99.73% |
| abi_entry_form_tight_per_w_set | 2071371ns | 2071371ns | +2.15% |
| abi_entry_form_tight_runtime_w | 2027696ns | 2027696ns | base |
| abi_entry_form_tight_scalar_anchor | 2035580ns | 2035580ns | +0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2013337ns | base | --- | [2008568, 2052470] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2019642ns | no significant difference | [-33818, +68551]ns | [2011712, 2084001] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_tight_null_entry | 3199ns | -2010168.9ns (-99.8%) | [-2049271, -2005346]ns | [3106, 3284] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2022011ns | +10138.1ns (+0.5%) | [+1142, +118683]ns | [2011175, 2171153] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_entry_form_tight_scalar_anchor | 2032879ns | no significant difference | [-23889, +27676]ns | [2027129, 2037798] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2011428ns | -0.0% | -99.8% | +0.7% | +0.9% |
| 2 | 2005709ns | +0.3% | -99.8% | +0.2% | +1.4% |
| 3 | 2012318ns | +0.3% | -99.8% | +0.3% | +1.3% |
| 4 | 2014879ns | +6.5% | -99.8% | +9.7% | +1.1% |
| 5 | 2014356ns | +0.3% | -99.8% | -0.0% | +0.8% |
| 6 | 2090061ns | -3.2% | -99.8% | +2.0% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | -0.182 | ok |
| abi_entry_form_tight_null_entry | -0.153 | ok |
| abi_entry_form_tight_per_w_set | -0.368 | moderate- |
| abi_entry_form_tight_runtime_w | 0.008 | ok |
| abi_entry_form_tight_scalar_anchor | 0.275 | moderate+ |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 1/6, lost 4/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 0/6, lost 5/6
- **abi_entry_form_tight_scalar_anchor**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6177078.1ns | 2038451.8ns | 303.0% | HIGH |
| abi_entry_form_tight_null_entry | 121534.5ns | 3196.3ns | 3802.4% | HIGH |
| abi_entry_form_tight_per_w_set | 6330131.2ns | 2068112.8ns | 306.1% | HIGH |
| abi_entry_form_tight_runtime_w | 6112492.8ns | 2024791.7ns | 301.9% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6159362.4ns | 2032602.0ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2010906.7-2084001.0 ns)
  2010906.7 |########################################
  2014561.4 |####################
  2018216.1 |####################
  2021870.8 |####################
  2025525.6 |
  2029180.3 |
  2032835.0 |
  2036489.7 |
  2040144.4 |
  2043799.1 |
  2047453.9 |
  2051108.6 |
  2054763.3 |
  2058418.0 |
  2062072.7 |
  2065727.4 |
  2069382.1 |
  2073036.9 |
  2076691.6 |
  2080346.3 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 3097.1-3284.2 ns)
   3097.1 |####################
   3106.5 |####################
   3115.8 |
   3125.2 |
   3134.5 |
   3143.9 |
   3153.2 |
   3162.6 |
   3171.9 |
   3181.3 |
   3190.6 |########################################
   3200.0 |
   3209.4 |
   3218.7 |
   3228.1 |
   3237.4 |####################
   3246.8 |
   3256.1 |
   3265.5 |
   3274.8 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2008755.8-2171152.7 ns)
  2008755.8 |########################################
  2016875.6 |####################
  2024995.5 |####################
  2033115.3 |
  2041235.2 |
  2049355.0 |
  2057474.9 |
  2065594.7 |
  2073714.6 |
  2081834.4 |
  2089954.2 |
  2098074.1 |
  2106193.9 |
  2114313.8 |
  2122433.6 |
  2130553.5 |####################
  2138673.3 |
  2146793.2 |
  2154913.0 |
  2163032.9 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2005708.8-2052469.9 ns)
  2005708.8 |####################
  2008046.9 |
  2010384.9 |########################################
  2012723.0 |########################################
  2015061.0 |
  2017399.1 |
  2019737.1 |
  2022075.2 |
  2024413.3 |
  2026751.3 |
  2029089.4 |
  2031427.4 |
  2033765.5 |
  2036103.5 |
  2038441.6 |
  2040779.7 |
  2043117.7 |
  2045455.8 |
  2047793.8 |
  2050131.9 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2025641.7-2037798.4 ns)
  2025641.7 |########################################
  2026249.5 |
  2026857.4 |
  2027465.2 |
  2028073.0 |########################################
  2028680.9 |
  2029288.7 |
  2029896.5 |
  2030504.4 |########################################
  2031112.2 |
  2031720.0 |
  2032327.9 |
  2032935.7 |
  2033543.5 |
  2034151.4 |
  2034759.2 |########################################
  2035367.0 |
  2035974.9 |
  2036582.7 |########################################
  2037190.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=3777.3% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.2% of algo (FFI overhead may distort results)
