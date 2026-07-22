# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_tight_rec20 shows alternating (throttle bounce) (autocorr -0.59)

carrier_lay_tight_rec20's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (561 ns) is smaller than the fastest variant's own run-to-run std-dev (1.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.2% of the fastest

All 5 variants sit between 45.61 us and 46.17 us - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_tight_rec16's edge over baseline is significant but tiny (-2 ns, 0.00%)

carrier_lay_tight_rec16 differs from baseline carrier_lay_tight_rec24 by -2 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_tight_rec12** at 45613.9 ns median (-0.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 45613.9 ns, slowest 46174.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 47648ns | 48157ns | 44726ns | 48081ns | 48461ns | -1.68% |
| carrier_lay_tight_rec16 | 48234ns | 48552ns | 46307ns | 48530ns | 48753ns | -0.48% |
| carrier_lay_tight_rec20 | 48413ns | 48509ns | 47789ns | 48339ns | 48838ns | -0.11% |
| carrier_lay_tight_rec24 | 48465ns | 48527ns | 47896ns | 48452ns | 48768ns | base |
| carrier_lay_tight_rec32 | 48562ns | 48766ns | 47321ns | 48649ns | 49053ns | +0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 45172ns | 42368ns | 45982ns | -1.62% | 0.023 |
| carrier_lay_tight_rec16 | 45715ns | 43908ns | 46207ns | -0.43% | 0.022 |
| carrier_lay_tight_rec20 | 45902ns | 45452ns | 46327ns | -0.02% | 0.022 |
| carrier_lay_tight_rec24 | 45914ns | 45391ns | 46260ns | base | 0.022 |
| carrier_lay_tight_rec32 | 45960ns | 44838ns | 46389ns | +0.10% | 0.022 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_lay_tight_rec12; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.022 | 92.9% |
| carrier_lay_tight_rec16 | 0.022 | 92.0% |
| carrier_lay_tight_rec20 | 0.022 | 92.3% |
| carrier_lay_tight_rec24 | 0.022 | 92.4% |
| carrier_lay_tight_rec32 | 0.022 | 91.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 47648ns | 47648ns | -1.68% |
| carrier_lay_tight_rec16 | 48234ns | 48234ns | -0.48% |
| carrier_lay_tight_rec20 | 48413ns | 48413ns | -0.11% |
| carrier_lay_tight_rec24 | 48465ns | 48465ns | base |
| carrier_lay_tight_rec32 | 48562ns | 48562ns | +0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 45868ns | base | --- | [45613, 46260] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 45614ns | -244.8ns (-0.5%) | [-1896, -85]ns | [43920, 45982] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_tight_rec16 | 46043ns | no significant difference | [-839, +246]ns | [44896, 46207] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec20 | 45903ns | no significant difference | [-357, +265]ns | [45478, 46327] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 46175ns | no significant difference | [-765, +767]ns | [45315, 46389] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 45391ns | -6.7% | -3.3% | +0.3% | +2.5% |
| 2 | 45883ns | -0.8% | +0.6% | +0.5% | -0.2% |
| 3 | 45835ns | -0.3% | +0.4% | -0.8% | +0.6% |
| 4 | 46241ns | -1.7% | -0.4% | +0.7% | +0.0% |
| 5 | 45854ns | -0.1% | +0.1% | +0.0% | +0.9% |
| 6 | 46279ns | -0.3% | -0.1% | -0.7% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | 0.015 | ok |
| carrier_lay_tight_rec16 | -0.105 | ok |
| carrier_lay_tight_rec20 | -0.594 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec24 | -0.093 | ok |
| carrier_lay_tight_rec32 | -0.181 | ok |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 6/6, lost 0/6
- **carrier_lay_tight_rec16**: won 2/6, lost 2/6
- **carrier_lay_tight_rec20**: won 2/6, lost 3/6
- **carrier_lay_tight_rec32**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 97307.2ns | 45171.8ns | 215.4% | HIGH |
| carrier_lay_tight_rec16 | 92454.7ns | 45715.5ns | 202.2% | HIGH |
| carrier_lay_tight_rec20 | 91939.5ns | 45902.5ns | 200.3% | HIGH |
| carrier_lay_tight_rec24 | 92012.4ns | 45913.6ns | 200.4% | HIGH |
| carrier_lay_tight_rec32 | 92098.9ns | 45959.8ns | 200.4% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 42367.9-45981.7 ns)
  42367.9 |####################
  42548.6 |
  42729.3 |
  42910.0 |
  43090.7 |
  43271.3 |
  43452.0 |
  43632.7 |
  43813.4 |
  43994.1 |
  44174.8 |
  44355.5 |
  44536.2 |
  44716.8 |
  44897.5 |
  45078.2 |
  45258.9 |
  45439.6 |########################################
  45620.3 |####################
  45801.0 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 43908.3-46207.3 ns)
  43908.3 |####################
  44023.2 |
  44138.2 |
  44253.2 |
  44368.1 |
  44483.1 |
  44598.0 |
  44713.0 |
  44827.9 |
  44942.9 |
  45057.8 |
  45172.8 |
  45287.7 |
  45402.7 |
  45517.6 |
  45632.6 |
  45747.5 |
  45862.5 |####################
  45977.4 |########################################
  46092.4 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 45451.7-46326.7 ns)
  45451.7 |########################################
  45495.4 |########################################
  45539.2 |
  45582.9 |
  45626.7 |
  45670.4 |
  45714.2 |
  45757.9 |
  45801.7 |
  45845.4 |########################################
  45889.2 |
  45932.9 |########################################
  45976.7 |
  46020.4 |
  46064.2 |########################################
  46107.9 |
  46151.7 |
  46195.4 |
  46239.2 |
  46282.9 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 45390.8-46259.8 ns)
  45390.8 |####################
  45434.2 |
  45477.7 |
  45521.2 |
  45564.6 |
  45608.1 |
  45651.5 |
  45695.0 |
  45738.4 |
  45781.9 |
  45825.3 |########################################
  45868.8 |####################
  45912.2 |
  45955.7 |
  45999.1 |
  46042.6 |
  46086.0 |
  46129.5 |
  46172.9 |
  46216.4 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 44837.5-46389.4 ns)
  44837.5 |####################
  44915.1 |
  44992.7 |
  45070.3 |
  45147.9 |
  45225.5 |
  45303.1 |
  45380.7 |
  45458.3 |
  45535.9 |
  45613.4 |
  45691.0 |
  45768.6 |####################
  45846.2 |
  45923.8 |
  46001.4 |
  46079.0 |####################
  46156.6 |
  46234.2 |########################################
  46311.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=201.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=200.8% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=200.3% of algo (FFI overhead may distort results)
