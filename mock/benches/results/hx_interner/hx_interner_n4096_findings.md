# Interner intern hot path: FNV vs FxHash

2 variants, 6 samples per variant.
Baseline: **hx_interner__fnv**

## Highlights

Baseline for all deltas below: **hx_interner__fnv**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_interner__fnv is fastest but the noisiest (CV 5.7%)

hx_interner__fnv wins on median (40.75 us) yet has the highest variance (CV 5.7%), while hx_interner__fx is the steadiest (CV 1.8%, 43.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_interner__fnv)

The baseline hx_interner__fnv is the fastest (40.75 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader hx_interner__fnv vs stability leader hx_interner__fx (+8% speed for 3.1x steadier)

hx_interner__fnv is fastest (40.75 us, CV 5.7%); hx_interner__fx gives up 7.7% median for 3.1x lower variance (CV 1.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (hx_interner__fnv) is the fastest** at 40750.7 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 40750.7 ns, slowest 43901.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_interner__fnv | 43387ns | 43026ns | 41138ns | 42494ns | 45850ns | base |
| hx_interner__fx | 46228ns | 46155ns | 45081ns | 45916ns | 47269ns | +6.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_interner__fnv | 41151ns | 38971ns | 43592ns | base | 0.100 |
| hx_interner__fx | 43946ns | 42895ns | 44884ns | +6.79% | 0.093 |

## Performance model

- Peak throughput: **0.105 Gops/s** (hx_interner__fnv; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_interner__fnv | 0.101 | 95.6% |
| hx_interner__fx | 0.093 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_interner__fnv | 43387ns | 43387ns | base |
| hx_interner__fx | 46228ns | 46228ns | +6.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_interner__fnv | 40751ns | base | --- | [39111, 43592] | --- | --- | --- | --- |
| hx_interner__fx | 43902ns | +3505.5ns (+8.6%) | [+540, +4338]ns | [43051, 44884] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_interner__fnv | hx_interner__fx |
|---|---|---|
| 1 | 38971ns | +11.6% |
| 2 | 39251ns | +9.3% |
| 3 | 40474ns | +6.8% |
| 4 | 41236ns | +8.2% |
| 5 | 45948ns | -3.6% |
| 6 | 41028ns | +10.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_interner__fnv | 0.163 | ok |
| hx_interner__fx | 0.362 | moderate+ |

**Consistency summary:**

- **hx_interner__fx**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_interner__fnv | 2.4ns | 41151.2ns | 0.0% |  |
| hx_interner__fx | 3.1ns | 43945.6ns | 0.0% |  |

## Distribution (algo ns)

```
hx_interner__fnv (n=6, range 38970.8-43591.9 ns)
  38970.8 |########################################
  39201.9 |########################################
  39432.9 |
  39664.0 |
  39895.0 |
  40126.1 |
  40357.1 |########################################
  40588.2 |
  40819.2 |########################################
  41050.3 |########################################
  41281.3 |
  41512.4 |
  41743.4 |
  41974.5 |
  42205.5 |
  42436.6 |
  42667.6 |
  42898.7 |
  43129.7 |
  43360.8 |
  (0 below, 1 above range)

hx_interner__fx (n=6, range 42895.4-44883.6 ns)
  42895.4 |########################################
  42994.8 |
  43094.2 |
  43193.6 |########################################
  43293.0 |
  43392.4 |
  43491.8 |########################################
  43591.3 |
  43690.7 |
  43790.1 |
  43889.5 |
  43988.9 |
  44088.3 |
  44187.7 |
  44287.1 |########################################
  44386.5 |
  44485.9 |
  44585.3 |########################################
  44684.7 |
  44784.1 |
  (0 below, 1 above range)

```
