# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_madd_rec32 is fastest but the noisiest (CV 7.4%)

carrier_lay_madd_rec32 wins on median (11.50 us) yet has the highest variance (CV 7.4%), while carrier_lay_madd_rec20 is the steadiest (CV 4.0%, 11.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_lay_madd_rec32, carrier_lay_madd_rec24) are a dead heat (<1%)

carrier_lay_madd_rec32 (11.50 us) and carrier_lay_madd_rec24 (11.60 us) differ by 0.83%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_lay_madd_rec16 shows alternating (throttle bounce) (autocorr -0.57)

carrier_lay_madd_rec16's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (393 ns) is smaller than the fastest variant's own run-to-run std-dev (856 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_madd_rec32 vs stability leader carrier_lay_madd_rec20 (+2% speed for 1.8x steadier)

carrier_lay_madd_rec32 is fastest (11.50 us, CV 7.4%); carrier_lay_madd_rec20 gives up 1.6% median for 1.8x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.4% of the fastest

All 5 variants sit between 11.50 us and 11.89 us - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_madd_rec16's edge over baseline is significant but tiny (-18 ns, 0.16%)

carrier_lay_madd_rec16 differs from baseline carrier_lay_madd_rec24 by -18 ns (0.16%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_madd_rec32** at 11501.5 ns median (-0.8% vs baseline)
- Spread: 1.03x (fastest 11501.5 ns, slowest 11894.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 14131ns | 14245ns | 12868ns | 13884ns | 15134ns | +0.28% |
| carrier_lay_madd_rec16 | 14073ns | 14300ns | 13089ns | 13950ns | 14749ns | -0.14% |
| carrier_lay_madd_rec20 | 14027ns | 14053ns | 13367ns | 13833ns | 14647ns | -0.47% |
| carrier_lay_madd_rec24 | 14092ns | 13964ns | 13005ns | 13696ns | 15231ns | base |
| carrier_lay_madd_rec32 | 14083ns | 13814ns | 13038ns | 13569ns | 15377ns | -0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 11737ns | 10728ns | 12574ns | +0.36% | 0.022 |
| carrier_lay_madd_rec16 | 11725ns | 10919ns | 12306ns | +0.25% | 0.022 |
| carrier_lay_madd_rec20 | 11660ns | 11110ns | 12173ns | -0.30% | 0.022 |
| carrier_lay_madd_rec24 | 11695ns | 10785ns | 12626ns | base | 0.022 |
| carrier_lay_madd_rec32 | 11718ns | 10858ns | 12768ns | +0.19% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 310112 | 1312709 | 0.236 | 1.00× |
| carrier_lay_madd_rec16 | 306600 | 1305902 | 0.235 | 0.99× |
| carrier_lay_madd_rec20 | 311387 | 1327930 | 0.234 | 1.01× |
| carrier_lay_madd_rec24 | 309233 | 1312582 | 0.236 | 1.00× |
| carrier_lay_madd_rec32 | 306988 | 1307121 | 0.235 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_lay_madd_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.022 | 91.1% |
| carrier_lay_madd_rec16 | 0.022 | 90.2% |
| carrier_lay_madd_rec20 | 0.022 | 91.8% |
| carrier_lay_madd_rec24 | 0.022 | 92.5% |
| carrier_lay_madd_rec32 | 0.022 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 14131ns | 14131ns | +0.28% |
| carrier_lay_madd_rec16 | 14073ns | 14073ns | -0.14% |
| carrier_lay_madd_rec20 | 14027ns | 14027ns | -0.47% |
| carrier_lay_madd_rec24 | 14092ns | 14092ns | base |
| carrier_lay_madd_rec32 | 14083ns | 14083ns | -0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 11597ns | base | --- | [10863, 12626] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 11774ns | no significant difference | [-986, +1035]ns | [10863, 12574] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_madd_rec16 | 11894ns | no significant difference | [-406, +513]ns | [10975, 12306] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 11682ns | no significant difference | [-881, +717]ns | [11124, 12173] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 11501ns | no significant difference | [-938, +963]ns | [10883, 12768] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 12597ns | -14.8% | -2.5% | -9.9% | -13.8% |
| 2 | 11046ns | -0.4% | -0.1% | +0.6% | -1.2% |
| 3 | 12148ns | +3.7% | +1.5% | +0.4% | +5.1% |
| 4 | 10785ns | +15.0% | +7.8% | +11.4% | +12.1% |
| 5 | 10940ns | +1.8% | -0.2% | +1.8% | -0.3% |
| 6 | 12656ns | -0.8% | -3.9% | -4.0% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.053 | ok |
| carrier_lay_madd_rec16 | -0.567 | HIGH- (thermal bounce) |
| carrier_lay_madd_rec20 | -0.278 | moderate- |
| carrier_lay_madd_rec24 | -0.354 | moderate- |
| carrier_lay_madd_rec32 | -0.208 | moderate- |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 3/6, lost 3/6
- **carrier_lay_madd_rec16**: won 4/6, lost 2/6
- **carrier_lay_madd_rec20**: won 2/6, lost 4/6
- **carrier_lay_madd_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 90831.1ns | 11737.0ns | 773.9% | HIGH |
| carrier_lay_madd_rec16 | 90360.9ns | 11725.1ns | 770.7% | HIGH |
| carrier_lay_madd_rec20 | 91747.5ns | 11659.8ns | 786.9% | HIGH |
| carrier_lay_madd_rec24 | 90562.3ns | 11695.3ns | 774.3% | HIGH |
| carrier_lay_madd_rec32 | 90279.1ns | 11717.6ns | 770.5% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 10728.3-12574.4 ns)
  10728.3 |########################################
  10820.6 |
  10912.9 |########################################
  11005.2 |
  11097.5 |########################################
  11189.8 |
  11282.1 |
  11374.4 |
  11466.7 |
  11559.0 |
  11651.3 |
  11743.6 |
  11835.9 |
  11928.2 |
  12020.5 |
  12112.8 |
  12205.1 |
  12297.4 |
  12389.7 |########################################
  12482.0 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 10918.8-12306.2 ns)
  10918.8 |########################################
  10988.2 |########################################
  11057.5 |
  11126.9 |
  11196.3 |
  11265.7 |
  11335.0 |
  11404.4 |
  11473.8 |
  11543.2 |
  11612.5 |########################################
  11681.9 |
  11751.3 |
  11820.6 |
  11890.0 |
  11959.4 |
  12028.8 |
  12098.1 |########################################
  12167.5 |
  12236.9 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 11109.6-12172.9 ns)
  11109.6 |########################################
  11162.8 |
  11215.9 |
  11269.1 |
  11322.3 |####################
  11375.4 |
  11428.6 |
  11481.8 |
  11534.9 |
  11588.1 |
  11641.2 |
  11694.4 |
  11747.6 |
  11800.7 |
  11853.9 |
  11907.1 |
  11960.2 |
  12013.4 |####################
  12066.6 |
  12119.7 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 10785.4-12626.2 ns)
  10785.4 |########################################
  10877.4 |########################################
  10969.5 |########################################
  11061.5 |
  11153.6 |
  11245.6 |
  11337.7 |
  11429.7 |
  11521.7 |
  11613.8 |
  11705.8 |
  11797.9 |
  11889.9 |
  11982.0 |
  12074.0 |########################################
  12166.0 |
  12258.1 |
  12350.1 |
  12442.2 |
  12534.2 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 10858.3-12768.5 ns)
  10858.3 |########################################
  10953.8 |
  11049.3 |
  11144.8 |
  11240.3 |
  11335.8 |
  11431.4 |
  11526.9 |
  11622.4 |
  11717.9 |
  11813.4 |
  11908.9 |
  12004.4 |#############
  12099.9 |
  12195.4 |
  12291.0 |
  12386.5 |
  12482.0 |
  12577.5 |
  12673.0 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=757.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=757.8% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=783.8% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=774.6% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=776.5% of algo (FFI overhead may distort results)
