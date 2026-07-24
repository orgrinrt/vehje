# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_wideselect_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_wideselect_runtime_w has the worst median (2.09 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_wideselect_null_entry at 3.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_wideselect_null_entry dominates: 59672% faster than the next best (abi_entry_form_wideselect_scalar_anchor)

abi_entry_form_wideselect_null_entry (3.48 us) leads abi_entry_form_wideselect_scalar_anchor (2.08 ms) by 59672%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.09 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_runtime_w is an outlier: 601.5x slower than the field

abi_entry_form_wideselect_runtime_w (2.09 ms) is 601.5x the fastest (3.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_runtime_w} (59672% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_runtime_w} with a 59672% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 601.5x the fastest

Fastest abi_entry_form_wideselect_null_entry (3.48 us) to slowest abi_entry_form_wideselect_runtime_w (2.09 ms): 601.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 3478.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 601.53x (fastest 3478.8 ns, slowest 2092572.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2127941ns | 2092162ns | 2068923ns | 2086992ns | 2218873ns | -0.36% |
| abi_entry_form_wideselect_null_entry | 5795ns | 5799ns | 5580ns | 5741ns | 5984ns | -99.73% |
| abi_entry_form_wideselect_per_w_set | 2318893ns | 2084779ns | 2071165ns | 2081532ns | 2798799ns | +8.58% |
| abi_entry_form_wideselect_runtime_w | 2135651ns | 2095786ns | 2080695ns | 2091831ns | 2228858ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2090786ns | 2082340ns | 2069459ns | 2078923ns | 2119245ns | -2.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2124458ns | 2065970ns | 2214855ns | -0.36% | 0.000 |
| abi_entry_form_wideselect_null_entry | 3472ns | 3350ns | 3582ns | -99.84% | 0.001 |
| abi_entry_form_wideselect_per_w_set | 2315231ns | 2068389ns | 2794035ns | +8.59% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2132167ns | 2077569ns | 2224846ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2087508ns | 2066163ns | 2115672ns | -2.09% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 67269.2 | 2170378.9 | 2124458.1 | n/a |
| abi_entry_form_wideselect_null_entry | 29227.0 | 3569.0 | 3471.7 | n/a |
| abi_entry_form_wideselect_per_w_set | 71955.3 | 2335620.5 | 2315231.2 | 0 |
| abi_entry_form_wideselect_runtime_w | 72182.3 | 2130823.1 | 2132167.0 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 63961.4 | 2135458.2 | 2087508.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_wideselect_null_entry | 0.001 | 96.3% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.2% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.2% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2127941ns | 2127941ns | -0.36% |
| abi_entry_form_wideselect_null_entry | 5795ns | 5795ns | -99.73% |
| abi_entry_form_wideselect_per_w_set | 2318893ns | 2318893ns | +8.58% |
| abi_entry_form_wideselect_runtime_w | 2135651ns | 2135651ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2090786ns | 2090786ns | -2.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2092573ns | base | --- | [2079082, 2224846] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2088759ns | no significant difference | [-132776, +118107]ns | [2069760, 2214855] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_wideselect_null_entry | 3479ns | -2089094.0ns (-99.8%) | [-2221393, -2075599]ns | [3355, 3582] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2081541ns | no significant difference | [-133327, +695148]ns | [2070118, 2794035] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2079330ns | no significant difference | [-123216, +4681]ns | [2067522, 2115672] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2295220ns | -10.0% | -99.9% | -9.7% | -9.8% |
| 2 | 2080594ns | -0.3% | -99.8% | -0.6% | -0.6% |
| 3 | 2098529ns | +10.2% | -99.8% | +65.7% | -0.4% |
| 4 | 2077569ns | +1.1% | -99.8% | +0.6% | +0.9% |
| 5 | 2086617ns | -0.5% | -99.8% | -0.6% | -1.0% |
| 6 | 2154473ns | -1.7% | -99.8% | -2.0% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.218 | moderate- |
| abi_entry_form_wideselect_null_entry | 0.137 | ok |
| abi_entry_form_wideselect_per_w_set | -0.238 | moderate- |
| abi_entry_form_wideselect_runtime_w | -0.094 | ok |
| abi_entry_form_wideselect_scalar_anchor | -0.252 | moderate- |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_wideselect_scalar_anchor**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6559383.2ns | 2124458.1ns | 308.8% | HIGH |
| abi_entry_form_wideselect_null_entry | 121676.1ns | 3471.7ns | 3504.8% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6973471.5ns | 2315231.2ns | 301.2% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6492847.7ns | 2132167.0ns | 304.5% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6400540.8ns | 2087508.1ns | 306.6% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2065970.0-2214855.0 ns)
  2065970.0 |####################
  2073414.2 |########################################
  2080858.5 |
  2088302.8 |
  2095747.0 |####################
  2103191.2 |
  2110635.5 |
  2118079.8 |####################
  2125524.0 |
  2132968.2 |
  2140412.5 |
  2147856.8 |
  2155301.0 |
  2162745.2 |
  2170189.5 |
  2177633.8 |
  2185078.0 |
  2192522.2 |
  2199966.5 |
  2207410.8 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 3350.4-3581.7 ns)
   3350.4 |########################################
   3362.0 |
   3373.5 |
   3385.1 |
   3396.7 |
   3408.2 |
   3419.8 |
   3431.3 |####################
   3442.9 |
   3454.5 |
   3466.0 |
   3477.6 |
   3489.2 |
   3500.7 |
   3512.3 |####################
   3523.8 |
   3535.4 |
   3547.0 |####################
   3558.5 |
   3570.1 |
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2068388.8-2794035.2 ns)
  2068388.8 |########################################
  2104671.1 |##########
  2140953.4 |
  2177235.8 |
  2213518.1 |
  2249800.4 |
  2286082.7 |
  2322365.0 |
  2358647.4 |
  2394929.7 |
  2431212.0 |
  2467494.3 |
  2503776.6 |
  2540059.0 |
  2576341.3 |
  2612623.6 |
  2648905.9 |
  2685188.2 |
  2721470.6 |
  2757752.9 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2077569.2-2224846.5 ns)
  2077569.2 |########################################
  2084933.1 |####################
  2092296.9 |####################
  2099660.8 |
  2107024.6 |
  2114388.5 |
  2121752.4 |
  2129116.2 |
  2136480.1 |
  2143844.0 |
  2151207.8 |####################
  2158571.7 |
  2165935.6 |
  2173299.4 |
  2180663.3 |
  2188027.1 |
  2195391.0 |
  2202754.9 |
  2210118.7 |
  2217482.6 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2066163.3-2115671.7 ns)
  2066163.3 |####################
  2068638.7 |########################################
  2071114.1 |
  2073589.6 |
  2076065.0 |
  2078540.4 |
  2081015.8 |
  2083491.2 |
  2085966.7 |
  2088442.1 |####################
  2090917.5 |
  2093392.9 |
  2095868.3 |####################
  2098343.8 |
  2100819.2 |
  2103294.6 |
  2105770.0 |
  2108245.4 |
  2110720.9 |
  2113196.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=3494.2% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: CV=22.4% (high variance, measurements may be unstable)
- **abi_entry_form_wideselect_per_w_set**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=303.7% of algo (FFI overhead may distort results)
