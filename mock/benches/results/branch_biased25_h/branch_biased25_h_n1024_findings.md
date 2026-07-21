# Branch strategies, heavy-arm, biased25: ~25% taken (b<64)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_biased25**

## Key findings

- **Baseline (br_branch_h_biased25) is the fastest** at 8668.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.19x (fastest 8668.1 ns, slowest 18985.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 11194ns | 11236ns | 10710ns | 11068ns | 11624ns | base |
| br_predicate_h_biased25 | 21841ns | 21842ns | 20698ns | 21488ns | 22941ns | +95.11% |
| br_profiled_hot_h_biased25 | 12495ns | 12461ns | 10688ns | 12426ns | 13503ns | +11.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_biased25 | 8661ns | 8279ns | 9025ns | base | 0.118 |
| br_predicate_h_biased25 | 19064ns | 18279ns | 19896ns | +120.11% | 0.054 |
| br_profiled_hot_h_biased25 | 10012ns | 8550ns | 10849ns | +15.60% | 0.102 |

## Performance model

- Peak throughput: **0.124 Gops/s** (br_branch_h_biased25; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_biased25 | 0.118 | 95.5% |
| br_predicate_h_biased25 | 0.054 | 43.6% |
| br_profiled_hot_h_biased25 | 0.102 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_biased25 | 11194ns | 11194ns | base |
| br_predicate_h_biased25 | 21841ns | 21841ns | +95.11% |
| br_profiled_hot_h_biased25 | 12495ns | 12495ns | +11.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 8668ns | base | --- | [8291, 9025] | --- | --- | --- | --- |
| br_predicate_h_biased25 | 18986ns | +10553.3ns (+121.7%) | [+9332, +11324]ns | [18311, 19896] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| br_profiled_hot_h_biased25 | 9993ns | +1488.4ns (+17.2%) | [+286, +2279]ns | [9195, 10849] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_biased25 | br_predicate_h_biased25 | br_profiled_hot_h_biased25 |
|---|---|---|---|
| 1 | 8750ns | +108.9% | -2.3% |
| 2 | 8586ns | +120.2% | +14.6% |
| 3 | 8303ns | +135.4% | +34.1% |
| 4 | 8841ns | +129.0% | +19.5% |
| 5 | 8279ns | +130.3% | +20.9% |
| 6 | 9208ns | +99.2% | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_biased25 | -0.520 | HIGH- (thermal bounce) |
| br_predicate_h_biased25 | 0.221 | moderate+ |
| br_profiled_hot_h_biased25 | 0.181 | ok |

**Consistency summary:**

- **br_predicate_h_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_h_biased25**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_biased25 | 4.4ns | 8661.2ns | 0.1% |  |
| br_predicate_h_biased25 | 5.8ns | 19064.2ns | 0.0% |  |
| br_profiled_hot_h_biased25 | 3.8ns | 10012.2ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_biased25 (n=6, range 8278.8-9024.5 ns)
   8278.8 |########################################
   8316.1 |
   8353.4 |
   8390.7 |
   8427.9 |
   8465.2 |
   8502.5 |
   8539.8 |
   8577.1 |####################
   8614.4 |
   8651.7 |
   8689.0 |
   8726.2 |####################
   8763.5 |
   8800.8 |
   8838.1 |####################
   8875.4 |
   8912.7 |
   8950.0 |
   8987.3 |
  (0 below, 1 above range)

br_predicate_h_biased25 (n=6, range 18279.2-19896.0 ns)
  18279.2 |########################################
  18360.0 |
  18440.9 |
  18521.7 |
  18602.6 |
  18683.4 |
  18764.3 |
  18845.1 |####################
  18925.9 |
  19006.8 |####################
  19087.6 |
  19168.5 |
  19249.3 |
  19330.2 |
  19411.0 |
  19491.8 |####################
  19572.7 |
  19653.5 |
  19734.4 |
  19815.2 |
  (0 below, 1 above range)

br_profiled_hot_h_biased25 (n=6, range 8549.6-10849.2 ns)
   8549.6 |####################
   8664.6 |
   8779.6 |
   8894.5 |
   9009.5 |
   9124.5 |
   9239.5 |
   9354.5 |
   9469.4 |
   9584.4 |
   9699.4 |
   9814.4 |####################
   9929.4 |########################################
  10044.3 |
  10159.3 |
  10274.3 |
  10389.3 |
  10504.3 |####################
  10619.2 |
  10734.2 |
  (0 below, 1 above range)

```
