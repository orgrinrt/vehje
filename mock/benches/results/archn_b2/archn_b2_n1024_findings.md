# Per-branch strategy (NATIVE tier): archetype 2

5 variants, 6 samples per variant.
Baseline: **an_b2_table**

## Highlights

Baseline for all deltas below: **an_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b2_table, an_b2_tree) are a dead heat (<1%)

an_b2_table (8.61 us) and an_b2_tree (8.66 us) differ by 0.61%, inside the noise, even though the wider field spreads 29.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (an_b2_table)

The baseline an_b2_table is the fastest (8.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_b2_table, an_b2_tree, an_b2_prof, an_b2_seq} vs {an_b2_pred} (27% apart)

The field splits into a fast tier {an_b2_table, an_b2_tree, an_b2_prof, an_b2_seq} and a slow tier {an_b2_pred} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_b2_table vs stability leader an_b2_tree (+1% speed for 1.1x steadier)

an_b2_table is fastest (8.61 us, CV 7.2%); an_b2_tree gives up 0.6% median for 1.1x lower variance (CV 6.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b2_tree's edge over baseline is significant but tiny (20 ns, 0.23%)

an_b2_tree differs from baseline an_b2_table by 20 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b2_table) is the fastest** at 8609.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.30x (fastest 8609.4 ns, slowest 11160.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b2_pred | 13087ns | 13738ns | 11512ns | 13019ns | 13977ns | +22.45% |
| an_b2_prof | 11036ns | 11302ns | 9382ns | 10822ns | 12184ns | +3.26% |
| an_b2_seq | 10954ns | 11382ns | 9422ns | 10741ns | 12038ns | +2.49% |
| an_b2_table | 10688ns | 11203ns | 9529ns | 10646ns | 11330ns | base |
| an_b2_tree | 10799ns | 11245ns | 9503ns | 10760ns | 11504ns | +1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b2_pred | 10639ns | 9365ns | 11361ns | +29.12% | 0.096 |
| an_b2_prof | 8519ns | 7240ns | 9403ns | +3.38% | 0.120 |
| an_b2_seq | 8448ns | 7270ns | 9273ns | +2.53% | 0.121 |
| an_b2_table | 8240ns | 7363ns | 8745ns | base | 0.124 |
| an_b2_tree | 8266ns | 7312ns | 8698ns | +0.32% | 0.124 |

## Performance model

- Peak throughput: **0.141 Gops/s** (an_b2_prof; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b2_pred | 0.092 | 64.9% |
| an_b2_prof | 0.117 | 83.1% |
| an_b2_seq | 0.117 | 82.4% |
| an_b2_table | 0.119 | 84.1% |
| an_b2_tree | 0.118 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b2_pred | 13087ns | 13087ns | +22.45% |
| an_b2_prof | 11036ns | 11036ns | +3.26% |
| an_b2_seq | 10954ns | 10954ns | +2.49% |
| an_b2_table | 10688ns | 10688ns | base |
| an_b2_tree | 10799ns | 10799ns | +1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b2_table | 8609ns | base | --- | [7365, 8745] | --- | --- | --- | --- |
| an_b2_pred | 11161ns | +2497.7ns (+29.0%) | [+2030, +2670]ns | [9395, 11361] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b2_prof | 8717ns | no significant difference | [-52, +658]ns | [7436, 9403] | no | 0.4375 | 0.2188 | 0 |
| an_b2_seq | 8784ns | no significant difference | [-77, +610]ns | [7288, 9273] | no | 0.9167 | 0.6875 | 0 |
| an_b2_tree | 8662ns | no significant difference | [-71, +129]ns | [7438, 8698] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b2_table | an_b2_pred | an_b2_prof | an_b2_seq | an_b2_tree |
|---|---|---|---|---|---|
| 1 | 7368ns | +27.9% | +3.6% | -0.8% | +2.7% |
| 2 | 7363ns | +27.2% | -1.7% | -1.3% | -0.7% |
| 3 | 8628ns | +29.9% | +2.3% | +6.2% | +0.5% |
| 4 | 8791ns | +30.6% | +7.8% | +0.5% | -1.0% |
| 5 | 8699ns | +27.8% | +7.3% | +7.9% | -0.0% |
| 6 | 8591ns | +30.8% | +0.2% | +1.6% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b2_pred | 0.423 | moderate+ |
| an_b2_prof | 0.460 | moderate+ |
| an_b2_seq | 0.328 | moderate+ |
| an_b2_table | 0.454 | moderate+ |
| an_b2_tree | 0.389 | moderate+ |

**Consistency summary:**

- **an_b2_pred**: won 0/6, lost 6/6
- **an_b2_prof**: won 1/6, lost 5/6
- **an_b2_seq**: won 2/6, lost 4/6
- **an_b2_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b2_pred | 4.5ns | 10639.1ns | 0.0% |  |
| an_b2_prof | 5.5ns | 8518.7ns | 0.1% |  |
| an_b2_seq | 5.4ns | 8448.4ns | 0.1% |  |
| an_b2_table | 6.3ns | 8239.9ns | 0.1% |  |
| an_b2_tree | 6.3ns | 8266.1ns | 0.1% |  |

## Distribution (algo ns)

```
an_b2_pred (n=6, range 9364.6-11361.5 ns)
   9364.6 |########################################
   9464.4 |
   9564.3 |
   9664.1 |
   9764.0 |
   9863.8 |
   9963.7 |
  10063.5 |
  10163.3 |
  10263.2 |
  10363.0 |
  10462.9 |
  10562.7 |
  10662.6 |
  10762.4 |
  10862.2 |
  10962.1 |
  11061.9 |####################
  11161.8 |########################################
  11261.6 |
  (0 below, 1 above range)

an_b2_prof (n=6, range 7239.6-9403.2 ns)
   7239.6 |########################################
   7347.8 |
   7456.0 |
   7564.1 |########################################
   7672.3 |
   7780.5 |
   7888.7 |
   7996.8 |
   8105.0 |
   8213.2 |
   8321.4 |
   8429.6 |
   8537.7 |########################################
   8645.9 |
   8754.1 |########################################
   8862.3 |
   8970.4 |
   9078.6 |
   9186.8 |
   9295.0 |########################################
  (0 below, 1 above range)

an_b2_seq (n=6, range 7270.4-9273.2 ns)
   7270.4 |########################################
   7370.5 |
   7470.7 |
   7570.8 |
   7670.9 |
   7771.1 |
   7871.2 |
   7971.4 |
   8071.5 |
   8171.6 |
   8271.8 |
   8371.9 |
   8472.1 |
   8572.2 |
   8672.3 |####################
   8772.5 |####################
   8872.6 |
   8972.7 |
   9072.9 |####################
   9173.0 |
  (0 below, 1 above range)

an_b2_table (n=6, range 7362.9-8744.8 ns)
   7362.9 |########################################
   7432.0 |
   7501.1 |
   7570.2 |
   7639.3 |
   7708.4 |
   7777.5 |
   7846.6 |
   7915.7 |
   7984.8 |
   8053.8 |
   8122.9 |
   8192.0 |
   8261.1 |
   8330.2 |
   8399.3 |
   8468.4 |
   8537.5 |####################
   8606.6 |####################
   8675.7 |####################
  (0 below, 1 above range)

an_b2_tree (n=6, range 7312.1-8698.1 ns)
   7312.1 |#############
   7381.4 |
   7450.7 |
   7520.0 |#############
   7589.3 |
   7658.6 |
   7727.9 |
   7797.2 |
   7866.5 |
   7935.8 |
   8005.1 |
   8074.4 |
   8143.7 |
   8213.0 |
   8282.3 |
   8351.6 |
   8420.9 |
   8490.2 |
   8559.5 |
   8628.8 |########################################
  (0 below, 1 above range)

```
