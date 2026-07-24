# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_scatter_null_entry dominates: 54055% faster than the next best (abi_entry_form_scatter_scalar_anchor)

abi_entry_form_scatter_null_entry (4.00 us) leads abi_entry_form_scatter_scalar_anchor (2.17 ms) by 54055%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.18 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_dispatch_table is an outlier: 547.1x slower than the field

abi_entry_form_scatter_dispatch_table (2.19 ms) is 547.1x the fastest (4.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_dispatch_table} (54055% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_dispatch_table} with a 54055% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 547.1x the fastest

Fastest abi_entry_form_scatter_null_entry (4.00 us) to slowest abi_entry_form_scatter_dispatch_table (2.19 ms): 547.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 4003.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 547.08x (fastest 4003.9 ns, slowest 2190496.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2208239ns | 2194258ns | 2182488ns | 2191035ns | 2246920ns | +0.55% |
| abi_entry_form_scatter_null_entry | 6311ns | 6281ns | 6189ns | 6251ns | 6462ns | -99.71% |
| abi_entry_form_scatter_per_w_set | 2217745ns | 2184480ns | 2174054ns | 2182577ns | 2292342ns | +0.98% |
| abi_entry_form_scatter_runtime_w | 2196199ns | 2183128ns | 2173145ns | 2180409ns | 2231411ns | base |
| abi_entry_form_scatter_scalar_anchor | 2184991ns | 2171639ns | 2164503ns | 2170432ns | 2217073ns | -0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2204401ns | 2178823ns | 2242726ns | +0.54% | 0.000 |
| abi_entry_form_scatter_null_entry | 4020ns | 3955ns | 4090ns | -99.82% | 0.001 |
| abi_entry_form_scatter_per_w_set | 2213856ns | 2170653ns | 2288323ns | +0.97% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2192561ns | 2169769ns | 2227379ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2181614ns | 2161487ns | 2213357ns | -0.50% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 75656.5 | 2210482.4 | 2204401.1 | n/a |
| abi_entry_form_scatter_null_entry | 26768.1 | 4173.9 | 4019.6 | n/a |
| abi_entry_form_scatter_per_w_set | 77678.9 | 2204847.4 | 2213856.3 | n/a |
| abi_entry_form_scatter_runtime_w | 72937.9 | 2269826.6 | 2192560.6 | n/a |
| abi_entry_form_scatter_scalar_anchor | 65986.2 | 2180004.1 | 2181614.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_scatter_null_entry | 0.001 | 98.8% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.2% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.2% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2208239ns | 2208239ns | +0.55% |
| abi_entry_form_scatter_null_entry | 6311ns | 6311ns | -99.71% |
| abi_entry_form_scatter_per_w_set | 2217745ns | 2217745ns | +0.98% |
| abi_entry_form_scatter_runtime_w | 2196199ns | 2196199ns | base |
| abi_entry_form_scatter_scalar_anchor | 2184991ns | 2184991ns | -0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2179595ns | base | --- | [2170708, 2227379] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2190497ns | no significant difference | [-9248, +33868]ns | [2179980, 2242726] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_scatter_null_entry | 4004ns | -2175627.2ns (-99.8%) | [-2223349, -2166647]ns | [3964, 4090] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2180607ns | no significant difference | [-54740, +117615]ns | [2172638, 2288323] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2168339ns | no significant difference | [-59040, +36909]ns | [2163148, 2213357] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2181249ns | +0.6% | -99.8% | +0.0% | +1.1% |
| 2 | 2171647ns | +0.3% | -99.8% | +1.1% | +2.3% |
| 3 | 2247948ns | +1.9% | -99.8% | -3.3% | -3.6% |
| 4 | 2177942ns | +0.4% | -99.8% | +0.1% | -0.6% |
| 5 | 2169769ns | +1.2% | -99.8% | +9.7% | -0.4% |
| 6 | 2206810ns | -1.2% | -99.8% | -1.6% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.346 | moderate- |
| abi_entry_form_scatter_null_entry | 0.173 | ok |
| abi_entry_form_scatter_per_w_set | -0.305 | moderate- |
| abi_entry_form_scatter_runtime_w | -0.377 | moderate- |
| abi_entry_form_scatter_scalar_anchor | 0.356 | moderate+ |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 1/6, lost 5/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 2/6, lost 2/6
- **abi_entry_form_scatter_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6710330.2ns | 2204401.1ns | 304.4% | HIGH |
| abi_entry_form_scatter_null_entry | 120776.3ns | 4019.6ns | 3004.7% | HIGH |
| abi_entry_form_scatter_per_w_set | 6712652.5ns | 2213856.3ns | 303.2% | HIGH |
| abi_entry_form_scatter_runtime_w | 6760407.1ns | 2192560.6ns | 308.3% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6613069.3ns | 2181614.4ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2178822.9-2242726.5 ns)
  2178822.9 |########################################
  2182018.1 |
  2185213.3 |####################
  2188408.4 |
  2191603.6 |
  2194798.8 |########################################
  2197994.0 |
  2201189.2 |
  2204384.3 |
  2207579.5 |
  2210774.7 |
  2213969.9 |
  2217165.1 |
  2220360.2 |
  2223555.4 |
  2226750.6 |
  2229945.8 |
  2233141.0 |
  2236336.1 |
  2239531.3 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 3955.4-4090.4 ns)
   3955.4 |########################################
   3962.2 |
   3968.9 |########################################
   3975.7 |########################################
   3982.4 |
   3989.2 |
   3995.9 |
   4002.7 |
   4009.4 |
   4016.2 |
   4022.9 |########################################
   4029.6 |
   4036.4 |
   4043.1 |
   4049.9 |
   4056.6 |
   4063.4 |
   4070.1 |
   4076.9 |
   4083.6 |########################################
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2170652.9-2288323.2 ns)
  2170652.9 |########################################
  2176536.4 |########################################
  2182419.9 |
  2188303.4 |
  2194187.0 |####################
  2200070.5 |
  2205954.0 |
  2211837.5 |
  2217721.0 |
  2223604.5 |
  2229488.0 |
  2235371.5 |
  2241255.1 |
  2247138.6 |
  2253022.1 |
  2258905.6 |
  2264789.1 |
  2270672.6 |
  2276556.1 |
  2282439.6 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2169768.7-2227378.8 ns)
  2169768.7 |########################################
  2172649.2 |
  2175529.7 |####################
  2178410.2 |####################
  2181290.7 |
  2184171.2 |
  2187051.7 |
  2189932.2 |
  2192812.7 |
  2195693.2 |
  2198573.7 |
  2201454.2 |
  2204334.7 |####################
  2207215.2 |
  2210095.7 |
  2212976.2 |
  2215856.7 |
  2218737.2 |
  2221617.7 |
  2224498.2 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2161487.1-2213356.7 ns)
  2161487.1 |########################################
  2164080.6 |########################################
  2166674.1 |########################################
  2169267.5 |########################################
  2171861.0 |
  2174454.5 |
  2177048.0 |
  2179641.4 |
  2182234.9 |
  2184828.4 |
  2187421.9 |
  2190015.4 |
  2192608.8 |
  2195202.3 |
  2197795.8 |
  2200389.3 |
  2202982.7 |########################################
  2205576.2 |
  2208169.7 |
  2210763.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=3013.1% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.8% of algo (FFI overhead may distort results)
