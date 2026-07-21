# Node record width: 16B pool-spill vs 24B inline operands

2 variants, 6 samples per variant.
Baseline: **hx_recwidth__rec24**

## Highlights

Baseline for all deltas below: **hx_recwidth__rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_recwidth__rec24 dominates: 29% faster than the next best (hx_recwidth__rec16)

hx_recwidth__rec24 (335 ns) leads hx_recwidth__rec16 (431 ns) by 29%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_recwidth__rec24 is fastest but the noisiest (CV 6.1%)

hx_recwidth__rec24 wins on median (335 ns) yet has the highest variance (CV 6.1%), while hx_recwidth__rec16 is the steadiest (CV 5.2%, 431 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_recwidth__rec24)

The baseline hx_recwidth__rec24 is the fastest (335 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (hx_recwidth__rec24) is the fastest** at 334.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.29x (fastest 334.8 ns, slowest 430.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_recwidth__rec16 | 2970ns | 2929ns | 2822ns | 2897ns | 3156ns | +3.45% |
| hx_recwidth__rec24 | 2871ns | 2861ns | 2446ns | 2824ns | 3154ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_recwidth__rec16 | 437ns | 418ns | 462ns | +33.12% | 0.586 |
| hx_recwidth__rec24 | 328ns | 288ns | 345ns | base | 0.780 |

## Performance model

- Peak throughput: **0.889 Gops/s** (hx_recwidth__rec24; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_recwidth__rec16 | 0.594 | 66.9% |
| hx_recwidth__rec24 | 0.765 | 86.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_recwidth__rec16 | 2970ns | 2970ns | +3.45% |
| hx_recwidth__rec24 | 2871ns | 2871ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_recwidth__rec24 | 335ns | base | --- | [304, 345] | --- | --- | --- | --- |
| hx_recwidth__rec16 | 431ns | +107.7ns (+32.2%) | [+85, +134]ns | [418, 462] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_recwidth__rec24 | hx_recwidth__rec16 |
|---|---|---|
| 1 | 288ns | +45.2% |
| 2 | 343ns | +29.1% |
| 3 | 342ns | +40.1% |
| 4 | 347ns | +20.7% |
| 5 | 321ns | +30.4% |
| 6 | 327ns | +35.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_recwidth__rec16 | -0.132 | ok |
| hx_recwidth__rec24 | -0.101 | ok |

**Consistency summary:**

- **hx_recwidth__rec16**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_recwidth__rec16 | 5.0ns | 436.8ns | 1.2% |  |
| hx_recwidth__rec24 | 4.8ns | 328.1ns | 1.5% |  |

## Distribution (algo ns)

```
hx_recwidth__rec16 (n=6, range 417.9-461.6 ns)
    417.9 |########################################
    420.1 |
    422.3 |
    424.5 |
    426.6 |
    428.8 |
    431.0 |
    433.2 |
    435.4 |
    437.6 |
    439.8 |
    442.0 |##########################
    444.1 |
    446.3 |
    448.5 |
    450.7 |
    452.9 |
    455.1 |
    457.3 |
    459.5 |
  (0 below, 1 above range)

hx_recwidth__rec24 (n=6, range 287.9-345.2 ns)
    287.9 |####################
    290.8 |
    293.6 |
    296.5 |
    299.4 |
    302.2 |
    305.1 |
    308.0 |
    310.8 |
    313.7 |
    316.6 |
    319.4 |####################
    322.3 |
    325.1 |####################
    328.0 |
    330.9 |
    333.7 |
    336.6 |
    339.5 |
    342.3 |########################################
  (0 below, 1 above range)

```
