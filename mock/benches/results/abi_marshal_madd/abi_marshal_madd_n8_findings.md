# abi_marshal (madd)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_madd_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_madd_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_madd_marshal_null dominates: 12704% faster than the next best (abi_marshal_madd_aos)

abi_marshal_madd_marshal_null (20.97 us) leads abi_marshal_madd_aos (2.68 ms) by 12704%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_madd_marshal_null beats baseline by 99% (significant)

abi_marshal_madd_marshal_null is -2.66 ms (99%) faster than baseline abi_marshal_madd_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_madd_soa_transposed is an outlier: 129.4x slower than the field

abi_marshal_madd_soa_transposed (2.71 ms) is 129.4x the fastest (20.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_madd_marshal_null} vs {abi_marshal_madd_aos, abi_marshal_madd_soa_native, abi_marshal_madd_soa_transposed} (12704% apart)

The field splits into a fast tier {abi_marshal_madd_marshal_null} and a slow tier {abi_marshal_madd_aos, abi_marshal_madd_soa_native, abi_marshal_madd_soa_transposed} with a 12704% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 129.4x the fastest

Fastest abi_marshal_madd_marshal_null (20.97 us) to slowest abi_marshal_madd_soa_transposed (2.71 ms): 129.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_madd_marshal_null** at 20965.4 ns median (-99.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 129.35x (fastest 20965.4 ns, slowest 2711940.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2702004ns | 2687060ns | 2657815ns | 2684313ns | 2750635ns | base |
| abi_marshal_madd_marshal_null | 23239ns | 23337ns | 22641ns | 23109ns | 23734ns | -99.14% |
| abi_marshal_madd_soa_native | 2710656ns | 2699776ns | 2683368ns | 2695800ns | 2746584ns | +0.32% |
| abi_marshal_madd_soa_transposed | 2717995ns | 2714554ns | 2711462ns | 2713742ns | 2727639ns | +0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2699289ns | 2655440ns | 2747606ns | base | 0.000 |
| abi_marshal_madd_marshal_null | 20902ns | 20344ns | 21366ns | -99.23% | 0.000 |
| abi_marshal_madd_soa_native | 2707946ns | 2680773ns | 2743827ns | +0.32% | 0.000 |
| abi_marshal_madd_soa_transposed | 2715408ns | 2708953ns | 2724987ns | +0.60% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_madd_aos | 45870.6 | 2705129.6 | 2699288.7 | n/a |
| abi_marshal_madd_marshal_null | 27784.6 | 21130.7 | 20902.4 | n/a |
| abi_marshal_madd_soa_native | 43386.9 | 2708630.9 | 2707946.0 | 0 |
| abi_marshal_madd_soa_transposed | 42175.8 | 2717720.9 | 2715407.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_madd_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_madd_aos | 0.000 | 0.8% |
| abi_marshal_madd_marshal_null | 0.000 | 97.0% |
| abi_marshal_madd_soa_native | 0.000 | 0.8% |
| abi_marshal_madd_soa_transposed | 0.000 | 0.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_madd_aos | 2702004ns | 2702004ns | base |
| abi_marshal_madd_marshal_null | 23239ns | 23239ns | -99.14% |
| abi_marshal_madd_soa_native | 2710656ns | 2710656ns | +0.32% |
| abi_marshal_madd_soa_transposed | 2717995ns | 2717995ns | +0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_madd_aos | 2684402ns | base | --- | [2665858, 2747606] | --- | --- | --- | --- |
| abi_marshal_madd_marshal_null | 20965ns | -2663229.2ns (-99.2%) | [-2726763, -2645167]ns | [20375, 21366] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_madd_soa_native | 2697002ns | no significant difference | [-25526, +37073]ns | [2683008, 2743827] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_madd_soa_transposed | 2711941ns | no significant difference | [-31859, +53158]ns | [2709296, 2724987] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_madd_aos | abi_marshal_madd_marshal_null | abi_marshal_madd_soa_native | abi_marshal_madd_soa_transposed |
|---|---|---|---|---|
| 1 | 2687504ns | -99.2% | +1.8% | +0.9% |
| 2 | 2796913ns | -99.2% | -1.6% | -3.1% |
| 3 | 2698298ns | -99.2% | -0.2% | +1.0% |
| 4 | 2676276ns | -99.2% | +0.3% | +1.9% |
| 5 | 2655440ns | -99.2% | +1.0% | +2.1% |
| 6 | 2681301ns | -99.2% | +0.7% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_madd_aos | 0.046 | ok |
| abi_marshal_madd_marshal_null | -0.134 | ok |
| abi_marshal_madd_soa_native | 0.408 | moderate+ |
| abi_marshal_madd_soa_transposed | 0.152 | ok |

**Consistency summary:**

- **abi_marshal_madd_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_madd_soa_native**: won 2/6, lost 4/6
- **abi_marshal_madd_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_madd_aos | 8159392.7ns | 2699288.7ns | 302.3% | HIGH |
| abi_marshal_madd_marshal_null | 170927.1ns | 20902.4ns | 817.7% | HIGH |
| abi_marshal_madd_soa_native | 8168444.0ns | 2707946.0ns | 301.6% | HIGH |
| abi_marshal_madd_soa_transposed | 8195007.9ns | 2715407.8ns | 301.8% | HIGH |

## Distribution (algo ns)

```
abi_marshal_madd_aos (n=6, range 2655440.0-2747605.8 ns)
  2655440.0 |########################################
  2660048.3 |
  2664656.6 |
  2669264.9 |
  2673873.2 |########################################
  2678481.5 |########################################
  2683089.7 |########################################
  2687698.0 |
  2692306.3 |
  2696914.6 |########################################
  2701522.9 |
  2706131.2 |
  2710739.5 |
  2715347.8 |
  2719956.1 |
  2724564.3 |
  2729172.6 |
  2733780.9 |
  2738389.2 |
  2742997.5 |
  (0 below, 1 above range)

abi_marshal_madd_marshal_null (n=6, range 20344.2-21366.2 ns)
  20344.2 |########################################
  20395.3 |########################################
  20446.4 |
  20497.5 |
  20548.6 |
  20599.7 |
  20650.8 |
  20701.9 |
  20753.0 |
  20804.1 |
  20855.2 |
  20906.3 |########################################
  20957.4 |########################################
  21008.5 |
  21059.6 |
  21110.7 |
  21161.8 |
  21212.9 |
  21264.0 |
  21315.1 |########################################
  (0 below, 1 above range)

abi_marshal_madd_soa_native (n=6, range 2680773.3-2743827.1 ns)
  2680773.3 |########################################
  2683926.0 |########################################
  2687078.7 |
  2690231.4 |########################################
  2693384.1 |
  2696536.8 |
  2699689.4 |########################################
  2702842.1 |
  2705994.8 |
  2709147.5 |
  2712300.2 |
  2715452.9 |
  2718605.6 |
  2721758.3 |
  2724911.0 |
  2728063.6 |
  2731216.3 |
  2734369.0 |########################################
  2737521.7 |
  2740674.4 |
  (0 below, 1 above range)

abi_marshal_madd_soa_transposed (n=6, range 2708953.3-2724987.1 ns)
  2708953.3 |########################################
  2709755.0 |
  2710556.7 |
  2711358.4 |########################################
  2712160.1 |
  2712961.8 |
  2713763.4 |
  2714565.1 |
  2715366.8 |
  2716168.5 |
  2716970.2 |
  2717771.9 |
  2718573.6 |
  2719375.3 |
  2720177.0 |
  2720978.6 |
  2721780.3 |
  2722582.0 |
  2723383.7 |####################
  2724185.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_madd_aos**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_marshal_madd_marshal_null**: bridge=816.0% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_native**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_marshal_madd_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)
