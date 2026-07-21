# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (771 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 771.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.29x (fastest 771.0 ns, slowest 992.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 3370ns | 3304ns | 3105ns | 3247ns | 3687ns | +4.91% |
| iterfuse_pull2 | 3212ns | 3266ns | 2896ns | 3162ns | 3447ns | base |
| iterfuse_push2 | 3258ns | 3345ns | 2861ns | 3190ns | 3560ns | +1.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 1020ns | 949ns | 1117ns | +29.59% | 0.063 |
| iterfuse_pull2 | 787ns | 727ns | 850ns | base | 0.081 |
| iterfuse_push2 | 808ns | 688ns | 880ns | +2.69% | 0.079 |

## Performance model

- Peak throughput: **0.093 Gops/s** (iterfuse_push2; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.064 | 69.3% |
| iterfuse_pull2 | 0.083 | 89.2% |
| iterfuse_push2 | 0.077 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 3370ns | 3370ns | +4.91% |
| iterfuse_pull2 | 3212ns | 3212ns | base |
| iterfuse_push2 | 3258ns | 3258ns | +1.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 771ns | base | --- | [739, 850] | --- | --- | --- | --- |
| iterfuse_mat2 | 992ns | +212.0ns (+27.5%) | [+184, +303]ns | [950, 1117] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| iterfuse_push2 | 834ns | no significant difference | [-50, +83]ns | [709, 880] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 727ns | +30.7% | +13.4% |
| 2 | 751ns | +26.3% | -2.8% |
| 3 | 767ns | +26.2% | -10.3% |
| 4 | 775ns | +40.1% | +8.9% |
| 5 | 853ns | +34.6% | +6.6% |
| 6 | 848ns | +20.0% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | 0.399 | moderate+ |
| iterfuse_pull2 | 0.469 | moderate+ |
| iterfuse_push2 | 0.344 | moderate+ |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 85.2ns | 1019.7ns | 8.4% | HIGH |
| iterfuse_pull2 | 4.3ns | 786.9ns | 0.5% |  |
| iterfuse_push2 | 3.7ns | 808.0ns | 0.5% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 948.7-1117.0 ns)
    948.7 |########################################
    957.1 |
    965.5 |####################
    974.0 |
    982.4 |
    990.8 |
    999.2 |
   1007.6 |
   1016.0 |####################
   1024.5 |
   1032.9 |
   1041.3 |
   1049.7 |
   1058.1 |
   1066.5 |
   1075.0 |
   1083.4 |####################
   1091.8 |
   1100.2 |
   1108.6 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 727.1-850.4 ns)
    727.1 |########################################
    733.3 |
    739.4 |
    745.6 |########################################
    751.8 |
    757.9 |
    764.1 |########################################
    770.3 |########################################
    776.4 |
    782.6 |
    788.8 |
    794.9 |
    801.1 |
    807.2 |
    813.4 |
    819.6 |
    825.7 |
    831.9 |
    838.1 |
    844.2 |########################################
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 687.9-880.5 ns)
    687.9 |########################################
    697.5 |
    707.2 |
    716.8 |
    726.4 |########################################
    736.0 |
    745.7 |
    755.3 |
    764.9 |
    774.5 |
    784.2 |
    793.8 |
    803.4 |
    813.1 |
    822.7 |########################################
    832.3 |
    841.9 |########################################
    851.6 |########################################
    861.2 |
    870.8 |
  (0 below, 1 above range)

```
