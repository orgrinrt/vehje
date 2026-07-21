# Match lowering: if-chain vs jump-table vs decision-tree, K=64 arms, uniform hits

3 variants, 6 samples per variant.
Baseline: **ml_jumptable_u64**

## Highlights

Baseline for all deltas below: **ml_jumptable_u64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ml_tree_u64 is an outlier: 7.7x slower than the field

ml_tree_u64 (7.75 ms) is 7.7x the fastest (1.01 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (ml_jumptable_u64, ml_ifchain_u64) are a dead heat (<1%)

ml_jumptable_u64 (1.01 ms) and ml_ifchain_u64 (1.01 ms) differ by 0.13%, inside the noise, even though the wider field spreads 666.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (ml_jumptable_u64)

The baseline ml_jumptable_u64 is the fastest (1.01 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.7x the fastest

Fastest ml_jumptable_u64 (1.01 ms) to slowest ml_tree_u64 (7.75 ms): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ml_jumptable_u64) is the fastest** at 1011315.4 ns median
- 1 variant significantly slower than baseline
- Spread: 7.67x (fastest 1011315.4 ns, slowest 7753257.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ml_ifchain_u64 | 1018273ns | 1015920ns | 1000848ns | 1013099ns | 1034747ns | +0.14% |
| ml_jumptable_u64 | 1016807ns | 1014669ns | 1008394ns | 1012947ns | 1026802ns | base |
| ml_tree_u64 | 7761508ns | 7757287ns | 7738145ns | 7755319ns | 7782472ns | +663.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ml_ifchain_u64 | 1015157ns | 997334ns | 1032194ns | +0.14% | 0.016 |
| ml_jumptable_u64 | 1013756ns | 1005126ns | 1024403ns | base | 0.016 |
| ml_tree_u64 | 7757422ns | 7733743ns | 7778559ns | +665.22% | 0.002 |

## Performance model

- Peak throughput: **0.016 Gops/s** (ml_ifchain_u64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ml_ifchain_u64 | 0.016 | 98.5% |
| ml_jumptable_u64 | 0.016 | 98.6% |
| ml_tree_u64 | 0.002 | 12.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ml_ifchain_u64 | 1018273ns | 1018273ns | +0.14% |
| ml_jumptable_u64 | 1016807ns | 1016807ns | base |
| ml_tree_u64 | 7761508ns | 7761508ns | +663.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ml_jumptable_u64 | 1011315ns | base | --- | [1005551, 1024403] | --- | --- | --- | --- |
| ml_ifchain_u64 | 1012654ns | no significant difference | [-12659, +18060]ns | [1000624, 1032194] | no | 1.0000 | 1.0000 | 0 |
| ml_tree_u64 | 7753257ns | +6741941.7ns (+666.7%) | [+6726315, +6762740]ns | [7740450, 7778559] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ml_jumptable_u64 | ml_ifchain_u64 | ml_tree_u64 |
|---|---|---|---|
| 1 | 1011825ns | -1.4% | +666.6% |
| 2 | 1026512ns | -1.1% | +659.1% |
| 3 | 1010806ns | -0.7% | +666.7% |
| 4 | 1022294ns | +0.6% | +657.8% |
| 5 | 1005126ns | +0.4% | +672.5% |
| 6 | 1005975ns | +3.0% | +668.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ml_ifchain_u64 | -0.324 | moderate- |
| ml_jumptable_u64 | -0.245 | moderate- |
| ml_tree_u64 | -0.224 | moderate- |

**Consistency summary:**

- **ml_ifchain_u64**: won 3/6, lost 3/6
- **ml_tree_u64**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ml_ifchain_u64 | 15.1ns | 1015157.4ns | 0.0% |  |
| ml_jumptable_u64 | 19.9ns | 1013756.3ns | 0.0% |  |
| ml_tree_u64 | 162.3ns | 7757422.1ns | 0.0% |  |

## Distribution (algo ns)

```
ml_ifchain_u64 (n=6, range 997334.2-1032194.3 ns)
  997334.2 |########################################
  999077.2 |
  1000820.2 |
  1002563.2 |########################################
  1004306.2 |
  1006049.2 |
  1007792.2 |
  1009535.3 |########################################
  1011278.3 |
  1013021.3 |
  1014764.3 |########################################
  1016507.3 |
  1018250.3 |
  1019993.3 |
  1021736.3 |
  1023479.3 |
  1025222.3 |
  1026965.3 |########################################
  1028708.3 |
  1030451.3 |
  (0 below, 1 above range)

ml_jumptable_u64 (n=6, range 1005126.2-1024402.9 ns)
  1005126.2 |########################################
  1006090.0 |
  1007053.9 |
  1008017.7 |
  1008981.5 |
  1009945.4 |####################
  1010909.2 |####################
  1011873.1 |
  1012836.9 |
  1013800.7 |
  1014764.6 |
  1015728.4 |
  1016692.2 |
  1017656.1 |
  1018619.9 |
  1019583.8 |
  1020547.6 |
  1021511.4 |####################
  1022475.3 |
  1023439.1 |
  (0 below, 1 above range)

ml_tree_u64 (n=6, range 7733742.9-7778559.2 ns)
  7733742.9 |########################################
  7735983.7 |
  7738224.5 |
  7740465.3 |
  7742706.2 |
  7744947.0 |########################################
  7747187.8 |
  7749428.6 |########################################
  7751669.4 |
  7753910.2 |
  7756151.0 |########################################
  7758391.8 |
  7760632.7 |
  7762873.5 |########################################
  7765114.3 |
  7767355.1 |
  7769595.9 |
  7771836.7 |
  7774077.5 |
  7776318.3 |
  (0 below, 1 above range)

```
