# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_scatter_rec12 shows alternating (throttle bounce) (autocorr -0.67)

carrier_lay_scatter_rec12's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (230 ns) is smaller than the fastest variant's own run-to-run std-dev (280 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### carrier_lay_scatter_rec20's comparison is tie-heavy (17% tied pairs)

17% of paired samples for carrier_lay_scatter_rec20 are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Whole field within 2.2% of the fastest

All 5 variants sit between 10.49 us and 10.72 us - a 2.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_scatter_rec20's edge over baseline is significant but tiny (18 ns, 0.17%)

carrier_lay_scatter_rec20 differs from baseline carrier_lay_scatter_rec24 by 18 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_scatter_rec16** at 10486.0 ns median (-2.0% vs baseline)
- Spread: 1.02x (fastest 10486.0 ns, slowest 10715.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 12961ns | 13070ns | 12412ns | 12909ns | 13313ns | -0.90% |
| carrier_lay_scatter_rec16 | 12804ns | 12811ns | 12143ns | 12788ns | 13160ns | -2.09% |
| carrier_lay_scatter_rec20 | 13864ns | 13117ns | 12579ns | 13011ns | 15785ns | +6.01% |
| carrier_lay_scatter_rec24 | 13078ns | 13163ns | 12564ns | 13056ns | 13368ns | base |
| carrier_lay_scatter_rec32 | 13006ns | 13016ns | 12516ns | 12959ns | 13322ns | -0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 10549ns | 10146ns | 10818ns | -1.02% | 0.024 |
| carrier_lay_scatter_rec16 | 10473ns | 9973ns | 10773ns | -1.73% | 0.024 |
| carrier_lay_scatter_rec20 | 11191ns | 10289ns | 12461ns | +5.00% | 0.023 |
| carrier_lay_scatter_rec24 | 10658ns | 10167ns | 10920ns | base | 0.024 |
| carrier_lay_scatter_rec32 | 10586ns | 10286ns | 10807ns | -0.67% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 313997 | 1449462 | 0.217 | 1.03× |
| carrier_lay_scatter_rec16 | 312031 | 1434174 | 0.218 | 1.02× |
| carrier_lay_scatter_rec20 | 307719 | 1398039 | 0.220 | 1.01× |
| carrier_lay_scatter_rec24 | 304430 | 1402208 | 0.217 | 1.00× |
| carrier_lay_scatter_rec32 | 310438 | 1425830 | 0.218 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_scatter_rec16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.024 | 94.2% |
| carrier_lay_scatter_rec16 | 0.024 | 95.1% |
| carrier_lay_scatter_rec20 | 0.024 | 93.1% |
| carrier_lay_scatter_rec24 | 0.024 | 93.2% |
| carrier_lay_scatter_rec32 | 0.024 | 94.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 12961ns | 12961ns | -0.90% |
| carrier_lay_scatter_rec16 | 12804ns | 12804ns | -2.09% |
| carrier_lay_scatter_rec20 | 13864ns | 13864ns | +6.01% |
| carrier_lay_scatter_rec24 | 13078ns | 13078ns | base |
| carrier_lay_scatter_rec32 | 13006ns | 13006ns | -0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 10698ns | base | --- | [10356, 10920] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 10582ns | no significant difference | [-384, +121]ns | [10246, 10818] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 10486ns | no significant difference | [-667, +240]ns | [10161, 10773] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec20 | 10716ns | no significant difference | [-523, +2105]ns | [10397, 12461] | no | 1.0000 | 1.0000 | **1** (17%, HIGH) |
| carrier_lay_scatter_rec32 | 10612ns | no significant difference | [-389, +253]ns | [10339, 10807] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 10167ns | -0.2% | +1.8% | +5.7% | +3.8% |
| 2 | 10716ns | +1.6% | -2.9% | +0.0% | +1.1% |
| 3 | 10545ns | -1.0% | +2.8% | +34.5% | -2.5% |
| 4 | 10848ns | -1.1% | -2.5% | -5.1% | -4.2% |
| 5 | 10992ns | -5.9% | -9.3% | -4.4% | -2.9% |
| 6 | 10680ns | +0.6% | +0.2% | +0.3% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.672 | HIGH- (thermal bounce) |
| carrier_lay_scatter_rec16 | -0.309 | moderate- |
| carrier_lay_scatter_rec20 | -0.272 | moderate- |
| carrier_lay_scatter_rec24 | 0.036 | ok |
| carrier_lay_scatter_rec32 | -0.097 | ok |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 4/6, lost 2/6
- **carrier_lay_scatter_rec16**: won 3/6, lost 3/6
- **carrier_lay_scatter_rec20**: won 2/6, lost 3/6
- **carrier_lay_scatter_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 92154.7ns | 10548.9ns | 873.6% | HIGH |
| carrier_lay_scatter_rec16 | 91642.9ns | 10473.2ns | 875.0% | HIGH |
| carrier_lay_scatter_rec20 | 92552.0ns | 11191.1ns | 827.0% | HIGH |
| carrier_lay_scatter_rec24 | 90881.3ns | 10657.8ns | 852.7% | HIGH |
| carrier_lay_scatter_rec32 | 92000.2ns | 10586.2ns | 869.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 10146.2-10818.5 ns)
  10146.2 |####################
  10179.8 |
  10213.4 |
  10247.0 |
  10280.7 |
  10314.3 |####################
  10347.9 |
  10381.5 |
  10415.1 |####################
  10448.7 |
  10482.4 |
  10516.0 |
  10549.6 |
  10583.2 |
  10616.8 |
  10650.4 |
  10684.0 |
  10717.7 |########################################
  10751.3 |
  10784.9 |
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 9973.3-10772.7 ns)
   9973.3 |########################################
  10013.3 |
  10053.2 |
  10093.2 |
  10133.2 |
  10173.1 |
  10213.1 |
  10253.1 |
  10293.1 |
  10333.0 |########################################
  10373.0 |########################################
  10413.0 |
  10452.9 |
  10492.9 |
  10532.9 |########################################
  10572.9 |
  10612.8 |
  10652.8 |
  10692.8 |########################################
  10732.7 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 10289.2-12461.0 ns)
  10289.2 |####################
  10397.8 |####################
  10506.4 |
  10615.0 |########################################
  10723.6 |####################
  10832.2 |
  10940.7 |
  11049.3 |
  11157.9 |
  11266.5 |
  11375.1 |
  11483.7 |
  11592.3 |
  11700.9 |
  11809.5 |
  11918.0 |
  12026.6 |
  12135.2 |
  12243.8 |
  12352.4 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 10166.7-10919.6 ns)
  10166.7 |########################################
  10204.3 |
  10242.0 |
  10279.6 |
  10317.3 |
  10354.9 |
  10392.6 |
  10430.2 |
  10467.9 |
  10505.5 |
  10543.2 |########################################
  10580.8 |
  10618.4 |
  10656.1 |########################################
  10693.7 |########################################
  10731.4 |
  10769.0 |
  10806.7 |
  10844.3 |########################################
  10882.0 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 10285.8-10806.9 ns)
  10285.8 |########################################
  10311.9 |
  10337.9 |
  10364.0 |
  10390.0 |########################################
  10416.1 |
  10442.1 |
  10468.2 |
  10494.2 |
  10520.3 |
  10546.4 |########################################
  10572.4 |
  10598.5 |
  10624.5 |
  10650.6 |########################################
  10676.6 |
  10702.7 |
  10728.7 |
  10754.8 |
  10780.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=869.3% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=874.7% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=852.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=852.8% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=864.4% of algo (FFI overhead may distort results)
