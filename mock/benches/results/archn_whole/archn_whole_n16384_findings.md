# Whole-program single strategy (NATIVE tier)

5 variants, 6 samples per variant.
Baseline: **an_whole_table**

## Highlights

Baseline for all deltas below: **an_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (an_whole_prof, an_whole_seq) are a dead heat (<1%)

an_whole_prof (311.51 us) and an_whole_seq (311.96 us) differ by 0.14%, inside the noise, even though the wider field spreads 6.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### an_whole_seq shows alternating (throttle bounce) (autocorr -0.56)

an_whole_seq's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: an_whole_prof** at 311513.5 ns median (-4.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.07x (fastest 311513.5 ns, slowest 331783.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_whole_pred | 323089ns | 319680ns | 316814ns | 319543ns | 331545ns | -1.91% |
| an_whole_prof | 314466ns | 313940ns | 306625ns | 311633ns | 322635ns | -4.53% |
| an_whole_seq | 319179ns | 314466ns | 304975ns | 313916ns | 334175ns | -3.10% |
| an_whole_table | 329384ns | 328228ns | 324740ns | 327437ns | 334626ns | base |
| an_whole_tree | 335326ns | 334244ns | 330658ns | 333372ns | 340590ns | +1.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_whole_pred | 320627ns | 314293ns | 328897ns | -1.91% | 0.051 |
| an_whole_prof | 311941ns | 303968ns | 320159ns | -4.57% | 0.053 |
| an_whole_seq | 316726ns | 302678ns | 331585ns | -3.10% | 0.052 |
| an_whole_table | 326870ns | 322118ns | 331904ns | base | 0.050 |
| an_whole_tree | 332785ns | 328200ns | 337952ns | +1.81% | 0.049 |

## Performance model

- Peak throughput: **0.054 Gops/s** (an_whole_seq; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_whole_pred | 0.052 | 95.4% |
| an_whole_prof | 0.053 | 97.2% |
| an_whole_seq | 0.053 | 97.0% |
| an_whole_table | 0.050 | 92.9% |
| an_whole_tree | 0.049 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_whole_pred | 323089ns | 323089ns | -1.91% |
| an_whole_prof | 314466ns | 314466ns | -4.53% |
| an_whole_seq | 319179ns | 319179ns | -3.10% |
| an_whole_table | 329384ns | 329384ns | base |
| an_whole_tree | 335326ns | 335326ns | +1.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_whole_table | 325874ns | base | --- | [322832, 331904] | --- | --- | --- | --- |
| an_whole_pred | 317408ns | -7682.3ns (-2.4%) | [-9459, -1587]ns | [315578, 328897] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| an_whole_prof | 311514ns | -14360.5ns (-4.4%) | [-24612, -5815]ns | [304150, 320159] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_whole_seq | 311956ns | no significant difference | [-19044, +1934]ns | [306638, 331585] | no | 0.2917 | 0.2188 | 0 |
| an_whole_tree | 331784ns | no significant difference | [-1455, +13502]ns | [328618, 337952] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_whole_table | an_whole_pred | an_whole_prof | an_whole_seq | an_whole_tree |
|---|---|---|---|---|---|
| 1 | 323547ns | -2.9% | -1.0% | -3.3% | +2.3% |
| 2 | 335407ns | -2.4% | -9.4% | -4.5% | -0.8% |
| 3 | 324965ns | -2.2% | -5.2% | -6.9% | +1.3% |
| 4 | 328401ns | +0.6% | -2.6% | +4.4% | -0.1% |
| 5 | 326782ns | -3.0% | -3.6% | -4.8% | +4.0% |
| 6 | 322118ns | -1.6% | -5.5% | -3.6% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_whole_pred | -0.517 | HIGH- (thermal bounce) |
| an_whole_prof | -0.237 | moderate- |
| an_whole_seq | -0.558 | HIGH- (thermal bounce) |
| an_whole_table | -0.420 | moderate- |
| an_whole_tree | 0.089 | ok |

**Consistency summary:**

- **an_whole_pred**: won 5/6, lost 1/6
- **an_whole_prof**: won 6/6, lost 0/6
- **an_whole_seq**: won 5/6, lost 1/6
- **an_whole_tree**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_whole_pred | 6.0ns | 320627.4ns | 0.0% |  |
| an_whole_prof | 7.2ns | 311940.8ns | 0.0% |  |
| an_whole_seq | 15.6ns | 316726.2ns | 0.0% |  |
| an_whole_table | 11.7ns | 326870.1ns | 0.0% |  |
| an_whole_tree | 8.3ns | 332784.7ns | 0.0% |  |

## Distribution (algo ns)

```
an_whole_pred (n=6, range 314293.3-328896.8 ns)
  314293.3 |####################
  315023.5 |
  315753.7 |
  316483.8 |########################################
  317214.0 |####################
  317944.2 |
  318674.4 |
  319404.5 |
  320134.7 |
  320864.9 |
  321595.1 |
  322325.3 |
  323055.4 |
  323785.6 |
  324515.8 |
  325246.0 |
  325976.1 |
  326706.3 |####################
  327436.5 |
  328166.7 |
  (0 below, 1 above range)

an_whole_prof (n=6, range 303967.5-320158.8 ns)
  303967.5 |########################################
  304777.1 |
  305586.6 |
  306396.2 |
  307205.8 |
  308015.3 |####################
  308824.9 |
  309634.4 |
  310444.0 |
  311253.6 |
  312063.1 |
  312872.7 |
  313682.2 |
  314491.8 |####################
  315301.4 |
  316110.9 |
  316920.5 |
  317730.1 |
  318539.6 |
  319349.2 |####################
  (0 below, 1 above range)

an_whole_seq (n=6, range 302677.5-331585.4 ns)
  302677.5 |####################
  304122.9 |
  305568.3 |
  307013.7 |
  308459.1 |
  309904.5 |########################################
  311349.9 |
  312795.3 |####################
  314240.7 |
  315686.1 |
  317131.5 |
  318576.8 |
  320022.2 |####################
  321467.6 |
  322913.0 |
  324358.4 |
  325803.8 |
  327249.2 |
  328694.6 |
  330140.0 |
  (0 below, 1 above range)

an_whole_table (n=6, range 322117.9-331903.9 ns)
  322117.9 |########################################
  322607.2 |
  323096.5 |########################################
  323585.8 |
  324075.1 |
  324564.4 |########################################
  325053.7 |
  325543.0 |
  326032.3 |
  326521.6 |########################################
  327010.9 |
  327500.2 |
  327989.5 |########################################
  328478.8 |
  328968.1 |
  329457.4 |
  329946.7 |
  330436.0 |
  330925.3 |
  331414.6 |
  (0 below, 1 above range)

an_whole_tree (n=6, range 328199.6-337952.1 ns)
  328199.6 |########################################
  328687.2 |########################################
  329174.8 |
  329662.5 |
  330150.1 |
  330637.7 |########################################
  331125.3 |
  331613.0 |
  332100.6 |
  332588.2 |########################################
  333075.8 |
  333563.5 |
  334051.1 |
  334538.7 |
  335026.3 |
  335514.0 |
  336001.6 |########################################
  336489.2 |
  336976.8 |
  337464.5 |
  (0 below, 1 above range)

```
