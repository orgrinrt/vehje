# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 59956% faster than the next best (abi_entry_form_tight_dispatch_table)

abi_entry_form_tight_null_entry (3.42 us) leads abi_entry_form_tight_dispatch_table (2.05 ms) by 59956%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.05 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 606.2x slower than the field

abi_entry_form_tight_scalar_anchor (2.07 ms) is 606.2x the fastest (3.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_scalar_anchor} (59956% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_scalar_anchor} with a 59956% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 606.2x the fastest

Fastest abi_entry_form_tight_null_entry (3.42 us) to slowest abi_entry_form_tight_scalar_anchor (2.07 ms): 606.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 3416.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 606.24x (fastest 3416.2 ns, slowest 2071071.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2069300ns | 2055246ns | 2043340ns | 2053569ns | 2105877ns | +0.41% |
| abi_entry_form_tight_null_entry | 5678ns | 5685ns | 5538ns | 5674ns | 5753ns | -99.72% |
| abi_entry_form_tight_per_w_set | 2174246ns | 2061032ns | 2053370ns | 2058632ns | 2408106ns | +5.50% |
| abi_entry_form_tight_runtime_w | 2060831ns | 2056957ns | 2039325ns | 2053033ns | 2083281ns | base |
| abi_entry_form_tight_scalar_anchor | 2121671ns | 2074506ns | 2067410ns | 2072977ns | 2221843ns | +2.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2065736ns | 2040378ns | 2101958ns | +0.42% | 0.000 |
| abi_entry_form_tight_null_entry | 3411ns | 3336ns | 3460ns | -99.83% | 0.001 |
| abi_entry_form_tight_per_w_set | 2170732ns | 2049940ns | 2404561ns | +5.52% | 0.000 |
| abi_entry_form_tight_runtime_w | 2057125ns | 2035562ns | 2079355ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2117984ns | 2063876ns | 2217597ns | +2.96% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 71408.4 | 2061662.7 | 2065735.6 | 0 |
| abi_entry_form_tight_null_entry | 28409.8 | 3507.2 | 3411.0 | n/a |
| abi_entry_form_tight_per_w_set | 75430.8 | 2064056.3 | 2170731.7 | n/a |
| abi_entry_form_tight_runtime_w | 72993.7 | 2059139.9 | 2057124.8 | n/a |
| abi_entry_form_tight_scalar_anchor | 71746.2 | 2116458.8 | 2117983.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_tight_null_entry | 0.001 | 97.6% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.2% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.2% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2069300ns | 2069300ns | +0.41% |
| abi_entry_form_tight_null_entry | 5678ns | 5678ns | -99.72% |
| abi_entry_form_tight_per_w_set | 2174246ns | 2174246ns | +5.50% |
| abi_entry_form_tight_runtime_w | 2060831ns | 2060831ns | base |
| abi_entry_form_tight_scalar_anchor | 2121671ns | 2121671ns | +2.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2053304ns | base | --- | [2038715, 2079355] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2051669ns | no significant difference | [-29470, +56453]ns | [2043580, 2101958] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_tight_null_entry | 3416ns | -2049897.3ns (-99.8%) | [-2075960, -2035285]ns | [3357, 3460] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2057457ns | no significant difference | [-22316, +365187]ns | [2050177, 2404561] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_scalar_anchor | 2071072ns | no significant difference | [-10566, +168939]ns | [2065282, 2217597] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2051161ns | -0.0% | -99.8% | +0.1% | +0.6% |
| 2 | 2041868ns | -0.1% | -99.8% | +34.5% | +5.1% |
| 3 | 2076409ns | -1.4% | -99.8% | -0.6% | -0.3% |
| 4 | 2055447ns | +0.4% | -99.8% | -0.3% | +11.4% |
| 5 | 2082300ns | -1.4% | -99.8% | -1.5% | -0.7% |
| 6 | 2035562ns | +5.1% | -99.8% | +1.3% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | -0.003 | ok |
| abi_entry_form_tight_null_entry | 0.161 | ok |
| abi_entry_form_tight_per_w_set | -0.224 | moderate- |
| abi_entry_form_tight_runtime_w | -0.471 | moderate- |
| abi_entry_form_tight_scalar_anchor | -0.429 | moderate- |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 2/6, lost 2/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 3/6, lost 2/6
- **abi_entry_form_tight_scalar_anchor**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6280124.0ns | 2065735.6ns | 304.0% | HIGH |
| abi_entry_form_tight_null_entry | 120772.1ns | 3411.0ns | 3540.7% | HIGH |
| abi_entry_form_tight_per_w_set | 6296916.1ns | 2170731.7ns | 290.1% | HIGH |
| abi_entry_form_tight_runtime_w | 6258936.0ns | 2057124.8ns | 304.3% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6410811.0ns | 2117983.6ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2040378.3-2101958.0 ns)
  2040378.3 |########################################
  2043457.3 |
  2046536.3 |########################################
  2049615.2 |########################################
  2052694.2 |########################################
  2055773.2 |
  2058852.2 |
  2061931.2 |########################################
  2065010.2 |
  2068089.1 |
  2071168.1 |
  2074247.1 |
  2077326.1 |
  2080405.1 |
  2083484.1 |
  2086563.0 |
  2089642.0 |
  2092721.0 |
  2095800.0 |
  2098879.0 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 3335.8-3459.8 ns)
   3335.8 |########################################
   3342.0 |
   3348.2 |
   3354.4 |
   3360.6 |
   3366.8 |
   3373.0 |########################################
   3379.2 |
   3385.4 |
   3391.6 |
   3397.8 |
   3404.0 |
   3410.2 |########################################
   3416.4 |########################################
   3422.6 |
   3428.8 |
   3435.0 |
   3441.2 |########################################
   3447.4 |
   3453.6 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2049940.4-2404561.2 ns)
  2049940.4 |########################################
  2067671.4 |
  2085402.5 |
  2103133.5 |
  2120864.6 |
  2138595.6 |
  2156326.7 |
  2174057.7 |
  2191788.7 |
  2209519.8 |
  2227250.8 |
  2244981.9 |
  2262712.9 |
  2280444.0 |
  2298175.0 |
  2315906.0 |
  2333637.1 |
  2351368.1 |
  2369099.2 |
  2386830.2 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2035562.5-2079354.8 ns)
  2035562.5 |########################################
  2037752.1 |
  2039941.7 |########################################
  2042131.3 |
  2044321.0 |
  2046510.6 |
  2048700.2 |
  2050889.8 |########################################
  2053079.4 |
  2055269.0 |########################################
  2057458.6 |
  2059648.3 |
  2061837.9 |
  2064027.5 |
  2066217.1 |
  2068406.7 |
  2070596.3 |
  2072786.0 |
  2074975.6 |########################################
  2077165.2 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2063876.2-2217596.9 ns)
  2063876.2 |########################################
  2071562.2 |
  2079248.3 |
  2086934.3 |
  2094620.3 |
  2102306.4 |
  2109992.4 |
  2117678.4 |
  2125364.5 |
  2133050.5 |
  2140736.6 |##########
  2148422.6 |
  2156108.6 |
  2163794.7 |
  2171480.7 |
  2179166.7 |
  2186852.8 |
  2194538.8 |
  2202224.8 |
  2209910.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=3527.4% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=302.6% of algo (FFI overhead may distort results)
