# Native ceiling: switch vs fn-table interp vs shape-specialized native, opaque program (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_ceiling_native**

## Highlights

Baseline for all deltas below: **carrier_ceiling_native**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ceiling_native dominates: 101% faster than the next best (carrier_ceiling_switch)

carrier_ceiling_native (5.02 us) leads carrier_ceiling_switch (10.08 us) by 101%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceiling_fntable is an outlier: 2.2x slower than the field

carrier_ceiling_fntable (10.85 us) is 2.2x the fastest (5.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_ceiling_switch shows alternating (throttle bounce) (autocorr -0.57)

carrier_ceiling_switch's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_ceiling_native)

The baseline carrier_ceiling_native is the fastest (5.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_ceiling_native) is the fastest** at 5022.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.16x (fastest 5022.5 ns, slowest 10850.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceiling_fntable | 13193ns | 13449ns | 10893ns | 13444ns | 13966ns | +78.31% |
| carrier_ceiling_native | 7399ns | 7561ns | 6534ns | 7395ns | 7837ns | base |
| carrier_ceiling_switch | 11940ns | 12677ns | 10268ns | 11988ns | 12704ns | +61.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceiling_fntable | 10650ns | 8778ns | 11286ns | +115.81% | 0.024 |
| carrier_ceiling_native | 4935ns | 4354ns | 5250ns | base | 0.052 |
| carrier_ceiling_switch | 9491ns | 8159ns | 10105ns | +92.33% | 0.027 |

## Performance model

- Peak throughput: **0.059 Gops/s** (carrier_ceiling_native; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceiling_fntable | 0.024 | 40.1% |
| carrier_ceiling_native | 0.051 | 86.7% |
| carrier_ceiling_switch | 0.025 | 43.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceiling_fntable | 13193ns | 13193ns | +78.31% |
| carrier_ceiling_native | 7399ns | 7399ns | base |
| carrier_ceiling_switch | 11940ns | 11940ns | +61.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceiling_native | 5022ns | base | --- | [4533, 5250] | --- | --- | --- | --- |
| carrier_ceiling_fntable | 10850ns | +5725.1ns (+114.0%) | [+4758, +6662]ns | [9813, 11286] | YES | 0.0313 | 0.0313 | 0 |
| carrier_ceiling_switch | 10077ns | +4830.5ns (+96.2%) | [+3759, +5079]ns | [8292, 10105] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceiling_native | carrier_ceiling_fntable | carrier_ceiling_switch |
|---|---|---|---|
| 1 | 4854ns | +80.8% | +107.6% |
| 2 | 4354ns | +149.2% | +93.5% |
| 3 | 5191ns | +112.5% | +95.1% |
| 4 | 5243ns | +107.0% | +92.3% |
| 5 | 4711ns | +145.0% | +73.2% |
| 6 | 5256ns | +106.4% | +91.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceiling_fntable | 0.030 | ok |
| carrier_ceiling_native | -0.249 | moderate- |
| carrier_ceiling_switch | -0.573 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_ceiling_fntable**: won 0/6, lost 6/6
- **carrier_ceiling_switch**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceiling_fntable | 94.9ns | 10649.8ns | 0.9% |  |
| carrier_ceiling_native | 5.6ns | 4934.9ns | 0.1% |  |
| carrier_ceiling_switch | 91.7ns | 9491.2ns | 1.0% |  |

## Distribution (algo ns)

```
carrier_ceiling_fntable (n=6, range 8777.9-11286.2 ns)
   8777.9 |#############
   8903.3 |
   9028.7 |
   9154.2 |
   9279.6 |
   9405.0 |
   9530.4 |
   9655.8 |
   9781.2 |
   9906.7 |
  10032.1 |
  10157.5 |
  10282.9 |
  10408.3 |
  10533.7 |
  10659.2 |
  10784.6 |########################################
  10910.0 |#############
  11035.4 |
  11160.8 |
  (0 below, 1 above range)

carrier_ceiling_native (n=6, range 4354.2-5249.5 ns)
   4354.2 |########################################
   4399.0 |
   4443.7 |
   4488.5 |
   4533.3 |
   4578.0 |
   4622.8 |
   4667.6 |########################################
   4712.3 |
   4757.1 |
   4801.9 |
   4846.6 |########################################
   4891.4 |
   4936.2 |
   4980.9 |
   5025.7 |
   5070.5 |
   5115.2 |
   5160.0 |########################################
   5204.8 |########################################
  (0 below, 1 above range)

carrier_ceiling_switch (n=6, range 8159.2-10104.5 ns)
   8159.2 |#############
   8256.5 |
   8353.7 |#############
   8451.0 |
   8548.3 |
   8645.5 |
   8742.8 |
   8840.1 |
   8937.3 |
   9034.6 |
   9131.9 |
   9229.1 |
   9326.4 |
   9423.7 |
   9520.9 |
   9618.2 |
   9715.5 |
   9812.7 |
   9910.0 |
  10007.3 |########################################
  (0 below, 1 above range)

```
