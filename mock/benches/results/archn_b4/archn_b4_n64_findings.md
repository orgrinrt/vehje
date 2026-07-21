# Per-branch strategy (NATIVE tier): archetype 4

5 variants, 6 samples per variant.
Baseline: **an_b4_table**

## Highlights

Baseline for all deltas below: **an_b4_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_b4_table is fastest but the noisiest (CV 10.8%)

an_b4_table wins on median (572 ns) yet has the highest variance (CV 10.8%), while an_b4_tree is the steadiest (CV 10.2%, 593 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### an_b4_tree shows warm-up / thermal drift (autocorr +0.56)

an_b4_tree's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (an_b4_table)

The baseline an_b4_table is the fastest (572 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader an_b4_table vs stability leader an_b4_tree (+4% speed for 1.1x steadier)

an_b4_table is fastest (572 ns, CV 10.8%); an_b4_tree gives up 3.7% median for 1.1x lower variance (CV 10.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### an_b4_tree's edge over baseline is significant but tiny (-1 ns, 0.11%)

an_b4_tree differs from baseline an_b4_table by -1 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (an_b4_table) is the fastest** at 571.7 ns median
- 2 variants significantly slower than baseline
- Spread: 1.29x (fastest 571.7 ns, slowest 737.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_b4_pred | 3367ns | 3516ns | 2761ns | 3355ns | 3688ns | +8.21% |
| an_b4_prof | 3045ns | 3058ns | 2644ns | 2923ns | 3430ns | -2.13% |
| an_b4_seq | 3138ns | 3296ns | 2616ns | 3113ns | 3437ns | +0.85% |
| an_b4_table | 3112ns | 3147ns | 2635ns | 3003ns | 3513ns | base |
| an_b4_tree | 3125ns | 3267ns | 2636ns | 3085ns | 3429ns | +0.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_b4_pred | 706ns | 584ns | 776ns | +24.89% | 0.091 |
| an_b4_prof | 575ns | 495ns | 646ns | +1.75% | 0.111 |
| an_b4_seq | 592ns | 495ns | 651ns | +4.73% | 0.108 |
| an_b4_table | 565ns | 486ns | 637ns | base | 0.113 |
| an_b4_tree | 567ns | 482ns | 623ns | +0.26% | 0.113 |

## Performance model

- Peak throughput: **0.133 Gops/s** (an_b4_tree; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_b4_pred | 0.087 | 65.4% |
| an_b4_prof | 0.110 | 83.0% |
| an_b4_seq | 0.103 | 77.6% |
| an_b4_table | 0.112 | 84.3% |
| an_b4_tree | 0.108 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_b4_pred | 3367ns | 3367ns | +8.21% |
| an_b4_prof | 3045ns | 3045ns | -2.13% |
| an_b4_seq | 3138ns | 3138ns | +0.85% |
| an_b4_table | 3112ns | 3112ns | base |
| an_b4_tree | 3125ns | 3125ns | +0.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_b4_table | 572ns | base | --- | [488, 637] | --- | --- | --- | --- |
| an_b4_pred | 737ns | +152.2ns (+26.6%) | [+90, +180]ns | [605, 776] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_b4_prof | 581ns | no significant difference | [-10, +28]ns | [499, 646] | no | 0.6875 | 0.6875 | 0 |
| an_b4_seq | 621ns | +25.6ns (+4.5%) | [+2, +53]ns | [505, 651] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| an_b4_tree | 593ns | no significant difference | [-20, +25]ns | [485, 623] | no | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_b4_table | an_b4_pred | an_b4_prof | an_b4_seq | an_b4_tree |
|---|---|---|---|---|---|
| 1 | 574ns | +29.2% | -1.7% | +13.6% | +8.9% |
| 2 | 621ns | +30.6% | +4.6% | +4.6% | -0.1% |
| 3 | 653ns | +12.7% | -1.7% | -0.9% | -5.5% |
| 4 | 570ns | +29.7% | +4.8% | +4.5% | -0.1% |
| 5 | 489ns | +28.0% | +1.2% | +5.2% | -0.2% |
| 6 | 486ns | +20.2% | +3.7% | +1.9% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_b4_pred | 0.420 | moderate+ |
| an_b4_prof | 0.432 | moderate+ |
| an_b4_seq | 0.555 | HIGH+ (drift/warm-up) |
| an_b4_table | 0.498 | moderate+ |
| an_b4_tree | 0.561 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **an_b4_pred**: won 0/6, lost 6/6
- **an_b4_prof**: won 2/6, lost 4/6
- **an_b4_seq**: won 1/6, lost 5/6
- **an_b4_tree**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_b4_pred | 5.2ns | 706.2ns | 0.7% |  |
| an_b4_prof | 6.2ns | 575.3ns | 1.1% |  |
| an_b4_seq | 6.6ns | 592.2ns | 1.1% |  |
| an_b4_table | 6.8ns | 565.4ns | 1.2% |  |
| an_b4_tree | 7.7ns | 566.9ns | 1.4% |  |

## Distribution (algo ns)

```
an_b4_pred (n=6, range 584.2-776.2 ns)
    584.2 |####################
    593.8 |
    603.4 |
    613.0 |
    622.6 |####################
    632.2 |
    641.8 |
    651.4 |
    661.0 |
    670.6 |
    680.2 |
    689.8 |
    699.4 |
    709.0 |
    718.6 |
    728.2 |####################
    737.8 |########################################
    747.4 |
    757.0 |
    766.6 |
  (0 below, 1 above range)

an_b4_prof (n=6, range 494.6-645.9 ns)
    494.6 |########################################
    502.2 |########################################
    509.7 |
    517.3 |
    524.9 |
    532.4 |
    540.0 |
    547.5 |
    555.1 |
    562.7 |########################################
    570.2 |
    577.8 |
    585.4 |
    592.9 |########################################
    600.5 |
    608.0 |
    615.6 |
    623.2 |
    630.7 |
    638.3 |########################################
  (0 below, 1 above range)

an_b4_seq (n=6, range 495.4-650.7 ns)
    495.4 |####################
    503.2 |
    510.9 |####################
    518.7 |
    526.5 |
    534.2 |
    542.0 |
    549.7 |
    557.5 |
    565.3 |
    573.0 |
    580.8 |
    588.6 |####################
    596.3 |
    604.1 |
    611.8 |
    619.6 |
    627.4 |
    635.1 |
    642.9 |########################################
  (0 below, 1 above range)

an_b4_table (n=6, range 486.2-637.0 ns)
    486.2 |########################################
    493.7 |
    501.3 |
    508.8 |
    516.4 |
    523.9 |
    531.5 |
    539.0 |
    546.5 |
    554.1 |
    561.6 |
    569.2 |########################################
    576.7 |
    584.3 |
    591.8 |
    599.3 |
    606.9 |
    614.4 |####################
    622.0 |
    629.5 |
  (0 below, 1 above range)

an_b4_tree (n=6, range 482.1-622.7 ns)
    482.1 |########################################
    489.1 |
    496.2 |
    503.2 |
    510.2 |
    517.2 |
    524.3 |
    531.3 |
    538.3 |
    545.4 |
    552.4 |
    559.4 |
    566.5 |####################
    573.5 |
    580.5 |
    587.6 |
    594.6 |
    601.6 |
    608.6 |
    615.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **an_b4_seq**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **an_b4_tree**: autocorrelation=0.56 (measurement drift or warm-up artifact)
