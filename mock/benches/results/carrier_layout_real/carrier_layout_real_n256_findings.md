# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_real_rec12 is fastest but the noisiest (CV 5.2%)

carrier_lay_real_rec12 wins on median (11.08 us) yet has the highest variance (CV 5.2%), while carrier_lay_real_rec16 is the steadiest (CV 0.9%, 11.19 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_lay_real_rec24 shows alternating (throttle bounce) (autocorr -0.53)

carrier_lay_real_rec24's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (157 ns) is smaller than the fastest variant's own run-to-run std-dev (571 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_real_rec12 vs stability leader carrier_lay_real_rec16 (+1% speed for 6.0x steadier)

carrier_lay_real_rec12 is fastest (11.08 us, CV 5.2%); carrier_lay_real_rec16 gives up 1.0% median for 6.0x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.4% of the fastest

All 5 variants sit between 11.08 us and 11.24 us - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_real_rec12** at 11078.4 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 11078.4 ns, slowest 11235.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 13243ns | 13560ns | 12085ns | 13166ns | 13938ns | -2.73% |
| carrier_lay_real_rec16 | 13789ns | 13733ns | 13674ns | 13728ns | 13939ns | +1.28% |
| carrier_lay_real_rec20 | 13775ns | 13748ns | 13622ns | 13708ns | 13952ns | +1.18% |
| carrier_lay_real_rec24 | 13615ns | 13616ns | 13337ns | 13534ns | 13875ns | base |
| carrier_lay_real_rec32 | 13761ns | 13773ns | 13457ns | 13741ns | 13942ns | +1.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 10840ns | 9936ns | 11381ns | -2.35% | 0.024 |
| carrier_lay_real_rec16 | 11220ns | 11133ns | 11326ns | +1.07% | 0.023 |
| carrier_lay_real_rec20 | 11257ns | 11152ns | 11381ns | +1.41% | 0.023 |
| carrier_lay_real_rec24 | 11101ns | 10877ns | 11294ns | base | 0.023 |
| carrier_lay_real_rec32 | 11175ns | 10947ns | 11323ns | +0.67% | 0.023 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 303925 | 1375229 | 0.221 | 1.04× |
| carrier_lay_real_rec16 | 291269 | 1308099 | 0.223 | 1.00× |
| carrier_lay_real_rec20 | 289029 | 1305312 | 0.221 | 0.99× |
| carrier_lay_real_rec24 | 292345 | 1317926 | 0.222 | 1.00× |
| carrier_lay_real_rec32 | 291016 | 1311518 | 0.222 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_real_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.023 | 89.7% |
| carrier_lay_real_rec16 | 0.023 | 88.8% |
| carrier_lay_real_rec20 | 0.023 | 88.4% |
| carrier_lay_real_rec24 | 0.023 | 89.5% |
| carrier_lay_real_rec32 | 0.023 | 89.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 13243ns | 13243ns | -2.73% |
| carrier_lay_real_rec16 | 13789ns | 13789ns | +1.28% |
| carrier_lay_real_rec20 | 13775ns | 13775ns | +1.18% |
| carrier_lay_real_rec24 | 13615ns | 13615ns | base |
| carrier_lay_real_rec32 | 13761ns | 13761ns | +1.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 11108ns | base | --- | [10901, 11294] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 11078ns | no significant difference | [-960, +394]ns | [10061, 11381] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec16 | 11194ns | no significant difference | [-107, +406]ns | [11139, 11326] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec20 | 11236ns | +137.3ns (+1.2%) | [+61, +270]ns | [11155, 11381] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_real_rec32 | 11163ns | no significant difference | [-187, +268]ns | [11039, 11323] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 10925ns | -9.1% | +4.5% | +2.1% | +2.0% |
| 2 | 11097ns | +2.9% | +0.3% | +0.5% | +2.5% |
| 3 | 11273ns | -2.4% | -0.4% | +0.6% | -0.1% |
| 4 | 10877ns | +4.3% | +2.9% | +2.8% | +2.3% |
| 5 | 11315ns | -1.4% | -1.5% | +0.9% | -3.3% |
| 6 | 11118ns | -8.4% | +0.7% | +1.5% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.200 | moderate- |
| carrier_lay_real_rec16 | -0.270 | moderate- |
| carrier_lay_real_rec20 | -0.190 | ok |
| carrier_lay_real_rec24 | -0.528 | HIGH- (thermal bounce) |
| carrier_lay_real_rec32 | 0.183 | ok |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 4/6, lost 2/6
- **carrier_lay_real_rec16**: won 2/6, lost 4/6
- **carrier_lay_real_rec20**: won 0/6, lost 6/6
- **carrier_lay_real_rec32**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 90945.5ns | 10840.2ns | 839.0% | HIGH |
| carrier_lay_real_rec16 | 90262.8ns | 11219.6ns | 804.5% | HIGH |
| carrier_lay_real_rec20 | 90373.8ns | 11257.2ns | 802.8% | HIGH |
| carrier_lay_real_rec24 | 90072.6ns | 11100.9ns | 811.4% | HIGH |
| carrier_lay_real_rec32 | 90170.9ns | 11174.8ns | 806.9% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 9936.2-11380.9 ns)
   9936.2 |########################################
  10008.4 |
  10080.7 |
  10152.9 |########################################
  10225.1 |
  10297.4 |
  10369.6 |
  10441.8 |
  10514.1 |
  10586.3 |
  10658.5 |
  10730.8 |
  10803.0 |
  10875.2 |
  10947.5 |########################################
  11019.7 |
  11091.9 |########################################
  11164.2 |
  11236.4 |
  11308.6 |########################################
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 11132.9-11325.8 ns)
  11132.9 |####################
  11142.5 |####################
  11152.2 |
  11161.8 |
  11171.5 |
  11181.1 |
  11190.8 |########################################
  11200.4 |
  11210.1 |
  11219.7 |
  11229.3 |####################
  11239.0 |
  11248.6 |
  11258.3 |
  11267.9 |
  11277.6 |
  11287.2 |
  11296.9 |
  11306.5 |
  11316.2 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 11151.7-11380.8 ns)
  11151.7 |########################################
  11163.2 |
  11174.6 |####################
  11186.1 |
  11197.5 |
  11209.0 |
  11220.4 |
  11231.9 |
  11243.3 |
  11254.8 |
  11266.2 |
  11277.7 |####################
  11289.2 |
  11300.6 |
  11312.1 |
  11323.5 |
  11335.0 |####################
  11346.4 |
  11357.9 |
  11369.3 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 10876.7-11294.3 ns)
  10876.7 |########################################
  10897.6 |
  10918.5 |########################################
  10939.3 |
  10960.2 |
  10981.1 |
  11002.0 |
  11022.9 |
  11043.8 |
  11064.6 |
  11085.5 |########################################
  11106.4 |########################################
  11127.3 |
  11148.2 |
  11169.1 |
  11189.9 |
  11210.8 |
  11231.7 |
  11252.6 |########################################
  11273.5 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 10947.1-11322.9 ns)
  10947.1 |########################################
  10965.9 |
  10984.7 |
  11003.5 |
  11022.3 |
  11041.0 |
  11059.8 |
  11078.6 |
  11097.4 |
  11116.2 |########################################
  11135.0 |########################################
  11153.8 |
  11172.6 |########################################
  11191.4 |
  11210.2 |
  11229.0 |
  11247.7 |
  11266.5 |########################################
  11285.3 |
  11304.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=822.3% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=804.8% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=803.9% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=811.6% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=807.4% of algo (FFI overhead may distort results)
