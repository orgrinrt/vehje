# Whole-program single strategy (NATIVE tier)

5 variants, 6 samples per variant.
Baseline: **an_whole_table**

## Highlights

Baseline for all deltas below: **an_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_whole_pred is an outlier: 2.1x slower than the field

an_whole_pred (15.72 us) is 2.1x the fastest (7.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {an_whole_seq, an_whole_prof, an_whole_tree, an_whole_table} vs {an_whole_pred} (83% apart)

The field splits into a fast tier {an_whole_seq, an_whole_prof, an_whole_tree, an_whole_table} and a slow tier {an_whole_pred} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: an_whole_seq** at 7619.6 ns median (-11.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.06x (fastest 7619.6 ns, slowest 15719.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_whole_pred | 17484ns | 18338ns | 15369ns | 17386ns | 18690ns | +57.12% |
| an_whole_prof | 10074ns | 10497ns | 8698ns | 9936ns | 10968ns | -9.48% |
| an_whole_seq | 10058ns | 10115ns | 8887ns | 9812ns | 11012ns | -9.62% |
| an_whole_table | 11128ns | 11062ns | 9985ns | 10858ns | 12106ns | base |
| an_whole_tree | 10760ns | 10922ns | 9490ns | 10590ns | 11651ns | -3.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_whole_pred | 14990ns | 13217ns | 15979ns | +74.40% | 0.068 |
| an_whole_prof | 7623ns | 6575ns | 8323ns | -11.31% | 0.134 |
| an_whole_seq | 7590ns | 6719ns | 8316ns | -11.69% | 0.135 |
| an_whole_table | 8595ns | 7712ns | 9337ns | base | 0.119 |
| an_whole_tree | 8311ns | 7337ns | 8994ns | -3.31% | 0.123 |

## Performance model

- Peak throughput: **0.156 Gops/s** (an_whole_prof; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_whole_pred | 0.065 | 41.8% |
| an_whole_prof | 0.129 | 82.9% |
| an_whole_seq | 0.134 | 86.3% |
| an_whole_table | 0.120 | 76.7% |
| an_whole_tree | 0.122 | 78.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_whole_pred | 17484ns | 17484ns | +57.12% |
| an_whole_prof | 10074ns | 10074ns | -9.48% |
| an_whole_seq | 10058ns | 10058ns | -9.62% |
| an_whole_table | 11128ns | 11128ns | base |
| an_whole_tree | 10760ns | 10760ns | -3.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_whole_table | 8568ns | base | --- | [7881, 9337] | --- | --- | --- | --- |
| an_whole_pred | 15719ns | +6575.3ns (+76.7%) | [+5298, +7311]ns | [13272, 15979] | YES | 0.0417 | 0.0313 | 0 |
| an_whole_prof | 7930ns | -1229.5ns (-14.4%) | [-1534, -152]ns | [6618, 8323] | YES | 0.0417 | 0.0313 | 0 |
| an_whole_seq | 7620ns | -911.3ns (-10.6%) | [-1385, -718]ns | [6835, 8316] | YES | 0.0417 | 0.0313 | 0 |
| an_whole_tree | 8425ns | no significant difference | [-582, +185]ns | [7514, 8994] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_whole_table | an_whole_pred | an_whole_prof | an_whole_seq | an_whole_tree |
|---|---|---|---|---|---|
| 1 | 8236ns | +60.5% | -20.2% | -15.6% | -6.6% |
| 2 | 7712ns | +72.8% | -13.6% | -12.9% | -4.9% |
| 3 | 8899ns | +76.9% | -2.5% | -9.3% | +3.4% |
| 4 | 9321ns | +73.0% | -15.1% | -8.1% | -5.7% |
| 5 | 8050ns | +96.6% | -1.0% | -8.4% | +0.8% |
| 6 | 9353ns | +67.8% | -15.0% | -15.9% | -6.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_whole_pred | 0.457 | moderate+ |
| an_whole_prof | 0.150 | ok |
| an_whole_seq | 0.135 | ok |
| an_whole_table | -0.225 | moderate- |
| an_whole_tree | -0.006 | ok |

**Consistency summary:**

- **an_whole_pred**: won 0/6, lost 6/6
- **an_whole_prof**: won 6/6, lost 0/6
- **an_whole_seq**: won 6/6, lost 0/6
- **an_whole_tree**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_whole_pred | 5.6ns | 14989.9ns | 0.0% |  |
| an_whole_prof | 5.4ns | 7623.5ns | 0.1% |  |
| an_whole_seq | 5.5ns | 7590.4ns | 0.1% |  |
| an_whole_table | 5.8ns | 8595.3ns | 0.1% |  |
| an_whole_tree | 5.7ns | 8311.1ns | 0.1% |  |

## Distribution (algo ns)

```
an_whole_pred (n=6, range 13217.1-15978.8 ns)
  13217.1 |########################################
  13355.2 |
  13493.3 |
  13631.3 |
  13769.4 |
  13907.5 |
  14045.6 |
  14183.7 |
  14321.8 |
  14459.8 |
  14597.9 |
  14736.0 |
  14874.1 |
  15012.2 |
  15150.3 |
  15288.3 |
  15426.4 |
  15564.5 |####################
  15702.6 |########################################
  15840.7 |
  (0 below, 1 above range)

an_whole_prof (n=6, range 6575.4-8323.1 ns)
   6575.4 |##########################
   6662.8 |
   6750.2 |
   6837.6 |
   6924.9 |
   7012.3 |
   7099.7 |
   7187.1 |
   7274.5 |
   7361.9 |
   7449.2 |
   7536.6 |
   7624.0 |
   7711.4 |
   7798.8 |
   7886.2 |########################################
   7973.6 |
   8060.9 |
   8148.3 |
   8235.7 |
  (0 below, 1 above range)

an_whole_seq (n=6, range 6718.8-8316.2 ns)
   6718.8 |########################################
   6798.7 |
   6878.5 |########################################
   6958.4 |
   7038.3 |
   7118.2 |
   7198.0 |
   7277.9 |
   7357.8 |########################################
   7437.7 |
   7517.5 |
   7597.4 |
   7677.3 |
   7757.1 |
   7837.0 |########################################
   7916.9 |
   7996.8 |########################################
   8076.6 |
   8156.5 |
   8236.4 |
  (0 below, 1 above range)

an_whole_table (n=6, range 7712.5-9337.0 ns)
   7712.5 |########################################
   7793.7 |
   7875.0 |
   7956.2 |
   8037.4 |########################################
   8118.6 |
   8199.9 |########################################
   8281.1 |
   8362.3 |
   8443.5 |
   8524.8 |
   8606.0 |
   8687.2 |
   8768.5 |
   8849.7 |########################################
   8930.9 |
   9012.1 |
   9093.4 |
   9174.6 |
   9255.8 |########################################
  (0 below, 1 above range)

an_whole_tree (n=6, range 7336.7-8993.8 ns)
   7336.7 |########################################
   7419.6 |
   7502.4 |
   7585.3 |
   7668.1 |########################################
   7751.0 |
   7833.8 |
   7916.7 |
   7999.5 |
   8082.4 |########################################
   8165.2 |
   8248.1 |
   8330.9 |
   8413.8 |
   8496.6 |
   8579.5 |
   8662.3 |########################################
   8745.2 |########################################
   8828.0 |
   8910.9 |
  (0 below, 1 above range)

```
