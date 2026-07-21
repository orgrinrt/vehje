# Record update (60% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s60**

## Highlights

Baseline for all deltas below: **rec_reuse_s60**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_mut dominates: 383% faster than the next best (rec_reuse_s60)

rec_mut (17.00 us) leads rec_reuse_s60 (82.06 us) by 383%, a clear separation rather than a photo finish. CV 5.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### rec_mut beats baseline by 79% (significant)

rec_mut is -65.23 us (79%) faster than baseline rec_reuse_s60, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rec_copy is an outlier: 7.0x slower than the field

rec_copy (118.71 us) is 7.0x the fastest (17.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.0x the fastest

Fastest rec_mut (17.00 us) to slowest rec_copy (118.71 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 17003.3 ns median (-79.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.98x (fastest 17003.3 ns, slowest 118705.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 121117ns | 121042ns | 116270ns | 119770ns | 125559ns | +43.27% |
| rec_mut | 19233ns | 19366ns | 18103ns | 18948ns | 20226ns | -77.25% |
| rec_reuse_s60 | 84540ns | 84374ns | 81572ns | 83642ns | 87371ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 118773ns | 114024ns | 123134ns | +44.43% | 0.034 |
| rec_mut | 16876ns | 15890ns | 17729ns | -79.48% | 0.243 |
| rec_reuse_s60 | 82238ns | 79375ns | 84968ns | base | 0.050 |

## Performance model

- Peak throughput: **0.258 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 13.4% |
| rec_mut | 0.241 | 93.5% |
| rec_reuse_s60 | 0.050 | 19.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 121117ns | 121117ns | +43.27% |
| rec_mut | 19233ns | 19233ns | -77.25% |
| rec_reuse_s60 | 84540ns | 84540ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s60 | 82061ns | base | --- | [79684, 84968] | --- | --- | --- | --- |
| rec_copy | 118706ns | +35226.5ns (+42.9%) | [+33504, +40875]ns | [114478, 123134] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 17003ns | -65226.0ns (-79.5%) | [-67293, -63566]ns | [15895, 17729] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s60 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 79992ns | +53.7% | -80.1% |
| 2 | 84527ns | +45.8% | -79.1% |
| 3 | 83889ns | +39.6% | -78.8% |
| 4 | 80234ns | +42.1% | -80.2% |
| 5 | 79375ns | +44.8% | -79.4% |
| 6 | 85409ns | +40.9% | -79.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | 0.394 | moderate+ |
| rec_mut | -0.198 | ok |
| rec_reuse_s60 | -0.227 | moderate- |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 22056.2ns | 118772.6ns | 18.6% | HIGH |
| rec_mut | 22413.2ns | 16875.9ns | 132.8% | HIGH |
| rec_reuse_s60 | 22995.1ns | 82237.6ns | 28.0% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 114023.8-123134.0 ns)
  114023.8 |########################################
  114479.3 |########################################
  114934.8 |
  115390.3 |
  115845.8 |
  116301.3 |
  116756.8 |########################################
  117212.4 |
  117667.9 |
  118123.4 |
  118578.9 |
  119034.4 |
  119489.9 |
  119945.4 |########################################
  120400.9 |
  120856.4 |
  121311.9 |
  121767.4 |
  122222.9 |
  122678.4 |########################################
  (0 below, 1 above range)

rec_mut (n=6, range 15890.0-17729.2 ns)
  15890.0 |########################################
  15982.0 |
  16073.9 |
  16165.9 |
  16257.8 |####################
  16349.8 |
  16441.7 |
  16533.7 |
  16625.7 |
  16717.6 |
  16809.6 |
  16901.5 |
  16993.5 |
  17085.4 |
  17177.4 |
  17269.4 |
  17361.3 |
  17453.3 |
  17545.2 |
  17637.2 |########################################
  (0 below, 1 above range)

rec_reuse_s60 (n=6, range 79375.4-84967.8 ns)
  79375.4 |########################################
  79655.0 |
  79934.6 |########################################
  80214.3 |########################################
  80493.9 |
  80773.5 |
  81053.1 |
  81332.7 |
  81612.3 |
  81892.0 |
  82171.6 |
  82451.2 |
  82730.8 |
  83010.4 |
  83290.0 |
  83569.7 |
  83849.3 |########################################
  84128.9 |
  84408.5 |########################################
  84688.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=18.6% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=132.3% of algo (FFI overhead may distort results)
- **rec_reuse_s60**: bridge=28.0% of algo (FFI overhead may distort results)
