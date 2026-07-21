# Record update (60% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s60**

## Highlights

Baseline for all deltas below: **rec_reuse_s60**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 389% faster than the next best (rec_reuse_s60)

rec_mut (4.23 us) leads rec_reuse_s60 (20.70 us) by 389%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 79% (significant)

rec_mut is -16.39 us (79%) faster than baseline rec_reuse_s60, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 6.8x slower than the field

rec_copy (28.90 us) is 6.8x the fastest (4.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 6.4%)

rec_mut wins on median (4.23 us) yet has the highest variance (CV 6.4%), while rec_reuse_s60 is the steadiest (CV 4.2%, 20.70 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.8x the fastest

Fastest rec_mut (4.23 us) to slowest rec_copy (28.90 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 4234.6 ns median (-79.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.83x (fastest 4234.6 ns, slowest 28901.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 31700ns | 31147ns | 30503ns | 30964ns | 33403ns | +36.44% |
| rec_mut | 6622ns | 6482ns | 6250ns | 6436ns | 7086ns | -71.50% |
| rec_reuse_s60 | 23233ns | 22980ns | 22172ns | 22731ns | 24517ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 29344ns | 28302ns | 30786ns | +40.38% | 0.035 |
| rec_mut | 4329ns | 4096ns | 4621ns | -79.29% | 0.237 |
| rec_reuse_s60 | 20903ns | 19939ns | 22021ns | base | 0.049 |

## Performance model

- Peak throughput: **0.250 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 14.2% |
| rec_mut | 0.242 | 96.7% |
| rec_reuse_s60 | 0.049 | 19.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 31700ns | 31700ns | +36.44% |
| rec_mut | 6622ns | 6622ns | -71.50% |
| rec_reuse_s60 | 23233ns | 23233ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s60 | 20702ns | base | --- | [19987, 22021] | --- | --- | --- | --- |
| rec_copy | 28902ns | +8658.7ns (+41.8%) | [+7442, +9222]ns | [28346, 30786] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 4235ns | -16391.7ns (-79.2%) | [-17517, -15815]ns | [4131, 4621] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s60 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 20950ns | +42.3% | -79.5% |
| 2 | 22270ns | +42.6% | -77.9% |
| 3 | 20453ns | +43.8% | -78.8% |
| 4 | 20035ns | +41.7% | -79.2% |
| 5 | 19939ns | +42.4% | -79.0% |
| 6 | 21772ns | +30.0% | -81.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.347 | moderate+ |
| rec_mut | 0.085 | ok |
| rec_reuse_s60 | -0.036 | ok |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21533.0ns | 29344.3ns | 73.4% | HIGH |
| rec_mut | 19826.7ns | 4328.7ns | 458.0% | HIGH |
| rec_reuse_s60 | 23071.3ns | 20903.3ns | 110.4% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 28301.7-30785.7 ns)
  28301.7 |########################################
  28425.9 |
  28550.1 |
  28674.3 |
  28798.5 |
  28922.7 |
  29046.9 |
  29171.1 |
  29295.3 |#############
  29419.5 |
  29543.7 |
  29667.9 |
  29792.1 |#############
  29916.3 |
  30040.5 |
  30164.7 |
  30288.9 |
  30413.1 |
  30537.3 |
  30661.5 |
  (0 below, 1 above range)

rec_mut (n=6, range 4096.2-4620.9 ns)
   4096.2 |########################################
   4122.4 |
   4148.7 |########################################
   4174.9 |########################################
   4201.1 |
   4227.4 |
   4253.6 |
   4279.8 |########################################
   4306.1 |########################################
   4332.3 |
   4358.5 |
   4384.8 |
   4411.0 |
   4437.2 |
   4463.5 |
   4489.7 |
   4515.9 |
   4542.2 |
   4568.4 |
   4594.6 |
  (0 below, 1 above range)

rec_reuse_s60 (n=6, range 19939.2-22021.5 ns)
  19939.2 |########################################
  20043.3 |
  20147.4 |
  20251.5 |
  20355.7 |####################
  20459.8 |
  20563.9 |
  20668.0 |
  20772.1 |
  20876.2 |####################
  20980.3 |
  21084.4 |
  21188.5 |
  21292.7 |
  21396.8 |
  21500.9 |
  21605.0 |
  21709.1 |####################
  21813.2 |
  21917.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=73.4% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=459.8% of algo (FFI overhead may distort results)
- **rec_reuse_s60**: bridge=110.4% of algo (FFI overhead may distort results)
