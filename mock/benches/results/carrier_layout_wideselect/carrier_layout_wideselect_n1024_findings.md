# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 3.0% of the fastest

All 5 variants sit between 41.99 us and 43.24 us - a 3.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_wideselect_rec20** at 41986.0 ns median (-0.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 41986.0 ns, slowest 43242.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 45359ns | 45882ns | 42508ns | 45854ns | 46043ns | +1.96% |
| carrier_lay_wideselect_rec16 | 44549ns | 44805ns | 43311ns | 44322ns | 45508ns | +0.14% |
| carrier_lay_wideselect_rec20 | 44633ns | 44489ns | 43825ns | 44450ns | 45312ns | +0.33% |
| carrier_lay_wideselect_rec24 | 44487ns | 44502ns | 42899ns | 44308ns | 45550ns | base |
| carrier_lay_wideselect_rec32 | 44407ns | 44545ns | 43765ns | 44392ns | 44749ns | -0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 42800ns | 40138ns | 43543ns | +1.75% | 0.024 |
| carrier_lay_wideselect_rec16 | 41946ns | 40721ns | 42899ns | -0.28% | 0.024 |
| carrier_lay_wideselect_rec20 | 42094ns | 41201ns | 42743ns | +0.07% | 0.024 |
| carrier_lay_wideselect_rec24 | 42066ns | 40624ns | 42935ns | base | 0.024 |
| carrier_lay_wideselect_rec32 | 41927ns | 41248ns | 42236ns | -0.33% | 0.024 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_wideselect_rec12; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.024 | 92.8% |
| carrier_lay_wideselect_rec16 | 0.024 | 95.2% |
| carrier_lay_wideselect_rec20 | 0.024 | 95.6% |
| carrier_lay_wideselect_rec24 | 0.024 | 95.2% |
| carrier_lay_wideselect_rec32 | 0.024 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 45359ns | 45359ns | +1.96% |
| carrier_lay_wideselect_rec16 | 44549ns | 44549ns | +0.14% |
| carrier_lay_wideselect_rec20 | 44633ns | 44633ns | +0.33% |
| carrier_lay_wideselect_rec24 | 44487ns | 44487ns | base |
| carrier_lay_wideselect_rec32 | 44407ns | 44407ns | -0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 42148ns | base | --- | [41114, 42935] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 43242ns | +813.8ns (+1.9%) | [+0, +1389]ns | [41615, 43543] | YES (adj: no) | 0.6875 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 42154ns | no significant difference | [-639, +555]ns | [40785, 42899] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec20 | 41986ns | no significant difference | [-598, +942]ns | [41554, 42743] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec32 | 42197ns | no significant difference | [-735, +429]ns | [41348, 42236] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 40624ns | -1.2% | +0.6% | +3.2% | +1.5% |
| 2 | 43156ns | +1.1% | -0.5% | -0.6% | -2.2% |
| 3 | 42312ns | +2.1% | -0.8% | -0.6% | -0.1% |
| 4 | 41983ns | +2.6% | +2.1% | +1.4% | +0.6% |
| 5 | 42713ns | +1.7% | -0.9% | -1.9% | -1.3% |
| 6 | 41605ns | +4.0% | -2.1% | -1.0% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | -0.148 | ok |
| carrier_lay_wideselect_rec16 | -0.238 | moderate- |
| carrier_lay_wideselect_rec20 | -0.079 | ok |
| carrier_lay_wideselect_rec24 | -0.422 | moderate- |
| carrier_lay_wideselect_rec32 | -0.057 | ok |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec16**: won 4/6, lost 2/6
- **carrier_lay_wideselect_rec20**: won 4/6, lost 2/6
- **carrier_lay_wideselect_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 115963.1ns | 42799.9ns | 270.9% | HIGH |
| carrier_lay_wideselect_rec16 | 113518.0ns | 41946.0ns | 270.6% | HIGH |
| carrier_lay_wideselect_rec20 | 113285.7ns | 42094.3ns | 269.1% | HIGH |
| carrier_lay_wideselect_rec24 | 112091.2ns | 42065.5ns | 266.5% | HIGH |
| carrier_lay_wideselect_rec32 | 113737.8ns | 41927.2ns | 271.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 40137.9-43542.7 ns)
  40137.9 |####################
  40308.1 |
  40478.4 |
  40648.6 |
  40818.9 |
  40989.1 |
  41159.3 |
  41329.6 |
  41499.8 |
  41670.1 |
  41840.3 |
  42010.5 |
  42180.8 |
  42351.0 |
  42521.3 |
  42691.5 |
  42861.7 |
  43032.0 |####################
  43202.2 |########################################
  43372.5 |####################
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 40721.2-42898.6 ns)
  40721.2 |########################################
  40830.1 |########################################
  40938.9 |
  41047.8 |
  41156.7 |
  41265.5 |
  41374.4 |
  41483.3 |
  41592.1 |
  41701.0 |
  41809.9 |
  41918.7 |########################################
  42027.6 |
  42136.5 |
  42245.3 |########################################
  42354.2 |
  42463.1 |
  42571.9 |
  42680.8 |
  42789.7 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 41201.2-42743.3 ns)
  41201.2 |####################
  41278.3 |
  41355.4 |
  41432.5 |
  41509.6 |
  41586.7 |
  41663.8 |
  41741.0 |
  41818.1 |
  41895.2 |########################################
  41972.3 |
  42049.4 |####################
  42126.5 |
  42203.6 |
  42280.7 |
  42357.8 |
  42434.9 |
  42512.0 |####################
  42589.1 |
  42666.2 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 40623.8-42934.8 ns)
  40623.8 |########################################
  40739.3 |
  40854.9 |
  40970.4 |
  41086.0 |
  41201.5 |
  41317.1 |
  41432.6 |
  41548.2 |########################################
  41663.7 |
  41779.3 |
  41894.8 |########################################
  42010.4 |
  42125.9 |
  42241.5 |########################################
  42357.0 |
  42472.6 |
  42588.1 |
  42703.7 |########################################
  42819.2 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 41247.9-42236.4 ns)
  41247.9 |####################
  41297.3 |
  41346.8 |
  41396.2 |
  41445.6 |####################
  41495.0 |
  41544.5 |
  41593.9 |
  41643.3 |
  41692.7 |
  41742.2 |
  41791.6 |
  41841.0 |
  41890.5 |
  41939.9 |
  41989.3 |
  42038.7 |
  42088.2 |
  42137.6 |####################
  42187.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=267.0% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=267.4% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=268.0% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=265.5% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=270.1% of algo (FFI overhead may distort results)
