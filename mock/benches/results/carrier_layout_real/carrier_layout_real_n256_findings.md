# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.3% of the fastest

All 5 variants sit between 11.18 us and 11.33 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_real_rec16's edge over baseline is significant but tiny (-18 ns, 0.16%)

carrier_lay_real_rec16 differs from baseline carrier_lay_real_rec24 by -18 ns (0.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_real_rec16** at 11175.2 ns median (-0.3% vs baseline)
- Spread: 1.01x (fastest 11175.2 ns, slowest 11325.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 13547ns | 13844ns | 11992ns | 13819ns | 13918ns | -1.83% |
| carrier_lay_real_rec16 | 13755ns | 13771ns | 13533ns | 13717ns | 13924ns | -0.32% |
| carrier_lay_real_rec20 | 13810ns | 13803ns | 13675ns | 13765ns | 13945ns | +0.08% |
| carrier_lay_real_rec24 | 13799ns | 13789ns | 13647ns | 13762ns | 13931ns | base |
| carrier_lay_real_rec32 | 13870ns | 13791ns | 13632ns | 13757ns | 14157ns | +0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 11095ns | 9838ns | 11394ns | -1.31% | 0.023 |
| carrier_lay_real_rec16 | 11215ns | 11086ns | 11363ns | -0.25% | 0.023 |
| carrier_lay_real_rec20 | 11238ns | 11205ns | 11292ns | -0.04% | 0.023 |
| carrier_lay_real_rec24 | 11243ns | 11183ns | 11333ns | base | 0.023 |
| carrier_lay_real_rec32 | 11221ns | 11164ns | 11298ns | -0.19% | 0.023 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_real_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.023 | 86.9% |
| carrier_lay_real_rec16 | 0.023 | 88.0% |
| carrier_lay_real_rec20 | 0.023 | 87.7% |
| carrier_lay_real_rec24 | 0.023 | 87.8% |
| carrier_lay_real_rec32 | 0.023 | 87.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 13547ns | 13547ns | -1.83% |
| carrier_lay_real_rec16 | 13755ns | 13755ns | -0.32% |
| carrier_lay_real_rec20 | 13810ns | 13810ns | +0.08% |
| carrier_lay_real_rec24 | 13799ns | 13799ns | base |
| carrier_lay_real_rec32 | 13870ns | 13870ns | +0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 11211ns | base | --- | [11185, 11333] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 11325ns | no significant difference | [-766, +209]ns | [10567, 11394] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec16 | 11175ns | no significant difference | [-159, +92]ns | [11106, 11363] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec20 | 11214ns | no significant difference | [-124, +92]ns | [11207, 11292] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec32 | 11201ns | no significant difference | [-144, +95]ns | [11165, 11298] | no | 0.6875 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 11342ns | -13.3% | -1.9% | -1.1% | -1.5% |
| 2 | 11218ns | +1.1% | +0.7% | +0.7% | -0.1% |
| 3 | 11324ns | -0.2% | +0.9% | -1.1% | -1.0% |
| 4 | 11204ns | +1.0% | -0.1% | +0.1% | -0.1% |
| 5 | 11187ns | +2.0% | -0.9% | +0.2% | -0.2% |
| 6 | 11183ns | +1.7% | -0.3% | +1.0% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.029 | ok |
| carrier_lay_real_rec16 | 0.205 | moderate+ |
| carrier_lay_real_rec20 | -0.367 | moderate- |
| carrier_lay_real_rec24 | -0.086 | ok |
| carrier_lay_real_rec32 | -0.198 | ok |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 2/6, lost 4/6
- **carrier_lay_real_rec16**: won 3/6, lost 2/6
- **carrier_lay_real_rec20**: won 2/6, lost 4/6
- **carrier_lay_real_rec32**: won 3/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 90558.2ns | 11095.3ns | 816.2% | HIGH |
| carrier_lay_real_rec16 | 89889.3ns | 11214.6ns | 801.5% | HIGH |
| carrier_lay_real_rec20 | 90003.9ns | 11237.9ns | 800.9% | HIGH |
| carrier_lay_real_rec24 | 89987.0ns | 11242.8ns | 800.4% | HIGH |
| carrier_lay_real_rec32 | 89782.2ns | 11221.2ns | 800.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 9837.9-11393.8 ns)
   9837.9 |####################
   9915.7 |
   9993.5 |
  10071.3 |
  10149.1 |
  10226.9 |
  10304.7 |
  10382.4 |
  10460.2 |
  10538.0 |
  10615.8 |
  10693.6 |
  10771.4 |
  10849.2 |
  10927.0 |
  11004.8 |
  11082.6 |
  11160.4 |
  11238.2 |########################################
  11316.0 |########################################
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 11086.2-11363.0 ns)
  11086.2 |########################################
  11100.0 |
  11113.9 |########################################
  11127.7 |
  11141.6 |########################################
  11155.4 |
  11169.2 |
  11183.1 |########################################
  11196.9 |
  11210.7 |
  11224.6 |
  11238.4 |
  11252.2 |
  11266.1 |
  11279.9 |
  11293.8 |########################################
  11307.6 |
  11321.4 |
  11335.3 |
  11349.1 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 11204.6-11292.5 ns)
  11204.6 |####################
  11209.0 |########################################
  11213.4 |####################
  11217.8 |
  11222.2 |
  11226.6 |
  11231.0 |
  11235.4 |
  11239.8 |
  11244.2 |
  11248.5 |
  11252.9 |
  11257.3 |
  11261.7 |
  11266.1 |
  11270.5 |
  11274.9 |
  11279.3 |
  11283.7 |
  11288.1 |####################
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 11182.9-11333.1 ns)
  11182.9 |########################################
  11190.4 |
  11197.9 |####################
  11205.4 |
  11212.9 |####################
  11220.5 |
  11228.0 |
  11235.5 |
  11243.0 |
  11250.5 |
  11258.0 |
  11265.5 |
  11273.0 |
  11280.5 |
  11288.0 |
  11295.5 |
  11303.1 |
  11310.6 |
  11318.1 |####################
  11325.6 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 11163.8-11297.7 ns)
  11163.8 |########################################
  11170.5 |
  11177.2 |
  11183.9 |
  11190.6 |####################
  11197.3 |
  11204.0 |####################
  11210.7 |####################
  11217.4 |
  11224.1 |
  11230.8 |
  11237.4 |
  11244.1 |
  11250.8 |
  11257.5 |
  11264.2 |
  11270.9 |
  11277.6 |
  11284.3 |
  11291.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=801.0% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=802.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=801.3% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=802.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=801.7% of algo (FFI overhead may distort results)
