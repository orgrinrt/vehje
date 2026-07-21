# Per-branch strategy (NATIVE tier): archetype 3

5 variants, 6 samples per variant.
Baseline: **an_b3_table**

## Highlights

Baseline for all deltas below: **an_b3_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b3_table, an_b3_prof) are a dead heat (<1%)

an_b3_table (8.61 us) and an_b3_prof (8.68 us) differ by 0.79%, inside the noise, even though the wider field spreads 59.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b3_tree shows alternating (throttle bounce) (autocorr -0.53)

an_b3_tree's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (an_b3_table)

The baseline an_b3_table is the fastest (8.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b3_table, an_b3_prof, an_b3_seq, an_b3_tree} vs {an_b3_pred} (58% apart)

The field splits into a fast tier {an_b3_table, an_b3_prof, an_b3_seq, an_b3_tree} and a slow tier {an_b3_pred} with a 58% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b3_table vs stability leader an_b3_tree (+1% speed for 1.8x steadier)

an_b3_table is fastest (8.61 us, CV 6.4%); an_b3_tree gives up 0.8% median for 1.8x lower variance (CV 3.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b3_prof's edge over baseline is significant but tiny (29 ns, 0.33%)

an_b3_prof differs from baseline an_b3_table by 29 ns (0.33%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b3_table) is the fastest** at 8614.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.59x (fastest 8614.5 ns, slowest 13694.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b3_pred | 16378ns | 16176ns | 15563ns | 15974ns | 17391ns | +51.11% |
| an_b3_prof | 10847ns | 11266ns | 9367ns | 10744ns | 11743ns | +0.08% |
| an_b3_seq | 10769ns | 11264ns | 9710ns | 10786ns | 11273ns | -0.64% |
| an_b3_table | 10838ns | 11179ns | 9403ns | 10991ns | 11326ns | base |
| an_b3_tree | 11137ns | 11314ns | 10532ns | 11084ns | 11519ns | +2.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b3_pred | 13882ns | 13152ns | 14792ns | +65.48% | 0.074 |
| an_b3_prof | 8347ns | 7251ns | 9050ns | -0.50% | 0.123 |
| an_b3_seq | 8305ns | 7470ns | 8710ns | -1.00% | 0.123 |
| an_b3_table | 8389ns | 7252ns | 8818ns | base | 0.122 |
| an_b3_tree | 8570ns | 8128ns | 8873ns | +2.15% | 0.119 |

## Performance model

- Peak throughput: **0.141 Gops/s** (an_b3_prof; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b3_pred | 0.075 | 52.9% |
| an_b3_prof | 0.118 | 83.5% |
| an_b3_seq | 0.118 | 83.5% |
| an_b3_table | 0.119 | 84.2% |
| an_b3_tree | 0.118 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b3_pred | 16378ns | 16378ns | +51.11% |
| an_b3_prof | 10847ns | 10847ns | +0.08% |
| an_b3_seq | 10769ns | 10769ns | -0.64% |
| an_b3_table | 10838ns | 10838ns | base |
| an_b3_tree | 11137ns | 11137ns | +2.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b3_table | 8615ns | base | --- | [7735, 8818] | --- | --- | --- | --- |
| an_b3_pred | 13695ns | +4948.8ns (+57.4%) | [+4473, +7058]ns | [13160, 14792] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b3_prof | 8683ns | no significant difference | [-796, +642]ns | [7308, 9050] | no | 1.0000 | 1.0000 | 0 |
| an_b3_seq | 8686ns | no significant difference | [-1151, +958]ns | [7519, 8710] | no | 1.0000 | 1.0000 | 0 |
| an_b3_tree | 8687ns | no significant difference | [-521, +999]ns | [8149, 8873] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b3_table | an_b3_pred | an_b3_prof | an_b3_seq | an_b3_tree |
|---|---|---|---|---|---|
| 1 | 7252ns | +97.7% | -0.0% | +20.1% | +22.0% |
| 2 | 8742ns | +50.6% | -15.7% | -13.4% | -6.5% |
| 3 | 8894ns | +59.6% | -2.4% | -2.1% | -1.6% |
| 4 | 8218ns | +85.5% | +14.4% | +5.6% | +4.9% |
| 5 | 8631ns | +52.4% | +0.7% | +0.8% | +3.1% |
| 6 | 8598ns | +53.5% | +1.2% | -13.1% | -5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b3_pred | -0.168 | ok |
| an_b3_prof | 0.436 | moderate+ |
| an_b3_seq | -0.339 | moderate- |
| an_b3_table | -0.167 | ok |
| an_b3_tree | -0.528 | HIGH- (thermal bounce) |

**Consistency summary:**

- **an_b3_pred**: won 0/6, lost 6/6
- **an_b3_prof**: won 2/6, lost 3/6
- **an_b3_seq**: won 3/6, lost 3/6
- **an_b3_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b3_pred | 23.2ns | 13882.3ns | 0.2% |  |
| an_b3_prof | 6.1ns | 8347.0ns | 0.1% |  |
| an_b3_seq | 6.8ns | 8304.9ns | 0.1% |  |
| an_b3_table | 6.0ns | 8389.1ns | 0.1% |  |
| an_b3_tree | 7.1ns | 8569.8ns | 0.1% |  |

## Distribution (algo ns)

```
an_b3_pred (n=6, range 13152.1-14792.5 ns)
  13152.1 |########################################
  13234.1 |
  13316.1 |
  13398.2 |
  13480.2 |
  13562.2 |
  13644.2 |
  13726.2 |
  13808.3 |
  13890.3 |
  13972.3 |
  14054.3 |
  14136.3 |#############
  14218.4 |
  14300.4 |#############
  14382.4 |
  14464.4 |
  14546.4 |
  14628.5 |
  14710.5 |
  (0 below, 1 above range)

an_b3_prof (n=6, range 7250.8-9049.6 ns)
   7250.8 |####################
   7340.7 |####################
   7430.7 |
   7520.6 |
   7610.6 |
   7700.5 |
   7790.4 |
   7880.4 |
   7970.3 |
   8060.3 |
   8150.2 |
   8240.1 |
   8330.1 |
   8420.0 |
   8510.0 |
   8599.9 |########################################
   8689.8 |####################
   8779.8 |
   8869.7 |
   8959.7 |
  (0 below, 1 above range)

an_b3_seq (n=6, range 7470.0-8709.6 ns)
   7470.0 |#############
   7532.0 |#############
   7594.0 |
   7655.9 |
   7717.9 |
   7779.9 |
   7841.9 |
   7903.9 |
   7965.8 |
   8027.8 |
   8089.8 |
   8151.8 |
   8213.8 |
   8275.7 |
   8337.7 |
   8399.7 |
   8461.7 |
   8523.7 |
   8585.6 |
   8647.6 |########################################
  (0 below, 1 above range)

an_b3_table (n=6, range 7252.1-8817.9 ns)
   7252.1 |####################
   7330.4 |
   7408.7 |
   7487.0 |
   7565.3 |
   7643.6 |
   7721.8 |
   7800.1 |
   7878.4 |
   7956.7 |
   8035.0 |
   8113.3 |
   8191.6 |####################
   8269.9 |
   8348.2 |
   8426.5 |
   8504.7 |
   8583.0 |########################################
   8661.3 |
   8739.6 |####################
  (0 below, 1 above range)

an_b3_tree (n=6, range 8127.9-8873.2 ns)
   8127.9 |########################################
   8165.2 |########################################
   8202.4 |
   8239.7 |
   8277.0 |
   8314.2 |
   8351.5 |
   8388.7 |
   8426.0 |
   8463.3 |
   8500.5 |
   8537.8 |
   8575.1 |
   8612.3 |########################################
   8649.6 |
   8686.8 |
   8724.1 |########################################
   8761.4 |
   8798.6 |
   8835.9 |########################################
  (0 below, 1 above range)

```
