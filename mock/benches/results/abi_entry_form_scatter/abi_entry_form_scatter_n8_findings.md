# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_scatter_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_scatter_runtime_w has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_scatter_null_entry at 3.13 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_scatter_null_entry dominates: 69301% faster than the next best (abi_entry_form_scatter_per_w_set)

abi_entry_form_scatter_null_entry (3.13 us) leads abi_entry_form_scatter_per_w_set (2.17 ms) by 69301%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.18 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_runtime_w is an outlier: 699.6x slower than the field

abi_entry_form_scatter_runtime_w (2.19 ms) is 699.6x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_scatter_null_entry shows alternating (throttle bounce) (autocorr -0.52)

abi_entry_form_scatter_null_entry's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_runtime_w} (69301% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_runtime_w} with a 69301% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 699.6x the fastest

Fastest abi_entry_form_scatter_null_entry (3.13 us) to slowest abi_entry_form_scatter_runtime_w (2.19 ms): 699.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 3126.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 699.62x (fastest 3126.4 ns, slowest 2187325.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2199539ns | 2188467ns | 2168310ns | 2182463ns | 2240767ns | -0.28% |
| abi_entry_form_scatter_null_entry | 5492ns | 5524ns | 5310ns | 5454ns | 5639ns | -99.75% |
| abi_entry_form_scatter_per_w_set | 2178977ns | 2173159ns | 2171280ns | 2172767ns | 2192140ns | -1.21% |
| abi_entry_form_scatter_runtime_w | 2205640ns | 2191059ns | 2172539ns | 2187081ns | 2250029ns | base |
| abi_entry_form_scatter_scalar_anchor | 2183036ns | 2181879ns | 2176792ns | 2180314ns | 2190242ns | -1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2195776ns | 2164845ns | 2236757ns | -0.28% | 0.000 |
| abi_entry_form_scatter_null_entry | 3139ns | 3041ns | 3235ns | -99.86% | 0.003 |
| abi_entry_form_scatter_per_w_set | 2175511ns | 2168087ns | 2188478ns | -1.20% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2201942ns | 2169168ns | 2246232ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2179629ns | 2173332ns | 2186828ns | -1.01% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 73039.0 | 2190526.6 | 2195775.6 | n/a |
| abi_entry_form_scatter_null_entry | 29034.5 | 3219.6 | 3139.1 | n/a |
| abi_entry_form_scatter_per_w_set | 72932.9 | 2176104.7 | 2175511.4 | n/a |
| abi_entry_form_scatter_runtime_w | 73434.7 | 2210445.8 | 2201942.2 | n/a |
| abi_entry_form_scatter_scalar_anchor | 69520.4 | 2179780.6 | 2179629.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_scatter_null_entry | 0.003 | 97.3% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.1% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.1% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2199539ns | 2199539ns | -0.28% |
| abi_entry_form_scatter_null_entry | 5492ns | 5492ns | -99.75% |
| abi_entry_form_scatter_per_w_set | 2178977ns | 2178977ns | -1.21% |
| abi_entry_form_scatter_runtime_w | 2205640ns | 2205640ns | base |
| abi_entry_form_scatter_scalar_anchor | 2183036ns | 2183036ns | -1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2187325ns | base | --- | [2172269, 2246232] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2184741ns | no significant difference | [-68463, +56594]ns | [2165829, 2236757] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_scatter_null_entry | 3126ns | -2184166.5ns (-99.9%) | [-2243043, -2169200]ns | [3056, 3235] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2169793ns | no significant difference | [-75862, +8315]ns | [2168264, 2188478] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2178544ns | no significant difference | [-63538, +6275]ns | [2173515, 2186828] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2169168ns | +2.8% | -99.9% | +0.5% | +0.6% |
| 2 | 2187552ns | -1.0% | -99.9% | -0.8% | -0.6% |
| 3 | 2175370ns | -0.4% | -99.9% | -0.3% | +0.0% |
| 4 | 2301307ns | -5.0% | -99.9% | -5.8% | -4.7% |
| 5 | 2191158ns | +2.4% | -99.9% | +0.2% | -0.8% |
| 6 | 2187098ns | -0.2% | -99.9% | -0.8% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.177 | ok |
| abi_entry_form_scatter_null_entry | -0.524 | HIGH- (thermal bounce) |
| abi_entry_form_scatter_per_w_set | -0.305 | moderate- |
| abi_entry_form_scatter_runtime_w | -0.221 | moderate- |
| abi_entry_form_scatter_scalar_anchor | -0.508 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_scatter_scalar_anchor**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6657456.0ns | 2195775.6ns | 303.2% | HIGH |
| abi_entry_form_scatter_null_entry | 120425.8ns | 3139.1ns | 3836.4% | HIGH |
| abi_entry_form_scatter_per_w_set | 6600831.4ns | 2175511.4ns | 303.4% | HIGH |
| abi_entry_form_scatter_runtime_w | 6729179.9ns | 2201942.2ns | 305.6% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6606990.6ns | 2179629.1ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2164845.0-2236756.9 ns)
  2164845.0 |########################################
  2168440.6 |
  2172036.2 |
  2175631.8 |
  2179227.4 |####################
  2182823.0 |
  2186418.6 |####################
  2190014.1 |
  2193609.7 |
  2197205.3 |
  2200800.9 |
  2204396.5 |
  2207992.1 |
  2211587.7 |
  2215183.3 |
  2218778.9 |
  2222374.5 |
  2225970.1 |
  2229565.7 |####################
  2233161.3 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 3041.2-3234.6 ns)
   3041.2 |########################################
   3050.9 |
   3060.5 |
   3070.2 |########################################
   3079.9 |
   3089.5 |########################################
   3099.2 |
   3108.9 |
   3118.5 |
   3128.2 |
   3137.9 |
   3147.5 |########################################
   3157.2 |
   3166.9 |
   3176.5 |
   3186.2 |
   3195.9 |
   3205.5 |
   3215.2 |########################################
   3224.9 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2168086.7-2188477.5 ns)
  2168086.7 |########################################
  2169106.2 |
  2170125.8 |#############
  2171145.3 |
  2172164.9 |
  2173184.4 |
  2174203.9 |
  2175223.5 |
  2176243.0 |
  2177262.6 |
  2178282.1 |
  2179301.6 |
  2180321.2 |#############
  2181340.7 |
  2182360.3 |
  2183379.8 |
  2184399.3 |
  2185418.9 |
  2186438.4 |
  2187458.0 |
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2169168.3-2246232.3 ns)
  2169168.3 |####################
  2173021.5 |####################
  2176874.7 |
  2180727.9 |
  2184581.1 |########################################
  2188434.3 |####################
  2192287.5 |
  2196140.7 |
  2199993.9 |
  2203847.1 |
  2207700.3 |
  2211553.5 |
  2215406.7 |
  2219259.9 |
  2223113.1 |
  2226966.3 |
  2230819.5 |
  2234672.7 |
  2238525.9 |
  2242379.1 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2173332.1-2186828.4 ns)
  2173332.1 |########################################
  2174006.9 |
  2174681.7 |
  2175356.5 |####################
  2176031.4 |
  2176706.2 |
  2177381.0 |
  2178055.8 |
  2178730.6 |
  2179405.4 |
  2180080.2 |
  2180755.0 |
  2181429.9 |########################################
  2182104.7 |
  2182779.5 |
  2183454.3 |
  2184129.1 |
  2184803.9 |
  2185478.7 |
  2186153.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=3841.4% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.1% of algo (FFI overhead may distort results)
