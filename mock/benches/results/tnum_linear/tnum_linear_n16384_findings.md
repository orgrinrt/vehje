# tnum linear transfers: add/and/or/shl vs concrete u64 (abstract arith)

5 variants, 6 samples per variant.
Baseline: **tl_concrete**

## Highlights

Baseline for all deltas below: **tl_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tl_shl dominates: 101% faster than the next best (tl_or)

tl_shl (87.81 us) leads tl_or (176.73 us) by 101%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tl_shl beats baseline by 50% (significant)

tl_shl is -90.22 us (50%) faster than baseline tl_concrete, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### tl_and is an outlier: 2.4x slower than the field

tl_and (213.21 us) is 2.4x the fastest (87.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {tl_shl} vs {tl_or, tl_concrete, tl_add, tl_and} (101% apart)

The field splits into a fast tier {tl_shl} and a slow tier {tl_or, tl_concrete, tl_add, tl_and} with a 101% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: tl_shl** at 87809.5 ns median (-50.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.43x (fastest 87809.5 ns, slowest 213212.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tl_add | 213399ns | 212121ns | 211388ns | 212109ns | 216340ns | +18.32% |
| tl_and | 217350ns | 215806ns | 213111ns | 215423ns | 222361ns | +20.51% |
| tl_concrete | 180353ns | 181229ns | 172722ns | 178753ns | 186569ns | base |
| tl_or | 179202ns | 179143ns | 170648ns | 176724ns | 187196ns | -0.64% |
| tl_shl | 92407ns | 90100ns | 88350ns | 89565ns | 98699ns | -48.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tl_add | 210986ns | 209113ns | 213867ns | +18.60% | 0.078 |
| tl_and | 214826ns | 210585ns | 219936ns | +20.76% | 0.076 |
| tl_concrete | 177890ns | 170186ns | 183939ns | base | 0.092 |
| tl_or | 176774ns | 168240ns | 184656ns | -0.63% | 0.093 |
| tl_shl | 90048ns | 86120ns | 96132ns | -49.38% | 0.182 |

## Performance model

- Peak throughput: **0.190 Gops/s** (tl_shl; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tl_add | 0.078 | 41.1% |
| tl_and | 0.077 | 40.4% |
| tl_concrete | 0.092 | 48.1% |
| tl_or | 0.093 | 48.7% |
| tl_shl | 0.187 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tl_add | 213399ns | 213399ns | +18.32% |
| tl_and | 217350ns | 217350ns | +20.51% |
| tl_concrete | 180353ns | 180353ns | base |
| tl_or | 179202ns | 179202ns | -0.64% |
| tl_shl | 92407ns | 92407ns | -48.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tl_concrete | 178872ns | base | --- | [170861, 183939] | --- | --- | --- | --- |
| tl_add | 209764ns | +34995.4ns (+19.6%) | [+25389, +38903]ns | [209328, 213867] | YES | 0.0417 | 0.0313 | 0 |
| tl_and | 213212ns | +39175.6ns (+21.9%) | [+27813, +43819]ns | [211330, 219936] | YES | 0.0417 | 0.0313 | 0 |
| tl_or | 176733ns | no significant difference | [-11171, +9052]ns | [168934, 184656] | no | 1.0000 | 1.0000 | 0 |
| tl_shl | 87810ns | -90222.1ns (-50.4%) | [-93555, -79751]ns | [86202, 96132] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tl_concrete | tl_add | tl_and | tl_or | tl_shl |
|---|---|---|---|---|---|
| 1 | 186693ns | +12.2% | +12.8% | -1.9% | -48.5% |
| 2 | 171535ns | +22.3% | +24.5% | +1.2% | -49.8% |
| 3 | 179024ns | +17.9% | +20.4% | -5.2% | -51.8% |
| 4 | 178719ns | +21.3% | +25.6% | +0.6% | -50.3% |
| 5 | 181185ns | +15.4% | +17.5% | -7.1% | -52.1% |
| 6 | 170186ns | +23.3% | +24.6% | +9.4% | -43.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tl_add | -0.160 | ok |
| tl_and | -0.016 | ok |
| tl_concrete | -0.447 | moderate- |
| tl_or | -0.463 | moderate- |
| tl_shl | -0.174 | ok |

**Consistency summary:**

- **tl_add**: won 0/6, lost 6/6
- **tl_and**: won 0/6, lost 6/6
- **tl_or**: won 3/6, lost 3/6
- **tl_shl**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tl_add | 5.1ns | 210986.1ns | 0.0% |  |
| tl_and | 6.7ns | 214826.1ns | 0.0% |  |
| tl_concrete | 4.8ns | 177890.4ns | 0.0% |  |
| tl_or | 6.0ns | 176774.1ns | 0.0% |  |
| tl_shl | 4.1ns | 90047.8ns | 0.0% |  |

## Distribution (algo ns)

```
tl_add (n=6, range 209113.3-213866.9 ns)
  209113.3 |####################
  209351.0 |####################
  209588.7 |########################################
  209826.3 |
  210064.0 |
  210301.7 |
  210539.4 |
  210777.0 |####################
  211014.7 |
  211252.4 |
  211490.1 |
  211727.8 |
  211965.4 |
  212203.1 |
  212440.8 |
  212678.5 |
  212916.1 |
  213153.8 |
  213391.5 |
  213629.2 |
  (0 below, 1 above range)

tl_and (n=6, range 210585.0-219936.2 ns)
  210585.0 |########################################
  211052.6 |
  211520.1 |
  211987.7 |########################################
  212455.2 |########################################
  212922.8 |
  213390.4 |########################################
  213857.9 |
  214325.5 |
  214793.0 |
  215260.6 |########################################
  215728.2 |
  216195.7 |
  216663.3 |
  217130.8 |
  217598.4 |
  218066.0 |
  218533.5 |
  219001.1 |
  219468.6 |
  (0 below, 1 above range)

tl_concrete (n=6, range 170185.8-183939.1 ns)
  170185.8 |####################
  170873.5 |####################
  171561.1 |
  172248.8 |
  172936.5 |
  173624.1 |
  174311.8 |
  174999.5 |
  175687.1 |
  176374.8 |
  177062.5 |
  177750.1 |
  178437.8 |########################################
  179125.5 |
  179813.1 |
  180500.8 |####################
  181188.5 |
  181876.1 |
  182563.8 |
  183251.5 |
  (0 below, 1 above range)

tl_or (n=6, range 168239.6-184655.6 ns)
  168239.6 |########################################
  169060.4 |########################################
  169881.2 |
  170702.0 |
  171522.8 |
  172343.6 |
  173164.4 |########################################
  173985.2 |
  174806.0 |
  175626.8 |
  176447.6 |
  177268.4 |
  178089.2 |
  178910.0 |
  179730.8 |########################################
  180551.6 |
  181372.4 |
  182193.2 |
  183014.0 |########################################
  183834.8 |
  (0 below, 1 above range)

tl_shl (n=6, range 86120.4-96131.6 ns)
  86120.4 |########################################
  86621.0 |####################
  87121.5 |
  87622.1 |
  88122.6 |
  88623.2 |####################
  89123.8 |
  89624.3 |
  90124.9 |
  90625.5 |
  91126.0 |
  91626.6 |
  92127.1 |
  92627.7 |
  93128.3 |
  93628.8 |
  94129.4 |
  94630.0 |
  95130.5 |
  95631.1 |####################
  (0 below, 1 above range)

```
