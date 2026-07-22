# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 88% faster than the next best (zig_tail)

zig_switch (512.78 us) leads zig_tail (962.72 us) by 88%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 6.4%)

zig_switch wins on median (512.78 us) yet has the highest variance (CV 6.4%), while zig_tail is the steadiest (CV 1.0%, 962.72 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (512.78 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 512780.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.88x (fastest 512780.7 ns, slowest 962720.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 526583ns | 515500ns | 482421ns | 513374ns | 568477ns | base |
| zig_tail | 962640ns | 965564ns | 948895ns | 961018ns | 971945ns | +82.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 523847ns | 479575ns | 565740ns | base | 0.016 |
| zig_tail | 959848ns | 946482ns | 968998ns | +83.23% | 0.009 |

## Performance model

- Peak throughput: **0.017 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.016 | 93.5% |
| zig_tail | 0.009 | 49.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 526583ns | 526583ns | base |
| zig_tail | 962640ns | 962640ns | +82.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 512781ns | base | --- | [493021, 565740] | --- | --- | --- | --- |
| zig_tail | 962721ns | +446656.8ns (+87.1%) | [+391225, +470121]ns | [947826, 968998] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 515660ns | +88.7% |
| 2 | 551678ns | +74.9% |
| 3 | 479575ns | +100.6% |
| 4 | 509902ns | +85.6% |
| 5 | 506468ns | +90.2% |
| 6 | 579803ns | +63.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.245 | moderate- |
| zig_tail | -0.068 | ok |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 7.8ns | 523847.4ns | 0.0% |  |
| zig_tail | 5.1ns | 959848.2ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 479574.6-565740.2 ns)
  479574.6 |########################################
  483882.9 |
  488191.2 |
  492499.4 |
  496807.7 |
  501116.0 |
  505424.3 |########################################
  509732.6 |########################################
  514040.8 |########################################
  518349.1 |
  522657.4 |
  526965.7 |
  531274.0 |
  535582.2 |
  539890.5 |
  544198.8 |
  548507.1 |########################################
  552815.4 |
  557123.6 |
  561431.9 |
  (0 below, 1 above range)

zig_tail (n=6, range 946482.5-968998.1 ns)
  946482.5 |########################################
  947608.3 |
  948734.1 |########################################
  949859.8 |
  950985.6 |
  952111.4 |
  953237.2 |
  954363.0 |
  955488.7 |
  956614.5 |
  957740.3 |
  958866.1 |
  959991.9 |
  961117.6 |########################################
  962243.4 |########################################
  963369.2 |
  964495.0 |########################################
  965620.8 |
  966746.5 |
  967872.3 |
  (0 below, 1 above range)

```
