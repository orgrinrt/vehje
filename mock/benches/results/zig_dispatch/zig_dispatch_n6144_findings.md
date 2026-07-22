# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 119% faster than the next best (zig_tail)

zig_switch (329.14 us) leads zig_tail (720.51 us) by 119%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 5.4%)

zig_switch wins on median (329.14 us) yet has the highest variance (CV 5.4%), while zig_tail is the steadiest (CV 2.0%, 720.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (329.14 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 329138.3 ns median
- 1 variant significantly slower than baseline
- Spread: 2.19x (fastest 329138.3 ns, slowest 720508.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 326879ns | 331645ns | 300067ns | 323281ns | 345682ns | base |
| zig_tail | 717004ns | 723196ns | 684825ns | 721987ns | 725620ns | +119.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 324280ns | 297793ns | 342870ns | base | 0.019 |
| zig_tail | 714069ns | 682485ns | 722316ns | +120.20% | 0.009 |

## Performance model

- Peak throughput: **0.021 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 6144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.019 | 90.5% |
| zig_tail | 0.009 | 41.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 326879ns | 326879ns | base |
| zig_tail | 717004ns | 717004ns | +119.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 329138ns | base | --- | [300831, 342870] | --- | --- | --- | --- |
| zig_tail | 720509ns | +382485.8ns (+116.2%) | [+367205, +419677]ns | [699383, 722316] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 332669ns | +115.3% |
| 2 | 303870ns | +137.4% |
| 3 | 325608ns | +109.6% |
| 4 | 345652ns | +109.2% |
| 5 | 340088ns | +112.1% |
| 6 | 297793ns | +141.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.132 | ok |
| zig_tail | -0.322 | moderate- |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 5.1ns | 324279.9ns | 0.0% |  |
| zig_tail | 3.2ns | 714069.4ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 297792.9-342870.0 ns)
  297792.9 |########################################
  300046.8 |
  302300.6 |########################################
  304554.5 |
  306808.3 |
  309062.2 |
  311316.0 |
  313569.9 |
  315823.7 |
  318077.6 |
  320331.5 |
  322585.3 |
  324839.2 |########################################
  327093.0 |
  329346.9 |
  331600.7 |########################################
  333854.6 |
  336108.4 |
  338362.3 |########################################
  340616.1 |
  (0 below, 1 above range)

zig_tail (n=6, range 682485.4-722316.4 ns)
  682485.4 |####################
  684477.0 |
  686468.5 |
  688460.1 |
  690451.6 |
  692443.2 |
  694434.7 |
  696426.3 |
  698417.8 |
  700409.4 |
  702400.9 |
  704392.5 |
  706384.0 |
  708375.6 |
  710367.1 |
  712358.7 |
  714350.2 |####################
  716341.8 |
  718333.3 |####################
  720324.9 |########################################
  (0 below, 1 above range)

```
