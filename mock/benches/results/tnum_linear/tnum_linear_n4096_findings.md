# tnum linear transfers: add/and/or/shl vs concrete u64 (abstract arith)

5 variants, 6 samples per variant.
Baseline: **tl_concrete**

## Highlights

Baseline for all deltas below: **tl_concrete**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### tl_shl dominates: 101% faster than the next best (tl_concrete)

tl_shl (22.05 us) leads tl_concrete (44.32 us) by 101%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### tl_shl beats baseline by 49% (significant)

tl_shl is -21.59 us (49%) faster than baseline tl_concrete, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### tl_add is an outlier: 2.5x slower than the field

tl_add (54.31 us) is 2.5x the fastest (22.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {tl_shl} vs {tl_concrete, tl_or, tl_and, tl_add} (101% apart)

The field splits into a fast tier {tl_shl} and a slow tier {tl_concrete, tl_or, tl_and, tl_add} with a 101% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### tl_or's edge over baseline is significant but tiny (-31 ns, 0.07%)

tl_or differs from baseline tl_concrete by -31 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: tl_shl** at 22045.4 ns median (-50.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.46x (fastest 22045.4 ns, slowest 54314.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| tl_add | 57134ns | 56660ns | 54329ns | 55914ns | 60367ns | +21.65% |
| tl_and | 57925ns | 56485ns | 54777ns | 56053ns | 62307ns | +23.33% |
| tl_concrete | 46967ns | 46642ns | 44822ns | 46421ns | 48858ns | base |
| tl_or | 47671ns | 47492ns | 44889ns | 46833ns | 50318ns | +1.50% |
| tl_shl | 24791ns | 24275ns | 23704ns | 24089ns | 26386ns | -47.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| tl_add | 54770ns | 52106ns | 57833ns | +22.77% | 0.075 |
| tl_and | 55533ns | 52595ns | 59704ns | +24.48% | 0.074 |
| tl_concrete | 44610ns | 42608ns | 46381ns | base | 0.092 |
| tl_or | 45312ns | 42703ns | 47856ns | +1.57% | 0.090 |
| tl_shl | 22487ns | 21445ns | 23925ns | -49.59% | 0.182 |

## Performance model

- Peak throughput: **0.191 Gops/s** (tl_shl; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| tl_add | 0.075 | 39.5% |
| tl_and | 0.076 | 39.6% |
| tl_concrete | 0.092 | 48.4% |
| tl_or | 0.091 | 47.6% |
| tl_shl | 0.186 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| tl_add | 57134ns | 57134ns | +21.65% |
| tl_and | 57925ns | 57925ns | +23.33% |
| tl_concrete | 46967ns | 46967ns | base |
| tl_or | 47671ns | 47671ns | +1.50% |
| tl_shl | 24791ns | 24791ns | -47.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| tl_concrete | 44323ns | base | --- | [43127, 46381] | --- | --- | --- | --- |
| tl_add | 54315ns | +9598.8ns (+21.7%) | [+8820, +12061]ns | [52163, 57833] | YES | 0.0417 | 0.0313 | 0 |
| tl_and | 54096ns | +10741.7ns (+24.2%) | [+8702, +13323]ns | [52798, 59704] | YES | 0.0417 | 0.0313 | 0 |
| tl_or | 45092ns | no significant difference | [-1242, +3379]ns | [42989, 47856] | no | 1.0000 | 1.0000 | 0 |
| tl_shl | 22045ns | -21590.8ns (-48.7%) | [-24228, -20552]ns | [21490, 23925] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | tl_concrete | tl_add | tl_and | tl_or | tl_shl |
|---|---|---|---|---|---|
| 1 | 44545ns | +22.8% | +18.1% | -2.8% | -51.9% |
| 2 | 47909ns | +20.2% | +23.9% | -2.5% | -52.9% |
| 3 | 44101ns | +31.6% | +25.0% | +7.3% | -45.8% |
| 4 | 44853ns | +20.2% | +33.9% | +7.9% | -46.6% |
| 5 | 42608ns | +22.3% | +24.5% | +2.1% | -49.5% |
| 6 | 43646ns | +19.6% | +21.4% | -2.2% | -50.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| tl_add | 0.465 | moderate+ |
| tl_and | -0.346 | moderate- |
| tl_concrete | -0.036 | ok |
| tl_or | 0.176 | ok |
| tl_shl | 0.231 | moderate+ |

**Consistency summary:**

- **tl_add**: won 0/6, lost 6/6
- **tl_and**: won 0/6, lost 6/6
- **tl_or**: won 3/6, lost 3/6
- **tl_shl**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| tl_add | 6.3ns | 54770.3ns | 0.0% |  |
| tl_and | 4.5ns | 55532.8ns | 0.0% |  |
| tl_concrete | 3.3ns | 44610.4ns | 0.0% |  |
| tl_or | 4.2ns | 45312.4ns | 0.0% |  |
| tl_shl | 3.4ns | 22486.9ns | 0.0% |  |

## Distribution (algo ns)

```
tl_add (n=6, range 52106.2-57833.3 ns)
  52106.2 |########################################
  52392.6 |
  52678.9 |
  52965.3 |
  53251.6 |
  53538.0 |
  53824.3 |####################
  54110.7 |
  54397.0 |
  54683.4 |####################
  54969.8 |
  55256.1 |
  55542.5 |
  55828.8 |
  56115.2 |
  56401.5 |
  56687.9 |
  56974.2 |
  57260.6 |
  57546.9 |####################
  (0 below, 1 above range)

tl_and (n=6, range 52595.0-59704.2 ns)
  52595.0 |####################
  52950.5 |########################################
  53305.9 |
  53661.4 |
  54016.8 |
  54372.3 |
  54727.7 |
  55083.2 |####################
  55438.7 |
  55794.1 |
  56149.6 |
  56505.0 |
  56860.5 |
  57215.9 |
  57571.4 |
  57926.9 |
  58282.3 |
  58637.8 |
  58993.2 |####################
  59348.7 |
  (0 below, 1 above range)

tl_concrete (n=6, range 42608.3-46380.8 ns)
  42608.3 |########################################
  42796.9 |
  42985.6 |
  43174.2 |
  43362.8 |
  43551.4 |########################################
  43740.1 |
  43928.7 |########################################
  44117.3 |
  44305.9 |
  44494.6 |########################################
  44683.2 |########################################
  44871.8 |
  45060.4 |
  45249.1 |
  45437.7 |
  45626.3 |
  45814.9 |
  46003.6 |
  46192.2 |
  (0 below, 1 above range)

tl_or (n=6, range 42702.9-47856.4 ns)
  42702.9 |########################################
  42960.6 |
  43218.3 |########################################
  43475.9 |########################################
  43733.6 |
  43991.3 |
  44249.0 |
  44506.6 |
  44764.3 |
  45022.0 |
  45279.7 |
  45537.4 |
  45795.0 |
  46052.7 |
  46310.4 |
  46568.1 |########################################
  46825.7 |
  47083.4 |########################################
  47341.1 |
  47598.8 |
  (0 below, 1 above range)

tl_shl (n=6, range 21445.0-23925.2 ns)
  21445.0 |########################################
  21569.0 |
  21693.0 |
  21817.0 |
  21941.0 |
  22065.0 |
  22189.1 |
  22313.1 |
  22437.1 |#############
  22561.1 |
  22685.1 |
  22809.1 |
  22933.1 |
  23057.1 |
  23181.1 |
  23305.1 |
  23429.2 |
  23553.2 |
  23677.2 |
  23801.2 |#############
  (0 below, 1 above range)

```
