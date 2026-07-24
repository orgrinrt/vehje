# abi_marshal (madd)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_madd_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_madd_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_madd_marshal_null dominates: 9815% faster than the next best (abi_marshal_madd_aos)

abi_marshal_madd_marshal_null (27.09 us) leads abi_marshal_madd_aos (2.69 ms) by 9815%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_madd_marshal_null beats baseline by 99% (significant)

abi_marshal_madd_marshal_null is -2.66 ms (99%) faster than baseline abi_marshal_madd_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_madd_soa_transposed is an outlier: 101.6x slower than the field

abi_marshal_madd_soa_transposed (2.75 ms) is 101.6x the fastest (27.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_madd_marshal_null} vs {abi_marshal_madd_aos, abi_marshal_madd_soa_native, abi_marshal_madd_soa_transposed} (9815% apart)

The field splits into a fast tier {abi_marshal_madd_marshal_null} and a slow tier {abi_marshal_madd_aos, abi_marshal_madd_soa_native, abi_marshal_madd_soa_transposed} with a 9815% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 101.6x the fastest

Fastest abi_marshal_madd_marshal_null (27.09 us) to slowest abi_marshal_madd_soa_transposed (2.75 ms): 101.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_madd_marshal_null** at 27090.8 ns median (-99.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 101.58x (fastest 27090.8 ns, slowest 2751922.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2688400ns | 2688554ns | 2674196ns | 2685453ns | 2699921ns | base |
| abi_marshal_madd_marshal_null | 29310ns | 29344ns | 28689ns | 29163ns | 29839ns | -98.91% |
| abi_marshal_madd_soa_native | 2722207ns | 2722232ns | 2712401ns | 2719854ns | 2730640ns | +1.26% |
| abi_marshal_madd_soa_transposed | 2759016ns | 2754483ns | 2752339ns | 2753951ns | 2769951ns | +2.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2685869ns | 2671730ns | 2697360ns | base | 0.000 |
| abi_marshal_madd_marshal_null | 27071ns | 26510ns | 27585ns | -98.99% | 0.001 |
| abi_marshal_madd_soa_native | 2719604ns | 2709667ns | 2727982ns | +1.26% | 0.000 |
| abi_marshal_madd_soa_transposed | 2756458ns | 2749836ns | 2767395ns | +2.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_madd_aos | 41116.9 | 2688946.8 | 2685869.4 | n/a |
| abi_marshal_madd_marshal_null | 27645.1 | 27581.7 | 27070.9 | n/a |
| abi_marshal_madd_soa_native | 41825.3 | 2717962.3 | 2719604.0 | n/a |
| abi_marshal_madd_soa_transposed | 41518.0 | 2753639.7 | 2756457.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_marshal_madd_marshal_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_madd_aos | 0.000 | 1.0% |
| abi_marshal_madd_marshal_null | 0.001 | 97.9% |
| abi_marshal_madd_soa_native | 0.000 | 1.0% |
| abi_marshal_madd_soa_transposed | 0.000 | 1.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_madd_aos | 2688400ns | 2688400ns | base |
| abi_marshal_madd_marshal_null | 29310ns | 29310ns | -98.91% |
| abi_marshal_madd_soa_native | 2722207ns | 2722207ns | +1.26% |
| abi_marshal_madd_soa_transposed | 2759016ns | 2759016ns | +2.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2686032ns | base | --- | [2674216, 2697360] | --- | --- | --- | --- |
| abi_marshal_madd_marshal_null | 27091ns | -2658941.6ns (-99.0%) | [-2670278, -2647176]ns | [26537, 27585] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_madd_soa_native | 2719670ns | +32187.8ns (+1.2%) | [+26361, +42655]ns | [2711159, 2727982] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_madd_soa_transposed | 2751922ns | +69175.4ns (+2.6%) | [+55182, +87407]ns | [2750056, 2767395] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_madd_aos | abi_marshal_madd_marshal_null | abi_marshal_madd_soa_native | abi_marshal_madd_soa_transposed |
|---|---|---|---|---|
| 1 | 2681242ns | -99.0% | +1.2% | +2.7% |
| 2 | 2676701ns | -99.0% | +1.2% | +2.8% |
| 3 | 2671730ns | -99.0% | +2.0% | +3.7% |
| 4 | 2698925ns | -99.0% | +1.2% | +1.9% |
| 5 | 2690823ns | -99.0% | +0.9% | +2.2% |
| 6 | 2695795ns | -99.0% | +1.0% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_madd_aos | 0.169 | ok |
| abi_marshal_madd_marshal_null | -0.128 | ok |
| abi_marshal_madd_soa_native | 0.029 | ok |
| abi_marshal_madd_soa_transposed | -0.389 | moderate- |

**Consistency summary:**

- **abi_marshal_madd_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_madd_soa_native**: won 0/6, lost 6/6
- **abi_marshal_madd_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_madd_aos | 8101952.2ns | 2685869.4ns | 301.7% | HIGH |
| abi_marshal_madd_marshal_null | 190010.4ns | 27070.9ns | 701.9% | HIGH |
| abi_marshal_madd_soa_native | 8205691.1ns | 2719604.0ns | 301.7% | HIGH |
| abi_marshal_madd_soa_transposed | 8309125.5ns | 2756457.6ns | 301.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_madd_aos (n=6, range 2671730.0-2697360.0 ns)
  2671730.0 |########################################
  2673011.5 |
  2674293.0 |
  2675574.5 |########################################
  2676856.0 |
  2678137.5 |
  2679419.0 |
  2680700.5 |########################################
  2681982.0 |
  2683263.5 |
  2684545.0 |
  2685826.5 |
  2687108.0 |
  2688389.5 |
  2689671.0 |########################################
  2690952.5 |
  2692234.0 |
  2693515.5 |
  2694797.0 |########################################
  2696078.5 |
  (0 below, 1 above range)

abi_marshal_madd_marshal_null (n=6, range 26510.0-27584.8 ns)
  26510.0 |########################################
  26563.7 |########################################
  26617.5 |########################################
  26671.2 |
  26725.0 |
  26778.7 |
  26832.4 |
  26886.2 |
  26939.9 |
  26993.7 |
  27047.4 |
  27101.1 |
  27154.9 |
  27208.6 |
  27262.4 |
  27316.1 |
  27369.8 |
  27423.6 |
  27477.3 |########################################
  27531.1 |########################################
  (0 below, 1 above range)

abi_marshal_madd_soa_native (n=6, range 2709666.7-2727982.5 ns)
  2709666.7 |####################
  2710582.5 |
  2711498.3 |
  2712414.1 |####################
  2713329.9 |
  2714245.7 |
  2715161.4 |####################
  2716077.2 |
  2716993.0 |
  2717908.8 |
  2718824.6 |
  2719740.4 |
  2720656.2 |
  2721572.0 |
  2722487.8 |
  2723403.5 |########################################
  2724319.3 |
  2725235.1 |
  2726150.9 |
  2727066.7 |
  (0 below, 1 above range)

abi_marshal_madd_soa_transposed (n=6, range 2749835.8-2767394.5 ns)
  2749835.8 |########################################
  2750713.7 |####################
  2751591.7 |####################
  2752469.6 |
  2753347.5 |
  2754225.5 |
  2755103.4 |
  2755981.4 |
  2756859.3 |
  2757737.2 |
  2758615.2 |
  2759493.1 |
  2760371.0 |
  2761249.0 |
  2762126.9 |
  2763004.9 |####################
  2763882.8 |
  2764760.7 |
  2765638.7 |
  2766516.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_madd_aos**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_marshal_madd_marshal_null**: bridge=699.6% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_native**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_transposed**: bridge=301.5% of algo (FFI overhead may distort results)
