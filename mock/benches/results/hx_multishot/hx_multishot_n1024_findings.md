# Bounded multi-shot enumeration vs single-shot

2 variants, 6 samples per variant.
Baseline: **hx_multishot__single**

## Highlights

Baseline for all deltas below: **hx_multishot__single**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_multishot__single dominates: 505% faster than the next best (hx_multishot__multishot)

hx_multishot__single (131 ns) leads hx_multishot__multishot (795 ns) by 505%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (hx_multishot__single)

The baseline hx_multishot__single is the fastest (131 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.0x the fastest

Fastest hx_multishot__single (131 ns) to slowest hx_multishot__multishot (795 ns): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_multishot__single) is the fastest** at 131.4 ns median
- 1 variant significantly slower than baseline
- Spread: 6.05x (fastest 131.4 ns, slowest 794.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_multishot__multishot | 3240ns | 3370ns | 2781ns | 3299ns | 3379ns | +21.17% |
| hx_multishot__single | 2674ns | 2716ns | 2265ns | 2675ns | 2876ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_multishot__multishot | 765ns | 655ns | 800ns | +495.73% | 1.338 |
| hx_multishot__single | 128ns | 110ns | 134ns | base | 7.970 |

## Performance model

- Peak throughput: **9.275 Gops/s** (hx_multishot__single; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_multishot__multishot | 1.288 | 13.9% |
| hx_multishot__single | 7.790 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_multishot__multishot | 3240ns | 3240ns | +21.17% |
| hx_multishot__single | 2674ns | 2674ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_multishot__single | 131ns | base | --- | [120, 134] | --- | --- | --- | --- |
| hx_multishot__multishot | 795ns | +662.9ns (+504.3%) | [+580, +668]ns | [701, 800] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_multishot__single | hx_multishot__multishot |
|---|---|---|
| 1 | 110ns | +493.3% |
| 2 | 135ns | +489.5% |
| 3 | 134ns | +497.9% |
| 4 | 129ns | +516.3% |
| 5 | 131ns | +512.2% |
| 6 | 132ns | +465.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_multishot__multishot | -0.058 | ok |
| hx_multishot__single | -0.174 | ok |

**Consistency summary:**

- **hx_multishot__multishot**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_multishot__multishot | 3.2ns | 765.4ns | 0.4% |  |
| hx_multishot__single | 2.9ns | 128.5ns | 2.3% |  |

## Distribution (algo ns)

```
hx_multishot__multishot (n=6, range 655.0-800.4 ns)
    655.0 |#############
    662.3 |
    669.5 |
    676.8 |
    684.1 |
    691.4 |
    698.6 |
    705.9 |
    713.2 |
    720.4 |
    727.7 |
    735.0 |
    742.2 |#############
    749.5 |
    756.8 |
    764.0 |
    771.3 |
    778.6 |
    785.9 |
    793.1 |########################################
  (0 below, 1 above range)

hx_multishot__single (n=6, range 110.4-134.4 ns)
    110.4 |########################################
    111.6 |
    112.8 |
    114.0 |
    115.2 |
    116.4 |
    117.6 |
    118.8 |
    120.0 |
    121.2 |
    122.4 |
    123.6 |
    124.8 |
    126.0 |
    127.2 |
    128.4 |########################################
    129.6 |
    130.8 |########################################
    132.0 |########################################
    133.2 |########################################
  (0 below, 1 above range)

```
