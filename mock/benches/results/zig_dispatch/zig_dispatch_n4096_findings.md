# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 243% faster than the next best (zig_tail)

zig_switch (130.20 us) leads zig_tail (446.29 us) by 243%, a clear separation rather than a photo finish. CV 13.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 13.6%)

zig_switch wins on median (130.20 us) yet has the highest variance (CV 13.6%), while zig_tail is the steadiest (CV 5.4%, 446.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (130.20 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.4x the fastest

Fastest zig_switch (130.20 us) to slowest zig_tail (446.29 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (zig_switch) is the fastest** at 130202.3 ns median
- 1 variant significantly slower than baseline
- Spread: 3.43x (fastest 130202.3 ns, slowest 446291.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 137500ns | 132365ns | 111245ns | 131372ns | 159820ns | base |
| zig_tail | 438561ns | 448649ns | 396206ns | 437792ns | 460892ns | +218.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 135130ns | 108679ns | 157256ns | base | 0.030 |
| zig_tail | 436215ns | 393977ns | 458439ns | +222.81% | 0.009 |

## Performance model

- Peak throughput: **0.038 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.031 | 83.5% |
| zig_tail | 0.009 | 24.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 137500ns | 137500ns | base |
| zig_tail | 438561ns | 438561ns | +218.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 130202ns | base | --- | [117930, 157256] | --- | --- | --- | --- |
| zig_tail | 446291ns | +296844.6ns (+228.0%) | [+273834, +332578]ns | [403915, 458439] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 127182ns | +209.8% |
| 2 | 128665ns | +221.7% |
| 3 | 108679ns | +312.4% |
| 4 | 131740ns | +247.2% |
| 5 | 151036ns | +204.3% |
| 6 | 163476ns | +171.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.379 | moderate+ |
| zig_tail | 0.463 | moderate+ |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 1.8ns | 135129.5ns | 0.0% |  |
| zig_tail | 3.2ns | 436215.1ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 108679.2-157255.8 ns)
  108679.2 |########################################
  111108.0 |
  113536.9 |
  115965.7 |
  118394.5 |
  120823.3 |
  123252.2 |
  125681.0 |########################################
  128109.8 |########################################
  130538.7 |########################################
  132967.5 |
  135396.3 |
  137825.2 |
  140254.0 |
  142682.8 |
  145111.6 |
  147540.5 |
  149969.3 |########################################
  152398.1 |
  154827.0 |
  (0 below, 1 above range)

zig_tail (n=6, range 393976.7-458439.0 ns)
  393976.7 |########################################
  397199.8 |
  400422.9 |
  403646.0 |
  406869.2 |
  410092.3 |
  413315.4 |########################################
  416538.5 |
  419761.6 |
  422984.7 |
  426207.8 |
  429430.9 |
  432654.1 |
  435877.2 |
  439100.3 |
  442323.4 |########################################
  445546.5 |########################################
  448769.6 |
  451992.7 |
  455215.8 |########################################
  (0 below, 1 above range)

```
