# Native ceiling: switch vs fn-table interp vs shape-specialized native, opaque program (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_ceiling_native**

## Highlights

Baseline for all deltas below: **carrier_ceiling_native**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ceiling_native dominates: 102% faster than the next best (carrier_ceiling_switch)

carrier_ceiling_native (70.08 us) leads carrier_ceiling_switch (141.44 us) by 102%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceiling_fntable is an outlier: 2.1x slower than the field

carrier_ceiling_fntable (148.81 us) is 2.1x the fastest (70.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_ceiling_native shows alternating (throttle bounce) (autocorr -0.56)

carrier_ceiling_native's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_ceiling_native)

The baseline carrier_ceiling_native is the fastest (70.08 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_ceiling_native) is the fastest** at 70077.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.12x (fastest 70077.9 ns, slowest 148811.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceiling_fntable | 150779ns | 151256ns | 145279ns | 150517ns | 153922ns | +106.19% |
| carrier_ceiling_native | 73128ns | 72592ns | 70789ns | 72046ns | 75919ns | base |
| carrier_ceiling_switch | 143399ns | 143797ns | 138088ns | 142997ns | 146659ns | +96.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceiling_fntable | 148410ns | 143045ns | 151499ns | +110.01% | 0.028 |
| carrier_ceiling_native | 70667ns | 68448ns | 73352ns | base | 0.058 |
| carrier_ceiling_switch | 140997ns | 135648ns | 144221ns | +99.52% | 0.029 |

## Performance model

- Peak throughput: **0.060 Gops/s** (carrier_ceiling_native; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceiling_fntable | 0.028 | 46.0% |
| carrier_ceiling_native | 0.058 | 97.7% |
| carrier_ceiling_switch | 0.029 | 48.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceiling_fntable | 150779ns | 150779ns | +106.19% |
| carrier_ceiling_native | 73128ns | 73128ns | base |
| carrier_ceiling_switch | 143399ns | 143399ns | +96.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceiling_native | 70078ns | base | --- | [68572, 73352] | --- | --- | --- | --- |
| carrier_ceiling_fntable | 148811ns | +77923.4ns (+111.2%) | [+75312, +79991]ns | [144919, 151499] | YES | 0.0313 | 0.0313 | 0 |
| carrier_ceiling_switch | 141440ns | +71210.6ns (+101.6%) | [+66264, +73514]ns | [137330, 144221] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceiling_native | carrier_ceiling_fntable | carrier_ceiling_switch |
|---|---|---|---|
| 1 | 68696ns | +108.2% | +106.2% |
| 2 | 73594ns | +106.1% | +98.3% |
| 3 | 71135ns | +107.2% | +98.6% |
| 4 | 69021ns | +112.7% | +96.5% |
| 5 | 73110ns | +107.0% | +90.1% |
| 6 | 68448ns | +119.5% | +108.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceiling_fntable | -0.339 | moderate- |
| carrier_ceiling_native | -0.556 | HIGH- (thermal bounce) |
| carrier_ceiling_switch | 0.178 | ok |

**Consistency summary:**

- **carrier_ceiling_fntable**: won 0/6, lost 6/6
- **carrier_ceiling_switch**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceiling_fntable | 1098.1ns | 148409.6ns | 0.7% |  |
| carrier_ceiling_native | 5.1ns | 70667.3ns | 0.0% |  |
| carrier_ceiling_switch | 1138.8ns | 140997.1ns | 0.8% |  |

## Distribution (algo ns)

```
carrier_ceiling_fntable (n=6, range 143045.0-151498.5 ns)
  143045.0 |########################################
  143467.7 |
  143890.4 |
  144313.0 |
  144735.7 |
  145158.4 |
  145581.1 |
  146003.7 |
  146426.4 |########################################
  146849.1 |
  147271.8 |########################################
  147694.5 |
  148117.1 |
  148539.8 |
  148962.5 |
  149385.2 |
  149807.8 |########################################
  150230.5 |
  150653.2 |
  151075.9 |########################################
  (0 below, 1 above range)

carrier_ceiling_native (n=6, range 68447.9-73352.1 ns)
  68447.9 |########################################
  68693.1 |########################################
  68938.3 |########################################
  69183.5 |
  69428.7 |
  69673.9 |
  69919.2 |
  70164.4 |
  70409.6 |
  70654.8 |
  70900.0 |########################################
  71145.2 |
  71390.4 |
  71635.6 |
  71880.8 |
  72126.1 |
  72371.3 |
  72616.5 |
  72861.7 |
  73106.9 |########################################
  (0 below, 1 above range)

carrier_ceiling_switch (n=6, range 135647.9-144221.0 ns)
  135647.9 |####################
  136076.6 |
  136505.2 |
  136933.9 |
  137362.5 |
  137791.2 |
  138219.8 |
  138648.5 |####################
  139077.2 |
  139505.8 |
  139934.5 |
  140363.1 |
  140791.8 |
  141220.4 |########################################
  141649.1 |
  142077.8 |
  142506.4 |####################
  142935.1 |
  143363.7 |
  143792.4 |
  (0 below, 1 above range)

```
