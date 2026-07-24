# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 84729% faster than the next best (abi_entry_form_tight_runtime_w)

abi_entry_form_tight_null_entry (2.37 us) leads abi_entry_form_tight_runtime_w (2.01 ms) by 84729%, a clear separation rather than a photo finish. CV 104.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.01 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 856.7x slower than the field

abi_entry_form_tight_scalar_anchor (2.03 ms) is 856.7x the fastest (2.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_tight_null_entry is fastest but the noisiest (CV 104.9%)

abi_entry_form_tight_null_entry wins on median (2.37 us) yet has the highest variance (CV 104.9%), while abi_entry_form_tight_runtime_w is the steadiest (CV 0.3%, 2.01 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_scalar_anchor} (84729% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_runtime_w, abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_scalar_anchor} with a 84729% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 856.7x the fastest

Fastest abi_entry_form_tight_null_entry (2.37 us) to slowest abi_entry_form_tight_scalar_anchor (2.03 ms): 856.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_entry_form_tight_null_entry is inconsistent: worst-20% is 2.8x its best-20%

abi_entry_form_tight_null_entry's best 20% of batches run at 2.24 us but its worst 20% at 6.25 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 2373.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 856.69x (fastest 2373.7 ns, slowest 2033517.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2021254ns | 2019424ns | 2011220ns | 2017389ns | 2032068ns | +0.29% |
| abi_entry_form_tight_null_entry | 7709ns | 4762ns | 4418ns | 4713ns | 13848ns | -99.62% |
| abi_entry_form_tight_per_w_set | 2016116ns | 2016544ns | 2004642ns | 2015436ns | 2022873ns | +0.04% |
| abi_entry_form_tight_runtime_w | 2015358ns | 2016502ns | 2007925ns | 2013804ns | 2021407ns | base |
| abi_entry_form_tight_scalar_anchor | 2047381ns | 2036461ns | 2030993ns | 2035119ns | 2073968ns | +1.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2018370ns | 2008554ns | 2028750ns | +0.29% | 0.000 |
| abi_entry_form_tight_null_entry | 3632ns | 2237ns | 6246ns | -99.82% | 0.009 |
| abi_entry_form_tight_per_w_set | 2013277ns | 2002000ns | 2019896ns | +0.04% | 0.000 |
| abi_entry_form_tight_runtime_w | 2012493ns | 2005329ns | 2018255ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2044390ns | 2028255ns | 2070670ns | +1.58% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 51061.8 | 2019297.3 | 2018369.7 | n/a |
| abi_entry_form_tight_null_entry | 41156.1 | 3006.6 | 3632.0 | n/a |
| abi_entry_form_tight_per_w_set | 48872.5 | 2013350.2 | 2013276.5 | 1 |
| abi_entry_form_tight_runtime_w | 49437.2 | 2011792.3 | 2012493.1 | n/a |
| abi_entry_form_tight_scalar_anchor | 55644.9 | 2039907.0 | 2044389.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_tight_null_entry | 0.013 | 94.2% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.1% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.1% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2021254ns | 2021254ns | +0.29% |
| abi_entry_form_tight_null_entry | 7709ns | 7709ns | -99.62% |
| abi_entry_form_tight_per_w_set | 2016116ns | 2016116ns | +0.04% |
| abi_entry_form_tight_runtime_w | 2015358ns | 2015358ns | base |
| abi_entry_form_tight_scalar_anchor | 2047381ns | 2047381ns | +1.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2013591ns | base | --- | [2005633, 2018255] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2016766ns | no significant difference | [-886, +11823]ns | [2009593, 2028750] | no | 0.2917 | 0.2188 | 0 |
| abi_entry_form_tight_null_entry | 2374ns | -2008552.9ns (-99.7%) | [-2014673, -2003357]ns | [2276, 6246] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2013684ns | no significant difference | [-5954, +7209]ns | [2006249, 2019896] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_tight_scalar_anchor | 2033517ns | +24097.3ns (+1.2%) | [+15842, +55751]ns | [2028982, 2070670] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2005329ns | +0.3% | -99.9% | -0.2% | +1.1% |
| 2 | 2014209ns | +0.3% | -99.9% | +0.2% | +1.3% |
| 3 | 2005937ns | +0.4% | -99.9% | +0.5% | +1.3% |
| 4 | 2015629ns | -0.4% | -99.5% | +0.3% | +4.2% |
| 5 | 2012973ns | +0.4% | -99.9% | -0.1% | +0.8% |
| 6 | 2020882ns | +0.8% | -99.8% | -0.4% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.083 | ok |
| abi_entry_form_tight_null_entry | -0.283 | moderate- |
| abi_entry_form_tight_per_w_set | -0.253 | moderate- |
| abi_entry_form_tight_runtime_w | -0.217 | moderate- |
| abi_entry_form_tight_scalar_anchor | -0.335 | moderate- |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 1/6, lost 5/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_tight_scalar_anchor**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6111694.4ns | 2018369.7ns | 302.8% | HIGH |
| abi_entry_form_tight_null_entry | 156061.9ns | 3632.0ns | 4296.9% | HIGH |
| abi_entry_form_tight_per_w_set | 6088375.8ns | 2013276.5ns | 302.4% | HIGH |
| abi_entry_form_tight_runtime_w | 6089156.7ns | 2012493.1ns | 302.6% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6179902.2ns | 2044389.6ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2008554.2-2028750.0 ns)
  2008554.2 |########################################
  2009564.0 |
  2010573.8 |########################################
  2011583.6 |
  2012593.4 |########################################
  2013603.1 |
  2014612.9 |
  2015622.7 |
  2016632.5 |
  2017642.3 |
  2018652.1 |
  2019661.9 |########################################
  2020671.7 |########################################
  2021681.5 |
  2022691.3 |
  2023701.1 |
  2024710.8 |
  2025720.6 |
  2026730.4 |
  2027740.2 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 2237.1-6246.4 ns)
   2237.1 |########################################
   2437.6 |
   2638.0 |
   2838.5 |
   3039.0 |
   3239.4 |##########
   3439.9 |
   3640.4 |
   3840.8 |
   4041.3 |
   4241.8 |
   4442.2 |
   4642.7 |
   4843.2 |
   5043.6 |
   5244.1 |
   5444.6 |
   5645.0 |
   5845.5 |
   6046.0 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2001999.6-2019896.5 ns)
  2001999.6 |########################################
  2002894.4 |
  2003789.3 |
  2004684.1 |
  2005579.0 |
  2006473.8 |
  2007368.7 |
  2008263.5 |
  2009158.3 |
  2010053.2 |########################################
  2010948.0 |
  2011842.9 |########################################
  2012737.7 |
  2013632.6 |
  2014527.4 |########################################
  2015422.2 |
  2016317.1 |
  2017211.9 |
  2018106.8 |########################################
  2019001.6 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2005328.7-2018255.4 ns)
  2005328.7 |########################################
  2005975.0 |
  2006621.4 |
  2007267.7 |
  2007914.1 |
  2008560.4 |
  2009206.7 |
  2009853.1 |
  2010499.4 |
  2011145.7 |
  2011792.1 |
  2012438.4 |####################
  2013084.8 |
  2013731.1 |####################
  2014377.4 |
  2015023.8 |####################
  2015670.1 |
  2016316.4 |
  2016962.8 |
  2017609.1 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2028255.4-2070669.8 ns)
  2028255.4 |########################################
  2030376.1 |####################
  2032496.8 |
  2034617.6 |####################
  2036738.3 |
  2038859.0 |####################
  2040979.7 |
  2043100.4 |
  2045221.2 |
  2047341.9 |
  2049462.6 |
  2051583.3 |
  2053704.0 |
  2055824.8 |
  2057945.5 |
  2060066.2 |
  2062186.9 |
  2064307.6 |
  2066428.4 |
  2068549.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: CV=68.6% (high variance, measurements may be unstable)
- **abi_entry_form_tight_null_entry**: bridge=4939.5% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.0% of algo (FFI overhead may distort results)
