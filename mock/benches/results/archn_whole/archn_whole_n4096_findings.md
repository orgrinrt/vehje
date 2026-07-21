# Whole-program single strategy (NATIVE tier)

5 variants, 6 samples per variant.
Baseline: **an_whole_table**

## Highlights

Baseline for all deltas below: **an_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_whole_tree is fastest but the noisiest (CV 15.0%)

an_whole_tree wins on median (45.88 us) yet has the highest variance (CV 15.0%), while an_whole_prof is the steadiest (CV 5.6%, 49.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader an_whole_tree vs stability leader an_whole_prof (+7% speed for 2.7x steadier)

an_whole_tree is fastest (45.88 us, CV 15.0%); an_whole_prof gives up 7.0% median for 2.7x lower variance (CV 5.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: an_whole_tree** at 45880.4 ns median (-15.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.47x (fastest 45880.4 ns, slowest 67612.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_whole_pred | 69540ns | 70074ns | 61765ns | 68962ns | 74294ns | +22.13% |
| an_whole_prof | 51847ns | 51352ns | 48133ns | 50797ns | 55280ns | -8.94% |
| an_whole_seq | 52868ns | 52524ns | 43650ns | 51273ns | 59869ns | -7.15% |
| an_whole_table | 56939ns | 56918ns | 50968ns | 55643ns | 61868ns | base |
| an_whole_tree | 50048ns | 48335ns | 41263ns | 47179ns | 58745ns | -12.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_whole_pred | 67079ns | 59471ns | 71749ns | +23.09% | 0.061 |
| an_whole_prof | 49496ns | 45863ns | 52766ns | -9.17% | 0.083 |
| an_whole_seq | 50460ns | 41172ns | 57304ns | -7.40% | 0.081 |
| an_whole_table | 54495ns | 48636ns | 59283ns | base | 0.075 |
| an_whole_tree | 47688ns | 39035ns | 56376ns | -12.49% | 0.086 |

## Performance model

- Peak throughput: **0.105 Gops/s** (an_whole_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_whole_pred | 0.061 | 57.7% |
| an_whole_prof | 0.083 | 79.5% |
| an_whole_seq | 0.082 | 77.8% |
| an_whole_table | 0.075 | 71.6% |
| an_whole_tree | 0.089 | 85.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_whole_pred | 69540ns | 69540ns | +22.13% |
| an_whole_prof | 51847ns | 51847ns | -8.94% |
| an_whole_seq | 52868ns | 52868ns | -7.15% |
| an_whole_table | 56939ns | 56939ns | base |
| an_whole_tree | 50048ns | 50048ns | -12.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_whole_table | 54524ns | base | --- | [49676, 59283] | --- | --- | --- | --- |
| an_whole_pred | 67612ns | +12494.0ns (+22.9%) | [+9912, +15347]ns | [61876, 71749] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_whole_prof | 49071ns | -3583.1ns (-6.6%) | [-10049, -1363]ns | [46651, 52766] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_whole_seq | 50180ns | no significant difference | [-12272, +5145]ns | [43895, 57304] | no | 0.6875 | 0.6875 | 0 |
| an_whole_tree | 45880ns | no significant difference | [-11829, +651]ns | [40807, 56376] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_whole_table | an_whole_pred | an_whole_prof | an_whole_seq | an_whole_tree |
|---|---|---|---|---|---|
| 1 | 48636ns | +32.2% | -5.7% | +17.7% | -6.0% |
| 2 | 55681ns | +23.5% | -7.9% | +3.0% | +7.6% |
| 3 | 54492ns | +27.6% | -0.5% | -24.4% | -15.5% |
| 4 | 62885ns | +17.6% | -20.6% | -17.8% | -16.0% |
| 5 | 54556ns | +21.8% | -13.0% | -14.6% | -22.0% |
| 6 | 50717ns | +17.3% | -4.9% | -4.0% | -23.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_whole_pred | 0.138 | ok |
| an_whole_prof | 0.126 | ok |
| an_whole_seq | -0.133 | ok |
| an_whole_table | -0.056 | ok |
| an_whole_tree | -0.122 | ok |

**Consistency summary:**

- **an_whole_pred**: won 0/6, lost 6/6
- **an_whole_prof**: won 6/6, lost 0/6
- **an_whole_seq**: won 4/6, lost 2/6
- **an_whole_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_whole_pred | 9.9ns | 67079.2ns | 0.0% |  |
| an_whole_prof | 5.7ns | 49496.2ns | 0.0% |  |
| an_whole_seq | 5.9ns | 50459.5ns | 0.0% |  |
| an_whole_table | 5.2ns | 54494.6ns | 0.0% |  |
| an_whole_tree | 5.4ns | 47687.9ns | 0.0% |  |

## Distribution (algo ns)

```
an_whole_pred (n=6, range 59471.2-71749.1 ns)
  59471.2 |########################################
  60085.1 |
  60699.0 |
  61312.9 |
  61926.8 |
  62540.7 |
  63154.6 |
  63768.5 |########################################
  64382.4 |
  64996.3 |
  65610.2 |
  66224.1 |########################################
  66838.0 |
  67451.9 |
  68065.8 |
  68679.7 |########################################
  69293.6 |########################################
  69907.5 |
  70521.4 |
  71135.3 |
  (0 below, 1 above range)

an_whole_prof (n=6, range 45862.9-52766.2 ns)
  45862.9 |########################################
  46208.1 |
  46553.2 |
  46898.4 |
  47243.6 |########################################
  47588.7 |
  47933.9 |########################################
  48279.1 |
  48624.2 |
  48969.4 |
  49314.6 |
  49659.7 |########################################
  50004.9 |
  50350.1 |
  50695.2 |
  51040.4 |########################################
  51385.6 |
  51730.7 |
  52075.9 |
  52421.1 |
  (0 below, 1 above range)

an_whole_seq (n=6, range 41172.5-57303.6 ns)
  41172.5 |########################################
  41979.1 |
  42785.6 |
  43592.2 |
  44398.7 |
  45205.3 |
  46011.8 |########################################
  46818.4 |
  47624.9 |
  48431.5 |########################################
  49238.0 |
  50044.6 |
  50851.1 |
  51657.7 |########################################
  52464.2 |
  53270.8 |
  54077.3 |
  54883.9 |
  55690.4 |
  56497.0 |########################################
  (0 below, 1 above range)

an_whole_table (n=6, range 48636.2-59282.9 ns)
  48636.2 |####################
  49168.5 |
  49700.9 |
  50233.2 |####################
  50765.5 |
  51297.9 |
  51830.2 |
  52362.5 |
  52894.9 |
  53427.2 |
  53959.6 |
  54491.9 |########################################
  55024.2 |
  55556.6 |####################
  56088.9 |
  56621.2 |
  57153.6 |
  57685.9 |
  58218.2 |
  58750.6 |
  (0 below, 1 above range)

an_whole_tree (n=6, range 39035.4-56375.8 ns)
  39035.4 |########################################
  39902.4 |
  40769.4 |
  41636.5 |
  42503.5 |########################################
  43370.5 |
  44237.5 |
  45104.6 |########################################
  45971.6 |########################################
  46838.6 |
  47705.6 |
  48572.6 |
  49439.7 |
  50306.7 |
  51173.7 |
  52040.7 |########################################
  52907.8 |
  53774.8 |
  54641.8 |
  55508.8 |
  (0 below, 1 above range)

```
