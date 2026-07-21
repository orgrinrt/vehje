# Name resolution: flat shadow-stack vs hashed-per-scope vs linear scope-chain walk

3 variants, 6 samples per variant.
Baseline: **resolve_flat**

## Highlights

Baseline for all deltas below: **resolve_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### resolve_flat dominates: 532% faster than the next best (resolve_linear)

resolve_flat (3.97 us) leads resolve_linear (25.04 us) by 532%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### resolve_hashed is an outlier: 18.1x slower than the field

resolve_hashed (71.87 us) is 18.1x the fastest (3.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### resolve_flat is fastest but the noisiest (CV 6.1%)

resolve_flat wins on median (3.97 us) yet has the highest variance (CV 6.1%), while resolve_hashed is the steadiest (CV 4.8%, 71.87 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### resolve_flat shows alternating (throttle bounce) (autocorr -0.54)

resolve_flat's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (resolve_flat)

The baseline resolve_flat is the fastest (3.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 18.1x the fastest

Fastest resolve_flat (3.97 us) to slowest resolve_hashed (71.87 us): 18.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (resolve_flat) is the fastest** at 3965.4 ns median
- 2 variants significantly slower than baseline
- Spread: 18.12x (fastest 3965.4 ns, slowest 71869.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| resolve_flat | 6350ns | 6306ns | 5872ns | 6223ns | 6780ns | base |
| resolve_hashed | 73052ns | 74247ns | 66289ns | 73208ns | 76200ns | +1050.40% |
| resolve_linear | 27115ns | 27599ns | 23783ns | 27434ns | 28302ns | +326.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| resolve_flat | 3993ns | 3695ns | 4259ns | base | 1.026 |
| resolve_hashed | 70636ns | 64110ns | 73712ns | +1668.83% | 0.058 |
| resolve_linear | 24616ns | 21599ns | 25700ns | +516.41% | 0.166 |

## Performance model

- Peak throughput: **1.109 Gops/s** (resolve_flat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| resolve_flat | 1.033 | 93.2% |
| resolve_hashed | 0.057 | 5.1% |
| resolve_linear | 0.164 | 14.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| resolve_flat | 6350ns | 6350ns | base |
| resolve_hashed | 73052ns | 73052ns | +1050.40% |
| resolve_linear | 27115ns | 27115ns | +326.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| resolve_flat | 3965ns | base | --- | [3756, 4259] | --- | --- | --- | --- |
| resolve_hashed | 71870ns | +67755.4ns (+1708.7%) | [+62571, +69602]ns | [66327, 73712] | YES | 0.0313 | 0.0313 | 0 |
| resolve_linear | 25045ns | +21078.1ns (+531.6%) | [+19347, +21442]ns | [23102, 25700] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | resolve_flat | resolve_hashed | resolve_linear |
|---|---|---|---|
| 1 | 3695ns | +1635.0% | +484.6% |
| 2 | 4115ns | +1622.4% | +513.6% |
| 3 | 4113ns | +1671.2% | +515.4% |
| 4 | 3818ns | +1817.2% | +550.6% |
| 5 | 4403ns | +1586.0% | +492.5% |
| 6 | 3816ns | +1696.1% | +544.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| resolve_flat | -0.538 | HIGH- (thermal bounce) |
| resolve_hashed | 0.088 | ok |
| resolve_linear | -0.083 | ok |

**Consistency summary:**

- **resolve_hashed**: won 0/6, lost 6/6
- **resolve_linear**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| resolve_flat | 3.9ns | 3993.4ns | 0.1% |  |
| resolve_hashed | 4.3ns | 70636.1ns | 0.0% |  |
| resolve_linear | 2.6ns | 24615.6ns | 0.0% |  |

## Distribution (algo ns)

```
resolve_flat (n=6, range 3695.0-4259.1 ns)
   3695.0 |####################
   3723.2 |
   3751.4 |
   3779.6 |
   3807.8 |########################################
   3836.0 |
   3864.2 |
   3892.5 |
   3920.7 |
   3948.9 |
   3977.1 |
   4005.3 |
   4033.5 |
   4061.7 |
   4089.9 |########################################
   4118.1 |
   4146.3 |
   4174.5 |
   4202.7 |
   4230.9 |
  (0 below, 1 above range)

resolve_hashed (n=6, range 64109.6-73711.7 ns)
  64109.6 |####################
  64589.7 |
  65069.8 |
  65549.9 |
  66030.0 |
  66510.1 |
  66990.2 |
  67470.3 |
  67950.4 |
  68430.5 |####################
  68910.6 |
  69390.8 |
  69870.9 |
  70351.0 |
  70831.1 |####################
  71311.2 |
  71791.3 |
  72271.4 |
  72751.5 |########################################
  73231.6 |
  (0 below, 1 above range)

resolve_linear (n=6, range 21599.2-25699.8 ns)
  21599.2 |########################################
  21804.2 |
  22009.3 |
  22214.3 |
  22419.3 |
  22624.4 |
  22829.4 |
  23034.4 |
  23239.4 |
  23444.5 |
  23649.5 |
  23854.5 |
  24059.6 |
  24264.6 |
  24469.6 |########################################
  24674.7 |########################################
  24879.7 |
  25084.7 |########################################
  25289.7 |########################################
  25494.8 |
  (0 below, 1 above range)

```
