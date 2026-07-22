# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (131 ns) is smaller than the fastest variant's own run-to-run std-dev (167 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.2% of the fastest

All 5 variants sit between 11.19 us and 11.32 us - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_tight_rec20's edge over baseline is significant but tiny (7 ns, 0.06%)

carrier_lay_tight_rec20 differs from baseline carrier_lay_tight_rec24 by 7 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_tight_rec16** at 11191.8 ns median (-0.1% vs baseline)
- Spread: 1.01x (fastest 11191.8 ns, slowest 11322.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 13613ns | 13926ns | 12066ns | 13875ns | 13994ns | -1.60% |
| carrier_lay_tight_rec16 | 13745ns | 13752ns | 13260ns | 13734ns | 14002ns | -0.65% |
| carrier_lay_tight_rec20 | 13717ns | 13748ns | 13512ns | 13692ns | 13857ns | -0.85% |
| carrier_lay_tight_rec24 | 13835ns | 13791ns | 13568ns | 13760ns | 14080ns | base |
| carrier_lay_tight_rec32 | 13812ns | 13819ns | 13610ns | 13808ns | 13920ns | -0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 11068ns | 9822ns | 11358ns | -1.34% | 0.023 |
| carrier_lay_tight_rec16 | 11178ns | 10855ns | 11344ns | -0.37% | 0.023 |
| carrier_lay_tight_rec20 | 11189ns | 11030ns | 11258ns | -0.26% | 0.023 |
| carrier_lay_tight_rec24 | 11219ns | 11090ns | 11334ns | base | 0.023 |
| carrier_lay_tight_rec32 | 11225ns | 11111ns | 11319ns | +0.06% | 0.023 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_tight_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.023 | 86.7% |
| carrier_lay_tight_rec16 | 0.023 | 87.8% |
| carrier_lay_tight_rec20 | 0.023 | 87.5% |
| carrier_lay_tight_rec24 | 0.023 | 87.7% |
| carrier_lay_tight_rec32 | 0.023 | 87.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 13613ns | 13613ns | -1.60% |
| carrier_lay_tight_rec16 | 13745ns | 13745ns | -0.65% |
| carrier_lay_tight_rec20 | 13717ns | 13717ns | -0.85% |
| carrier_lay_tight_rec24 | 13835ns | 13835ns | base |
| carrier_lay_tight_rec32 | 13812ns | 13812ns | -0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 11201ns | base | --- | [11121, 11334] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 11322ns | no significant difference | [-731, +214]ns | [10524, 11358] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec16 | 11192ns | no significant difference | [-282, +222]ns | [10997, 11344] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec20 | 11221ns | no significant difference | [-197, +101]ns | [11088, 11258] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec32 | 11221ns | no significant difference | [-196, +171]ns | [11137, 11319] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 11202ns | -12.3% | -3.1% | -0.5% | +0.4% |
| 2 | 11199ns | +1.6% | +0.3% | +0.6% | +1.7% |
| 3 | 11153ns | +1.7% | +1.5% | +0.6% | +0.4% |
| 4 | 11361ns | -0.4% | -1.9% | -2.9% | -2.2% |
| 5 | 11306ns | -0.7% | -1.5% | -0.6% | -1.3% |
| 6 | 11090ns | +2.2% | +2.5% | +1.2% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | -0.084 | ok |
| carrier_lay_tight_rec16 | -0.129 | ok |
| carrier_lay_tight_rec20 | -0.336 | moderate- |
| carrier_lay_tight_rec24 | -0.133 | ok |
| carrier_lay_tight_rec32 | 0.188 | ok |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 3/6, lost 3/6
- **carrier_lay_tight_rec16**: won 3/6, lost 3/6
- **carrier_lay_tight_rec20**: won 3/6, lost 3/6
- **carrier_lay_tight_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 90137.6ns | 11068.1ns | 814.4% | HIGH |
| carrier_lay_tight_rec16 | 90054.8ns | 11177.6ns | 805.7% | HIGH |
| carrier_lay_tight_rec20 | 89629.4ns | 11188.9ns | 801.1% | HIGH |
| carrier_lay_tight_rec24 | 90093.7ns | 11218.5ns | 803.1% | HIGH |
| carrier_lay_tight_rec32 | 89770.6ns | 11225.5ns | 799.7% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 9821.7-11358.1 ns)
   9821.7 |#############
   9898.5 |
   9975.3 |
  10052.2 |
  10129.0 |
  10205.8 |
  10282.6 |
  10359.4 |
  10436.3 |
  10513.1 |
  10589.9 |
  10666.7 |
  10743.5 |
  10820.4 |
  10897.2 |
  10974.0 |
  11050.8 |
  11127.6 |
  11204.5 |#############
  11281.3 |########################################
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 10854.6-11343.8 ns)
  10854.6 |####################
  10879.1 |
  10903.5 |
  10928.0 |
  10952.4 |
  10976.9 |
  11001.3 |
  11025.8 |
  11050.3 |
  11074.7 |
  11099.2 |
  11123.6 |########################################
  11148.1 |
  11172.5 |
  11197.0 |
  11221.5 |####################
  11245.9 |
  11270.4 |
  11294.8 |####################
  11319.3 |
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 11029.6-11257.5 ns)
  11029.6 |####################
  11041.0 |
  11052.4 |
  11063.8 |
  11075.2 |
  11086.6 |
  11098.0 |
  11109.4 |
  11120.8 |
  11132.2 |
  11143.5 |####################
  11154.9 |
  11166.3 |
  11177.7 |
  11189.1 |
  11200.5 |
  11211.9 |########################################
  11223.3 |
  11234.7 |####################
  11246.1 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 11089.6-11333.5 ns)
  11089.6 |########################################
  11101.8 |
  11114.0 |
  11126.2 |
  11138.4 |
  11150.6 |########################################
  11162.8 |
  11175.0 |
  11187.2 |########################################
  11199.4 |########################################
  11211.5 |
  11223.7 |
  11235.9 |
  11248.1 |
  11260.3 |
  11272.5 |
  11284.7 |
  11296.9 |########################################
  11309.1 |
  11321.3 |
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 11111.2-11318.5 ns)
  11111.2 |########################################
  11121.6 |
  11131.9 |
  11142.3 |
  11152.7 |
  11163.0 |########################################
  11173.4 |
  11183.8 |
  11194.1 |########################################
  11204.5 |
  11214.9 |
  11225.2 |
  11235.6 |########################################
  11246.0 |########################################
  11256.3 |
  11266.7 |
  11277.1 |
  11287.4 |
  11297.8 |
  11308.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=797.8% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=803.6% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=800.1% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=804.5% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=799.0% of algo (FFI overhead may distort results)
