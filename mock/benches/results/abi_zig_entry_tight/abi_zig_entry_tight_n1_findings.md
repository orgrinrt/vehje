# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 38659% faster than the next best (abi_zig_entry_tight_zig_dispatch)

abi_zig_entry_tight_zig_null (5.13 us) leads abi_zig_entry_tight_zig_dispatch (1.99 ms) by 38659%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -2.00 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_runtime_w is an outlier: 621.8x slower than the field

abi_zig_entry_tight_zig_tail_runtime_w (3.19 ms) is 621.8x the fastest (5.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_tail_dispatch, abi_zig_entry_tight_zig_tail_runtime_w} (38659% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_tail_dispatch, abi_zig_entry_tight_zig_tail_runtime_w} with a 38659% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 621.8x the fastest

Fastest abi_zig_entry_tight_zig_null (5.13 us) to slowest abi_zig_entry_tight_zig_tail_runtime_w (3.19 ms): 621.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 5128.4 ns median (-99.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 621.84x (fastest 5128.4 ns, slowest 3189019.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2034798ns | 1995332ns | 1991439ns | 1994245ns | 2117307ns | +1.33% |
| abi_zig_entry_tight_zig_dispatch | 2030026ns | 1990478ns | 1985988ns | 1989356ns | 2113049ns | +1.09% |
| abi_zig_entry_tight_zig_null | 7470ns | 7541ns | 7254ns | 7456ns | 7600ns | -99.63% |
| abi_zig_entry_tight_zig_per_w_set | 2005353ns | 1990529ns | 1984147ns | 1988547ns | 2041167ns | -0.14% |
| abi_zig_entry_tight_zig_runtime_w | 2008102ns | 2006709ns | 2001988ns | 2005430ns | 2015167ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3110484ns | 3110180ns | 3107317ns | 3109590ns | 3113408ns | +54.90% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3190963ns | 3191909ns | 3184146ns | 3190047ns | 3195745ns | +58.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2032119ns | 1988907ns | 2114484ns | +1.34% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 2027239ns | 1983272ns | 2110161ns | +1.09% | 0.000 |
| abi_zig_entry_tight_zig_null | 5110ns | 4983ns | 5214ns | -99.75% | 0.000 |
| abi_zig_entry_tight_zig_per_w_set | 2002584ns | 1981456ns | 2038278ns | -0.14% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 2005299ns | 1999073ns | 2012362ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3107624ns | 3104561ns | 3110639ns | +54.97% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3188144ns | 3181370ns | 3192959ns | +58.99% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 180900.5 | 2022528.8 | 2032119.4 | 0 |
| abi_zig_entry_tight_zig_dispatch | 187065.7 | 1999384.2 | 2027239.4 | n/a |
| abi_zig_entry_tight_zig_null | 160695.7 | 5251.7 | 5109.8 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 186208.8 | 1997331.2 | 2002584.2 | n/a |
| abi_zig_entry_tight_zig_runtime_w | 186729.5 | 2004403.6 | 2005298.6 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 190146.5 | 3105725.1 | 3107624.0 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 191454.3 | 3113601.5 | 3188144.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.3% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.3% |
| abi_zig_entry_tight_zig_null | 0.000 | 97.2% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.3% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2034798ns | 2034798ns | +1.33% |
| abi_zig_entry_tight_zig_dispatch | 2030026ns | 2030026ns | +1.09% |
| abi_zig_entry_tight_zig_null | 7470ns | 7470ns | -99.63% |
| abi_zig_entry_tight_zig_per_w_set | 2005353ns | 2005353ns | -0.14% |
| abi_zig_entry_tight_zig_runtime_w | 2008102ns | 2008102ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3110484ns | 3110484ns | +54.90% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3190963ns | 3190963ns | +58.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 2003903ns | base | --- | [1999631, 2012362] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1992695ns | no significant difference | [-21567, +113566]ns | [1989180, 2114484] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1987710ns | no significant difference | [-18067, +99409]ns | [1983848, 2110161] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_tight_zig_null | 5128ns | -1998748.9ns (-99.7%) | [-2007263, -1994555]ns | [4987, 5214] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1987749ns | no significant difference | [-20805, +27205]ns | [1981727, 2038278] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3107297ns | +1104136.7ns (+55.1%) | [+1095726, +1107114]ns | [3104936, 3110639] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3189019ns | +1185513.9ns (+59.2%) | [+1172681, +1190342]ns | [3182455, 3192959] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2005990ns | -0.6% | -0.8% | -99.7% | -1.2% | +54.9% | +58.6% |
| 2 | 2018733ns | -1.3% | +10.5% | -99.8% | +3.3% | +54.0% | +58.0% |
| 3 | 2000190ns | -0.5% | -0.7% | -99.7% | -0.7% | +55.2% | +59.5% |
| 4 | 2002763ns | -0.4% | -1.0% | -99.7% | -0.6% | +55.1% | +59.2% |
| 5 | 1999073ns | +11.8% | -0.7% | -99.8% | -0.9% | +55.3% | +59.3% |
| 6 | 2005043ns | -0.8% | -0.8% | -99.7% | -0.7% | +55.3% | +59.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.234 | moderate- |
| abi_zig_entry_tight_zig_dispatch | -0.214 | moderate- |
| abi_zig_entry_tight_zig_null | -0.455 | moderate- |
| abi_zig_entry_tight_zig_per_w_set | -0.291 | moderate- |
| abi_zig_entry_tight_zig_runtime_w | -0.115 | ok |
| abi_zig_entry_tight_zig_tail_dispatch | -0.178 | ok |
| abi_zig_entry_tight_zig_tail_runtime_w | -0.308 | moderate- |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_tight_zig_dispatch**: won 5/6, lost 1/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 5/6, lost 1/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6297786.2ns | 2032119.4ns | 309.9% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6350437.7ns | 2027239.4ns | 313.3% | HIGH |
| abi_zig_entry_tight_zig_null | 318438.9ns | 5109.8ns | 6231.9% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6250116.2ns | 2002584.2ns | 312.1% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6274528.8ns | 2005298.6ns | 312.9% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9565874.7ns | 3107624.0ns | 307.8% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9598947.1ns | 3188144.5ns | 301.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1988906.7-2114484.1 ns)
  1988906.7 |########################################
  1995185.6 |
  2001464.4 |
  2007743.3 |
  2014022.2 |
  2020301.1 |
  2026579.9 |
  2032858.8 |
  2039137.7 |
  2045416.6 |
  2051695.4 |
  2057974.3 |
  2064253.2 |
  2070532.0 |
  2076810.9 |
  2083089.8 |
  2089368.7 |
  2095647.5 |
  2101926.4 |
  2108205.3 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1983272.1-2110160.9 ns)
  1983272.1 |########################################
  1989616.5 |
  1995961.0 |
  2002305.4 |
  2008649.9 |
  2014994.3 |
  2021338.7 |
  2027683.2 |
  2034027.6 |
  2040372.0 |
  2046716.5 |
  2053060.9 |
  2059405.4 |
  2065749.8 |
  2072094.2 |
  2078438.7 |
  2084783.1 |
  2091127.5 |
  2097472.0 |
  2103816.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 4983.3-5213.8 ns)
   4983.3 |########################################
   4994.8 |
   5006.3 |
   5017.9 |
   5029.4 |
   5040.9 |
   5052.4 |
   5064.0 |
   5075.5 |
   5087.0 |####################
   5098.5 |
   5110.0 |
   5121.6 |
   5133.1 |
   5144.6 |
   5156.1 |####################
   5167.7 |
   5179.2 |
   5190.7 |
   5202.2 |####################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1981455.8-2038277.5 ns)
  1981455.8 |########################################
  1984296.9 |####################
  1987138.0 |####################
  1989979.1 |####################
  1992820.1 |
  1995661.2 |
  1998502.3 |
  2001343.4 |
  2004184.5 |
  2007025.6 |
  2009866.6 |
  2012707.7 |
  2015548.8 |
  2018389.9 |
  2021231.0 |
  2024072.1 |
  2026913.2 |
  2029754.2 |
  2032595.3 |
  2035436.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1999072.9-2012361.6 ns)
  1999072.9 |########################################
  1999737.3 |########################################
  2000401.8 |
  2001066.2 |
  2001730.6 |
  2002395.1 |########################################
  2003059.5 |
  2003724.0 |
  2004388.4 |########################################
  2005052.8 |
  2005717.3 |########################################
  2006381.7 |
  2007046.1 |
  2007710.6 |
  2008375.0 |
  2009039.5 |
  2009703.9 |
  2010368.3 |
  2011032.8 |
  2011697.2 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3104561.2-3110639.0 ns)
  3104561.2 |########################################
  3104865.1 |
  3105169.0 |########################################
  3105472.9 |
  3105776.8 |
  3106080.6 |
  3106384.5 |########################################
  3106688.4 |
  3106992.3 |
  3107296.2 |
  3107600.1 |
  3107904.0 |########################################
  3108207.9 |########################################
  3108511.7 |
  3108815.6 |
  3109119.5 |
  3109423.4 |
  3109727.3 |
  3110031.2 |
  3110335.1 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3181370.4-3192958.8 ns)
  3181370.4 |########################################
  3181949.8 |
  3182529.2 |
  3183108.7 |########################################
  3183688.1 |
  3184267.5 |
  3184846.9 |
  3185426.3 |
  3186005.7 |
  3186585.2 |
  3187164.6 |
  3187744.0 |
  3188323.4 |########################################
  3188902.8 |########################################
  3189482.2 |
  3190061.7 |########################################
  3190641.1 |
  3191220.5 |
  3191799.9 |
  3192379.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=6211.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=300.8% of algo (FFI overhead may distort results)
