# Per-branch strategy (NATIVE tier): archetype 3

5 variants, 6 samples per variant.
Baseline: **an_b3_table**

## Highlights

Baseline for all deltas below: **an_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b3_seq shows alternating (throttle bounce) (autocorr -0.55)

an_b3_seq's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {an_b3_tree, an_b3_prof, an_b3_table, an_b3_seq} vs {an_b3_pred} (58% apart)

The field splits into a fast tier {an_b3_tree, an_b3_prof, an_b3_table, an_b3_seq} and a slow tier {an_b3_pred} with a 58% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b3_tree vs stability leader an_b3_prof (+3% speed for 1.3x steadier)

an_b3_tree is fastest (548 ns, CV 6.8%); an_b3_prof gives up 2.8% median for 1.3x lower variance (CV 5.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b3_prof's edge over baseline is significant but tiny (-5 ns, 0.85%)

an_b3_prof differs from baseline an_b3_table by -5 ns (0.85%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_b3_tree** at 547.7 ns median (-2.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.66x (fastest 547.7 ns, slowest 908.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b3_pred | 3399ns | 3486ns | 2997ns | 3338ns | 3693ns | +4.21% |
| an_b3_prof | 3029ns | 3129ns | 2691ns | 3062ns | 3148ns | -7.15% |
| an_b3_seq | 3272ns | 3160ns | 2641ns | 3155ns | 3763ns | +0.31% |
| an_b3_table | 3262ns | 3145ns | 2730ns | 3077ns | 3804ns | base |
| an_b3_tree | 2955ns | 3034ns | 2636ns | 2929ns | 3155ns | -9.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b3_pred | 894ns | 795ns | 973ns | +51.08% | 0.072 |
| an_b3_prof | 550ns | 488ns | 574ns | -6.97% | 0.116 |
| an_b3_seq | 600ns | 485ns | 704ns | +1.49% | 0.107 |
| an_b3_table | 592ns | 497ns | 698ns | base | 0.108 |
| an_b3_tree | 535ns | 482ns | 572ns | -9.52% | 0.120 |

## Performance model

- Peak throughput: **0.133 Gops/s** (an_b3_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b3_pred | 0.070 | 53.1% |
| an_b3_prof | 0.114 | 85.6% |
| an_b3_seq | 0.111 | 83.9% |
| an_b3_table | 0.113 | 85.5% |
| an_b3_tree | 0.117 | 88.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b3_pred | 3399ns | 3399ns | +4.21% |
| an_b3_prof | 3029ns | 3029ns | -7.15% |
| an_b3_seq | 3272ns | 3272ns | +0.31% |
| an_b3_table | 3262ns | 3262ns | base |
| an_b3_tree | 2955ns | 2955ns | -9.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b3_table | 564ns | base | --- | [513, 698] | --- | --- | --- | --- |
| an_b3_pred | 909ns | +360.2ns (+63.9%) | [+166, +380]ns | [800, 973] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b3_prof | 563ns | no significant difference | [-138, +19]ns | [514, 574] | no | 0.9167 | 0.6875 | 0 |
| an_b3_seq | 575ns | no significant difference | [-94, +126]ns | [522, 704] | no | 1.0000 | 1.0000 | 0 |
| an_b3_tree | 548ns | -27.7ns (-4.9%) | [-141, -0]ns | [486, 572] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b3_table | an_b3_pred | an_b3_prof | an_b3_seq | an_b3_tree |
|---|---|---|---|---|---|
| 1 | 770ns | +3.2% | -29.9% | -18.3% | -30.4% |
| 2 | 497ns | +61.9% | -1.9% | -2.5% | -3.0% |
| 3 | 530ns | +71.8% | +7.1% | +47.2% | -7.6% |
| 4 | 570ns | +66.8% | -0.1% | +0.4% | -0.3% |
| 5 | 558ns | +62.5% | +0.2% | +0.4% | +0.2% |
| 6 | 625ns | +59.4% | -7.4% | -7.6% | -7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b3_pred | 0.329 | moderate+ |
| an_b3_prof | 0.061 | ok |
| an_b3_seq | -0.550 | HIGH- (thermal bounce) |
| an_b3_table | -0.212 | moderate- |
| an_b3_tree | 0.327 | moderate+ |

**Consistency summary:**

- **an_b3_pred**: won 0/6, lost 6/6
- **an_b3_prof**: won 3/6, lost 2/6
- **an_b3_seq**: won 3/6, lost 3/6
- **an_b3_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b3_pred | 4.4ns | 893.8ns | 0.5% |  |
| an_b3_prof | 6.2ns | 550.4ns | 1.1% |  |
| an_b3_seq | 6.0ns | 600.4ns | 1.0% |  |
| an_b3_table | 6.9ns | 591.6ns | 1.2% |  |
| an_b3_tree | 5.7ns | 535.3ns | 1.1% |  |

## Distribution (algo ns)

```
an_b3_pred (n=6, range 795.0-973.1 ns)
    795.0 |####################
    803.9 |####################
    812.8 |
    821.7 |
    830.6 |
    839.5 |
    848.4 |
    857.3 |
    866.2 |
    875.1 |
    884.0 |
    893.0 |
    901.9 |########################################
    910.8 |
    919.7 |
    928.6 |
    937.5 |
    946.4 |####################
    955.3 |
    964.2 |
  (0 below, 1 above range)

an_b3_prof (n=6, range 487.9-574.0 ns)
    487.9 |####################
    492.2 |
    496.5 |
    500.8 |
    505.1 |
    509.4 |
    513.7 |
    518.0 |
    522.3 |
    526.6 |
    531.0 |
    535.3 |
    539.6 |####################
    543.9 |
    548.2 |
    552.5 |
    556.8 |####################
    561.1 |
    565.4 |########################################
    569.7 |
  (0 below, 1 above range)

an_b3_seq (n=6, range 484.6-704.2 ns)
    484.6 |########################################
    495.6 |
    506.6 |
    517.5 |
    528.5 |
    539.5 |
    550.5 |########################################
    561.5 |########################################
    572.4 |########################################
    583.4 |
    594.4 |
    605.4 |
    616.4 |
    627.3 |########################################
    638.3 |
    649.3 |
    660.3 |
    671.3 |
    682.2 |
    693.2 |
  (0 below, 1 above range)

an_b3_table (n=6, range 497.1-697.5 ns)
    497.1 |########################################
    507.1 |
    517.1 |
    527.2 |########################################
    537.2 |
    547.2 |
    557.2 |########################################
    567.2 |########################################
    577.3 |
    587.3 |
    597.3 |
    607.3 |
    617.3 |########################################
    627.4 |
    637.4 |
    647.4 |
    657.4 |
    667.4 |
    677.5 |
    687.5 |
  (0 below, 1 above range)

an_b3_tree (n=6, range 482.1-572.5 ns)
    482.1 |########################################
    486.6 |########################################
    491.1 |
    495.7 |
    500.2 |
    504.7 |
    509.2 |
    513.7 |
    518.3 |
    522.8 |
    527.3 |
    531.8 |########################################
    536.3 |
    540.9 |
    545.4 |
    549.9 |
    554.4 |
    558.9 |########################################
    563.5 |########################################
    568.0 |
  (0 below, 1 above range)

```
