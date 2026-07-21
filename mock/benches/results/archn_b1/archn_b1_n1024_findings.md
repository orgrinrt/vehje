# Per-branch strategy (NATIVE tier): archetype 1

5 variants, 6 samples per variant.
Baseline: **an_b1_table**

## Highlights

Baseline for all deltas below: **an_b1_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (an_b1_table)

The baseline an_b1_table is the fastest (8.24 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### an_b1_prof's edge over baseline is significant but tiny (36 ns, 0.43%)

an_b1_prof differs from baseline an_b1_table by 36 ns (0.43%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b1_table) is the fastest** at 8238.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 8238.4 ns, slowest 10173.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b1_pred | 12420ns | 12879ns | 10753ns | 12575ns | 13022ns | +18.32% |
| an_b1_prof | 10702ns | 11274ns | 9304ns | 10655ns | 11471ns | +1.95% |
| an_b1_seq | 10706ns | 11238ns | 9451ns | 10655ns | 11408ns | +1.99% |
| an_b1_table | 10497ns | 10645ns | 9468ns | 10281ns | 11334ns | base |
| an_b1_tree | 10782ns | 11280ns | 9440ns | 10750ns | 11501ns | +2.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b1_pred | 9872ns | 8606ns | 10361ns | +21.76% | 0.104 |
| an_b1_prof | 8242ns | 7168ns | 8828ns | +1.66% | 0.124 |
| an_b1_seq | 8238ns | 7290ns | 8738ns | +1.61% | 0.124 |
| an_b1_table | 8108ns | 7309ns | 8731ns | base | 0.126 |
| an_b1_tree | 8318ns | 7262ns | 8889ns | +2.59% | 0.123 |

## Performance model

- Peak throughput: **0.143 Gops/s** (an_b1_prof; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b1_pred | 0.101 | 70.5% |
| an_b1_prof | 0.118 | 82.6% |
| an_b1_seq | 0.118 | 82.7% |
| an_b1_table | 0.124 | 87.0% |
| an_b1_tree | 0.118 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b1_pred | 12420ns | 12420ns | +18.32% |
| an_b1_prof | 10702ns | 10702ns | +1.95% |
| an_b1_seq | 10706ns | 10706ns | +1.99% |
| an_b1_table | 10497ns | 10497ns | base |
| an_b1_tree | 10782ns | 10782ns | +2.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b1_table | 8238ns | base | --- | [7353, 8731] | --- | --- | --- | --- |
| an_b1_pred | 10173ns | +1813.3ns (+22.0%) | [+1326, +2153]ns | [9081, 10361] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b1_prof | 8682ns | no significant difference | [-568, +936]ns | [7217, 8828] | no | 1.0000 | 1.0000 | 0 |
| an_b1_seq | 8672ns | no significant difference | [-77, +485]ns | [7306, 8738] | no | 0.9167 | 0.6875 | 0 |
| an_b1_tree | 8697ns | no significant difference | [-52, +514]ns | [7367, 8889] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b1_table | an_b1_pred | an_b1_prof | an_b1_seq | an_b1_tree |
|---|---|---|---|---|---|
| 1 | 8244ns | +25.5% | -13.1% | +6.5% | +5.0% |
| 2 | 8232ns | +26.1% | +6.7% | +5.2% | +7.5% |
| 3 | 7397ns | +29.2% | +17.9% | -1.0% | -1.8% |
| 4 | 7309ns | +17.7% | -0.6% | -0.3% | +2.2% |
| 5 | 8757ns | +15.5% | +1.3% | -0.9% | +2.0% |
| 6 | 8706ns | +17.5% | -0.7% | -0.1% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b1_pred | 0.110 | ok |
| an_b1_prof | -0.360 | moderate- |
| an_b1_seq | 0.188 | ok |
| an_b1_table | 0.187 | ok |
| an_b1_tree | 0.091 | ok |

**Consistency summary:**

- **an_b1_pred**: won 0/6, lost 6/6
- **an_b1_prof**: won 3/6, lost 3/6
- **an_b1_seq**: won 4/6, lost 2/6
- **an_b1_tree**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b1_pred | 6.2ns | 9871.9ns | 0.1% |  |
| an_b1_prof | 6.2ns | 8242.4ns | 0.1% |  |
| an_b1_seq | 6.5ns | 8238.3ns | 0.1% |  |
| an_b1_table | 6.2ns | 8107.7ns | 0.1% |  |
| an_b1_tree | 7.0ns | 8317.6ns | 0.1% |  |

## Distribution (algo ns)

```
an_b1_pred (n=6, range 8605.8-10361.0 ns)
   8605.8 |########################################
   8693.6 |
   8781.3 |
   8869.1 |
   8956.8 |
   9044.6 |
   9132.4 |
   9220.1 |
   9307.9 |
   9395.7 |
   9483.4 |########################################
   9571.2 |
   9658.9 |
   9746.7 |
   9834.5 |
   9922.2 |
  10010.0 |
  10097.8 |########################################
  10185.5 |########################################
  10273.3 |########################################
  (0 below, 1 above range)

an_b1_prof (n=6, range 7167.5-8828.4 ns)
   7167.5 |########################################
   7250.5 |########################################
   7333.6 |
   7416.6 |
   7499.7 |
   7582.7 |
   7665.8 |
   7748.8 |
   7831.8 |
   7914.9 |
   7997.9 |
   8081.0 |
   8164.0 |
   8247.1 |
   8330.1 |
   8413.1 |
   8496.2 |
   8579.2 |########################################
   8662.3 |########################################
   8745.3 |########################################
  (0 below, 1 above range)

an_b1_seq (n=6, range 7290.4-8737.7 ns)
   7290.4 |########################################
   7362.8 |
   7435.1 |
   7507.5 |
   7579.9 |
   7652.2 |
   7724.6 |
   7797.0 |
   7869.3 |
   7941.7 |
   8014.1 |
   8086.4 |
   8158.8 |
   8231.1 |
   8303.5 |
   8375.9 |
   8448.2 |
   8520.6 |
   8593.0 |####################
   8665.3 |########################################
  (0 below, 1 above range)

an_b1_table (n=6, range 7309.2-8731.5 ns)
   7309.2 |########################################
   7380.3 |########################################
   7451.4 |
   7522.5 |
   7593.6 |
   7664.8 |
   7735.9 |
   7807.0 |
   7878.1 |
   7949.2 |
   8020.3 |
   8091.4 |
   8162.6 |########################################
   8233.7 |########################################
   8304.8 |
   8375.9 |
   8447.0 |
   8518.1 |
   8589.2 |
   8660.3 |########################################
  (0 below, 1 above range)

an_b1_tree (n=6, range 7262.5-8889.1 ns)
   7262.5 |########################################
   7343.8 |
   7425.2 |########################################
   7506.5 |
   7587.8 |
   7669.2 |
   7750.5 |
   7831.8 |
   7913.2 |
   7994.5 |
   8075.8 |
   8157.2 |
   8238.5 |
   8319.8 |
   8401.2 |
   8482.5 |
   8563.8 |
   8645.2 |########################################
   8726.5 |########################################
   8807.8 |########################################
  (0 below, 1 above range)

```
