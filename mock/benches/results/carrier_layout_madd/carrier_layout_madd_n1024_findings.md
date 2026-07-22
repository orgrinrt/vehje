# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_madd_rec24 shows alternating (throttle bounce) (autocorr -0.52)

carrier_lay_madd_rec24's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (409 ns) is smaller than the fastest variant's own run-to-run std-dev (1.18 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.8% of the fastest

All 5 variants sit between 49.37 us and 49.78 us - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_lay_madd_rec20's edge over baseline is significant but tiny (34 ns, 0.07%)

carrier_lay_madd_rec20 differs from baseline carrier_lay_madd_rec24 by 34 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_madd_rec12** at 49365.8 ns median (-0.6% vs baseline)
- Spread: 1.01x (fastest 49365.8 ns, slowest 49775.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 51422ns | 51931ns | 48714ns | 51849ns | 52135ns | -1.45% |
| carrier_lay_madd_rec16 | 51717ns | 51947ns | 50024ns | 51839ns | 52379ns | -0.89% |
| carrier_lay_madd_rec20 | 52185ns | 52177ns | 51710ns | 52169ns | 52446ns | +0.01% |
| carrier_lay_madd_rec24 | 52181ns | 52118ns | 51975ns | 52081ns | 52434ns | base |
| carrier_lay_madd_rec32 | 52450ns | 52262ns | 51732ns | 52141ns | 53273ns | +0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 48907ns | 46310ns | 49632ns | -1.44% | 0.021 |
| carrier_lay_madd_rec16 | 49189ns | 47558ns | 49849ns | -0.87% | 0.021 |
| carrier_lay_madd_rec20 | 49679ns | 49412ns | 49906ns | +0.11% | 0.021 |
| carrier_lay_madd_rec24 | 49623ns | 49320ns | 49814ns | base | 0.021 |
| carrier_lay_madd_rec32 | 49916ns | 49265ns | 50699ns | +0.59% | 0.021 |

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_madd_rec12; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 93.8% |
| carrier_lay_madd_rec16 | 0.021 | 93.7% |
| carrier_lay_madd_rec20 | 0.021 | 93.3% |
| carrier_lay_madd_rec24 | 0.021 | 93.3% |
| carrier_lay_madd_rec32 | 0.021 | 93.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 51422ns | 51422ns | -1.45% |
| carrier_lay_madd_rec16 | 51717ns | 51717ns | -0.89% |
| carrier_lay_madd_rec20 | 52185ns | 52185ns | +0.01% |
| carrier_lay_madd_rec24 | 52181ns | 52181ns | base |
| carrier_lay_madd_rec32 | 52450ns | 52450ns | +0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 49641ns | base | --- | [49415, 49814] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 49366ns | no significant difference | [-1862, +0]ns | [47724, 49632] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_madd_rec16 | 49409ns | no significant difference | [-1332, +210]ns | [48310, 49849] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 49629ns | no significant difference | [-205, +340]ns | [49502, 49906] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 49775ns | no significant difference | [-265, +885]ns | [49274, 50699] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 49627ns | -6.7% | -0.2% | +0.1% | +1.2% |
| 2 | 49775ns | -0.4% | -4.5% | -0.3% | +1.4% |
| 3 | 49655ns | -0.7% | -0.6% | -0.5% | -0.6% |
| 4 | 49510ns | +0.4% | -0.9% | +0.8% | -0.5% |
| 5 | 49853ns | -0.8% | +0.6% | +0.1% | +2.1% |
| 6 | 49320ns | -0.4% | +0.2% | +0.6% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.076 | ok |
| carrier_lay_madd_rec16 | -0.207 | moderate- |
| carrier_lay_madd_rec20 | -0.052 | ok |
| carrier_lay_madd_rec24 | -0.517 | HIGH- (thermal bounce) |
| carrier_lay_madd_rec32 | -0.424 | moderate- |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 5/6, lost 1/6
- **carrier_lay_madd_rec16**: won 4/6, lost 2/6
- **carrier_lay_madd_rec20**: won 2/6, lost 2/6
- **carrier_lay_madd_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 98062.9ns | 48907.3ns | 200.5% | HIGH |
| carrier_lay_madd_rec16 | 98593.2ns | 49189.3ns | 200.4% | HIGH |
| carrier_lay_madd_rec20 | 99516.5ns | 49679.0ns | 200.3% | HIGH |
| carrier_lay_madd_rec24 | 99409.3ns | 49623.1ns | 200.3% | HIGH |
| carrier_lay_madd_rec32 | 99957.8ns | 49915.9ns | 200.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 46310.0-49631.7 ns)
  46310.0 |####################
  46476.1 |
  46642.2 |
  46808.3 |
  46974.3 |
  47140.4 |
  47306.5 |
  47472.6 |
  47638.7 |
  47804.8 |
  47970.8 |
  48136.9 |
  48303.0 |
  48469.1 |
  48635.2 |
  48801.3 |
  48967.4 |
  49133.4 |########################################
  49299.5 |####################
  49465.6 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 47558.3-49848.9 ns)
  47558.3 |########################################
  47672.8 |
  47787.4 |
  47901.9 |
  48016.4 |
  48131.0 |
  48245.5 |
  48360.0 |
  48474.6 |
  48589.1 |
  48703.6 |
  48818.2 |
  48932.7 |
  49047.2 |########################################
  49161.8 |
  49276.3 |########################################
  49390.8 |########################################
  49505.4 |########################################
  49619.9 |
  49734.4 |
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 49412.5-49905.8 ns)
  49412.5 |####################
  49437.2 |
  49461.8 |
  49486.5 |
  49511.2 |
  49535.8 |
  49560.5 |
  49585.2 |########################################
  49609.8 |
  49634.5 |####################
  49659.2 |
  49683.8 |
  49708.5 |
  49733.2 |
  49757.8 |
  49782.5 |
  49807.2 |
  49831.8 |
  49856.5 |
  49881.2 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 49319.6-49813.8 ns)
  49319.6 |########################################
  49344.3 |
  49369.0 |
  49393.7 |
  49418.4 |
  49443.1 |
  49467.8 |
  49492.6 |########################################
  49517.3 |
  49542.0 |
  49566.7 |
  49591.4 |
  49616.1 |########################################
  49640.8 |########################################
  49665.5 |
  49690.2 |
  49714.9 |
  49739.6 |
  49764.3 |########################################
  49789.0 |
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 49265.4-50698.6 ns)
  49265.4 |########################################
  49337.1 |####################
  49408.7 |
  49480.4 |
  49552.0 |
  49623.7 |
  49695.3 |
  49767.0 |
  49838.7 |
  49910.3 |
  49982.0 |
  50053.6 |
  50125.3 |
  50196.9 |####################
  50268.6 |
  50340.3 |
  50411.9 |
  50483.6 |####################
  50555.2 |
  50626.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=200.6% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=200.4% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=200.3% of algo (FFI overhead may distort results)
