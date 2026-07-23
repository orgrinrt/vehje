# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_wideselect_rec12 shows alternating (throttle bounce) (autocorr -0.54)

carrier_lay_wideselect_rec12's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.5% of the fastest

All 5 variants sit between 2.16 ms and 2.17 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_wideselect_rec20** at 2158492.7 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 2158492.7 ns, slowest 2169141.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2168114ns | 2172370ns | 2144593ns | 2168093ns | 2179907ns | -0.04% |
| carrier_lay_wideselect_rec16 | 2168245ns | 2170291ns | 2146980ns | 2168387ns | 2178665ns | -0.04% |
| carrier_lay_wideselect_rec20 | 2162075ns | 2162051ns | 2149888ns | 2159614ns | 2171861ns | -0.32% |
| carrier_lay_wideselect_rec24 | 2169019ns | 2169101ns | 2162781ns | 2168266ns | 2173268ns | base |
| carrier_lay_wideselect_rec32 | 2168541ns | 2169181ns | 2158791ns | 2167884ns | 2174400ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2164994ns | 2142082ns | 2176780ns | -0.03% | 0.008 |
| carrier_lay_wideselect_rec16 | 2164989ns | 2143528ns | 2175448ns | -0.03% | 0.008 |
| carrier_lay_wideselect_rec20 | 2158741ns | 2146615ns | 2168344ns | -0.32% | 0.008 |
| carrier_lay_wideselect_rec24 | 2165733ns | 2159813ns | 2170101ns | base | 0.008 |
| carrier_lay_wideselect_rec32 | 2165301ns | 2155788ns | 2171010ns | -0.02% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 13470335 | 20846865 | 0.646 | 1.00× |
| carrier_lay_wideselect_rec16 | 13491832 | 19254688 | 0.701 | 1.00× |
| carrier_lay_wideselect_rec20 | 13450376 | 19254722 | 0.699 | 0.99× |
| carrier_lay_wideselect_rec24 | 13519981 | 19255957 | 0.702 | 1.00× |
| carrier_lay_wideselect_rec32 | 13500103 | 19255587 | 0.701 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_lay_wideselect_rec12; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.008 | 98.8% |
| carrier_lay_wideselect_rec16 | 0.008 | 98.9% |
| carrier_lay_wideselect_rec20 | 0.008 | 99.2% |
| carrier_lay_wideselect_rec24 | 0.008 | 98.9% |
| carrier_lay_wideselect_rec32 | 0.008 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2168114ns | 2168114ns | -0.04% |
| carrier_lay_wideselect_rec16 | 2168245ns | 2168245ns | -0.04% |
| carrier_lay_wideselect_rec20 | 2162075ns | 2162075ns | -0.32% |
| carrier_lay_wideselect_rec24 | 2169019ns | 2169019ns | base |
| carrier_lay_wideselect_rec32 | 2168541ns | 2168541ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 2165606ns | base | --- | [2161493, 2170101] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 2169142ns | no significant difference | [-18913, +11411]ns | [2149060, 2176780] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec16 | 2166721ns | no significant difference | [-16223, +11134]ns | [2152797, 2175448] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec20 | 2158493ns | no significant difference | [-19635, +2021]ns | [2149385, 2168344] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_wideselect_rec32 | 2165694ns | no significant difference | [-8773, +5379]ns | [2159200, 2171010] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 2168815ns | +0.7% | +0.4% | +0.2% | +0.2% |
| 2 | 2171387ns | -1.3% | -0.4% | -0.9% | -0.4% |
| 3 | 2166653ns | +0.2% | -1.1% | -0.9% | +0.1% |
| 4 | 2159813ns | +0.4% | +0.6% | -0.3% | +0.3% |
| 5 | 2163172ns | +0.3% | +0.1% | +0.0% | +0.1% |
| 6 | 2164559ns | -0.4% | +0.1% | -0.0% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | -0.542 | HIGH- (thermal bounce) |
| carrier_lay_wideselect_rec16 | -0.196 | ok |
| carrier_lay_wideselect_rec20 | 0.111 | ok |
| carrier_lay_wideselect_rec24 | 0.414 | moderate+ |
| carrier_lay_wideselect_rec32 | -0.213 | moderate- |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 2/6, lost 4/6
- **carrier_lay_wideselect_rec16**: won 2/6, lost 4/6
- **carrier_lay_wideselect_rec20**: won 3/6, lost 1/6
- **carrier_lay_wideselect_rec32**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 2165308.7ns | 2164994.0ns | 100.0% | HIGH |
| carrier_lay_wideselect_rec16 | 2166956.4ns | 2164988.7ns | 100.1% | HIGH |
| carrier_lay_wideselect_rec20 | 2162569.3ns | 2158740.6ns | 100.2% | HIGH |
| carrier_lay_wideselect_rec24 | 2168824.4ns | 2165733.4ns | 100.1% | HIGH |
| carrier_lay_wideselect_rec32 | 2166098.1ns | 2165301.4ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 2142082.5-2176780.0 ns)
  2142082.5 |####################
  2143817.4 |
  2145552.2 |
  2147287.1 |
  2149022.0 |
  2150756.9 |
  2152491.8 |
  2154226.6 |
  2155961.5 |####################
  2157696.4 |
  2159431.2 |
  2161166.1 |
  2162901.0 |
  2164635.9 |
  2166370.8 |
  2168105.6 |####################
  2169840.5 |########################################
  2171575.4 |
  2173310.2 |
  2175045.1 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 2143528.3-2175448.3 ns)
  2143528.3 |########################################
  2145124.3 |
  2146720.3 |
  2148316.3 |
  2149912.3 |
  2151508.3 |
  2153104.3 |
  2154700.3 |
  2156296.3 |
  2157892.3 |
  2159488.3 |
  2161084.3 |########################################
  2162680.3 |
  2164276.3 |########################################
  2165872.3 |
  2167468.3 |########################################
  2169064.3 |
  2170660.3 |
  2172256.3 |########################################
  2173852.3 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 2146615.4-2168344.2 ns)
  2146615.4 |########################################
  2147701.8 |
  2148788.3 |
  2149874.7 |
  2150961.1 |
  2152047.6 |########################################
  2153134.0 |########################################
  2154220.5 |
  2155306.9 |
  2156393.3 |
  2157479.8 |
  2158566.2 |
  2159652.7 |
  2160739.1 |
  2161825.5 |
  2162912.0 |########################################
  2163998.4 |########################################
  2165084.8 |
  2166171.3 |
  2167257.7 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 2159812.9-2170101.2 ns)
  2159812.9 |########################################
  2160327.3 |
  2160841.7 |
  2161356.2 |
  2161870.6 |
  2162385.0 |
  2162899.4 |########################################
  2163413.8 |
  2163928.2 |
  2164442.7 |########################################
  2164957.1 |
  2165471.5 |
  2165985.9 |
  2166500.3 |########################################
  2167014.7 |
  2167529.2 |
  2168043.6 |
  2168558.0 |########################################
  2169072.4 |
  2169586.8 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 2155787.9-2171009.6 ns)
  2155787.9 |########################################
  2156549.0 |
  2157310.1 |
  2158071.2 |
  2158832.2 |
  2159593.3 |
  2160354.4 |
  2161115.5 |
  2161876.6 |########################################
  2162637.7 |
  2163398.8 |
  2164159.8 |
  2164920.9 |########################################
  2165682.0 |########################################
  2166443.1 |
  2167204.2 |########################################
  2167965.3 |
  2168726.3 |
  2169487.4 |
  2170248.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=100.1% of algo (FFI overhead may distort results)
