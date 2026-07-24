# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_wideselect_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_wideselect_runtime_w has the worst median (2.09 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_wideselect_null_entry at 4.82 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_wideselect_null_entry dominates: 42795% faster than the next best (abi_entry_form_wideselect_per_w_set)

abi_entry_form_wideselect_null_entry (4.82 us) leads abi_entry_form_wideselect_per_w_set (2.07 ms) by 42795%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.08 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_runtime_w is an outlier: 432.8x slower than the field

abi_entry_form_wideselect_runtime_w (2.09 ms) is 432.8x the fastest (4.82 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w} (42795% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w} with a 42795% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 432.8x the fastest

Fastest abi_entry_form_wideselect_null_entry (4.82 us) to slowest abi_entry_form_wideselect_runtime_w (2.09 ms): 432.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 4823.3 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 432.77x (fastest 4823.3 ns, slowest 2087393.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2105133ns | 2078247ns | 2066518ns | 2076073ns | 2168030ns | -0.69% |
| abi_entry_form_wideselect_null_entry | 7178ns | 7079ns | 6956ns | 7052ns | 7479ns | -99.66% |
| abi_entry_form_wideselect_per_w_set | 2071834ns | 2071864ns | 2065052ns | 2070251ns | 2077597ns | -2.26% |
| abi_entry_form_wideselect_runtime_w | 2119703ns | 2090624ns | 2076355ns | 2087839ns | 2189172ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2119917ns | 2079802ns | 2067788ns | 2076747ns | 2210737ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2101889ns | 2063615ns | 2164418ns | -0.68% | 0.000 |
| abi_entry_form_wideselect_null_entry | 4890ns | 4752ns | 5074ns | -99.77% | 0.000 |
| abi_entry_form_wideselect_per_w_set | 2068803ns | 2062311ns | 2074207ns | -2.24% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2116286ns | 2073077ns | 2185424ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2116571ns | 2064872ns | 2206952ns | +0.01% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 63159.0 | 2093871.7 | 2101889.4 | n/a |
| abi_entry_form_wideselect_null_entry | 28531.2 | 4946.8 | 4889.7 | n/a |
| abi_entry_form_wideselect_per_w_set | 62304.1 | 2069896.8 | 2068802.7 | n/a |
| abi_entry_form_wideselect_runtime_w | 68708.5 | 2120337.5 | 2116285.7 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 65454.1 | 2200620.2 | 2116570.6 | 11 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_wideselect_null_entry | 0.000 | 98.5% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.2% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.2% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2105133ns | 2105133ns | -0.69% |
| abi_entry_form_wideselect_null_entry | 7178ns | 7178ns | -99.66% |
| abi_entry_form_wideselect_per_w_set | 2071834ns | 2071834ns | -2.26% |
| abi_entry_form_wideselect_runtime_w | 2119703ns | 2119703ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2119917ns | 2119917ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2087393ns | base | --- | [2076040, 2185424] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2075075ns | no significant difference | [-110349, +88040]ns | [2066176, 2164418] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_wideselect_null_entry | 4823ns | -2082597.1ns (-99.8%) | [-2180625, -2070966]ns | [4772, 5074] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2068937ns | -23791.9ns (-1.1%) | [-112403, -6254]ns | [2063264, 2074207] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2076463ns | no significant difference | [-21097, +26167]ns | [2066296, 2206952] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2079678ns | +0.1% | -99.8% | -0.5% | -0.7% |
| 2 | 2095108ns | -1.3% | -99.8% | -1.5% | -1.3% |
| 3 | 2242938ns | -7.2% | -99.8% | -7.8% | +2.1% |
| 4 | 2073077ns | +8.4% | -99.8% | -0.1% | -0.2% |
| 5 | 2079002ns | -0.7% | -99.8% | -0.8% | +0.3% |
| 6 | 2127910ns | -2.8% | -99.8% | -2.4% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.236 | moderate- |
| abi_entry_form_wideselect_null_entry | 0.022 | ok |
| abi_entry_form_wideselect_per_w_set | -0.494 | moderate- |
| abi_entry_form_wideselect_runtime_w | -0.292 | moderate- |
| abi_entry_form_wideselect_scalar_anchor | -0.336 | moderate- |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6348454.1ns | 2101889.4ns | 302.0% | HIGH |
| abi_entry_form_wideselect_null_entry | 124738.3ns | 4889.7ns | 2551.0% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6277805.6ns | 2068802.7ns | 303.5% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6435127.1ns | 2116285.7ns | 304.1% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6500991.1ns | 2116570.6ns | 307.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2063614.6-2164417.7 ns)
  2063614.6 |####################
  2068654.8 |########################################
  2073694.9 |
  2078735.1 |########################################
  2083775.2 |
  2088815.4 |
  2093855.5 |
  2098895.7 |
  2103935.8 |
  2108976.0 |
  2114016.2 |
  2119056.3 |
  2124096.5 |
  2129136.6 |
  2134176.8 |
  2139216.9 |
  2144257.1 |
  2149297.2 |
  2154337.4 |
  2159377.5 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 4752.5-5073.8 ns)
   4752.5 |########################################
   4768.6 |
   4784.6 |########################################
   4800.7 |########################################
   4816.8 |
   4832.8 |########################################
   4848.9 |
   4864.9 |
   4881.0 |
   4897.1 |
   4913.1 |
   4929.2 |
   4945.2 |
   4961.3 |########################################
   4977.4 |
   4993.4 |
   5009.5 |
   5025.6 |
   5041.6 |
   5057.7 |
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2062311.2-2074207.3 ns)
  2062311.2 |########################################
  2062906.0 |
  2063500.8 |
  2064095.6 |########################################
  2064690.4 |
  2065285.2 |
  2065880.0 |
  2066474.8 |
  2067069.6 |
  2067664.4 |
  2068259.2 |########################################
  2068854.1 |
  2069448.9 |########################################
  2070043.7 |
  2070638.5 |########################################
  2071233.3 |
  2071828.1 |
  2072422.9 |
  2073017.7 |
  2073612.5 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2073076.7-2185424.1 ns)
  2073076.7 |####################
  2078694.1 |########################################
  2084311.4 |
  2089928.8 |####################
  2095546.2 |
  2101163.6 |
  2106780.9 |
  2112398.3 |
  2118015.7 |
  2123633.1 |####################
  2129250.4 |
  2134867.8 |
  2140485.2 |
  2146102.5 |
  2151719.9 |
  2157337.3 |
  2162954.7 |
  2168572.0 |
  2174189.4 |
  2179806.8 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2064871.7-2206952.5 ns)
  2064871.7 |########################################
  2071975.7 |
  2079079.8 |#############
  2086183.8 |
  2093287.9 |
  2100391.9 |
  2107495.9 |
  2114600.0 |
  2121704.0 |#############
  2128808.1 |
  2135912.1 |
  2143016.1 |
  2150120.2 |
  2157224.2 |
  2164328.3 |
  2171432.3 |
  2178536.3 |
  2185640.4 |
  2192744.4 |
  2199848.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=2592.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=303.8% of algo (FFI overhead may distort results)
