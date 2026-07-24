# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_wideselect_null_entry dominates: 89602% faster than the next best (abi_entry_form_wideselect_runtime_w)

abi_entry_form_wideselect_null_entry (2.30 us) leads abi_entry_form_wideselect_runtime_w (2.07 ms) by 89602%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_scalar_anchor is an outlier: 899.0x slower than the field

abi_entry_form_wideselect_scalar_anchor (2.07 ms) is 899.0x the fastest (2.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor} (89602% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor} with a 89602% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 899.0x the fastest

Fastest abi_entry_form_wideselect_null_entry (2.30 us) to slowest abi_entry_form_wideselect_scalar_anchor (2.07 ms): 899.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 2302.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 899.05x (fastest 2302.9 ns, slowest 2070421.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2173109ns | 2071428ns | 2063568ns | 2070797ns | 2381349ns | +4.93% |
| abi_entry_form_wideselect_null_entry | 4708ns | 4705ns | 4432ns | 4676ns | 4892ns | -99.77% |
| abi_entry_form_wideselect_per_w_set | 2113294ns | 2071025ns | 2058246ns | 2067602ns | 2209357ns | +2.04% |
| abi_entry_form_wideselect_runtime_w | 2071032ns | 2068806ns | 2064243ns | 2067991ns | 2078990ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2073467ns | 2073405ns | 2068168ns | 2071661ns | 2078826ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2169908ns | 2060956ns | 2377537ns | +4.93% | 0.000 |
| abi_entry_form_wideselect_null_entry | 2319ns | 2220ns | 2413ns | -99.89% | 0.014 |
| abi_entry_form_wideselect_per_w_set | 2110148ns | 2055475ns | 2205562ns | +2.04% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2068000ns | 2061469ns | 2075698ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2070550ns | 2065181ns | 2075889ns | +0.12% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 60441.7 | 2126399.5 | 2169908.3 | n/a |
| abi_entry_form_wideselect_null_entry | 28818.8 | 2455.0 | 2319.4 | n/a |
| abi_entry_form_wideselect_per_w_set | 62715.5 | 2097447.1 | 2110147.9 | n/a |
| abi_entry_form_wideselect_runtime_w | 56722.9 | 2069398.0 | 2067999.6 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 55225.6 | 2069302.3 | 2070549.7 | 1 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_wideselect_null_entry | 0.014 | 96.4% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.1% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.1% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2173109ns | 2173109ns | +4.93% |
| abi_entry_form_wideselect_null_entry | 4708ns | 4708ns | -99.77% |
| abi_entry_form_wideselect_per_w_set | 2113294ns | 2113294ns | +2.04% |
| abi_entry_form_wideselect_runtime_w | 2071032ns | 2071032ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2073467ns | 2073467ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2065744ns | base | --- | [2062558, 2075698] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2068493ns | no significant difference | [-11129, +313208]ns | [2063694, 2377537] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_wideselect_null_entry | 2303ns | -2063440.9ns (-99.9%) | [-2073455, -2060144]ns | [2242, 2413] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2068194ns | no significant difference | [-13420, +137066]ns | [2056689, 2205562] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2070421ns | no significant difference | [-5811, +12822]ns | [2065339, 2075889] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2077652ns | -0.5% | -99.9% | +0.1% | -0.2% |
| 2 | 2061469ns | +0.4% | -99.9% | +0.3% | +0.6% |
| 3 | 2063646ns | +29.9% | -99.9% | +13.0% | +0.7% |
| 4 | 2065012ns | +0.5% | -99.9% | +0.2% | +0.0% |
| 5 | 2066475ns | -0.0% | -99.9% | -0.4% | +0.1% |
| 6 | 2073742ns | -0.6% | -99.9% | -0.9% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.219 | moderate- |
| abi_entry_form_wideselect_null_entry | 0.055 | ok |
| abi_entry_form_wideselect_per_w_set | -0.207 | moderate- |
| abi_entry_form_wideselect_runtime_w | -0.130 | ok |
| abi_entry_form_wideselect_scalar_anchor | 0.169 | ok |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 2/6, lost 3/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 2/6, lost 3/6
- **abi_entry_form_wideselect_scalar_anchor**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6436816.2ns | 2169908.3ns | 296.6% | HIGH |
| abi_entry_form_wideselect_null_entry | 118006.7ns | 2319.4ns | 5087.8% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6363405.4ns | 2110147.9ns | 301.6% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6272671.9ns | 2067999.6ns | 303.3% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6267972.9ns | 2070549.7ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2060956.2-2377537.3 ns)
  2060956.2 |########################################
  2076785.3 |
  2092614.3 |
  2108443.4 |
  2124272.4 |
  2140101.5 |
  2155930.5 |
  2171759.6 |
  2187588.6 |
  2203417.7 |
  2219246.8 |
  2235075.8 |
  2250904.9 |
  2266733.9 |
  2282563.0 |
  2298392.0 |
  2314221.1 |
  2330050.1 |
  2345879.2 |
  2361708.2 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 2220.4-2413.1 ns)
   2220.4 |########################################
   2230.0 |
   2239.7 |
   2249.3 |
   2258.9 |########################################
   2268.6 |########################################
   2278.2 |
   2287.9 |
   2297.5 |
   2307.1 |
   2316.8 |
   2326.4 |########################################
   2336.0 |
   2345.7 |
   2355.3 |
   2365.0 |
   2374.6 |
   2384.2 |
   2393.9 |
   2403.5 |########################################
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2055474.6-2205561.7 ns)
  2055474.6 |########################################
  2062979.0 |########################################
  2070483.3 |
  2077987.7 |####################
  2085492.0 |
  2092996.4 |
  2100500.7 |
  2108005.1 |
  2115509.4 |
  2123013.8 |
  2130518.2 |
  2138022.5 |
  2145526.9 |
  2153031.2 |
  2160535.6 |
  2168039.9 |
  2175544.3 |
  2183048.6 |
  2190553.0 |
  2198057.3 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2061469.2-2075697.5 ns)
  2061469.2 |########################################
  2062180.6 |
  2062892.0 |
  2063603.4 |########################################
  2064314.9 |########################################
  2065026.3 |
  2065737.7 |
  2066449.1 |########################################
  2067160.5 |
  2067871.9 |
  2068583.4 |
  2069294.8 |
  2070006.2 |
  2070717.6 |
  2071429.0 |
  2072140.4 |
  2072851.8 |
  2073563.3 |########################################
  2074274.7 |
  2074986.1 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2065180.8-2075888.8 ns)
  2065180.8 |########################################
  2065716.2 |
  2066251.6 |
  2066787.0 |
  2067322.4 |####################
  2067857.8 |
  2068393.2 |
  2068928.6 |
  2069464.0 |
  2069999.4 |
  2070534.8 |
  2071070.2 |
  2071605.6 |
  2072141.0 |
  2072676.4 |
  2073211.8 |####################
  2073747.2 |####################
  2074282.6 |
  2074818.0 |
  2075353.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=5110.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=303.0% of algo (FFI overhead may distort results)
