# Per-type strategy (NATIVE tier): all ifchain

5 variants, 6 samples per variant.
Baseline: **an_ifchain_table**

## Highlights

Baseline for all deltas below: **an_ifchain_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_ifchain_pred is an outlier: 2.2x slower than the field

an_ifchain_pred (1.03 us) is 2.2x the fastest (479 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### an_ifchain_prof is fastest but the noisiest (CV 10.1%)

an_ifchain_prof wins on median (479 ns) yet has the highest variance (CV 10.1%), while an_ifchain_pred is the steadiest (CV 7.6%, 1.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {an_ifchain_prof, an_ifchain_seq, an_ifchain_table, an_ifchain_tree} vs {an_ifchain_pred} (82% apart)

The field splits into a fast tier {an_ifchain_prof, an_ifchain_seq, an_ifchain_table, an_ifchain_tree} and a slow tier {an_ifchain_pred} with a 82% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### an_ifchain_tree's comparison is tie-heavy (17% tied pairs)

17% of paired samples for an_ifchain_tree are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### an_ifchain_tree's edge over baseline is significant but tiny (0 ns, 0.04%)

an_ifchain_tree differs from baseline an_ifchain_table by 0 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: an_ifchain_prof** at 479.1 ns median (-13.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.15x (fastest 479.1 ns, slowest 1030.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_ifchain_pred | 3519ns | 3532ns | 3082ns | 3460ns | 3826ns | +16.52% |
| an_ifchain_prof | 2914ns | 2884ns | 2594ns | 2814ns | 3225ns | -3.51% |
| an_ifchain_seq | 2905ns | 2889ns | 2578ns | 2789ns | 3242ns | -3.81% |
| an_ifchain_table | 3020ns | 3105ns | 2623ns | 2949ns | 3326ns | base |
| an_ifchain_tree | 3025ns | 3129ns | 2636ns | 2993ns | 3268ns | +0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_ifchain_pred | 1023ns | 908ns | 1105ns | +88.67% | 0.063 |
| an_ifchain_prof | 487ns | 439ns | 543ns | -10.13% | 0.131 |
| an_ifchain_seq | 488ns | 428ns | 541ns | -9.96% | 0.131 |
| an_ifchain_table | 542ns | 471ns | 596ns | base | 0.118 |
| an_ifchain_tree | 550ns | 485ns | 597ns | +1.53% | 0.116 |

## Performance model

- Peak throughput: **0.150 Gops/s** (an_ifchain_seq; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_ifchain_pred | 0.062 | 41.5% |
| an_ifchain_prof | 0.134 | 89.2% |
| an_ifchain_seq | 0.131 | 87.7% |
| an_ifchain_table | 0.116 | 77.5% |
| an_ifchain_tree | 0.113 | 75.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_ifchain_pred | 3519ns | 3519ns | +16.52% |
| an_ifchain_prof | 2914ns | 2914ns | -3.51% |
| an_ifchain_seq | 2905ns | 2905ns | -3.81% |
| an_ifchain_table | 3020ns | 3020ns | base |
| an_ifchain_tree | 3025ns | 3025ns | +0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_ifchain_table | 552ns | base | --- | [478, 596] | --- | --- | --- | --- |
| an_ifchain_pred | 1031ns | +488.1ns (+88.5%) | [+430, +524]ns | [933, 1105] | YES | 0.0417 | 0.0313 | 0 |
| an_ifchain_prof | 479ns | -51.6ns (-9.4%) | [-78, -35]ns | [439, 543] | YES | 0.0417 | 0.0313 | 0 |
| an_ifchain_seq | 487ns | -49.7ns (-9.0%) | [-70, -43]ns | [436, 541] | YES | 0.0417 | 0.0313 | 0 |
| an_ifchain_tree | 568ns | no significant difference | [-1, +26]ns | [486, 597] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_ifchain_table | an_ifchain_pred | an_ifchain_prof | an_ifchain_seq | an_ifchain_tree |
|---|---|---|---|---|---|
| 1 | 538ns | +96.6% | -18.4% | -9.1% | +6.6% |
| 2 | 622ns | +84.7% | -9.1% | -8.8% | -0.3% |
| 3 | 565ns | +77.5% | -9.4% | -8.9% | +0.1% |
| 4 | 570ns | +85.9% | -8.8% | -14.8% | -0.1% |
| 5 | 471ns | +103.3% | -6.6% | -9.3% | +3.5% |
| 6 | 485ns | +87.0% | -8.1% | -8.6% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_ifchain_pred | 0.174 | ok |
| an_ifchain_prof | -0.046 | ok |
| an_ifchain_seq | 0.391 | moderate+ |
| an_ifchain_table | 0.264 | moderate+ |
| an_ifchain_tree | 0.414 | moderate+ |

**Consistency summary:**

- **an_ifchain_pred**: won 0/6, lost 6/6
- **an_ifchain_prof**: won 6/6, lost 0/6
- **an_ifchain_seq**: won 6/6, lost 0/6
- **an_ifchain_tree**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_ifchain_pred | 5.6ns | 1022.7ns | 0.5% |  |
| an_ifchain_prof | 5.9ns | 487.1ns | 1.2% |  |
| an_ifchain_seq | 5.4ns | 488.1ns | 1.1% |  |
| an_ifchain_table | 5.8ns | 542.1ns | 1.1% |  |
| an_ifchain_tree | 6.7ns | 550.3ns | 1.2% |  |

## Distribution (algo ns)

```
an_ifchain_pred (n=6, range 907.5-1104.8 ns)
    907.5 |####################
    917.4 |
    927.2 |
    937.1 |
    947.0 |
    956.8 |####################
    966.7 |
    976.6 |
    986.4 |
    996.3 |####################
   1006.1 |
   1016.0 |
   1025.9 |
   1035.7 |
   1045.6 |
   1055.5 |########################################
   1065.3 |
   1075.2 |
   1085.1 |
   1094.9 |
  (0 below, 1 above range)

an_ifchain_prof (n=6, range 438.7-542.9 ns)
    438.7 |########################################
    443.9 |####################
    449.1 |
    454.3 |
    459.5 |
    464.8 |
    470.0 |
    475.2 |
    480.4 |
    485.6 |
    490.8 |
    496.0 |
    501.2 |
    506.4 |
    511.6 |####################
    516.9 |####################
    522.1 |
    527.3 |
    532.5 |
    537.7 |
  (0 below, 1 above range)

an_ifchain_seq (n=6, range 427.5-541.2 ns)
    427.5 |####################
    433.2 |
    438.9 |####################
    444.6 |
    450.2 |
    455.9 |
    461.6 |
    467.3 |
    473.0 |
    478.7 |
    484.4 |########################################
    490.1 |
    495.8 |
    501.4 |
    507.1 |
    512.8 |####################
    518.5 |
    524.2 |
    529.9 |
    535.6 |
  (0 below, 1 above range)

an_ifchain_table (n=6, range 471.2-596.2 ns)
    471.2 |####################
    477.5 |
    483.7 |####################
    490.0 |
    496.2 |
    502.5 |
    508.7 |
    515.0 |
    521.2 |
    527.5 |
    533.7 |####################
    540.0 |
    546.2 |
    552.5 |
    558.7 |
    565.0 |########################################
    571.2 |
    577.5 |
    583.7 |
    590.0 |
  (0 below, 1 above range)

an_ifchain_tree (n=6, range 485.4-597.0 ns)
    485.4 |########################################
    491.0 |
    496.6 |
    502.1 |
    507.7 |
    513.3 |
    518.9 |
    524.5 |
    530.1 |
    535.6 |
    541.2 |
    546.8 |
    552.4 |
    558.0 |
    563.6 |####################
    569.1 |########################################
    574.7 |
    580.3 |
    585.9 |
    591.5 |
  (0 below, 1 above range)

```
