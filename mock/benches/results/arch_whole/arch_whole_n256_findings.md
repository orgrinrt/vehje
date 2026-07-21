# Whole-program single strategy (all branches), interp tier

5 variants, 6 samples per variant.
Baseline: **ab_whole_table**

## Highlights

Baseline for all deltas below: **ab_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_whole_pred is an outlier: 4.0x slower than the field

ab_whole_pred (337.01 us) is 4.0x the fastest (83.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ab_whole_tree is fastest but the noisiest (CV 5.4%)

ab_whole_tree wins on median (83.75 us) yet has the highest variance (CV 5.4%), while ab_whole_pred is the steadiest (CV 1.6%, 337.01 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (ab_whole_tree, ab_whole_table) are a dead heat (<1%)

ab_whole_tree (83.75 us) and ab_whole_table (83.80 us) differ by 0.06%, inside the noise, even though the wider field spreads 302.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {ab_whole_tree, ab_whole_table, ab_whole_prof, ab_whole_seq} vs {ab_whole_pred} (264% apart)

The field splits into a fast tier {ab_whole_tree, ab_whole_table, ab_whole_prof, ab_whole_seq} and a slow tier {ab_whole_pred} with a 264% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.0x the fastest

Fastest ab_whole_tree (83.75 us) to slowest ab_whole_pred (337.01 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ab_whole_tree** at 83747.8 ns median (-0.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.02x (fastest 83747.8 ns, slowest 337005.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_whole_pred | 337947ns | 339460ns | 327853ns | 337834ns | 343163ns | +287.75% |
| ab_whole_prof | 87913ns | 86374ns | 84861ns | 86139ns | 92101ns | +0.87% |
| ab_whole_seq | 93498ns | 95268ns | 86592ns | 92584ns | 98321ns | +7.28% |
| ab_whole_table | 87156ns | 86097ns | 82392ns | 85963ns | 91328ns | base |
| ab_whole_tree | 88048ns | 86080ns | 83566ns | 85267ns | 94461ns | +1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_whole_pred | 335431ns | 325162ns | 340539ns | +295.34% | 0.001 |
| ab_whole_prof | 85603ns | 82663ns | 89695ns | +0.89% | 0.003 |
| ab_whole_seq | 91026ns | 84342ns | 95700ns | +7.28% | 0.003 |
| ab_whole_table | 84846ns | 80235ns | 88888ns | base | 0.003 |
| ab_whole_tree | 85693ns | 81412ns | 91914ns | +1.00% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_whole_table; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_whole_pred | 0.001 | 23.8% |
| ab_whole_prof | 0.003 | 95.4% |
| ab_whole_seq | 0.003 | 86.5% |
| ab_whole_table | 0.003 | 95.7% |
| ab_whole_tree | 0.003 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_whole_pred | 337947ns | 337947ns | +287.75% |
| ab_whole_prof | 87913ns | 87913ns | +0.87% |
| ab_whole_seq | 93498ns | 93498ns | +7.28% |
| ab_whole_table | 87156ns | 87156ns | base |
| ab_whole_tree | 88048ns | 88048ns | +1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_whole_table | 83800ns | base | --- | [81849, 88888] | --- | --- | --- | --- |
| ab_whole_pred | 337005ns | +249647.5ns (+297.9%) | [+245370, +256739]ns | [328749, 340539] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_whole_prof | 84071ns | no significant difference | [-142, +2062]ns | [83043, 89695] | no | 1.0000 | 1.0000 | 0 |
| ab_whole_seq | 92705ns | +5295.9ns (+6.3%) | [+603, +12641]ns | [84672, 95700] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_whole_tree | 83748ns | no significant difference | [-2128, +5738]ns | [81416, 91914] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_whole_table | ab_whole_pred | ab_whole_prof | ab_whole_seq | ab_whole_tree |
|---|---|---|---|---|---|
| 1 | 83974ns | +306.5% | -0.1% | +8.2% | +8.5% |
| 2 | 83463ns | +301.5% | +0.9% | +1.1% | -2.4% |
| 3 | 80235ns | +305.3% | +3.0% | +17.9% | +5.4% |
| 4 | 93101ns | +264.0% | -0.1% | +3.9% | -0.4% |
| 5 | 84675ns | +292.5% | +2.0% | +0.4% | -2.1% |
| 6 | 83625ns | +306.2% | -0.2% | +13.0% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_whole_pred | -0.322 | moderate- |
| ab_whole_prof | -0.156 | ok |
| ab_whole_seq | -0.418 | moderate- |
| ab_whole_table | -0.338 | moderate- |
| ab_whole_tree | -0.272 | moderate- |

**Consistency summary:**

- **ab_whole_pred**: won 0/6, lost 6/6
- **ab_whole_prof**: won 1/6, lost 3/6
- **ab_whole_seq**: won 0/6, lost 6/6
- **ab_whole_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_whole_pred | 8.4ns | 335431.0ns | 0.0% |  |
| ab_whole_prof | 5.6ns | 85603.1ns | 0.0% |  |
| ab_whole_seq | 6.7ns | 91025.6ns | 0.0% |  |
| ab_whole_table | 4.3ns | 84845.5ns | 0.0% |  |
| ab_whole_tree | 4.3ns | 85692.7ns | 0.0% |  |

## Distribution (algo ns)

```
ab_whole_pred (n=6, range 325162.1-340538.9 ns)
  325162.1 |########################################
  325930.9 |
  326699.8 |
  327468.6 |
  328237.5 |
  329006.3 |
  329775.2 |
  330544.0 |
  331312.8 |
  332081.7 |########################################
  332850.5 |
  333619.4 |
  334388.2 |########################################
  335157.1 |
  335925.9 |
  336694.7 |
  337463.6 |
  338232.4 |########################################
  339001.3 |########################################
  339770.1 |
  (0 below, 1 above range)

ab_whole_prof (n=6, range 82663.3-89695.0 ns)
  82663.3 |########################################
  83014.9 |
  83366.5 |########################################
  83718.1 |########################################
  84069.6 |########################################
  84421.2 |
  84772.8 |
  85124.4 |
  85476.0 |
  85827.6 |
  86179.1 |########################################
  86530.7 |
  86882.3 |
  87233.9 |
  87585.5 |
  87937.1 |
  88288.7 |
  88640.2 |
  88991.8 |
  89343.4 |
  (0 below, 1 above range)

ab_whole_seq (n=6, range 84341.7-95699.6 ns)
  84341.7 |########################################
  84909.6 |########################################
  85477.5 |
  86045.4 |
  86613.3 |
  87181.2 |
  87749.1 |
  88316.9 |
  88884.8 |
  89452.7 |
  90020.6 |
  90588.5 |########################################
  91156.4 |
  91724.3 |
  92292.2 |
  92860.1 |
  93428.0 |
  93995.9 |########################################
  94563.8 |########################################
  95131.7 |
  (0 below, 1 above range)

ab_whole_table (n=6, range 80235.0-88888.1 ns)
  80235.0 |####################
  80667.7 |
  81100.3 |
  81533.0 |
  81965.6 |
  82398.3 |
  82830.9 |
  83263.6 |########################################
  83696.2 |####################
  84128.9 |
  84561.6 |####################
  84994.2 |
  85426.9 |
  85859.5 |
  86292.2 |
  86724.8 |
  87157.5 |
  87590.1 |
  88022.8 |
  88455.4 |
  (0 below, 1 above range)

ab_whole_tree (n=6, range 81411.7-91913.9 ns)
  81411.7 |########################################
  81936.8 |
  82461.9 |####################
  82987.0 |
  83512.1 |
  84037.3 |
  84562.4 |####################
  85087.5 |
  85612.6 |
  86137.7 |
  86662.8 |
  87187.9 |
  87713.1 |
  88238.2 |
  88763.3 |
  89288.4 |
  89813.5 |
  90338.6 |
  90863.7 |####################
  91388.8 |
  (0 below, 1 above range)

```
