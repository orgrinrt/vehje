# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_lay_tight_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_lay_tight_rec24 has the worst median (10.64 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_lay_tight_rec16 at 10.26 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_lay_tight_rec16 is fastest but the noisiest (CV 9.2%)

carrier_lay_tight_rec16 wins on median (10.26 us) yet has the highest variance (CV 9.2%), while carrier_lay_tight_rec20 is the steadiest (CV 2.2%, 10.52 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_lay_tight_rec20 shows alternating (throttle bounce) (autocorr -0.62)

carrier_lay_tight_rec20's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (374 ns) is smaller than the fastest variant's own run-to-run std-dev (940 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_tight_rec16 vs stability leader carrier_lay_tight_rec20 (+3% speed for 4.2x steadier)

carrier_lay_tight_rec16 is fastest (10.26 us, CV 9.2%); carrier_lay_tight_rec20 gives up 2.6% median for 4.2x lower variance (CV 2.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.6% of the fastest

All 5 variants sit between 10.26 us and 10.64 us - a 3.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_tight_rec32's edge over baseline is significant but tiny (43 ns, 0.40%)

carrier_lay_tight_rec32 differs from baseline carrier_lay_tight_rec24 by 43 ns (0.40%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_tight_rec16** at 10261.6 ns median (-3.5% vs baseline)
- Spread: 1.04x (fastest 10261.6 ns, slowest 10635.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 12680ns | 12738ns | 11838ns | 12642ns | 13157ns | -1.32% |
| carrier_lay_tight_rec16 | 13094ns | 12572ns | 12485ns | 12545ns | 14221ns | +1.91% |
| carrier_lay_tight_rec20 | 12788ns | 12827ns | 12429ns | 12700ns | 13098ns | -0.48% |
| carrier_lay_tight_rec24 | 12849ns | 12958ns | 12399ns | 12789ns | 13164ns | base |
| carrier_lay_tight_rec32 | 12850ns | 12945ns | 12344ns | 12757ns | 13242ns | +0.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 10399ns | 9713ns | 10800ns | -1.34% | 0.025 |
| carrier_lay_tight_rec16 | 10750ns | 10234ns | 11752ns | +1.99% | 0.024 |
| carrier_lay_tight_rec20 | 10487ns | 10196ns | 10739ns | -0.51% | 0.024 |
| carrier_lay_tight_rec24 | 10540ns | 10165ns | 10797ns | base | 0.024 |
| carrier_lay_tight_rec32 | 10539ns | 10114ns | 10865ns | -0.01% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 306985 | 1449042 | 0.212 | 1.01× |
| carrier_lay_tight_rec16 | 310994 | 1450744 | 0.214 | 1.02× |
| carrier_lay_tight_rec20 | 306825 | 1446879 | 0.212 | 1.00× |
| carrier_lay_tight_rec24 | 305348 | 1438428 | 0.212 | 1.00× |
| carrier_lay_tight_rec32 | 304220 | 1433957 | 0.212 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_tight_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.025 | 93.1% |
| carrier_lay_tight_rec16 | 0.025 | 94.7% |
| carrier_lay_tight_rec20 | 0.024 | 92.3% |
| carrier_lay_tight_rec24 | 0.024 | 91.3% |
| carrier_lay_tight_rec32 | 0.024 | 91.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 12680ns | 12680ns | -1.32% |
| carrier_lay_tight_rec16 | 13094ns | 13094ns | +1.91% |
| carrier_lay_tight_rec20 | 12788ns | 12788ns | -0.48% |
| carrier_lay_tight_rec24 | 12849ns | 12849ns | base |
| carrier_lay_tight_rec32 | 12850ns | 12850ns | +0.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 10636ns | base | --- | [10188, 10797] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 10438ns | no significant difference | [-708, +396]ns | [9960, 10800] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec16 | 10262ns | no significant difference | [-539, +1116]ns | [10235, 11752] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec20 | 10524ns | no significant difference | [-581, +335]ns | [10198, 10739] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 10619ns | no significant difference | [-353, +306]ns | [10133, 10865] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 10710ns | -9.3% | -4.0% | +0.3% | +1.3% |
| 2 | 10165ns | +0.4% | +0.7% | +2.7% | +4.0% |
| 3 | 10211ns | +4.4% | +0.3% | +3.9% | -0.6% |
| 4 | 10675ns | -2.5% | +20.1% | -4.4% | +1.9% |
| 5 | 10597ns | +3.2% | +0.8% | +1.3% | -4.6% |
| 6 | 10885ns | -3.8% | -6.0% | -6.3% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | 0.145 | ok |
| carrier_lay_tight_rec16 | -0.124 | ok |
| carrier_lay_tight_rec20 | -0.625 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec24 | 0.103 | ok |
| carrier_lay_tight_rec32 | -0.587 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 3/6, lost 3/6
- **carrier_lay_tight_rec16**: won 2/6, lost 4/6
- **carrier_lay_tight_rec20**: won 2/6, lost 4/6
- **carrier_lay_tight_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 90316.1ns | 10399.4ns | 868.5% | HIGH |
| carrier_lay_tight_rec16 | 91782.1ns | 10749.8ns | 853.8% | HIGH |
| carrier_lay_tight_rec20 | 90981.9ns | 10487.2ns | 867.6% | HIGH |
| carrier_lay_tight_rec24 | 90790.9ns | 10540.5ns | 861.4% | HIGH |
| carrier_lay_tight_rec32 | 90497.2ns | 10539.0ns | 858.7% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 9712.9-10799.8 ns)
   9712.9 |########################################
   9767.2 |
   9821.6 |
   9875.9 |
   9930.3 |
   9984.6 |
  10039.0 |
  10093.3 |
  10147.7 |
  10202.0 |########################################
  10256.3 |
  10310.7 |
  10365.0 |########################################
  10419.4 |########################################
  10473.7 |
  10528.1 |
  10582.4 |
  10636.8 |########################################
  10691.1 |
  10745.5 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 10234.2-11752.3 ns)
  10234.2 |########################################
  10310.1 |
  10386.0 |
  10461.9 |
  10537.8 |
  10613.7 |##########
  10689.6 |
  10765.5 |
  10841.4 |
  10917.3 |
  10993.2 |
  11069.2 |
  11145.1 |
  11221.0 |
  11296.9 |
  11372.8 |
  11448.7 |
  11524.6 |
  11600.5 |
  11676.4 |
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 10196.2-10739.4 ns)
  10196.2 |########################################
  10223.4 |
  10250.5 |
  10277.7 |
  10304.8 |
  10332.0 |
  10359.2 |
  10386.3 |
  10413.5 |
  10440.6 |####################
  10467.8 |
  10495.0 |
  10522.1 |
  10549.3 |
  10576.4 |
  10603.6 |####################
  10630.8 |
  10657.9 |
  10685.1 |
  10712.2 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 10165.4-10797.3 ns)
  10165.4 |########################################
  10197.0 |########################################
  10228.6 |
  10260.2 |
  10291.8 |
  10323.4 |
  10355.0 |
  10386.6 |
  10418.2 |
  10449.8 |
  10481.3 |
  10512.9 |
  10544.5 |
  10576.1 |########################################
  10607.7 |
  10639.3 |
  10670.9 |########################################
  10702.5 |########################################
  10734.1 |
  10765.7 |
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 10113.7-10865.0 ns)
  10113.7 |########################################
  10151.3 |########################################
  10188.8 |
  10226.4 |
  10264.0 |
  10301.5 |
  10339.1 |
  10376.7 |
  10414.2 |
  10451.8 |
  10489.4 |
  10526.9 |
  10564.5 |########################################
  10602.0 |
  10639.6 |########################################
  10677.2 |
  10714.7 |
  10752.3 |
  10789.9 |
  10827.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=872.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=895.5% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=866.9% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=852.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=852.6% of algo (FFI overhead may distort results)
