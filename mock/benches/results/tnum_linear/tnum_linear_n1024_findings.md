# tnum linear transfers: add/and/or/shl vs concrete u64 (abstract arith)

5 variants, 6 samples per variant.
Baseline: **tl_concrete**

## Highlights

Baseline for all deltas below: **tl_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tl_shl dominates: 90% faster than the next best (tl_concrete)

tl_shl (6.42 us) leads tl_concrete (12.22 us) by 90%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tl_shl beats baseline by 47% (significant)

tl_shl is -5.80 us (47%) faster than baseline tl_concrete, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### tl_add is an outlier: 2.4x slower than the field

tl_add (15.52 us) is 2.4x the fastest (6.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {tl_shl} vs {tl_concrete, tl_or, tl_and, tl_add} (90% apart)

The field splits into a fast tier {tl_shl} and a slow tier {tl_concrete, tl_or, tl_and, tl_add} with a 90% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### tl_or's edge over baseline is significant but tiny (-32 ns, 0.26%)

tl_or differs from baseline tl_concrete by -32 ns (0.26%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: tl_shl** at 6417.5 ns median (-47.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.42x (fastest 6417.5 ns, slowest 15515.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tl_add | 17256ns | 18119ns | 15198ns | 17155ns | 18438ns | +20.23% |
| tl_and | 17217ns | 17749ns | 15458ns | 17007ns | 18412ns | +19.95% |
| tl_concrete | 14353ns | 14718ns | 12899ns | 14121ns | 15428ns | base |
| tl_or | 14448ns | 15256ns | 12643ns | 14454ns | 15343ns | +0.66% |
| tl_shl | 8567ns | 9004ns | 7570ns | 8538ns | 9109ns | -40.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tl_add | 14764ns | 13001ns | 15773ns | +24.00% | 0.069 |
| tl_and | 14791ns | 13278ns | 15813ns | +24.23% | 0.069 |
| tl_concrete | 11907ns | 10678ns | 12802ns | base | 0.086 |
| tl_or | 12008ns | 10505ns | 12745ns | +0.85% | 0.085 |
| tl_shl | 6109ns | 5388ns | 6505ns | -48.69% | 0.168 |

## Performance model

- Peak throughput: **0.190 Gops/s** (tl_shl; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tl_add | 0.066 | 34.7% |
| tl_and | 0.067 | 35.3% |
| tl_concrete | 0.084 | 44.1% |
| tl_or | 0.081 | 42.5% |
| tl_shl | 0.160 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tl_add | 17256ns | 17256ns | +20.23% |
| tl_and | 17217ns | 17217ns | +19.95% |
| tl_concrete | 14353ns | 14353ns | base |
| tl_or | 14448ns | 14448ns | +0.66% |
| tl_shl | 8567ns | 8567ns | -40.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tl_concrete | 12222ns | base | --- | [10695, 12802] | --- | --- | --- | --- |
| tl_add | 15515ns | +2833.6ns (+23.2%) | [+2310, +3430]ns | [13006, 15773] | YES | 0.0417 | 0.0313 | 0 |
| tl_and | 15249ns | +2780.9ns (+22.8%) | [+2402, +3470]ns | [13311, 15813] | YES | 0.0417 | 0.0313 | 0 |
| tl_or | 12687ns | no significant difference | [-183, +520]ns | [10593, 12745] | no | 1.0000 | 1.0000 | 0 |
| tl_shl | 6418ns | -5801.1ns (-47.5%) | [-6300, -5291]ns | [5405, 6505] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tl_concrete | tl_add | tl_and | tl_or | tl_shl |
|---|---|---|---|---|---|
| 1 | 10713ns | +21.4% | +23.9% | -1.9% | -49.7% |
| 2 | 12806ns | +21.3% | +22.6% | -1.2% | -49.9% |
| 3 | 12557ns | +23.4% | +17.8% | +1.6% | -48.9% |
| 4 | 12798ns | +23.6% | +23.6% | -0.5% | -48.5% |
| 5 | 11888ns | +32.3% | +32.9% | +7.0% | -46.0% |
| 6 | 10678ns | +21.8% | +25.0% | +0.0% | -49.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tl_add | -0.073 | ok |
| tl_and | -0.244 | moderate- |
| tl_concrete | 0.020 | ok |
| tl_or | -0.062 | ok |
| tl_shl | -0.031 | ok |

**Consistency summary:**

- **tl_add**: won 0/6, lost 6/6
- **tl_and**: won 0/6, lost 6/6
- **tl_or**: won 3/6, lost 2/6
- **tl_shl**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tl_add | 4.1ns | 14764.5ns | 0.0% |  |
| tl_and | 3.5ns | 14791.0ns | 0.0% |  |
| tl_concrete | 3.9ns | 11906.6ns | 0.0% |  |
| tl_or | 3.0ns | 12008.0ns | 0.0% |  |
| tl_shl | 3.4ns | 6109.2ns | 0.1% |  |

## Distribution (algo ns)

```
tl_add (n=6, range 13000.8-15772.9 ns)
  13000.8 |########################################
  13139.4 |
  13278.0 |
  13416.6 |
  13555.2 |
  13693.8 |
  13832.4 |
  13971.0 |
  14109.6 |
  14248.2 |
  14386.9 |
  14525.5 |
  14664.1 |
  14802.7 |
  14941.3 |
  15079.9 |
  15218.5 |
  15357.1 |
  15495.7 |########################################
  15634.3 |####################
  (0 below, 1 above range)

tl_and (n=6, range 13277.5-15813.1 ns)
  13277.5 |########################################
  13404.3 |
  13531.1 |
  13657.8 |
  13784.6 |
  13911.4 |
  14038.2 |
  14165.0 |
  14291.7 |
  14418.5 |
  14545.3 |
  14672.1 |####################
  14798.9 |
  14925.6 |
  15052.4 |
  15179.2 |
  15306.0 |
  15432.8 |
  15559.5 |
  15686.3 |########################################
  (0 below, 1 above range)

tl_concrete (n=6, range 10677.9-12802.0 ns)
  10677.9 |########################################
  10784.1 |
  10890.3 |
  10996.5 |
  11102.7 |
  11208.9 |
  11315.1 |
  11421.4 |
  11527.6 |
  11633.8 |
  11740.0 |
  11846.2 |####################
  11952.4 |
  12058.6 |
  12164.8 |
  12271.0 |
  12377.2 |
  12483.4 |####################
  12589.6 |
  12695.8 |####################
  (0 below, 1 above range)

tl_or (n=6, range 10504.6-12744.5 ns)
  10504.6 |#############
  10616.6 |#############
  10728.6 |
  10840.6 |
  10952.6 |
  11064.6 |
  11176.6 |
  11288.6 |
  11400.6 |
  11512.6 |
  11624.6 |
  11736.6 |
  11848.6 |
  11960.6 |
  12072.6 |
  12184.6 |
  12296.6 |
  12408.6 |
  12520.6 |
  12632.6 |########################################
  (0 below, 1 above range)

tl_shl (n=6, range 5387.9-6505.4 ns)
   5387.9 |##########################
   5443.8 |
   5499.6 |
   5555.5 |
   5611.4 |
   5667.3 |
   5723.1 |
   5779.0 |
   5834.9 |
   5890.8 |
   5946.6 |
   6002.5 |
   6058.4 |
   6114.3 |
   6170.1 |
   6226.0 |
   6281.9 |
   6337.8 |
   6393.6 |########################################
   6449.5 |
  (0 below, 1 above range)

```
