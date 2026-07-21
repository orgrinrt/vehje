# Name resolution: linear scope-chain vs flat shadow-stack

2 variants, 6 samples per variant.
Baseline: **hx_resolve__flat**

## Highlights

Baseline for all deltas below: **hx_resolve__flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_resolve__flat dominates: 887% faster than the next best (hx_resolve__linear)

hx_resolve__flat (1.18 us) leads hx_resolve__linear (11.63 us) by 887%, a clear separation rather than a photo finish. CV 6.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_resolve__flat is fastest but the noisiest (CV 6.5%)

hx_resolve__flat wins on median (1.18 us) yet has the highest variance (CV 6.5%), while hx_resolve__linear is the steadiest (CV 3.4%, 11.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_resolve__flat)

The baseline hx_resolve__flat is the fastest (1.18 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.9x the fastest

Fastest hx_resolve__flat (1.18 us) to slowest hx_resolve__linear (11.63 us): 9.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_resolve__flat) is the fastest** at 1179.2 ns median
- 1 variant significantly slower than baseline
- Spread: 9.87x (fastest 1179.2 ns, slowest 11633.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_resolve__flat | 3693ns | 3750ns | 3535ns | 3686ns | 3783ns | base |
| hx_resolve__linear | 14098ns | 14204ns | 13292ns | 14036ns | 14594ns | +281.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_resolve__flat | 1143ns | 990ns | 1205ns | base | 0.896 |
| hx_resolve__linear | 11570ns | 10884ns | 11990ns | +911.87% | 0.089 |

## Performance model

- Peak throughput: **1.035 Gops/s** (hx_resolve__flat; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_resolve__flat | 0.868 | 83.9% |
| hx_resolve__linear | 0.088 | 8.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_resolve__flat | 3693ns | 3693ns | base |
| hx_resolve__linear | 14098ns | 14098ns | +281.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_resolve__flat | 1179ns | base | --- | [1046, 1205] | --- | --- | --- | --- |
| hx_resolve__linear | 11634ns | +10441.5ns (+885.5%) | [+9989, +10848]ns | [11085, 11990] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_resolve__flat | hx_resolve__linear |
|---|---|---|
| 1 | 990ns | +1040.4% |
| 2 | 1103ns | +994.1% |
| 3 | 1203ns | +804.8% |
| 4 | 1206ns | +855.5% |
| 5 | 1180ns | +909.2% |
| 6 | 1178ns | +896.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_resolve__flat | 0.314 | moderate+ |
| hx_resolve__linear | -0.435 | moderate- |

**Consistency summary:**

- **hx_resolve__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_resolve__flat | 4.1ns | 1143.4ns | 0.4% |  |
| hx_resolve__linear | 3.2ns | 11569.5ns | 0.0% |  |

## Distribution (algo ns)

```
hx_resolve__flat (n=6, range 989.6-1204.6 ns)
    989.6 |####################
   1000.3 |
   1011.1 |
   1021.8 |
   1032.6 |
   1043.3 |
   1054.1 |
   1064.8 |
   1075.6 |
   1086.3 |
   1097.1 |####################
   1107.8 |
   1118.6 |
   1129.3 |
   1140.1 |
   1150.8 |
   1161.6 |
   1172.3 |########################################
   1183.1 |
   1193.8 |####################
  (0 below, 1 above range)

hx_resolve__linear (n=6, range 10884.2-11989.8 ns)
  10884.2 |########################################
  10939.5 |
  10994.8 |
  11050.0 |
  11105.3 |
  11160.6 |
  11215.9 |
  11271.2 |########################################
  11326.4 |
  11381.7 |
  11437.0 |
  11492.3 |########################################
  11547.6 |
  11602.8 |
  11658.1 |
  11713.4 |########################################
  11768.7 |
  11824.0 |
  11879.2 |########################################
  11934.5 |
  (0 below, 1 above range)

```
