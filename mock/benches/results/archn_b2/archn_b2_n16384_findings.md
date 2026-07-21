# Per-branch strategy (NATIVE tier): archetype 2

5 variants, 6 samples per variant.
Baseline: **an_b2_table**

## Highlights

Baseline for all deltas below: **an_b2_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b2_pred dominates: 35% faster than the next best (an_b2_prof)

an_b2_pred (244.51 us) leads an_b2_prof (329.54 us) by 35%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### an_b2_pred beats baseline by 26% (significant)

an_b2_pred is -86.05 us (26%) faster than baseline an_b2_table, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### an_b2_prof shows alternating (throttle bounce) (autocorr -0.65)

an_b2_prof's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {an_b2_pred} vs {an_b2_prof, an_b2_table, an_b2_tree, an_b2_seq} (35% apart)

The field splits into a fast tier {an_b2_pred} and a slow tier {an_b2_prof, an_b2_table, an_b2_tree, an_b2_seq} with a 35% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: an_b2_pred** at 244510.4 ns median (-26.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.39x (fastest 244510.4 ns, slowest 340121.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b2_pred | 249669ns | 246793ns | 243759ns | 246420ns | 257498ns | -25.41% |
| an_b2_prof | 333617ns | 332220ns | 325305ns | 330369ns | 342646ns | -0.33% |
| an_b2_seq | 341717ns | 342612ns | 332330ns | 341640ns | 346526ns | +2.09% |
| an_b2_table | 334727ns | 336119ns | 323342ns | 334924ns | 340123ns | base |
| an_b2_tree | 336931ns | 338931ns | 322538ns | 338350ns | 341999ns | +0.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b2_pred | 247309ns | 241234ns | 255069ns | -25.54% | 0.066 |
| an_b2_prof | 330935ns | 322494ns | 340030ns | -0.36% | 0.050 |
| an_b2_seq | 339125ns | 329792ns | 343659ns | +2.11% | 0.048 |
| an_b2_table | 332121ns | 320617ns | 337475ns | base | 0.049 |
| an_b2_tree | 334255ns | 320078ns | 339206ns | +0.64% | 0.049 |

## Performance model

- Peak throughput: **0.068 Gops/s** (an_b2_pred; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b2_pred | 0.067 | 98.7% |
| an_b2_prof | 0.050 | 73.2% |
| an_b2_seq | 0.048 | 70.9% |
| an_b2_table | 0.049 | 72.3% |
| an_b2_tree | 0.049 | 71.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b2_pred | 249669ns | 249669ns | -25.41% |
| an_b2_prof | 333617ns | 333617ns | -0.33% |
| an_b2_seq | 341717ns | 341717ns | +2.09% |
| an_b2_table | 334727ns | 334727ns | base |
| an_b2_tree | 336931ns | 336931ns | +0.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b2_table | 333719ns | base | --- | [325170, 337475] | --- | --- | --- | --- |
| an_b2_pred | 244510ns | -86048.2ns (-25.8%) | [-91881, -76508]ns | [242347, 255069] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b2_prof | 329541ns | no significant difference | [-11576, +7920]ns | [323233, 340030] | no | 1.0000 | 1.0000 | 0 |
| an_b2_seq | 340121ns | +6184.6ns (+1.9%) | [+626, +14201]ns | [333594, 343659] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b2_tree | 336352ns | no significant difference | [-5940, +11322]ns | [327205, 339206] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b2_table | an_b2_pred | an_b2_prof | an_b2_seq | an_b2_tree |
|---|---|---|---|---|---|
| 1 | 336214ns | -27.0% | -2.2% | +0.4% | +0.6% |
| 2 | 329722ns | -26.8% | -1.7% | +0.0% | -2.9% |
| 3 | 331224ns | -25.2% | +1.9% | +3.0% | +1.0% |
| 4 | 338381ns | -22.4% | -4.7% | +1.0% | +0.0% |
| 5 | 336568ns | -27.7% | +1.8% | +2.7% | -0.7% |
| 6 | 320617ns | -24.1% | +3.0% | +5.7% | +6.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b2_pred | -0.099 | ok |
| an_b2_prof | -0.646 | HIGH- (thermal bounce) |
| an_b2_seq | 0.132 | ok |
| an_b2_table | -0.171 | ok |
| an_b2_tree | -0.216 | moderate- |

**Consistency summary:**

- **an_b2_pred**: won 6/6, lost 0/6
- **an_b2_prof**: won 3/6, lost 3/6
- **an_b2_seq**: won 0/6, lost 5/6
- **an_b2_tree**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b2_pred | 6.0ns | 247308.7ns | 0.0% |  |
| an_b2_prof | 12.2ns | 330934.7ns | 0.0% |  |
| an_b2_seq | 10.8ns | 339124.8ns | 0.0% |  |
| an_b2_table | 12.3ns | 332121.0ns | 0.0% |  |
| an_b2_tree | 14.6ns | 334254.7ns | 0.0% |  |

## Distribution (algo ns)

```
an_b2_pred (n=6, range 241233.7-255069.0 ns)
  241233.7 |####################
  241925.5 |
  242617.2 |
  243309.0 |########################################
  244000.8 |
  244692.5 |
  245384.3 |####################
  246076.0 |
  246767.8 |
  247459.6 |####################
  248151.3 |
  248843.1 |
  249534.9 |
  250226.6 |
  250918.4 |
  251610.1 |
  252301.9 |
  252993.7 |
  253685.4 |
  254377.2 |
  (0 below, 1 above range)

an_b2_prof (n=6, range 322494.2-340030.2 ns)
  322494.2 |########################################
  323371.0 |########################################
  324247.8 |
  325124.6 |
  326001.4 |
  326878.2 |
  327755.0 |
  328631.8 |########################################
  329508.6 |########################################
  330385.4 |
  331262.2 |
  332139.0 |
  333015.8 |
  333892.6 |
  334769.4 |
  335646.2 |
  336523.0 |
  337399.8 |########################################
  338276.6 |
  339153.4 |
  (0 below, 1 above range)

an_b2_seq (n=6, range 329792.1-343659.2 ns)
  329792.1 |########################################
  330485.5 |
  331178.8 |
  331872.2 |
  332565.5 |
  333258.9 |
  333952.2 |
  334645.6 |
  335338.9 |
  336032.3 |
  336725.6 |########################################
  337419.0 |
  338112.3 |
  338805.7 |########################################
  339499.0 |
  340192.4 |
  340885.7 |########################################
  341579.1 |########################################
  342272.4 |
  342965.8 |
  (0 below, 1 above range)

an_b2_table (n=6, range 320616.7-337474.5 ns)
  320616.7 |####################
  321459.6 |
  322302.5 |
  323145.4 |
  323988.3 |
  324831.2 |
  325674.1 |
  326516.9 |
  327359.8 |
  328202.7 |
  329045.6 |####################
  329888.5 |
  330731.4 |####################
  331574.3 |
  332417.2 |
  333260.1 |
  334103.0 |
  334945.9 |
  335788.8 |########################################
  336631.7 |
  (0 below, 1 above range)

an_b2_tree (n=6, range 320078.3-339206.2 ns)
  320078.3 |########################################
  321034.7 |
  321991.1 |
  322947.5 |
  323903.9 |
  324860.3 |
  325816.7 |
  326773.1 |
  327729.5 |
  328685.9 |
  329642.3 |
  330598.7 |
  331555.1 |
  332511.5 |
  333467.9 |########################################
  334424.3 |########################################
  335380.7 |
  336337.1 |
  337293.5 |########################################
  338249.9 |########################################
  (0 below, 1 above range)

```
