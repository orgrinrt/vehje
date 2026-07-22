# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.8% of the fastest

All 5 variants sit between 12.33 us and 12.54 us - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_madd_rec12's edge over baseline is significant but tiny (-36 ns, 0.29%)

carrier_lay_madd_rec12 differs from baseline carrier_lay_madd_rec24 by -36 ns (0.29%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_madd_rec32** at 12325.8 ns median (-1.3% vs baseline)
- Spread: 1.02x (fastest 12325.8 ns, slowest 12542.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 14705ns | 14933ns | 13082ns | 14904ns | 15217ns | -2.70% |
| carrier_lay_madd_rec16 | 14997ns | 15119ns | 14455ns | 15023ns | 15231ns | -0.77% |
| carrier_lay_madd_rec20 | 15152ns | 15223ns | 14863ns | 15115ns | 15354ns | +0.26% |
| carrier_lay_madd_rec24 | 15113ns | 15096ns | 14723ns | 14992ns | 15491ns | base |
| carrier_lay_madd_rec32 | 14985ns | 14927ns | 14797ns | 14905ns | 15198ns | -0.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 12163ns | 10814ns | 12570ns | -2.57% | 0.021 |
| carrier_lay_madd_rec16 | 12381ns | 11932ns | 12555ns | -0.83% | 0.021 |
| carrier_lay_madd_rec20 | 12511ns | 12248ns | 12710ns | +0.22% | 0.020 |
| carrier_lay_madd_rec24 | 12484ns | 12102ns | 12800ns | base | 0.021 |
| carrier_lay_madd_rec32 | 12373ns | 12217ns | 12529ns | -0.89% | 0.021 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_lay_madd_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 87.4% |
| carrier_lay_madd_rec16 | 0.020 | 86.5% |
| carrier_lay_madd_rec20 | 0.020 | 86.2% |
| carrier_lay_madd_rec24 | 0.020 | 86.6% |
| carrier_lay_madd_rec32 | 0.021 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 14705ns | 14705ns | -2.70% |
| carrier_lay_madd_rec16 | 14997ns | 14997ns | -0.77% |
| carrier_lay_madd_rec20 | 15152ns | 15152ns | +0.26% |
| carrier_lay_madd_rec24 | 15113ns | 15113ns | base |
| carrier_lay_madd_rec32 | 14985ns | 14985ns | -0.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 12493ns | base | --- | [12159, 12800] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 12373ns | no significant difference | [-1138, +210]ns | [11545, 12570] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_madd_rec16 | 12501ns | no significant difference | [-358, +199]ns | [12086, 12555] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_madd_rec20 | 12542ns | no significant difference | [-376, +282]ns | [12282, 12710] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 12326ns | no significant difference | [-322, +156]ns | [12264, 12529] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 12914ns | -16.3% | -3.4% | -2.4% | -2.1% |
| 2 | 12610ns | +0.2% | -0.5% | +1.1% | -2.3% |
| 3 | 12686ns | -1.4% | -1.3% | -3.5% | -2.8% |
| 4 | 12376ns | -0.7% | -1.1% | +2.4% | -0.5% |
| 5 | 12216ns | +2.0% | -2.3% | +2.2% | +1.6% |
| 6 | 12102ns | +1.4% | +3.8% | +1.8% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.157 | ok |
| carrier_lay_madd_rec16 | -0.006 | ok |
| carrier_lay_madd_rec20 | -0.416 | moderate- |
| carrier_lay_madd_rec24 | 0.402 | moderate+ |
| carrier_lay_madd_rec32 | -0.171 | ok |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 3/6, lost 3/6
- **carrier_lay_madd_rec16**: won 5/6, lost 1/6
- **carrier_lay_madd_rec20**: won 2/6, lost 4/6
- **carrier_lay_madd_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 90364.2ns | 12162.6ns | 743.0% | HIGH |
| carrier_lay_madd_rec16 | 90275.6ns | 12380.8ns | 729.2% | HIGH |
| carrier_lay_madd_rec20 | 90901.5ns | 12511.2ns | 726.6% | HIGH |
| carrier_lay_madd_rec24 | 92040.0ns | 12483.9ns | 737.3% | HIGH |
| carrier_lay_madd_rec32 | 92333.9ns | 12372.9ns | 746.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 10814.2-12570.2 ns)
  10814.2 |####################
  10902.0 |
  10989.8 |
  11077.6 |
  11165.4 |
  11253.2 |
  11341.0 |
  11428.8 |
  11516.6 |
  11604.4 |
  11692.2 |
  11780.0 |
  11867.8 |
  11955.6 |
  12043.4 |
  12131.2 |
  12219.0 |########################################
  12306.8 |
  12394.6 |####################
  12482.4 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 11932.1-12555.0 ns)
  11932.1 |########################################
  11963.2 |
  11994.4 |
  12025.5 |
  12056.7 |
  12087.8 |
  12119.0 |
  12150.1 |
  12181.3 |
  12212.4 |########################################
  12243.5 |
  12274.7 |
  12305.8 |
  12337.0 |
  12368.1 |
  12399.3 |
  12430.4 |
  12461.6 |########################################
  12492.7 |########################################
  12523.9 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 12248.3-12709.5 ns)
  12248.3 |########################################
  12271.4 |
  12294.4 |########################################
  12317.5 |
  12340.5 |
  12363.6 |
  12386.7 |
  12409.7 |
  12432.8 |
  12455.9 |
  12478.9 |########################################
  12502.0 |
  12525.0 |
  12548.1 |
  12571.2 |
  12594.2 |########################################
  12617.3 |
  12640.4 |
  12663.4 |########################################
  12686.5 |
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 12101.7-12800.0 ns)
  12101.7 |########################################
  12136.6 |
  12171.5 |
  12206.4 |########################################
  12241.4 |
  12276.3 |
  12311.2 |
  12346.1 |########################################
  12381.0 |
  12415.9 |
  12450.9 |
  12485.8 |
  12520.7 |
  12555.6 |
  12590.5 |########################################
  12625.4 |
  12660.3 |########################################
  12695.3 |
  12730.2 |
  12765.1 |
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 12217.1-12529.2 ns)
  12217.1 |########################################
  12232.7 |
  12248.3 |
  12263.9 |
  12279.5 |
  12295.1 |########################################
  12310.7 |########################################
  12326.3 |########################################
  12341.9 |
  12357.5 |
  12373.2 |
  12388.8 |
  12404.4 |########################################
  12420.0 |
  12435.6 |
  12451.2 |
  12466.8 |
  12482.4 |
  12498.0 |
  12513.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=713.7% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=716.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=720.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=738.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=749.1% of algo (FFI overhead may distort results)
