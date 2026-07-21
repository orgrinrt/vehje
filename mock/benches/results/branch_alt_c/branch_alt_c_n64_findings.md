# Branch strategies, cheap-arm, alt: strict alternation (i&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_alt**

## Key findings

- **Baseline (br_branch_c_alt) is the fastest** at 101.0 ns median
- 3 variants significantly slower than baseline
- Spread: 1.66x (fastest 101.0 ns, slowest 167.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_alt | 2443ns | 2348ns | 2258ns | 2318ns | 2723ns | base |
| br_lut_c_alt | 2527ns | 2585ns | 2314ns | 2495ns | 2680ns | +3.42% |
| br_mask_c_alt | 2488ns | 2417ns | 2396ns | 2414ns | 2646ns | +1.84% |
| br_predicate_c_alt | 2859ns | 2490ns | 2312ns | 2456ns | 3736ns | +17.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_alt | 105ns | 99ns | 114ns | base | 0.610 |
| br_lut_c_alt | 161ns | 147ns | 172ns | +53.34% | 0.398 |
| br_mask_c_alt | 171ns | 163ns | 181ns | +63.39% | 0.373 |
| br_predicate_c_alt | 187ns | 147ns | 251ns | +78.14% | 0.342 |

## Performance model

- Peak throughput: **0.645 Gops/s** (br_branch_c_alt; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_alt | 0.634 | 98.2% |
| br_lut_c_alt | 0.393 | 60.9% |
| br_mask_c_alt | 0.381 | 59.1% |
| br_predicate_c_alt | 0.400 | 61.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_alt | 2443ns | 2443ns | base |
| br_lut_c_alt | 2527ns | 2527ns | +3.42% |
| br_mask_c_alt | 2488ns | 2488ns | +1.84% |
| br_predicate_c_alt | 2859ns | 2859ns | +17.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_alt | 101ns | base | --- | [100, 114] | --- | --- | --- | --- |
| br_lut_c_alt | 163ns | +55.8ns (+55.2%) | [+47, +65]ns | [148, 172] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_alt | 168ns | +68.3ns (+67.6%) | [+55, +76]ns | [165, 181] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_alt | 160ns | +52.5ns (+52.0%) | [+42, +151]ns | [149, 251] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_alt | br_lut_c_alt | br_mask_c_alt | br_predicate_c_alt |
|---|---|---|---|---|
| 1 | 101ns | +61.6% | +80.7% | +233.1% |
| 2 | 100ns | +67.1% | +67.9% | +67.1% |
| 3 | 110ns | +48.6% | +64.2% | +50.2% |
| 4 | 99ns | +47.9% | +69.3% | +48.3% |
| 5 | 101ns | +46.9% | +65.5% | +49.4% |
| 6 | 119ns | +49.1% | +37.5% | +31.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_alt | -0.198 | ok |
| br_lut_c_alt | -0.042 | ok |
| br_mask_c_alt | -0.178 | ok |
| br_predicate_c_alt | 0.034 | ok |

**Consistency summary:**

- **br_lut_c_alt**: won 0/6, lost 6/6
- **br_mask_c_alt**: won 0/6, lost 6/6
- **br_predicate_c_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_alt | 4.2ns | 104.9ns | 4.0% |  |
| br_lut_c_alt | 3.5ns | 160.9ns | 2.2% |  |
| br_mask_c_alt | 3.5ns | 171.5ns | 2.1% |  |
| br_predicate_c_alt | 3.7ns | 186.9ns | 2.0% |  |

## Distribution (algo ns)

```
br_branch_c_alt (n=6, range 99.2-114.2 ns)
     99.2 |####################
    100.0 |####################
    100.7 |########################################
    101.5 |
    102.2 |
    103.0 |
    103.7 |
    104.5 |
    105.2 |
    106.0 |
    106.7 |
    107.4 |
    108.2 |
    108.9 |####################
    109.7 |
    110.4 |
    111.2 |
    111.9 |
    112.7 |
    113.4 |
  (0 below, 1 above range)

br_lut_c_alt (n=6, range 146.7-172.1 ns)
    146.7 |####################
    148.0 |####################
    149.2 |
    150.5 |
    151.8 |
    153.0 |
    154.3 |
    155.6 |
    156.9 |
    158.1 |
    159.4 |
    160.7 |
    161.9 |########################################
    163.2 |
    164.5 |
    165.8 |
    167.0 |####################
    168.3 |
    169.6 |
    170.8 |
  (0 below, 1 above range)

br_mask_c_alt (n=6, range 163.3-181.1 ns)
    163.3 |####################
    164.2 |
    165.1 |
    166.0 |
    166.9 |####################
    167.7 |########################################
    168.6 |
    169.5 |
    170.4 |
    171.3 |
    172.2 |
    173.1 |
    174.0 |
    174.8 |
    175.7 |
    176.6 |
    177.5 |
    178.4 |
    179.3 |####################
    180.2 |
  (0 below, 1 above range)

br_predicate_c_alt (n=6, range 147.1-251.4 ns)
    147.1 |########################################
    152.3 |####################
    157.5 |
    162.8 |########################################
    168.0 |
    173.2 |
    178.4 |
    183.6 |
    188.8 |
    194.1 |
    199.3 |
    204.5 |
    209.7 |
    214.9 |
    220.1 |
    225.4 |
    230.6 |
    235.8 |
    241.0 |
    246.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_predicate_c_alt**: CV=35.8% (high variance, measurements may be unstable)
