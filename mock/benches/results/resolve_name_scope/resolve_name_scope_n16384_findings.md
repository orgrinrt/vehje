# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 1157% faster than the next best (resolve_linear)

resolve_flat (15.17 us) leads resolve_linear (190.66 us) by 1157%, a clear separation rather than a photo finish. CV 7.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 20.7x slower than the field

resolve_hashed (314.39 us) is 20.7x the fastest (15.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### resolve_flat is fastest but the noisiest (CV 7.8%)

resolve_flat wins on median (15.17 us) yet has the highest variance (CV 7.8%), while resolve_hashed is the steadiest (CV 2.8%, 314.39 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### resolve_flat shows alternating (throttle bounce) (autocorr -0.54)

resolve_flat's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (15.17 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 20.7x the fastest

Fastest resolve_flat (15.17 us) to slowest resolve_hashed (314.39 us): 20.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 15170.6 ns median
- 2 variants significantly slower than baseline
- Spread: 20.72x (fastest 15170.6 ns, slowest 314388.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 18172ns | 17359ns | 17027ns | 17352ns | 19975ns | base |
| resolve_hashed | 320369ns | 316907ns | 312306ns | 315665ns | 331457ns | +1662.96% |
| resolve_linear | 191936ns | 193059ns | 176261ns | 190653ns | 201698ns | +956.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 15887ns | 14883ns | 17471ns | base | 1.031 |
| resolve_hashed | 317803ns | 309757ns | 328999ns | +1900.43% | 0.052 |
| resolve_linear | 189480ns | 173937ns | 199084ns | +1092.70% | 0.086 |

## Performance model

- Peak throughput: **1.101 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 1.080 | 98.1% |
| resolve_hashed | 0.052 | 4.7% |
| resolve_linear | 0.086 | 7.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 18172ns | 18172ns | base |
| resolve_hashed | 320369ns | 320369ns | +1662.96% |
| resolve_linear | 191936ns | 191936ns | +956.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 15171ns | base | --- | [15019, 17471] | --- | --- | --- | --- |
| resolve_hashed | 314389ns | +298545.2ns (+1967.9%) | [+294839, +312365]ns | [310021, 328999] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 190657ns | +174023.0ns (+1147.1%) | [+163669, +183089]ns | [178700, 199084] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 15163ns | +2107.1% | +1154.5% |
| 2 | 14883ns | +1981.3% | +1068.7% |
| 3 | 18106ns | +1685.9% | +955.4% |
| 4 | 15155ns | +1947.5% | +1244.4% |
| 5 | 16836ns | +1751.1% | +1054.9% |
| 6 | 15178ns | +1989.4% | +1108.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | -0.535 | HIGH- (thermal bounce) |
| resolve_hashed | -0.361 | moderate- |
| resolve_linear | 0.053 | ok |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 4.0ns | 15886.7ns | 0.0% |  |
| resolve_hashed | 7.7ns | 317803.0ns | 0.0% |  |
| resolve_linear | 11.3ns | 189480.3ns | 0.0% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 14882.9-17470.8 ns)
  14882.9 |#############
  15012.3 |
  15141.7 |########################################
  15271.1 |
  15400.5 |
  15529.9 |
  15659.3 |
  15788.7 |
  15918.1 |
  16047.5 |
  16176.8 |
  16306.2 |
  16435.6 |
  16565.0 |
  16694.4 |
  16823.8 |#############
  16953.2 |
  17082.6 |
  17212.0 |
  17341.4 |
  (0 below, 1 above range)

resolve_hashed (n=6, range 309756.7-328999.3 ns)
  309756.7 |########################################
  310718.8 |####################
  311681.0 |
  312643.1 |
  313605.2 |
  314567.4 |
  315529.5 |
  316491.6 |####################
  317453.8 |
  318415.9 |
  319378.0 |
  320340.2 |
  321302.3 |
  322264.4 |
  323226.6 |####################
  324188.7 |
  325150.8 |
  326113.0 |
  327075.1 |
  328037.2 |
  (0 below, 1 above range)

resolve_linear (n=6, range 173936.7-199083.8 ns)
  173936.7 |########################################
  175194.1 |
  176451.4 |
  177708.8 |
  178966.1 |
  180223.5 |
  181480.8 |
  182738.2 |########################################
  183995.5 |
  185252.9 |
  186510.2 |
  187767.6 |
  189024.9 |########################################
  190282.3 |########################################
  191539.6 |
  192797.0 |
  194054.3 |########################################
  195311.7 |
  196569.0 |
  197826.4 |
  (0 below, 1 above range)

```
