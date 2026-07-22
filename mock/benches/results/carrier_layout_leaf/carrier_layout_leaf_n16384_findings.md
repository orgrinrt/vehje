# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec16 shows alternating (throttle bounce) (autocorr -0.65)

carrier_lay_leaf_rec16's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.6% of the fastest

All 5 variants sit between 1.13 ms and 1.13 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_leaf_rec12** at 1125618.5 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 1125618.5 ns, slowest 1132470.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1129603ns | 1128662ns | 1125814ns | 1128344ns | 1133386ns | -0.36% |
| carrier_lay_leaf_rec16 | 1138238ns | 1135713ns | 1133178ns | 1135059ns | 1145538ns | +0.40% |
| carrier_lay_leaf_rec20 | 1133981ns | 1133077ns | 1129026ns | 1131914ns | 1139558ns | +0.03% |
| carrier_lay_leaf_rec24 | 1133678ns | 1133998ns | 1130477ns | 1133473ns | 1135584ns | base |
| carrier_lay_leaf_rec32 | 1135616ns | 1134015ns | 1131479ns | 1133764ns | 1140462ns | +0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1126565ns | 1123327ns | 1130299ns | -0.36% | 0.015 |
| carrier_lay_leaf_rec16 | 1135175ns | 1129784ns | 1142622ns | +0.40% | 0.014 |
| carrier_lay_leaf_rec20 | 1131075ns | 1126398ns | 1136741ns | +0.03% | 0.014 |
| carrier_lay_leaf_rec24 | 1130683ns | 1128000ns | 1132447ns | base | 0.014 |
| carrier_lay_leaf_rec32 | 1132517ns | 1128925ns | 1137065ns | +0.16% | 0.014 |

## Performance model

- Peak throughput: **0.015 Gops/s** (carrier_lay_leaf_rec12; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.015 | 99.8% |
| carrier_lay_leaf_rec16 | 0.014 | 99.2% |
| carrier_lay_leaf_rec20 | 0.015 | 99.4% |
| carrier_lay_leaf_rec24 | 0.014 | 99.3% |
| carrier_lay_leaf_rec32 | 0.014 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 1129603ns | 1129603ns | -0.36% |
| carrier_lay_leaf_rec16 | 1138238ns | 1138238ns | +0.40% |
| carrier_lay_leaf_rec20 | 1133981ns | 1133981ns | +0.03% |
| carrier_lay_leaf_rec24 | 1133678ns | 1133678ns | base |
| carrier_lay_leaf_rec32 | 1135616ns | 1135616ns | +0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 1130801ns | base | --- | [1128802, 1132447] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 1125618ns | -4015.8ns (-0.4%) | [-6227, -2112]ns | [1123777, 1130299] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_lay_leaf_rec16 | 1132470ns | +2613.7ns (+0.2%) | [+686, +10175]ns | [1130433, 1142622] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_lay_leaf_rec20 | 1129835ns | no significant difference | [-5422, +6771]ns | [1126649, 1136741] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec32 | 1131083ns | no significant difference | [-1461, +6025]ns | [1129404, 1137065] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 1129604ns | -0.4% | +0.0% | +0.2% | +0.9% |
| 2 | 1131308ns | -0.3% | +1.1% | -0.2% | -0.2% |
| 3 | 1130555ns | -0.4% | +0.1% | -0.4% | +0.1% |
| 4 | 1133587ns | -0.0% | +0.7% | -0.6% | +0.1% |
| 5 | 1128000ns | -0.3% | +0.3% | +0.2% | +0.2% |
| 6 | 1131047ns | -0.7% | +0.2% | +1.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | -0.233 | moderate- |
| carrier_lay_leaf_rec16 | -0.652 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec20 | 0.139 | ok |
| carrier_lay_leaf_rec24 | -0.571 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec32 | -0.299 | moderate- |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 5/6, lost 0/6
- **carrier_lay_leaf_rec16**: won 0/6, lost 5/6
- **carrier_lay_leaf_rec20**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec32**: won 1/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1128644.1ns | 1126565.0ns | 100.2% | HIGH |
| carrier_lay_leaf_rec16 | 1137197.7ns | 1135175.2ns | 100.2% | HIGH |
| carrier_lay_leaf_rec20 | 1133234.7ns | 1131075.4ns | 100.2% | HIGH |
| carrier_lay_leaf_rec24 | 1132607.7ns | 1130683.5ns | 100.2% | HIGH |
| carrier_lay_leaf_rec32 | 1134246.4ns | 1132517.0ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 1123327.1-1130299.4 ns)
  1123327.1 |########################################
  1123675.7 |
  1124024.3 |########################################
  1124372.9 |
  1124721.6 |
  1125070.2 |########################################
  1125418.8 |
  1125767.4 |########################################
  1126116.0 |
  1126464.6 |
  1126813.2 |
  1127161.9 |########################################
  1127510.5 |
  1127859.1 |
  1128207.7 |
  1128556.3 |
  1128904.9 |
  1129253.6 |
  1129602.2 |
  1129950.8 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 1129784.2-1142622.5 ns)
  1129784.2 |########################################
  1130426.1 |
  1131068.0 |########################################
  1131709.9 |########################################
  1132351.9 |
  1132993.8 |########################################
  1133635.7 |
  1134277.6 |
  1134919.5 |
  1135561.4 |
  1136203.4 |
  1136845.3 |
  1137487.2 |
  1138129.1 |
  1138771.0 |
  1139412.9 |
  1140054.8 |
  1140696.8 |########################################
  1141338.7 |
  1141980.6 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 1126397.5-1136741.5 ns)
  1126397.5 |########################################
  1126914.7 |
  1127431.9 |
  1127949.1 |
  1128466.3 |
  1128983.5 |####################
  1129500.7 |
  1130017.9 |####################
  1130535.1 |
  1131052.3 |####################
  1131569.5 |
  1132086.7 |
  1132603.9 |
  1133121.1 |
  1133638.3 |
  1134155.5 |
  1134672.7 |
  1135189.9 |
  1135707.1 |
  1136224.3 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 1128000.0-1132447.3 ns)
  1128000.0 |########################################
  1128222.4 |
  1128444.7 |
  1128667.1 |
  1128889.5 |
  1129111.8 |
  1129334.2 |
  1129556.6 |########################################
  1129778.9 |
  1130001.3 |
  1130223.6 |
  1130446.0 |########################################
  1130668.4 |
  1130890.7 |########################################
  1131113.1 |########################################
  1131335.5 |
  1131557.8 |
  1131780.2 |
  1132002.6 |
  1132224.9 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 1128925.4-1137064.5 ns)
  1128925.4 |########################################
  1129332.4 |
  1129739.3 |########################################
  1130146.3 |########################################
  1130553.2 |
  1130960.2 |
  1131367.1 |########################################
  1131774.1 |
  1132181.1 |
  1132588.0 |
  1132995.0 |
  1133401.9 |
  1133808.9 |
  1134215.8 |########################################
  1134622.8 |
  1135029.8 |
  1135436.7 |
  1135843.7 |
  1136250.6 |
  1136657.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=100.2% of algo (FFI overhead may distort results)
