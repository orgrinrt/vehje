# Branch strategies, cheap-arm, biased25: ~25% taken (b<64)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_biased25**

## Key findings

- **Baseline (br_branch_c_biased25) is the fastest** at 116.7 ns median
- 3 variants significantly slower than baseline
- Spread: 1.51x (fastest 116.7 ns, slowest 176.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 2675ns | 2699ns | 2527ns | 2696ns | 2719ns | base |
| br_lut_c_biased25 | 2801ns | 2814ns | 2575ns | 2737ns | 3010ns | +4.69% |
| br_mask_c_biased25 | 2712ns | 2748ns | 2573ns | 2731ns | 2753ns | +1.36% |
| br_predicate_c_biased25 | 2775ns | 2762ns | 2754ns | 2760ns | 2807ns | +3.71% |
| br_profiled_hot_c_biased25 | 2706ns | 2710ns | 2555ns | 2684ns | 2815ns | +1.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_biased25 | 115ns | 106ns | 118ns | base | 0.557 |
| br_lut_c_biased25 | 177ns | 165ns | 189ns | +54.02% | 0.362 |
| br_mask_c_biased25 | 171ns | 160ns | 177ns | +49.07% | 0.374 |
| br_predicate_c_biased25 | 176ns | 173ns | 179ns | +53.29% | 0.364 |
| br_profiled_hot_c_biased25 | 125ns | 105ns | 131ns | +8.43% | 0.514 |

## Performance model

- Peak throughput: **0.612 Gops/s** (br_profiled_hot_c_biased25; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_biased25 | 0.549 | 89.7% |
| br_lut_c_biased25 | 0.363 | 59.3% |
| br_mask_c_biased25 | 0.368 | 60.2% |
| br_predicate_c_biased25 | 0.365 | 59.6% |
| br_profiled_hot_c_biased25 | 0.502 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_biased25 | 2675ns | 2675ns | base |
| br_lut_c_biased25 | 2801ns | 2801ns | +4.69% |
| br_mask_c_biased25 | 2712ns | 2712ns | +1.36% |
| br_predicate_c_biased25 | 2775ns | 2775ns | +3.71% |
| br_profiled_hot_c_biased25 | 2706ns | 2706ns | +1.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_biased25 | 117ns | base | --- | [110, 118] | --- | --- | --- | --- |
| br_lut_c_biased25 | 176ns | +58.8ns (+50.4%) | [+50, +77]ns | [165, 189] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_biased25 | 174ns | +55.9ns (+47.9%) | [+47, +66]ns | [163, 177] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_biased25 | 175ns | +61.1ns (+52.3%) | [+58, +65]ns | [174, 179] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_biased25 | 128ns | no significant difference | [-1, +20]ns | [115, 131] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_biased25 | br_lut_c_biased25 | br_mask_c_biased25 | br_predicate_c_biased25 | br_profiled_hot_c_biased25 |
|---|---|---|---|---|---|
| 1 | 114ns | +44.8% | +45.1% | +54.3% | -8.0% |
| 2 | 106ns | +86.0% | +67.1% | +63.2% | +22.8% |
| 3 | 117ns | +50.1% | +37.0% | +53.0% | +6.4% |
| 4 | 118ns | +49.7% | +47.9% | +48.3% | +10.2% |
| 5 | 118ns | +53.5% | +46.8% | +51.4% | +12.8% |
| 6 | 116ns | +42.7% | +52.1% | +50.6% | +7.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_biased25 | 0.108 | ok |
| br_lut_c_biased25 | -0.440 | moderate- |
| br_mask_c_biased25 | -0.564 | HIGH- (thermal bounce) |
| br_predicate_c_biased25 | -0.527 | HIGH- (thermal bounce) |
| br_profiled_hot_c_biased25 | -0.129 | ok |

**Consistency summary:**

- **br_lut_c_biased25**: won 0/6, lost 6/6
- **br_mask_c_biased25**: won 0/6, lost 6/6
- **br_predicate_c_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_c_biased25**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_biased25 | 4.5ns | 114.8ns | 3.9% |  |
| br_lut_c_biased25 | 3.1ns | 176.9ns | 1.8% |  |
| br_mask_c_biased25 | 3.5ns | 171.2ns | 2.1% |  |
| br_predicate_c_biased25 | 2.5ns | 176.0ns | 1.4% |  |
| br_profiled_hot_c_biased25 | 3.9ns | 124.5ns | 3.1% |  |

## Distribution (algo ns)

```
br_branch_c_biased25 (n=6, range 106.2-117.9 ns)
    106.2 |########################################
    106.8 |
    107.4 |
    108.0 |
    108.5 |
    109.1 |
    109.7 |
    110.3 |
    110.9 |
    111.5 |
    112.1 |
    112.6 |
    113.2 |########################################
    113.8 |
    114.4 |
    115.0 |
    115.6 |
    116.1 |########################################
    116.7 |########################################
    117.3 |########################################
  (0 below, 1 above range)

br_lut_c_biased25 (n=6, range 164.6-188.9 ns)
    164.6 |########################################
    165.8 |
    167.0 |
    168.3 |
    169.5 |
    170.7 |
    171.9 |
    173.1 |
    174.3 |
    175.6 |####################
    176.8 |####################
    178.0 |
    179.2 |####################
    180.4 |
    181.6 |
    182.9 |
    184.1 |
    185.3 |
    186.5 |
    187.7 |
  (0 below, 1 above range)

br_mask_c_biased25 (n=6, range 160.4-177.1 ns)
    160.4 |########################################
    161.2 |
    162.1 |
    162.9 |
    163.7 |
    164.6 |########################################
    165.4 |
    166.2 |
    167.1 |
    167.9 |
    168.8 |
    169.6 |
    170.4 |
    171.3 |
    172.1 |########################################
    172.9 |
    173.8 |
    174.6 |########################################
    175.4 |
    176.3 |########################################
  (0 below, 1 above range)

br_predicate_c_biased25 (n=6, range 173.3-178.6 ns)
    173.3 |####################
    173.6 |
    173.8 |
    174.1 |
    174.4 |
    174.6 |
    174.9 |####################
    175.1 |########################################
    175.4 |
    175.7 |
    175.9 |
    176.2 |
    176.5 |
    176.7 |
    177.0 |
    177.2 |
    177.5 |
    177.8 |####################
    178.0 |
    178.3 |
  (0 below, 1 above range)

br_profiled_hot_c_biased25 (n=6, range 104.6-131.4 ns)
    104.6 |####################
    105.9 |
    107.3 |
    108.6 |
    110.0 |
    111.3 |
    112.7 |
    114.0 |
    115.3 |
    116.7 |
    118.0 |
    119.4 |
    120.7 |
    122.1 |
    123.4 |########################################
    124.7 |
    126.1 |
    127.4 |
    128.8 |
    130.1 |########################################
  (0 below, 1 above range)

```
