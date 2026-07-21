# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Key findings

- **Baseline (br_branch_c_rand50) is the fastest** at 133.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.61x (fastest 133.6 ns, slowest 214.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 2872ns | 2905ns | 2292ns | 2839ns | 3213ns | base |
| br_lut_c_rand50 | 2879ns | 2994ns | 2281ns | 2991ns | 3010ns | +0.24% |
| br_mask_c_rand50 | 2933ns | 3009ns | 2323ns | 2989ns | 3154ns | +2.12% |
| br_predicate_c_rand50 | 2854ns | 2981ns | 2344ns | 2909ns | 3026ns | -0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 134ns | 108ns | 150ns | base | 0.479 |
| br_lut_c_rand50 | 186ns | 149ns | 195ns | +39.38% | 0.344 |
| br_mask_c_rand50 | 206ns | 164ns | 225ns | +54.29% | 0.311 |
| br_predicate_c_rand50 | 189ns | 145ns | 216ns | +41.86% | 0.338 |

## Performance model

- Peak throughput: **0.591 Gops/s** (br_branch_c_rand50; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.479 | 81.1% |
| br_lut_c_rand50 | 0.331 | 56.1% |
| br_mask_c_rand50 | 0.298 | 50.5% |
| br_predicate_c_rand50 | 0.335 | 56.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 2872ns | 2872ns | base |
| br_lut_c_rand50 | 2879ns | 2879ns | +0.24% |
| br_mask_c_rand50 | 2933ns | 2933ns | +2.12% |
| br_predicate_c_rand50 | 2854ns | 2854ns | -0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 134ns | base | --- | [117, 150] | --- | --- | --- | --- |
| br_lut_c_rand50 | 193ns | +51.1ns (+38.2%) | [+42, +65]ns | [170, 195] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 215ns | +70.8ns (+53.0%) | [+61, +86]ns | [179, 225] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 191ns | +52.5ns (+39.3%) | [+38, +77]ns | [162, 216] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 108ns | +37.8% | +51.2% | +34.3% |
| 2 | 139ns | +39.2% | +53.5% | +37.7% |
| 3 | 128ns | +48.9% | +51.5% | +49.2% |
| 4 | 152ns | +28.2% | +53.4% | +25.4% |
| 5 | 148ns | +32.0% | +45.2% | +61.8% |
| 6 | 125ns | +53.5% | +72.4% | +41.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | -0.094 | ok |
| br_lut_c_rand50 | -0.039 | ok |
| br_mask_c_rand50 | -0.137 | ok |
| br_predicate_c_rand50 | -0.131 | ok |

**Consistency summary:**

- **br_lut_c_rand50**: won 0/6, lost 6/6
- **br_mask_c_rand50**: won 0/6, lost 6/6
- **br_predicate_c_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 4.6ns | 133.5ns | 3.4% |  |
| br_lut_c_rand50 | 3.8ns | 186.1ns | 2.1% |  |
| br_mask_c_rand50 | 5.0ns | 206.0ns | 2.4% |  |
| br_predicate_c_rand50 | 4.5ns | 189.4ns | 2.4% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 108.3-150.2 ns)
    108.3 |########################################
    110.4 |
    112.5 |
    114.6 |
    116.7 |
    118.8 |
    120.9 |
    123.0 |
    125.1 |########################################
    127.2 |########################################
    129.2 |
    131.3 |
    133.4 |
    135.5 |
    137.6 |########################################
    139.7 |
    141.8 |
    143.9 |
    146.0 |
    148.1 |########################################
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 149.2-195.4 ns)
    149.2 |####################
    151.5 |
    153.8 |
    156.1 |
    158.4 |
    160.8 |
    163.1 |
    165.4 |
    167.7 |
    170.0 |
    172.3 |
    174.6 |
    176.9 |
    179.2 |
    181.5 |
    183.8 |
    186.2 |
    188.5 |####################
    190.8 |####################
    193.1 |########################################
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 163.8-224.8 ns)
    163.8 |####################
    166.8 |
    169.9 |
    172.9 |
    176.0 |
    179.0 |
    182.1 |
    185.1 |
    188.2 |
    191.2 |####################
    194.3 |
    197.3 |
    200.4 |
    203.4 |
    206.5 |
    209.5 |
    212.6 |########################################
    215.6 |####################
    218.7 |
    221.7 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 145.4-215.8 ns)
    145.4 |####################
    148.9 |
    152.4 |
    156.0 |
    159.5 |
    163.0 |
    166.5 |
    170.1 |
    173.6 |
    177.1 |####################
    180.6 |
    184.1 |
    187.7 |########################################
    191.2 |####################
    194.7 |
    198.2 |
    201.8 |
    205.3 |
    208.8 |
    212.3 |
  (0 below, 1 above range)

```
