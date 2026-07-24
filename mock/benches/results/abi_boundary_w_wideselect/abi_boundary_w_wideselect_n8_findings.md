# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_wideselect_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_wideselect_scalar_runtime_w has the worst median (2.09 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_wideselect_null_entry at 3.01 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_wideselect_null_entry dominates: 30758% faster than the next best (abi_boundary_w_wideselect_soa_per_w)

abi_boundary_w_wideselect_null_entry (3.01 us) leads abi_boundary_w_wideselect_soa_per_w (928.25 us) by 30758%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.08 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_scalar_runtime_w is an outlier: 693.3x slower than the field

abi_boundary_w_wideselect_scalar_runtime_w (2.09 ms) is 693.3x the fastest (3.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_soa_runtime_w shows alternating (throttle bounce) (autocorr -0.65)

abi_boundary_w_wideselect_soa_runtime_w's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_runtime_w} (30758% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_scalar_runtime_w} with a 30758% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 693.3x the fastest

Fastest abi_boundary_w_wideselect_null_entry (3.01 us) to slowest abi_boundary_w_wideselect_scalar_runtime_w (2.09 ms): 693.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 3008.1 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 693.31x (fastest 3008.1 ns, slowest 2085568.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5270ns | 5269ns | 5122ns | 5228ns | 5406ns | -99.75% |
| abi_boundary_w_wideselect_scalar_anchor | 2082750ns | 2082192ns | 2076704ns | 2080365ns | 2089352ns | -0.25% |
| abi_boundary_w_wideselect_scalar_dispatch | 2076983ns | 2075626ns | 2063572ns | 2073841ns | 2088401ns | -0.52% |
| abi_boundary_w_wideselect_scalar_per_w | 2077345ns | 2076338ns | 2067108ns | 2074034ns | 2087429ns | -0.51% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2087944ns | 2089247ns | 2079111ns | 2086502ns | 2094524ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 943938ns | 946619ns | 934582ns | 943988ns | 948541ns | -54.79% |
| abi_boundary_w_wideselect_soa_per_w | 930249ns | 930945ns | 925104ns | 929962ns | 933253ns | -55.45% |
| abi_boundary_w_wideselect_soa_runtime_w | 938254ns | 937487ns | 933284ns | 936663ns | 943124ns | -55.06% |
| abi_boundary_w_wideselect_zig_runtime_w | 2000033ns | 1996425ns | 1989463ns | 1995052ns | 2012790ns | -4.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 3003ns | 2919ns | 3082ns | -99.86% | 0.003 |
| abi_boundary_w_wideselect_scalar_anchor | 2079406ns | 2073394ns | 2085984ns | -0.24% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2073790ns | 2060861ns | 2084762ns | -0.51% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2074060ns | 2064137ns | 2083885ns | -0.50% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2084446ns | 2075798ns | 2091013ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 940913ns | 932068ns | 945442ns | -54.86% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 927479ns | 922709ns | 930253ns | -55.50% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 935335ns | 930758ns | 939848ns | -55.13% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 1996615ns | 1986312ns | 2009190ns | -4.21% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 28617.5 | 3143.7 | 3003.1 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 70666.4 | 2079455.6 | 2079406.3 | n/a |
| abi_boundary_w_wideselect_scalar_dispatch | 65514.6 | 2075732.9 | 2073790.5 | n/a |
| abi_boundary_w_wideselect_scalar_per_w | 69153.1 | 2075275.5 | 2074059.8 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 76759.5 | 2084609.7 | 2084445.8 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 54142.8 | 940903.7 | 940913.4 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 47142.5 | 927907.2 | 927478.6 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 51941.2 | 935562.6 | 935334.5 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 241614.2 | 1996814.4 | 1996614.8 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.003 | 97.0% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5270ns | 5270ns | -99.75% |
| abi_boundary_w_wideselect_scalar_anchor | 2082750ns | 2082750ns | -0.25% |
| abi_boundary_w_wideselect_scalar_dispatch | 2076983ns | 2076983ns | -0.52% |
| abi_boundary_w_wideselect_scalar_per_w | 2077345ns | 2077345ns | -0.51% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2087944ns | 2087944ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 943938ns | 943938ns | -54.79% |
| abi_boundary_w_wideselect_soa_per_w | 930249ns | 930249ns | -55.45% |
| abi_boundary_w_wideselect_soa_runtime_w | 938254ns | 938254ns | -55.06% |
| abi_boundary_w_wideselect_zig_runtime_w | 2000033ns | 2000033ns | -4.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2085569ns | base | --- | [2076756, 2091013] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 3008ns | -2082552.8ns (-99.9%) | [-2088028, -2073748]ns | [2919, 3082] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2078723ns | no significant difference | [-11710, +461]ns | [2073512, 2085984] | no | 0.2188 | 0.2188 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2072462ns | -10193.0ns (-0.5%) | [-21420, -353]ns | [2064148, 2084762] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2073026ns | no significant difference | [-21716, +1328]ns | [2065269, 2083885] | no | 0.2188 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 943147ns | -1145295.8ns (-54.9%) | [-1149809, -1135492]ns | [934152, 945442] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 928246ns | -1157239.1ns (-55.5%) | [-1160844, -1152819]ns | [923937, 930253] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 934572ns | -1147923.9ns (-55.0%) | [-1156020, -1143390]ns | [931583, 939848] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 1992894ns | -90522.3ns (-4.3%) | [-96428, -76543]ns | [1987760, 2009190] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2083867ns | -99.9% | -0.1% | -1.1% | +0.2% | -54.6% | -55.5% | -55.3% | -4.7% |
| 2 | 2077714ns | -99.9% | -0.2% | +0.4% | -0.7% | -54.5% | -55.6% | -55.0% | -3.2% |
| 3 | 2093752ns | -99.9% | -0.2% | -0.4% | -1.1% | -54.9% | -55.6% | -55.4% | -4.1% |
| 4 | 2087270ns | -99.9% | -0.4% | -1.0% | -1.0% | -55.1% | -55.4% | -55.0% | -4.6% |
| 5 | 2075798ns | -99.9% | +0.1% | -0.4% | -0.0% | -55.1% | -55.4% | -55.2% | -4.2% |
| 6 | 2088274ns | -99.9% | -0.7% | -0.5% | -0.4% | -54.9% | -55.5% | -54.9% | -4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.206 | moderate+ |
| abi_boundary_w_wideselect_scalar_anchor | -0.412 | moderate- |
| abi_boundary_w_wideselect_scalar_dispatch | -0.152 | ok |
| abi_boundary_w_wideselect_scalar_per_w | -0.217 | moderate- |
| abi_boundary_w_wideselect_scalar_runtime_w | -0.392 | moderate- |
| abi_boundary_w_wideselect_soa_dispatch | 0.337 | moderate+ |
| abi_boundary_w_wideselect_soa_per_w | -0.242 | moderate- |
| abi_boundary_w_wideselect_soa_runtime_w | -0.649 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_zig_runtime_w | 0.026 | ok |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 4/6, lost 1/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 120758.1ns | 3003.1ns | 4021.1% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6310976.0ns | 2079406.3ns | 303.5% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6294516.4ns | 2073790.5ns | 303.5% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6299706.1ns | 2074059.8ns | 303.7% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6338915.3ns | 2084445.8ns | 304.1% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 2878414.0ns | 940913.4ns | 305.9% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 2835270.1ns | 927478.6ns | 305.7% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 2860991.7ns | 935334.5ns | 305.9% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6324765.1ns | 1996614.8ns | 316.8% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 2918.7-3082.1 ns)
   2918.7 |########################################
   2926.9 |
   2935.0 |
   2943.2 |
   2951.4 |
   2959.5 |
   2967.7 |
   2975.9 |
   2984.0 |
   2992.2 |####################
   3000.4 |
   3008.5 |
   3016.7 |####################
   3024.9 |
   3033.0 |
   3041.2 |
   3049.4 |####################
   3057.5 |
   3065.7 |
   3073.9 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2073393.7-2085984.1 ns)
  2073393.7 |########################################
  2074023.2 |
  2074652.7 |
  2075282.3 |
  2075911.8 |
  2076541.3 |
  2077170.8 |
  2077800.4 |
  2078429.9 |########################################
  2079059.4 |
  2079688.9 |
  2080318.4 |
  2080948.0 |
  2081577.5 |####################
  2082207.0 |
  2082836.5 |
  2083466.1 |
  2084095.6 |
  2084725.1 |
  2085354.6 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2060861.2-2084761.7 ns)
  2060861.2 |####################
  2062056.2 |
  2063251.2 |
  2064446.3 |
  2065641.3 |
  2066836.3 |########################################
  2068031.3 |
  2069226.4 |
  2070421.4 |
  2071616.4 |
  2072811.4 |
  2074006.5 |
  2075201.5 |
  2076396.5 |####################
  2077591.6 |
  2078786.6 |
  2079981.6 |
  2081176.6 |
  2082371.6 |
  2083566.7 |####################
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2064136.7-2083884.8 ns)
  2064136.7 |########################################
  2065124.1 |
  2066111.5 |########################################
  2067098.9 |
  2068086.3 |
  2069073.7 |
  2070061.1 |
  2071048.5 |########################################
  2072035.9 |
  2073023.3 |
  2074010.8 |########################################
  2074998.2 |
  2075985.6 |
  2076973.0 |
  2077960.4 |
  2078947.8 |
  2079935.2 |########################################
  2080922.6 |
  2081910.0 |
  2082897.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2075797.9-2091013.1 ns)
  2075797.9 |########################################
  2076558.7 |
  2077319.4 |########################################
  2078080.2 |
  2078840.9 |
  2079601.7 |
  2080362.5 |
  2081123.2 |
  2081884.0 |
  2082644.8 |
  2083405.5 |########################################
  2084166.3 |
  2084927.0 |
  2085687.8 |
  2086448.6 |
  2087209.3 |########################################
  2087970.1 |########################################
  2088730.9 |
  2089491.6 |
  2090252.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 932067.5-945441.7 ns)
  932067.5 |####################
  932736.2 |
  933404.9 |
  934073.6 |
  934742.3 |
  935411.0 |
  936079.7 |####################
  936748.5 |
  937417.2 |
  938085.9 |
  938754.6 |
  939423.3 |
  940092.0 |
  940760.7 |####################
  941429.4 |
  942098.1 |
  942766.8 |
  943435.5 |
  944104.2 |
  944772.9 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 922709.2-930252.9 ns)
  922709.2 |####################
  923086.4 |
  923463.6 |
  923840.8 |
  924217.9 |
  924595.1 |
  924972.3 |####################
  925349.5 |
  925726.7 |
  926103.9 |####################
  926481.0 |
  926858.2 |
  927235.4 |
  927612.6 |
  927989.8 |
  928367.0 |
  928744.2 |
  929121.3 |
  929498.5 |
  929875.7 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 930758.3-939848.1 ns)
  930758.3 |########################################
  931212.8 |
  931667.3 |
  932121.8 |########################################
  932576.3 |
  933030.8 |########################################
  933485.3 |
  933939.7 |
  934394.2 |
  934848.7 |
  935303.2 |
  935757.7 |########################################
  936212.2 |
  936666.7 |
  937121.2 |
  937575.7 |
  938030.2 |########################################
  938484.7 |
  938939.2 |
  939393.7 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 1986311.7-2009189.8 ns)
  1986311.7 |########################################
  1987455.6 |
  1988599.5 |########################################
  1989743.4 |
  1990887.3 |########################################
  1992031.2 |
  1993175.1 |########################################
  1994319.0 |
  1995462.9 |
  1996606.8 |
  1997750.8 |
  1998894.7 |
  2000038.6 |
  2001182.5 |
  2002326.4 |
  2003470.3 |
  2004614.2 |
  2005758.1 |
  2006902.0 |########################################
  2008045.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=4011.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=304.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=306.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=305.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=305.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=316.8% of algo (FFI overhead may distort results)
