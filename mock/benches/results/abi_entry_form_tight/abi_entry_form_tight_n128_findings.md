# abi_entry_form (tight)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_tight_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_tight_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_tight_null_entry dominates: 74102% faster than the next best (abi_entry_form_tight_per_w_set)

abi_entry_form_tight_null_entry (2.71 us) leads abi_entry_form_tight_per_w_set (2.01 ms) by 74102%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_tight_null_entry beats baseline by 100% (significant)

abi_entry_form_tight_null_entry is -2.02 ms (100%) faster than baseline abi_entry_form_tight_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_tight_scalar_anchor is an outlier: 751.8x slower than the field

abi_entry_form_tight_scalar_anchor (2.04 ms) is 751.8x the fastest (2.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_tight_null_entry} vs {abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_scalar_anchor} (74102% apart)

The field splits into a fast tier {abi_entry_form_tight_null_entry} and a slow tier {abi_entry_form_tight_per_w_set, abi_entry_form_tight_dispatch_table, abi_entry_form_tight_runtime_w, abi_entry_form_tight_scalar_anchor} with a 74102% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 751.8x the fastest

Fastest abi_entry_form_tight_null_entry (2.71 us) to slowest abi_entry_form_tight_scalar_anchor (2.04 ms): 751.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_tight_null_entry** at 2714.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 751.84x (fastest 2714.4 ns, slowest 2040806.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2021140ns | 2020333ns | 2014248ns | 2018462ns | 2028602ns | -0.42% |
| abi_entry_form_tight_null_entry | 5143ns | 5040ns | 4870ns | 5007ns | 5484ns | -99.75% |
| abi_entry_form_tight_per_w_set | 2037582ns | 2017034ns | 2015403ns | 2016907ns | 2079685ns | +0.39% |
| abi_entry_form_tight_runtime_w | 2029709ns | 2021998ns | 2013872ns | 2021330ns | 2050197ns | base |
| abi_entry_form_tight_scalar_anchor | 2121439ns | 2044025ns | 2031848ns | 2041586ns | 2286015ns | +4.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2018102ns | 2011438ns | 2025362ns | -0.42% | 0.000 |
| abi_entry_form_tight_null_entry | 2786ns | 2657ns | 2979ns | -99.86% | 0.046 |
| abi_entry_form_tight_per_w_set | 2034620ns | 2012555ns | 2076536ns | +0.40% | 0.000 |
| abi_entry_form_tight_runtime_w | 2026562ns | 2011159ns | 2046816ns | base | 0.000 |
| abi_entry_form_tight_scalar_anchor | 2118208ns | 2028953ns | 2282502ns | +4.52% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 57573.4 | 2017548.4 | 2018102.3 | n/a |
| abi_entry_form_tight_null_entry | 28906.6 | 2824.3 | 2786.0 | n/a |
| abi_entry_form_tight_per_w_set | 54559.0 | 2028256.0 | 2034620.2 | 1 |
| abi_entry_form_tight_runtime_w | 58802.7 | 2028995.9 | 2026562.2 | n/a |
| abi_entry_form_tight_scalar_anchor | 71755.2 | 2144737.0 | 2118207.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_entry_form_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_tight_null_entry | 0.047 | 97.9% |
| abi_entry_form_tight_per_w_set | 0.000 | 0.1% |
| abi_entry_form_tight_runtime_w | 0.000 | 0.1% |
| abi_entry_form_tight_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 2021140ns | 2021140ns | -0.42% |
| abi_entry_form_tight_null_entry | 5143ns | 5143ns | -99.75% |
| abi_entry_form_tight_per_w_set | 2037582ns | 2037582ns | +0.39% |
| abi_entry_form_tight_runtime_w | 2029709ns | 2029709ns | base |
| abi_entry_form_tight_scalar_anchor | 2121439ns | 2121439ns | +4.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_tight_runtime_w | 2018870ns | base | --- | [2014000, 2046816] | --- | --- | --- | --- |
| abi_entry_form_tight_dispatch_table | 2017301ns | no significant difference | [-35173, +10258]ns | [2011644, 2025362] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_tight_null_entry | 2714ns | -2016010.4ns (-99.9%) | [-2043992, -2011327]ns | [2665, 2979] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_tight_per_w_set | 2014131ns | no significant difference | [-33383, +61476]ns | [2013194, 2076536] | no | 0.6875 | 0.6875 | 0 |
| abi_entry_form_tight_scalar_anchor | 2040807ns | +19025.2ns (+0.9%) | [+13932, +241979]ns | [2031314, 2282502] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_tight_runtime_w | abi_entry_form_tight_dispatch_table | abi_entry_form_tight_null_entry | abi_entry_form_tight_per_w_set | abi_entry_form_tight_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2011159ns | +1.0% | -99.9% | +0.1% | +1.1% |
| 2 | 2018691ns | -0.0% | -99.9% | -0.1% | +0.5% |
| 3 | 2019049ns | +0.1% | -99.8% | -0.3% | +0.9% |
| 4 | 2066743ns | -2.7% | -99.9% | -2.6% | +1.0% |
| 5 | 2016841ns | -0.0% | -99.9% | +6.0% | +22.9% |
| 6 | 2026890ns | -0.8% | -99.9% | -0.6% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_tight_dispatch_table | 0.014 | ok |
| abi_entry_form_tight_null_entry | 0.001 | ok |
| abi_entry_form_tight_per_w_set | -0.247 | moderate- |
| abi_entry_form_tight_runtime_w | -0.250 | moderate- |
| abi_entry_form_tight_scalar_anchor | -0.129 | ok |

**Consistency summary:**

- **abi_entry_form_tight_dispatch_table**: won 2/6, lost 1/6
- **abi_entry_form_tight_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_tight_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_tight_scalar_anchor**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_tight_dispatch_table | 6112600.7ns | 2018102.3ns | 302.9% | HIGH |
| abi_entry_form_tight_null_entry | 121307.5ns | 2786.0ns | 4354.2% | HIGH |
| abi_entry_form_tight_per_w_set | 6186087.7ns | 2034620.2ns | 304.0% | HIGH |
| abi_entry_form_tight_runtime_w | 6154943.9ns | 2026562.2ns | 303.7% | HIGH |
| abi_entry_form_tight_scalar_anchor | 6426267.5ns | 2118207.6ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_tight_dispatch_table (n=6, range 2011438.3-2025362.2 ns)
  2011438.3 |########################################
  2012134.5 |
  2012830.7 |
  2013526.9 |
  2014223.1 |
  2014919.3 |
  2015615.5 |
  2016311.7 |####################
  2017007.9 |
  2017704.1 |####################
  2018400.3 |
  2019096.5 |
  2019792.7 |####################
  2020488.9 |
  2021185.1 |
  2021881.3 |
  2022577.5 |
  2023273.7 |
  2023969.9 |
  2024666.1 |
  (0 below, 1 above range)

abi_entry_form_tight_null_entry (n=6, range 2657.1-2978.8 ns)
   2657.1 |########################################
   2673.2 |####################
   2689.3 |
   2705.3 |
   2721.4 |
   2737.5 |
   2753.6 |####################
   2769.7 |
   2785.8 |
   2801.8 |
   2817.9 |
   2834.0 |
   2850.1 |
   2866.2 |
   2882.3 |####################
   2898.3 |
   2914.4 |
   2930.5 |
   2946.6 |
   2962.7 |
  (0 below, 1 above range)

abi_entry_form_tight_per_w_set (n=6, range 2012554.6-2076536.4 ns)
  2012554.6 |########################################
  2015753.7 |##########
  2018952.8 |
  2022151.9 |
  2025351.0 |
  2028550.1 |
  2031749.2 |
  2034948.2 |
  2038147.3 |
  2041346.4 |
  2044545.5 |
  2047744.6 |
  2050943.7 |
  2054142.8 |
  2057341.9 |
  2060541.0 |
  2063740.1 |
  2066939.2 |
  2070138.3 |
  2073337.4 |
  (0 below, 1 above range)

abi_entry_form_tight_runtime_w (n=6, range 2011158.7-2046816.5 ns)
  2011158.7 |####################
  2012941.6 |
  2014724.5 |
  2016507.4 |####################
  2018290.2 |########################################
  2020073.1 |
  2021856.0 |
  2023638.9 |
  2025421.8 |####################
  2027204.7 |
  2028987.6 |
  2030770.5 |
  2032553.4 |
  2034336.2 |
  2036119.1 |
  2037902.0 |
  2039684.9 |
  2041467.8 |
  2043250.7 |
  2045033.6 |
  (0 below, 1 above range)

abi_entry_form_tight_scalar_anchor (n=6, range 2028952.9-2282501.6 ns)
  2028952.9 |########################################
  2041630.3 |#############
  2054307.8 |
  2066985.2 |
  2079662.6 |#############
  2092340.1 |
  2105017.5 |
  2117695.0 |
  2130372.4 |
  2143049.8 |
  2155727.3 |
  2168404.7 |
  2181082.1 |
  2193759.6 |
  2206437.0 |
  2219114.5 |
  2231791.9 |
  2244469.3 |
  2257146.8 |
  2269824.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_tight_dispatch_table**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_null_entry**: bridge=4384.0% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_per_w_set**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_runtime_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_tight_scalar_anchor**: bridge=303.6% of algo (FFI overhead may distort results)
