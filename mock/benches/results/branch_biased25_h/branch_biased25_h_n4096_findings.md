# Branch strategies, heavy-arm, biased25: ~25% taken (b<64)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_biased25**

## Key findings

- **Baseline (br_branch_h_biased25) is the fastest** at 32859.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.22x (fastest 32859.4 ns, slowest 73064.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 34971ns | 35228ns | 32121ns | 34691ns | 36817ns | base |
| br_predicate_h_biased25 | 74592ns | 75634ns | 68954ns | 75328ns | 76308ns | +113.30% |
| br_profiled_hot_h_biased25 | 42394ns | 42689ns | 38063ns | 41796ns | 45456ns | +21.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_biased25 | 32611ns | 29935ns | 34336ns | base | 0.126 |
| br_predicate_h_biased25 | 72102ns | 66729ns | 73770ns | +121.10% | 0.057 |
| br_profiled_hot_h_biased25 | 39964ns | 35893ns | 42830ns | +22.55% | 0.102 |

## Performance model

- Peak throughput: **0.137 Gops/s** (br_branch_h_biased25; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_biased25 | 0.125 | 91.1% |
| br_predicate_h_biased25 | 0.056 | 41.0% |
| br_profiled_hot_h_biased25 | 0.102 | 74.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_biased25 | 34971ns | 34971ns | base |
| br_predicate_h_biased25 | 74592ns | 74592ns | +113.30% |
| br_profiled_hot_h_biased25 | 42394ns | 42394ns | +21.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 32859ns | base | --- | [30636, 34336] | --- | --- | --- | --- |
| br_predicate_h_biased25 | 73064ns | +39725.6ns (+120.9%) | [+37191, +41557]ns | [69471, 73770] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_biased25 | 40253ns | +7305.2ns (+22.2%) | [+6004, +8752]ns | [36809, 42830] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_biased25 | br_predicate_h_biased25 | br_profiled_hot_h_biased25 |
|---|---|---|---|
| 1 | 29935ns | +122.9% | +19.9% |
| 2 | 34042ns | +116.2% | +18.6% |
| 3 | 34626ns | +108.6% | +25.2% |
| 4 | 34046ns | +117.2% | +24.3% |
| 5 | 31677ns | +130.1% | +19.1% |
| 6 | 31338ns | +133.7% | +28.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_biased25 | 0.101 | ok |
| br_predicate_h_biased25 | -0.147 | ok |
| br_profiled_hot_h_biased25 | 0.052 | ok |

**Consistency summary:**

- **br_predicate_h_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_h_biased25**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_biased25 | 4.9ns | 32610.6ns | 0.0% |  |
| br_predicate_h_biased25 | 4.9ns | 72101.7ns | 0.0% |  |
| br_profiled_hot_h_biased25 | 3.7ns | 39964.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_biased25 (n=6, range 29934.6-34336.0 ns)
  29934.6 |####################
  30154.7 |
  30374.7 |
  30594.8 |
  30814.9 |
  31034.9 |
  31255.0 |####################
  31475.1 |####################
  31695.2 |
  31915.2 |
  32135.3 |
  32355.4 |
  32575.4 |
  32795.5 |
  33015.6 |
  33235.7 |
  33455.7 |
  33675.8 |
  33895.9 |########################################
  34115.9 |
  (0 below, 1 above range)

br_predicate_h_biased25 (n=6, range 66729.2-73769.8 ns)
  66729.2 |########################################
  67081.2 |
  67433.3 |
  67785.3 |
  68137.3 |
  68489.3 |
  68841.4 |
  69193.4 |
  69545.4 |
  69897.5 |
  70249.5 |
  70601.5 |
  70953.6 |
  71305.6 |
  71657.6 |
  72009.6 |########################################
  72361.7 |
  72713.7 |########################################
  73065.7 |########################################
  73417.8 |########################################
  (0 below, 1 above range)

br_profiled_hot_h_biased25 (n=6, range 35893.3-42830.4 ns)
  35893.3 |####################
  36240.2 |
  36587.0 |
  36933.9 |
  37280.7 |
  37627.6 |####################
  37974.4 |
  38321.3 |
  38668.1 |
  39015.0 |
  39361.9 |
  39708.7 |
  40055.6 |########################################
  40402.4 |
  40749.3 |
  41096.1 |
  41443.0 |
  41789.8 |
  42136.7 |####################
  42483.5 |
  (0 below, 1 above range)

```
