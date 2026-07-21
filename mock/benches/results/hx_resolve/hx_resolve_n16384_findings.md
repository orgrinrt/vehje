# Name resolution: linear scope-chain vs flat shadow-stack

2 variants, 6 samples per variant.
Baseline: **hx_resolve__flat**

## Highlights

Baseline for all deltas below: **hx_resolve__flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_resolve__flat dominates: 1767% faster than the next best (hx_resolve__linear)

hx_resolve__flat (14.92 us) leads hx_resolve__linear (278.63 us) by 1767%, a clear separation rather than a photo finish. CV 5.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_resolve__flat is fastest but the noisiest (CV 5.4%)

hx_resolve__flat wins on median (14.92 us) yet has the highest variance (CV 5.4%), while hx_resolve__linear is the steadiest (CV 3.2%, 278.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hx_resolve__linear shows alternating (throttle bounce) (autocorr -0.75)

hx_resolve__linear's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (hx_resolve__flat)

The baseline hx_resolve__flat is the fastest (14.92 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 18.7x the fastest

Fastest hx_resolve__flat (14.92 us) to slowest hx_resolve__linear (278.63 us): 18.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_resolve__flat) is the fastest** at 14921.6 ns median
- 1 variant significantly slower than baseline
- Spread: 18.67x (fastest 14921.6 ns, slowest 278626.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_resolve__flat | 17485ns | 17144ns | 16511ns | 17053ns | 18620ns | base |
| hx_resolve__linear | 279522ns | 281248ns | 267719ns | 277367ns | 288656ns | +1498.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_resolve__flat | 15228ns | 14368ns | 16237ns | base | 1.076 |
| hx_resolve__linear | 276952ns | 264996ns | 286083ns | +1718.65% | 0.059 |

## Performance model

- Peak throughput: **1.140 Gops/s** (hx_resolve__flat; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_resolve__flat | 1.098 | 96.3% |
| hx_resolve__linear | 0.059 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_resolve__flat | 17485ns | 17485ns | base |
| hx_resolve__linear | 279522ns | 279522ns | +1498.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_resolve__flat | 14922ns | base | --- | [14527, 16237] | --- | --- | --- | --- |
| hx_resolve__linear | 278626ns | +262389.8ns (+1758.5%) | [+251613, +271167]ns | [266145, 286083] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_resolve__flat | hx_resolve__linear |
|---|---|---|
| 1 | 15146ns | +1757.1% |
| 2 | 14368ns | +1744.4% |
| 3 | 14686ns | +1880.7% |
| 4 | 14698ns | +1718.6% |
| 5 | 15751ns | +1653.2% |
| 6 | 16722ns | +1581.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_resolve__flat | 0.347 | moderate+ |
| hx_resolve__linear | -0.746 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hx_resolve__linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_resolve__flat | 2.8ns | 15228.4ns | 0.0% |  |
| hx_resolve__linear | 5.1ns | 276951.6ns | 0.0% |  |

## Distribution (algo ns)

```
hx_resolve__flat (n=6, range 14367.5-16236.6 ns)
  14367.5 |####################
  14461.0 |
  14554.4 |
  14647.9 |########################################
  14741.3 |
  14834.8 |
  14928.2 |
  15021.7 |
  15115.2 |####################
  15208.6 |
  15302.1 |
  15395.5 |
  15489.0 |
  15582.4 |
  15675.9 |####################
  15769.4 |
  15862.8 |
  15956.3 |
  16049.7 |
  16143.2 |
  (0 below, 1 above range)

hx_resolve__linear (n=6, range 264995.8-286083.1 ns)
  264995.8 |####################
  266050.2 |
  267104.5 |####################
  268158.9 |
  269213.3 |
  270267.6 |
  271322.0 |
  272376.4 |
  273430.7 |
  274485.1 |
  275539.4 |####################
  276593.8 |
  277648.2 |
  278702.5 |
  279756.9 |
  280811.3 |########################################
  281865.6 |
  282920.0 |
  283974.4 |
  285028.7 |
  (0 below, 1 above range)

```
