# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_wideselect_null_entry dominates: 76517% faster than the next best (abi_entry_form_wideselect_per_w_set)

abi_entry_form_wideselect_null_entry (2.70 us) leads abi_entry_form_wideselect_per_w_set (2.07 ms) by 76517%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.07 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_dispatch_table is an outlier: 768.1x slower than the field

abi_entry_form_wideselect_dispatch_table (2.07 ms) is 768.1x the fastest (2.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_wideselect_per_w_set shows alternating (throttle bounce) (autocorr -0.50)

abi_entry_form_wideselect_per_w_set's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_dispatch_table} (76517% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_dispatch_table} with a 76517% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 768.1x the fastest

Fastest abi_entry_form_wideselect_null_entry (2.70 us) to slowest abi_entry_form_wideselect_dispatch_table (2.07 ms): 768.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 2695.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 768.07x (fastest 2695.8 ns, slowest 2070602.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2077204ns | 2073627ns | 2064647ns | 2072731ns | 2090192ns | +0.24% |
| abi_entry_form_wideselect_null_entry | 4960ns | 4975ns | 4804ns | 4939ns | 5069ns | -99.76% |
| abi_entry_form_wideselect_per_w_set | 2068993ns | 2068428ns | 2062138ns | 2067368ns | 2074860ns | -0.16% |
| abi_entry_form_wideselect_runtime_w | 2072232ns | 2073062ns | 2066862ns | 2071438ns | 2076108ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2071365ns | 2071232ns | 2062911ns | 2070141ns | 2077427ns | -0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2074071ns | 2061919ns | 2086731ns | +0.24% | 0.000 |
| abi_entry_form_wideselect_null_entry | 2695ns | 2627ns | 2759ns | -99.87% | 0.047 |
| abi_entry_form_wideselect_per_w_set | 2066006ns | 2059072ns | 2071790ns | -0.15% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2069160ns | 2064032ns | 2072995ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2068338ns | 2060030ns | 2074236ns | -0.04% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 60311.9 | 2074999.3 | 2074070.9 | n/a |
| abi_entry_form_wideselect_null_entry | 28003.1 | 2734.4 | 2695.3 | n/a |
| abi_entry_form_wideselect_per_w_set | 59147.5 | 2066249.0 | 2066005.9 | 0 |
| abi_entry_form_wideselect_runtime_w | 58987.2 | 2067336.0 | 2069160.4 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 57392.9 | 2068585.8 | 2068338.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_wideselect_null_entry | 0.047 | 97.4% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.1% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.1% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2077204ns | 2077204ns | +0.24% |
| abi_entry_form_wideselect_null_entry | 4960ns | 4960ns | -99.76% |
| abi_entry_form_wideselect_per_w_set | 2068993ns | 2068993ns | -0.16% |
| abi_entry_form_wideselect_runtime_w | 2072232ns | 2072232ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2071365ns | 2071365ns | -0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2069854ns | base | --- | [2064632, 2072995] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2070602ns | no significant difference | [-5838, +18822]ns | [2064879, 2086731] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_wideselect_null_entry | 2696ns | -2067157.9ns (-99.9%) | [-2070295, -2061942]ns | [2631, 2759] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2065475ns | no significant difference | [-7081, +1497]ns | [2060752, 2071790] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2068231ns | no significant difference | [-6467, +4634]ns | [2062548, 2074236] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2069121ns | -0.3% | -99.9% | -0.0% | -0.4% |
| 2 | 2065233ns | +0.7% | -99.9% | -0.1% | -0.0% |
| 3 | 2064032ns | +0.3% | -99.9% | -0.2% | +0.4% |
| 4 | 2070586ns | +1.1% | -99.9% | +0.2% | -0.2% |
| 5 | 2070818ns | -0.1% | -99.9% | -0.4% | -0.1% |
| 6 | 2075172ns | -0.2% | -99.9% | -0.3% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.435 | moderate- |
| abi_entry_form_wideselect_null_entry | -0.195 | ok |
| abi_entry_form_wideselect_per_w_set | -0.502 | HIGH- (thermal bounce) |
| abi_entry_form_wideselect_runtime_w | 0.306 | moderate+ |
| abi_entry_form_wideselect_scalar_anchor | 0.078 | ok |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 4/6, lost 1/6
- **abi_entry_form_wideselect_scalar_anchor**: won 2/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6289189.1ns | 2074070.9ns | 303.2% | HIGH |
| abi_entry_form_wideselect_null_entry | 119869.2ns | 2695.3ns | 4447.4% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6256740.9ns | 2066005.9ns | 302.8% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6267523.4ns | 2069160.4ns | 302.9% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6268720.8ns | 2068338.5ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2061919.2-2086731.4 ns)
  2061919.2 |########################################
  2063159.8 |
  2064400.4 |
  2065641.0 |
  2066881.6 |########################################
  2068122.3 |
  2069362.9 |########################################
  2070603.5 |########################################
  2071844.1 |
  2073084.7 |
  2074325.3 |
  2075565.9 |
  2076806.6 |
  2078047.2 |
  2079287.8 |########################################
  2080528.4 |
  2081769.0 |
  2083009.6 |
  2084250.2 |
  2085490.8 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 2626.7-2759.2 ns)
   2626.7 |####################
   2633.3 |########################################
   2639.9 |
   2646.6 |
   2653.2 |
   2659.8 |
   2666.4 |
   2673.1 |
   2679.7 |
   2686.3 |
   2692.9 |
   2699.5 |
   2706.2 |
   2712.8 |
   2719.4 |
   2726.0 |
   2732.7 |
   2739.3 |
   2745.9 |####################
   2752.5 |####################
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2059072.5-2071790.0 ns)
  2059072.5 |####################
  2059708.4 |
  2060344.2 |
  2060980.1 |
  2061616.0 |
  2062251.9 |########################################
  2062887.8 |
  2063523.6 |
  2064159.5 |
  2064795.4 |
  2065431.2 |
  2066067.1 |
  2066703.0 |
  2067338.9 |
  2067974.8 |####################
  2068610.6 |####################
  2069246.5 |
  2069882.4 |
  2070518.2 |
  2071154.1 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2064031.7-2072995.2 ns)
  2064031.7 |########################################
  2064479.9 |
  2064928.1 |########################################
  2065376.2 |
  2065824.4 |
  2066272.6 |
  2066720.8 |
  2067168.9 |
  2067617.1 |
  2068065.3 |
  2068513.5 |
  2068961.6 |########################################
  2069409.8 |
  2069858.0 |
  2070306.2 |########################################
  2070754.3 |########################################
  2071202.5 |
  2071650.7 |
  2072098.9 |
  2072547.0 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2060030.4-2074236.2 ns)
  2060030.4 |########################################
  2060740.7 |
  2061451.0 |
  2062161.3 |
  2062871.6 |
  2063581.9 |
  2064292.2 |
  2065002.4 |########################################
  2065712.7 |
  2066423.0 |########################################
  2067133.3 |
  2067843.6 |
  2068553.9 |
  2069264.2 |########################################
  2069974.5 |
  2070684.8 |
  2071395.1 |
  2072105.4 |
  2072815.7 |########################################
  2073526.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=4461.8% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=303.2% of algo (FFI overhead may distort results)
