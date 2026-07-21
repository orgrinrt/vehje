# Whole-program single strategy (NATIVE tier)

5 variants, 6 samples per variant.
Baseline: **an_whole_table**

## Highlights

Baseline for all deltas below: **an_whole_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_whole_pred is an outlier: 2.0x slower than the field

an_whole_pred (1.06 us) is 2.0x the fastest (520 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {an_whole_prof, an_whole_seq, an_whole_table, an_whole_tree} vs {an_whole_pred} (80% apart)

The field splits into a fast tier {an_whole_prof, an_whole_seq, an_whole_table, an_whole_tree} and a slow tier {an_whole_pred} with a 80% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_whole_table is inconsistent: worst-20% is 1.7x its best-20%

an_whole_table's best 20% of batches run at 473 ns but its worst 20% at 799 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### an_whole_tree's edge over baseline is significant but tiny (5 ns, 0.88%)

an_whole_tree differs from baseline an_whole_table by 5 ns (0.88%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_whole_prof** at 520.2 ns median (-9.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.04x (fastest 520.2 ns, slowest 1062.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_whole_pred | 3571ns | 3749ns | 3024ns | 3532ns | 3904ns | +4.90% |
| an_whole_prof | 3042ns | 3126ns | 2592ns | 2979ns | 3362ns | -10.63% |
| an_whole_seq | 3308ns | 3183ns | 2590ns | 2986ns | 4152ns | -2.81% |
| an_whole_table | 3404ns | 3166ns | 2621ns | 2987ns | 4421ns | base |
| an_whole_tree | 3229ns | 3265ns | 2732ns | 3139ns | 3611ns | -5.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_whole_pred | 1006ns | 853ns | 1102ns | +63.13% | 0.064 |
| an_whole_prof | 512ns | 438ns | 569ns | -17.06% | 0.125 |
| an_whole_seq | 569ns | 437ns | 700ns | -7.69% | 0.112 |
| an_whole_table | 617ns | 473ns | 799ns | base | 0.104 |
| an_whole_tree | 581ns | 477ns | 664ns | -5.72% | 0.110 |

## Performance model

- Peak throughput: **0.146 Gops/s** (an_whole_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_whole_pred | 0.060 | 41.1% |
| an_whole_prof | 0.123 | 84.0% |
| an_whole_seq | 0.113 | 77.0% |
| an_whole_table | 0.111 | 76.0% |
| an_whole_tree | 0.108 | 73.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_whole_pred | 3571ns | 3571ns | +4.90% |
| an_whole_prof | 3042ns | 3042ns | -10.63% |
| an_whole_seq | 3308ns | 3308ns | -2.81% |
| an_whole_table | 3404ns | 3404ns | base |
| an_whole_tree | 3229ns | 3229ns | -5.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_whole_table | 575ns | base | --- | [476, 799] | --- | --- | --- | --- |
| an_whole_pred | 1063ns | +417.5ns (+72.6%) | [+241, +509]ns | [854, 1102] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_whole_prof | 520ns | -30.6ns (-5.3%) | [-279, -6]ns | [445, 569] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| an_whole_seq | 568ns | no significant difference | [-122, +16]ns | [440, 700] | no | 0.6875 | 0.6875 | 0 |
| an_whole_tree | 592ns | no significant difference | [-148, +37]ns | [489, 664] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_whole_table | an_whole_pred | an_whole_prof | an_whole_seq | an_whole_tree |
|---|---|---|---|---|---|
| 1 | 992ns | +10.9% | -46.8% | -20.2% | -29.8% |
| 2 | 479ns | +78.1% | -8.5% | -7.5% | +4.4% |
| 3 | 473ns | +80.7% | -4.3% | -7.6% | +0.9% |
| 4 | 572ns | +79.2% | -1.5% | -7.8% | -0.1% |
| 5 | 606ns | +81.4% | -15.5% | +0.2% | +1.0% |
| 6 | 577ns | +91.0% | -0.5% | +5.3% | +9.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_whole_pred | 0.225 | moderate+ |
| an_whole_prof | 0.005 | ok |
| an_whole_seq | -0.069 | ok |
| an_whole_table | -0.134 | ok |
| an_whole_tree | 0.038 | ok |

**Consistency summary:**

- **an_whole_pred**: won 0/6, lost 6/6
- **an_whole_prof**: won 6/6, lost 0/6
- **an_whole_seq**: won 4/6, lost 2/6
- **an_whole_tree**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_whole_pred | 6.2ns | 1006.1ns | 0.6% |  |
| an_whole_prof | 6.7ns | 511.5ns | 1.3% |  |
| an_whole_seq | 7.6ns | 569.3ns | 1.3% |  |
| an_whole_table | 7.4ns | 616.7ns | 1.2% |  |
| an_whole_tree | 6.7ns | 581.5ns | 1.2% |  |

## Distribution (algo ns)

```
an_whole_pred (n=6, range 853.3-1101.7 ns)
    853.3 |########################################
    865.7 |
    878.1 |
    890.6 |
    903.0 |
    915.4 |
    927.8 |
    940.2 |
    952.6 |
    965.1 |
    977.5 |
    989.9 |
   1002.3 |
   1014.7 |####################
   1027.1 |
   1039.6 |
   1052.0 |
   1064.4 |
   1076.8 |
   1089.2 |########################################
  (0 below, 1 above range)

an_whole_prof (n=6, range 438.3-569.0 ns)
    438.3 |########################################
    444.8 |
    451.4 |########################################
    457.9 |
    464.4 |
    471.0 |
    477.5 |
    484.0 |
    490.6 |
    497.1 |
    503.6 |
    510.2 |########################################
    516.7 |
    523.2 |########################################
    529.8 |
    536.3 |
    542.8 |
    549.4 |
    555.9 |
    562.4 |########################################
  (0 below, 1 above range)

an_whole_seq (n=6, range 437.1-700.0 ns)
    437.1 |########################################
    450.2 |
    463.4 |
    476.5 |
    489.7 |
    502.8 |
    516.0 |####################
    529.1 |
    542.3 |
    555.4 |
    568.5 |
    581.7 |
    594.8 |########################################
    608.0 |
    621.1 |
    634.3 |
    647.4 |
    660.6 |
    673.7 |
    686.9 |
  (0 below, 1 above range)

an_whole_table (n=6, range 472.9-799.4 ns)
    472.9 |########################################
    489.2 |
    505.5 |
    521.9 |
    538.2 |
    554.5 |
    570.8 |########################################
    587.2 |
    603.5 |####################
    619.8 |
    636.1 |
    652.4 |
    668.8 |
    685.1 |
    701.4 |
    717.7 |
    734.1 |
    750.4 |
    766.7 |
    783.0 |
  (0 below, 1 above range)

an_whole_tree (n=6, range 477.1-663.8 ns)
    477.1 |########################################
    486.4 |
    495.8 |########################################
    505.1 |
    514.4 |
    523.8 |
    533.1 |
    542.4 |
    551.8 |
    561.1 |
    570.4 |########################################
    579.8 |
    589.1 |
    598.4 |
    607.8 |########################################
    617.1 |
    626.4 |########################################
    635.8 |
    645.1 |
    654.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **an_whole_seq**: CV=21.2% (high variance, measurements may be unstable)
- **an_whole_table**: CV=28.4% (high variance, measurements may be unstable)
