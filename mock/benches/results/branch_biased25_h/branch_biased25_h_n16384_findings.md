# Branch strategies, heavy-arm, biased25: ~25% taken (b<64)

3 variants, 6 samples per variant.
Baseline: **br_branch_h_biased25**

## Key findings

- **Baseline (br_branch_h_biased25) is the fastest** at 126063.6 ns median
- 2 variants significantly slower than baseline
- Spread: 2.09x (fastest 126063.6 ns, slowest 262892.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 128358ns | 128473ns | 126639ns | 128168ns | 129502ns | base |
| br_predicate_h_biased25 | 266142ns | 265301ns | 263050ns | 265243ns | 269036ns | +107.34% |
| br_profiled_hot_h_biased25 | 151507ns | 149891ns | 149006ns | 149734ns | 155416ns | +18.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_h_biased25 | 126002ns | 124203ns | 127165ns | base | 0.130 |
| br_predicate_h_biased25 | 263722ns | 260735ns | 266658ns | +109.30% | 0.062 |
| br_profiled_hot_h_biased25 | 149138ns | 146711ns | 153087ns | +18.36% | 0.110 |

## Performance model

- Peak throughput: **0.132 Gops/s** (br_branch_h_biased25; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_h_biased25 | 0.130 | 98.5% |
| br_predicate_h_biased25 | 0.062 | 47.2% |
| br_profiled_hot_h_biased25 | 0.111 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_h_biased25 | 128358ns | 128358ns | base |
| br_predicate_h_biased25 | 266142ns | 266142ns | +107.34% |
| br_profiled_hot_h_biased25 | 151507ns | 151507ns | +18.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_h_biased25 | 126064ns | base | --- | [124778, 127165] | --- | --- | --- | --- |
| br_predicate_h_biased25 | 262892ns | +137273.2ns (+108.9%) | [+136125, +139762]ns | [261616, 266658] | YES | 0.0313 | 0.0313 | 0 |
| br_profiled_hot_h_biased25 | 147461ns | +21902.3ns (+17.4%) | [+20077, +27428]ns | [146866, 153087] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_h_biased25 | br_predicate_h_biased25 | br_profiled_hot_h_biased25 |
|---|---|---|---|
| 1 | 126538ns | +111.1% | +17.3% |
| 2 | 125590ns | +109.4% | +17.4% |
| 3 | 124203ns | +109.9% | +18.1% |
| 4 | 125354ns | +109.4% | +25.8% |
| 5 | 127074ns | +106.8% | +16.0% |
| 6 | 127256ns | +109.2% | +15.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_h_biased25 | 0.342 | moderate+ |
| br_predicate_h_biased25 | 0.074 | ok |
| br_profiled_hot_h_biased25 | -0.294 | moderate- |

**Consistency summary:**

- **br_predicate_h_biased25**: won 0/6, lost 6/6
- **br_profiled_hot_h_biased25**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_h_biased25 | 4.5ns | 126002.3ns | 0.0% |  |
| br_predicate_h_biased25 | 6.7ns | 263722.3ns | 0.0% |  |
| br_profiled_hot_h_biased25 | 5.1ns | 149138.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_h_biased25 (n=6, range 124203.3-127165.0 ns)
  124203.3 |########################################
  124351.4 |
  124499.5 |
  124647.6 |
  124795.6 |
  124943.7 |
  125091.8 |
  125239.9 |########################################
  125388.0 |
  125536.1 |########################################
  125684.1 |
  125832.2 |
  125980.3 |
  126128.4 |
  126276.5 |
  126424.6 |########################################
  126572.7 |
  126720.7 |
  126868.8 |
  127016.9 |########################################
  (0 below, 1 above range)

br_predicate_h_biased25 (n=6, range 260735.4-266658.5 ns)
  260735.4 |########################################
  261031.6 |
  261327.7 |
  261623.9 |
  261920.0 |
  262216.2 |########################################
  262512.3 |########################################
  262808.5 |########################################
  263104.6 |
  263400.8 |
  263697.0 |
  263993.1 |
  264289.3 |
  264585.4 |
  264881.6 |
  265177.7 |
  265473.9 |
  265770.0 |
  266066.2 |########################################
  266362.3 |
  (0 below, 1 above range)

br_profiled_hot_h_biased25 (n=6, range 146710.8-153087.1 ns)
  146710.8 |########################################
  147029.6 |
  147348.4 |########################################
  147667.2 |
  147986.1 |
  148304.9 |####################
  148623.7 |
  148942.5 |
  149261.3 |
  149580.1 |
  149899.0 |
  150217.8 |
  150536.6 |
  150855.4 |
  151174.2 |
  151493.0 |
  151811.8 |
  152130.7 |
  152449.5 |
  152768.3 |
  (0 below, 1 above range)

```
