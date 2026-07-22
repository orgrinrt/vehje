# Iterator fusion (depth 2): materialized vs fused push vs fused pull

3 variants, 6 samples per variant.
Baseline: **iterfuse_pull2**

## Highlights

Baseline for all deltas below: **iterfuse_pull2**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### iterfuse_pull2 dominates: 22% faster than the next best (iterfuse_mat2)

iterfuse_pull2 (60.48 us) leads iterfuse_mat2 (73.89 us) by 22%, a clear separation rather than a photo finish. CV 15.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iterfuse_push2 is an outlier: 3.1x slower than the field

iterfuse_push2 (187.65 us) is 3.1x the fastest (60.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iterfuse_pull2 is fastest but the noisiest (CV 15.4%)

iterfuse_pull2 wins on median (60.48 us) yet has the highest variance (CV 15.4%), while iterfuse_push2 is the steadiest (CV 2.2%, 187.65 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iterfuse_mat2 shows alternating (throttle bounce) (autocorr -0.61)

iterfuse_mat2's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (iterfuse_pull2)

The baseline iterfuse_pull2 is the fastest (60.48 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.1x the fastest

Fastest iterfuse_pull2 (60.48 us) to slowest iterfuse_push2 (187.65 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (iterfuse_pull2) is the fastest** at 60484.4 ns median
- 2 variants significantly slower than baseline
- Spread: 3.10x (fastest 60484.4 ns, slowest 187654.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iterfuse_mat2 | 78056ns | 76114ns | 67264ns | 75082ns | 87912ns | +27.75% |
| iterfuse_pull2 | 61100ns | 62706ns | 49315ns | 58536ns | 70838ns | base |
| iterfuse_push2 | 189503ns | 190336ns | 183936ns | 188418ns | 193915ns | +210.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iterfuse_mat2 | 75784ns | 64988ns | 85631ns | +28.78% | 0.054 |
| iterfuse_pull2 | 58846ns | 47070ns | 68561ns | base | 0.070 |
| iterfuse_push2 | 187078ns | 181694ns | 191706ns | +217.91% | 0.022 |

## Performance model

- Peak throughput: **0.087 Gops/s** (iterfuse_pull2; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iterfuse_mat2 | 0.055 | 63.7% |
| iterfuse_pull2 | 0.068 | 77.8% |
| iterfuse_push2 | 0.022 | 25.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iterfuse_mat2 | 78056ns | 78056ns | +27.75% |
| iterfuse_pull2 | 61100ns | 61100ns | base |
| iterfuse_push2 | 189503ns | 189503ns | +210.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iterfuse_pull2 | 60484ns | base | --- | [47492, 68561] | --- | --- | --- | --- |
| iterfuse_mat2 | 73889ns | +16550.8ns (+27.4%) | [+5327, +28937]ns | [67833, 85631] | YES | 0.0313 | 0.0313 | 0 |
| iterfuse_push2 | 187655ns | +128910.9ns (+213.1%) | [+115362, +140424]ns | [181874, 191706] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | iterfuse_pull2 | iterfuse_mat2 | iterfuse_push2 |
|---|---|---|---|
| 1 | 65475ns | +31.8% | +178.1% |
| 2 | 47070ns | +38.1% | +308.1% |
| 3 | 47914ns | +77.4% | +279.2% |
| 4 | 71642ns | +3.9% | +159.3% |
| 5 | 65480ns | +12.1% | +189.4% |
| 6 | 55494ns | +27.4% | +244.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iterfuse_mat2 | -0.608 | HIGH- (thermal bounce) |
| iterfuse_pull2 | -0.051 | ok |
| iterfuse_push2 | -0.361 | moderate- |

**Consistency summary:**

- **iterfuse_mat2**: won 0/6, lost 6/6
- **iterfuse_push2**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iterfuse_mat2 | 696.0ns | 75784.2ns | 0.9% |  |
| iterfuse_pull2 | 4.1ns | 58845.8ns | 0.0% |  |
| iterfuse_push2 | 3.4ns | 187078.3ns | 0.0% |  |

## Distribution (algo ns)

```
iterfuse_mat2 (n=6, range 64987.5-85631.2 ns)
  64987.5 |########################################
  66019.7 |
  67051.9 |
  68084.1 |
  69116.2 |
  70148.4 |########################################
  71180.6 |
  72212.8 |
  73245.0 |########################################
  74277.2 |########################################
  75309.4 |
  76341.5 |
  77373.7 |
  78405.9 |
  79438.1 |
  80470.3 |
  81502.5 |
  82534.6 |
  83566.8 |
  84599.0 |########################################
  (0 below, 1 above range)

iterfuse_pull2 (n=6, range 47069.6-68561.2 ns)
  47069.6 |########################################
  48144.2 |
  49218.8 |
  50293.3 |
  51367.9 |
  52442.5 |
  53517.1 |
  54591.7 |####################
  55666.3 |
  56740.8 |
  57815.4 |
  58890.0 |
  59964.6 |
  61039.2 |
  62113.8 |
  63188.3 |
  64262.9 |
  65337.5 |########################################
  66412.1 |
  67486.7 |
  (0 below, 1 above range)

iterfuse_push2 (n=6, range 181694.2-191706.2 ns)
  181694.2 |########################################
  182194.8 |
  182695.4 |
  183196.0 |
  183696.6 |
  184197.2 |
  184697.8 |
  185198.4 |
  185699.0 |####################
  186199.6 |
  186700.2 |
  187200.8 |
  187701.4 |
  188202.0 |
  188702.6 |
  189203.2 |####################
  189703.8 |
  190204.4 |
  190705.0 |
  191205.6 |####################
  (0 below, 1 above range)

```
