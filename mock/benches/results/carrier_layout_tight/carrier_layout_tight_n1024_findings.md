# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_tight_rec24 shows alternating (throttle bounce) (autocorr -0.57)

carrier_lay_tight_rec24's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (697 ns) is smaller than the fastest variant's own run-to-run std-dev (1.76 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_tight_rec16 vs stability leader carrier_lay_tight_rec32 (+1% speed for 4.3x steadier)

carrier_lay_tight_rec16 is fastest (42.57 us, CV 4.1%); carrier_lay_tight_rec32 gives up 1.4% median for 4.3x lower variance (CV 1.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.6% of the fastest

All 5 variants sit between 42.57 us and 43.27 us - a 1.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_tight_rec20's edge over baseline is significant but tiny (43 ns, 0.10%)

carrier_lay_tight_rec20 differs from baseline carrier_lay_tight_rec24 by 43 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_tight_rec16** at 42569.8 ns median (-1.0% vs baseline)
- Spread: 1.02x (fastest 42569.8 ns, slowest 43266.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 46090ns | 45464ns | 44256ns | 45210ns | 48327ns | +2.03% |
| carrier_lay_tight_rec16 | 45806ns | 44740ns | 44244ns | 44613ns | 48378ns | +1.40% |
| carrier_lay_tight_rec20 | 45219ns | 45357ns | 44203ns | 44993ns | 46065ns | +0.10% |
| carrier_lay_tight_rec24 | 45174ns | 45194ns | 44397ns | 45106ns | 45666ns | base |
| carrier_lay_tight_rec32 | 45303ns | 45355ns | 44719ns | 45191ns | 45763ns | +0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 43867ns | 42123ns | 45991ns | +2.02% | 0.023 |
| carrier_lay_tight_rec16 | 43593ns | 42100ns | 46056ns | +1.38% | 0.023 |
| carrier_lay_tight_rec20 | 43031ns | 42078ns | 43803ns | +0.07% | 0.024 |
| carrier_lay_tight_rec24 | 43000ns | 42266ns | 43475ns | base | 0.024 |
| carrier_lay_tight_rec32 | 43123ns | 42568ns | 43575ns | +0.29% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 465380 | 2057575 | 0.226 | 0.97× |
| carrier_lay_tight_rec16 | 474190 | 2106661 | 0.225 | 0.98× |
| carrier_lay_tight_rec20 | 487172 | 2147930 | 0.227 | 1.01× |
| carrier_lay_tight_rec24 | 481558 | 2131160 | 0.226 | 1.00× |
| carrier_lay_tight_rec32 | 466710 | 2058520 | 0.227 | 0.97× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_lay_tight_rec20; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.024 | 97.3% |
| carrier_lay_tight_rec16 | 0.024 | 98.8% |
| carrier_lay_tight_rec20 | 0.024 | 97.4% |
| carrier_lay_tight_rec24 | 0.024 | 97.8% |
| carrier_lay_tight_rec32 | 0.024 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 46090ns | 46090ns | +2.03% |
| carrier_lay_tight_rec16 | 45806ns | 45806ns | +1.40% |
| carrier_lay_tight_rec20 | 45219ns | 45219ns | +0.10% |
| carrier_lay_tight_rec24 | 45174ns | 45174ns | base |
| carrier_lay_tight_rec32 | 45303ns | 45303ns | +0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 43014ns | base | --- | [42510, 43475] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 43267ns | no significant difference | [-799, +2643]ns | [42342, 45991] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec16 | 42570ns | no significant difference | [-1138, +3168]ns | [42152, 46056] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_tight_rec20 | 43184ns | no significant difference | [-748, +798]ns | [42105, 43803] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 43177ns | no significant difference | [-176, +450]ns | [42617, 43575] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 43510ns | -2.2% | +5.6% | +0.0% | +0.7% |
| 2 | 42753ns | +0.1% | +0.2% | +2.2% | +1.4% |
| 3 | 43256ns | +5.6% | -2.7% | +1.5% | -0.2% |
| 4 | 42772ns | -1.5% | -1.3% | +0.2% | -0.2% |
| 5 | 43440ns | +6.6% | -2.6% | -3.1% | -0.6% |
| 6 | 42266ns | +3.5% | +9.2% | -0.3% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | -0.543 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec16 | -0.005 | ok |
| carrier_lay_tight_rec20 | 0.559 | HIGH+ (drift/warm-up) |
| carrier_lay_tight_rec24 | -0.573 | HIGH- (thermal bounce) |
| carrier_lay_tight_rec32 | 0.072 | ok |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 2/6, lost 3/6
- **carrier_lay_tight_rec16**: won 3/6, lost 3/6
- **carrier_lay_tight_rec20**: won 2/6, lost 3/6
- **carrier_lay_tight_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 106036.2ns | 43866.5ns | 241.7% | HIGH |
| carrier_lay_tight_rec16 | 108855.8ns | 43592.5ns | 249.7% | HIGH |
| carrier_lay_tight_rec20 | 110741.6ns | 43030.7ns | 257.4% | HIGH |
| carrier_lay_tight_rec24 | 109579.2ns | 42999.6ns | 254.8% | HIGH |
| carrier_lay_tight_rec32 | 104692.3ns | 43123.1ns | 242.8% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 42122.9-45990.6 ns)
  42122.9 |########################################
  42316.3 |
  42509.7 |########################################
  42703.1 |########################################
  42896.4 |
  43089.8 |
  43283.2 |
  43476.6 |
  43670.0 |########################################
  43863.4 |
  44056.8 |
  44250.2 |
  44443.5 |
  44636.9 |
  44830.3 |
  45023.7 |
  45217.1 |
  45410.5 |
  45603.9 |########################################
  45797.3 |
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 42100.0-46055.8 ns)
  42100.0 |########################################
  42297.8 |####################
  42495.6 |
  42693.4 |####################
  42891.2 |
  43089.0 |
  43286.8 |
  43484.5 |
  43682.3 |
  43880.1 |
  44077.9 |
  44275.7 |
  44473.5 |
  44671.3 |
  44869.1 |
  45066.9 |
  45264.7 |
  45462.5 |
  45660.3 |
  45858.1 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 42077.9-43802.9 ns)
  42077.9 |########################################
  42164.2 |
  42250.4 |
  42336.7 |
  42422.9 |
  42509.2 |
  42595.4 |
  42681.7 |
  42767.9 |####################
  42854.2 |
  42940.4 |
  43026.6 |
  43112.9 |
  43199.1 |
  43285.4 |
  43371.6 |
  43457.9 |####################
  43544.1 |
  43630.4 |####################
  43716.6 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 42265.8-43475.2 ns)
  42265.8 |####################
  42326.3 |
  42386.7 |
  42447.2 |
  42507.7 |
  42568.2 |
  42628.6 |
  42689.1 |
  42749.6 |########################################
  42810.0 |
  42870.5 |
  42931.0 |
  42991.4 |
  43051.9 |
  43112.4 |
  43172.8 |
  43233.3 |####################
  43293.8 |
  43354.3 |
  43414.7 |####################
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 42568.3-43575.0 ns)
  42568.3 |########################################
  42618.6 |########################################
  42669.0 |
  42719.3 |
  42769.6 |
  42820.0 |
  42870.3 |
  42920.6 |
  42971.0 |
  43021.3 |
  43071.7 |
  43122.0 |########################################
  43172.3 |########################################
  43222.7 |
  43273.0 |
  43323.3 |########################################
  43373.7 |
  43424.0 |
  43474.3 |
  43524.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=239.3% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=269.7% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **carrier_lay_tight_rec20**: bridge=245.9% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=255.9% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=248.3% of algo (FFI overhead may distort results)
