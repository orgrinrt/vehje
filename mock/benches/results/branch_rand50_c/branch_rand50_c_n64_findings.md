# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Highlights

Baseline for all deltas below: **br_branch_c_rand50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### br_branch_c_rand50 dominates: 35% faster than the next best (br_predicate_c_rand50)

br_branch_c_rand50 (136 ns) leads br_predicate_c_rand50 (184 ns) by 35%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### br_lut_c_rand50 shows warm-up / thermal drift (autocorr +0.51)

br_lut_c_rand50's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (br_branch_c_rand50)

The baseline br_branch_c_rand50 is the fastest (136 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {br_branch_c_rand50} vs {br_predicate_c_rand50, br_lut_c_rand50, br_mask_c_rand50} (35% apart)

The field splits into a fast tier {br_branch_c_rand50} and a slow tier {br_predicate_c_rand50, br_lut_c_rand50, br_mask_c_rand50} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (br_branch_c_rand50) is the fastest** at 136.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.57x (fastest 136.2 ns, slowest 213.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 2890ns | 2920ns | 2686ns | 2844ns | 3062ns | base |
| br_lut_c_rand50 | 2905ns | 2863ns | 2734ns | 2821ns | 3118ns | +0.51% |
| br_mask_c_rand50 | 2995ns | 2993ns | 2768ns | 2971ns | 3145ns | +3.62% |
| br_predicate_c_rand50 | 2833ns | 2877ns | 2259ns | 2829ns | 3125ns | -2.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 136ns | 127ns | 144ns | base | 0.472 |
| br_lut_c_rand50 | 190ns | 178ns | 204ns | +40.01% | 0.337 |
| br_mask_c_rand50 | 212ns | 198ns | 225ns | +56.45% | 0.301 |
| br_predicate_c_rand50 | 182ns | 146ns | 200ns | +33.92% | 0.352 |

## Performance model

- Peak throughput: **0.504 Gops/s** (br_branch_c_rand50; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.470 | 93.3% |
| br_lut_c_rand50 | 0.341 | 67.7% |
| br_mask_c_rand50 | 0.299 | 59.5% |
| br_predicate_c_rand50 | 0.349 | 69.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 2890ns | 2890ns | base |
| br_lut_c_rand50 | 2905ns | 2905ns | +0.51% |
| br_mask_c_rand50 | 2995ns | 2995ns | +3.62% |
| br_predicate_c_rand50 | 2833ns | 2833ns | -2.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 136ns | base | --- | [127, 144] | --- | --- | --- | --- |
| br_lut_c_rand50 | 188ns | +55.2ns (+40.5%) | [+40, +67]ns | [179, 204] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 214ns | +76.5ns (+56.1%) | [+59, +94]ns | [198, 225] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 184ns | +51.2ns (+37.6%) | [+23, +64]ns | [161, 200] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 128ns | +42.8% | +55.2% | +14.4% |
| 2 | 127ns | +40.3% | +70.5% | +39.0% |
| 3 | 150ns | +19.8% | +32.0% | +18.6% |
| 4 | 138ns | +40.6% | +54.8% | +38.5% |
| 5 | 135ns | +54.8% | +73.0% | +40.0% |
| 6 | 137ns | +44.1% | +56.5% | +53.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | -0.073 | ok |
| br_lut_c_rand50 | 0.515 | HIGH+ (drift/warm-up) |
| br_mask_c_rand50 | -0.079 | ok |
| br_predicate_c_rand50 | 0.200 | moderate+ |

**Consistency summary:**

- **br_lut_c_rand50**: won 0/6, lost 6/6
- **br_mask_c_rand50**: won 0/6, lost 6/6
- **br_predicate_c_rand50**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 5.6ns | 135.7ns | 4.2% |  |
| br_lut_c_rand50 | 3.9ns | 190.0ns | 2.1% |  |
| br_mask_c_rand50 | 3.9ns | 212.3ns | 1.9% |  |
| br_predicate_c_rand50 | 3.7ns | 181.7ns | 2.0% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 127.1-143.6 ns)
    127.1 |########################################
    127.9 |
    128.7 |
    129.6 |
    130.4 |
    131.2 |
    132.0 |
    132.9 |
    133.7 |
    134.5 |
    135.3 |####################
    136.1 |
    137.0 |########################################
    137.8 |
    138.6 |
    139.4 |
    140.3 |
    141.1 |
    141.9 |
    142.7 |
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 178.3-203.6 ns)
    178.3 |########################################
    179.6 |
    180.8 |
    182.1 |####################
    183.4 |
    184.6 |
    185.9 |
    187.1 |
    188.4 |
    189.7 |
    190.9 |
    192.2 |####################
    193.5 |
    194.7 |
    196.0 |
    197.2 |####################
    198.5 |
    199.8 |
    201.0 |
    202.3 |
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 197.5-225.4 ns)
    197.5 |########################################
    198.9 |
    200.3 |
    201.7 |
    203.1 |
    204.5 |
    205.9 |
    207.3 |
    208.7 |
    210.1 |
    211.5 |
    212.9 |####################
    214.3 |####################
    215.7 |####################
    217.1 |
    218.5 |
    219.9 |
    221.3 |
    222.7 |
    224.1 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 145.8-200.4 ns)
    145.8 |####################
    148.5 |
    151.3 |
    154.0 |
    156.7 |
    159.5 |
    162.2 |
    164.9 |
    167.6 |
    170.4 |
    173.1 |
    175.8 |########################################
    178.6 |
    181.3 |
    184.0 |
    186.8 |
    189.5 |########################################
    192.2 |
    194.9 |
    197.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_lut_c_rand50**: autocorrelation=0.51 (measurement drift or warm-up artifact)
