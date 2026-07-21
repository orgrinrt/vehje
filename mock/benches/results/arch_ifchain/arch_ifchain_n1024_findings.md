# Per-type strategy: all ifchain branches one strategy, interp tier

5 variants, 6 samples per variant.
Baseline: **ab_ifchain_table**

## Highlights

Baseline for all deltas below: **ab_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ab_ifchain_pred is an outlier: 2.2x slower than the field

ab_ifchain_pred (725.76 us) is 2.2x the fastest (324.33 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ab_ifchain_prof shows alternating (throttle bounce) (autocorr -0.80)

ab_ifchain_prof's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ab_ifchain_table)

The baseline ab_ifchain_table is the fastest (324.33 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {ab_ifchain_table, ab_ifchain_tree, ab_ifchain_seq, ab_ifchain_prof} vs {ab_ifchain_pred} (118% apart)

The field splits into a fast tier {ab_ifchain_table, ab_ifchain_tree, ab_ifchain_seq, ab_ifchain_prof} and a slow tier {ab_ifchain_pred} with a 118% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (ab_ifchain_table) is the fastest** at 324326.2 ns median
- 3 variants significantly slower than baseline
- Spread: 2.24x (fastest 324326.2 ns, slowest 725760.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ab_ifchain_pred | 730299ns | 728757ns | 720415ns | 727560ns | 739349ns | +123.02% |
| ab_ifchain_prof | 338130ns | 335686ns | 330758ns | 334318ns | 347534ns | +3.26% |
| ab_ifchain_seq | 333918ns | 332924ns | 327009ns | 331996ns | 340257ns | +1.97% |
| ab_ifchain_table | 327459ns | 326820ns | 323977ns | 326333ns | 330888ns | base |
| ab_ifchain_tree | 329923ns | 330612ns | 326088ns | 329443ns | 332560ns | +0.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ab_ifchain_pred | 727515ns | 717418ns | 736988ns | +123.84% | 0.001 |
| ab_ifchain_prof | 335497ns | 328401ns | 344722ns | +3.22% | 0.003 |
| ab_ifchain_seq | 331396ns | 324447ns | 337902ns | +1.96% | 0.003 |
| ab_ifchain_table | 325016ns | 321518ns | 328630ns | base | 0.003 |
| ab_ifchain_tree | 327385ns | 323885ns | 329904ns | +0.73% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (ab_ifchain_table; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ab_ifchain_pred | 0.001 | 44.3% |
| ab_ifchain_prof | 0.003 | 96.5% |
| ab_ifchain_seq | 0.003 | 97.3% |
| ab_ifchain_table | 0.003 | 99.1% |
| ab_ifchain_tree | 0.003 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ab_ifchain_pred | 730299ns | 730299ns | +123.02% |
| ab_ifchain_prof | 338130ns | 338130ns | +3.26% |
| ab_ifchain_seq | 333918ns | 333918ns | +1.97% |
| ab_ifchain_table | 327459ns | 327459ns | base |
| ab_ifchain_tree | 329923ns | 329923ns | +0.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ab_ifchain_table | 324326ns | base | --- | [322091, 328630] | --- | --- | --- | --- |
| ab_ifchain_pred | 725761ns | +402454.6ns (+124.1%) | [+396684, +408359]ns | [719796, 736988] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_ifchain_prof | 333132ns | +7713.8ns (+2.4%) | [+2316, +21416]ns | [328638, 344722] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| ab_ifchain_seq | 330320ns | +6998.3ns (+2.2%) | [+2853, +9288]ns | [325964, 337902] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| ab_ifchain_tree | 328239ns | no significant difference | [-3080, +7308]ns | [324011, 329904] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ab_ifchain_table | ab_ifchain_pred | ab_ifchain_prof | ab_ifchain_seq | ab_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 323558ns | +121.7% | +1.5% | +1.2% | +1.9% |
| 2 | 321518ns | +126.0% | +7.5% | +2.7% | +2.6% |
| 3 | 329087ns | +123.3% | -0.1% | +2.7% | -1.6% |
| 4 | 325095ns | +123.0% | +5.8% | +1.6% | -0.3% |
| 5 | 322665ns | +123.8% | +2.7% | +0.6% | +1.3% |
| 6 | 328172ns | +125.2% | +2.0% | +3.0% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ab_ifchain_pred | -0.198 | ok |
| ab_ifchain_prof | -0.796 | HIGH- (thermal bounce) |
| ab_ifchain_seq | -0.318 | moderate- |
| ab_ifchain_table | -0.354 | moderate- |
| ab_ifchain_tree | 0.215 | moderate+ |

**Consistency summary:**

- **ab_ifchain_pred**: won 0/6, lost 6/6
- **ab_ifchain_prof**: won 0/6, lost 5/6
- **ab_ifchain_seq**: won 0/6, lost 6/6
- **ab_ifchain_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ab_ifchain_pred | 12.4ns | 727514.9ns | 0.0% |  |
| ab_ifchain_prof | 5.7ns | 335497.4ns | 0.0% |  |
| ab_ifchain_seq | 3.9ns | 331395.5ns | 0.0% |  |
| ab_ifchain_table | 6.4ns | 325015.7ns | 0.0% |  |
| ab_ifchain_tree | 6.4ns | 327384.8ns | 0.0% |  |

## Distribution (algo ns)

```
ab_ifchain_pred (n=6, range 717417.9-736988.1 ns)
  717417.9 |########################################
  718396.4 |
  719374.9 |
  720353.4 |
  721331.9 |########################################
  722310.5 |
  723289.0 |
  724267.5 |########################################
  725246.0 |
  726224.5 |########################################
  727203.0 |
  728181.5 |
  729160.0 |
  730138.5 |
  731117.0 |
  732095.6 |
  733074.1 |
  734052.6 |########################################
  735031.1 |
  736009.6 |
  (0 below, 1 above range)

ab_ifchain_prof (n=6, range 328401.2-344721.7 ns)
  328401.2 |########################################
  329217.2 |
  330033.2 |
  330849.3 |####################
  331665.3 |
  332481.3 |
  333297.3 |
  334113.4 |####################
  334929.4 |
  335745.4 |
  336561.4 |
  337377.5 |
  338193.5 |
  339009.5 |
  339825.5 |
  340641.6 |
  341457.6 |
  342273.6 |
  343089.6 |
  343905.7 |####################
  (0 below, 1 above range)

ab_ifchain_seq (n=6, range 324447.1-337902.2 ns)
  324447.1 |####################
  325119.9 |
  325792.6 |
  326465.4 |
  327138.1 |####################
  327810.9 |
  328483.6 |
  329156.4 |
  329829.2 |########################################
  330501.9 |
  331174.7 |
  331847.4 |
  332520.2 |
  333192.9 |
  333865.7 |
  334538.5 |
  335211.2 |
  335884.0 |
  336556.7 |
  337229.5 |####################
  (0 below, 1 above range)

ab_ifchain_table (n=6, range 321517.5-328629.6 ns)
  321517.5 |########################################
  321873.1 |
  322228.7 |
  322584.3 |########################################
  322939.9 |
  323295.5 |########################################
  323651.1 |
  324006.7 |
  324362.3 |
  324717.9 |
  325073.5 |########################################
  325429.2 |
  325784.8 |
  326140.4 |
  326496.0 |
  326851.6 |
  327207.2 |
  327562.8 |
  327918.4 |########################################
  328274.0 |
  (0 below, 1 above range)

ab_ifchain_tree (n=6, range 323884.6-329904.2 ns)
  323884.6 |########################################
  324185.6 |
  324486.6 |
  324787.5 |
  325088.5 |
  325389.5 |
  325690.5 |
  325991.5 |
  326292.4 |
  326593.4 |####################
  326894.4 |
  327195.4 |
  327496.4 |
  327797.3 |
  328098.3 |
  328399.3 |
  328700.3 |
  329001.3 |
  329302.2 |
  329603.2 |########################################
  (0 below, 1 above range)

```
