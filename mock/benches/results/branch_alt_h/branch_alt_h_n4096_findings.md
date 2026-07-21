# Branch strategies, heavy-arm, alt: strict alternation (i&1)

2 variants, 6 samples per variant.
Baseline: **br_branch_h_alt**

## Key findings

- **Baseline (br_branch_h_alt) is the fastest** at 35520.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.93x (fastest 35520.9 ns, slowest 68411.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_alt | 37869ns | 37951ns | 35430ns | 37392ns | 39806ns | base |
| br_predicate_h_alt | 75543ns | 70764ns | 67260ns | 70583ns | 87125ns | +99.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_alt | 35494ns | 33191ns | 37344ns | base | 0.115 |
| br_predicate_h_alt | 73068ns | 65047ns | 84303ns | +105.86% | 0.056 |

## Performance model

- Peak throughput: **0.123 Gops/s** (br_branch_h_alt; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_alt | 0.115 | 93.4% |
| br_predicate_h_alt | 0.060 | 48.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_alt | 37869ns | 37869ns | base |
| br_predicate_h_alt | 75543ns | 75543ns | +99.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_alt | 35521ns | base | --- | [33618, 37344] | --- | --- | --- | --- |
| br_predicate_h_alt | 68411ns | +33507.8ns (+94.3%) | [+32255, +46959]ns | [66490, 84303] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_alt | br_predicate_h_alt |
|---|---|---|
| 1 | 33191ns | +96.0% |
| 2 | 37314ns | +155.6% |
| 3 | 35143ns | +93.3% |
| 4 | 34044ns | +100.5% |
| 5 | 35899ns | +91.0% |
| 6 | 37374ns | +95.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_alt | -0.285 | moderate- |
| br_predicate_h_alt | -0.392 | moderate- |

**Consistency summary:**

- **br_predicate_h_alt**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_alt | 4.9ns | 35494.2ns | 0.0% |  |
| br_predicate_h_alt | 22.8ns | 73068.2ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_alt (n=6, range 33191.2-37344.0 ns)
  33191.2 |########################################
  33398.8 |
  33606.5 |
  33814.1 |
  34021.8 |########################################
  34229.4 |
  34437.0 |
  34644.7 |
  34852.3 |
  35060.0 |########################################
  35267.6 |
  35475.2 |
  35682.9 |
  35890.5 |########################################
  36098.2 |
  36305.8 |
  36513.4 |
  36721.1 |
  36928.7 |
  37136.4 |########################################
  (0 below, 1 above range)

br_predicate_h_alt (n=6, range 65047.1-84303.1 ns)
  65047.1 |####################
  66009.9 |
  66972.7 |####################
  67935.5 |########################################
  68898.3 |
  69861.1 |
  70823.9 |
  71786.7 |
  72749.5 |####################
  73712.3 |
  74675.1 |
  75637.9 |
  76600.7 |
  77563.5 |
  78526.3 |
  79489.1 |
  80451.9 |
  81414.7 |
  82377.5 |
  83340.3 |
  (0 below, 1 above range)

```
