# Per-branch strategy (NATIVE tier): archetype 4

5 variants, 6 samples per variant.
Baseline: **an_b4_table**

## Highlights

Baseline for all deltas below: **an_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_b4_table, an_b4_tree) are a dead heat (<1%)

an_b4_table (330.25 us) and an_b4_tree (331.78 us) differ by 0.46%, inside the noise, even though the wider field spreads 9.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_b4_tree shows alternating (throttle bounce) (autocorr -0.75)

an_b4_tree's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (an_b4_table)

The baseline an_b4_table is the fastest (330.25 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### an_b4_tree's edge over baseline is significant but tiny (-30 ns, 0.01%)

an_b4_tree differs from baseline an_b4_table by -30 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b4_table) is the fastest** at 330251.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.09x (fastest 330251.8 ns, slowest 360070.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b4_pred | 362301ns | 362756ns | 359191ns | 362000ns | 364307ns | +8.25% |
| an_b4_prof | 340911ns | 340385ns | 331459ns | 339037ns | 348448ns | +1.86% |
| an_b4_seq | 344679ns | 346455ns | 330285ns | 343040ns | 354334ns | +2.99% |
| an_b4_table | 334677ns | 332889ns | 329520ns | 332093ns | 341133ns | base |
| an_b4_tree | 334718ns | 334578ns | 329760ns | 333208ns | 339463ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b4_pred | 359642ns | 356418ns | 361621ns | +8.27% | 0.046 |
| an_b4_prof | 338278ns | 329097ns | 345728ns | +1.84% | 0.048 |
| an_b4_seq | 342056ns | 328095ns | 351650ns | +2.98% | 0.048 |
| an_b4_table | 332172ns | 327147ns | 338540ns | base | 0.049 |
| an_b4_tree | 332025ns | 327064ns | 336866ns | -0.04% | 0.049 |

## Performance model

- Peak throughput: **0.050 Gops/s** (an_b4_tree; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b4_pred | 0.046 | 90.8% |
| an_b4_prof | 0.049 | 96.9% |
| an_b4_seq | 0.048 | 95.1% |
| an_b4_table | 0.050 | 99.0% |
| an_b4_tree | 0.049 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b4_pred | 362301ns | 362301ns | +8.25% |
| an_b4_prof | 340911ns | 340911ns | +1.86% |
| an_b4_seq | 344679ns | 344679ns | +2.99% |
| an_b4_table | 334677ns | 334677ns | base |
| an_b4_tree | 334718ns | 334718ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b4_table | 330252ns | base | --- | [327726, 338540] | --- | --- | --- | --- |
| an_b4_pred | 360070ns | +29818.5ns (+9.0%) | [+18695, +33895]ns | [357235, 361621] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b4_prof | 337671ns | no significant difference | [-1764, +13025]ns | [331436, 345728] | no | 0.2917 | 0.2188 | 0 |
| an_b4_seq | 343763ns | +6491.2ns (+2.0%) | [+1672, +21486]ns | [330754, 351650] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_b4_tree | 331782ns | no significant difference | [-4058, +3646]ns | [327428, 336866] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b4_table | an_b4_pred | an_b4_prof | an_b4_seq | an_b4_tree |
|---|---|---|---|---|---|
| 1 | 328304ns | +10.3% | +0.2% | +6.8% | +0.6% |
| 2 | 339915ns | +5.3% | -1.3% | +0.7% | -2.0% |
| 3 | 328480ns | +9.7% | +3.4% | +1.5% | -0.4% |
| 4 | 337165ns | +5.7% | +2.2% | +2.4% | -0.2% |
| 5 | 327147ns | +10.4% | +2.0% | +0.3% | +0.2% |
| 6 | 332024ns | +8.4% | +4.5% | +6.3% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b4_pred | -0.522 | HIGH- (thermal bounce) |
| an_b4_prof | -0.163 | ok |
| an_b4_seq | -0.474 | moderate- |
| an_b4_table | -0.730 | HIGH- (thermal bounce) |
| an_b4_tree | -0.754 | HIGH- (thermal bounce) |

**Consistency summary:**

- **an_b4_pred**: won 0/6, lost 6/6
- **an_b4_prof**: won 1/6, lost 5/6
- **an_b4_seq**: won 0/6, lost 6/6
- **an_b4_tree**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b4_pred | 13.3ns | 359642.0ns | 0.0% |  |
| an_b4_prof | 14.0ns | 338278.1ns | 0.0% |  |
| an_b4_seq | 10.9ns | 342055.5ns | 0.0% |  |
| an_b4_table | 17.2ns | 332172.5ns | 0.0% |  |
| an_b4_tree | 11.7ns | 332025.2ns | 0.0% |  |

## Distribution (algo ns)

```
an_b4_pred (n=6, range 356417.9-361621.1 ns)
  356417.9 |########################################
  356678.1 |
  356938.2 |
  357198.4 |
  357458.5 |
  357718.7 |
  357978.8 |########################################
  358239.0 |
  358499.2 |
  358759.3 |
  359019.5 |
  359279.6 |
  359539.8 |
  359799.9 |########################################
  360060.1 |########################################
  360320.3 |
  360580.4 |
  360840.6 |########################################
  361100.7 |
  361360.9 |
  (0 below, 1 above range)

an_b4_prof (n=6, range 329096.7-345727.7 ns)
  329096.7 |########################################
  329928.2 |
  330759.8 |
  331591.4 |
  332422.9 |
  333254.5 |########################################
  334086.0 |
  334917.5 |########################################
  335749.1 |
  336580.7 |
  337412.2 |
  338243.8 |
  339075.3 |########################################
  339906.9 |
  340738.4 |
  341570.0 |
  342401.5 |
  343233.0 |
  344064.6 |########################################
  344896.2 |
  (0 below, 1 above range)

an_b4_seq (n=6, range 328095.0-351649.8 ns)
  328095.0 |########################################
  329272.7 |
  330450.5 |
  331628.2 |
  332806.0 |########################################
  333983.7 |
  335161.4 |
  336339.2 |
  337516.9 |
  338694.7 |
  339872.4 |
  341050.1 |
  342227.9 |########################################
  343405.6 |
  344583.4 |########################################
  345761.1 |
  346938.8 |
  348116.6 |
  349294.3 |
  350472.1 |########################################
  (0 below, 1 above range)

an_b4_table (n=6, range 327147.1-338540.0 ns)
  327147.1 |####################
  327716.7 |
  328286.4 |########################################
  328856.0 |
  329425.7 |
  329995.3 |
  330565.0 |
  331134.6 |
  331704.3 |####################
  332273.9 |
  332843.5 |
  333413.2 |
  333982.8 |
  334552.5 |
  335122.1 |
  335691.8 |
  336261.4 |
  336831.1 |####################
  337400.7 |
  337970.4 |
  (0 below, 1 above range)

an_b4_tree (n=6, range 327064.2-336866.5 ns)
  327064.2 |########################################
  327554.3 |########################################
  328044.4 |
  328534.5 |
  329024.7 |
  329514.8 |
  330004.9 |########################################
  330495.0 |
  330985.1 |
  331475.2 |
  331965.3 |
  332455.4 |
  332945.5 |########################################
  333435.7 |
  333925.8 |
  334415.9 |
  334906.0 |
  335396.1 |
  335886.2 |
  336376.3 |########################################
  (0 below, 1 above range)

```
