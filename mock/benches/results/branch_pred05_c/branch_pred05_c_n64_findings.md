# Branch strategies, cheap-arm, pred05: ~5% taken, predictable (b<13)

5 variants, 6 samples per variant.
Baseline: **br_branch_c_pred05**

## Key findings

- **Fastest: br_profiled_hot_c_pred05** at 121.9 ns median (-5.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.56x (fastest 121.9 ns, slowest 190.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 2951ns | 2924ns | 2525ns | 2915ns | 3219ns | base |
| br_lut_c_pred05 | 2806ns | 2757ns | 2587ns | 2755ns | 2992ns | -4.92% |
| br_mask_c_pred05 | 2826ns | 2975ns | 2282ns | 2901ns | 2986ns | -4.23% |
| br_predicate_c_pred05 | 2943ns | 2895ns | 2768ns | 2855ns | 3162ns | -0.28% |
| br_profiled_hot_c_pred05 | 2679ns | 2707ns | 2277ns | 2648ns | 2925ns | -9.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_pred05 | 128ns | 113ns | 136ns | base | 0.501 |
| br_lut_c_pred05 | 178ns | 167ns | 188ns | +39.68% | 0.359 |
| br_mask_c_pred05 | 182ns | 149ns | 193ns | +42.64% | 0.351 |
| br_predicate_c_pred05 | 183ns | 175ns | 190ns | +43.58% | 0.349 |
| br_profiled_hot_c_pred05 | 122ns | 104ns | 131ns | -4.74% | 0.526 |

## Performance model

- Peak throughput: **0.617 Gops/s** (br_profiled_hot_c_pred05; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_pred05 | 0.499 | 80.9% |
| br_lut_c_pred05 | 0.363 | 58.8% |
| br_mask_c_pred05 | 0.336 | 54.5% |
| br_predicate_c_pred05 | 0.346 | 56.2% |
| br_profiled_hot_c_pred05 | 0.525 | 85.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_pred05 | 2951ns | 2951ns | base |
| br_lut_c_pred05 | 2806ns | 2806ns | -4.92% |
| br_mask_c_pred05 | 2826ns | 2826ns | -4.23% |
| br_predicate_c_pred05 | 2943ns | 2943ns | -0.28% |
| br_profiled_hot_c_pred05 | 2679ns | 2679ns | -9.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_pred05 | 128ns | base | --- | [119, 136] | --- | --- | --- | --- |
| br_lut_c_pred05 | 176ns | +51.6ns (+40.2%) | [+38, +62]ns | [171, 188] | YES | 0.0417 | 0.0313 | 0 |
| br_mask_c_pred05 | 190ns | +56.2ns (+43.8%) | [+40, +68]ns | [163, 193] | YES | 0.0417 | 0.0313 | 0 |
| br_predicate_c_pred05 | 185ns | +54.2ns (+42.2%) | [+46, +67]ns | [176, 190] | YES | 0.0417 | 0.0313 | 0 |
| br_profiled_hot_c_pred05 | 122ns | no significant difference | [-11, +2]ns | [112, 131] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_pred05 | br_lut_c_pred05 | br_mask_c_pred05 | br_predicate_c_pred05 | br_profiled_hot_c_pred05 |
|---|---|---|---|---|---|
| 1 | 113ns | +54.5% | +31.2% | +60.4% | -8.4% |
| 2 | 124ns | +42.7% | +55.8% | +53.2% | +7.0% |
| 3 | 125ns | +49.8% | +53.1% | +40.4% | -4.0% |
| 4 | 139ns | +36.3% | +36.9% | +35.4% | -6.1% |
| 5 | 132ns | +33.4% | +46.3% | +43.8% | -9.5% |
| 6 | 134ns | +24.6% | +32.7% | +31.7% | -7.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_pred05 | 0.265 | moderate+ |
| br_lut_c_pred05 | 0.243 | moderate+ |
| br_mask_c_pred05 | -0.109 | ok |
| br_predicate_c_pred05 | -0.520 | HIGH- (thermal bounce) |
| br_profiled_hot_c_pred05 | -0.488 | moderate- |

**Consistency summary:**

- **br_lut_c_pred05**: won 0/6, lost 6/6
- **br_mask_c_pred05**: won 0/6, lost 6/6
- **br_predicate_c_pred05**: won 0/6, lost 6/6
- **br_profiled_hot_c_pred05**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_pred05 | 4.4ns | 127.7ns | 3.4% |  |
| br_lut_c_pred05 | 5.0ns | 178.4ns | 2.8% |  |
| br_mask_c_pred05 | 3.3ns | 182.2ns | 1.8% |  |
| br_predicate_c_pred05 | 4.1ns | 183.4ns | 2.2% |  |
| br_profiled_hot_c_pred05 | 4.8ns | 121.7ns | 3.9% |  |

## Distribution (algo ns)

```
br_branch_c_pred05 (n=6, range 113.3-136.3 ns)
    113.3 |####################
    114.5 |
    115.6 |
    116.8 |
    117.9 |
    119.0 |
    120.2 |
    121.4 |
    122.5 |
    123.7 |########################################
    124.8 |
    126.0 |
    127.1 |
    128.2 |
    129.4 |
    130.6 |
    131.7 |####################
    132.9 |####################
    134.0 |
    135.2 |
  (0 below, 1 above range)

br_lut_c_pred05 (n=6, range 166.7-187.9 ns)
    166.7 |########################################
    167.8 |
    168.8 |
    169.9 |
    170.9 |
    172.0 |
    173.1 |
    174.1 |########################################
    175.2 |########################################
    176.3 |########################################
    177.3 |
    178.4 |
    179.4 |
    180.5 |
    181.6 |
    182.6 |
    183.7 |
    184.8 |
    185.8 |########################################
    186.9 |
  (0 below, 1 above range)

br_mask_c_pred05 (n=6, range 148.7-193.1 ns)
    148.7 |####################
    150.9 |
    153.1 |
    155.4 |
    157.6 |
    159.8 |
    162.0 |
    164.2 |
    166.5 |
    168.7 |
    170.9 |
    173.1 |
    175.3 |####################
    177.6 |
    179.8 |
    182.0 |
    184.2 |
    186.4 |
    188.7 |########################################
    190.9 |####################
  (0 below, 1 above range)

br_predicate_c_pred05 (n=6, range 175.0-189.8 ns)
    175.0 |########################################
    175.7 |########################################
    176.5 |
    177.2 |
    178.0 |
    178.7 |
    179.4 |
    180.2 |
    180.9 |
    181.7 |########################################
    182.4 |
    183.1 |
    183.9 |
    184.6 |
    185.4 |
    186.1 |
    186.8 |
    187.6 |########################################
    188.3 |
    189.1 |########################################
  (0 below, 1 above range)

br_profiled_hot_c_pred05 (n=6, range 103.8-131.4 ns)
    103.8 |####################
    105.2 |
    106.6 |
    107.9 |
    109.3 |
    110.7 |
    112.1 |
    113.5 |
    114.9 |
    116.2 |
    117.6 |
    119.0 |########################################
    120.4 |
    121.8 |
    123.2 |####################
    124.5 |
    125.9 |
    127.3 |
    128.7 |
    130.1 |####################
  (0 below, 1 above range)

```
