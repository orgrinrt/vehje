# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_scatter_rec20 shows alternating (throttle bounce) (autocorr -0.58)

carrier_lay_scatter_rec20's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (675 ns) is smaller than the fastest variant's own run-to-run std-dev (2.12 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader carrier_lay_scatter_rec32 vs stability leader carrier_lay_scatter_rec24 (+1% speed for 1.1x steadier)

carrier_lay_scatter_rec32 is fastest (44.19 us, CV 4.8%); carrier_lay_scatter_rec24 gives up 0.7% median for 1.1x lower variance (CV 4.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_lay_scatter_rec20 is inconsistent: worst-20% is 1.7x its best-20%

carrier_lay_scatter_rec20's best 20% of batches run at 43.72 us but its worst 20% at 72.75 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.5% of the fastest

All 5 variants sit between 44.19 us and 44.87 us - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_scatter_rec32** at 44194.6 ns median (-0.7% vs baseline)
- Spread: 1.02x (fastest 44194.6 ns, slowest 44869.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 51373ns | 47471ns | 43246ns | 47254ns | 61616ns | +8.44% |
| carrier_lay_scatter_rec16 | 48671ns | 47183ns | 45279ns | 46559ns | 53535ns | +2.74% |
| carrier_lay_scatter_rec20 | 56366ns | 47480ns | 46062ns | 47211ns | 75250ns | +18.98% |
| carrier_lay_scatter_rec24 | 47375ns | 47026ns | 44579ns | 46889ns | 49502ns | base |
| carrier_lay_scatter_rec32 | 47971ns | 46766ns | 46608ns | 46722ns | 50526ns | +1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 48819ns | 40866ns | 59028ns | +8.89% | 0.021 |
| carrier_lay_scatter_rec16 | 46114ns | 42707ns | 50955ns | +2.85% | 0.022 |
| carrier_lay_scatter_rec20 | 53849ns | 43720ns | 72749ns | +20.11% | 0.019 |
| carrier_lay_scatter_rec24 | 44835ns | 42055ns | 46926ns | base | 0.023 |
| carrier_lay_scatter_rec32 | 45448ns | 44012ns | 48081ns | +1.37% | 0.023 |

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_lay_scatter_rec12; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.023 | 91.1% |
| carrier_lay_scatter_rec16 | 0.023 | 91.6% |
| carrier_lay_scatter_rec20 | 0.023 | 91.1% |
| carrier_lay_scatter_rec24 | 0.023 | 91.9% |
| carrier_lay_scatter_rec32 | 0.023 | 92.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 51373ns | 51373ns | +8.44% |
| carrier_lay_scatter_rec16 | 48671ns | 48671ns | +2.74% |
| carrier_lay_scatter_rec20 | 56366ns | 56366ns | +18.98% |
| carrier_lay_scatter_rec24 | 47375ns | 47375ns | base |
| carrier_lay_scatter_rec32 | 47971ns | 47971ns | +1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 44492ns | base | --- | [43086, 46926] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 44869ns | no significant difference | [-4022, +15047]ns | [42559, 59028] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_scatter_rec16 | 44614ns | no significant difference | [-3675, +7379]ns | [42772, 50955] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec20 | 44867ns | no significant difference | [-2059, +28257]ns | [43931, 72749] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_scatter_rec32 | 44195ns | no significant difference | [-2748, +4984]ns | [44068, 48081] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 48755ns | -16.2% | -12.4% | -7.9% | -9.5% |
| 2 | 42055ns | +5.2% | +33.1% | +4.0% | +10.1% |
| 3 | 44844ns | -0.3% | -0.0% | +58.2% | -1.9% |
| 4 | 45097ns | +2.0% | +1.9% | -0.6% | -1.9% |
| 5 | 44139ns | +63.2% | -3.0% | +68.9% | +12.9% |
| 6 | 44117ns | +2.1% | +0.7% | +0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.129 | ok |
| carrier_lay_scatter_rec16 | -0.321 | moderate- |
| carrier_lay_scatter_rec20 | -0.578 | HIGH- (thermal bounce) |
| carrier_lay_scatter_rec24 | -0.439 | moderate- |
| carrier_lay_scatter_rec32 | -0.431 | moderate- |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 2/6, lost 4/6
- **carrier_lay_scatter_rec16**: won 2/6, lost 3/6
- **carrier_lay_scatter_rec20**: won 2/6, lost 3/6
- **carrier_lay_scatter_rec32**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 100536.8ns | 48818.9ns | 205.9% | HIGH |
| carrier_lay_scatter_rec16 | 96520.7ns | 46113.5ns | 209.3% | HIGH |
| carrier_lay_scatter_rec20 | 109152.8ns | 53848.8ns | 202.7% | HIGH |
| carrier_lay_scatter_rec24 | 94663.0ns | 44834.6ns | 211.1% | HIGH |
| carrier_lay_scatter_rec32 | 94431.7ns | 45447.9ns | 207.8% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 40866.2-59028.3 ns)
  40866.2 |####################
  41774.3 |
  42682.4 |
  43590.5 |####################
  44498.6 |########################################
  45406.7 |####################
  46314.8 |
  47223.0 |
  48131.1 |
  49039.2 |
  49947.3 |
  50855.4 |
  51763.5 |
  52671.6 |
  53579.7 |
  54487.8 |
  55395.9 |
  56304.0 |
  57212.1 |
  58120.2 |
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 42707.1-50955.2 ns)
  42707.1 |########################################
  43119.5 |
  43531.9 |
  43944.3 |
  44356.7 |####################
  44769.1 |####################
  45181.5 |
  45593.9 |####################
  46006.3 |
  46418.7 |
  46831.1 |
  47243.6 |
  47656.0 |
  48068.4 |
  48480.8 |
  48893.2 |
  49305.6 |
  49718.0 |
  50130.4 |
  50542.8 |
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 43720.4-72748.6 ns)
  43720.4 |########################################
  45171.8 |
  46623.2 |
  48074.6 |
  49526.0 |
  50977.4 |
  52428.8 |
  53880.3 |
  55331.7 |
  56783.1 |
  58234.5 |
  59685.9 |
  61137.3 |
  62588.7 |
  64040.1 |
  65491.5 |
  66942.9 |
  68394.3 |
  69845.7 |##########
  71297.1 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 42055.4-46926.1 ns)
  42055.4 |####################
  42298.9 |
  42542.5 |
  42786.0 |
  43029.5 |
  43273.1 |
  43516.6 |
  43760.1 |
  44003.7 |########################################
  44247.2 |
  44490.7 |
  44734.3 |####################
  44977.8 |####################
  45221.3 |
  45464.9 |
  45708.4 |
  45951.9 |
  46195.5 |
  46439.0 |
  46682.5 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 44011.7-48081.2 ns)
  44011.7 |########################################
  44215.2 |#############
  44418.7 |
  44622.1 |
  44825.6 |
  45029.1 |
  45232.6 |
  45436.0 |
  45639.5 |
  45843.0 |
  46046.5 |
  46250.0 |#############
  46453.4 |
  46656.9 |
  46860.4 |
  47063.9 |
  47267.3 |
  47470.8 |
  47674.3 |
  47877.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: CV=21.5% (high variance, measurements may be unstable)
- **carrier_lay_scatter_rec12**: bridge=203.3% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=210.4% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: CV=24.9% (high variance, measurements may be unstable)
- **carrier_lay_scatter_rec20**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=207.5% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=207.0% of algo (FFI overhead may distort results)
