# Record width: wire-format stride and inline-vs-pool decode cost (carrier)

5 variants, 6 samples per variant.
Baseline: **carrier_rec24**

## Highlights

Baseline for all deltas below: **carrier_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (carrier_rec20, carrier_rec32) are a dead heat (<1%)

carrier_rec20 (34.12 us) and carrier_rec32 (34.23 us) differ by 0.35%, inside the noise, even though the wider field spreads 4.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_rec12 shows alternating (throttle bounce) (autocorr -0.65)

carrier_rec12's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_rec12 is inconsistent: worst-20% is 1.6x its best-20%

carrier_rec12's best 20% of batches run at 34.15 us but its worst 20% at 54.43 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 4.2% of the fastest

All 5 variants sit between 34.12 us and 35.54 us - a 4.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### carrier_rec16's edge over baseline is significant but tiny (41 ns, 0.12%)

carrier_rec16 differs from baseline carrier_rec24 by 41 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_rec20** at 34115.2 ns median (-3.0% vs baseline)
- Spread: 1.04x (fastest 34115.2 ns, slowest 35536.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_rec12 | 43725ns | 37959ns | 36292ns | 37640ns | 56568ns | +8.41% |
| carrier_rec16 | 38020ns | 36965ns | 36053ns | 36750ns | 40910ns | -5.73% |
| carrier_rec20 | 36448ns | 36416ns | 36095ns | 36311ns | 36828ns | -9.63% |
| carrier_rec24 | 40333ns | 37499ns | 35884ns | 37178ns | 47288ns | base |
| carrier_rec32 | 36366ns | 36376ns | 36087ns | 36318ns | 36578ns | -9.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_rec12 | 41479ns | 34155ns | 54431ns | +8.81% | 0.025 |
| carrier_rec16 | 35845ns | 33941ns | 38743ns | -5.97% | 0.029 |
| carrier_rec20 | 34220ns | 33915ns | 34616ns | -10.23% | 0.030 |
| carrier_rec24 | 38122ns | 33762ns | 45162ns | base | 0.027 |
| carrier_rec32 | 34178ns | 33760ns | 34386ns | -10.35% | 0.030 |

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_rec32; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_rec12 | 0.029 | 95.0% |
| carrier_rec16 | 0.029 | 97.2% |
| carrier_rec20 | 0.030 | 99.0% |
| carrier_rec24 | 0.029 | 96.0% |
| carrier_rec32 | 0.030 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_rec12 | 43725ns | 43725ns | +8.41% |
| carrier_rec16 | 38020ns | 38020ns | -5.73% |
| carrier_rec20 | 36448ns | 36448ns | -9.63% |
| carrier_rec24 | 40333ns | 40333ns | base |
| carrier_rec32 | 36366ns | 36366ns | -9.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_rec24 | 35169ns | base | --- | [34034, 45162] | --- | --- | --- | --- |
| carrier_rec12 | 35536ns | no significant difference | [-9778, +19371]ns | [34468, 54431] | no | 0.8750 | 0.2188 | 0 |
| carrier_rec16 | 34718ns | no significant difference | [-9401, +2531]ns | [34075, 38743] | no | 1.0000 | 1.0000 | 0 |
| carrier_rec20 | 34115ns | no significant difference | [-11234, +582]ns | [33929, 34616] | no | 0.9167 | 0.6875 | 0 |
| carrier_rec32 | 34234ns | no significant difference | [-11085, +188]ns | [33914, 34386] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_rec24 | carrier_rec12 | carrier_rec16 | carrier_rec20 | carrier_rec32 |
|---|---|---|---|---|---|
| 1 | 34527ns | +0.7% | +1.3% | -1.0% | -1.0% |
| 2 | 33762ns | +64.7% | +0.5% | +2.0% | +0.9% |
| 3 | 53966ns | -36.7% | -32.3% | -37.2% | -37.4% |
| 4 | 36359ns | +46.5% | +12.6% | -6.6% | -5.4% |
| 5 | 34305ns | +1.4% | -0.3% | +1.5% | +0.2% |
| 6 | 35810ns | +1.3% | -3.8% | -4.9% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_rec12 | -0.646 | HIGH- (thermal bounce) |
| carrier_rec16 | -0.063 | ok |
| carrier_rec20 | -0.421 | moderate- |
| carrier_rec24 | -0.215 | moderate- |
| carrier_rec32 | 0.061 | ok |

**Consistency summary:**

- **carrier_rec12**: won 1/6, lost 5/6
- **carrier_rec16**: won 3/6, lost 3/6
- **carrier_rec20**: won 4/6, lost 2/6
- **carrier_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_rec12 | 156.0ns | 41478.6ns | 0.4% |  |
| carrier_rec16 | 140.4ns | 35845.1ns | 0.4% |  |
| carrier_rec20 | 142.9ns | 34220.1ns | 0.4% |  |
| carrier_rec24 | 145.2ns | 38121.7ns | 0.4% |  |
| carrier_rec32 | 143.9ns | 34177.8ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_rec12 (n=6, range 34154.6-54431.2 ns)
  34154.6 |########################################
  35168.4 |
  36182.3 |#############
  37196.1 |
  38209.9 |
  39223.8 |
  40237.6 |
  41251.4 |
  42265.3 |
  43279.1 |
  44292.9 |
  45306.8 |
  46320.6 |
  47334.4 |
  48348.3 |
  49362.1 |
  50375.9 |
  51389.8 |
  52403.6 |#############
  53417.4 |
  (0 below, 1 above range)

carrier_rec16 (n=6, range 33940.8-38743.1 ns)
  33940.8 |########################################
  34180.9 |########################################
  34421.0 |########################################
  34661.1 |
  34901.3 |########################################
  35141.4 |
  35381.5 |
  35621.6 |
  35861.7 |
  36101.8 |
  36341.9 |########################################
  36582.1 |
  36822.2 |
  37062.3 |
  37302.4 |
  37542.5 |
  37782.6 |
  38022.8 |
  38262.9 |
  38503.0 |
  (0 below, 1 above range)

carrier_rec20 (n=6, range 33915.4-34616.4 ns)
  33915.4 |########################################
  33950.5 |
  33985.5 |
  34020.6 |####################
  34055.6 |
  34090.7 |
  34125.7 |
  34160.8 |####################
  34195.8 |
  34230.9 |
  34265.9 |
  34301.0 |
  34336.0 |
  34371.1 |
  34406.1 |####################
  34441.2 |
  34476.2 |
  34511.3 |
  34546.3 |
  34581.4 |
  (0 below, 1 above range)

carrier_rec24 (n=6, range 33762.5-45162.5 ns)
  33762.5 |########################################
  34332.5 |####################
  34902.5 |
  35472.5 |####################
  36042.5 |####################
  36612.5 |
  37182.5 |
  37752.5 |
  38322.5 |
  38892.5 |
  39462.5 |
  40032.5 |
  40602.5 |
  41172.5 |
  41742.5 |
  42312.5 |
  42882.5 |
  43452.5 |
  44022.5 |
  44592.5 |
  (0 below, 1 above range)

carrier_rec32 (n=6, range 33760.0-34385.8 ns)
  33760.0 |########################################
  33791.3 |
  33822.6 |
  33853.9 |
  33885.2 |
  33916.5 |
  33947.8 |
  33979.0 |
  34010.3 |
  34041.6 |########################################
  34072.9 |
  34104.2 |
  34135.5 |
  34166.8 |########################################
  34198.1 |
  34229.4 |
  34260.7 |########################################
  34292.0 |
  34323.3 |
  34354.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_rec12**: CV=22.2% (high variance, measurements may be unstable)
