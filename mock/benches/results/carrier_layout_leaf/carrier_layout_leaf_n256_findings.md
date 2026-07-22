# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec16's edge over baseline is significant but tiny (34 ns, 0.35%)

carrier_lay_leaf_rec16 differs from baseline carrier_lay_leaf_rec24 by 34 ns (0.35%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_leaf_rec32** at 9658.3 ns median (-1.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 9658.3 ns, slowest 10454.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 12403ns | 12306ns | 12199ns | 12293ns | 12669ns | +0.05% |
| carrier_lay_leaf_rec16 | 12453ns | 12432ns | 11910ns | 12298ns | 12956ns | +0.45% |
| carrier_lay_leaf_rec20 | 13306ns | 13136ns | 12403ns | 13008ns | 14205ns | +7.34% |
| carrier_lay_leaf_rec24 | 12397ns | 12398ns | 12012ns | 12282ns | 12761ns | base |
| carrier_lay_leaf_rec32 | 12306ns | 12309ns | 12067ns | 12231ns | 12540ns | -0.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 9848ns | 9454ns | 10192ns | +0.51% | 0.026 |
| carrier_lay_leaf_rec16 | 9877ns | 9474ns | 10278ns | +0.81% | 0.026 |
| carrier_lay_leaf_rec20 | 10676ns | 9786ns | 11590ns | +8.96% | 0.024 |
| carrier_lay_leaf_rec24 | 9798ns | 9512ns | 10083ns | base | 0.026 |
| carrier_lay_leaf_rec32 | 9682ns | 9438ns | 9923ns | -1.18% | 0.026 |

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_lay_leaf_rec32; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.026 | 96.1% |
| carrier_lay_leaf_rec16 | 0.026 | 95.9% |
| carrier_lay_leaf_rec20 | 0.024 | 90.3% |
| carrier_lay_leaf_rec24 | 0.026 | 96.5% |
| carrier_lay_leaf_rec32 | 0.027 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 12403ns | 12403ns | +0.05% |
| carrier_lay_leaf_rec16 | 12453ns | 12453ns | +0.45% |
| carrier_lay_leaf_rec20 | 13306ns | 13306ns | +7.34% |
| carrier_lay_leaf_rec24 | 12397ns | 12397ns | base |
| carrier_lay_leaf_rec32 | 12306ns | 12306ns | -0.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 9778ns | base | --- | [9534, 10083] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 9825ns | no significant difference | [-395, +447]ns | [9526, 10192] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_leaf_rec16 | 9838ns | no significant difference | [-507, +711]ns | [9516, 10278] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_leaf_rec20 | 10455ns | +659.4ns (+6.7%) | [+196, +1779]ns | [9985, 11590] | YES (adj: no) | 0.6875 | 0.2188 | 0 |
| carrier_lay_leaf_rec32 | 9658ns | no significant difference | [-579, +342]ns | [9465, 9923] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 10015ns | +1.5% | +0.7% | +24.5% | -5.2% |
| 2 | 9556ns | +0.4% | +0.0% | +6.6% | +2.6% |
| 3 | 9951ns | -5.0% | -3.4% | -1.7% | -1.5% |
| 4 | 10151ns | -2.9% | -6.7% | +5.5% | -6.3% |
| 5 | 9512ns | +7.4% | +10.1% | +11.6% | -0.8% |
| 6 | 9604ns | +2.0% | +4.8% | +7.2% | +4.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | -0.005 | ok |
| carrier_lay_leaf_rec16 | -0.008 | ok |
| carrier_lay_leaf_rec20 | -0.103 | ok |
| carrier_lay_leaf_rec24 | -0.217 | moderate- |
| carrier_lay_leaf_rec32 | -0.269 | moderate- |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 2/6, lost 4/6
- **carrier_lay_leaf_rec16**: won 2/6, lost 3/6
- **carrier_lay_leaf_rec20**: won 1/6, lost 5/6
- **carrier_lay_leaf_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 90880.7ns | 9847.9ns | 922.8% | HIGH |
| carrier_lay_leaf_rec16 | 91668.6ns | 9877.4ns | 928.1% | HIGH |
| carrier_lay_leaf_rec20 | 90760.3ns | 10676.4ns | 850.1% | HIGH |
| carrier_lay_leaf_rec24 | 92138.2ns | 9798.2ns | 940.4% | HIGH |
| carrier_lay_leaf_rec32 | 90750.0ns | 9682.2ns | 937.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 9453.7-10192.5 ns)
   9453.7 |########################################
   9490.6 |
   9527.6 |
   9564.5 |########################################
   9601.5 |
   9638.4 |
   9675.3 |
   9712.3 |
   9749.2 |
   9786.2 |########################################
   9823.1 |########################################
   9860.0 |
   9897.0 |
   9933.9 |
   9970.9 |
  10007.8 |
  10044.7 |
  10081.7 |
  10118.6 |
  10155.6 |########################################
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 9473.8-10278.1 ns)
   9473.8 |########################################
   9514.0 |
   9554.2 |########################################
   9594.4 |########################################
   9634.7 |
   9674.9 |
   9715.1 |
   9755.3 |
   9795.5 |
   9835.7 |
   9875.9 |
   9916.2 |
   9956.4 |
   9996.6 |
  10036.8 |########################################
  10077.0 |########################################
  10117.2 |
  10157.5 |
  10197.7 |
  10237.9 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 9785.8-11589.8 ns)
   9785.8 |########################################
   9876.0 |
   9966.2 |
  10056.4 |
  10146.6 |########################################
  10236.8 |########################################
  10327.0 |
  10417.2 |
  10507.4 |
  10597.6 |########################################
  10687.8 |########################################
  10778.0 |
  10868.2 |
  10958.4 |
  11048.6 |
  11138.8 |
  11229.0 |
  11319.2 |
  11409.4 |
  11499.6 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 9511.7-10083.1 ns)
   9511.7 |########################################
   9540.3 |########################################
   9568.8 |
   9597.4 |########################################
   9626.0 |
   9654.5 |
   9683.1 |
   9711.7 |
   9740.3 |
   9768.8 |
   9797.4 |
   9826.0 |
   9854.5 |
   9883.1 |
   9911.7 |
   9940.2 |########################################
   9968.8 |
   9997.4 |########################################
  10026.0 |
  10054.5 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 9437.9-9923.0 ns)
   9437.9 |########################################
   9462.2 |
   9486.4 |########################################
   9510.7 |########################################
   9534.9 |
   9559.2 |
   9583.4 |
   9607.7 |
   9631.9 |
   9656.2 |
   9680.4 |
   9704.7 |
   9728.9 |
   9753.2 |
   9777.4 |########################################
   9801.7 |########################################
   9825.9 |
   9850.2 |
   9874.4 |
   9898.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=919.8% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=929.0% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=866.7% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=938.8% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=928.1% of algo (FFI overhead may distort results)
