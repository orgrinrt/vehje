# tnum linear transfers: add/and/or/shl vs concrete u64 (abstract arith)

5 variants, 6 samples per variant.
Baseline: **tl_concrete**

## Highlights

Baseline for all deltas below: **tl_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tl_shl dominates: 98% faster than the next best (tl_concrete)

tl_shl (1.61 us) leads tl_concrete (3.19 us) by 98%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tl_shl beats baseline by 51% (significant)

tl_shl is -1.64 us (51%) faster than baseline tl_concrete, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### tl_and is an outlier: 2.5x slower than the field

tl_and (3.98 us) is 2.5x the fastest (1.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {tl_shl} vs {tl_concrete, tl_or, tl_add, tl_and} (98% apart)

The field splits into a fast tier {tl_shl} and a slow tier {tl_concrete, tl_or, tl_add, tl_and} with a 98% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### tl_or's edge over baseline is significant but tiny (14 ns, 0.44%)

tl_or differs from baseline tl_concrete by 14 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: tl_shl** at 1612.2 ns median (-49.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.47x (fastest 1612.2 ns, slowest 3978.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tl_add | 6352ns | 6512ns | 5528ns | 6512ns | 6524ns | +11.30% |
| tl_and | 6260ns | 6579ns | 5532ns | 6278ns | 6595ns | +9.68% |
| tl_concrete | 5707ns | 5808ns | 4849ns | 5798ns | 5999ns | base |
| tl_or | 5758ns | 5787ns | 4875ns | 5673ns | 6328ns | +0.90% |
| tl_shl | 4001ns | 4197ns | 3528ns | 4014ns | 4220ns | -29.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tl_add | 3826ns | 3331ns | 3927ns | +21.42% | 0.067 |
| tl_and | 3788ns | 3351ns | 3985ns | +20.23% | 0.068 |
| tl_concrete | 3151ns | 2672ns | 3327ns | base | 0.081 |
| tl_or | 3196ns | 2689ns | 3521ns | +1.43% | 0.080 |
| tl_shl | 1535ns | 1357ns | 1614ns | -51.29% | 0.167 |

## Performance model

- Peak throughput: **0.189 Gops/s** (tl_shl; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tl_add | 0.065 | 34.6% |
| tl_and | 0.064 | 34.1% |
| tl_concrete | 0.080 | 42.5% |
| tl_or | 0.080 | 42.3% |
| tl_shl | 0.159 | 84.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tl_add | 6352ns | 6352ns | +11.30% |
| tl_and | 6260ns | 6260ns | +9.68% |
| tl_concrete | 5707ns | 5707ns | base |
| tl_or | 5758ns | 5758ns | +0.90% |
| tl_shl | 4001ns | 4001ns | -29.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tl_concrete | 3194ns | base | --- | [2931, 3327] | --- | --- | --- | --- |
| tl_add | 3924ns | +692.9ns (+21.7%) | [+597, +735]ns | [3626, 3927] | YES | 0.0417 | 0.0313 | 0 |
| tl_and | 3979ns | +673.8ns (+21.1%) | [+446, +792]ns | [3401, 3985] | YES | 0.0417 | 0.0313 | 0 |
| tl_or | 3204ns | no significant difference | [-74, +195]ns | [2862, 3521] | no | 0.2188 | 0.2188 | 0 |
| tl_shl | 1612ns | -1642.5ns (-51.4%) | [-1760, -1446]ns | [1378, 1614] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tl_concrete | tl_add | tl_and | tl_or | tl_shl |
|---|---|---|---|---|---|
| 1 | 2672ns | +24.7% | +25.4% | +0.7% | -49.2% |
| 2 | 3194ns | +22.7% | +8.0% | +0.4% | -56.2% |
| 3 | 3316ns | +18.3% | +20.1% | +0.5% | -51.3% |
| 4 | 3338ns | +17.6% | +19.1% | +11.2% | -51.7% |
| 5 | 3191ns | +23.1% | +24.9% | -4.9% | -49.4% |
| 6 | 3194ns | +22.9% | +24.7% | +0.2% | -49.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tl_add | -0.026 | ok |
| tl_and | 0.425 | moderate+ |
| tl_concrete | 0.089 | ok |
| tl_or | -0.032 | ok |
| tl_shl | 0.428 | moderate+ |

**Consistency summary:**

- **tl_add**: won 0/6, lost 6/6
- **tl_and**: won 0/6, lost 6/6
- **tl_or**: won 1/6, lost 5/6
- **tl_shl**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tl_add | 4.8ns | 3825.6ns | 0.1% |  |
| tl_and | 3.8ns | 3788.3ns | 0.1% |  |
| tl_concrete | 4.6ns | 3150.8ns | 0.1% |  |
| tl_or | 4.5ns | 3195.8ns | 0.1% |  |
| tl_shl | 3.7ns | 1534.6ns | 0.2% |  |

## Distribution (algo ns)

```
tl_add (n=6, range 3330.8-3927.1 ns)
   3330.8 |##########
   3360.6 |
   3390.4 |
   3420.2 |
   3450.1 |
   3479.9 |
   3509.7 |
   3539.5 |
   3569.3 |
   3599.1 |
   3628.9 |
   3658.8 |
   3688.6 |
   3718.4 |
   3748.2 |
   3778.0 |
   3807.8 |
   3837.7 |
   3867.5 |
   3897.3 |########################################
  (0 below, 1 above range)

tl_and (n=6, range 3351.2-3985.4 ns)
   3351.2 |#############
   3382.9 |
   3414.6 |
   3446.3 |#############
   3478.0 |
   3509.8 |
   3541.5 |
   3573.2 |
   3604.9 |
   3636.6 |
   3668.3 |
   3700.0 |
   3731.8 |
   3763.5 |
   3795.2 |
   3826.9 |
   3858.6 |
   3890.3 |
   3922.0 |
   3953.7 |########################################
  (0 below, 1 above range)

tl_concrete (n=6, range 2671.7-3327.2 ns)
   2671.7 |#############
   2704.5 |
   2737.3 |
   2770.0 |
   2802.8 |
   2835.6 |
   2868.4 |
   2901.1 |
   2933.9 |
   2966.7 |
   2999.5 |
   3032.3 |
   3065.0 |
   3097.8 |
   3130.6 |
   3163.4 |########################################
   3196.1 |
   3228.9 |
   3261.7 |
   3294.5 |#############
  (0 below, 1 above range)

tl_or (n=6, range 2689.2-3521.4 ns)
   2689.2 |####################
   2730.8 |
   2772.4 |
   2814.0 |
   2855.6 |
   2897.3 |
   2938.9 |
   2980.5 |
   3022.1 |####################
   3063.7 |
   3105.3 |
   3146.9 |
   3188.5 |########################################
   3230.2 |
   3271.8 |
   3313.4 |####################
   3355.0 |
   3396.6 |
   3438.2 |
   3479.8 |
  (0 below, 1 above range)

tl_shl (n=6, range 1356.7-1613.7 ns)
   1356.7 |####################
   1369.5 |
   1382.4 |
   1395.2 |####################
   1408.1 |
   1421.0 |
   1433.8 |
   1446.7 |
   1459.5 |
   1472.4 |
   1485.2 |
   1498.0 |
   1510.9 |
   1523.8 |
   1536.6 |
   1549.5 |
   1562.3 |
   1575.2 |
   1588.0 |
   1600.9 |########################################
  (0 below, 2 above range)

```
