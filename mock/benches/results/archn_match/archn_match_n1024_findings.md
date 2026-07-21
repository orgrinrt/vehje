# Per-type strategy (NATIVE tier): all match

5 variants, 6 samples per variant.
Baseline: **an_match_table**

## Highlights

Baseline for all deltas below: **an_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_match_table is fastest but the noisiest (CV 10.5%)

an_match_table wins on median (8.64 us) yet has the highest variance (CV 10.5%), while an_match_tree is the steadiest (CV 6.4%, 8.70 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (an_match_table, an_match_prof) are a dead heat (<1%)

an_match_table (8.64 us) and an_match_prof (8.66 us) differ by 0.28%, inside the noise, even though the wider field spreads 82.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (an_match_table)

The baseline an_match_table is the fastest (8.64 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {an_match_table, an_match_prof, an_match_tree, an_match_seq} vs {an_match_pred} (73% apart)

The field splits into a fast tier {an_match_table, an_match_prof, an_match_tree, an_match_seq} and a slow tier {an_match_pred} with a 73% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader an_match_table vs stability leader an_match_tree (+1% speed for 1.6x steadier)

an_match_table is fastest (8.64 us, CV 10.5%); an_match_tree gives up 0.7% median for 1.6x lower variance (CV 6.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_match_prof's edge over baseline is significant but tiny (-21 ns, 0.25%)

an_match_prof differs from baseline an_match_table by -21 ns (0.25%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_match_table) is the fastest** at 8635.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.82x (fastest 8635.4 ns, slowest 15751.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_match_pred | 18140ns | 18330ns | 15291ns | 18269ns | 19372ns | +64.19% |
| an_match_prof | 10906ns | 11247ns | 9350ns | 10649ns | 12068ns | -1.29% |
| an_match_seq | 11397ns | 11820ns | 9589ns | 11386ns | 12316ns | +3.15% |
| an_match_table | 11048ns | 11211ns | 9411ns | 10722ns | 12356ns | base |
| an_match_tree | 11021ns | 11278ns | 9455ns | 11263ns | 11441ns | -0.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_match_pred | 15583ns | 13140ns | 16637ns | +82.59% | 0.066 |
| an_match_prof | 8405ns | 7232ns | 9284ns | -1.52% | 0.122 |
| an_match_seq | 8782ns | 7411ns | 9454ns | +2.90% | 0.117 |
| an_match_table | 8534ns | 7257ns | 9579ns | base | 0.120 |
| an_match_tree | 8514ns | 7298ns | 8861ns | -0.24% | 0.120 |

## Performance model

- Peak throughput: **0.142 Gops/s** (an_match_prof; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_match_pred | 0.065 | 45.9% |
| an_match_prof | 0.118 | 83.5% |
| an_match_seq | 0.112 | 79.3% |
| an_match_table | 0.119 | 83.8% |
| an_match_tree | 0.118 | 83.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_match_pred | 18140ns | 18140ns | +64.19% |
| an_match_prof | 10906ns | 10906ns | -1.29% |
| an_match_seq | 11397ns | 11397ns | +3.15% |
| an_match_table | 11048ns | 11048ns | base |
| an_match_tree | 11021ns | 11021ns | -0.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_match_table | 8635ns | base | --- | [7389, 9579] | --- | --- | --- | --- |
| an_match_pred | 15752ns | +7062.3ns (+81.8%) | [+6374, +7710]ns | [14361, 16637] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_match_prof | 8659ns | no significant difference | [-435, +68]ns | [7271, 9284] | no | 0.9167 | 0.6875 | 0 |
| an_match_seq | 9126ns | no significant difference | [-374, +1048]ns | [7767, 9454] | no | 1.0000 | 1.0000 | 0 |
| an_match_tree | 8698ns | no significant difference | [-884, +764]ns | [7984, 8861] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_match_table | an_match_pred | an_match_prof | an_match_seq | an_match_tree |
|---|---|---|---|---|---|
| 1 | 7521ns | +108.6% | -3.8% | +17.6% | +19.1% |
| 2 | 9732ns | +74.5% | -0.1% | -2.9% | -10.4% |
| 3 | 9425ns | +72.8% | -6.2% | -0.2% | -8.0% |
| 4 | 8686ns | +82.1% | -0.4% | +8.9% | +0.9% |
| 5 | 8585ns | +81.5% | +1.0% | -5.4% | +1.1% |
| 6 | 7257ns | +81.1% | +0.7% | +2.1% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_match_pred | 0.154 | ok |
| an_match_prof | -0.233 | moderate- |
| an_match_seq | 0.371 | moderate+ |
| an_match_table | -0.014 | ok |
| an_match_tree | 0.003 | ok |

**Consistency summary:**

- **an_match_pred**: won 0/6, lost 6/6
- **an_match_prof**: won 3/6, lost 2/6
- **an_match_seq**: won 3/6, lost 3/6
- **an_match_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_match_pred | 5.4ns | 15583.3ns | 0.0% |  |
| an_match_prof | 6.7ns | 8404.9ns | 0.1% |  |
| an_match_seq | 5.6ns | 8782.1ns | 0.1% |  |
| an_match_table | 5.7ns | 8534.4ns | 0.1% |  |
| an_match_tree | 6.9ns | 8514.3ns | 0.1% |  |

## Distribution (algo ns)

```
an_match_pred (n=6, range 13139.6-16636.8 ns)
  13139.6 |########################################
  13314.5 |
  13489.3 |
  13664.2 |
  13839.0 |
  14013.9 |
  14188.8 |
  14363.6 |
  14538.5 |
  14713.4 |
  14888.2 |
  15063.1 |
  15237.9 |
  15412.8 |########################################
  15587.7 |########################################
  15762.5 |########################################
  15937.4 |
  16112.3 |
  16287.1 |########################################
  16462.0 |
  (0 below, 1 above range)

an_match_prof (n=6, range 7232.5-9284.4 ns)
   7232.5 |########################################
   7335.1 |
   7437.7 |
   7540.3 |
   7642.9 |
   7745.5 |
   7848.1 |
   7950.7 |
   8053.3 |
   8155.9 |
   8258.5 |
   8361.0 |
   8463.6 |
   8566.2 |########################################
   8668.8 |
   8771.4 |####################
   8874.0 |
   8976.6 |
   9079.2 |
   9181.8 |
  (0 below, 1 above range)

an_match_seq (n=6, range 7411.2-9453.8 ns)
   7411.2 |####################
   7513.3 |
   7615.5 |
   7717.6 |
   7819.7 |
   7921.8 |
   8024.0 |####################
   8126.1 |
   8228.2 |
   8330.3 |
   8432.5 |
   8534.6 |
   8636.7 |
   8738.9 |
   8841.0 |####################
   8943.1 |
   9045.2 |
   9147.4 |
   9249.5 |
   9351.6 |########################################
  (0 below, 1 above range)

an_match_table (n=6, range 7257.1-9578.8 ns)
   7257.1 |########################################
   7373.2 |
   7489.3 |########################################
   7605.3 |
   7721.4 |
   7837.5 |
   7953.6 |
   8069.7 |
   8185.8 |
   8301.8 |
   8417.9 |
   8534.0 |########################################
   8650.1 |########################################
   8766.2 |
   8882.3 |
   8998.3 |
   9114.4 |
   9230.5 |
   9346.6 |########################################
   9462.7 |
  (0 below, 1 above range)

an_match_tree (n=6, range 7297.5-8861.2 ns)
   7297.5 |####################
   7375.7 |
   7453.9 |
   7532.1 |
   7610.2 |
   7688.4 |
   7766.6 |
   7844.8 |
   7923.0 |
   8001.2 |
   8079.4 |
   8157.6 |
   8235.8 |
   8313.9 |
   8392.1 |
   8470.3 |
   8548.5 |
   8626.7 |########################################
   8704.9 |########################################
   8783.1 |
  (0 below, 1 above range)

```
