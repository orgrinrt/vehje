# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_wideselect_rec32 shows alternating (throttle bounce) (autocorr -0.59)

carrier_lay_wideselect_rec32's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 4.5% of the fastest

All 5 variants sit between 10.74 us and 11.23 us - a 4.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_wideselect_rec20's edge over baseline is significant but tiny (-34 ns, 0.31%)

carrier_lay_wideselect_rec20 differs from baseline carrier_lay_wideselect_rec24 by -34 ns (0.31%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_wideselect_rec16** at 10743.8 ns median (-1.8% vs baseline)
- Spread: 1.05x (fastest 10743.8 ns, slowest 11230.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 13551ns | 13798ns | 11993ns | 13763ns | 14012ns | +0.38% |
| carrier_lay_wideselect_rec16 | 13435ns | 13446ns | 13344ns | 13413ns | 13513ns | -0.49% |
| carrier_lay_wideselect_rec20 | 13463ns | 13417ns | 13343ns | 13411ns | 13603ns | -0.27% |
| carrier_lay_wideselect_rec24 | 13500ns | 13495ns | 13355ns | 13461ns | 13632ns | base |
| carrier_lay_wideselect_rec32 | 13483ns | 13462ns | 13284ns | 13420ns | 13676ns | -0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 11038ns | 9773ns | 11421ns | +0.86% | 0.023 |
| carrier_lay_wideselect_rec16 | 10778ns | 10699ns | 10879ns | -1.52% | 0.024 |
| carrier_lay_wideselect_rec20 | 10894ns | 10750ns | 10983ns | -0.45% | 0.023 |
| carrier_lay_wideselect_rec24 | 10944ns | 10741ns | 11115ns | base | 0.023 |
| carrier_lay_wideselect_rec32 | 10866ns | 10800ns | 10908ns | -0.71% | 0.024 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_wideselect_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.023 | 87.0% |
| carrier_lay_wideselect_rec16 | 0.024 | 91.0% |
| carrier_lay_wideselect_rec20 | 0.024 | 89.8% |
| carrier_lay_wideselect_rec24 | 0.023 | 89.3% |
| carrier_lay_wideselect_rec32 | 0.024 | 89.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 13551ns | 13551ns | +0.38% |
| carrier_lay_wideselect_rec16 | 13435ns | 13435ns | -0.49% |
| carrier_lay_wideselect_rec20 | 13463ns | 13463ns | -0.27% |
| carrier_lay_wideselect_rec24 | 13500ns | 13500ns | base |
| carrier_lay_wideselect_rec32 | 13483ns | 13483ns | -0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 10939ns | base | --- | [10777, 11115] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 11230ns | no significant difference | [-536, +439]ns | [10464, 11421] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 10744ns | no significant difference | [-379, +15]ns | [10710, 10879] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec20 | 10889ns | no significant difference | [-173, +58]ns | [10811, 10983] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_wideselect_rec32 | 10875ns | no significant difference | [-266, +124]ns | [10816, 10908] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 10965ns | -10.9% | -2.1% | -0.3% | -0.9% |
| 2 | 10813ns | +3.4% | +0.2% | -0.6% | +0.7% |
| 3 | 10741ns | +3.8% | -0.4% | +1.2% | +1.6% |
| 4 | 11070ns | +3.5% | -3.1% | -0.4% | -2.4% |
| 5 | 11161ns | +1.1% | -3.7% | -2.5% | -2.4% |
| 6 | 10913ns | +4.3% | +0.1% | -0.1% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.037 | ok |
| carrier_lay_wideselect_rec16 | -0.131 | ok |
| carrier_lay_wideselect_rec20 | -0.189 | ok |
| carrier_lay_wideselect_rec24 | 0.155 | ok |
| carrier_lay_wideselect_rec32 | -0.590 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec16**: won 4/6, lost 1/6
- **carrier_lay_wideselect_rec20**: won 5/6, lost 1/6
- **carrier_lay_wideselect_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 90106.9ns | 11038.1ns | 816.3% | HIGH |
| carrier_lay_wideselect_rec16 | 87513.2ns | 10777.7ns | 812.0% | HIGH |
| carrier_lay_wideselect_rec20 | 88247.9ns | 10894.2ns | 810.0% | HIGH |
| carrier_lay_wideselect_rec24 | 88371.0ns | 10943.9ns | 807.5% | HIGH |
| carrier_lay_wideselect_rec32 | 87896.1ns | 10866.5ns | 808.9% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 9773.3-11420.6 ns)
   9773.3 |########################################
   9855.7 |
   9938.0 |
  10020.4 |
  10102.8 |
  10185.1 |
  10267.5 |
  10349.9 |
  10432.2 |
  10514.6 |
  10597.0 |
  10679.3 |
  10761.7 |
  10844.0 |
  10926.4 |
  11008.8 |
  11091.1 |########################################
  11173.5 |########################################
  11255.9 |########################################
  11338.2 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 10699.2-10878.8 ns)
  10699.2 |########################################
  10708.2 |
  10717.2 |########################################
  10726.1 |
  10735.1 |########################################
  10744.1 |########################################
  10753.1 |
  10762.0 |
  10771.0 |
  10780.0 |
  10789.0 |
  10798.0 |
  10806.9 |
  10815.9 |
  10824.9 |########################################
  10833.9 |
  10842.8 |
  10851.8 |
  10860.8 |
  10869.8 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 10750.4-10982.9 ns)
  10750.4 |########################################
  10762.0 |
  10773.6 |
  10785.3 |
  10796.9 |
  10808.5 |
  10820.1 |
  10831.8 |
  10843.4 |
  10855.0 |
  10866.7 |########################################
  10878.3 |########################################
  10889.9 |########################################
  10901.5 |
  10913.2 |
  10924.8 |########################################
  10936.4 |
  10948.0 |
  10959.7 |
  10971.3 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 10741.2-11115.4 ns)
  10741.2 |########################################
  10759.9 |
  10778.6 |
  10797.3 |########################################
  10816.0 |
  10834.8 |
  10853.5 |
  10872.2 |
  10890.9 |
  10909.6 |########################################
  10928.3 |
  10947.0 |########################################
  10965.7 |
  10984.4 |
  11003.1 |
  11021.9 |
  11040.6 |
  11059.3 |########################################
  11078.0 |
  11096.7 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 10800.4-10907.7 ns)
  10800.4 |########################################
  10805.8 |
  10811.1 |
  10816.5 |
  10821.9 |
  10827.2 |########################################
  10832.6 |
  10838.0 |
  10843.3 |
  10848.7 |
  10854.0 |
  10859.4 |
  10864.8 |########################################
  10870.1 |
  10875.5 |
  10880.9 |########################################
  10886.2 |
  10891.6 |
  10897.0 |########################################
  10902.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=801.5% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=814.7% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=809.3% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=808.1% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=809.0% of algo (FFI overhead may distort results)
