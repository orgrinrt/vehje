# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 18% faster than the next best (iterfuse_push2)

iterfuse_pull2 (752.46 us) leads iterfuse_push2 (884.28 us) by 18%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (752.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 752455.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.26x (fastest 752455.2 ns, slowest 948605.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 951875ns | 950929ns | 946286ns | 949450ns | 958306ns | +25.28% |
| iterfuse_pull2 | 759809ns | 755457ns | 721610ns | 748817ns | 795398ns | base |
| iterfuse_push2 | 885938ns | 887561ns | 872742ns | 883844ns | 895677ns | +16.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 949551ns | 943939ns | 956027ns | +25.45% | 0.017 |
| iterfuse_pull2 | 756928ns | 719108ns | 792185ns | base | 0.022 |
| iterfuse_push2 | 882803ns | 869418ns | 892870ns | +16.63% | 0.019 |

## Performance model

- Peak throughput: **0.023 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.017 | 75.8% |
| iterfuse_pull2 | 0.022 | 95.6% |
| iterfuse_push2 | 0.019 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 951875ns | 951875ns | +25.28% |
| iterfuse_pull2 | 759809ns | 759809ns | base |
| iterfuse_push2 | 885938ns | 885938ns | +16.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 752455ns | base | --- | [726143, 792185] | --- | --- | --- | --- |
| iterfuse_mat2 | 948605ns | +198337.8ns (+26.4%) | [+159448, +220085]ns | [944022, 956027] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 884276ns | +131820.7ns (+17.5%) | [+90859, +154945]ns | [871263, 892870] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 777749ns | +23.3% | +12.3% |
| 2 | 806620ns | +17.0% | +10.7% |
| 3 | 733179ns | +28.7% | +21.8% |
| 4 | 719108ns | +31.9% | +20.9% |
| 5 | 763009ns | +24.9% | +15.9% |
| 6 | 741902ns | +27.9% | +19.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.134 | ok |
| iterfuse_pull2 | 0.084 | ok |
| iterfuse_push2 | -0.310 | moderate- |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 4126.1ns | 949551.2ns | 0.4% |  |
| iterfuse_pull2 | 9.9ns | 756927.7ns | 0.0% |  |
| iterfuse_push2 | 17.7ns | 882802.8ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 943938.7-956026.9 ns)
  943938.7 |########################################
  944543.1 |
  945147.5 |
  945751.9 |
  946356.3 |
  946960.8 |
  947565.2 |
  948169.6 |########################################
  948774.0 |
  949378.4 |
  949982.8 |
  950587.2 |
  951191.6 |
  951796.0 |
  952400.4 |####################
  953004.8 |
  953609.3 |
  954213.7 |
  954818.1 |
  955422.5 |
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 719107.5-792184.8 ns)
  719107.5 |########################################
  722761.4 |
  726415.2 |
  730069.1 |########################################
  733723.0 |
  737376.8 |
  741030.7 |########################################
  744684.6 |
  748338.4 |
  751992.3 |
  755646.2 |
  759300.0 |
  762953.9 |########################################
  766607.7 |
  770261.6 |
  773915.5 |
  777569.3 |########################################
  781223.2 |
  784877.1 |
  788530.9 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 869418.3-892869.8 ns)
  869418.3 |####################
  870590.9 |
  871763.4 |
  872936.0 |####################
  874108.6 |
  875281.2 |
  876453.7 |
  877626.3 |
  878798.9 |
  879971.5 |
  881144.0 |
  882316.6 |
  883489.2 |########################################
  884661.7 |
  885834.3 |
  887006.9 |
  888179.5 |
  889352.0 |
  890524.6 |
  891697.2 |####################
  (0 below, 1 above range)

```
