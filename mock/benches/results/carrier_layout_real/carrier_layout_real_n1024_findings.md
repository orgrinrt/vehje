# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (276 ns) is smaller than the fastest variant's own run-to-run std-dev (841 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (carrier_lay_real_rec24)

The baseline carrier_lay_real_rec24 is the fastest (41.64 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.7% of the fastest

All 5 variants sit between 41.64 us and 41.92 us - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_real_rec20's edge over baseline is significant but tiny (48 ns, 0.12%)

carrier_lay_real_rec20 differs from baseline carrier_lay_real_rec24 by 48 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (carrier_lay_real_rec24) is the fastest** at 41643.3 ns median
- Spread: 1.01x (fastest 41643.3 ns, slowest 41919.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 44228ns | 44308ns | 42900ns | 44246ns | 44864ns | -0.44% |
| carrier_lay_real_rec16 | 43934ns | 44122ns | 42710ns | 44063ns | 44355ns | -1.10% |
| carrier_lay_real_rec20 | 44589ns | 44120ns | 42602ns | 44048ns | 46395ns | +0.38% |
| carrier_lay_real_rec24 | 44422ns | 44071ns | 43789ns | 44010ns | 45355ns | base |
| carrier_lay_real_rec32 | 44327ns | 44345ns | 43870ns | 44247ns | 44677ns | -0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 41818ns | 40568ns | 42430ns | -0.44% | 0.024 |
| carrier_lay_real_rec16 | 41520ns | 40417ns | 41910ns | -1.15% | 0.025 |
| carrier_lay_real_rec20 | 42203ns | 40316ns | 43928ns | +0.47% | 0.024 |
| carrier_lay_real_rec24 | 42003ns | 41511ns | 42846ns | base | 0.024 |
| carrier_lay_real_rec32 | 41963ns | 41685ns | 42271ns | -0.10% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 512077 | 2263472 | 0.226 | 1.01× |
| carrier_lay_real_rec16 | 516806 | 2278510 | 0.227 | 1.02× |
| carrier_lay_real_rec20 | 498328 | 2197909 | 0.227 | 0.98× |
| carrier_lay_real_rec24 | 508003 | 2239182 | 0.227 | 1.00× |
| carrier_lay_real_rec32 | 515128 | 2268249 | 0.227 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_lay_real_rec20; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.024 | 96.3% |
| carrier_lay_real_rec16 | 0.025 | 96.7% |
| carrier_lay_real_rec20 | 0.025 | 96.6% |
| carrier_lay_real_rec24 | 0.025 | 96.8% |
| carrier_lay_real_rec32 | 0.024 | 96.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 44228ns | 44228ns | -0.44% |
| carrier_lay_real_rec16 | 43934ns | 43934ns | -1.10% |
| carrier_lay_real_rec20 | 44589ns | 44589ns | +0.38% |
| carrier_lay_real_rec24 | 44422ns | 44422ns | base |
| carrier_lay_real_rec32 | 44327ns | 44327ns | -0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 41643ns | base | --- | [41521, 42846] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 41885ns | no significant difference | [-1098, +469]ns | [41138, 42430] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec16 | 41682ns | no significant difference | [-1460, +161]ns | [40968, 41910] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec20 | 41755ns | no significant difference | [-1628, +2178]ns | [40925, 43928] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_real_rec32 | 41919ns | no significant difference | [-702, +493]ns | [41699, 42271] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 41530ns | -2.3% | +0.4% | +10.1% | +1.2% |
| 2 | 41827ns | +0.5% | -0.7% | -3.6% | +0.0% |
| 3 | 41738ns | -0.1% | +0.0% | +0.3% | -0.1% |
| 4 | 43865ns | -2.8% | -4.1% | -4.0% | -3.1% |
| 5 | 41511ns | +1.7% | +0.3% | +0.4% | +1.2% |
| 6 | 41548ns | +0.4% | -2.7% | -0.0% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.042 | ok |
| carrier_lay_real_rec16 | 0.032 | ok |
| carrier_lay_real_rec20 | -0.329 | moderate- |
| carrier_lay_real_rec24 | -0.249 | moderate- |
| carrier_lay_real_rec32 | -0.251 | moderate- |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 2/6, lost 3/6
- **carrier_lay_real_rec16**: won 3/6, lost 2/6
- **carrier_lay_real_rec20**: won 2/6, lost 3/6
- **carrier_lay_real_rec32**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 122019.9ns | 41818.0ns | 291.8% | HIGH |
| carrier_lay_real_rec16 | 124132.4ns | 41519.8ns | 299.0% | HIGH |
| carrier_lay_real_rec20 | 119587.9ns | 42202.6ns | 283.4% | HIGH |
| carrier_lay_real_rec24 | 122384.3ns | 42003.4ns | 291.4% | HIGH |
| carrier_lay_real_rec32 | 124565.7ns | 41963.0ns | 296.8% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 40567.5-42430.2 ns)
  40567.5 |####################
  40660.6 |
  40753.8 |
  40846.9 |
  40940.1 |
  41033.2 |
  41126.3 |
  41219.5 |
  41312.6 |
  41405.7 |
  41498.9 |
  41592.0 |
  41685.2 |########################################
  41778.3 |
  41871.4 |
  41964.6 |####################
  42057.7 |
  42150.8 |####################
  42244.0 |
  42337.1 |
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 40417.1-41909.8 ns)
  40417.1 |####################
  40491.7 |
  40566.4 |
  40641.0 |
  40715.6 |
  40790.3 |
  40864.9 |
  40939.5 |
  41014.2 |
  41088.8 |
  41163.4 |
  41238.1 |
  41312.7 |
  41387.3 |
  41462.0 |####################
  41536.6 |
  41611.2 |####################
  41685.9 |########################################
  41760.5 |
  41835.1 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 40315.8-43927.7 ns)
  40315.8 |########################################
  40496.4 |
  40677.0 |
  40857.6 |
  41038.2 |
  41218.8 |
  41399.4 |########################################
  41580.0 |########################################
  41760.6 |########################################
  41941.2 |########################################
  42121.8 |
  42302.3 |
  42482.9 |
  42663.5 |
  42844.1 |
  43024.7 |
  43205.3 |
  43385.9 |
  43566.5 |
  43747.1 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 41511.2-42846.1 ns)
  41511.2 |########################################
  41577.9 |
  41644.7 |
  41711.4 |#############
  41778.2 |#############
  41844.9 |
  41911.7 |
  41978.4 |
  42045.1 |
  42111.9 |
  42178.6 |
  42245.4 |
  42312.1 |
  42378.9 |
  42445.6 |
  42512.3 |
  42579.1 |
  42645.8 |
  42712.6 |
  42779.3 |
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 41685.0-42271.2 ns)
  41685.0 |########################################
  41714.3 |
  41743.6 |
  41772.9 |
  41802.2 |
  41831.6 |####################
  41860.9 |
  41890.2 |
  41919.5 |
  41948.8 |
  41978.1 |####################
  42007.4 |####################
  42036.8 |
  42066.1 |
  42095.4 |
  42124.7 |
  42154.0 |
  42183.3 |
  42212.6 |
  42241.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=297.7% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=298.8% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=297.8% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=298.9% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=296.8% of algo (FFI overhead may distort results)
