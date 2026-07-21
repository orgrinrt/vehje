# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 190% faster than the next best (zig_tail)

zig_switch (148.10 us) leads zig_tail (429.98 us) by 190%, a clear separation rather than a photo finish. CV 8.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### zig_switch is fastest but the noisiest (CV 8.0%)

zig_switch wins on median (148.10 us) yet has the highest variance (CV 8.0%), while zig_tail is the steadiest (CV 2.9%, 429.98 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (148.10 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (zig_switch) is the fastest** at 148105.0 ns median
- 1 variant significantly slower than baseline
- Spread: 2.90x (fastest 148105.0 ns, slowest 429981.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 148711ns | 150315ns | 127855ns | 147541ns | 160893ns | base |
| zig_tail | 431146ns | 432584ns | 408727ns | 430204ns | 443766ns | +189.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 146440ns | 125649ns | 158509ns | base | 0.028 |
| zig_tail | 428676ns | 406505ns | 441236ns | +192.73% | 0.010 |

## Performance model

- Peak throughput: **0.033 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.028 | 84.8% |
| zig_tail | 0.010 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 148711ns | 148711ns | base |
| zig_tail | 431146ns | 431146ns | +189.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 148105ns | base | --- | [132704, 158509] | --- | --- | --- | --- |
| zig_tail | 429981ns | +282106.0ns (+190.5%) | [+272746, +291857]ns | [414810, 441236] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 125649ns | +223.5% |
| 2 | 153996ns | +191.3% |
| 3 | 157232ns | +175.9% |
| 4 | 159787ns | +168.3% |
| 5 | 142214ns | +203.3% |
| 6 | 139760ns | +202.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | 0.048 | ok |
| zig_tail | -0.371 | moderate- |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 1.9ns | 146439.5ns | 0.0% |  |
| zig_tail | 5.7ns | 428675.8ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 125648.8-158509.2 ns)
  125648.8 |########################################
  127291.8 |
  128934.8 |
  130577.9 |
  132220.9 |
  133863.9 |
  135506.9 |
  137149.9 |
  138793.0 |########################################
  140436.0 |
  142079.0 |########################################
  143722.0 |
  145365.0 |
  147008.1 |
  148651.1 |
  150294.1 |
  151937.1 |
  153580.1 |########################################
  155223.2 |
  156866.2 |########################################
  (0 below, 1 above range)

zig_tail (n=6, range 406505.4-441235.6 ns)
  406505.4 |########################################
  408241.9 |
  409978.4 |
  411714.9 |
  413451.4 |
  415188.0 |
  416924.5 |
  418661.0 |
  420397.5 |
  422134.0 |########################################
  423870.5 |
  425607.0 |
  427343.5 |########################################
  429080.0 |
  430816.5 |########################################
  432553.0 |########################################
  434289.6 |
  436026.1 |
  437762.6 |
  439499.1 |
  (0 below, 1 above range)

```
