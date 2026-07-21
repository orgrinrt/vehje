# Per-branch strategy (NATIVE tier): archetype 5

5 variants, 6 samples per variant.
Baseline: **an_b5_table**

## Highlights

Baseline for all deltas below: **an_b5_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b5_tree, an_b5_table) are a dead heat (<1%)

an_b5_tree (55.81 us) and an_b5_table (55.88 us) differ by 0.12%, inside the noise, even though the wider field spreads 28.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b5_tree shows alternating (throttle bounce) (autocorr -0.72)

an_b5_tree's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: an_b5_tree** at 55810.0 ns median (-0.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.28x (fastest 55810.0 ns, slowest 71456.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b5_pred | 74174ns | 73726ns | 70538ns | 72946ns | 77836ns | +25.29% |
| an_b5_prof | 72429ns | 70325ns | 69544ns | 70065ns | 77419ns | +22.34% |
| an_b5_seq | 67758ns | 67486ns | 60826ns | 66152ns | 73633ns | +14.45% |
| an_b5_table | 59205ns | 58225ns | 49078ns | 57312ns | 67106ns | base |
| an_b5_tree | 56983ns | 58122ns | 49794ns | 55453ns | 62872ns | -3.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b5_pred | 71829ns | 68318ns | 75293ns | +26.34% | 0.057 |
| an_b5_prof | 70167ns | 67348ns | 75037ns | +23.41% | 0.058 |
| an_b5_seq | 65485ns | 58548ns | 71316ns | +15.18% | 0.063 |
| an_b5_table | 56855ns | 46778ns | 64653ns | base | 0.072 |
| an_b5_tree | 54642ns | 47599ns | 60384ns | -3.89% | 0.075 |

## Performance model

- Peak throughput: **0.088 Gops/s** (an_b5_table; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b5_pred | 0.057 | 65.5% |
| an_b5_prof | 0.060 | 68.7% |
| an_b5_seq | 0.063 | 71.7% |
| an_b5_table | 0.073 | 83.7% |
| an_b5_tree | 0.073 | 83.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b5_pred | 74174ns | 74174ns | +25.29% |
| an_b5_prof | 72429ns | 72429ns | +22.34% |
| an_b5_seq | 67758ns | 67758ns | +14.45% |
| an_b5_table | 59205ns | 59205ns | base |
| an_b5_tree | 56983ns | 56983ns | -3.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b5_table | 55879ns | base | --- | [50034, 64653] | --- | --- | --- | --- |
| an_b5_pred | 71457ns | +13561.9ns (+24.3%) | [+9936, +21423]ns | [68737, 75293] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b5_prof | 68104ns | +13187.0ns (+23.6%) | [+9297, +17451]ns | [67360, 75037] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b5_seq | 65255ns | no significant difference | [-2932, +18908]ns | [59884, 71316] | no | 0.9167 | 0.6875 | 0 |
| an_b5_tree | 55810ns | no significant difference | [-12401, +7764]ns | [47731, 60384] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b5_table | an_b5_pred | an_b5_prof | an_b5_seq | an_b5_tree |
|---|---|---|---|---|---|
| 1 | 53289ns | +34.9% | +26.4% | +9.9% | -10.2% |
| 2 | 46778ns | +51.8% | +44.0% | +48.8% | +29.6% |
| 3 | 65797ns | +17.1% | +12.8% | -5.4% | -27.7% |
| 4 | 58462ns | +16.9% | +17.3% | +24.9% | +2.9% |
| 5 | 53295ns | +29.8% | +26.8% | +28.1% | +2.7% |
| 6 | 63510ns | +15.8% | +19.4% | -3.6% | -10.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b5_pred | -0.354 | moderate- |
| an_b5_prof | -0.278 | moderate- |
| an_b5_seq | -0.362 | moderate- |
| an_b5_table | -0.273 | moderate- |
| an_b5_tree | -0.724 | HIGH- (thermal bounce) |

**Consistency summary:**

- **an_b5_pred**: won 0/6, lost 6/6
- **an_b5_prof**: won 0/6, lost 6/6
- **an_b5_seq**: won 2/6, lost 4/6
- **an_b5_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b5_pred | 4.4ns | 71828.8ns | 0.0% |  |
| an_b5_prof | 5.6ns | 70166.8ns | 0.0% |  |
| an_b5_seq | 6.3ns | 65484.6ns | 0.0% |  |
| an_b5_table | 10.9ns | 56855.2ns | 0.0% |  |
| an_b5_tree | 5.9ns | 54641.6ns | 0.0% |  |

## Distribution (algo ns)

```
an_b5_pred (n=6, range 68318.3-75292.7 ns)
  68318.3 |########################################
  68667.0 |
  69015.7 |########################################
  69364.5 |
  69713.2 |
  70061.9 |
  70410.6 |
  70759.3 |########################################
  71108.1 |
  71456.8 |
  71805.5 |########################################
  72154.2 |
  72502.9 |
  72851.7 |
  73200.4 |########################################
  73549.1 |
  73897.8 |
  74246.5 |
  74595.3 |
  74944.0 |
  (0 below, 1 above range)

an_b5_prof (n=6, range 67347.5-75036.9 ns)
  67347.5 |########################################
  67732.0 |
  68116.4 |
  68500.9 |#############
  68885.4 |
  69269.8 |
  69654.3 |
  70038.8 |
  70423.2 |
  70807.7 |
  71192.2 |
  71576.6 |
  71961.1 |
  72345.6 |
  72730.0 |
  73114.5 |
  73499.0 |
  73883.4 |#############
  74267.9 |
  74652.4 |
  (0 below, 1 above range)

an_b5_seq (n=6, range 58547.9-71315.6 ns)
  58547.9 |########################################
  59186.3 |
  59824.7 |
  60463.1 |
  61101.4 |########################################
  61739.8 |########################################
  62378.2 |
  63016.6 |
  63655.0 |
  64293.4 |
  64931.8 |
  65570.2 |
  66208.6 |
  66846.9 |
  67485.3 |
  68123.7 |########################################
  68762.1 |
  69400.5 |########################################
  70038.9 |
  70677.3 |
  (0 below, 1 above range)

an_b5_table (n=6, range 46778.3-64653.1 ns)
  46778.3 |####################
  47672.0 |
  48565.8 |
  49459.5 |
  50353.3 |
  51247.0 |
  52140.8 |
  53034.5 |########################################
  53928.2 |
  54822.0 |
  55715.7 |
  56609.5 |
  57503.2 |
  58397.0 |####################
  59290.7 |
  60184.4 |
  61078.2 |
  61971.9 |
  62865.7 |####################
  63759.4 |
  (0 below, 1 above range)

an_b5_tree (n=6, range 47599.2-60383.8 ns)
  47599.2 |########################################
  48238.4 |
  48877.7 |
  49516.9 |
  50156.1 |
  50795.3 |
  51434.6 |
  52073.8 |
  52713.0 |
  53352.2 |
  53991.5 |
  54630.7 |####################
  55269.9 |
  55909.2 |
  56548.4 |####################
  57187.6 |
  57826.8 |
  58466.1 |
  59105.3 |
  59744.5 |####################
  (0 below, 1 above range)

```
